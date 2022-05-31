#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct AnimationDirection(pub i32);
impl AnimationDirection {
    pub const Left: Self = Self(0i32);
    pub const Top: Self = Self(1i32);
    pub const Right: Self = Self(2i32);
    pub const Bottom: Self = Self(3i32);
}
impl ::core::marker::Copy for AnimationDirection {}
impl ::core::clone::Clone for AnimationDirection {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AnimationDirection {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for AnimationDirection {
    type Abi = Self;
}
impl ::core::fmt::Debug for AnimationDirection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AnimationDirection").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AnimationDirection {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Controls.Primitives.AnimationDirection;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct AppBarButtonTemplateSettings(::windows_core::IUnknown);
impl AppBarButtonTemplateSettings {
    pub fn KeyboardAcceleratorTextMinWidth(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).KeyboardAcceleratorTextMinWidth)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
}
impl ::core::clone::Clone for AppBarButtonTemplateSettings {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppBarButtonTemplateSettings {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppBarButtonTemplateSettings {}
impl ::core::fmt::Debug for AppBarButtonTemplateSettings {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppBarButtonTemplateSettings").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AppBarButtonTemplateSettings {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Controls.Primitives.AppBarButtonTemplateSettings;{cbc9b39d-0c95-4951-bff2-13963691c366})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for AppBarButtonTemplateSettings {
    type Vtable = IAppBarButtonTemplateSettings_Vtbl;
    const IID: ::windows_core::GUID = <IAppBarButtonTemplateSettings as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for AppBarButtonTemplateSettings {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.AppBarButtonTemplateSettings";
}
impl ::core::convert::From<AppBarButtonTemplateSettings> for ::windows_core::IUnknown {
    fn from(value: AppBarButtonTemplateSettings) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppBarButtonTemplateSettings> for ::windows_core::IUnknown {
    fn from(value: &AppBarButtonTemplateSettings) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for AppBarButtonTemplateSettings {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a AppBarButtonTemplateSettings {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppBarButtonTemplateSettings> for ::windows_core::IInspectable {
    fn from(value: AppBarButtonTemplateSettings) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppBarButtonTemplateSettings> for ::windows_core::IInspectable {
    fn from(value: &AppBarButtonTemplateSettings) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for AppBarButtonTemplateSettings {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a AppBarButtonTemplateSettings {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppBarButtonTemplateSettings> for super::super::DependencyObject {
    fn from(value: AppBarButtonTemplateSettings) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&AppBarButtonTemplateSettings> for super::super::DependencyObject {
    fn from(value: &AppBarButtonTemplateSettings) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::DependencyObject> for AppBarButtonTemplateSettings {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::DependencyObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::DependencyObject> for &AppBarButtonTemplateSettings {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::DependencyObject> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for AppBarButtonTemplateSettings {}
unsafe impl ::core::marker::Sync for AppBarButtonTemplateSettings {}
#[repr(transparent)]
pub struct AppBarTemplateSettings(::windows_core::IUnknown);
impl AppBarTemplateSettings {
    pub fn ClipRect(&self) -> ::windows_core::Result<::winrt_foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::Rect>::zeroed();
            (::windows_core::Interface::vtable(this).ClipRect)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Rect>(result__)
        }
    }
    pub fn CompactVerticalDelta(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).CompactVerticalDelta)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn CompactRootMargin(&self) -> ::windows_core::Result<super::super::Thickness> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::Thickness>::zeroed();
            (::windows_core::Interface::vtable(this).CompactRootMargin)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Thickness>(result__)
        }
    }
    pub fn MinimalVerticalDelta(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).MinimalVerticalDelta)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn MinimalRootMargin(&self) -> ::windows_core::Result<super::super::Thickness> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::Thickness>::zeroed();
            (::windows_core::Interface::vtable(this).MinimalRootMargin)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Thickness>(result__)
        }
    }
    pub fn HiddenVerticalDelta(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).HiddenVerticalDelta)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn HiddenRootMargin(&self) -> ::windows_core::Result<super::super::Thickness> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::Thickness>::zeroed();
            (::windows_core::Interface::vtable(this).HiddenRootMargin)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Thickness>(result__)
        }
    }
    pub fn NegativeCompactVerticalDelta(&self) -> ::windows_core::Result<f64> {
        let this = &::windows_core::Interface::cast::<IAppBarTemplateSettings2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).NegativeCompactVerticalDelta)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn NegativeMinimalVerticalDelta(&self) -> ::windows_core::Result<f64> {
        let this = &::windows_core::Interface::cast::<IAppBarTemplateSettings2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).NegativeMinimalVerticalDelta)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn NegativeHiddenVerticalDelta(&self) -> ::windows_core::Result<f64> {
        let this = &::windows_core::Interface::cast::<IAppBarTemplateSettings2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).NegativeHiddenVerticalDelta)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
}
impl ::core::clone::Clone for AppBarTemplateSettings {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppBarTemplateSettings {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppBarTemplateSettings {}
impl ::core::fmt::Debug for AppBarTemplateSettings {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppBarTemplateSettings").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AppBarTemplateSettings {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Controls.Primitives.AppBarTemplateSettings;{bcc2a863-eb35-423c-8389-d7827be3bf67})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for AppBarTemplateSettings {
    type Vtable = IAppBarTemplateSettings_Vtbl;
    const IID: ::windows_core::GUID = <IAppBarTemplateSettings as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for AppBarTemplateSettings {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.AppBarTemplateSettings";
}
impl ::core::convert::From<AppBarTemplateSettings> for ::windows_core::IUnknown {
    fn from(value: AppBarTemplateSettings) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppBarTemplateSettings> for ::windows_core::IUnknown {
    fn from(value: &AppBarTemplateSettings) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for AppBarTemplateSettings {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a AppBarTemplateSettings {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppBarTemplateSettings> for ::windows_core::IInspectable {
    fn from(value: AppBarTemplateSettings) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppBarTemplateSettings> for ::windows_core::IInspectable {
    fn from(value: &AppBarTemplateSettings) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for AppBarTemplateSettings {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a AppBarTemplateSettings {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppBarTemplateSettings> for super::super::DependencyObject {
    fn from(value: AppBarTemplateSettings) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&AppBarTemplateSettings> for super::super::DependencyObject {
    fn from(value: &AppBarTemplateSettings) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::DependencyObject> for AppBarTemplateSettings {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::DependencyObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::DependencyObject> for &AppBarTemplateSettings {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::DependencyObject> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for AppBarTemplateSettings {}
unsafe impl ::core::marker::Sync for AppBarTemplateSettings {}
#[repr(transparent)]
pub struct AppBarToggleButtonTemplateSettings(::windows_core::IUnknown);
impl AppBarToggleButtonTemplateSettings {
    pub fn KeyboardAcceleratorTextMinWidth(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).KeyboardAcceleratorTextMinWidth)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
}
impl ::core::clone::Clone for AppBarToggleButtonTemplateSettings {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppBarToggleButtonTemplateSettings {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppBarToggleButtonTemplateSettings {}
impl ::core::fmt::Debug for AppBarToggleButtonTemplateSettings {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppBarToggleButtonTemplateSettings").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AppBarToggleButtonTemplateSettings {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Controls.Primitives.AppBarToggleButtonTemplateSettings;{aaf99c48-d8f4-40d9-9fa3-3a64f0fec5d8})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for AppBarToggleButtonTemplateSettings {
    type Vtable = IAppBarToggleButtonTemplateSettings_Vtbl;
    const IID: ::windows_core::GUID = <IAppBarToggleButtonTemplateSettings as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for AppBarToggleButtonTemplateSettings {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.AppBarToggleButtonTemplateSettings";
}
impl ::core::convert::From<AppBarToggleButtonTemplateSettings> for ::windows_core::IUnknown {
    fn from(value: AppBarToggleButtonTemplateSettings) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppBarToggleButtonTemplateSettings> for ::windows_core::IUnknown {
    fn from(value: &AppBarToggleButtonTemplateSettings) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for AppBarToggleButtonTemplateSettings {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a AppBarToggleButtonTemplateSettings {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppBarToggleButtonTemplateSettings> for ::windows_core::IInspectable {
    fn from(value: AppBarToggleButtonTemplateSettings) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppBarToggleButtonTemplateSettings> for ::windows_core::IInspectable {
    fn from(value: &AppBarToggleButtonTemplateSettings) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for AppBarToggleButtonTemplateSettings {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a AppBarToggleButtonTemplateSettings {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppBarToggleButtonTemplateSettings> for super::super::DependencyObject {
    fn from(value: AppBarToggleButtonTemplateSettings) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&AppBarToggleButtonTemplateSettings> for super::super::DependencyObject {
    fn from(value: &AppBarToggleButtonTemplateSettings) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::DependencyObject> for AppBarToggleButtonTemplateSettings {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::DependencyObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::DependencyObject> for &AppBarToggleButtonTemplateSettings {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::DependencyObject> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for AppBarToggleButtonTemplateSettings {}
unsafe impl ::core::marker::Sync for AppBarToggleButtonTemplateSettings {}
#[repr(transparent)]
pub struct ButtonBase(::windows_core::IUnknown);
impl ButtonBase {
    pub fn ClickMode(&self) -> ::windows_core::Result<super::ClickMode> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::ClickMode>::zeroed();
            (::windows_core::Interface::vtable(this).ClickMode)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::ClickMode>(result__)
        }
    }
    pub fn SetClickMode(&self, value: super::ClickMode) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetClickMode)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsPointerOver(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsPointerOver)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn IsPressed(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsPressed)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn Command(&self) -> ::windows_core::Result<super::super::Input::ICommand> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Command)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Input::ICommand>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn SetCommand<'a, Param0: ::windows_core::IntoParam<'a, super::super::Input::ICommand>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetCommand)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn CommandParameter(&self) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows_core::Interface::vtable(this).CommandParameter)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::IInspectable>(result__)
        }
    }
    pub fn SetCommandParameter<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::IInspectable>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetCommandParameter)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn Click<'a, Param0: ::windows_core::IntoParam<'a, super::super::RoutedEventHandler>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).Click)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveClick<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveClick)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn ClickModeProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::IButtonBaseStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ClickModeProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn IsPointerOverProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::IButtonBaseStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).IsPointerOverProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn IsPressedProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::IButtonBaseStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).IsPressedProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn CommandProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::IButtonBaseStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CommandProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn CommandParameterProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::IButtonBaseStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CommandParameterProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn IButtonBaseStatics<R, F: FnOnce(&IButtonBaseStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<ButtonBase, IButtonBaseStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for ButtonBase {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ButtonBase {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ButtonBase {}
impl ::core::fmt::Debug for ButtonBase {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ButtonBase").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ButtonBase {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Controls.Primitives.ButtonBase;{fa002c1a-494e-46cf-91d4-e14a8d798674})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for ButtonBase {
    type Vtable = IButtonBase_Vtbl;
    const IID: ::windows_core::GUID = <IButtonBase as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for ButtonBase {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.ButtonBase";
}
impl ::core::convert::From<ButtonBase> for ::windows_core::IUnknown {
    fn from(value: ButtonBase) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ButtonBase> for ::windows_core::IUnknown {
    fn from(value: &ButtonBase) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ButtonBase {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ButtonBase {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ButtonBase> for ::windows_core::IInspectable {
    fn from(value: ButtonBase) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ButtonBase> for ::windows_core::IInspectable {
    fn from(value: &ButtonBase) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for ButtonBase {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a ButtonBase {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<ButtonBase> for super::super::super::Composition::IAnimationObject {
    type Error = ::windows_core::Error;
    fn try_from(value: ButtonBase) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<&ButtonBase> for super::super::super::Composition::IAnimationObject {
    type Error = ::windows_core::Error;
    fn try_from(value: &ButtonBase) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows_core::IntoParam<'a, super::super::super::Composition::IAnimationObject> for ButtonBase {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::super::Composition::IAnimationObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows_core::IntoParam<'a, super::super::super::Composition::IAnimationObject> for &ButtonBase {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::super::Composition::IAnimationObject> {
        ::core::convert::TryInto::<super::super::super::Composition::IAnimationObject>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<ButtonBase> for super::super::super::Composition::IVisualElement {
    type Error = ::windows_core::Error;
    fn try_from(value: ButtonBase) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<&ButtonBase> for super::super::super::Composition::IVisualElement {
    type Error = ::windows_core::Error;
    fn try_from(value: &ButtonBase) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows_core::IntoParam<'a, super::super::super::Composition::IVisualElement> for ButtonBase {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::super::Composition::IVisualElement> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows_core::IntoParam<'a, super::super::super::Composition::IVisualElement> for &ButtonBase {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::super::Composition::IVisualElement> {
        ::core::convert::TryInto::<super::super::super::Composition::IVisualElement>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::From<ButtonBase> for super::ContentControl {
    fn from(value: ButtonBase) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&ButtonBase> for super::ContentControl {
    fn from(value: &ButtonBase) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::ContentControl> for ButtonBase {
    fn into_param(self) -> ::windows_core::Param<'a, super::ContentControl> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::ContentControl> for &ButtonBase {
    fn into_param(self) -> ::windows_core::Param<'a, super::ContentControl> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::ContentControl>::into(self))
    }
}
impl ::core::convert::From<ButtonBase> for super::Control {
    fn from(value: ButtonBase) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&ButtonBase> for super::Control {
    fn from(value: &ButtonBase) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::Control> for ButtonBase {
    fn into_param(self) -> ::windows_core::Param<'a, super::Control> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::Control> for &ButtonBase {
    fn into_param(self) -> ::windows_core::Param<'a, super::Control> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::Control>::into(self))
    }
}
impl ::core::convert::From<ButtonBase> for super::super::FrameworkElement {
    fn from(value: ButtonBase) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&ButtonBase> for super::super::FrameworkElement {
    fn from(value: &ButtonBase) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::FrameworkElement> for ButtonBase {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::FrameworkElement> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::FrameworkElement> for &ButtonBase {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::FrameworkElement> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::super::FrameworkElement>::into(self))
    }
}
impl ::core::convert::From<ButtonBase> for super::super::UIElement {
    fn from(value: ButtonBase) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&ButtonBase> for super::super::UIElement {
    fn from(value: &ButtonBase) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::UIElement> for ButtonBase {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::UIElement> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::UIElement> for &ButtonBase {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::UIElement> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::super::UIElement>::into(self))
    }
}
impl ::core::convert::From<ButtonBase> for super::super::DependencyObject {
    fn from(value: ButtonBase) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&ButtonBase> for super::super::DependencyObject {
    fn from(value: &ButtonBase) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::DependencyObject> for ButtonBase {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::DependencyObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::DependencyObject> for &ButtonBase {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::DependencyObject> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for ButtonBase {}
unsafe impl ::core::marker::Sync for ButtonBase {}
#[repr(transparent)]
pub struct CalendarPanel(::windows_core::IUnknown);
impl CalendarPanel {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<CalendarPanel, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for CalendarPanel {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CalendarPanel {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CalendarPanel {}
impl ::core::fmt::Debug for CalendarPanel {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CalendarPanel").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for CalendarPanel {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Controls.Primitives.CalendarPanel;{fcd55a2d-02d3-4ee6-9a90-9df3ead00994})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for CalendarPanel {
    type Vtable = ICalendarPanel_Vtbl;
    const IID: ::windows_core::GUID = <ICalendarPanel as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for CalendarPanel {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.CalendarPanel";
}
impl ::core::convert::From<CalendarPanel> for ::windows_core::IUnknown {
    fn from(value: CalendarPanel) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CalendarPanel> for ::windows_core::IUnknown {
    fn from(value: &CalendarPanel) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for CalendarPanel {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a CalendarPanel {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<CalendarPanel> for ::windows_core::IInspectable {
    fn from(value: CalendarPanel) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CalendarPanel> for ::windows_core::IInspectable {
    fn from(value: &CalendarPanel) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for CalendarPanel {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a CalendarPanel {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<CalendarPanel> for super::super::super::Composition::IAnimationObject {
    type Error = ::windows_core::Error;
    fn try_from(value: CalendarPanel) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<&CalendarPanel> for super::super::super::Composition::IAnimationObject {
    type Error = ::windows_core::Error;
    fn try_from(value: &CalendarPanel) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows_core::IntoParam<'a, super::super::super::Composition::IAnimationObject> for CalendarPanel {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::super::Composition::IAnimationObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows_core::IntoParam<'a, super::super::super::Composition::IAnimationObject> for &CalendarPanel {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::super::Composition::IAnimationObject> {
        ::core::convert::TryInto::<super::super::super::Composition::IAnimationObject>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<CalendarPanel> for super::super::super::Composition::IVisualElement {
    type Error = ::windows_core::Error;
    fn try_from(value: CalendarPanel) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<&CalendarPanel> for super::super::super::Composition::IVisualElement {
    type Error = ::windows_core::Error;
    fn try_from(value: &CalendarPanel) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows_core::IntoParam<'a, super::super::super::Composition::IVisualElement> for CalendarPanel {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::super::Composition::IVisualElement> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows_core::IntoParam<'a, super::super::super::Composition::IVisualElement> for &CalendarPanel {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::super::Composition::IVisualElement> {
        ::core::convert::TryInto::<super::super::super::Composition::IVisualElement>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::From<CalendarPanel> for super::Panel {
    fn from(value: CalendarPanel) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&CalendarPanel> for super::Panel {
    fn from(value: &CalendarPanel) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::Panel> for CalendarPanel {
    fn into_param(self) -> ::windows_core::Param<'a, super::Panel> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::Panel> for &CalendarPanel {
    fn into_param(self) -> ::windows_core::Param<'a, super::Panel> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::Panel>::into(self))
    }
}
impl ::core::convert::From<CalendarPanel> for super::super::FrameworkElement {
    fn from(value: CalendarPanel) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&CalendarPanel> for super::super::FrameworkElement {
    fn from(value: &CalendarPanel) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::FrameworkElement> for CalendarPanel {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::FrameworkElement> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::FrameworkElement> for &CalendarPanel {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::FrameworkElement> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::super::FrameworkElement>::into(self))
    }
}
impl ::core::convert::From<CalendarPanel> for super::super::UIElement {
    fn from(value: CalendarPanel) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&CalendarPanel> for super::super::UIElement {
    fn from(value: &CalendarPanel) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::UIElement> for CalendarPanel {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::UIElement> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::UIElement> for &CalendarPanel {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::UIElement> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::super::UIElement>::into(self))
    }
}
impl ::core::convert::From<CalendarPanel> for super::super::DependencyObject {
    fn from(value: CalendarPanel) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&CalendarPanel> for super::super::DependencyObject {
    fn from(value: &CalendarPanel) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::DependencyObject> for CalendarPanel {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::DependencyObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::DependencyObject> for &CalendarPanel {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::DependencyObject> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for CalendarPanel {}
unsafe impl ::core::marker::Sync for CalendarPanel {}
#[repr(transparent)]
pub struct CalendarViewTemplateSettings(::windows_core::IUnknown);
impl CalendarViewTemplateSettings {
    pub fn MinViewWidth(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).MinViewWidth)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn HeaderText(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).HeaderText)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn WeekDay1(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).WeekDay1)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn WeekDay2(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).WeekDay2)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn WeekDay3(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).WeekDay3)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn WeekDay4(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).WeekDay4)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn WeekDay5(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).WeekDay5)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn WeekDay6(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).WeekDay6)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn WeekDay7(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).WeekDay7)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn HasMoreContentAfter(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).HasMoreContentAfter)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn HasMoreContentBefore(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).HasMoreContentBefore)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn HasMoreViews(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).HasMoreViews)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn ClipRect(&self) -> ::windows_core::Result<::winrt_foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::Rect>::zeroed();
            (::windows_core::Interface::vtable(this).ClipRect)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Rect>(result__)
        }
    }
    pub fn CenterX(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).CenterX)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn CenterY(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).CenterY)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
}
impl ::core::clone::Clone for CalendarViewTemplateSettings {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CalendarViewTemplateSettings {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CalendarViewTemplateSettings {}
impl ::core::fmt::Debug for CalendarViewTemplateSettings {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CalendarViewTemplateSettings").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for CalendarViewTemplateSettings {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Controls.Primitives.CalendarViewTemplateSettings;{56c71483-64e1-477c-8a0b-cb2f3334b9b0})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for CalendarViewTemplateSettings {
    type Vtable = ICalendarViewTemplateSettings_Vtbl;
    const IID: ::windows_core::GUID = <ICalendarViewTemplateSettings as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for CalendarViewTemplateSettings {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.CalendarViewTemplateSettings";
}
impl ::core::convert::From<CalendarViewTemplateSettings> for ::windows_core::IUnknown {
    fn from(value: CalendarViewTemplateSettings) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CalendarViewTemplateSettings> for ::windows_core::IUnknown {
    fn from(value: &CalendarViewTemplateSettings) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for CalendarViewTemplateSettings {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a CalendarViewTemplateSettings {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<CalendarViewTemplateSettings> for ::windows_core::IInspectable {
    fn from(value: CalendarViewTemplateSettings) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CalendarViewTemplateSettings> for ::windows_core::IInspectable {
    fn from(value: &CalendarViewTemplateSettings) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for CalendarViewTemplateSettings {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a CalendarViewTemplateSettings {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<CalendarViewTemplateSettings> for super::super::DependencyObject {
    fn from(value: CalendarViewTemplateSettings) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&CalendarViewTemplateSettings> for super::super::DependencyObject {
    fn from(value: &CalendarViewTemplateSettings) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::DependencyObject> for CalendarViewTemplateSettings {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::DependencyObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::DependencyObject> for &CalendarViewTemplateSettings {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::DependencyObject> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for CalendarViewTemplateSettings {}
unsafe impl ::core::marker::Sync for CalendarViewTemplateSettings {}
#[repr(transparent)]
pub struct CarouselPanel(::windows_core::IUnknown);
impl CarouselPanel {
    pub fn CanVerticallyScroll(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).CanVerticallyScroll)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetCanVerticallyScroll(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetCanVerticallyScroll)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn CanHorizontallyScroll(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).CanHorizontallyScroll)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetCanHorizontallyScroll(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetCanHorizontallyScroll)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ExtentWidth(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).ExtentWidth)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn ExtentHeight(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).ExtentHeight)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn ViewportWidth(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).ViewportWidth)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn ViewportHeight(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).ViewportHeight)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn HorizontalOffset(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).HorizontalOffset)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn VerticalOffset(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).VerticalOffset)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn ScrollOwner(&self) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows_core::Interface::vtable(this).ScrollOwner)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::IInspectable>(result__)
        }
    }
    pub fn SetScrollOwner<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::IInspectable>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetScrollOwner)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn LineUp(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).LineUp)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn LineDown(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).LineDown)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn LineLeft(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).LineLeft)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn LineRight(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).LineRight)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn PageUp(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).PageUp)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn PageDown(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).PageDown)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn PageLeft(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).PageLeft)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn PageRight(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).PageRight)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn MouseWheelUp(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).MouseWheelUp)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn MouseWheelDown(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).MouseWheelDown)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn MouseWheelLeft(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).MouseWheelLeft)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn MouseWheelRight(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).MouseWheelRight)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn SetHorizontalOffset(&self, offset: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetHorizontalOffset)(::windows_core::Interface::as_raw(this), offset).ok() }
    }
    pub fn SetVerticalOffset(&self, offset: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetVerticalOffset)(::windows_core::Interface::as_raw(this), offset).ok() }
    }
    pub fn MakeVisible<'a, Param0: ::windows_core::IntoParam<'a, super::super::UIElement>, Param1: ::windows_core::IntoParam<'a, ::winrt_foundation::Rect>>(&self, visual: Param0, rectangle: Param1) -> ::windows_core::Result<::winrt_foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::Rect>::zeroed();
            (::windows_core::Interface::vtable(this).MakeVisible)(::windows_core::Interface::as_raw(this), visual.into_param().abi(), rectangle.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Rect>(result__)
        }
    }
    pub fn new() -> ::windows_core::Result<CarouselPanel> {
        Self::ICarouselPanelFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateInstance)(::windows_core::Interface::as_raw(this), ::core::ptr::null_mut(), &mut ::core::option::Option::<::windows_core::IInspectable>::None as *mut _ as _, result__.as_mut_ptr()).from_abi::<CarouselPanel>(result__)
        })
    }
    pub fn compose<T: ::windows_core::Compose>(compose: T) -> ::windows_core::Result<CarouselPanel> {
        Self::ICarouselPanelFactory(|this| unsafe {
            let (derived__, base__) = ::windows_core::Compose::compose(compose);
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateInstance)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(&derived__), base__ as *mut _ as _, result__.as_mut_ptr()).from_abi::<CarouselPanel>(result__)
        })
    }
    pub fn AreHorizontalSnapPointsRegular(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IScrollSnapPointsInfo>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).AreHorizontalSnapPointsRegular)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn AreVerticalSnapPointsRegular(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IScrollSnapPointsInfo>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).AreVerticalSnapPointsRegular)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn HorizontalSnapPointsChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventHandler<::windows_core::IInspectable>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = &::windows_core::Interface::cast::<IScrollSnapPointsInfo>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).HorizontalSnapPointsChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveHorizontalSnapPointsChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IScrollSnapPointsInfo>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).RemoveHorizontalSnapPointsChanged)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn VerticalSnapPointsChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventHandler<::windows_core::IInspectable>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = &::windows_core::Interface::cast::<IScrollSnapPointsInfo>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).VerticalSnapPointsChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveVerticalSnapPointsChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IScrollSnapPointsInfo>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).RemoveVerticalSnapPointsChanged)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetIrregularSnapPoints(&self, orientation: super::Orientation, alignment: SnapPointsAlignment) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<f32>> {
        let this = &::windows_core::Interface::cast::<IScrollSnapPointsInfo>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetIrregularSnapPoints)(::windows_core::Interface::as_raw(this), orientation, alignment, result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<f32>>(result__)
        }
    }
    pub fn GetRegularSnapPoints(&self, orientation: super::Orientation, alignment: SnapPointsAlignment, offset: &mut f32) -> ::windows_core::Result<f32> {
        let this = &::windows_core::Interface::cast::<IScrollSnapPointsInfo>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f32>::zeroed();
            (::windows_core::Interface::vtable(this).GetRegularSnapPoints)(::windows_core::Interface::as_raw(this), orientation, alignment, offset, result__.as_mut_ptr()).from_abi::<f32>(result__)
        }
    }
    pub fn ICarouselPanelFactory<R, F: FnOnce(&ICarouselPanelFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<CarouselPanel, ICarouselPanelFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for CarouselPanel {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CarouselPanel {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CarouselPanel {}
impl ::core::fmt::Debug for CarouselPanel {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CarouselPanel").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for CarouselPanel {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Controls.Primitives.CarouselPanel;{deab78b2-373b-4151-8785-e544d0d9362b})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for CarouselPanel {
    type Vtable = ICarouselPanel_Vtbl;
    const IID: ::windows_core::GUID = <ICarouselPanel as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for CarouselPanel {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.CarouselPanel";
}
impl ::core::convert::From<CarouselPanel> for ::windows_core::IUnknown {
    fn from(value: CarouselPanel) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CarouselPanel> for ::windows_core::IUnknown {
    fn from(value: &CarouselPanel) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for CarouselPanel {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a CarouselPanel {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<CarouselPanel> for ::windows_core::IInspectable {
    fn from(value: CarouselPanel) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CarouselPanel> for ::windows_core::IInspectable {
    fn from(value: &CarouselPanel) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for CarouselPanel {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a CarouselPanel {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<CarouselPanel> for super::super::super::Composition::IAnimationObject {
    type Error = ::windows_core::Error;
    fn try_from(value: CarouselPanel) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<&CarouselPanel> for super::super::super::Composition::IAnimationObject {
    type Error = ::windows_core::Error;
    fn try_from(value: &CarouselPanel) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows_core::IntoParam<'a, super::super::super::Composition::IAnimationObject> for CarouselPanel {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::super::Composition::IAnimationObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows_core::IntoParam<'a, super::super::super::Composition::IAnimationObject> for &CarouselPanel {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::super::Composition::IAnimationObject> {
        ::core::convert::TryInto::<super::super::super::Composition::IAnimationObject>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<CarouselPanel> for IScrollSnapPointsInfo {
    type Error = ::windows_core::Error;
    fn try_from(value: CarouselPanel) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&CarouselPanel> for IScrollSnapPointsInfo {
    type Error = ::windows_core::Error;
    fn try_from(value: &CarouselPanel) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IScrollSnapPointsInfo> for CarouselPanel {
    fn into_param(self) -> ::windows_core::Param<'a, IScrollSnapPointsInfo> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IScrollSnapPointsInfo> for &CarouselPanel {
    fn into_param(self) -> ::windows_core::Param<'a, IScrollSnapPointsInfo> {
        ::core::convert::TryInto::<IScrollSnapPointsInfo>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<CarouselPanel> for super::super::super::Composition::IVisualElement {
    type Error = ::windows_core::Error;
    fn try_from(value: CarouselPanel) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<&CarouselPanel> for super::super::super::Composition::IVisualElement {
    type Error = ::windows_core::Error;
    fn try_from(value: &CarouselPanel) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows_core::IntoParam<'a, super::super::super::Composition::IVisualElement> for CarouselPanel {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::super::Composition::IVisualElement> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows_core::IntoParam<'a, super::super::super::Composition::IVisualElement> for &CarouselPanel {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::super::Composition::IVisualElement> {
        ::core::convert::TryInto::<super::super::super::Composition::IVisualElement>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::From<CarouselPanel> for super::VirtualizingPanel {
    fn from(value: CarouselPanel) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&CarouselPanel> for super::VirtualizingPanel {
    fn from(value: &CarouselPanel) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::VirtualizingPanel> for CarouselPanel {
    fn into_param(self) -> ::windows_core::Param<'a, super::VirtualizingPanel> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::VirtualizingPanel> for &CarouselPanel {
    fn into_param(self) -> ::windows_core::Param<'a, super::VirtualizingPanel> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::VirtualizingPanel>::into(self))
    }
}
impl ::core::convert::From<CarouselPanel> for super::Panel {
    fn from(value: CarouselPanel) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&CarouselPanel> for super::Panel {
    fn from(value: &CarouselPanel) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::Panel> for CarouselPanel {
    fn into_param(self) -> ::windows_core::Param<'a, super::Panel> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::Panel> for &CarouselPanel {
    fn into_param(self) -> ::windows_core::Param<'a, super::Panel> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::Panel>::into(self))
    }
}
impl ::core::convert::From<CarouselPanel> for super::super::FrameworkElement {
    fn from(value: CarouselPanel) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&CarouselPanel> for super::super::FrameworkElement {
    fn from(value: &CarouselPanel) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::FrameworkElement> for CarouselPanel {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::FrameworkElement> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::FrameworkElement> for &CarouselPanel {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::FrameworkElement> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::super::FrameworkElement>::into(self))
    }
}
impl ::core::convert::From<CarouselPanel> for super::super::UIElement {
    fn from(value: CarouselPanel) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&CarouselPanel> for super::super::UIElement {
    fn from(value: &CarouselPanel) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::UIElement> for CarouselPanel {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::UIElement> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::UIElement> for &CarouselPanel {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::UIElement> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::super::UIElement>::into(self))
    }
}
impl ::core::convert::From<CarouselPanel> for super::super::DependencyObject {
    fn from(value: CarouselPanel) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&CarouselPanel> for super::super::DependencyObject {
    fn from(value: &CarouselPanel) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::DependencyObject> for CarouselPanel {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::DependencyObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::DependencyObject> for &CarouselPanel {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::DependencyObject> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for CarouselPanel {}
unsafe impl ::core::marker::Sync for CarouselPanel {}
#[repr(transparent)]
pub struct ColorPickerSlider(::windows_core::IUnknown);
impl ColorPickerSlider {
    pub fn ColorChannel(&self) -> ::windows_core::Result<super::ColorPickerHsvChannel> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::ColorPickerHsvChannel>::zeroed();
            (::windows_core::Interface::vtable(this).ColorChannel)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::ColorPickerHsvChannel>(result__)
        }
    }
    pub fn SetColorChannel(&self, value: super::ColorPickerHsvChannel) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetColorChannel)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn new() -> ::windows_core::Result<ColorPickerSlider> {
        Self::IColorPickerSliderFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateInstance)(::windows_core::Interface::as_raw(this), ::core::ptr::null_mut(), &mut ::core::option::Option::<::windows_core::IInspectable>::None as *mut _ as _, result__.as_mut_ptr()).from_abi::<ColorPickerSlider>(result__)
        })
    }
    pub fn compose<T: ::windows_core::Compose>(compose: T) -> ::windows_core::Result<ColorPickerSlider> {
        Self::IColorPickerSliderFactory(|this| unsafe {
            let (derived__, base__) = ::windows_core::Compose::compose(compose);
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateInstance)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(&derived__), base__ as *mut _ as _, result__.as_mut_ptr()).from_abi::<ColorPickerSlider>(result__)
        })
    }
    pub fn ColorChannelProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::IColorPickerSliderStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ColorChannelProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn IColorPickerSliderFactory<R, F: FnOnce(&IColorPickerSliderFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<ColorPickerSlider, IColorPickerSliderFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IColorPickerSliderStatics<R, F: FnOnce(&IColorPickerSliderStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<ColorPickerSlider, IColorPickerSliderStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for ColorPickerSlider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ColorPickerSlider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ColorPickerSlider {}
impl ::core::fmt::Debug for ColorPickerSlider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ColorPickerSlider").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ColorPickerSlider {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Controls.Primitives.ColorPickerSlider;{94394d83-e0df-4c5f-bbcd-8155f4020440})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for ColorPickerSlider {
    type Vtable = IColorPickerSlider_Vtbl;
    const IID: ::windows_core::GUID = <IColorPickerSlider as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for ColorPickerSlider {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.ColorPickerSlider";
}
impl ::core::convert::From<ColorPickerSlider> for ::windows_core::IUnknown {
    fn from(value: ColorPickerSlider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ColorPickerSlider> for ::windows_core::IUnknown {
    fn from(value: &ColorPickerSlider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ColorPickerSlider {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ColorPickerSlider {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ColorPickerSlider> for ::windows_core::IInspectable {
    fn from(value: ColorPickerSlider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ColorPickerSlider> for ::windows_core::IInspectable {
    fn from(value: &ColorPickerSlider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for ColorPickerSlider {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a ColorPickerSlider {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<ColorPickerSlider> for super::super::super::Composition::IAnimationObject {
    type Error = ::windows_core::Error;
    fn try_from(value: ColorPickerSlider) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<&ColorPickerSlider> for super::super::super::Composition::IAnimationObject {
    type Error = ::windows_core::Error;
    fn try_from(value: &ColorPickerSlider) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows_core::IntoParam<'a, super::super::super::Composition::IAnimationObject> for ColorPickerSlider {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::super::Composition::IAnimationObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows_core::IntoParam<'a, super::super::super::Composition::IAnimationObject> for &ColorPickerSlider {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::super::Composition::IAnimationObject> {
        ::core::convert::TryInto::<super::super::super::Composition::IAnimationObject>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<ColorPickerSlider> for super::super::super::Composition::IVisualElement {
    type Error = ::windows_core::Error;
    fn try_from(value: ColorPickerSlider) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<&ColorPickerSlider> for super::super::super::Composition::IVisualElement {
    type Error = ::windows_core::Error;
    fn try_from(value: &ColorPickerSlider) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows_core::IntoParam<'a, super::super::super::Composition::IVisualElement> for ColorPickerSlider {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::super::Composition::IVisualElement> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows_core::IntoParam<'a, super::super::super::Composition::IVisualElement> for &ColorPickerSlider {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::super::Composition::IVisualElement> {
        ::core::convert::TryInto::<super::super::super::Composition::IVisualElement>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::From<ColorPickerSlider> for super::Slider {
    fn from(value: ColorPickerSlider) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&ColorPickerSlider> for super::Slider {
    fn from(value: &ColorPickerSlider) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::Slider> for ColorPickerSlider {
    fn into_param(self) -> ::windows_core::Param<'a, super::Slider> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::Slider> for &ColorPickerSlider {
    fn into_param(self) -> ::windows_core::Param<'a, super::Slider> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::Slider>::into(self))
    }
}
impl ::core::convert::From<ColorPickerSlider> for RangeBase {
    fn from(value: ColorPickerSlider) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&ColorPickerSlider> for RangeBase {
    fn from(value: &ColorPickerSlider) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, RangeBase> for ColorPickerSlider {
    fn into_param(self) -> ::windows_core::Param<'a, RangeBase> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, RangeBase> for &ColorPickerSlider {
    fn into_param(self) -> ::windows_core::Param<'a, RangeBase> {
        ::windows_core::Param::Owned(::core::convert::Into::<RangeBase>::into(self))
    }
}
impl ::core::convert::From<ColorPickerSlider> for super::Control {
    fn from(value: ColorPickerSlider) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&ColorPickerSlider> for super::Control {
    fn from(value: &ColorPickerSlider) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::Control> for ColorPickerSlider {
    fn into_param(self) -> ::windows_core::Param<'a, super::Control> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::Control> for &ColorPickerSlider {
    fn into_param(self) -> ::windows_core::Param<'a, super::Control> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::Control>::into(self))
    }
}
impl ::core::convert::From<ColorPickerSlider> for super::super::FrameworkElement {
    fn from(value: ColorPickerSlider) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&ColorPickerSlider> for super::super::FrameworkElement {
    fn from(value: &ColorPickerSlider) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::FrameworkElement> for ColorPickerSlider {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::FrameworkElement> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::FrameworkElement> for &ColorPickerSlider {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::FrameworkElement> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::super::FrameworkElement>::into(self))
    }
}
impl ::core::convert::From<ColorPickerSlider> for super::super::UIElement {
    fn from(value: ColorPickerSlider) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&ColorPickerSlider> for super::super::UIElement {
    fn from(value: &ColorPickerSlider) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::UIElement> for ColorPickerSlider {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::UIElement> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::UIElement> for &ColorPickerSlider {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::UIElement> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::super::UIElement>::into(self))
    }
}
impl ::core::convert::From<ColorPickerSlider> for super::super::DependencyObject {
    fn from(value: ColorPickerSlider) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&ColorPickerSlider> for super::super::DependencyObject {
    fn from(value: &ColorPickerSlider) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::DependencyObject> for ColorPickerSlider {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::DependencyObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::DependencyObject> for &ColorPickerSlider {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::DependencyObject> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for ColorPickerSlider {}
unsafe impl ::core::marker::Sync for ColorPickerSlider {}
#[repr(transparent)]
pub struct ColorSpectrum(::windows_core::IUnknown);
impl ColorSpectrum {
    pub fn Color(&self) -> ::windows_core::Result<super::super::super::Color> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::super::Color>::zeroed();
            (::windows_core::Interface::vtable(this).Color)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Color>(result__)
        }
    }
    pub fn SetColor<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Color>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetColor)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn HsvColor(&self) -> ::windows_core::Result<::winrt_foundation::Numerics::Vector4> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::Numerics::Vector4>::zeroed();
            (::windows_core::Interface::vtable(this).HsvColor)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Numerics::Vector4>(result__)
        }
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn SetHsvColor<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Numerics::Vector4>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetHsvColor)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn MinHue(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows_core::Interface::vtable(this).MinHue)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    pub fn SetMinHue(&self, value: i32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetMinHue)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn MaxHue(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows_core::Interface::vtable(this).MaxHue)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    pub fn SetMaxHue(&self, value: i32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetMaxHue)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn MinSaturation(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows_core::Interface::vtable(this).MinSaturation)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    pub fn SetMinSaturation(&self, value: i32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetMinSaturation)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn MaxSaturation(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows_core::Interface::vtable(this).MaxSaturation)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    pub fn SetMaxSaturation(&self, value: i32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetMaxSaturation)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn MinValue(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows_core::Interface::vtable(this).MinValue)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    pub fn SetMinValue(&self, value: i32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetMinValue)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn MaxValue(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows_core::Interface::vtable(this).MaxValue)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    pub fn SetMaxValue(&self, value: i32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetMaxValue)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Shape(&self) -> ::windows_core::Result<super::ColorSpectrumShape> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::ColorSpectrumShape>::zeroed();
            (::windows_core::Interface::vtable(this).Shape)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::ColorSpectrumShape>(result__)
        }
    }
    pub fn SetShape(&self, value: super::ColorSpectrumShape) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetShape)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Components(&self) -> ::windows_core::Result<super::ColorSpectrumComponents> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::ColorSpectrumComponents>::zeroed();
            (::windows_core::Interface::vtable(this).Components)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::ColorSpectrumComponents>(result__)
        }
    }
    pub fn SetComponents(&self, value: super::ColorSpectrumComponents) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetComponents)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ColorChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<ColorSpectrum, super::ColorChangedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).ColorChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveColorChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveColorChanged)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn new() -> ::windows_core::Result<ColorSpectrum> {
        Self::IColorSpectrumFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateInstance)(::windows_core::Interface::as_raw(this), ::core::ptr::null_mut(), &mut ::core::option::Option::<::windows_core::IInspectable>::None as *mut _ as _, result__.as_mut_ptr()).from_abi::<ColorSpectrum>(result__)
        })
    }
    pub fn compose<T: ::windows_core::Compose>(compose: T) -> ::windows_core::Result<ColorSpectrum> {
        Self::IColorSpectrumFactory(|this| unsafe {
            let (derived__, base__) = ::windows_core::Compose::compose(compose);
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateInstance)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(&derived__), base__ as *mut _ as _, result__.as_mut_ptr()).from_abi::<ColorSpectrum>(result__)
        })
    }
    pub fn ColorProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::IColorSpectrumStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ColorProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn HsvColorProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::IColorSpectrumStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).HsvColorProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn MinHueProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::IColorSpectrumStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).MinHueProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn MaxHueProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::IColorSpectrumStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).MaxHueProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn MinSaturationProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::IColorSpectrumStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).MinSaturationProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn MaxSaturationProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::IColorSpectrumStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).MaxSaturationProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn MinValueProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::IColorSpectrumStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).MinValueProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn MaxValueProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::IColorSpectrumStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).MaxValueProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn ShapeProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::IColorSpectrumStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ShapeProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn ComponentsProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::IColorSpectrumStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ComponentsProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn IColorSpectrumFactory<R, F: FnOnce(&IColorSpectrumFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<ColorSpectrum, IColorSpectrumFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IColorSpectrumStatics<R, F: FnOnce(&IColorSpectrumStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<ColorSpectrum, IColorSpectrumStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for ColorSpectrum {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ColorSpectrum {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ColorSpectrum {}
impl ::core::fmt::Debug for ColorSpectrum {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ColorSpectrum").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ColorSpectrum {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Controls.Primitives.ColorSpectrum;{ce46f271-f509-4f98-8288-e4942fb385df})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for ColorSpectrum {
    type Vtable = IColorSpectrum_Vtbl;
    const IID: ::windows_core::GUID = <IColorSpectrum as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for ColorSpectrum {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.ColorSpectrum";
}
impl ::core::convert::From<ColorSpectrum> for ::windows_core::IUnknown {
    fn from(value: ColorSpectrum) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ColorSpectrum> for ::windows_core::IUnknown {
    fn from(value: &ColorSpectrum) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ColorSpectrum {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ColorSpectrum {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ColorSpectrum> for ::windows_core::IInspectable {
    fn from(value: ColorSpectrum) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ColorSpectrum> for ::windows_core::IInspectable {
    fn from(value: &ColorSpectrum) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for ColorSpectrum {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a ColorSpectrum {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<ColorSpectrum> for super::super::super::Composition::IAnimationObject {
    type Error = ::windows_core::Error;
    fn try_from(value: ColorSpectrum) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<&ColorSpectrum> for super::super::super::Composition::IAnimationObject {
    type Error = ::windows_core::Error;
    fn try_from(value: &ColorSpectrum) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows_core::IntoParam<'a, super::super::super::Composition::IAnimationObject> for ColorSpectrum {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::super::Composition::IAnimationObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows_core::IntoParam<'a, super::super::super::Composition::IAnimationObject> for &ColorSpectrum {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::super::Composition::IAnimationObject> {
        ::core::convert::TryInto::<super::super::super::Composition::IAnimationObject>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<ColorSpectrum> for super::super::super::Composition::IVisualElement {
    type Error = ::windows_core::Error;
    fn try_from(value: ColorSpectrum) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<&ColorSpectrum> for super::super::super::Composition::IVisualElement {
    type Error = ::windows_core::Error;
    fn try_from(value: &ColorSpectrum) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows_core::IntoParam<'a, super::super::super::Composition::IVisualElement> for ColorSpectrum {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::super::Composition::IVisualElement> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows_core::IntoParam<'a, super::super::super::Composition::IVisualElement> for &ColorSpectrum {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::super::Composition::IVisualElement> {
        ::core::convert::TryInto::<super::super::super::Composition::IVisualElement>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::From<ColorSpectrum> for super::Control {
    fn from(value: ColorSpectrum) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&ColorSpectrum> for super::Control {
    fn from(value: &ColorSpectrum) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::Control> for ColorSpectrum {
    fn into_param(self) -> ::windows_core::Param<'a, super::Control> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::Control> for &ColorSpectrum {
    fn into_param(self) -> ::windows_core::Param<'a, super::Control> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::Control>::into(self))
    }
}
impl ::core::convert::From<ColorSpectrum> for super::super::FrameworkElement {
    fn from(value: ColorSpectrum) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&ColorSpectrum> for super::super::FrameworkElement {
    fn from(value: &ColorSpectrum) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::FrameworkElement> for ColorSpectrum {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::FrameworkElement> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::FrameworkElement> for &ColorSpectrum {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::FrameworkElement> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::super::FrameworkElement>::into(self))
    }
}
impl ::core::convert::From<ColorSpectrum> for super::super::UIElement {
    fn from(value: ColorSpectrum) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&ColorSpectrum> for super::super::UIElement {
    fn from(value: &ColorSpectrum) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::UIElement> for ColorSpectrum {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::UIElement> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::UIElement> for &ColorSpectrum {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::UIElement> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::super::UIElement>::into(self))
    }
}
impl ::core::convert::From<ColorSpectrum> for super::super::DependencyObject {
    fn from(value: ColorSpectrum) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&ColorSpectrum> for super::super::DependencyObject {
    fn from(value: &ColorSpectrum) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::DependencyObject> for ColorSpectrum {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::DependencyObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::DependencyObject> for &ColorSpectrum {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::DependencyObject> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for ColorSpectrum {}
unsafe impl ::core::marker::Sync for ColorSpectrum {}
#[repr(transparent)]
pub struct ComboBoxTemplateSettings(::windows_core::IUnknown);
impl ComboBoxTemplateSettings {
    pub fn DropDownOpenedHeight(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).DropDownOpenedHeight)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn DropDownClosedHeight(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).DropDownClosedHeight)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn DropDownOffset(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).DropDownOffset)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn SelectedItemDirection(&self) -> ::windows_core::Result<AnimationDirection> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<AnimationDirection>::zeroed();
            (::windows_core::Interface::vtable(this).SelectedItemDirection)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AnimationDirection>(result__)
        }
    }
    pub fn DropDownContentMinWidth(&self) -> ::windows_core::Result<f64> {
        let this = &::windows_core::Interface::cast::<IComboBoxTemplateSettings2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).DropDownContentMinWidth)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
}
impl ::core::clone::Clone for ComboBoxTemplateSettings {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ComboBoxTemplateSettings {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ComboBoxTemplateSettings {}
impl ::core::fmt::Debug for ComboBoxTemplateSettings {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ComboBoxTemplateSettings").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ComboBoxTemplateSettings {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Controls.Primitives.ComboBoxTemplateSettings;{83285c4e-17f6-4aa3-b61b-e87c718604ea})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for ComboBoxTemplateSettings {
    type Vtable = IComboBoxTemplateSettings_Vtbl;
    const IID: ::windows_core::GUID = <IComboBoxTemplateSettings as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for ComboBoxTemplateSettings {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.ComboBoxTemplateSettings";
}
impl ::core::convert::From<ComboBoxTemplateSettings> for ::windows_core::IUnknown {
    fn from(value: ComboBoxTemplateSettings) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ComboBoxTemplateSettings> for ::windows_core::IUnknown {
    fn from(value: &ComboBoxTemplateSettings) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ComboBoxTemplateSettings {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ComboBoxTemplateSettings {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ComboBoxTemplateSettings> for ::windows_core::IInspectable {
    fn from(value: ComboBoxTemplateSettings) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ComboBoxTemplateSettings> for ::windows_core::IInspectable {
    fn from(value: &ComboBoxTemplateSettings) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for ComboBoxTemplateSettings {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a ComboBoxTemplateSettings {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ComboBoxTemplateSettings> for super::super::DependencyObject {
    fn from(value: ComboBoxTemplateSettings) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&ComboBoxTemplateSettings> for super::super::DependencyObject {
    fn from(value: &ComboBoxTemplateSettings) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::DependencyObject> for ComboBoxTemplateSettings {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::DependencyObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::DependencyObject> for &ComboBoxTemplateSettings {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::DependencyObject> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for ComboBoxTemplateSettings {}
unsafe impl ::core::marker::Sync for ComboBoxTemplateSettings {}
#[repr(transparent)]
pub struct CommandBarFlyoutCommandBar(::windows_core::IUnknown);
impl CommandBarFlyoutCommandBar {
    pub fn FlyoutTemplateSettings(&self) -> ::windows_core::Result<CommandBarFlyoutCommandBarTemplateSettings> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FlyoutTemplateSettings)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<CommandBarFlyoutCommandBarTemplateSettings>(result__)
        }
    }
    pub fn new() -> ::windows_core::Result<CommandBarFlyoutCommandBar> {
        Self::ICommandBarFlyoutCommandBarFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateInstance)(::windows_core::Interface::as_raw(this), ::core::ptr::null_mut(), &mut ::core::option::Option::<::windows_core::IInspectable>::None as *mut _ as _, result__.as_mut_ptr()).from_abi::<CommandBarFlyoutCommandBar>(result__)
        })
    }
    pub fn compose<T: ::windows_core::Compose>(compose: T) -> ::windows_core::Result<CommandBarFlyoutCommandBar> {
        Self::ICommandBarFlyoutCommandBarFactory(|this| unsafe {
            let (derived__, base__) = ::windows_core::Compose::compose(compose);
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateInstance)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(&derived__), base__ as *mut _ as _, result__.as_mut_ptr()).from_abi::<CommandBarFlyoutCommandBar>(result__)
        })
    }
    pub fn ICommandBarFlyoutCommandBarFactory<R, F: FnOnce(&ICommandBarFlyoutCommandBarFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<CommandBarFlyoutCommandBar, ICommandBarFlyoutCommandBarFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for CommandBarFlyoutCommandBar {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CommandBarFlyoutCommandBar {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CommandBarFlyoutCommandBar {}
impl ::core::fmt::Debug for CommandBarFlyoutCommandBar {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CommandBarFlyoutCommandBar").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for CommandBarFlyoutCommandBar {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Controls.Primitives.CommandBarFlyoutCommandBar;{14146e7c-38c4-55c4-b706-ce18f6061e7e})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for CommandBarFlyoutCommandBar {
    type Vtable = ICommandBarFlyoutCommandBar_Vtbl;
    const IID: ::windows_core::GUID = <ICommandBarFlyoutCommandBar as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for CommandBarFlyoutCommandBar {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.CommandBarFlyoutCommandBar";
}
impl ::core::convert::From<CommandBarFlyoutCommandBar> for ::windows_core::IUnknown {
    fn from(value: CommandBarFlyoutCommandBar) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CommandBarFlyoutCommandBar> for ::windows_core::IUnknown {
    fn from(value: &CommandBarFlyoutCommandBar) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for CommandBarFlyoutCommandBar {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a CommandBarFlyoutCommandBar {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<CommandBarFlyoutCommandBar> for ::windows_core::IInspectable {
    fn from(value: CommandBarFlyoutCommandBar) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CommandBarFlyoutCommandBar> for ::windows_core::IInspectable {
    fn from(value: &CommandBarFlyoutCommandBar) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for CommandBarFlyoutCommandBar {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a CommandBarFlyoutCommandBar {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<CommandBarFlyoutCommandBar> for super::super::super::Composition::IAnimationObject {
    type Error = ::windows_core::Error;
    fn try_from(value: CommandBarFlyoutCommandBar) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<&CommandBarFlyoutCommandBar> for super::super::super::Composition::IAnimationObject {
    type Error = ::windows_core::Error;
    fn try_from(value: &CommandBarFlyoutCommandBar) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows_core::IntoParam<'a, super::super::super::Composition::IAnimationObject> for CommandBarFlyoutCommandBar {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::super::Composition::IAnimationObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows_core::IntoParam<'a, super::super::super::Composition::IAnimationObject> for &CommandBarFlyoutCommandBar {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::super::Composition::IAnimationObject> {
        ::core::convert::TryInto::<super::super::super::Composition::IAnimationObject>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<CommandBarFlyoutCommandBar> for super::super::super::Composition::IVisualElement {
    type Error = ::windows_core::Error;
    fn try_from(value: CommandBarFlyoutCommandBar) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<&CommandBarFlyoutCommandBar> for super::super::super::Composition::IVisualElement {
    type Error = ::windows_core::Error;
    fn try_from(value: &CommandBarFlyoutCommandBar) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows_core::IntoParam<'a, super::super::super::Composition::IVisualElement> for CommandBarFlyoutCommandBar {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::super::Composition::IVisualElement> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows_core::IntoParam<'a, super::super::super::Composition::IVisualElement> for &CommandBarFlyoutCommandBar {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::super::Composition::IVisualElement> {
        ::core::convert::TryInto::<super::super::super::Composition::IVisualElement>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::From<CommandBarFlyoutCommandBar> for super::CommandBar {
    fn from(value: CommandBarFlyoutCommandBar) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&CommandBarFlyoutCommandBar> for super::CommandBar {
    fn from(value: &CommandBarFlyoutCommandBar) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::CommandBar> for CommandBarFlyoutCommandBar {
    fn into_param(self) -> ::windows_core::Param<'a, super::CommandBar> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::CommandBar> for &CommandBarFlyoutCommandBar {
    fn into_param(self) -> ::windows_core::Param<'a, super::CommandBar> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::CommandBar>::into(self))
    }
}
impl ::core::convert::From<CommandBarFlyoutCommandBar> for super::AppBar {
    fn from(value: CommandBarFlyoutCommandBar) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&CommandBarFlyoutCommandBar> for super::AppBar {
    fn from(value: &CommandBarFlyoutCommandBar) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::AppBar> for CommandBarFlyoutCommandBar {
    fn into_param(self) -> ::windows_core::Param<'a, super::AppBar> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::AppBar> for &CommandBarFlyoutCommandBar {
    fn into_param(self) -> ::windows_core::Param<'a, super::AppBar> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::AppBar>::into(self))
    }
}
impl ::core::convert::From<CommandBarFlyoutCommandBar> for super::ContentControl {
    fn from(value: CommandBarFlyoutCommandBar) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&CommandBarFlyoutCommandBar> for super::ContentControl {
    fn from(value: &CommandBarFlyoutCommandBar) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::ContentControl> for CommandBarFlyoutCommandBar {
    fn into_param(self) -> ::windows_core::Param<'a, super::ContentControl> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::ContentControl> for &CommandBarFlyoutCommandBar {
    fn into_param(self) -> ::windows_core::Param<'a, super::ContentControl> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::ContentControl>::into(self))
    }
}
impl ::core::convert::From<CommandBarFlyoutCommandBar> for super::Control {
    fn from(value: CommandBarFlyoutCommandBar) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&CommandBarFlyoutCommandBar> for super::Control {
    fn from(value: &CommandBarFlyoutCommandBar) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::Control> for CommandBarFlyoutCommandBar {
    fn into_param(self) -> ::windows_core::Param<'a, super::Control> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::Control> for &CommandBarFlyoutCommandBar {
    fn into_param(self) -> ::windows_core::Param<'a, super::Control> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::Control>::into(self))
    }
}
impl ::core::convert::From<CommandBarFlyoutCommandBar> for super::super::FrameworkElement {
    fn from(value: CommandBarFlyoutCommandBar) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&CommandBarFlyoutCommandBar> for super::super::FrameworkElement {
    fn from(value: &CommandBarFlyoutCommandBar) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::FrameworkElement> for CommandBarFlyoutCommandBar {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::FrameworkElement> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::FrameworkElement> for &CommandBarFlyoutCommandBar {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::FrameworkElement> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::super::FrameworkElement>::into(self))
    }
}
impl ::core::convert::From<CommandBarFlyoutCommandBar> for super::super::UIElement {
    fn from(value: CommandBarFlyoutCommandBar) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&CommandBarFlyoutCommandBar> for super::super::UIElement {
    fn from(value: &CommandBarFlyoutCommandBar) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::UIElement> for CommandBarFlyoutCommandBar {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::UIElement> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::UIElement> for &CommandBarFlyoutCommandBar {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::UIElement> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::super::UIElement>::into(self))
    }
}
impl ::core::convert::From<CommandBarFlyoutCommandBar> for super::super::DependencyObject {
    fn from(value: CommandBarFlyoutCommandBar) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&CommandBarFlyoutCommandBar> for super::super::DependencyObject {
    fn from(value: &CommandBarFlyoutCommandBar) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::DependencyObject> for CommandBarFlyoutCommandBar {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::DependencyObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::DependencyObject> for &CommandBarFlyoutCommandBar {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::DependencyObject> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for CommandBarFlyoutCommandBar {}
unsafe impl ::core::marker::Sync for CommandBarFlyoutCommandBar {}
#[repr(transparent)]
pub struct CommandBarFlyoutCommandBarTemplateSettings(::windows_core::IUnknown);
impl CommandBarFlyoutCommandBarTemplateSettings {
    pub fn OpenAnimationStartPosition(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).OpenAnimationStartPosition)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn OpenAnimationEndPosition(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).OpenAnimationEndPosition)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn CloseAnimationEndPosition(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).CloseAnimationEndPosition)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn CurrentWidth(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).CurrentWidth)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn ExpandedWidth(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).ExpandedWidth)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn WidthExpansionDelta(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).WidthExpansionDelta)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn WidthExpansionAnimationStartPosition(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).WidthExpansionAnimationStartPosition)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn WidthExpansionAnimationEndPosition(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).WidthExpansionAnimationEndPosition)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn WidthExpansionMoreButtonAnimationStartPosition(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).WidthExpansionMoreButtonAnimationStartPosition)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn WidthExpansionMoreButtonAnimationEndPosition(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).WidthExpansionMoreButtonAnimationEndPosition)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn ExpandUpOverflowVerticalPosition(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).ExpandUpOverflowVerticalPosition)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn ExpandDownOverflowVerticalPosition(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).ExpandDownOverflowVerticalPosition)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn ExpandUpAnimationStartPosition(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).ExpandUpAnimationStartPosition)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn ExpandUpAnimationEndPosition(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).ExpandUpAnimationEndPosition)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn ExpandUpAnimationHoldPosition(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).ExpandUpAnimationHoldPosition)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn ExpandDownAnimationStartPosition(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).ExpandDownAnimationStartPosition)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn ExpandDownAnimationEndPosition(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).ExpandDownAnimationEndPosition)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn ExpandDownAnimationHoldPosition(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).ExpandDownAnimationHoldPosition)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn ContentClipRect(&self) -> ::windows_core::Result<::winrt_foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::Rect>::zeroed();
            (::windows_core::Interface::vtable(this).ContentClipRect)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Rect>(result__)
        }
    }
    pub fn OverflowContentClipRect(&self) -> ::windows_core::Result<::winrt_foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::Rect>::zeroed();
            (::windows_core::Interface::vtable(this).OverflowContentClipRect)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Rect>(result__)
        }
    }
}
impl ::core::clone::Clone for CommandBarFlyoutCommandBarTemplateSettings {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CommandBarFlyoutCommandBarTemplateSettings {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CommandBarFlyoutCommandBarTemplateSettings {}
impl ::core::fmt::Debug for CommandBarFlyoutCommandBarTemplateSettings {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CommandBarFlyoutCommandBarTemplateSettings").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for CommandBarFlyoutCommandBarTemplateSettings {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Controls.Primitives.CommandBarFlyoutCommandBarTemplateSettings;{47642c44-26ff-5d14-9cfc-77dc64f3a447})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for CommandBarFlyoutCommandBarTemplateSettings {
    type Vtable = ICommandBarFlyoutCommandBarTemplateSettings_Vtbl;
    const IID: ::windows_core::GUID = <ICommandBarFlyoutCommandBarTemplateSettings as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for CommandBarFlyoutCommandBarTemplateSettings {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.CommandBarFlyoutCommandBarTemplateSettings";
}
impl ::core::convert::From<CommandBarFlyoutCommandBarTemplateSettings> for ::windows_core::IUnknown {
    fn from(value: CommandBarFlyoutCommandBarTemplateSettings) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CommandBarFlyoutCommandBarTemplateSettings> for ::windows_core::IUnknown {
    fn from(value: &CommandBarFlyoutCommandBarTemplateSettings) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for CommandBarFlyoutCommandBarTemplateSettings {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a CommandBarFlyoutCommandBarTemplateSettings {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<CommandBarFlyoutCommandBarTemplateSettings> for ::windows_core::IInspectable {
    fn from(value: CommandBarFlyoutCommandBarTemplateSettings) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CommandBarFlyoutCommandBarTemplateSettings> for ::windows_core::IInspectable {
    fn from(value: &CommandBarFlyoutCommandBarTemplateSettings) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for CommandBarFlyoutCommandBarTemplateSettings {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a CommandBarFlyoutCommandBarTemplateSettings {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<CommandBarFlyoutCommandBarTemplateSettings> for super::super::DependencyObject {
    fn from(value: CommandBarFlyoutCommandBarTemplateSettings) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&CommandBarFlyoutCommandBarTemplateSettings> for super::super::DependencyObject {
    fn from(value: &CommandBarFlyoutCommandBarTemplateSettings) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::DependencyObject> for CommandBarFlyoutCommandBarTemplateSettings {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::DependencyObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::DependencyObject> for &CommandBarFlyoutCommandBarTemplateSettings {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::DependencyObject> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for CommandBarFlyoutCommandBarTemplateSettings {}
unsafe impl ::core::marker::Sync for CommandBarFlyoutCommandBarTemplateSettings {}
#[repr(transparent)]
pub struct CommandBarTemplateSettings(::windows_core::IUnknown);
impl CommandBarTemplateSettings {
    pub fn ContentHeight(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).ContentHeight)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn OverflowContentClipRect(&self) -> ::windows_core::Result<::winrt_foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::Rect>::zeroed();
            (::windows_core::Interface::vtable(this).OverflowContentClipRect)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Rect>(result__)
        }
    }
    pub fn OverflowContentMinWidth(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).OverflowContentMinWidth)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn OverflowContentMaxHeight(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).OverflowContentMaxHeight)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn OverflowContentHorizontalOffset(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).OverflowContentHorizontalOffset)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn OverflowContentHeight(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).OverflowContentHeight)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn NegativeOverflowContentHeight(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).NegativeOverflowContentHeight)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn OverflowContentMaxWidth(&self) -> ::windows_core::Result<f64> {
        let this = &::windows_core::Interface::cast::<ICommandBarTemplateSettings2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).OverflowContentMaxWidth)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn EffectiveOverflowButtonVisibility(&self) -> ::windows_core::Result<super::super::Visibility> {
        let this = &::windows_core::Interface::cast::<ICommandBarTemplateSettings3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::Visibility>::zeroed();
            (::windows_core::Interface::vtable(this).EffectiveOverflowButtonVisibility)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Visibility>(result__)
        }
    }
    pub fn OverflowContentCompactYTranslation(&self) -> ::windows_core::Result<f64> {
        let this = &::windows_core::Interface::cast::<ICommandBarTemplateSettings4>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).OverflowContentCompactYTranslation)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn OverflowContentMinimalYTranslation(&self) -> ::windows_core::Result<f64> {
        let this = &::windows_core::Interface::cast::<ICommandBarTemplateSettings4>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).OverflowContentMinimalYTranslation)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn OverflowContentHiddenYTranslation(&self) -> ::windows_core::Result<f64> {
        let this = &::windows_core::Interface::cast::<ICommandBarTemplateSettings4>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).OverflowContentHiddenYTranslation)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
}
impl ::core::clone::Clone for CommandBarTemplateSettings {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CommandBarTemplateSettings {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CommandBarTemplateSettings {}
impl ::core::fmt::Debug for CommandBarTemplateSettings {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CommandBarTemplateSettings").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for CommandBarTemplateSettings {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Controls.Primitives.CommandBarTemplateSettings;{61c8f92c-05aa-414a-a2ae-482c5a46c08e})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for CommandBarTemplateSettings {
    type Vtable = ICommandBarTemplateSettings_Vtbl;
    const IID: ::windows_core::GUID = <ICommandBarTemplateSettings as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for CommandBarTemplateSettings {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.CommandBarTemplateSettings";
}
impl ::core::convert::From<CommandBarTemplateSettings> for ::windows_core::IUnknown {
    fn from(value: CommandBarTemplateSettings) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CommandBarTemplateSettings> for ::windows_core::IUnknown {
    fn from(value: &CommandBarTemplateSettings) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for CommandBarTemplateSettings {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a CommandBarTemplateSettings {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<CommandBarTemplateSettings> for ::windows_core::IInspectable {
    fn from(value: CommandBarTemplateSettings) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CommandBarTemplateSettings> for ::windows_core::IInspectable {
    fn from(value: &CommandBarTemplateSettings) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for CommandBarTemplateSettings {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a CommandBarTemplateSettings {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<CommandBarTemplateSettings> for super::super::DependencyObject {
    fn from(value: CommandBarTemplateSettings) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&CommandBarTemplateSettings> for super::super::DependencyObject {
    fn from(value: &CommandBarTemplateSettings) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::DependencyObject> for CommandBarTemplateSettings {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::DependencyObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::DependencyObject> for &CommandBarTemplateSettings {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::DependencyObject> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for CommandBarTemplateSettings {}
unsafe impl ::core::marker::Sync for CommandBarTemplateSettings {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct ComponentResourceLocation(pub i32);
impl ComponentResourceLocation {
    pub const Application: Self = Self(0i32);
    pub const Nested: Self = Self(1i32);
}
impl ::core::marker::Copy for ComponentResourceLocation {}
impl ::core::clone::Clone for ComponentResourceLocation {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ComponentResourceLocation {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for ComponentResourceLocation {
    type Abi = Self;
}
impl ::core::fmt::Debug for ComponentResourceLocation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ComponentResourceLocation").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ComponentResourceLocation {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Controls.Primitives.ComponentResourceLocation;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct DragCompletedEventArgs(::windows_core::IUnknown);
impl DragCompletedEventArgs {
    pub fn HorizontalChange(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).HorizontalChange)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn VerticalChange(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).VerticalChange)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn Canceled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).Canceled)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn CreateInstanceWithHorizontalChangeVerticalChangeAndCanceled(horizontalchange: f64, verticalchange: f64, canceled: bool) -> ::windows_core::Result<DragCompletedEventArgs> {
        Self::IDragCompletedEventArgsFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateInstanceWithHorizontalChangeVerticalChangeAndCanceled)(::windows_core::Interface::as_raw(this), horizontalchange, verticalchange, canceled, ::core::ptr::null_mut(), &mut ::core::option::Option::<::windows_core::IInspectable>::None as *mut _ as _, result__.as_mut_ptr()).from_abi::<DragCompletedEventArgs>(result__)
        })
    }
    pub fn CreateInstanceWithHorizontalChangeVerticalChangeAndCanceled_compose<T: ::windows_core::Compose>(horizontalchange: f64, verticalchange: f64, canceled: bool, compose: T) -> ::windows_core::Result<DragCompletedEventArgs> {
        Self::IDragCompletedEventArgsFactory(|this| unsafe {
            let (derived__, base__) = ::windows_core::Compose::compose(compose);
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateInstanceWithHorizontalChangeVerticalChangeAndCanceled)(::windows_core::Interface::as_raw(this), horizontalchange, verticalchange, canceled, ::core::mem::transmute_copy(&derived__), base__ as *mut _ as _, result__.as_mut_ptr()).from_abi::<DragCompletedEventArgs>(result__)
        })
    }
    pub fn IDragCompletedEventArgsFactory<R, F: FnOnce(&IDragCompletedEventArgsFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<DragCompletedEventArgs, IDragCompletedEventArgsFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for DragCompletedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for DragCompletedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DragCompletedEventArgs {}
impl ::core::fmt::Debug for DragCompletedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DragCompletedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for DragCompletedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Controls.Primitives.DragCompletedEventArgs;{b04f29a1-bd16-48f6-a511-9c2763641331})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for DragCompletedEventArgs {
    type Vtable = IDragCompletedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IDragCompletedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for DragCompletedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.DragCompletedEventArgs";
}
impl ::core::convert::From<DragCompletedEventArgs> for ::windows_core::IUnknown {
    fn from(value: DragCompletedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DragCompletedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &DragCompletedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for DragCompletedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a DragCompletedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<DragCompletedEventArgs> for ::windows_core::IInspectable {
    fn from(value: DragCompletedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DragCompletedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &DragCompletedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for DragCompletedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a DragCompletedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<DragCompletedEventArgs> for super::super::RoutedEventArgs {
    fn from(value: DragCompletedEventArgs) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&DragCompletedEventArgs> for super::super::RoutedEventArgs {
    fn from(value: &DragCompletedEventArgs) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::RoutedEventArgs> for DragCompletedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::RoutedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::RoutedEventArgs> for &DragCompletedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::RoutedEventArgs> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::super::RoutedEventArgs>::into(self))
    }
}
unsafe impl ::core::marker::Send for DragCompletedEventArgs {}
unsafe impl ::core::marker::Sync for DragCompletedEventArgs {}
#[repr(transparent)]
pub struct DragCompletedEventHandler(pub ::windows_core::IUnknown);
impl DragCompletedEventHandler {
    pub fn new<F: FnMut(&::core::option::Option<::windows_core::IInspectable>, &::core::option::Option<DragCompletedEventArgs>) -> ::windows_core::Result<()> + ::core::marker::Send + 'static>(invoke: F) -> Self {
        let com = DragCompletedEventHandlerBox::<F> { vtable: &DragCompletedEventHandlerBox::<F>::VTABLE, count: ::windows_core::RefCount::new(1), invoke };
        unsafe { ::core::mem::transmute(::windows_core::alloc::boxed::Box::new(com)) }
    }
    pub fn Invoke<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::IInspectable>, Param1: ::windows_core::IntoParam<'a, DragCompletedEventArgs>>(&self, sender: Param0, e: Param1) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Invoke)(::windows_core::Interface::as_raw(this), sender.into_param().abi(), e.into_param().abi()).ok() }
    }
}
#[repr(C)]
struct DragCompletedEventHandlerBox<F: FnMut(&::core::option::Option<::windows_core::IInspectable>, &::core::option::Option<DragCompletedEventArgs>) -> ::windows_core::Result<()> + ::core::marker::Send + 'static> {
    vtable: *const DragCompletedEventHandler_Vtbl,
    invoke: F,
    count: ::windows_core::RefCount,
}
impl<F: FnMut(&::core::option::Option<::windows_core::IInspectable>, &::core::option::Option<DragCompletedEventArgs>) -> ::windows_core::Result<()> + ::core::marker::Send + 'static> DragCompletedEventHandlerBox<F> {
    const VTABLE: DragCompletedEventHandler_Vtbl = DragCompletedEventHandler_Vtbl {
        base__: ::windows_core::IUnknownVtbl { QueryInterface: Self::QueryInterface, AddRef: Self::AddRef, Release: Self::Release },
        Invoke: Self::Invoke,
    };
    unsafe extern "system" fn QueryInterface(this: ::windows_core::RawPtr, iid: &::windows_core::GUID, interface: *mut *const ::core::ffi::c_void) -> ::windows_core::HRESULT {
        let this = this as *mut ::windows_core::RawPtr as *mut Self;
        *interface = if iid == &<DragCompletedEventHandler as ::windows_core::Interface>::IID || iid == &<::windows_core::IUnknown as ::windows_core::Interface>::IID || iid == &<::windows_core::IAgileObject as ::windows_core::Interface>::IID { &mut (*this).vtable as *mut _ as _ } else { ::core::ptr::null_mut() };
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
    unsafe extern "system" fn Invoke(this: *mut ::core::ffi::c_void, sender: *mut ::core::ffi::c_void, e: ::windows_core::RawPtr) -> ::windows_core::HRESULT {
        let this = this as *mut ::windows_core::RawPtr as *mut Self;
        ((*this).invoke)(::core::mem::transmute(&sender), ::core::mem::transmute(&e)).into()
    }
}
impl ::core::clone::Clone for DragCompletedEventHandler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for DragCompletedEventHandler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DragCompletedEventHandler {}
impl ::core::fmt::Debug for DragCompletedEventHandler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DragCompletedEventHandler").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for DragCompletedEventHandler {
    type Vtable = DragCompletedEventHandler_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x36b28888_19ac_4b4e_9137_a6cf2b023883);
}
unsafe impl ::windows_core::RuntimeType for DragCompletedEventHandler {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"{36b28888-19ac-4b4e-9137-a6cf2b023883}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct DragCompletedEventHandler_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub Invoke: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sender: *mut ::core::ffi::c_void, e: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct DragDeltaEventArgs(::windows_core::IUnknown);
impl DragDeltaEventArgs {
    pub fn HorizontalChange(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).HorizontalChange)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn VerticalChange(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).VerticalChange)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn CreateInstanceWithHorizontalChangeAndVerticalChange(horizontalchange: f64, verticalchange: f64) -> ::windows_core::Result<DragDeltaEventArgs> {
        Self::IDragDeltaEventArgsFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateInstanceWithHorizontalChangeAndVerticalChange)(::windows_core::Interface::as_raw(this), horizontalchange, verticalchange, ::core::ptr::null_mut(), &mut ::core::option::Option::<::windows_core::IInspectable>::None as *mut _ as _, result__.as_mut_ptr()).from_abi::<DragDeltaEventArgs>(result__)
        })
    }
    pub fn CreateInstanceWithHorizontalChangeAndVerticalChange_compose<T: ::windows_core::Compose>(horizontalchange: f64, verticalchange: f64, compose: T) -> ::windows_core::Result<DragDeltaEventArgs> {
        Self::IDragDeltaEventArgsFactory(|this| unsafe {
            let (derived__, base__) = ::windows_core::Compose::compose(compose);
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateInstanceWithHorizontalChangeAndVerticalChange)(::windows_core::Interface::as_raw(this), horizontalchange, verticalchange, ::core::mem::transmute_copy(&derived__), base__ as *mut _ as _, result__.as_mut_ptr()).from_abi::<DragDeltaEventArgs>(result__)
        })
    }
    pub fn IDragDeltaEventArgsFactory<R, F: FnOnce(&IDragDeltaEventArgsFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<DragDeltaEventArgs, IDragDeltaEventArgsFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for DragDeltaEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for DragDeltaEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DragDeltaEventArgs {}
impl ::core::fmt::Debug for DragDeltaEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DragDeltaEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for DragDeltaEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Controls.Primitives.DragDeltaEventArgs;{2c2dd73c-2806-49fc-aae9-6d792b572b6a})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for DragDeltaEventArgs {
    type Vtable = IDragDeltaEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IDragDeltaEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for DragDeltaEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.DragDeltaEventArgs";
}
impl ::core::convert::From<DragDeltaEventArgs> for ::windows_core::IUnknown {
    fn from(value: DragDeltaEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DragDeltaEventArgs> for ::windows_core::IUnknown {
    fn from(value: &DragDeltaEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for DragDeltaEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a DragDeltaEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<DragDeltaEventArgs> for ::windows_core::IInspectable {
    fn from(value: DragDeltaEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DragDeltaEventArgs> for ::windows_core::IInspectable {
    fn from(value: &DragDeltaEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for DragDeltaEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a DragDeltaEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<DragDeltaEventArgs> for super::super::RoutedEventArgs {
    fn from(value: DragDeltaEventArgs) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&DragDeltaEventArgs> for super::super::RoutedEventArgs {
    fn from(value: &DragDeltaEventArgs) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::RoutedEventArgs> for DragDeltaEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::RoutedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::RoutedEventArgs> for &DragDeltaEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::RoutedEventArgs> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::super::RoutedEventArgs>::into(self))
    }
}
unsafe impl ::core::marker::Send for DragDeltaEventArgs {}
unsafe impl ::core::marker::Sync for DragDeltaEventArgs {}
#[repr(transparent)]
pub struct DragDeltaEventHandler(pub ::windows_core::IUnknown);
impl DragDeltaEventHandler {
    pub fn new<F: FnMut(&::core::option::Option<::windows_core::IInspectable>, &::core::option::Option<DragDeltaEventArgs>) -> ::windows_core::Result<()> + ::core::marker::Send + 'static>(invoke: F) -> Self {
        let com = DragDeltaEventHandlerBox::<F> { vtable: &DragDeltaEventHandlerBox::<F>::VTABLE, count: ::windows_core::RefCount::new(1), invoke };
        unsafe { ::core::mem::transmute(::windows_core::alloc::boxed::Box::new(com)) }
    }
    pub fn Invoke<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::IInspectable>, Param1: ::windows_core::IntoParam<'a, DragDeltaEventArgs>>(&self, sender: Param0, e: Param1) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Invoke)(::windows_core::Interface::as_raw(this), sender.into_param().abi(), e.into_param().abi()).ok() }
    }
}
#[repr(C)]
struct DragDeltaEventHandlerBox<F: FnMut(&::core::option::Option<::windows_core::IInspectable>, &::core::option::Option<DragDeltaEventArgs>) -> ::windows_core::Result<()> + ::core::marker::Send + 'static> {
    vtable: *const DragDeltaEventHandler_Vtbl,
    invoke: F,
    count: ::windows_core::RefCount,
}
impl<F: FnMut(&::core::option::Option<::windows_core::IInspectable>, &::core::option::Option<DragDeltaEventArgs>) -> ::windows_core::Result<()> + ::core::marker::Send + 'static> DragDeltaEventHandlerBox<F> {
    const VTABLE: DragDeltaEventHandler_Vtbl = DragDeltaEventHandler_Vtbl {
        base__: ::windows_core::IUnknownVtbl { QueryInterface: Self::QueryInterface, AddRef: Self::AddRef, Release: Self::Release },
        Invoke: Self::Invoke,
    };
    unsafe extern "system" fn QueryInterface(this: ::windows_core::RawPtr, iid: &::windows_core::GUID, interface: *mut *const ::core::ffi::c_void) -> ::windows_core::HRESULT {
        let this = this as *mut ::windows_core::RawPtr as *mut Self;
        *interface = if iid == &<DragDeltaEventHandler as ::windows_core::Interface>::IID || iid == &<::windows_core::IUnknown as ::windows_core::Interface>::IID || iid == &<::windows_core::IAgileObject as ::windows_core::Interface>::IID { &mut (*this).vtable as *mut _ as _ } else { ::core::ptr::null_mut() };
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
    unsafe extern "system" fn Invoke(this: *mut ::core::ffi::c_void, sender: *mut ::core::ffi::c_void, e: ::windows_core::RawPtr) -> ::windows_core::HRESULT {
        let this = this as *mut ::windows_core::RawPtr as *mut Self;
        ((*this).invoke)(::core::mem::transmute(&sender), ::core::mem::transmute(&e)).into()
    }
}
impl ::core::clone::Clone for DragDeltaEventHandler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for DragDeltaEventHandler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DragDeltaEventHandler {}
impl ::core::fmt::Debug for DragDeltaEventHandler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DragDeltaEventHandler").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for DragDeltaEventHandler {
    type Vtable = DragDeltaEventHandler_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4ac24f9f_ac28_49e9_9189_dccffeb66472);
}
unsafe impl ::windows_core::RuntimeType for DragDeltaEventHandler {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"{4ac24f9f-ac28-49e9-9189-dccffeb66472}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct DragDeltaEventHandler_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub Invoke: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sender: *mut ::core::ffi::c_void, e: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct DragStartedEventArgs(::windows_core::IUnknown);
impl DragStartedEventArgs {
    pub fn HorizontalOffset(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).HorizontalOffset)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn VerticalOffset(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).VerticalOffset)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn CreateInstanceWithHorizontalOffsetAndVerticalOffset(horizontaloffset: f64, verticaloffset: f64) -> ::windows_core::Result<DragStartedEventArgs> {
        Self::IDragStartedEventArgsFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateInstanceWithHorizontalOffsetAndVerticalOffset)(::windows_core::Interface::as_raw(this), horizontaloffset, verticaloffset, ::core::ptr::null_mut(), &mut ::core::option::Option::<::windows_core::IInspectable>::None as *mut _ as _, result__.as_mut_ptr()).from_abi::<DragStartedEventArgs>(result__)
        })
    }
    pub fn CreateInstanceWithHorizontalOffsetAndVerticalOffset_compose<T: ::windows_core::Compose>(horizontaloffset: f64, verticaloffset: f64, compose: T) -> ::windows_core::Result<DragStartedEventArgs> {
        Self::IDragStartedEventArgsFactory(|this| unsafe {
            let (derived__, base__) = ::windows_core::Compose::compose(compose);
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateInstanceWithHorizontalOffsetAndVerticalOffset)(::windows_core::Interface::as_raw(this), horizontaloffset, verticaloffset, ::core::mem::transmute_copy(&derived__), base__ as *mut _ as _, result__.as_mut_ptr()).from_abi::<DragStartedEventArgs>(result__)
        })
    }
    pub fn IDragStartedEventArgsFactory<R, F: FnOnce(&IDragStartedEventArgsFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<DragStartedEventArgs, IDragStartedEventArgsFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for DragStartedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for DragStartedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DragStartedEventArgs {}
impl ::core::fmt::Debug for DragStartedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DragStartedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for DragStartedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Controls.Primitives.DragStartedEventArgs;{9f915dd0-a124-4366-bd85-2408214aeed4})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for DragStartedEventArgs {
    type Vtable = IDragStartedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IDragStartedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for DragStartedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.DragStartedEventArgs";
}
impl ::core::convert::From<DragStartedEventArgs> for ::windows_core::IUnknown {
    fn from(value: DragStartedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DragStartedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &DragStartedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for DragStartedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a DragStartedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<DragStartedEventArgs> for ::windows_core::IInspectable {
    fn from(value: DragStartedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DragStartedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &DragStartedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for DragStartedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a DragStartedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<DragStartedEventArgs> for super::super::RoutedEventArgs {
    fn from(value: DragStartedEventArgs) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&DragStartedEventArgs> for super::super::RoutedEventArgs {
    fn from(value: &DragStartedEventArgs) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::RoutedEventArgs> for DragStartedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::RoutedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::RoutedEventArgs> for &DragStartedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::RoutedEventArgs> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::super::RoutedEventArgs>::into(self))
    }
}
unsafe impl ::core::marker::Send for DragStartedEventArgs {}
unsafe impl ::core::marker::Sync for DragStartedEventArgs {}
#[repr(transparent)]
pub struct DragStartedEventHandler(pub ::windows_core::IUnknown);
impl DragStartedEventHandler {
    pub fn new<F: FnMut(&::core::option::Option<::windows_core::IInspectable>, &::core::option::Option<DragStartedEventArgs>) -> ::windows_core::Result<()> + ::core::marker::Send + 'static>(invoke: F) -> Self {
        let com = DragStartedEventHandlerBox::<F> { vtable: &DragStartedEventHandlerBox::<F>::VTABLE, count: ::windows_core::RefCount::new(1), invoke };
        unsafe { ::core::mem::transmute(::windows_core::alloc::boxed::Box::new(com)) }
    }
    pub fn Invoke<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::IInspectable>, Param1: ::windows_core::IntoParam<'a, DragStartedEventArgs>>(&self, sender: Param0, e: Param1) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Invoke)(::windows_core::Interface::as_raw(this), sender.into_param().abi(), e.into_param().abi()).ok() }
    }
}
#[repr(C)]
struct DragStartedEventHandlerBox<F: FnMut(&::core::option::Option<::windows_core::IInspectable>, &::core::option::Option<DragStartedEventArgs>) -> ::windows_core::Result<()> + ::core::marker::Send + 'static> {
    vtable: *const DragStartedEventHandler_Vtbl,
    invoke: F,
    count: ::windows_core::RefCount,
}
impl<F: FnMut(&::core::option::Option<::windows_core::IInspectable>, &::core::option::Option<DragStartedEventArgs>) -> ::windows_core::Result<()> + ::core::marker::Send + 'static> DragStartedEventHandlerBox<F> {
    const VTABLE: DragStartedEventHandler_Vtbl = DragStartedEventHandler_Vtbl {
        base__: ::windows_core::IUnknownVtbl { QueryInterface: Self::QueryInterface, AddRef: Self::AddRef, Release: Self::Release },
        Invoke: Self::Invoke,
    };
    unsafe extern "system" fn QueryInterface(this: ::windows_core::RawPtr, iid: &::windows_core::GUID, interface: *mut *const ::core::ffi::c_void) -> ::windows_core::HRESULT {
        let this = this as *mut ::windows_core::RawPtr as *mut Self;
        *interface = if iid == &<DragStartedEventHandler as ::windows_core::Interface>::IID || iid == &<::windows_core::IUnknown as ::windows_core::Interface>::IID || iid == &<::windows_core::IAgileObject as ::windows_core::Interface>::IID { &mut (*this).vtable as *mut _ as _ } else { ::core::ptr::null_mut() };
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
    unsafe extern "system" fn Invoke(this: *mut ::core::ffi::c_void, sender: *mut ::core::ffi::c_void, e: ::windows_core::RawPtr) -> ::windows_core::HRESULT {
        let this = this as *mut ::windows_core::RawPtr as *mut Self;
        ((*this).invoke)(::core::mem::transmute(&sender), ::core::mem::transmute(&e)).into()
    }
}
impl ::core::clone::Clone for DragStartedEventHandler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for DragStartedEventHandler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DragStartedEventHandler {}
impl ::core::fmt::Debug for DragStartedEventHandler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DragStartedEventHandler").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for DragStartedEventHandler {
    type Vtable = DragStartedEventHandler_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd2eea48a_c65a_495d_a2f1_72c66989142d);
}
unsafe impl ::windows_core::RuntimeType for DragStartedEventHandler {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"{d2eea48a-c65a-495d-a2f1-72c66989142d}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct DragStartedEventHandler_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub Invoke: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sender: *mut ::core::ffi::c_void, e: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct EdgeTransitionLocation(pub i32);
impl EdgeTransitionLocation {
    pub const Left: Self = Self(0i32);
    pub const Top: Self = Self(1i32);
    pub const Right: Self = Self(2i32);
    pub const Bottom: Self = Self(3i32);
}
impl ::core::marker::Copy for EdgeTransitionLocation {}
impl ::core::clone::Clone for EdgeTransitionLocation {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for EdgeTransitionLocation {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for EdgeTransitionLocation {
    type Abi = Self;
}
impl ::core::fmt::Debug for EdgeTransitionLocation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EdgeTransitionLocation").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for EdgeTransitionLocation {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Controls.Primitives.EdgeTransitionLocation;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct FlyoutBase(::windows_core::IUnknown);
impl FlyoutBase {
    pub fn Placement(&self) -> ::windows_core::Result<FlyoutPlacementMode> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<FlyoutPlacementMode>::zeroed();
            (::windows_core::Interface::vtable(this).Placement)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<FlyoutPlacementMode>(result__)
        }
    }
    pub fn SetPlacement(&self, value: FlyoutPlacementMode) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetPlacement)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Opened<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventHandler<::windows_core::IInspectable>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).Opened)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveOpened<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveOpened)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn Closed<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventHandler<::windows_core::IInspectable>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).Closed)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveClosed<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveClosed)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn Opening<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventHandler<::windows_core::IInspectable>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).Opening)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveOpening<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveOpening)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn ShowAt<'a, Param0: ::windows_core::IntoParam<'a, super::super::FrameworkElement>>(&self, placementtarget: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).ShowAt)(::windows_core::Interface::as_raw(this), placementtarget.into_param().abi()).ok() }
    }
    pub fn Hide(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Hide)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Target(&self) -> ::windows_core::Result<super::super::FrameworkElement> {
        let this = &::windows_core::Interface::cast::<IFlyoutBase2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Target)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::FrameworkElement>(result__)
        }
    }
    pub fn AllowFocusOnInteraction(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IFlyoutBase2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).AllowFocusOnInteraction)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetAllowFocusOnInteraction(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IFlyoutBase2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetAllowFocusOnInteraction)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn LightDismissOverlayMode(&self) -> ::windows_core::Result<super::LightDismissOverlayMode> {
        let this = &::windows_core::Interface::cast::<IFlyoutBase2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::LightDismissOverlayMode>::zeroed();
            (::windows_core::Interface::vtable(this).LightDismissOverlayMode)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::LightDismissOverlayMode>(result__)
        }
    }
    pub fn SetLightDismissOverlayMode(&self, value: super::LightDismissOverlayMode) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IFlyoutBase2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetLightDismissOverlayMode)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn AllowFocusWhenDisabled(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IFlyoutBase2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).AllowFocusWhenDisabled)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetAllowFocusWhenDisabled(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IFlyoutBase2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetAllowFocusWhenDisabled)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ElementSoundMode(&self) -> ::windows_core::Result<super::super::ElementSoundMode> {
        let this = &::windows_core::Interface::cast::<IFlyoutBase2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::ElementSoundMode>::zeroed();
            (::windows_core::Interface::vtable(this).ElementSoundMode)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::ElementSoundMode>(result__)
        }
    }
    pub fn SetElementSoundMode(&self, value: super::super::ElementSoundMode) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IFlyoutBase2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetElementSoundMode)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Closing<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<FlyoutBase, FlyoutBaseClosingEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = &::windows_core::Interface::cast::<IFlyoutBase2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).Closing)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveClosing<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IFlyoutBase2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).RemoveClosing)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn OverlayInputPassThroughElement(&self) -> ::windows_core::Result<super::super::DependencyObject> {
        let this = &::windows_core::Interface::cast::<IFlyoutBase3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).OverlayInputPassThroughElement)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyObject>(result__)
        }
    }
    pub fn SetOverlayInputPassThroughElement<'a, Param0: ::windows_core::IntoParam<'a, super::super::DependencyObject>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IFlyoutBase3>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetOverlayInputPassThroughElement)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn TryInvokeKeyboardAccelerator<'a, Param0: ::windows_core::IntoParam<'a, super::super::Input::ProcessKeyboardAcceleratorEventArgs>>(&self, args: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IFlyoutBase4>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).TryInvokeKeyboardAccelerator)(::windows_core::Interface::as_raw(this), args.into_param().abi()).ok() }
    }
    pub fn ShowMode(&self) -> ::windows_core::Result<FlyoutShowMode> {
        let this = &::windows_core::Interface::cast::<IFlyoutBase5>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<FlyoutShowMode>::zeroed();
            (::windows_core::Interface::vtable(this).ShowMode)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<FlyoutShowMode>(result__)
        }
    }
    pub fn SetShowMode(&self, value: FlyoutShowMode) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IFlyoutBase5>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetShowMode)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn InputDevicePrefersPrimaryCommands(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IFlyoutBase5>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).InputDevicePrefersPrimaryCommands)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn AreOpenCloseAnimationsEnabled(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IFlyoutBase5>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).AreOpenCloseAnimationsEnabled)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetAreOpenCloseAnimationsEnabled(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IFlyoutBase5>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetAreOpenCloseAnimationsEnabled)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsOpen(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IFlyoutBase5>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsOpen)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn ShowAt2<'a, Param0: ::windows_core::IntoParam<'a, super::super::DependencyObject>, Param1: ::windows_core::IntoParam<'a, FlyoutShowOptions>>(&self, placementtarget: Param0, showoptions: Param1) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IFlyoutBase5>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).ShowAt)(::windows_core::Interface::as_raw(this), placementtarget.into_param().abi(), showoptions.into_param().abi()).ok() }
    }
    pub fn ShouldConstrainToRootBounds(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IFlyoutBase6>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).ShouldConstrainToRootBounds)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetShouldConstrainToRootBounds(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IFlyoutBase6>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetShouldConstrainToRootBounds)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsConstrainedToRootBounds(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IFlyoutBase6>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsConstrainedToRootBounds)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn XamlRoot(&self) -> ::windows_core::Result<super::super::XamlRoot> {
        let this = &::windows_core::Interface::cast::<IFlyoutBase6>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).XamlRoot)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::XamlRoot>(result__)
        }
    }
    pub fn SetXamlRoot<'a, Param0: ::windows_core::IntoParam<'a, super::super::XamlRoot>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IFlyoutBase6>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetXamlRoot)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn PlacementProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::IFlyoutBaseStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).PlacementProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn AttachedFlyoutProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::IFlyoutBaseStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).AttachedFlyoutProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn GetAttachedFlyout<'a, Param0: ::windows_core::IntoParam<'a, super::super::FrameworkElement>>(element: Param0) -> ::windows_core::Result<FlyoutBase> {
        Self::IFlyoutBaseStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetAttachedFlyout)(::windows_core::Interface::as_raw(this), element.into_param().abi(), result__.as_mut_ptr()).from_abi::<FlyoutBase>(result__)
        })
    }
    pub fn SetAttachedFlyout<'a, Param0: ::windows_core::IntoParam<'a, super::super::FrameworkElement>, Param1: ::windows_core::IntoParam<'a, FlyoutBase>>(element: Param0, value: Param1) -> ::windows_core::Result<()> {
        Self::IFlyoutBaseStatics(|this| unsafe { (::windows_core::Interface::vtable(this).SetAttachedFlyout)(::windows_core::Interface::as_raw(this), element.into_param().abi(), value.into_param().abi()).ok() })
    }
    pub fn ShowAttachedFlyout<'a, Param0: ::windows_core::IntoParam<'a, super::super::FrameworkElement>>(flyoutowner: Param0) -> ::windows_core::Result<()> {
        Self::IFlyoutBaseStatics(|this| unsafe { (::windows_core::Interface::vtable(this).ShowAttachedFlyout)(::windows_core::Interface::as_raw(this), flyoutowner.into_param().abi()).ok() })
    }
    pub fn AllowFocusOnInteractionProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::IFlyoutBaseStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).AllowFocusOnInteractionProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn LightDismissOverlayModeProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::IFlyoutBaseStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).LightDismissOverlayModeProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn AllowFocusWhenDisabledProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::IFlyoutBaseStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).AllowFocusWhenDisabledProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn ElementSoundModeProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::IFlyoutBaseStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ElementSoundModeProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn OverlayInputPassThroughElementProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::IFlyoutBaseStatics3(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).OverlayInputPassThroughElementProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn TargetProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::IFlyoutBaseStatics5(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TargetProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn ShowModeProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::IFlyoutBaseStatics5(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ShowModeProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn InputDevicePrefersPrimaryCommandsProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::IFlyoutBaseStatics5(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).InputDevicePrefersPrimaryCommandsProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn AreOpenCloseAnimationsEnabledProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::IFlyoutBaseStatics5(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).AreOpenCloseAnimationsEnabledProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn IsOpenProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::IFlyoutBaseStatics5(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).IsOpenProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn ShouldConstrainToRootBoundsProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::IFlyoutBaseStatics6(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ShouldConstrainToRootBoundsProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn IFlyoutBaseStatics<R, F: FnOnce(&IFlyoutBaseStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<FlyoutBase, IFlyoutBaseStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IFlyoutBaseStatics2<R, F: FnOnce(&IFlyoutBaseStatics2) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<FlyoutBase, IFlyoutBaseStatics2> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IFlyoutBaseStatics3<R, F: FnOnce(&IFlyoutBaseStatics3) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<FlyoutBase, IFlyoutBaseStatics3> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IFlyoutBaseStatics5<R, F: FnOnce(&IFlyoutBaseStatics5) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<FlyoutBase, IFlyoutBaseStatics5> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IFlyoutBaseStatics6<R, F: FnOnce(&IFlyoutBaseStatics6) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<FlyoutBase, IFlyoutBaseStatics6> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for FlyoutBase {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for FlyoutBase {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for FlyoutBase {}
impl ::core::fmt::Debug for FlyoutBase {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FlyoutBase").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for FlyoutBase {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Controls.Primitives.FlyoutBase;{723eea0b-d12e-430d-a9f0-9bb32bbf9913})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for FlyoutBase {
    type Vtable = IFlyoutBase_Vtbl;
    const IID: ::windows_core::GUID = <IFlyoutBase as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for FlyoutBase {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.FlyoutBase";
}
impl ::core::convert::From<FlyoutBase> for ::windows_core::IUnknown {
    fn from(value: FlyoutBase) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&FlyoutBase> for ::windows_core::IUnknown {
    fn from(value: &FlyoutBase) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for FlyoutBase {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a FlyoutBase {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<FlyoutBase> for ::windows_core::IInspectable {
    fn from(value: FlyoutBase) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&FlyoutBase> for ::windows_core::IInspectable {
    fn from(value: &FlyoutBase) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for FlyoutBase {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a FlyoutBase {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<FlyoutBase> for super::super::DependencyObject {
    fn from(value: FlyoutBase) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&FlyoutBase> for super::super::DependencyObject {
    fn from(value: &FlyoutBase) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::DependencyObject> for FlyoutBase {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::DependencyObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::DependencyObject> for &FlyoutBase {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::DependencyObject> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for FlyoutBase {}
unsafe impl ::core::marker::Sync for FlyoutBase {}
#[repr(transparent)]
pub struct FlyoutBaseClosingEventArgs(::windows_core::IUnknown);
impl FlyoutBaseClosingEventArgs {
    pub fn Cancel(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).Cancel)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetCancel(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetCancel)(::windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl ::core::clone::Clone for FlyoutBaseClosingEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for FlyoutBaseClosingEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for FlyoutBaseClosingEventArgs {}
impl ::core::fmt::Debug for FlyoutBaseClosingEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FlyoutBaseClosingEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for FlyoutBaseClosingEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Controls.Primitives.FlyoutBaseClosingEventArgs;{d075852d-b09a-4fd1-b005-db2ba01206fb})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for FlyoutBaseClosingEventArgs {
    type Vtable = IFlyoutBaseClosingEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IFlyoutBaseClosingEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for FlyoutBaseClosingEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.FlyoutBaseClosingEventArgs";
}
impl ::core::convert::From<FlyoutBaseClosingEventArgs> for ::windows_core::IUnknown {
    fn from(value: FlyoutBaseClosingEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&FlyoutBaseClosingEventArgs> for ::windows_core::IUnknown {
    fn from(value: &FlyoutBaseClosingEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for FlyoutBaseClosingEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a FlyoutBaseClosingEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<FlyoutBaseClosingEventArgs> for ::windows_core::IInspectable {
    fn from(value: FlyoutBaseClosingEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&FlyoutBaseClosingEventArgs> for ::windows_core::IInspectable {
    fn from(value: &FlyoutBaseClosingEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for FlyoutBaseClosingEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a FlyoutBaseClosingEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for FlyoutBaseClosingEventArgs {}
unsafe impl ::core::marker::Sync for FlyoutBaseClosingEventArgs {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct FlyoutPlacementMode(pub i32);
impl FlyoutPlacementMode {
    pub const Top: Self = Self(0i32);
    pub const Bottom: Self = Self(1i32);
    pub const Left: Self = Self(2i32);
    pub const Right: Self = Self(3i32);
    pub const Full: Self = Self(4i32);
    pub const TopEdgeAlignedLeft: Self = Self(5i32);
    pub const TopEdgeAlignedRight: Self = Self(6i32);
    pub const BottomEdgeAlignedLeft: Self = Self(7i32);
    pub const BottomEdgeAlignedRight: Self = Self(8i32);
    pub const LeftEdgeAlignedTop: Self = Self(9i32);
    pub const LeftEdgeAlignedBottom: Self = Self(10i32);
    pub const RightEdgeAlignedTop: Self = Self(11i32);
    pub const RightEdgeAlignedBottom: Self = Self(12i32);
    pub const Auto: Self = Self(13i32);
}
impl ::core::marker::Copy for FlyoutPlacementMode {}
impl ::core::clone::Clone for FlyoutPlacementMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FlyoutPlacementMode {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for FlyoutPlacementMode {
    type Abi = Self;
}
impl ::core::fmt::Debug for FlyoutPlacementMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FlyoutPlacementMode").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for FlyoutPlacementMode {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Controls.Primitives.FlyoutPlacementMode;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct FlyoutShowMode(pub i32);
impl FlyoutShowMode {
    pub const Auto: Self = Self(0i32);
    pub const Standard: Self = Self(1i32);
    pub const Transient: Self = Self(2i32);
    pub const TransientWithDismissOnPointerMoveAway: Self = Self(3i32);
}
impl ::core::marker::Copy for FlyoutShowMode {}
impl ::core::clone::Clone for FlyoutShowMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FlyoutShowMode {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for FlyoutShowMode {
    type Abi = Self;
}
impl ::core::fmt::Debug for FlyoutShowMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FlyoutShowMode").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for FlyoutShowMode {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Controls.Primitives.FlyoutShowMode;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct FlyoutShowOptions(::windows_core::IUnknown);
impl FlyoutShowOptions {
    pub fn Position(&self) -> ::windows_core::Result<::winrt_foundation::IReference<::winrt_foundation::Point>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Position)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IReference<::winrt_foundation::Point>>(result__)
        }
    }
    pub fn SetPosition<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::IReference<::winrt_foundation::Point>>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetPosition)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn ExclusionRect(&self) -> ::windows_core::Result<::winrt_foundation::IReference<::winrt_foundation::Rect>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ExclusionRect)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IReference<::winrt_foundation::Rect>>(result__)
        }
    }
    pub fn SetExclusionRect<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::IReference<::winrt_foundation::Rect>>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetExclusionRect)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn ShowMode(&self) -> ::windows_core::Result<FlyoutShowMode> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<FlyoutShowMode>::zeroed();
            (::windows_core::Interface::vtable(this).ShowMode)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<FlyoutShowMode>(result__)
        }
    }
    pub fn SetShowMode(&self, value: FlyoutShowMode) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetShowMode)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Placement(&self) -> ::windows_core::Result<FlyoutPlacementMode> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<FlyoutPlacementMode>::zeroed();
            (::windows_core::Interface::vtable(this).Placement)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<FlyoutPlacementMode>(result__)
        }
    }
    pub fn SetPlacement(&self, value: FlyoutPlacementMode) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetPlacement)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn new() -> ::windows_core::Result<FlyoutShowOptions> {
        Self::IFlyoutShowOptionsFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateInstance)(::windows_core::Interface::as_raw(this), ::core::ptr::null_mut(), &mut ::core::option::Option::<::windows_core::IInspectable>::None as *mut _ as _, result__.as_mut_ptr()).from_abi::<FlyoutShowOptions>(result__)
        })
    }
    pub fn compose<T: ::windows_core::Compose>(compose: T) -> ::windows_core::Result<FlyoutShowOptions> {
        Self::IFlyoutShowOptionsFactory(|this| unsafe {
            let (derived__, base__) = ::windows_core::Compose::compose(compose);
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateInstance)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(&derived__), base__ as *mut _ as _, result__.as_mut_ptr()).from_abi::<FlyoutShowOptions>(result__)
        })
    }
    pub fn IFlyoutShowOptionsFactory<R, F: FnOnce(&IFlyoutShowOptionsFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<FlyoutShowOptions, IFlyoutShowOptionsFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for FlyoutShowOptions {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for FlyoutShowOptions {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for FlyoutShowOptions {}
impl ::core::fmt::Debug for FlyoutShowOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FlyoutShowOptions").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for FlyoutShowOptions {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Controls.Primitives.FlyoutShowOptions;{57d693ad-0c74-54dd-b110-1ee43fabadd9})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for FlyoutShowOptions {
    type Vtable = IFlyoutShowOptions_Vtbl;
    const IID: ::windows_core::GUID = <IFlyoutShowOptions as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for FlyoutShowOptions {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.FlyoutShowOptions";
}
impl ::core::convert::From<FlyoutShowOptions> for ::windows_core::IUnknown {
    fn from(value: FlyoutShowOptions) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&FlyoutShowOptions> for ::windows_core::IUnknown {
    fn from(value: &FlyoutShowOptions) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for FlyoutShowOptions {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a FlyoutShowOptions {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<FlyoutShowOptions> for ::windows_core::IInspectable {
    fn from(value: FlyoutShowOptions) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&FlyoutShowOptions> for ::windows_core::IInspectable {
    fn from(value: &FlyoutShowOptions) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for FlyoutShowOptions {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a FlyoutShowOptions {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for FlyoutShowOptions {}
unsafe impl ::core::marker::Sync for FlyoutShowOptions {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct GeneratorDirection(pub i32);
impl GeneratorDirection {
    pub const Forward: Self = Self(0i32);
    pub const Backward: Self = Self(1i32);
}
impl ::core::marker::Copy for GeneratorDirection {}
impl ::core::clone::Clone for GeneratorDirection {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for GeneratorDirection {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for GeneratorDirection {
    type Abi = Self;
}
impl ::core::fmt::Debug for GeneratorDirection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GeneratorDirection").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for GeneratorDirection {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Controls.Primitives.GeneratorDirection;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(C)]
pub struct GeneratorPosition {
    pub Index: i32,
    pub Offset: i32,
}
impl ::core::marker::Copy for GeneratorPosition {}
impl ::core::clone::Clone for GeneratorPosition {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for GeneratorPosition {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GeneratorPosition").field("Index", &self.Index).field("Offset", &self.Offset).finish()
    }
}
unsafe impl ::windows_core::Abi for GeneratorPosition {
    type Abi = Self;
}
unsafe impl ::windows_core::RuntimeType for GeneratorPosition {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"struct(Windows.UI.Xaml.Controls.Primitives.GeneratorPosition;i4;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
impl ::core::cmp::PartialEq for GeneratorPosition {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<GeneratorPosition>()) == 0 }
    }
}
impl ::core::cmp::Eq for GeneratorPosition {}
impl ::core::default::Default for GeneratorPosition {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
pub struct GeneratorPositionHelper(::windows_core::IUnknown);
impl GeneratorPositionHelper {
    pub fn FromIndexAndOffset(index: i32, offset: i32) -> ::windows_core::Result<GeneratorPosition> {
        Self::IGeneratorPositionHelperStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<GeneratorPosition>::zeroed();
            (::windows_core::Interface::vtable(this).FromIndexAndOffset)(::windows_core::Interface::as_raw(this), index, offset, result__.as_mut_ptr()).from_abi::<GeneratorPosition>(result__)
        })
    }
    pub fn IGeneratorPositionHelperStatics<R, F: FnOnce(&IGeneratorPositionHelperStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<GeneratorPositionHelper, IGeneratorPositionHelperStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for GeneratorPositionHelper {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for GeneratorPositionHelper {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GeneratorPositionHelper {}
impl ::core::fmt::Debug for GeneratorPositionHelper {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GeneratorPositionHelper").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for GeneratorPositionHelper {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Controls.Primitives.GeneratorPositionHelper;{cd40318d-7745-40d9-ab9d-abbda4a7ffea})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for GeneratorPositionHelper {
    type Vtable = IGeneratorPositionHelper_Vtbl;
    const IID: ::windows_core::GUID = <IGeneratorPositionHelper as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for GeneratorPositionHelper {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.GeneratorPositionHelper";
}
impl ::core::convert::From<GeneratorPositionHelper> for ::windows_core::IUnknown {
    fn from(value: GeneratorPositionHelper) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&GeneratorPositionHelper> for ::windows_core::IUnknown {
    fn from(value: &GeneratorPositionHelper) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for GeneratorPositionHelper {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a GeneratorPositionHelper {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<GeneratorPositionHelper> for ::windows_core::IInspectable {
    fn from(value: GeneratorPositionHelper) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&GeneratorPositionHelper> for ::windows_core::IInspectable {
    fn from(value: &GeneratorPositionHelper) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for GeneratorPositionHelper {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a GeneratorPositionHelper {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for GeneratorPositionHelper {}
unsafe impl ::core::marker::Sync for GeneratorPositionHelper {}
#[repr(transparent)]
pub struct GridViewItemPresenter(::windows_core::IUnknown);
impl GridViewItemPresenter {
    pub fn SelectionCheckMarkVisualEnabled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).SelectionCheckMarkVisualEnabled)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetSelectionCheckMarkVisualEnabled(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetSelectionCheckMarkVisualEnabled)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn CheckHintBrush(&self) -> ::windows_core::Result<super::super::Media::Brush> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CheckHintBrush)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Media::Brush>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetCheckHintBrush<'a, Param0: ::windows_core::IntoParam<'a, super::super::Media::Brush>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetCheckHintBrush)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn CheckSelectingBrush(&self) -> ::windows_core::Result<super::super::Media::Brush> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CheckSelectingBrush)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Media::Brush>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetCheckSelectingBrush<'a, Param0: ::windows_core::IntoParam<'a, super::super::Media::Brush>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetCheckSelectingBrush)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn CheckBrush(&self) -> ::windows_core::Result<super::super::Media::Brush> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CheckBrush)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Media::Brush>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetCheckBrush<'a, Param0: ::windows_core::IntoParam<'a, super::super::Media::Brush>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetCheckBrush)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn DragBackground(&self) -> ::windows_core::Result<super::super::Media::Brush> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).DragBackground)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Media::Brush>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetDragBackground<'a, Param0: ::windows_core::IntoParam<'a, super::super::Media::Brush>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetDragBackground)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn DragForeground(&self) -> ::windows_core::Result<super::super::Media::Brush> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).DragForeground)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Media::Brush>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetDragForeground<'a, Param0: ::windows_core::IntoParam<'a, super::super::Media::Brush>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetDragForeground)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn FocusBorderBrush(&self) -> ::windows_core::Result<super::super::Media::Brush> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FocusBorderBrush)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Media::Brush>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetFocusBorderBrush<'a, Param0: ::windows_core::IntoParam<'a, super::super::Media::Brush>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetFocusBorderBrush)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn PlaceholderBackground(&self) -> ::windows_core::Result<super::super::Media::Brush> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).PlaceholderBackground)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Media::Brush>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetPlaceholderBackground<'a, Param0: ::windows_core::IntoParam<'a, super::super::Media::Brush>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetPlaceholderBackground)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn PointerOverBackground(&self) -> ::windows_core::Result<super::super::Media::Brush> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).PointerOverBackground)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Media::Brush>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetPointerOverBackground<'a, Param0: ::windows_core::IntoParam<'a, super::super::Media::Brush>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetPointerOverBackground)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SelectedBackground(&self) -> ::windows_core::Result<super::super::Media::Brush> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SelectedBackground)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Media::Brush>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetSelectedBackground<'a, Param0: ::windows_core::IntoParam<'a, super::super::Media::Brush>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetSelectedBackground)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SelectedForeground(&self) -> ::windows_core::Result<super::super::Media::Brush> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SelectedForeground)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Media::Brush>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetSelectedForeground<'a, Param0: ::windows_core::IntoParam<'a, super::super::Media::Brush>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetSelectedForeground)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SelectedPointerOverBackground(&self) -> ::windows_core::Result<super::super::Media::Brush> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SelectedPointerOverBackground)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Media::Brush>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetSelectedPointerOverBackground<'a, Param0: ::windows_core::IntoParam<'a, super::super::Media::Brush>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetSelectedPointerOverBackground)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SelectedPointerOverBorderBrush(&self) -> ::windows_core::Result<super::super::Media::Brush> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SelectedPointerOverBorderBrush)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Media::Brush>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetSelectedPointerOverBorderBrush<'a, Param0: ::windows_core::IntoParam<'a, super::super::Media::Brush>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetSelectedPointerOverBorderBrush)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn SelectedBorderThickness(&self) -> ::windows_core::Result<super::super::Thickness> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::Thickness>::zeroed();
            (::windows_core::Interface::vtable(this).SelectedBorderThickness)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Thickness>(result__)
        }
    }
    pub fn SetSelectedBorderThickness<'a, Param0: ::windows_core::IntoParam<'a, super::super::Thickness>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetSelectedBorderThickness)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn DisabledOpacity(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).DisabledOpacity)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn SetDisabledOpacity(&self, value: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetDisabledOpacity)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn DragOpacity(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).DragOpacity)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn SetDragOpacity(&self, value: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetDragOpacity)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ReorderHintOffset(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).ReorderHintOffset)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn SetReorderHintOffset(&self, value: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetReorderHintOffset)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "deprecated")]
    pub fn GridViewItemPresenterHorizontalContentAlignment(&self) -> ::windows_core::Result<super::super::HorizontalAlignment> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::HorizontalAlignment>::zeroed();
            (::windows_core::Interface::vtable(this).GridViewItemPresenterHorizontalContentAlignment)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::HorizontalAlignment>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn SetGridViewItemPresenterHorizontalContentAlignment(&self, value: super::super::HorizontalAlignment) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetGridViewItemPresenterHorizontalContentAlignment)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "deprecated")]
    pub fn GridViewItemPresenterVerticalContentAlignment(&self) -> ::windows_core::Result<super::super::VerticalAlignment> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::VerticalAlignment>::zeroed();
            (::windows_core::Interface::vtable(this).GridViewItemPresenterVerticalContentAlignment)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::VerticalAlignment>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn SetGridViewItemPresenterVerticalContentAlignment(&self, value: super::super::VerticalAlignment) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetGridViewItemPresenterVerticalContentAlignment)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "deprecated")]
    pub fn GridViewItemPresenterPadding(&self) -> ::windows_core::Result<super::super::Thickness> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::Thickness>::zeroed();
            (::windows_core::Interface::vtable(this).GridViewItemPresenterPadding)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Thickness>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn SetGridViewItemPresenterPadding<'a, Param0: ::windows_core::IntoParam<'a, super::super::Thickness>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetGridViewItemPresenterPadding)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn PointerOverBackgroundMargin(&self) -> ::windows_core::Result<super::super::Thickness> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::Thickness>::zeroed();
            (::windows_core::Interface::vtable(this).PointerOverBackgroundMargin)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Thickness>(result__)
        }
    }
    pub fn SetPointerOverBackgroundMargin<'a, Param0: ::windows_core::IntoParam<'a, super::super::Thickness>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetPointerOverBackgroundMargin)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn ContentMargin(&self) -> ::windows_core::Result<super::super::Thickness> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::Thickness>::zeroed();
            (::windows_core::Interface::vtable(this).ContentMargin)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Thickness>(result__)
        }
    }
    pub fn SetContentMargin<'a, Param0: ::windows_core::IntoParam<'a, super::super::Thickness>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetContentMargin)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn new() -> ::windows_core::Result<GridViewItemPresenter> {
        Self::IGridViewItemPresenterFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateInstance)(::windows_core::Interface::as_raw(this), ::core::ptr::null_mut(), &mut ::core::option::Option::<::windows_core::IInspectable>::None as *mut _ as _, result__.as_mut_ptr()).from_abi::<GridViewItemPresenter>(result__)
        })
    }
    pub fn compose<T: ::windows_core::Compose>(compose: T) -> ::windows_core::Result<GridViewItemPresenter> {
        Self::IGridViewItemPresenterFactory(|this| unsafe {
            let (derived__, base__) = ::windows_core::Compose::compose(compose);
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateInstance)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(&derived__), base__ as *mut _ as _, result__.as_mut_ptr()).from_abi::<GridViewItemPresenter>(result__)
        })
    }
    pub fn SelectionCheckMarkVisualEnabledProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::IGridViewItemPresenterStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SelectionCheckMarkVisualEnabledProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn CheckHintBrushProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::IGridViewItemPresenterStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CheckHintBrushProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn CheckSelectingBrushProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::IGridViewItemPresenterStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CheckSelectingBrushProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn CheckBrushProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::IGridViewItemPresenterStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CheckBrushProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn DragBackgroundProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::IGridViewItemPresenterStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).DragBackgroundProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn DragForegroundProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::IGridViewItemPresenterStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).DragForegroundProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn FocusBorderBrushProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::IGridViewItemPresenterStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FocusBorderBrushProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn PlaceholderBackgroundProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::IGridViewItemPresenterStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).PlaceholderBackgroundProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn PointerOverBackgroundProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::IGridViewItemPresenterStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).PointerOverBackgroundProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn SelectedBackgroundProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::IGridViewItemPresenterStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SelectedBackgroundProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn SelectedForegroundProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::IGridViewItemPresenterStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SelectedForegroundProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn SelectedPointerOverBackgroundProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::IGridViewItemPresenterStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SelectedPointerOverBackgroundProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn SelectedPointerOverBorderBrushProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::IGridViewItemPresenterStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SelectedPointerOverBorderBrushProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn SelectedBorderThicknessProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::IGridViewItemPresenterStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SelectedBorderThicknessProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn DisabledOpacityProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::IGridViewItemPresenterStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).DisabledOpacityProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn DragOpacityProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::IGridViewItemPresenterStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).DragOpacityProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn ReorderHintOffsetProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::IGridViewItemPresenterStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ReorderHintOffsetProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[cfg(feature = "deprecated")]
    pub fn GridViewItemPresenterHorizontalContentAlignmentProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::IGridViewItemPresenterStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GridViewItemPresenterHorizontalContentAlignmentProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[cfg(feature = "deprecated")]
    pub fn GridViewItemPresenterVerticalContentAlignmentProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::IGridViewItemPresenterStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GridViewItemPresenterVerticalContentAlignmentProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[cfg(feature = "deprecated")]
    pub fn GridViewItemPresenterPaddingProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::IGridViewItemPresenterStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GridViewItemPresenterPaddingProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn PointerOverBackgroundMarginProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::IGridViewItemPresenterStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).PointerOverBackgroundMarginProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn ContentMarginProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::IGridViewItemPresenterStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ContentMarginProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn IGridViewItemPresenterFactory<R, F: FnOnce(&IGridViewItemPresenterFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<GridViewItemPresenter, IGridViewItemPresenterFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IGridViewItemPresenterStatics<R, F: FnOnce(&IGridViewItemPresenterStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<GridViewItemPresenter, IGridViewItemPresenterStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for GridViewItemPresenter {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for GridViewItemPresenter {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GridViewItemPresenter {}
impl ::core::fmt::Debug for GridViewItemPresenter {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GridViewItemPresenter").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for GridViewItemPresenter {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Controls.Primitives.GridViewItemPresenter;{214f9010-56e2-4821-8a1c-2305709af94b})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for GridViewItemPresenter {
    type Vtable = IGridViewItemPresenter_Vtbl;
    const IID: ::windows_core::GUID = <IGridViewItemPresenter as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for GridViewItemPresenter {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.GridViewItemPresenter";
}
impl ::core::convert::From<GridViewItemPresenter> for ::windows_core::IUnknown {
    fn from(value: GridViewItemPresenter) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&GridViewItemPresenter> for ::windows_core::IUnknown {
    fn from(value: &GridViewItemPresenter) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for GridViewItemPresenter {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a GridViewItemPresenter {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<GridViewItemPresenter> for ::windows_core::IInspectable {
    fn from(value: GridViewItemPresenter) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&GridViewItemPresenter> for ::windows_core::IInspectable {
    fn from(value: &GridViewItemPresenter) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for GridViewItemPresenter {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a GridViewItemPresenter {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<GridViewItemPresenter> for super::super::super::Composition::IAnimationObject {
    type Error = ::windows_core::Error;
    fn try_from(value: GridViewItemPresenter) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<&GridViewItemPresenter> for super::super::super::Composition::IAnimationObject {
    type Error = ::windows_core::Error;
    fn try_from(value: &GridViewItemPresenter) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows_core::IntoParam<'a, super::super::super::Composition::IAnimationObject> for GridViewItemPresenter {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::super::Composition::IAnimationObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows_core::IntoParam<'a, super::super::super::Composition::IAnimationObject> for &GridViewItemPresenter {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::super::Composition::IAnimationObject> {
        ::core::convert::TryInto::<super::super::super::Composition::IAnimationObject>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<GridViewItemPresenter> for super::super::super::Composition::IVisualElement {
    type Error = ::windows_core::Error;
    fn try_from(value: GridViewItemPresenter) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<&GridViewItemPresenter> for super::super::super::Composition::IVisualElement {
    type Error = ::windows_core::Error;
    fn try_from(value: &GridViewItemPresenter) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows_core::IntoParam<'a, super::super::super::Composition::IVisualElement> for GridViewItemPresenter {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::super::Composition::IVisualElement> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows_core::IntoParam<'a, super::super::super::Composition::IVisualElement> for &GridViewItemPresenter {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::super::Composition::IVisualElement> {
        ::core::convert::TryInto::<super::super::super::Composition::IVisualElement>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::From<GridViewItemPresenter> for super::ContentPresenter {
    fn from(value: GridViewItemPresenter) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&GridViewItemPresenter> for super::ContentPresenter {
    fn from(value: &GridViewItemPresenter) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::ContentPresenter> for GridViewItemPresenter {
    fn into_param(self) -> ::windows_core::Param<'a, super::ContentPresenter> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::ContentPresenter> for &GridViewItemPresenter {
    fn into_param(self) -> ::windows_core::Param<'a, super::ContentPresenter> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::ContentPresenter>::into(self))
    }
}
impl ::core::convert::From<GridViewItemPresenter> for super::super::FrameworkElement {
    fn from(value: GridViewItemPresenter) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&GridViewItemPresenter> for super::super::FrameworkElement {
    fn from(value: &GridViewItemPresenter) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::FrameworkElement> for GridViewItemPresenter {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::FrameworkElement> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::FrameworkElement> for &GridViewItemPresenter {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::FrameworkElement> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::super::FrameworkElement>::into(self))
    }
}
impl ::core::convert::From<GridViewItemPresenter> for super::super::UIElement {
    fn from(value: GridViewItemPresenter) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&GridViewItemPresenter> for super::super::UIElement {
    fn from(value: &GridViewItemPresenter) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::UIElement> for GridViewItemPresenter {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::UIElement> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::UIElement> for &GridViewItemPresenter {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::UIElement> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::super::UIElement>::into(self))
    }
}
impl ::core::convert::From<GridViewItemPresenter> for super::super::DependencyObject {
    fn from(value: GridViewItemPresenter) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&GridViewItemPresenter> for super::super::DependencyObject {
    fn from(value: &GridViewItemPresenter) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::DependencyObject> for GridViewItemPresenter {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::DependencyObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::DependencyObject> for &GridViewItemPresenter {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::DependencyObject> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for GridViewItemPresenter {}
unsafe impl ::core::marker::Sync for GridViewItemPresenter {}
#[repr(transparent)]
pub struct GridViewItemTemplateSettings(::windows_core::IUnknown);
impl GridViewItemTemplateSettings {
    pub fn DragItemsCount(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows_core::Interface::vtable(this).DragItemsCount)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
}
impl ::core::clone::Clone for GridViewItemTemplateSettings {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for GridViewItemTemplateSettings {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GridViewItemTemplateSettings {}
impl ::core::fmt::Debug for GridViewItemTemplateSettings {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GridViewItemTemplateSettings").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for GridViewItemTemplateSettings {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Controls.Primitives.GridViewItemTemplateSettings;{9e30baaf-165d-4267-a45e-1a43a75706ac})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for GridViewItemTemplateSettings {
    type Vtable = IGridViewItemTemplateSettings_Vtbl;
    const IID: ::windows_core::GUID = <IGridViewItemTemplateSettings as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for GridViewItemTemplateSettings {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.GridViewItemTemplateSettings";
}
impl ::core::convert::From<GridViewItemTemplateSettings> for ::windows_core::IUnknown {
    fn from(value: GridViewItemTemplateSettings) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&GridViewItemTemplateSettings> for ::windows_core::IUnknown {
    fn from(value: &GridViewItemTemplateSettings) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for GridViewItemTemplateSettings {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a GridViewItemTemplateSettings {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<GridViewItemTemplateSettings> for ::windows_core::IInspectable {
    fn from(value: GridViewItemTemplateSettings) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&GridViewItemTemplateSettings> for ::windows_core::IInspectable {
    fn from(value: &GridViewItemTemplateSettings) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for GridViewItemTemplateSettings {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a GridViewItemTemplateSettings {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<GridViewItemTemplateSettings> for super::super::DependencyObject {
    fn from(value: GridViewItemTemplateSettings) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&GridViewItemTemplateSettings> for super::super::DependencyObject {
    fn from(value: &GridViewItemTemplateSettings) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::DependencyObject> for GridViewItemTemplateSettings {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::DependencyObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::DependencyObject> for &GridViewItemTemplateSettings {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::DependencyObject> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for GridViewItemTemplateSettings {}
unsafe impl ::core::marker::Sync for GridViewItemTemplateSettings {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct GroupHeaderPlacement(pub i32);
impl GroupHeaderPlacement {
    pub const Top: Self = Self(0i32);
    pub const Left: Self = Self(1i32);
}
impl ::core::marker::Copy for GroupHeaderPlacement {}
impl ::core::clone::Clone for GroupHeaderPlacement {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for GroupHeaderPlacement {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for GroupHeaderPlacement {
    type Abi = Self;
}
impl ::core::fmt::Debug for GroupHeaderPlacement {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GroupHeaderPlacement").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for GroupHeaderPlacement {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Controls.Primitives.GroupHeaderPlacement;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppBarButtonTemplateSettings(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppBarButtonTemplateSettings {
    type Vtable = IAppBarButtonTemplateSettings_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xcbc9b39d_0c95_4951_bff2_13963691c366);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppBarButtonTemplateSettings_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub KeyboardAcceleratorTextMinWidth: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppBarTemplateSettings(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppBarTemplateSettings {
    type Vtable = IAppBarTemplateSettings_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xbcc2a863_eb35_423c_8389_d7827be3bf67);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppBarTemplateSettings_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub ClipRect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::Rect) -> ::windows_core::HRESULT,
    pub CompactVerticalDelta: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub CompactRootMargin: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Thickness) -> ::windows_core::HRESULT,
    pub MinimalVerticalDelta: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub MinimalRootMargin: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Thickness) -> ::windows_core::HRESULT,
    pub HiddenVerticalDelta: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub HiddenRootMargin: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Thickness) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppBarTemplateSettings2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppBarTemplateSettings2 {
    type Vtable = IAppBarTemplateSettings2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xcbe66259_0399_5bcc_b925_4d5f5c9a4568);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppBarTemplateSettings2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub NegativeCompactVerticalDelta: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub NegativeMinimalVerticalDelta: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub NegativeHiddenVerticalDelta: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppBarToggleButtonTemplateSettings(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppBarToggleButtonTemplateSettings {
    type Vtable = IAppBarToggleButtonTemplateSettings_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xaaf99c48_d8f4_40d9_9fa3_3a64f0fec5d8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppBarToggleButtonTemplateSettings_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub KeyboardAcceleratorTextMinWidth: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IButtonBase(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IButtonBase {
    type Vtable = IButtonBase_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xfa002c1a_494e_46cf_91d4_e14a8d798674);
}
#[repr(C)]
#[doc(hidden)]
pub struct IButtonBase_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub ClickMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::ClickMode) -> ::windows_core::HRESULT,
    pub SetClickMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::ClickMode) -> ::windows_core::HRESULT,
    pub IsPointerOver: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub IsPressed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(feature = "UI_Xaml_Input")]
    pub Command: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Input"))]
    Command: usize,
    #[cfg(feature = "UI_Xaml_Input")]
    pub SetCommand: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Input"))]
    SetCommand: usize,
    pub CommandParameter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetCommandParameter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Click: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveClick: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IButtonBaseFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IButtonBaseFactory {
    type Vtable = IButtonBaseFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x389b7c71_5220_42b2_9992_2690c1a6702f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IButtonBaseFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CreateInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IButtonBaseStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IButtonBaseStatics {
    type Vtable = IButtonBaseStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x67ef17e1_fe37_474f_9e97_3b5e0b30f2df);
}
#[repr(C)]
#[doc(hidden)]
pub struct IButtonBaseStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub ClickModeProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub IsPointerOverProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub IsPressedProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CommandProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CommandParameterProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICalendarPanel(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICalendarPanel {
    type Vtable = ICalendarPanel_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xfcd55a2d_02d3_4ee6_9a90_9df3ead00994);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICalendarPanel_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICalendarViewTemplateSettings(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICalendarViewTemplateSettings {
    type Vtable = ICalendarViewTemplateSettings_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x56c71483_64e1_477c_8a0b_cb2f3334b9b0);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICalendarViewTemplateSettings_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub MinViewWidth: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub HeaderText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub WeekDay1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub WeekDay2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub WeekDay3: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub WeekDay4: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub WeekDay5: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub WeekDay6: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub WeekDay7: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub HasMoreContentAfter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub HasMoreContentBefore: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub HasMoreViews: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub ClipRect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::Rect) -> ::windows_core::HRESULT,
    pub CenterX: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub CenterY: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICarouselPanel(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICarouselPanel {
    type Vtable = ICarouselPanel_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xdeab78b2_373b_4151_8785_e544d0d9362b);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICarouselPanel_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CanVerticallyScroll: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetCanVerticallyScroll: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub CanHorizontallyScroll: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetCanHorizontallyScroll: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub ExtentWidth: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub ExtentHeight: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub ViewportWidth: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub ViewportHeight: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub HorizontalOffset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub VerticalOffset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub ScrollOwner: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetScrollOwner: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub LineUp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub LineDown: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub LineLeft: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub LineRight: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub PageUp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub PageDown: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub PageLeft: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub PageRight: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub MouseWheelUp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub MouseWheelDown: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub MouseWheelLeft: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub MouseWheelRight: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetHorizontalOffset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, offset: f64) -> ::windows_core::HRESULT,
    pub SetVerticalOffset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, offset: f64) -> ::windows_core::HRESULT,
    pub MakeVisible: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, visual: ::windows_core::RawPtr, rectangle: ::winrt_foundation::Rect, result__: *mut ::winrt_foundation::Rect) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICarouselPanelFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICarouselPanelFactory {
    type Vtable = ICarouselPanelFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc1109404_9ae1_440e_a0dd_bbb6e2293cbe);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICarouselPanelFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CreateInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IColorPickerSlider(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IColorPickerSlider {
    type Vtable = IColorPickerSlider_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x94394d83_e0df_4c5f_bbcd_8155f4020440);
}
#[repr(C)]
#[doc(hidden)]
pub struct IColorPickerSlider_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub ColorChannel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::ColorPickerHsvChannel) -> ::windows_core::HRESULT,
    pub SetColorChannel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::ColorPickerHsvChannel) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IColorPickerSliderFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IColorPickerSliderFactory {
    type Vtable = IColorPickerSliderFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x06d879a2_8c07_4b1e_a940_9fbce8f49639);
}
#[repr(C)]
#[doc(hidden)]
pub struct IColorPickerSliderFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CreateInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IColorPickerSliderStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IColorPickerSliderStatics {
    type Vtable = IColorPickerSliderStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x22eafc6a_9fe3_4eee_8734_a1398ec4413a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IColorPickerSliderStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub ColorChannelProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IColorSpectrum(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IColorSpectrum {
    type Vtable = IColorSpectrum_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xce46f271_f509_4f98_8288_e4942fb385df);
}
#[repr(C)]
#[doc(hidden)]
pub struct IColorSpectrum_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Color: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Color) -> ::windows_core::HRESULT,
    pub SetColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::super::Color) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Numerics")]
    pub HsvColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::Numerics::Vector4) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    HsvColor: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub SetHsvColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::winrt_foundation::Numerics::Vector4) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    SetHsvColor: usize,
    pub MinHue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub SetMinHue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows_core::HRESULT,
    pub MaxHue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub SetMaxHue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows_core::HRESULT,
    pub MinSaturation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub SetMinSaturation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows_core::HRESULT,
    pub MaxSaturation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub SetMaxSaturation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows_core::HRESULT,
    pub MinValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub SetMinValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows_core::HRESULT,
    pub MaxValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub SetMaxValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows_core::HRESULT,
    pub Shape: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::ColorSpectrumShape) -> ::windows_core::HRESULT,
    pub SetShape: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::ColorSpectrumShape) -> ::windows_core::HRESULT,
    pub Components: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::ColorSpectrumComponents) -> ::windows_core::HRESULT,
    pub SetComponents: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::ColorSpectrumComponents) -> ::windows_core::HRESULT,
    pub ColorChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveColorChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IColorSpectrumFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IColorSpectrumFactory {
    type Vtable = IColorSpectrumFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x90c7e61e_904d_42ab_b44f_e68dbf0cdee9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IColorSpectrumFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CreateInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IColorSpectrumStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IColorSpectrumStatics {
    type Vtable = IColorSpectrumStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x906bee7c_2cee_4e90_968b_f0a5bd691b4a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IColorSpectrumStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub ColorProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub HsvColorProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub MinHueProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub MaxHueProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub MinSaturationProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub MaxSaturationProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub MinValueProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub MaxValueProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ShapeProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ComponentsProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IComboBoxTemplateSettings(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IComboBoxTemplateSettings {
    type Vtable = IComboBoxTemplateSettings_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x83285c4e_17f6_4aa3_b61b_e87c718604ea);
}
#[repr(C)]
#[doc(hidden)]
pub struct IComboBoxTemplateSettings_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub DropDownOpenedHeight: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub DropDownClosedHeight: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub DropDownOffset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub SelectedItemDirection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AnimationDirection) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IComboBoxTemplateSettings2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IComboBoxTemplateSettings2 {
    type Vtable = IComboBoxTemplateSettings2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x00e90cd7_68be_449d_b5a7_76e26f703e9b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IComboBoxTemplateSettings2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub DropDownContentMinWidth: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICommandBarFlyoutCommandBar(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICommandBarFlyoutCommandBar {
    type Vtable = ICommandBarFlyoutCommandBar_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x14146e7c_38c4_55c4_b706_ce18f6061e7e);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICommandBarFlyoutCommandBar_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub FlyoutTemplateSettings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICommandBarFlyoutCommandBarFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICommandBarFlyoutCommandBarFactory {
    type Vtable = ICommandBarFlyoutCommandBarFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf8236f9f_5559_5697_8e6f_20d70ca17dd0);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICommandBarFlyoutCommandBarFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CreateInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICommandBarFlyoutCommandBarTemplateSettings(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICommandBarFlyoutCommandBarTemplateSettings {
    type Vtable = ICommandBarFlyoutCommandBarTemplateSettings_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x47642c44_26ff_5d14_9cfc_77dc64f3a447);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICommandBarFlyoutCommandBarTemplateSettings_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub OpenAnimationStartPosition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub OpenAnimationEndPosition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub CloseAnimationEndPosition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub CurrentWidth: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub ExpandedWidth: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub WidthExpansionDelta: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub WidthExpansionAnimationStartPosition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub WidthExpansionAnimationEndPosition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub WidthExpansionMoreButtonAnimationStartPosition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub WidthExpansionMoreButtonAnimationEndPosition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub ExpandUpOverflowVerticalPosition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub ExpandDownOverflowVerticalPosition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub ExpandUpAnimationStartPosition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub ExpandUpAnimationEndPosition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub ExpandUpAnimationHoldPosition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub ExpandDownAnimationStartPosition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub ExpandDownAnimationEndPosition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub ExpandDownAnimationHoldPosition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub ContentClipRect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::Rect) -> ::windows_core::HRESULT,
    pub OverflowContentClipRect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::Rect) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICommandBarTemplateSettings(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICommandBarTemplateSettings {
    type Vtable = ICommandBarTemplateSettings_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x61c8f92c_05aa_414a_a2ae_482c5a46c08e);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICommandBarTemplateSettings_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub ContentHeight: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub OverflowContentClipRect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::Rect) -> ::windows_core::HRESULT,
    pub OverflowContentMinWidth: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub OverflowContentMaxHeight: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub OverflowContentHorizontalOffset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub OverflowContentHeight: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub NegativeOverflowContentHeight: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICommandBarTemplateSettings2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICommandBarTemplateSettings2 {
    type Vtable = ICommandBarTemplateSettings2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xfbb24f93_c2e2_4177_a2b6_3cd705073cf6);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICommandBarTemplateSettings2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub OverflowContentMaxWidth: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICommandBarTemplateSettings3(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICommandBarTemplateSettings3 {
    type Vtable = ICommandBarTemplateSettings3_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3bd71eba_3403_4bfe_842d_2ce8c511d245);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICommandBarTemplateSettings3_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub EffectiveOverflowButtonVisibility: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Visibility) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICommandBarTemplateSettings4(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICommandBarTemplateSettings4 {
    type Vtable = ICommandBarTemplateSettings4_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf2562dd3_aa58_59c5_853b_828a19d1214e);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICommandBarTemplateSettings4_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub OverflowContentCompactYTranslation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub OverflowContentMinimalYTranslation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub OverflowContentHiddenYTranslation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDragCompletedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDragCompletedEventArgs {
    type Vtable = IDragCompletedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb04f29a1_bd16_48f6_a511_9c2763641331);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDragCompletedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub HorizontalChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub VerticalChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub Canceled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDragCompletedEventArgsFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDragCompletedEventArgsFactory {
    type Vtable = IDragCompletedEventArgsFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x36a7d99d_148c_495f_a0fc_afc871d62f33);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDragCompletedEventArgsFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CreateInstanceWithHorizontalChangeVerticalChangeAndCanceled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, horizontalchange: f64, verticalchange: f64, canceled: bool, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDragDeltaEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDragDeltaEventArgs {
    type Vtable = IDragDeltaEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2c2dd73c_2806_49fc_aae9_6d792b572b6a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDragDeltaEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub HorizontalChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub VerticalChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDragDeltaEventArgsFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDragDeltaEventArgsFactory {
    type Vtable = IDragDeltaEventArgsFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x46e7a1ef_ae15_44a6_8a04_95b0bf9ab876);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDragDeltaEventArgsFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CreateInstanceWithHorizontalChangeAndVerticalChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, horizontalchange: f64, verticalchange: f64, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDragStartedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDragStartedEventArgs {
    type Vtable = IDragStartedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9f915dd0_a124_4366_bd85_2408214aeed4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDragStartedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub HorizontalOffset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub VerticalOffset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDragStartedEventArgsFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDragStartedEventArgsFactory {
    type Vtable = IDragStartedEventArgsFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5eefe579_c706_4781_a308_c9e7f4c6a1d7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDragStartedEventArgsFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CreateInstanceWithHorizontalOffsetAndVerticalOffset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, horizontaloffset: f64, verticaloffset: f64, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IFlyoutBase(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IFlyoutBase {
    type Vtable = IFlyoutBase_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x723eea0b_d12e_430d_a9f0_9bb32bbf9913);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFlyoutBase_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Placement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut FlyoutPlacementMode) -> ::windows_core::HRESULT,
    pub SetPlacement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: FlyoutPlacementMode) -> ::windows_core::HRESULT,
    pub Opened: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveOpened: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub Closed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveClosed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub Opening: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveOpening: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub ShowAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, placementtarget: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Hide: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IFlyoutBase2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IFlyoutBase2 {
    type Vtable = IFlyoutBase2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf82b435e_65b3_41c6_a9e2_77b67bc4c00c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFlyoutBase2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Target: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub AllowFocusOnInteraction: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetAllowFocusOnInteraction: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub LightDismissOverlayMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::LightDismissOverlayMode) -> ::windows_core::HRESULT,
    pub SetLightDismissOverlayMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::LightDismissOverlayMode) -> ::windows_core::HRESULT,
    pub AllowFocusWhenDisabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetAllowFocusWhenDisabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub ElementSoundMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::ElementSoundMode) -> ::windows_core::HRESULT,
    pub SetElementSoundMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::ElementSoundMode) -> ::windows_core::HRESULT,
    pub Closing: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveClosing: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IFlyoutBase3(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IFlyoutBase3 {
    type Vtable = IFlyoutBase3_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa89c9712_48e0_4240_95b9_0dfd0826a8d3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFlyoutBase3_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub OverlayInputPassThroughElement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetOverlayInputPassThroughElement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IFlyoutBase4(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IFlyoutBase4 {
    type Vtable = IFlyoutBase4_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe3897d69_a37f_4828_9b70_0ef67c03b5f8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFlyoutBase4_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "UI_Xaml_Input")]
    pub TryInvokeKeyboardAccelerator: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, args: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Input"))]
    TryInvokeKeyboardAccelerator: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IFlyoutBase5(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IFlyoutBase5 {
    type Vtable = IFlyoutBase5_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xad3ec0c7_12bb_5a73_b78e_105192ca73d6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFlyoutBase5_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub ShowMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut FlyoutShowMode) -> ::windows_core::HRESULT,
    pub SetShowMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: FlyoutShowMode) -> ::windows_core::HRESULT,
    pub InputDevicePrefersPrimaryCommands: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub AreOpenCloseAnimationsEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetAreOpenCloseAnimationsEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub IsOpen: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub ShowAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, placementtarget: ::windows_core::RawPtr, showoptions: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IFlyoutBase6(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IFlyoutBase6 {
    type Vtable = IFlyoutBase6_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5399de8c_06cc_5b52_b65a_ff9322d1c940);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFlyoutBase6_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub ShouldConstrainToRootBounds: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetShouldConstrainToRootBounds: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub IsConstrainedToRootBounds: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub XamlRoot: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetXamlRoot: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IFlyoutBaseClosingEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IFlyoutBaseClosingEventArgs {
    type Vtable = IFlyoutBaseClosingEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd075852d_b09a_4fd1_b005_db2ba01206fb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFlyoutBaseClosingEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Cancel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetCancel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IFlyoutBaseFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IFlyoutBaseFactory {
    type Vtable = IFlyoutBaseFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1c3363d7_fca7_407e_920e_70e15e9f0bf1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFlyoutBaseFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CreateInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IFlyoutBaseOverrides(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IFlyoutBaseOverrides {
    type Vtable = IFlyoutBaseOverrides_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x101dec86_6f4d_45a4_9d0e_3ece6f16977e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFlyoutBaseOverrides_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CreatePresenter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IFlyoutBaseOverrides4(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IFlyoutBaseOverrides4 {
    type Vtable = IFlyoutBaseOverrides4_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa6bfd04d_5ff3_4418_add8_4042a88d2da5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFlyoutBaseOverrides4_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "UI_Xaml_Input")]
    pub OnProcessKeyboardAccelerators: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, args: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Input"))]
    OnProcessKeyboardAccelerators: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IFlyoutBaseStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IFlyoutBaseStatics {
    type Vtable = IFlyoutBaseStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe2d795e3_85c0_4de2_bac1_5294ca011a78);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFlyoutBaseStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub PlacementProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub AttachedFlyoutProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetAttachedFlyout: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetAttachedFlyout: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows_core::RawPtr, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ShowAttachedFlyout: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, flyoutowner: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IFlyoutBaseStatics2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IFlyoutBaseStatics2 {
    type Vtable = IFlyoutBaseStatics2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa8e913fe_2d60_4307_aad9_56b450121b58);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFlyoutBaseStatics2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub AllowFocusOnInteractionProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub LightDismissOverlayModeProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub AllowFocusWhenDisabledProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ElementSoundModeProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IFlyoutBaseStatics3(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IFlyoutBaseStatics3 {
    type Vtable = IFlyoutBaseStatics3_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7ba92e4f_dd16_4be4_99db_bd9d4406c0f8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFlyoutBaseStatics3_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub OverlayInputPassThroughElementProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IFlyoutBaseStatics5(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IFlyoutBaseStatics5 {
    type Vtable = IFlyoutBaseStatics5_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x69edb25c_992a_542b_bcff_2f7f855523bd);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFlyoutBaseStatics5_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub TargetProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ShowModeProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub InputDevicePrefersPrimaryCommandsProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub AreOpenCloseAnimationsEnabledProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub IsOpenProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IFlyoutBaseStatics6(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IFlyoutBaseStatics6 {
    type Vtable = IFlyoutBaseStatics6_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x96d49254_c91b_5246_8b39_afc2a2c50cf8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFlyoutBaseStatics6_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub ShouldConstrainToRootBoundsProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IFlyoutShowOptions(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IFlyoutShowOptions {
    type Vtable = IFlyoutShowOptions_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x57d693ad_0c74_54dd_b110_1ee43fabadd9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFlyoutShowOptions_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Position: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetPosition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ExclusionRect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetExclusionRect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ShowMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut FlyoutShowMode) -> ::windows_core::HRESULT,
    pub SetShowMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: FlyoutShowMode) -> ::windows_core::HRESULT,
    pub Placement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut FlyoutPlacementMode) -> ::windows_core::HRESULT,
    pub SetPlacement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: FlyoutPlacementMode) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IFlyoutShowOptionsFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IFlyoutShowOptionsFactory {
    type Vtable = IFlyoutShowOptionsFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xce596f61_2eb4_5b4e_af69_f9af42320eee);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFlyoutShowOptionsFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CreateInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGeneratorPositionHelper(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGeneratorPositionHelper {
    type Vtable = IGeneratorPositionHelper_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xcd40318d_7745_40d9_ab9d_abbda4a7ffea);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGeneratorPositionHelper_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGeneratorPositionHelperStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGeneratorPositionHelperStatics {
    type Vtable = IGeneratorPositionHelperStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xad4095cd_60ec_4588_8d60_39d29097a4df);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGeneratorPositionHelperStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub FromIndexAndOffset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: i32, offset: i32, result__: *mut GeneratorPosition) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGridViewItemPresenter(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGridViewItemPresenter {
    type Vtable = IGridViewItemPresenter_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x214f9010_56e2_4821_8a1c_2305709af94b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGridViewItemPresenter_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub SelectionCheckMarkVisualEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetSelectionCheckMarkVisualEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    #[cfg(feature = "UI_Xaml_Media")]
    pub CheckHintBrush: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    CheckHintBrush: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetCheckHintBrush: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetCheckHintBrush: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub CheckSelectingBrush: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    CheckSelectingBrush: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetCheckSelectingBrush: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetCheckSelectingBrush: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub CheckBrush: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    CheckBrush: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetCheckBrush: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetCheckBrush: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub DragBackground: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    DragBackground: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetDragBackground: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetDragBackground: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub DragForeground: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    DragForeground: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetDragForeground: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetDragForeground: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub FocusBorderBrush: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    FocusBorderBrush: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetFocusBorderBrush: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetFocusBorderBrush: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub PlaceholderBackground: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    PlaceholderBackground: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetPlaceholderBackground: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetPlaceholderBackground: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub PointerOverBackground: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    PointerOverBackground: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetPointerOverBackground: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetPointerOverBackground: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SelectedBackground: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SelectedBackground: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetSelectedBackground: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetSelectedBackground: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SelectedForeground: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SelectedForeground: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetSelectedForeground: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetSelectedForeground: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SelectedPointerOverBackground: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SelectedPointerOverBackground: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetSelectedPointerOverBackground: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetSelectedPointerOverBackground: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SelectedPointerOverBorderBrush: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SelectedPointerOverBorderBrush: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetSelectedPointerOverBorderBrush: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetSelectedPointerOverBorderBrush: usize,
    pub SelectedBorderThickness: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Thickness) -> ::windows_core::HRESULT,
    pub SetSelectedBorderThickness: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Thickness) -> ::windows_core::HRESULT,
    pub DisabledOpacity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub SetDisabledOpacity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows_core::HRESULT,
    pub DragOpacity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub SetDragOpacity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows_core::HRESULT,
    pub ReorderHintOffset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub SetReorderHintOffset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows_core::HRESULT,
    #[cfg(feature = "deprecated")]
    pub GridViewItemPresenterHorizontalContentAlignment: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::HorizontalAlignment) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    GridViewItemPresenterHorizontalContentAlignment: usize,
    #[cfg(feature = "deprecated")]
    pub SetGridViewItemPresenterHorizontalContentAlignment: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::HorizontalAlignment) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SetGridViewItemPresenterHorizontalContentAlignment: usize,
    #[cfg(feature = "deprecated")]
    pub GridViewItemPresenterVerticalContentAlignment: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::VerticalAlignment) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    GridViewItemPresenterVerticalContentAlignment: usize,
    #[cfg(feature = "deprecated")]
    pub SetGridViewItemPresenterVerticalContentAlignment: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::VerticalAlignment) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SetGridViewItemPresenterVerticalContentAlignment: usize,
    #[cfg(feature = "deprecated")]
    pub GridViewItemPresenterPadding: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Thickness) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    GridViewItemPresenterPadding: usize,
    #[cfg(feature = "deprecated")]
    pub SetGridViewItemPresenterPadding: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Thickness) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SetGridViewItemPresenterPadding: usize,
    pub PointerOverBackgroundMargin: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Thickness) -> ::windows_core::HRESULT,
    pub SetPointerOverBackgroundMargin: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Thickness) -> ::windows_core::HRESULT,
    pub ContentMargin: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Thickness) -> ::windows_core::HRESULT,
    pub SetContentMargin: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Thickness) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGridViewItemPresenterFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGridViewItemPresenterFactory {
    type Vtable = IGridViewItemPresenterFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x53c12178_63bb_4a65_a3f1_ab114cfc6ffe);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGridViewItemPresenterFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CreateInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGridViewItemPresenterStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGridViewItemPresenterStatics {
    type Vtable = IGridViewItemPresenterStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe958f8c4_277e_4a72_a01e_9e1688980178);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGridViewItemPresenterStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub SelectionCheckMarkVisualEnabledProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CheckHintBrushProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CheckSelectingBrushProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CheckBrushProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub DragBackgroundProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub DragForegroundProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub FocusBorderBrushProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub PlaceholderBackgroundProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub PointerOverBackgroundProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SelectedBackgroundProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SelectedForegroundProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SelectedPointerOverBackgroundProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SelectedPointerOverBorderBrushProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SelectedBorderThicknessProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub DisabledOpacityProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub DragOpacityProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ReorderHintOffsetProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "deprecated")]
    pub GridViewItemPresenterHorizontalContentAlignmentProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    GridViewItemPresenterHorizontalContentAlignmentProperty: usize,
    #[cfg(feature = "deprecated")]
    pub GridViewItemPresenterVerticalContentAlignmentProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    GridViewItemPresenterVerticalContentAlignmentProperty: usize,
    #[cfg(feature = "deprecated")]
    pub GridViewItemPresenterPaddingProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    GridViewItemPresenterPaddingProperty: usize,
    pub PointerOverBackgroundMarginProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ContentMarginProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGridViewItemTemplateSettings(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGridViewItemTemplateSettings {
    type Vtable = IGridViewItemTemplateSettings_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9e30baaf_165d_4267_a45e_1a43a75706ac);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGridViewItemTemplateSettings_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub DragItemsCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IItemsChangedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IItemsChangedEventArgs {
    type Vtable = IItemsChangedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe8b45568_7d10_421e_be29_81839a91de20);
}
#[repr(C)]
#[doc(hidden)]
pub struct IItemsChangedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Action: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub Position: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut GeneratorPosition) -> ::windows_core::HRESULT,
    pub OldPosition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut GeneratorPosition) -> ::windows_core::HRESULT,
    pub ItemCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub ItemUICount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IJumpListItemBackgroundConverter(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IJumpListItemBackgroundConverter {
    type Vtable = IJumpListItemBackgroundConverter_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x81177858_d224_410c_b16c_c5b6bb6188b2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IJumpListItemBackgroundConverter_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "UI_Xaml_Media")]
    pub Enabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    Enabled: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetEnabled: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub Disabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    Disabled: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetDisabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetDisabled: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IJumpListItemBackgroundConverterStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IJumpListItemBackgroundConverterStatics {
    type Vtable = IJumpListItemBackgroundConverterStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x20e7c3dd_6f27_4808_b0be_83e0e9b5cc45);
}
#[repr(C)]
#[doc(hidden)]
pub struct IJumpListItemBackgroundConverterStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub EnabledProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub DisabledProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IJumpListItemForegroundConverter(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IJumpListItemForegroundConverter {
    type Vtable = IJumpListItemForegroundConverter_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1590ed38_c504_4796_a63a_5bfc9eefaae8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IJumpListItemForegroundConverter_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "UI_Xaml_Media")]
    pub Enabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    Enabled: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetEnabled: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub Disabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    Disabled: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetDisabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetDisabled: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IJumpListItemForegroundConverterStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IJumpListItemForegroundConverterStatics {
    type Vtable = IJumpListItemForegroundConverterStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x474e7352_210c_4673_ac6a_413f0e2c7750);
}
#[repr(C)]
#[doc(hidden)]
pub struct IJumpListItemForegroundConverterStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub EnabledProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub DisabledProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILayoutInformation(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ILayoutInformation {
    type Vtable = ILayoutInformation_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb5384c9b_c8cf_41b3_bf16_18c8420e72c9);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILayoutInformation_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILayoutInformationStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ILayoutInformationStatics {
    type Vtable = ILayoutInformationStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xcf06cf99_58e9_4682_8326_50caab65ed7c);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILayoutInformationStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub GetLayoutExceptionElement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dispatcher: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetLayoutSlot: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::Rect) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILayoutInformationStatics2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ILayoutInformationStatics2 {
    type Vtable = ILayoutInformationStatics2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x760315b5_6d4e_4939_ac61_639863cea36b);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILayoutInformationStatics2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub GetAvailableSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::Size) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IListViewItemPresenter(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IListViewItemPresenter {
    type Vtable = IListViewItemPresenter_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xfc8946bd_a3a2_4969_8174_25b5d3c28033);
}
#[repr(C)]
#[doc(hidden)]
pub struct IListViewItemPresenter_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub SelectionCheckMarkVisualEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetSelectionCheckMarkVisualEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    #[cfg(feature = "UI_Xaml_Media")]
    pub CheckHintBrush: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    CheckHintBrush: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetCheckHintBrush: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetCheckHintBrush: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub CheckSelectingBrush: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    CheckSelectingBrush: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetCheckSelectingBrush: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetCheckSelectingBrush: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub CheckBrush: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    CheckBrush: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetCheckBrush: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetCheckBrush: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub DragBackground: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    DragBackground: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetDragBackground: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetDragBackground: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub DragForeground: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    DragForeground: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetDragForeground: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetDragForeground: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub FocusBorderBrush: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    FocusBorderBrush: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetFocusBorderBrush: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetFocusBorderBrush: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub PlaceholderBackground: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    PlaceholderBackground: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetPlaceholderBackground: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetPlaceholderBackground: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub PointerOverBackground: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    PointerOverBackground: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetPointerOverBackground: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetPointerOverBackground: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SelectedBackground: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SelectedBackground: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetSelectedBackground: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetSelectedBackground: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SelectedForeground: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SelectedForeground: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetSelectedForeground: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetSelectedForeground: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SelectedPointerOverBackground: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SelectedPointerOverBackground: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetSelectedPointerOverBackground: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetSelectedPointerOverBackground: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SelectedPointerOverBorderBrush: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SelectedPointerOverBorderBrush: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetSelectedPointerOverBorderBrush: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetSelectedPointerOverBorderBrush: usize,
    pub SelectedBorderThickness: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Thickness) -> ::windows_core::HRESULT,
    pub SetSelectedBorderThickness: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Thickness) -> ::windows_core::HRESULT,
    pub DisabledOpacity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub SetDisabledOpacity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows_core::HRESULT,
    pub DragOpacity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub SetDragOpacity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows_core::HRESULT,
    pub ReorderHintOffset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub SetReorderHintOffset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows_core::HRESULT,
    #[cfg(feature = "deprecated")]
    pub ListViewItemPresenterHorizontalContentAlignment: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::HorizontalAlignment) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    ListViewItemPresenterHorizontalContentAlignment: usize,
    #[cfg(feature = "deprecated")]
    pub SetListViewItemPresenterHorizontalContentAlignment: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::HorizontalAlignment) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SetListViewItemPresenterHorizontalContentAlignment: usize,
    #[cfg(feature = "deprecated")]
    pub ListViewItemPresenterVerticalContentAlignment: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::VerticalAlignment) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    ListViewItemPresenterVerticalContentAlignment: usize,
    #[cfg(feature = "deprecated")]
    pub SetListViewItemPresenterVerticalContentAlignment: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::VerticalAlignment) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SetListViewItemPresenterVerticalContentAlignment: usize,
    #[cfg(feature = "deprecated")]
    pub ListViewItemPresenterPadding: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Thickness) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    ListViewItemPresenterPadding: usize,
    #[cfg(feature = "deprecated")]
    pub SetListViewItemPresenterPadding: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Thickness) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SetListViewItemPresenterPadding: usize,
    pub PointerOverBackgroundMargin: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Thickness) -> ::windows_core::HRESULT,
    pub SetPointerOverBackgroundMargin: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Thickness) -> ::windows_core::HRESULT,
    pub ContentMargin: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Thickness) -> ::windows_core::HRESULT,
    pub SetContentMargin: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Thickness) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IListViewItemPresenter2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IListViewItemPresenter2 {
    type Vtable = IListViewItemPresenter2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf5dc5496_e122_4c57_a625_ac4b08fb2d4c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IListViewItemPresenter2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SelectedPressedBackground: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SelectedPressedBackground: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetSelectedPressedBackground: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetSelectedPressedBackground: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub PressedBackground: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    PressedBackground: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetPressedBackground: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetPressedBackground: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub CheckBoxBrush: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    CheckBoxBrush: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetCheckBoxBrush: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetCheckBoxBrush: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub FocusSecondaryBorderBrush: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    FocusSecondaryBorderBrush: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetFocusSecondaryBorderBrush: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetFocusSecondaryBorderBrush: usize,
    pub CheckMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ListViewItemPresenterCheckMode) -> ::windows_core::HRESULT,
    pub SetCheckMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ListViewItemPresenterCheckMode) -> ::windows_core::HRESULT,
    #[cfg(feature = "UI_Xaml_Media")]
    pub PointerOverForeground: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    PointerOverForeground: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetPointerOverForeground: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetPointerOverForeground: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IListViewItemPresenter3(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IListViewItemPresenter3 {
    type Vtable = IListViewItemPresenter3_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x36620013_0390_4e30_ad97_8744404f7010);
}
#[repr(C)]
#[doc(hidden)]
pub struct IListViewItemPresenter3_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "UI_Xaml_Media")]
    pub RevealBackground: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    RevealBackground: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetRevealBackground: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetRevealBackground: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub RevealBorderBrush: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    RevealBorderBrush: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetRevealBorderBrush: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetRevealBorderBrush: usize,
    pub RevealBorderThickness: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Thickness) -> ::windows_core::HRESULT,
    pub SetRevealBorderThickness: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Thickness) -> ::windows_core::HRESULT,
    pub RevealBackgroundShowsAboveContent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetRevealBackgroundShowsAboveContent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IListViewItemPresenter4(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IListViewItemPresenter4 {
    type Vtable = IListViewItemPresenter4_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xda600ac1_adea_5940_a18f_57582f96d99a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IListViewItemPresenter4_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SelectedDisabledBackground: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SelectedDisabledBackground: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetSelectedDisabledBackground: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetSelectedDisabledBackground: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub CheckPressedBrush: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    CheckPressedBrush: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetCheckPressedBrush: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetCheckPressedBrush: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub CheckDisabledBrush: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    CheckDisabledBrush: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetCheckDisabledBrush: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetCheckDisabledBrush: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub CheckBoxPointerOverBrush: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    CheckBoxPointerOverBrush: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetCheckBoxPointerOverBrush: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetCheckBoxPointerOverBrush: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub CheckBoxPressedBrush: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    CheckBoxPressedBrush: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetCheckBoxPressedBrush: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetCheckBoxPressedBrush: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub CheckBoxDisabledBrush: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    CheckBoxDisabledBrush: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetCheckBoxDisabledBrush: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetCheckBoxDisabledBrush: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub CheckBoxSelectedBrush: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    CheckBoxSelectedBrush: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetCheckBoxSelectedBrush: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetCheckBoxSelectedBrush: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub CheckBoxSelectedPointerOverBrush: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    CheckBoxSelectedPointerOverBrush: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetCheckBoxSelectedPointerOverBrush: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetCheckBoxSelectedPointerOverBrush: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub CheckBoxSelectedPressedBrush: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    CheckBoxSelectedPressedBrush: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetCheckBoxSelectedPressedBrush: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetCheckBoxSelectedPressedBrush: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub CheckBoxSelectedDisabledBrush: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    CheckBoxSelectedDisabledBrush: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetCheckBoxSelectedDisabledBrush: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetCheckBoxSelectedDisabledBrush: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub CheckBoxBorderBrush: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    CheckBoxBorderBrush: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetCheckBoxBorderBrush: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetCheckBoxBorderBrush: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub CheckBoxPointerOverBorderBrush: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    CheckBoxPointerOverBorderBrush: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetCheckBoxPointerOverBorderBrush: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetCheckBoxPointerOverBorderBrush: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub CheckBoxPressedBorderBrush: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    CheckBoxPressedBorderBrush: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetCheckBoxPressedBorderBrush: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetCheckBoxPressedBorderBrush: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub CheckBoxDisabledBorderBrush: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    CheckBoxDisabledBorderBrush: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetCheckBoxDisabledBorderBrush: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetCheckBoxDisabledBorderBrush: usize,
    pub CheckBoxCornerRadius: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::CornerRadius) -> ::windows_core::HRESULT,
    pub SetCheckBoxCornerRadius: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::CornerRadius) -> ::windows_core::HRESULT,
    pub SelectionIndicatorCornerRadius: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::CornerRadius) -> ::windows_core::HRESULT,
    pub SetSelectionIndicatorCornerRadius: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::CornerRadius) -> ::windows_core::HRESULT,
    pub SelectionIndicatorVisualEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetSelectionIndicatorVisualEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub SelectionIndicatorMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ListViewItemPresenterSelectionIndicatorMode) -> ::windows_core::HRESULT,
    pub SetSelectionIndicatorMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ListViewItemPresenterSelectionIndicatorMode) -> ::windows_core::HRESULT,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SelectionIndicatorBrush: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SelectionIndicatorBrush: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetSelectionIndicatorBrush: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetSelectionIndicatorBrush: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SelectionIndicatorPointerOverBrush: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SelectionIndicatorPointerOverBrush: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetSelectionIndicatorPointerOverBrush: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetSelectionIndicatorPointerOverBrush: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SelectionIndicatorPressedBrush: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SelectionIndicatorPressedBrush: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetSelectionIndicatorPressedBrush: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetSelectionIndicatorPressedBrush: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SelectionIndicatorDisabledBrush: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SelectionIndicatorDisabledBrush: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetSelectionIndicatorDisabledBrush: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetSelectionIndicatorDisabledBrush: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SelectedBorderBrush: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SelectedBorderBrush: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetSelectedBorderBrush: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetSelectedBorderBrush: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SelectedPressedBorderBrush: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SelectedPressedBorderBrush: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetSelectedPressedBorderBrush: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetSelectedPressedBorderBrush: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SelectedDisabledBorderBrush: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SelectedDisabledBorderBrush: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetSelectedDisabledBorderBrush: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetSelectedDisabledBorderBrush: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SelectedInnerBorderBrush: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SelectedInnerBorderBrush: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetSelectedInnerBorderBrush: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetSelectedInnerBorderBrush: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub PointerOverBorderBrush: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    PointerOverBorderBrush: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetPointerOverBorderBrush: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetPointerOverBorderBrush: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IListViewItemPresenterFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IListViewItemPresenterFactory {
    type Vtable = IListViewItemPresenterFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe0777cfd_f7e4_4a67_9ac0_a994fcacd020);
}
#[repr(C)]
#[doc(hidden)]
pub struct IListViewItemPresenterFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CreateInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IListViewItemPresenterStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IListViewItemPresenterStatics {
    type Vtable = IListViewItemPresenterStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6504a55a_15dd_42fb_aa5d_2d8ce2e9c294);
}
#[repr(C)]
#[doc(hidden)]
pub struct IListViewItemPresenterStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub SelectionCheckMarkVisualEnabledProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CheckHintBrushProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CheckSelectingBrushProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CheckBrushProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub DragBackgroundProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub DragForegroundProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub FocusBorderBrushProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub PlaceholderBackgroundProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub PointerOverBackgroundProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SelectedBackgroundProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SelectedForegroundProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SelectedPointerOverBackgroundProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SelectedPointerOverBorderBrushProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SelectedBorderThicknessProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub DisabledOpacityProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub DragOpacityProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ReorderHintOffsetProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "deprecated")]
    pub ListViewItemPresenterHorizontalContentAlignmentProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    ListViewItemPresenterHorizontalContentAlignmentProperty: usize,
    #[cfg(feature = "deprecated")]
    pub ListViewItemPresenterVerticalContentAlignmentProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    ListViewItemPresenterVerticalContentAlignmentProperty: usize,
    #[cfg(feature = "deprecated")]
    pub ListViewItemPresenterPaddingProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    ListViewItemPresenterPaddingProperty: usize,
    pub PointerOverBackgroundMarginProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ContentMarginProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IListViewItemPresenterStatics2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IListViewItemPresenterStatics2 {
    type Vtable = IListViewItemPresenterStatics2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4cb3b945_d24d_42a3_9e83_a86d0618bf1b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IListViewItemPresenterStatics2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub SelectedPressedBackgroundProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub PressedBackgroundProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CheckBoxBrushProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub FocusSecondaryBorderBrushProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CheckModeProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub PointerOverForegroundProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IListViewItemPresenterStatics3(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IListViewItemPresenterStatics3 {
    type Vtable = IListViewItemPresenterStatics3_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc3d3d11e_fa26_4ce7_a4ed_ff948f01b7c0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IListViewItemPresenterStatics3_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub RevealBackgroundProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub RevealBorderBrushProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub RevealBorderThicknessProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub RevealBackgroundShowsAboveContentProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IListViewItemPresenterStatics4(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IListViewItemPresenterStatics4 {
    type Vtable = IListViewItemPresenterStatics4_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3917159e_74a1_5e7e_a743_e45be9fb919b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IListViewItemPresenterStatics4_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub SelectedDisabledBackgroundProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CheckPressedBrushProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CheckDisabledBrushProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CheckBoxPointerOverBrushProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CheckBoxPressedBrushProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CheckBoxDisabledBrushProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CheckBoxSelectedBrushProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CheckBoxSelectedPointerOverBrushProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CheckBoxSelectedPressedBrushProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CheckBoxSelectedDisabledBrushProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CheckBoxBorderBrushProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CheckBoxPointerOverBorderBrushProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CheckBoxPressedBorderBrushProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CheckBoxDisabledBorderBrushProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CheckBoxCornerRadiusProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SelectionIndicatorCornerRadiusProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SelectionIndicatorVisualEnabledProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SelectionIndicatorModeProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SelectionIndicatorBrushProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SelectionIndicatorPointerOverBrushProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SelectionIndicatorPressedBrushProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SelectionIndicatorDisabledBrushProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SelectedBorderBrushProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SelectedPressedBorderBrushProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SelectedDisabledBorderBrushProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SelectedInnerBorderBrushProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub PointerOverBorderBrushProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IListViewItemTemplateSettings(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IListViewItemTemplateSettings {
    type Vtable = IListViewItemTemplateSettings_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x67af84bf_8279_4686_9326_cd189f27575d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IListViewItemTemplateSettings_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub DragItemsCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILoopingSelector(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ILoopingSelector {
    type Vtable = ILoopingSelector_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4c9a3e04_4827_49d9_8806_093957b0fd21);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILoopingSelector_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub ShouldLoop: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetShouldLoop: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Items: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Items: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub SetItems: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SetItems: usize,
    pub SelectedIndex: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub SetSelectedIndex: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows_core::HRESULT,
    pub SelectedItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetSelectedItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub ItemWidth: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub SetItemWidth: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows_core::HRESULT,
    pub ItemHeight: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub SetItemHeight: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows_core::HRESULT,
    pub ItemTemplate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetItemTemplate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SelectionChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveSelectionChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILoopingSelectorItem(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ILoopingSelectorItem {
    type Vtable = ILoopingSelectorItem_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc69714b9_27c6_4433_9d7c_0dbfb2f4344f);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILoopingSelectorItem_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILoopingSelectorPanel(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ILoopingSelectorPanel {
    type Vtable = ILoopingSelectorPanel_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x40a9ba70_1011_4778_87f7_6bfd20d6377d);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILoopingSelectorPanel_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILoopingSelectorStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ILoopingSelectorStatics {
    type Vtable = ILoopingSelectorStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x03e8bafa_8c7d_4fc5_b92a_f049fb933cc5);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILoopingSelectorStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub ShouldLoopProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ItemsProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SelectedIndexProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SelectedItemProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ItemWidthProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ItemHeightProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ItemTemplateProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMenuFlyoutItemTemplateSettings(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMenuFlyoutItemTemplateSettings {
    type Vtable = IMenuFlyoutItemTemplateSettings_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x56ad1809_3a16_4147_81cb_d0b35c834e0f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMenuFlyoutItemTemplateSettings_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub KeyboardAcceleratorTextMinWidth: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMenuFlyoutPresenterTemplateSettings(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMenuFlyoutPresenterTemplateSettings {
    type Vtable = IMenuFlyoutPresenterTemplateSettings_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd68fd00d_629d_4349_ac51_b877c80983b8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMenuFlyoutPresenterTemplateSettings_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub FlyoutContentMinWidth: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct INavigationViewItemPresenter(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for INavigationViewItemPresenter {
    type Vtable = INavigationViewItemPresenter_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9956d3fc_4693_59cb_b6bf_37249058be96);
}
#[repr(C)]
#[doc(hidden)]
pub struct INavigationViewItemPresenter_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Icon: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetIcon: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct INavigationViewItemPresenterFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for INavigationViewItemPresenterFactory {
    type Vtable = INavigationViewItemPresenterFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xbb062c50_4a36_52e7_9459_e89d02f3fc42);
}
#[repr(C)]
#[doc(hidden)]
pub struct INavigationViewItemPresenterFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CreateInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct INavigationViewItemPresenterStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for INavigationViewItemPresenterStatics {
    type Vtable = INavigationViewItemPresenterStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x52814604_cfc1_5ad5_a3aa_fa355be6bd76);
}
#[repr(C)]
#[doc(hidden)]
pub struct INavigationViewItemPresenterStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub IconProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IOrientedVirtualizingPanel(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IOrientedVirtualizingPanel {
    type Vtable = IOrientedVirtualizingPanel_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf077b577_39bd_46ee_bdd7_0826beed71b8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOrientedVirtualizingPanel_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CanVerticallyScroll: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetCanVerticallyScroll: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub CanHorizontallyScroll: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetCanHorizontallyScroll: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub ExtentWidth: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub ExtentHeight: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub ViewportWidth: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub ViewportHeight: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub HorizontalOffset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub VerticalOffset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub ScrollOwner: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetScrollOwner: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub LineUp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub LineDown: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub LineLeft: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub LineRight: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub PageUp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub PageDown: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub PageLeft: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub PageRight: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub MouseWheelUp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub MouseWheelDown: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub MouseWheelLeft: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub MouseWheelRight: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetHorizontalOffset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, offset: f64) -> ::windows_core::HRESULT,
    pub SetVerticalOffset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, offset: f64) -> ::windows_core::HRESULT,
    pub MakeVisible: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, visual: ::windows_core::RawPtr, rectangle: ::winrt_foundation::Rect, result__: *mut ::winrt_foundation::Rect) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IOrientedVirtualizingPanelFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IOrientedVirtualizingPanelFactory {
    type Vtable = IOrientedVirtualizingPanelFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7b8eaeaf_f92f_439d_9ebf_e9919f56c94d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOrientedVirtualizingPanelFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPickerFlyoutBase(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPickerFlyoutBase {
    type Vtable = IPickerFlyoutBase_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe33574ea_1076_44d1_9383_dc24ac5cff2a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPickerFlyoutBase_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPickerFlyoutBaseFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPickerFlyoutBaseFactory {
    type Vtable = IPickerFlyoutBaseFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7ec27a53_9502_4beb_b342_00566c8f16b0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPickerFlyoutBaseFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CreateInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPickerFlyoutBaseOverrides(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPickerFlyoutBaseOverrides {
    type Vtable = IPickerFlyoutBaseOverrides_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5bfc4f4a_4822_47b4_a958_77c20ba120d3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPickerFlyoutBaseOverrides_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub OnConfirmed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub ShouldShowConfirmationButtons: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPickerFlyoutBaseStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPickerFlyoutBaseStatics {
    type Vtable = IPickerFlyoutBaseStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5a4d0ac5_89ae_40e5_a7f1_bb702355adf3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPickerFlyoutBaseStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub TitleProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetTitle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows_core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetTitle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows_core::RawPtr, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPivotHeaderItem(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPivotHeaderItem {
    type Vtable = IPivotHeaderItem_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x594572c2_82aa_410b_9e55_fd8e2c98862d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPivotHeaderItem_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPivotHeaderItemFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPivotHeaderItemFactory {
    type Vtable = IPivotHeaderItemFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x14308b37_185b_4117_bc77_dda2eb261b99);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPivotHeaderItemFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CreateInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPivotHeaderPanel(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPivotHeaderPanel {
    type Vtable = IPivotHeaderPanel_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x21484ebc_9241_4203_bd37_6c08fb096612);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPivotHeaderPanel_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPivotPanel(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPivotPanel {
    type Vtable = IPivotPanel_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xad4ebe80_22a9_4ca3_9212_2773b6359ff3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPivotPanel_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPopup(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPopup {
    type Vtable = IPopup_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x62418240_e6d3_4705_a1dc_39156456ee29);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPopup_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Child: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetChild: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub IsOpen: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetIsOpen: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub HorizontalOffset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub SetHorizontalOffset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows_core::HRESULT,
    pub VerticalOffset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub SetVerticalOffset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Foundation_Collections", feature = "UI_Xaml_Media_Animation"))]
    pub ChildTransitions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "UI_Xaml_Media_Animation")))]
    ChildTransitions: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "UI_Xaml_Media_Animation"))]
    pub SetChildTransitions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "UI_Xaml_Media_Animation")))]
    SetChildTransitions: usize,
    pub IsLightDismissEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetIsLightDismissEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub Opened: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveOpened: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub Closed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveClosed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPopup2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPopup2 {
    type Vtable = IPopup2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x376a8c4c_aac0_4b20_966a_0b9364feb4b5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPopup2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub LightDismissOverlayMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::LightDismissOverlayMode) -> ::windows_core::HRESULT,
    pub SetLightDismissOverlayMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::LightDismissOverlayMode) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPopup3(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPopup3 {
    type Vtable = IPopup3_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf9c46915_a65c_5f68_9f54_310a1b51095f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPopup3_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub ShouldConstrainToRootBounds: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetShouldConstrainToRootBounds: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub IsConstrainedToRootBounds: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPopup4(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPopup4 {
    type Vtable = IPopup4_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1870b836_df2f_5fc6_a5f2_748ed6ce7321);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPopup4_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub PlacementTarget: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetPlacementTarget: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub DesiredPlacement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PopupPlacementMode) -> ::windows_core::HRESULT,
    pub SetDesiredPlacement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: PopupPlacementMode) -> ::windows_core::HRESULT,
    pub ActualPlacement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PopupPlacementMode) -> ::windows_core::HRESULT,
    pub ActualPlacementChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveActualPlacementChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPopupStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPopupStatics {
    type Vtable = IPopupStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5ae3bf1a_6e34_40d6_8a7f_ca822aaf59e3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPopupStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub ChildProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub IsOpenProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub HorizontalOffsetProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub VerticalOffsetProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ChildTransitionsProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub IsLightDismissEnabledProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPopupStatics2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPopupStatics2 {
    type Vtable = IPopupStatics2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2b9ae9ec_55ef_43b6_b459_12e40ffa4302);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPopupStatics2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub LightDismissOverlayModeProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPopupStatics3(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPopupStatics3 {
    type Vtable = IPopupStatics3_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x00789589_c580_558f_945a_7d02ee004d3e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPopupStatics3_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub ShouldConstrainToRootBoundsProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPopupStatics4(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPopupStatics4 {
    type Vtable = IPopupStatics4_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd1a42c06_8bfa_5164_8554_48bfe6bd4cc6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPopupStatics4_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub PlacementTargetProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub DesiredPlacementProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IProgressBarTemplateSettings(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IProgressBarTemplateSettings {
    type Vtable = IProgressBarTemplateSettings_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3fe2ea2a_e3f2_4c2b_9488_918d77d2bbe4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProgressBarTemplateSettings_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub EllipseDiameter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub EllipseOffset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub EllipseAnimationWellPosition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub EllipseAnimationEndPosition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub ContainerAnimationStartPosition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub ContainerAnimationEndPosition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub IndicatorLengthDelta: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IProgressRingTemplateSettings(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IProgressRingTemplateSettings {
    type Vtable = IProgressRingTemplateSettings_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb9b675ec_c723_42e6_83e9_9826272bdc0e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProgressRingTemplateSettings_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub EllipseDiameter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub EllipseOffset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Thickness) -> ::windows_core::HRESULT,
    pub MaxSideLength: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRangeBase(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IRangeBase {
    type Vtable = IRangeBase_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xfa002c1a_494e_46cf_91d4_e14a8d798675);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRangeBase_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Minimum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub SetMinimum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows_core::HRESULT,
    pub Maximum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub SetMaximum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows_core::HRESULT,
    pub SmallChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub SetSmallChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows_core::HRESULT,
    pub LargeChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub SetLargeChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows_core::HRESULT,
    pub Value: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub SetValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows_core::HRESULT,
    pub ValueChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveValueChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRangeBaseFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IRangeBaseFactory {
    type Vtable = IRangeBaseFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x389b7c71_5220_42b2_9992_2690c1a67030);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRangeBaseFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CreateInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRangeBaseOverrides(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IRangeBaseOverrides {
    type Vtable = IRangeBaseOverrides_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4291af39_7f0b_4bc2_99c4_06e7062682d8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRangeBaseOverrides_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub OnMinimumChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, oldminimum: f64, newminimum: f64) -> ::windows_core::HRESULT,
    pub OnMaximumChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, oldmaximum: f64, newmaximum: f64) -> ::windows_core::HRESULT,
    pub OnValueChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, oldvalue: f64, newvalue: f64) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRangeBaseStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IRangeBaseStatics {
    type Vtable = IRangeBaseStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x67ef17e1_fe37_474f_9e97_3b5e0b30f2e0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRangeBaseStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub MinimumProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub MaximumProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SmallChangeProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub LargeChangeProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ValueProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRangeBaseValueChangedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IRangeBaseValueChangedEventArgs {
    type Vtable = IRangeBaseValueChangedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa1921777_d5c1_4f9c_a7b0_0401b7e6dc5c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRangeBaseValueChangedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub OldValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub NewValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRepeatButton(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IRepeatButton {
    type Vtable = IRepeatButton_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x02200df9_021a_484a_a93b_0f31020314e5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRepeatButton_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Delay: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub SetDelay: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows_core::HRESULT,
    pub Interval: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub SetInterval: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRepeatButtonStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IRepeatButtonStatics {
    type Vtable = IRepeatButtonStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3914ac4e_f462_4f73_8197_e8846639c682);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRepeatButtonStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub DelayProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub IntervalProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IScrollBar(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IScrollBar {
    type Vtable = IScrollBar_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf57ae6ca_d1a6_4b90_a4e9_54df1ba8d2ec);
}
#[repr(C)]
#[doc(hidden)]
pub struct IScrollBar_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Orientation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::Orientation) -> ::windows_core::HRESULT,
    pub SetOrientation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::Orientation) -> ::windows_core::HRESULT,
    pub ViewportSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub SetViewportSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows_core::HRESULT,
    pub IndicatorMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ScrollingIndicatorMode) -> ::windows_core::HRESULT,
    pub SetIndicatorMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ScrollingIndicatorMode) -> ::windows_core::HRESULT,
    pub Scroll: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveScroll: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IScrollBarStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IScrollBarStatics {
    type Vtable = IScrollBarStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x45eaf38d_b814_48cf_97f2_539eb16dfd4d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IScrollBarStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub OrientationProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ViewportSizeProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub IndicatorModeProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IScrollEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IScrollEventArgs {
    type Vtable = IScrollEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc57e5168_3afe_448d_b752_2f364c75d743);
}
#[repr(C)]
#[doc(hidden)]
pub struct IScrollEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub NewValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub ScrollEventType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ScrollEventType) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IScrollSnapPointsInfo(::windows_core::IUnknown);
impl IScrollSnapPointsInfo {
    pub fn AreHorizontalSnapPointsRegular(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).AreHorizontalSnapPointsRegular)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn AreVerticalSnapPointsRegular(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).AreVerticalSnapPointsRegular)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn HorizontalSnapPointsChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventHandler<::windows_core::IInspectable>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).HorizontalSnapPointsChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveHorizontalSnapPointsChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveHorizontalSnapPointsChanged)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn VerticalSnapPointsChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventHandler<::windows_core::IInspectable>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).VerticalSnapPointsChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveVerticalSnapPointsChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveVerticalSnapPointsChanged)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetIrregularSnapPoints(&self, orientation: super::Orientation, alignment: SnapPointsAlignment) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<f32>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetIrregularSnapPoints)(::windows_core::Interface::as_raw(this), orientation, alignment, result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<f32>>(result__)
        }
    }
    pub fn GetRegularSnapPoints(&self, orientation: super::Orientation, alignment: SnapPointsAlignment, offset: &mut f32) -> ::windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f32>::zeroed();
            (::windows_core::Interface::vtable(this).GetRegularSnapPoints)(::windows_core::Interface::as_raw(this), orientation, alignment, offset, result__.as_mut_ptr()).from_abi::<f32>(result__)
        }
    }
}
impl ::core::convert::From<IScrollSnapPointsInfo> for ::windows_core::IUnknown {
    fn from(value: IScrollSnapPointsInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IScrollSnapPointsInfo> for ::windows_core::IUnknown {
    fn from(value: &IScrollSnapPointsInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IScrollSnapPointsInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IScrollSnapPointsInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IScrollSnapPointsInfo> for ::windows_core::IInspectable {
    fn from(value: IScrollSnapPointsInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IScrollSnapPointsInfo> for ::windows_core::IInspectable {
    fn from(value: &IScrollSnapPointsInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for IScrollSnapPointsInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a IScrollSnapPointsInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IScrollSnapPointsInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IScrollSnapPointsInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IScrollSnapPointsInfo {}
impl ::core::fmt::Debug for IScrollSnapPointsInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IScrollSnapPointsInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for IScrollSnapPointsInfo {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"{1b5d1336-e61b-4d51-be41-fd8ddc55c58c}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for IScrollSnapPointsInfo {
    type Vtable = IScrollSnapPointsInfo_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1b5d1336_e61b_4d51_be41_fd8ddc55c58c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IScrollSnapPointsInfo_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub AreHorizontalSnapPointsRegular: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub AreVerticalSnapPointsRegular: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub HorizontalSnapPointsChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveHorizontalSnapPointsChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub VerticalSnapPointsChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveVerticalSnapPointsChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub GetIrregularSnapPoints: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, orientation: super::Orientation, alignment: SnapPointsAlignment, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetIrregularSnapPoints: usize,
    pub GetRegularSnapPoints: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, orientation: super::Orientation, alignment: SnapPointsAlignment, offset: *mut f32, result__: *mut f32) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISelector(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISelector {
    type Vtable = ISelector_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe30eb3a5_b36b_42dc_8527_cd25136c083c);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISelector_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub SelectedIndex: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub SetSelectedIndex: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows_core::HRESULT,
    pub SelectedItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetSelectedItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SelectedValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetSelectedValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SelectedValuePath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetSelectedValuePath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub IsSynchronizedWithCurrentItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetIsSynchronizedWithCurrentItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SelectionChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveSelectionChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISelectorFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISelectorFactory {
    type Vtable = ISelectorFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc9be2995_d136_4600_b187_8ad56079b48a);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISelectorFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISelectorItem(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISelectorItem {
    type Vtable = ISelectorItem_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x541c8d6c_0283_4581_b945_2a64c28a0646);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISelectorItem_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub IsSelected: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetIsSelected: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISelectorItemFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISelectorItemFactory {
    type Vtable = ISelectorItemFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb9363945_c86a_4b1e_9440_1879378d5313);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISelectorItemFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CreateInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISelectorItemStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISelectorItemStatics {
    type Vtable = ISelectorItemStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2a353ab8_cbe9_4303_92e7_c8906e218392);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISelectorItemStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub IsSelectedProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISelectorStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISelectorStatics {
    type Vtable = ISelectorStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x13300b06_bd10_4e09_bff7_71efb8bbb42b);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISelectorStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub SelectedIndexProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SelectedItemProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SelectedValueProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SelectedValuePathProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub IsSynchronizedWithCurrentItemProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetIsSelectionActive: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows_core::RawPtr, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISettingsFlyoutTemplateSettings(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISettingsFlyoutTemplateSettings {
    type Vtable = ISettingsFlyoutTemplateSettings_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xbcf14c10_cea7_43f1_9d68_57605ded69d4);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISettingsFlyoutTemplateSettings_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "UI_Xaml_Media")]
    pub HeaderBackground: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    HeaderBackground: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub HeaderForeground: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    HeaderForeground: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub BorderBrush: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    BorderBrush: usize,
    pub BorderThickness: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Thickness) -> ::windows_core::HRESULT,
    #[cfg(feature = "UI_Xaml_Media")]
    pub IconSource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    IconSource: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "UI_Xaml_Media_Animation"))]
    pub ContentTransitions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "UI_Xaml_Media_Animation")))]
    ContentTransitions: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISplitViewTemplateSettings(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISplitViewTemplateSettings {
    type Vtable = ISplitViewTemplateSettings_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc16ab5a7_4996_4443_b199_6b6b89124eab);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISplitViewTemplateSettings_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub OpenPaneLength: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub NegativeOpenPaneLength: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub OpenPaneLengthMinusCompactLength: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub NegativeOpenPaneLengthMinusCompactLength: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub OpenPaneGridLength: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::GridLength) -> ::windows_core::HRESULT,
    pub CompactPaneGridLength: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::GridLength) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IThumb(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IThumb {
    type Vtable = IThumb_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe8b2b281_0d6a_45cf_b333_2402b037f099);
}
#[repr(C)]
#[doc(hidden)]
pub struct IThumb_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub IsDragging: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub DragStarted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveDragStarted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub DragDelta: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveDragDelta: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub DragCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveDragCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub CancelDrag: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IThumbStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IThumbStatics {
    type Vtable = IThumbStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x955024eb_36f3_4672_a186_baaf626ac4ad);
}
#[repr(C)]
#[doc(hidden)]
pub struct IThumbStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub IsDraggingProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITickBar(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ITickBar {
    type Vtable = ITickBar_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x994683fa_f1f6_487d_a5ac_c15921bfa995);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITickBar_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "UI_Xaml_Media")]
    pub Fill: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    Fill: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetFill: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetFill: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITickBarStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ITickBarStatics {
    type Vtable = ITickBarStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2c6d7e40_799d_4a54_be09_1fefc61d018e);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITickBarStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub FillProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IToggleButton(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IToggleButton {
    type Vtable = IToggleButton_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x589877fb_0fc7_4036_9d8b_127dfa75c16d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IToggleButton_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub IsChecked: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetIsChecked: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub IsThreeState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetIsThreeState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub Checked: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveChecked: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub Unchecked: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveUnchecked: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub Indeterminate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveIndeterminate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IToggleButtonFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IToggleButtonFactory {
    type Vtable = IToggleButtonFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd56aa2fc_fc7f_449c_9855_7a1055d668a8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IToggleButtonFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CreateInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IToggleButtonOverrides(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IToggleButtonOverrides {
    type Vtable = IToggleButtonOverrides_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd20e4c28_f18b_491a_9a45_f1a04a9369a4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IToggleButtonOverrides_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub OnToggle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IToggleButtonStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IToggleButtonStatics {
    type Vtable = IToggleButtonStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xaf1eab12_0128_4f67_9c5a_82320c445d19);
}
#[repr(C)]
#[doc(hidden)]
pub struct IToggleButtonStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub IsCheckedProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub IsThreeStateProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IToggleSwitchTemplateSettings(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IToggleSwitchTemplateSettings {
    type Vtable = IToggleSwitchTemplateSettings_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x02b7bdcd_628a_4363_86e0_51d6e2e89e58);
}
#[repr(C)]
#[doc(hidden)]
pub struct IToggleSwitchTemplateSettings_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub KnobCurrentToOnOffset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub KnobCurrentToOffOffset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub KnobOnToOffOffset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub KnobOffToOnOffset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub CurtainCurrentToOnOffset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub CurtainCurrentToOffOffset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub CurtainOnToOffOffset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub CurtainOffToOnOffset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IToolTipTemplateSettings(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IToolTipTemplateSettings {
    type Vtable = IToolTipTemplateSettings_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd4388247_0ec4_4506_affd_afac2225b48c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IToolTipTemplateSettings_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub FromHorizontalOffset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub FromVerticalOffset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct ItemsChangedEventArgs(::windows_core::IUnknown);
impl ItemsChangedEventArgs {
    pub fn Action(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows_core::Interface::vtable(this).Action)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    pub fn Position(&self) -> ::windows_core::Result<GeneratorPosition> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<GeneratorPosition>::zeroed();
            (::windows_core::Interface::vtable(this).Position)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<GeneratorPosition>(result__)
        }
    }
    pub fn OldPosition(&self) -> ::windows_core::Result<GeneratorPosition> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<GeneratorPosition>::zeroed();
            (::windows_core::Interface::vtable(this).OldPosition)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<GeneratorPosition>(result__)
        }
    }
    pub fn ItemCount(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows_core::Interface::vtable(this).ItemCount)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    pub fn ItemUICount(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows_core::Interface::vtable(this).ItemUICount)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
}
impl ::core::clone::Clone for ItemsChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ItemsChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ItemsChangedEventArgs {}
impl ::core::fmt::Debug for ItemsChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ItemsChangedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ItemsChangedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Controls.Primitives.ItemsChangedEventArgs;{e8b45568-7d10-421e-be29-81839a91de20})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for ItemsChangedEventArgs {
    type Vtable = IItemsChangedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IItemsChangedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for ItemsChangedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.ItemsChangedEventArgs";
}
impl ::core::convert::From<ItemsChangedEventArgs> for ::windows_core::IUnknown {
    fn from(value: ItemsChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ItemsChangedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &ItemsChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ItemsChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ItemsChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ItemsChangedEventArgs> for ::windows_core::IInspectable {
    fn from(value: ItemsChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ItemsChangedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &ItemsChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for ItemsChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a ItemsChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for ItemsChangedEventArgs {}
unsafe impl ::core::marker::Sync for ItemsChangedEventArgs {}
#[repr(transparent)]
pub struct ItemsChangedEventHandler(pub ::windows_core::IUnknown);
impl ItemsChangedEventHandler {
    pub fn new<F: FnMut(&::core::option::Option<::windows_core::IInspectable>, &::core::option::Option<ItemsChangedEventArgs>) -> ::windows_core::Result<()> + ::core::marker::Send + 'static>(invoke: F) -> Self {
        let com = ItemsChangedEventHandlerBox::<F> { vtable: &ItemsChangedEventHandlerBox::<F>::VTABLE, count: ::windows_core::RefCount::new(1), invoke };
        unsafe { ::core::mem::transmute(::windows_core::alloc::boxed::Box::new(com)) }
    }
    pub fn Invoke<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::IInspectable>, Param1: ::windows_core::IntoParam<'a, ItemsChangedEventArgs>>(&self, sender: Param0, e: Param1) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Invoke)(::windows_core::Interface::as_raw(this), sender.into_param().abi(), e.into_param().abi()).ok() }
    }
}
#[repr(C)]
struct ItemsChangedEventHandlerBox<F: FnMut(&::core::option::Option<::windows_core::IInspectable>, &::core::option::Option<ItemsChangedEventArgs>) -> ::windows_core::Result<()> + ::core::marker::Send + 'static> {
    vtable: *const ItemsChangedEventHandler_Vtbl,
    invoke: F,
    count: ::windows_core::RefCount,
}
impl<F: FnMut(&::core::option::Option<::windows_core::IInspectable>, &::core::option::Option<ItemsChangedEventArgs>) -> ::windows_core::Result<()> + ::core::marker::Send + 'static> ItemsChangedEventHandlerBox<F> {
    const VTABLE: ItemsChangedEventHandler_Vtbl = ItemsChangedEventHandler_Vtbl {
        base__: ::windows_core::IUnknownVtbl { QueryInterface: Self::QueryInterface, AddRef: Self::AddRef, Release: Self::Release },
        Invoke: Self::Invoke,
    };
    unsafe extern "system" fn QueryInterface(this: ::windows_core::RawPtr, iid: &::windows_core::GUID, interface: *mut *const ::core::ffi::c_void) -> ::windows_core::HRESULT {
        let this = this as *mut ::windows_core::RawPtr as *mut Self;
        *interface = if iid == &<ItemsChangedEventHandler as ::windows_core::Interface>::IID || iid == &<::windows_core::IUnknown as ::windows_core::Interface>::IID || iid == &<::windows_core::IAgileObject as ::windows_core::Interface>::IID { &mut (*this).vtable as *mut _ as _ } else { ::core::ptr::null_mut() };
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
    unsafe extern "system" fn Invoke(this: *mut ::core::ffi::c_void, sender: *mut ::core::ffi::c_void, e: ::windows_core::RawPtr) -> ::windows_core::HRESULT {
        let this = this as *mut ::windows_core::RawPtr as *mut Self;
        ((*this).invoke)(::core::mem::transmute(&sender), ::core::mem::transmute(&e)).into()
    }
}
impl ::core::clone::Clone for ItemsChangedEventHandler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ItemsChangedEventHandler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ItemsChangedEventHandler {}
impl ::core::fmt::Debug for ItemsChangedEventHandler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ItemsChangedEventHandler").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for ItemsChangedEventHandler {
    type Vtable = ItemsChangedEventHandler_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x178257be_a304_482f_8bf0_b9d2e39612a3);
}
unsafe impl ::windows_core::RuntimeType for ItemsChangedEventHandler {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"{178257be-a304-482f-8bf0-b9d2e39612a3}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ItemsChangedEventHandler_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub Invoke: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sender: *mut ::core::ffi::c_void, e: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct JumpListItemBackgroundConverter(::windows_core::IUnknown);
impl JumpListItemBackgroundConverter {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<JumpListItemBackgroundConverter, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn Enabled(&self) -> ::windows_core::Result<super::super::Media::Brush> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Enabled)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Media::Brush>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetEnabled<'a, Param0: ::windows_core::IntoParam<'a, super::super::Media::Brush>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetEnabled)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn Disabled(&self) -> ::windows_core::Result<super::super::Media::Brush> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Disabled)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Media::Brush>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetDisabled<'a, Param0: ::windows_core::IntoParam<'a, super::super::Media::Brush>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetDisabled)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn EnabledProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::IJumpListItemBackgroundConverterStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).EnabledProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn DisabledProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::IJumpListItemBackgroundConverterStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).DisabledProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[cfg(all(feature = "UI_Xaml_Data", feature = "UI_Xaml_Interop"))]
    pub fn Convert<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::IInspectable>, Param1: ::windows_core::IntoParam<'a, super::super::Interop::TypeName>, Param2: ::windows_core::IntoParam<'a, ::windows_core::IInspectable>, Param3: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0, targettype: Param1, parameter: Param2, language: Param3) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = &::windows_core::Interface::cast::<super::super::Data::IValueConverter>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows_core::Interface::vtable(this).Convert)(::windows_core::Interface::as_raw(this), value.into_param().abi(), targettype.into_param().abi(), parameter.into_param().abi(), language.into_param().abi(), result__.as_mut_ptr()).from_abi::<::windows_core::IInspectable>(result__)
        }
    }
    #[cfg(all(feature = "UI_Xaml_Data", feature = "UI_Xaml_Interop"))]
    pub fn ConvertBack<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::IInspectable>, Param1: ::windows_core::IntoParam<'a, super::super::Interop::TypeName>, Param2: ::windows_core::IntoParam<'a, ::windows_core::IInspectable>, Param3: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0, targettype: Param1, parameter: Param2, language: Param3) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = &::windows_core::Interface::cast::<super::super::Data::IValueConverter>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows_core::Interface::vtable(this).ConvertBack)(::windows_core::Interface::as_raw(this), value.into_param().abi(), targettype.into_param().abi(), parameter.into_param().abi(), language.into_param().abi(), result__.as_mut_ptr()).from_abi::<::windows_core::IInspectable>(result__)
        }
    }
    pub fn IJumpListItemBackgroundConverterStatics<R, F: FnOnce(&IJumpListItemBackgroundConverterStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<JumpListItemBackgroundConverter, IJumpListItemBackgroundConverterStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for JumpListItemBackgroundConverter {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for JumpListItemBackgroundConverter {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for JumpListItemBackgroundConverter {}
impl ::core::fmt::Debug for JumpListItemBackgroundConverter {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("JumpListItemBackgroundConverter").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for JumpListItemBackgroundConverter {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Controls.Primitives.JumpListItemBackgroundConverter;{81177858-d224-410c-b16c-c5b6bb6188b2})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for JumpListItemBackgroundConverter {
    type Vtable = IJumpListItemBackgroundConverter_Vtbl;
    const IID: ::windows_core::GUID = <IJumpListItemBackgroundConverter as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for JumpListItemBackgroundConverter {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.JumpListItemBackgroundConverter";
}
impl ::core::convert::From<JumpListItemBackgroundConverter> for ::windows_core::IUnknown {
    fn from(value: JumpListItemBackgroundConverter) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&JumpListItemBackgroundConverter> for ::windows_core::IUnknown {
    fn from(value: &JumpListItemBackgroundConverter) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for JumpListItemBackgroundConverter {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a JumpListItemBackgroundConverter {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<JumpListItemBackgroundConverter> for ::windows_core::IInspectable {
    fn from(value: JumpListItemBackgroundConverter) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&JumpListItemBackgroundConverter> for ::windows_core::IInspectable {
    fn from(value: &JumpListItemBackgroundConverter) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for JumpListItemBackgroundConverter {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a JumpListItemBackgroundConverter {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "UI_Xaml_Data")]
impl ::core::convert::TryFrom<JumpListItemBackgroundConverter> for super::super::Data::IValueConverter {
    type Error = ::windows_core::Error;
    fn try_from(value: JumpListItemBackgroundConverter) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "UI_Xaml_Data")]
impl ::core::convert::TryFrom<&JumpListItemBackgroundConverter> for super::super::Data::IValueConverter {
    type Error = ::windows_core::Error;
    fn try_from(value: &JumpListItemBackgroundConverter) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "UI_Xaml_Data")]
impl<'a> ::windows_core::IntoParam<'a, super::super::Data::IValueConverter> for JumpListItemBackgroundConverter {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::Data::IValueConverter> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "UI_Xaml_Data")]
impl<'a> ::windows_core::IntoParam<'a, super::super::Data::IValueConverter> for &JumpListItemBackgroundConverter {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::Data::IValueConverter> {
        ::core::convert::TryInto::<super::super::Data::IValueConverter>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::From<JumpListItemBackgroundConverter> for super::super::DependencyObject {
    fn from(value: JumpListItemBackgroundConverter) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&JumpListItemBackgroundConverter> for super::super::DependencyObject {
    fn from(value: &JumpListItemBackgroundConverter) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::DependencyObject> for JumpListItemBackgroundConverter {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::DependencyObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::DependencyObject> for &JumpListItemBackgroundConverter {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::DependencyObject> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for JumpListItemBackgroundConverter {}
unsafe impl ::core::marker::Sync for JumpListItemBackgroundConverter {}
#[repr(transparent)]
pub struct JumpListItemForegroundConverter(::windows_core::IUnknown);
impl JumpListItemForegroundConverter {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<JumpListItemForegroundConverter, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn Enabled(&self) -> ::windows_core::Result<super::super::Media::Brush> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Enabled)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Media::Brush>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetEnabled<'a, Param0: ::windows_core::IntoParam<'a, super::super::Media::Brush>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetEnabled)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn Disabled(&self) -> ::windows_core::Result<super::super::Media::Brush> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Disabled)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Media::Brush>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetDisabled<'a, Param0: ::windows_core::IntoParam<'a, super::super::Media::Brush>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetDisabled)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn EnabledProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::IJumpListItemForegroundConverterStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).EnabledProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn DisabledProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::IJumpListItemForegroundConverterStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).DisabledProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[cfg(all(feature = "UI_Xaml_Data", feature = "UI_Xaml_Interop"))]
    pub fn Convert<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::IInspectable>, Param1: ::windows_core::IntoParam<'a, super::super::Interop::TypeName>, Param2: ::windows_core::IntoParam<'a, ::windows_core::IInspectable>, Param3: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0, targettype: Param1, parameter: Param2, language: Param3) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = &::windows_core::Interface::cast::<super::super::Data::IValueConverter>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows_core::Interface::vtable(this).Convert)(::windows_core::Interface::as_raw(this), value.into_param().abi(), targettype.into_param().abi(), parameter.into_param().abi(), language.into_param().abi(), result__.as_mut_ptr()).from_abi::<::windows_core::IInspectable>(result__)
        }
    }
    #[cfg(all(feature = "UI_Xaml_Data", feature = "UI_Xaml_Interop"))]
    pub fn ConvertBack<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::IInspectable>, Param1: ::windows_core::IntoParam<'a, super::super::Interop::TypeName>, Param2: ::windows_core::IntoParam<'a, ::windows_core::IInspectable>, Param3: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0, targettype: Param1, parameter: Param2, language: Param3) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = &::windows_core::Interface::cast::<super::super::Data::IValueConverter>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows_core::Interface::vtable(this).ConvertBack)(::windows_core::Interface::as_raw(this), value.into_param().abi(), targettype.into_param().abi(), parameter.into_param().abi(), language.into_param().abi(), result__.as_mut_ptr()).from_abi::<::windows_core::IInspectable>(result__)
        }
    }
    pub fn IJumpListItemForegroundConverterStatics<R, F: FnOnce(&IJumpListItemForegroundConverterStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<JumpListItemForegroundConverter, IJumpListItemForegroundConverterStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for JumpListItemForegroundConverter {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for JumpListItemForegroundConverter {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for JumpListItemForegroundConverter {}
impl ::core::fmt::Debug for JumpListItemForegroundConverter {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("JumpListItemForegroundConverter").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for JumpListItemForegroundConverter {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Controls.Primitives.JumpListItemForegroundConverter;{1590ed38-c504-4796-a63a-5bfc9eefaae8})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for JumpListItemForegroundConverter {
    type Vtable = IJumpListItemForegroundConverter_Vtbl;
    const IID: ::windows_core::GUID = <IJumpListItemForegroundConverter as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for JumpListItemForegroundConverter {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.JumpListItemForegroundConverter";
}
impl ::core::convert::From<JumpListItemForegroundConverter> for ::windows_core::IUnknown {
    fn from(value: JumpListItemForegroundConverter) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&JumpListItemForegroundConverter> for ::windows_core::IUnknown {
    fn from(value: &JumpListItemForegroundConverter) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for JumpListItemForegroundConverter {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a JumpListItemForegroundConverter {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<JumpListItemForegroundConverter> for ::windows_core::IInspectable {
    fn from(value: JumpListItemForegroundConverter) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&JumpListItemForegroundConverter> for ::windows_core::IInspectable {
    fn from(value: &JumpListItemForegroundConverter) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for JumpListItemForegroundConverter {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a JumpListItemForegroundConverter {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "UI_Xaml_Data")]
impl ::core::convert::TryFrom<JumpListItemForegroundConverter> for super::super::Data::IValueConverter {
    type Error = ::windows_core::Error;
    fn try_from(value: JumpListItemForegroundConverter) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "UI_Xaml_Data")]
impl ::core::convert::TryFrom<&JumpListItemForegroundConverter> for super::super::Data::IValueConverter {
    type Error = ::windows_core::Error;
    fn try_from(value: &JumpListItemForegroundConverter) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "UI_Xaml_Data")]
impl<'a> ::windows_core::IntoParam<'a, super::super::Data::IValueConverter> for JumpListItemForegroundConverter {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::Data::IValueConverter> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "UI_Xaml_Data")]
impl<'a> ::windows_core::IntoParam<'a, super::super::Data::IValueConverter> for &JumpListItemForegroundConverter {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::Data::IValueConverter> {
        ::core::convert::TryInto::<super::super::Data::IValueConverter>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::From<JumpListItemForegroundConverter> for super::super::DependencyObject {
    fn from(value: JumpListItemForegroundConverter) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&JumpListItemForegroundConverter> for super::super::DependencyObject {
    fn from(value: &JumpListItemForegroundConverter) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::DependencyObject> for JumpListItemForegroundConverter {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::DependencyObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::DependencyObject> for &JumpListItemForegroundConverter {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::DependencyObject> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for JumpListItemForegroundConverter {}
unsafe impl ::core::marker::Sync for JumpListItemForegroundConverter {}
#[repr(transparent)]
pub struct LayoutInformation(::windows_core::IUnknown);
impl LayoutInformation {
    pub fn GetLayoutExceptionElement<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::IInspectable>>(dispatcher: Param0) -> ::windows_core::Result<super::super::UIElement> {
        Self::ILayoutInformationStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetLayoutExceptionElement)(::windows_core::Interface::as_raw(this), dispatcher.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::UIElement>(result__)
        })
    }
    pub fn GetLayoutSlot<'a, Param0: ::windows_core::IntoParam<'a, super::super::FrameworkElement>>(element: Param0) -> ::windows_core::Result<::winrt_foundation::Rect> {
        Self::ILayoutInformationStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::Rect>::zeroed();
            (::windows_core::Interface::vtable(this).GetLayoutSlot)(::windows_core::Interface::as_raw(this), element.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Rect>(result__)
        })
    }
    pub fn GetAvailableSize<'a, Param0: ::windows_core::IntoParam<'a, super::super::UIElement>>(element: Param0) -> ::windows_core::Result<::winrt_foundation::Size> {
        Self::ILayoutInformationStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::Size>::zeroed();
            (::windows_core::Interface::vtable(this).GetAvailableSize)(::windows_core::Interface::as_raw(this), element.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Size>(result__)
        })
    }
    pub fn ILayoutInformationStatics<R, F: FnOnce(&ILayoutInformationStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<LayoutInformation, ILayoutInformationStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn ILayoutInformationStatics2<R, F: FnOnce(&ILayoutInformationStatics2) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<LayoutInformation, ILayoutInformationStatics2> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for LayoutInformation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for LayoutInformation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for LayoutInformation {}
impl ::core::fmt::Debug for LayoutInformation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LayoutInformation").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for LayoutInformation {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Controls.Primitives.LayoutInformation;{b5384c9b-c8cf-41b3-bf16-18c8420e72c9})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for LayoutInformation {
    type Vtable = ILayoutInformation_Vtbl;
    const IID: ::windows_core::GUID = <ILayoutInformation as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for LayoutInformation {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.LayoutInformation";
}
impl ::core::convert::From<LayoutInformation> for ::windows_core::IUnknown {
    fn from(value: LayoutInformation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LayoutInformation> for ::windows_core::IUnknown {
    fn from(value: &LayoutInformation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for LayoutInformation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a LayoutInformation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<LayoutInformation> for ::windows_core::IInspectable {
    fn from(value: LayoutInformation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LayoutInformation> for ::windows_core::IInspectable {
    fn from(value: &LayoutInformation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for LayoutInformation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a LayoutInformation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for LayoutInformation {}
unsafe impl ::core::marker::Sync for LayoutInformation {}
#[repr(transparent)]
pub struct ListViewItemPresenter(::windows_core::IUnknown);
impl ListViewItemPresenter {
    pub fn SelectionCheckMarkVisualEnabled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).SelectionCheckMarkVisualEnabled)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetSelectionCheckMarkVisualEnabled(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetSelectionCheckMarkVisualEnabled)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn CheckHintBrush(&self) -> ::windows_core::Result<super::super::Media::Brush> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CheckHintBrush)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Media::Brush>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetCheckHintBrush<'a, Param0: ::windows_core::IntoParam<'a, super::super::Media::Brush>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetCheckHintBrush)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn CheckSelectingBrush(&self) -> ::windows_core::Result<super::super::Media::Brush> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CheckSelectingBrush)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Media::Brush>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetCheckSelectingBrush<'a, Param0: ::windows_core::IntoParam<'a, super::super::Media::Brush>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetCheckSelectingBrush)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn CheckBrush(&self) -> ::windows_core::Result<super::super::Media::Brush> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CheckBrush)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Media::Brush>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetCheckBrush<'a, Param0: ::windows_core::IntoParam<'a, super::super::Media::Brush>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetCheckBrush)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn DragBackground(&self) -> ::windows_core::Result<super::super::Media::Brush> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).DragBackground)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Media::Brush>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetDragBackground<'a, Param0: ::windows_core::IntoParam<'a, super::super::Media::Brush>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetDragBackground)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn DragForeground(&self) -> ::windows_core::Result<super::super::Media::Brush> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).DragForeground)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Media::Brush>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetDragForeground<'a, Param0: ::windows_core::IntoParam<'a, super::super::Media::Brush>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetDragForeground)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn FocusBorderBrush(&self) -> ::windows_core::Result<super::super::Media::Brush> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FocusBorderBrush)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Media::Brush>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetFocusBorderBrush<'a, Param0: ::windows_core::IntoParam<'a, super::super::Media::Brush>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetFocusBorderBrush)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn PlaceholderBackground(&self) -> ::windows_core::Result<super::super::Media::Brush> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).PlaceholderBackground)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Media::Brush>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetPlaceholderBackground<'a, Param0: ::windows_core::IntoParam<'a, super::super::Media::Brush>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetPlaceholderBackground)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn PointerOverBackground(&self) -> ::windows_core::Result<super::super::Media::Brush> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).PointerOverBackground)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Media::Brush>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetPointerOverBackground<'a, Param0: ::windows_core::IntoParam<'a, super::super::Media::Brush>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetPointerOverBackground)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SelectedBackground(&self) -> ::windows_core::Result<super::super::Media::Brush> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SelectedBackground)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Media::Brush>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetSelectedBackground<'a, Param0: ::windows_core::IntoParam<'a, super::super::Media::Brush>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetSelectedBackground)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SelectedForeground(&self) -> ::windows_core::Result<super::super::Media::Brush> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SelectedForeground)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Media::Brush>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetSelectedForeground<'a, Param0: ::windows_core::IntoParam<'a, super::super::Media::Brush>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetSelectedForeground)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SelectedPointerOverBackground(&self) -> ::windows_core::Result<super::super::Media::Brush> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SelectedPointerOverBackground)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Media::Brush>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetSelectedPointerOverBackground<'a, Param0: ::windows_core::IntoParam<'a, super::super::Media::Brush>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetSelectedPointerOverBackground)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SelectedPointerOverBorderBrush(&self) -> ::windows_core::Result<super::super::Media::Brush> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SelectedPointerOverBorderBrush)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Media::Brush>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetSelectedPointerOverBorderBrush<'a, Param0: ::windows_core::IntoParam<'a, super::super::Media::Brush>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetSelectedPointerOverBorderBrush)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn SelectedBorderThickness(&self) -> ::windows_core::Result<super::super::Thickness> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::Thickness>::zeroed();
            (::windows_core::Interface::vtable(this).SelectedBorderThickness)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Thickness>(result__)
        }
    }
    pub fn SetSelectedBorderThickness<'a, Param0: ::windows_core::IntoParam<'a, super::super::Thickness>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetSelectedBorderThickness)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn DisabledOpacity(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).DisabledOpacity)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn SetDisabledOpacity(&self, value: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetDisabledOpacity)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn DragOpacity(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).DragOpacity)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn SetDragOpacity(&self, value: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetDragOpacity)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ReorderHintOffset(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).ReorderHintOffset)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn SetReorderHintOffset(&self, value: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetReorderHintOffset)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "deprecated")]
    pub fn ListViewItemPresenterHorizontalContentAlignment(&self) -> ::windows_core::Result<super::super::HorizontalAlignment> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::HorizontalAlignment>::zeroed();
            (::windows_core::Interface::vtable(this).ListViewItemPresenterHorizontalContentAlignment)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::HorizontalAlignment>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn SetListViewItemPresenterHorizontalContentAlignment(&self, value: super::super::HorizontalAlignment) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetListViewItemPresenterHorizontalContentAlignment)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "deprecated")]
    pub fn ListViewItemPresenterVerticalContentAlignment(&self) -> ::windows_core::Result<super::super::VerticalAlignment> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::VerticalAlignment>::zeroed();
            (::windows_core::Interface::vtable(this).ListViewItemPresenterVerticalContentAlignment)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::VerticalAlignment>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn SetListViewItemPresenterVerticalContentAlignment(&self, value: super::super::VerticalAlignment) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetListViewItemPresenterVerticalContentAlignment)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "deprecated")]
    pub fn ListViewItemPresenterPadding(&self) -> ::windows_core::Result<super::super::Thickness> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::Thickness>::zeroed();
            (::windows_core::Interface::vtable(this).ListViewItemPresenterPadding)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Thickness>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn SetListViewItemPresenterPadding<'a, Param0: ::windows_core::IntoParam<'a, super::super::Thickness>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetListViewItemPresenterPadding)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn PointerOverBackgroundMargin(&self) -> ::windows_core::Result<super::super::Thickness> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::Thickness>::zeroed();
            (::windows_core::Interface::vtable(this).PointerOverBackgroundMargin)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Thickness>(result__)
        }
    }
    pub fn SetPointerOverBackgroundMargin<'a, Param0: ::windows_core::IntoParam<'a, super::super::Thickness>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetPointerOverBackgroundMargin)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn ContentMargin(&self) -> ::windows_core::Result<super::super::Thickness> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::Thickness>::zeroed();
            (::windows_core::Interface::vtable(this).ContentMargin)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Thickness>(result__)
        }
    }
    pub fn SetContentMargin<'a, Param0: ::windows_core::IntoParam<'a, super::super::Thickness>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetContentMargin)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SelectedPressedBackground(&self) -> ::windows_core::Result<super::super::Media::Brush> {
        let this = &::windows_core::Interface::cast::<IListViewItemPresenter2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SelectedPressedBackground)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Media::Brush>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetSelectedPressedBackground<'a, Param0: ::windows_core::IntoParam<'a, super::super::Media::Brush>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IListViewItemPresenter2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetSelectedPressedBackground)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn PressedBackground(&self) -> ::windows_core::Result<super::super::Media::Brush> {
        let this = &::windows_core::Interface::cast::<IListViewItemPresenter2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).PressedBackground)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Media::Brush>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetPressedBackground<'a, Param0: ::windows_core::IntoParam<'a, super::super::Media::Brush>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IListViewItemPresenter2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetPressedBackground)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn CheckBoxBrush(&self) -> ::windows_core::Result<super::super::Media::Brush> {
        let this = &::windows_core::Interface::cast::<IListViewItemPresenter2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CheckBoxBrush)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Media::Brush>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetCheckBoxBrush<'a, Param0: ::windows_core::IntoParam<'a, super::super::Media::Brush>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IListViewItemPresenter2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetCheckBoxBrush)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn FocusSecondaryBorderBrush(&self) -> ::windows_core::Result<super::super::Media::Brush> {
        let this = &::windows_core::Interface::cast::<IListViewItemPresenter2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FocusSecondaryBorderBrush)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Media::Brush>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetFocusSecondaryBorderBrush<'a, Param0: ::windows_core::IntoParam<'a, super::super::Media::Brush>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IListViewItemPresenter2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetFocusSecondaryBorderBrush)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn CheckMode(&self) -> ::windows_core::Result<ListViewItemPresenterCheckMode> {
        let this = &::windows_core::Interface::cast::<IListViewItemPresenter2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ListViewItemPresenterCheckMode>::zeroed();
            (::windows_core::Interface::vtable(this).CheckMode)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ListViewItemPresenterCheckMode>(result__)
        }
    }
    pub fn SetCheckMode(&self, value: ListViewItemPresenterCheckMode) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IListViewItemPresenter2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetCheckMode)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn PointerOverForeground(&self) -> ::windows_core::Result<super::super::Media::Brush> {
        let this = &::windows_core::Interface::cast::<IListViewItemPresenter2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).PointerOverForeground)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Media::Brush>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetPointerOverForeground<'a, Param0: ::windows_core::IntoParam<'a, super::super::Media::Brush>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IListViewItemPresenter2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetPointerOverForeground)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn RevealBackground(&self) -> ::windows_core::Result<super::super::Media::Brush> {
        let this = &::windows_core::Interface::cast::<IListViewItemPresenter3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RevealBackground)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Media::Brush>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetRevealBackground<'a, Param0: ::windows_core::IntoParam<'a, super::super::Media::Brush>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IListViewItemPresenter3>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetRevealBackground)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn RevealBorderBrush(&self) -> ::windows_core::Result<super::super::Media::Brush> {
        let this = &::windows_core::Interface::cast::<IListViewItemPresenter3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RevealBorderBrush)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Media::Brush>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetRevealBorderBrush<'a, Param0: ::windows_core::IntoParam<'a, super::super::Media::Brush>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IListViewItemPresenter3>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetRevealBorderBrush)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn RevealBorderThickness(&self) -> ::windows_core::Result<super::super::Thickness> {
        let this = &::windows_core::Interface::cast::<IListViewItemPresenter3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::Thickness>::zeroed();
            (::windows_core::Interface::vtable(this).RevealBorderThickness)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Thickness>(result__)
        }
    }
    pub fn SetRevealBorderThickness<'a, Param0: ::windows_core::IntoParam<'a, super::super::Thickness>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IListViewItemPresenter3>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetRevealBorderThickness)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn RevealBackgroundShowsAboveContent(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IListViewItemPresenter3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).RevealBackgroundShowsAboveContent)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetRevealBackgroundShowsAboveContent(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IListViewItemPresenter3>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetRevealBackgroundShowsAboveContent)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SelectedDisabledBackground(&self) -> ::windows_core::Result<super::super::Media::Brush> {
        let this = &::windows_core::Interface::cast::<IListViewItemPresenter4>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SelectedDisabledBackground)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Media::Brush>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetSelectedDisabledBackground<'a, Param0: ::windows_core::IntoParam<'a, super::super::Media::Brush>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IListViewItemPresenter4>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetSelectedDisabledBackground)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn CheckPressedBrush(&self) -> ::windows_core::Result<super::super::Media::Brush> {
        let this = &::windows_core::Interface::cast::<IListViewItemPresenter4>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CheckPressedBrush)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Media::Brush>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetCheckPressedBrush<'a, Param0: ::windows_core::IntoParam<'a, super::super::Media::Brush>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IListViewItemPresenter4>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetCheckPressedBrush)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn CheckDisabledBrush(&self) -> ::windows_core::Result<super::super::Media::Brush> {
        let this = &::windows_core::Interface::cast::<IListViewItemPresenter4>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CheckDisabledBrush)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Media::Brush>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetCheckDisabledBrush<'a, Param0: ::windows_core::IntoParam<'a, super::super::Media::Brush>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IListViewItemPresenter4>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetCheckDisabledBrush)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn CheckBoxPointerOverBrush(&self) -> ::windows_core::Result<super::super::Media::Brush> {
        let this = &::windows_core::Interface::cast::<IListViewItemPresenter4>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CheckBoxPointerOverBrush)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Media::Brush>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetCheckBoxPointerOverBrush<'a, Param0: ::windows_core::IntoParam<'a, super::super::Media::Brush>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IListViewItemPresenter4>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetCheckBoxPointerOverBrush)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn CheckBoxPressedBrush(&self) -> ::windows_core::Result<super::super::Media::Brush> {
        let this = &::windows_core::Interface::cast::<IListViewItemPresenter4>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CheckBoxPressedBrush)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Media::Brush>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetCheckBoxPressedBrush<'a, Param0: ::windows_core::IntoParam<'a, super::super::Media::Brush>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IListViewItemPresenter4>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetCheckBoxPressedBrush)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn CheckBoxDisabledBrush(&self) -> ::windows_core::Result<super::super::Media::Brush> {
        let this = &::windows_core::Interface::cast::<IListViewItemPresenter4>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CheckBoxDisabledBrush)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Media::Brush>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetCheckBoxDisabledBrush<'a, Param0: ::windows_core::IntoParam<'a, super::super::Media::Brush>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IListViewItemPresenter4>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetCheckBoxDisabledBrush)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn CheckBoxSelectedBrush(&self) -> ::windows_core::Result<super::super::Media::Brush> {
        let this = &::windows_core::Interface::cast::<IListViewItemPresenter4>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CheckBoxSelectedBrush)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Media::Brush>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetCheckBoxSelectedBrush<'a, Param0: ::windows_core::IntoParam<'a, super::super::Media::Brush>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IListViewItemPresenter4>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetCheckBoxSelectedBrush)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn CheckBoxSelectedPointerOverBrush(&self) -> ::windows_core::Result<super::super::Media::Brush> {
        let this = &::windows_core::Interface::cast::<IListViewItemPresenter4>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CheckBoxSelectedPointerOverBrush)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Media::Brush>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetCheckBoxSelectedPointerOverBrush<'a, Param0: ::windows_core::IntoParam<'a, super::super::Media::Brush>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IListViewItemPresenter4>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetCheckBoxSelectedPointerOverBrush)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn CheckBoxSelectedPressedBrush(&self) -> ::windows_core::Result<super::super::Media::Brush> {
        let this = &::windows_core::Interface::cast::<IListViewItemPresenter4>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CheckBoxSelectedPressedBrush)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Media::Brush>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetCheckBoxSelectedPressedBrush<'a, Param0: ::windows_core::IntoParam<'a, super::super::Media::Brush>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IListViewItemPresenter4>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetCheckBoxSelectedPressedBrush)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn CheckBoxSelectedDisabledBrush(&self) -> ::windows_core::Result<super::super::Media::Brush> {
        let this = &::windows_core::Interface::cast::<IListViewItemPresenter4>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CheckBoxSelectedDisabledBrush)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Media::Brush>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetCheckBoxSelectedDisabledBrush<'a, Param0: ::windows_core::IntoParam<'a, super::super::Media::Brush>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IListViewItemPresenter4>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetCheckBoxSelectedDisabledBrush)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn CheckBoxBorderBrush(&self) -> ::windows_core::Result<super::super::Media::Brush> {
        let this = &::windows_core::Interface::cast::<IListViewItemPresenter4>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CheckBoxBorderBrush)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Media::Brush>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetCheckBoxBorderBrush<'a, Param0: ::windows_core::IntoParam<'a, super::super::Media::Brush>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IListViewItemPresenter4>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetCheckBoxBorderBrush)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn CheckBoxPointerOverBorderBrush(&self) -> ::windows_core::Result<super::super::Media::Brush> {
        let this = &::windows_core::Interface::cast::<IListViewItemPresenter4>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CheckBoxPointerOverBorderBrush)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Media::Brush>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetCheckBoxPointerOverBorderBrush<'a, Param0: ::windows_core::IntoParam<'a, super::super::Media::Brush>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IListViewItemPresenter4>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetCheckBoxPointerOverBorderBrush)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn CheckBoxPressedBorderBrush(&self) -> ::windows_core::Result<super::super::Media::Brush> {
        let this = &::windows_core::Interface::cast::<IListViewItemPresenter4>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CheckBoxPressedBorderBrush)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Media::Brush>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetCheckBoxPressedBorderBrush<'a, Param0: ::windows_core::IntoParam<'a, super::super::Media::Brush>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IListViewItemPresenter4>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetCheckBoxPressedBorderBrush)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn CheckBoxDisabledBorderBrush(&self) -> ::windows_core::Result<super::super::Media::Brush> {
        let this = &::windows_core::Interface::cast::<IListViewItemPresenter4>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CheckBoxDisabledBorderBrush)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Media::Brush>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetCheckBoxDisabledBorderBrush<'a, Param0: ::windows_core::IntoParam<'a, super::super::Media::Brush>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IListViewItemPresenter4>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetCheckBoxDisabledBorderBrush)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn CheckBoxCornerRadius(&self) -> ::windows_core::Result<super::super::CornerRadius> {
        let this = &::windows_core::Interface::cast::<IListViewItemPresenter4>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::CornerRadius>::zeroed();
            (::windows_core::Interface::vtable(this).CheckBoxCornerRadius)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::CornerRadius>(result__)
        }
    }
    pub fn SetCheckBoxCornerRadius<'a, Param0: ::windows_core::IntoParam<'a, super::super::CornerRadius>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IListViewItemPresenter4>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetCheckBoxCornerRadius)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn SelectionIndicatorCornerRadius(&self) -> ::windows_core::Result<super::super::CornerRadius> {
        let this = &::windows_core::Interface::cast::<IListViewItemPresenter4>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::CornerRadius>::zeroed();
            (::windows_core::Interface::vtable(this).SelectionIndicatorCornerRadius)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::CornerRadius>(result__)
        }
    }
    pub fn SetSelectionIndicatorCornerRadius<'a, Param0: ::windows_core::IntoParam<'a, super::super::CornerRadius>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IListViewItemPresenter4>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetSelectionIndicatorCornerRadius)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn SelectionIndicatorVisualEnabled(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IListViewItemPresenter4>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).SelectionIndicatorVisualEnabled)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetSelectionIndicatorVisualEnabled(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IListViewItemPresenter4>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetSelectionIndicatorVisualEnabled)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn SelectionIndicatorMode(&self) -> ::windows_core::Result<ListViewItemPresenterSelectionIndicatorMode> {
        let this = &::windows_core::Interface::cast::<IListViewItemPresenter4>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ListViewItemPresenterSelectionIndicatorMode>::zeroed();
            (::windows_core::Interface::vtable(this).SelectionIndicatorMode)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ListViewItemPresenterSelectionIndicatorMode>(result__)
        }
    }
    pub fn SetSelectionIndicatorMode(&self, value: ListViewItemPresenterSelectionIndicatorMode) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IListViewItemPresenter4>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetSelectionIndicatorMode)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SelectionIndicatorBrush(&self) -> ::windows_core::Result<super::super::Media::Brush> {
        let this = &::windows_core::Interface::cast::<IListViewItemPresenter4>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SelectionIndicatorBrush)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Media::Brush>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetSelectionIndicatorBrush<'a, Param0: ::windows_core::IntoParam<'a, super::super::Media::Brush>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IListViewItemPresenter4>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetSelectionIndicatorBrush)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SelectionIndicatorPointerOverBrush(&self) -> ::windows_core::Result<super::super::Media::Brush> {
        let this = &::windows_core::Interface::cast::<IListViewItemPresenter4>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SelectionIndicatorPointerOverBrush)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Media::Brush>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetSelectionIndicatorPointerOverBrush<'a, Param0: ::windows_core::IntoParam<'a, super::super::Media::Brush>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IListViewItemPresenter4>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetSelectionIndicatorPointerOverBrush)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SelectionIndicatorPressedBrush(&self) -> ::windows_core::Result<super::super::Media::Brush> {
        let this = &::windows_core::Interface::cast::<IListViewItemPresenter4>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SelectionIndicatorPressedBrush)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Media::Brush>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetSelectionIndicatorPressedBrush<'a, Param0: ::windows_core::IntoParam<'a, super::super::Media::Brush>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IListViewItemPresenter4>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetSelectionIndicatorPressedBrush)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SelectionIndicatorDisabledBrush(&self) -> ::windows_core::Result<super::super::Media::Brush> {
        let this = &::windows_core::Interface::cast::<IListViewItemPresenter4>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SelectionIndicatorDisabledBrush)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Media::Brush>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetSelectionIndicatorDisabledBrush<'a, Param0: ::windows_core::IntoParam<'a, super::super::Media::Brush>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IListViewItemPresenter4>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetSelectionIndicatorDisabledBrush)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SelectedBorderBrush(&self) -> ::windows_core::Result<super::super::Media::Brush> {
        let this = &::windows_core::Interface::cast::<IListViewItemPresenter4>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SelectedBorderBrush)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Media::Brush>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetSelectedBorderBrush<'a, Param0: ::windows_core::IntoParam<'a, super::super::Media::Brush>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IListViewItemPresenter4>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetSelectedBorderBrush)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SelectedPressedBorderBrush(&self) -> ::windows_core::Result<super::super::Media::Brush> {
        let this = &::windows_core::Interface::cast::<IListViewItemPresenter4>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SelectedPressedBorderBrush)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Media::Brush>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetSelectedPressedBorderBrush<'a, Param0: ::windows_core::IntoParam<'a, super::super::Media::Brush>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IListViewItemPresenter4>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetSelectedPressedBorderBrush)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SelectedDisabledBorderBrush(&self) -> ::windows_core::Result<super::super::Media::Brush> {
        let this = &::windows_core::Interface::cast::<IListViewItemPresenter4>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SelectedDisabledBorderBrush)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Media::Brush>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetSelectedDisabledBorderBrush<'a, Param0: ::windows_core::IntoParam<'a, super::super::Media::Brush>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IListViewItemPresenter4>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetSelectedDisabledBorderBrush)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SelectedInnerBorderBrush(&self) -> ::windows_core::Result<super::super::Media::Brush> {
        let this = &::windows_core::Interface::cast::<IListViewItemPresenter4>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SelectedInnerBorderBrush)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Media::Brush>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetSelectedInnerBorderBrush<'a, Param0: ::windows_core::IntoParam<'a, super::super::Media::Brush>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IListViewItemPresenter4>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetSelectedInnerBorderBrush)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn PointerOverBorderBrush(&self) -> ::windows_core::Result<super::super::Media::Brush> {
        let this = &::windows_core::Interface::cast::<IListViewItemPresenter4>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).PointerOverBorderBrush)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Media::Brush>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetPointerOverBorderBrush<'a, Param0: ::windows_core::IntoParam<'a, super::super::Media::Brush>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IListViewItemPresenter4>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetPointerOverBorderBrush)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn new() -> ::windows_core::Result<ListViewItemPresenter> {
        Self::IListViewItemPresenterFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateInstance)(::windows_core::Interface::as_raw(this), ::core::ptr::null_mut(), &mut ::core::option::Option::<::windows_core::IInspectable>::None as *mut _ as _, result__.as_mut_ptr()).from_abi::<ListViewItemPresenter>(result__)
        })
    }
    pub fn compose<T: ::windows_core::Compose>(compose: T) -> ::windows_core::Result<ListViewItemPresenter> {
        Self::IListViewItemPresenterFactory(|this| unsafe {
            let (derived__, base__) = ::windows_core::Compose::compose(compose);
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateInstance)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(&derived__), base__ as *mut _ as _, result__.as_mut_ptr()).from_abi::<ListViewItemPresenter>(result__)
        })
    }
    pub fn SelectionCheckMarkVisualEnabledProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::IListViewItemPresenterStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SelectionCheckMarkVisualEnabledProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn CheckHintBrushProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::IListViewItemPresenterStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CheckHintBrushProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn CheckSelectingBrushProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::IListViewItemPresenterStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CheckSelectingBrushProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn CheckBrushProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::IListViewItemPresenterStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CheckBrushProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn DragBackgroundProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::IListViewItemPresenterStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).DragBackgroundProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn DragForegroundProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::IListViewItemPresenterStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).DragForegroundProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn FocusBorderBrushProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::IListViewItemPresenterStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FocusBorderBrushProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn PlaceholderBackgroundProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::IListViewItemPresenterStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).PlaceholderBackgroundProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn PointerOverBackgroundProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::IListViewItemPresenterStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).PointerOverBackgroundProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn SelectedBackgroundProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::IListViewItemPresenterStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SelectedBackgroundProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn SelectedForegroundProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::IListViewItemPresenterStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SelectedForegroundProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn SelectedPointerOverBackgroundProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::IListViewItemPresenterStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SelectedPointerOverBackgroundProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn SelectedPointerOverBorderBrushProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::IListViewItemPresenterStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SelectedPointerOverBorderBrushProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn SelectedBorderThicknessProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::IListViewItemPresenterStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SelectedBorderThicknessProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn DisabledOpacityProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::IListViewItemPresenterStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).DisabledOpacityProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn DragOpacityProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::IListViewItemPresenterStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).DragOpacityProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn ReorderHintOffsetProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::IListViewItemPresenterStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ReorderHintOffsetProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[cfg(feature = "deprecated")]
    pub fn ListViewItemPresenterHorizontalContentAlignmentProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::IListViewItemPresenterStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ListViewItemPresenterHorizontalContentAlignmentProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[cfg(feature = "deprecated")]
    pub fn ListViewItemPresenterVerticalContentAlignmentProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::IListViewItemPresenterStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ListViewItemPresenterVerticalContentAlignmentProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[cfg(feature = "deprecated")]
    pub fn ListViewItemPresenterPaddingProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::IListViewItemPresenterStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ListViewItemPresenterPaddingProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn PointerOverBackgroundMarginProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::IListViewItemPresenterStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).PointerOverBackgroundMarginProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn ContentMarginProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::IListViewItemPresenterStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ContentMarginProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn SelectedPressedBackgroundProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::IListViewItemPresenterStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SelectedPressedBackgroundProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn PressedBackgroundProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::IListViewItemPresenterStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).PressedBackgroundProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn CheckBoxBrushProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::IListViewItemPresenterStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CheckBoxBrushProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn FocusSecondaryBorderBrushProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::IListViewItemPresenterStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FocusSecondaryBorderBrushProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn CheckModeProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::IListViewItemPresenterStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CheckModeProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn PointerOverForegroundProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::IListViewItemPresenterStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).PointerOverForegroundProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn RevealBackgroundProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::IListViewItemPresenterStatics3(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RevealBackgroundProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn RevealBorderBrushProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::IListViewItemPresenterStatics3(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RevealBorderBrushProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn RevealBorderThicknessProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::IListViewItemPresenterStatics3(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RevealBorderThicknessProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn RevealBackgroundShowsAboveContentProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::IListViewItemPresenterStatics3(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RevealBackgroundShowsAboveContentProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn SelectedDisabledBackgroundProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::IListViewItemPresenterStatics4(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SelectedDisabledBackgroundProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn CheckPressedBrushProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::IListViewItemPresenterStatics4(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CheckPressedBrushProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn CheckDisabledBrushProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::IListViewItemPresenterStatics4(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CheckDisabledBrushProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn CheckBoxPointerOverBrushProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::IListViewItemPresenterStatics4(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CheckBoxPointerOverBrushProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn CheckBoxPressedBrushProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::IListViewItemPresenterStatics4(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CheckBoxPressedBrushProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn CheckBoxDisabledBrushProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::IListViewItemPresenterStatics4(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CheckBoxDisabledBrushProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn CheckBoxSelectedBrushProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::IListViewItemPresenterStatics4(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CheckBoxSelectedBrushProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn CheckBoxSelectedPointerOverBrushProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::IListViewItemPresenterStatics4(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CheckBoxSelectedPointerOverBrushProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn CheckBoxSelectedPressedBrushProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::IListViewItemPresenterStatics4(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CheckBoxSelectedPressedBrushProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn CheckBoxSelectedDisabledBrushProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::IListViewItemPresenterStatics4(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CheckBoxSelectedDisabledBrushProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn CheckBoxBorderBrushProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::IListViewItemPresenterStatics4(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CheckBoxBorderBrushProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn CheckBoxPointerOverBorderBrushProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::IListViewItemPresenterStatics4(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CheckBoxPointerOverBorderBrushProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn CheckBoxPressedBorderBrushProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::IListViewItemPresenterStatics4(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CheckBoxPressedBorderBrushProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn CheckBoxDisabledBorderBrushProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::IListViewItemPresenterStatics4(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CheckBoxDisabledBorderBrushProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn CheckBoxCornerRadiusProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::IListViewItemPresenterStatics4(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CheckBoxCornerRadiusProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn SelectionIndicatorCornerRadiusProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::IListViewItemPresenterStatics4(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SelectionIndicatorCornerRadiusProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn SelectionIndicatorVisualEnabledProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::IListViewItemPresenterStatics4(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SelectionIndicatorVisualEnabledProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn SelectionIndicatorModeProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::IListViewItemPresenterStatics4(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SelectionIndicatorModeProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn SelectionIndicatorBrushProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::IListViewItemPresenterStatics4(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SelectionIndicatorBrushProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn SelectionIndicatorPointerOverBrushProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::IListViewItemPresenterStatics4(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SelectionIndicatorPointerOverBrushProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn SelectionIndicatorPressedBrushProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::IListViewItemPresenterStatics4(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SelectionIndicatorPressedBrushProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn SelectionIndicatorDisabledBrushProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::IListViewItemPresenterStatics4(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SelectionIndicatorDisabledBrushProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn SelectedBorderBrushProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::IListViewItemPresenterStatics4(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SelectedBorderBrushProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn SelectedPressedBorderBrushProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::IListViewItemPresenterStatics4(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SelectedPressedBorderBrushProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn SelectedDisabledBorderBrushProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::IListViewItemPresenterStatics4(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SelectedDisabledBorderBrushProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn SelectedInnerBorderBrushProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::IListViewItemPresenterStatics4(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SelectedInnerBorderBrushProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn PointerOverBorderBrushProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::IListViewItemPresenterStatics4(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).PointerOverBorderBrushProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn IListViewItemPresenterFactory<R, F: FnOnce(&IListViewItemPresenterFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<ListViewItemPresenter, IListViewItemPresenterFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IListViewItemPresenterStatics<R, F: FnOnce(&IListViewItemPresenterStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<ListViewItemPresenter, IListViewItemPresenterStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IListViewItemPresenterStatics2<R, F: FnOnce(&IListViewItemPresenterStatics2) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<ListViewItemPresenter, IListViewItemPresenterStatics2> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IListViewItemPresenterStatics3<R, F: FnOnce(&IListViewItemPresenterStatics3) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<ListViewItemPresenter, IListViewItemPresenterStatics3> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IListViewItemPresenterStatics4<R, F: FnOnce(&IListViewItemPresenterStatics4) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<ListViewItemPresenter, IListViewItemPresenterStatics4> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for ListViewItemPresenter {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ListViewItemPresenter {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ListViewItemPresenter {}
impl ::core::fmt::Debug for ListViewItemPresenter {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ListViewItemPresenter").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ListViewItemPresenter {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Controls.Primitives.ListViewItemPresenter;{fc8946bd-a3a2-4969-8174-25b5d3c28033})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for ListViewItemPresenter {
    type Vtable = IListViewItemPresenter_Vtbl;
    const IID: ::windows_core::GUID = <IListViewItemPresenter as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for ListViewItemPresenter {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.ListViewItemPresenter";
}
impl ::core::convert::From<ListViewItemPresenter> for ::windows_core::IUnknown {
    fn from(value: ListViewItemPresenter) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ListViewItemPresenter> for ::windows_core::IUnknown {
    fn from(value: &ListViewItemPresenter) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ListViewItemPresenter {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ListViewItemPresenter {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ListViewItemPresenter> for ::windows_core::IInspectable {
    fn from(value: ListViewItemPresenter) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ListViewItemPresenter> for ::windows_core::IInspectable {
    fn from(value: &ListViewItemPresenter) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for ListViewItemPresenter {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a ListViewItemPresenter {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<ListViewItemPresenter> for super::super::super::Composition::IAnimationObject {
    type Error = ::windows_core::Error;
    fn try_from(value: ListViewItemPresenter) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<&ListViewItemPresenter> for super::super::super::Composition::IAnimationObject {
    type Error = ::windows_core::Error;
    fn try_from(value: &ListViewItemPresenter) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows_core::IntoParam<'a, super::super::super::Composition::IAnimationObject> for ListViewItemPresenter {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::super::Composition::IAnimationObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows_core::IntoParam<'a, super::super::super::Composition::IAnimationObject> for &ListViewItemPresenter {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::super::Composition::IAnimationObject> {
        ::core::convert::TryInto::<super::super::super::Composition::IAnimationObject>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<ListViewItemPresenter> for super::super::super::Composition::IVisualElement {
    type Error = ::windows_core::Error;
    fn try_from(value: ListViewItemPresenter) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<&ListViewItemPresenter> for super::super::super::Composition::IVisualElement {
    type Error = ::windows_core::Error;
    fn try_from(value: &ListViewItemPresenter) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows_core::IntoParam<'a, super::super::super::Composition::IVisualElement> for ListViewItemPresenter {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::super::Composition::IVisualElement> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows_core::IntoParam<'a, super::super::super::Composition::IVisualElement> for &ListViewItemPresenter {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::super::Composition::IVisualElement> {
        ::core::convert::TryInto::<super::super::super::Composition::IVisualElement>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::From<ListViewItemPresenter> for super::ContentPresenter {
    fn from(value: ListViewItemPresenter) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&ListViewItemPresenter> for super::ContentPresenter {
    fn from(value: &ListViewItemPresenter) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::ContentPresenter> for ListViewItemPresenter {
    fn into_param(self) -> ::windows_core::Param<'a, super::ContentPresenter> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::ContentPresenter> for &ListViewItemPresenter {
    fn into_param(self) -> ::windows_core::Param<'a, super::ContentPresenter> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::ContentPresenter>::into(self))
    }
}
impl ::core::convert::From<ListViewItemPresenter> for super::super::FrameworkElement {
    fn from(value: ListViewItemPresenter) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&ListViewItemPresenter> for super::super::FrameworkElement {
    fn from(value: &ListViewItemPresenter) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::FrameworkElement> for ListViewItemPresenter {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::FrameworkElement> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::FrameworkElement> for &ListViewItemPresenter {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::FrameworkElement> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::super::FrameworkElement>::into(self))
    }
}
impl ::core::convert::From<ListViewItemPresenter> for super::super::UIElement {
    fn from(value: ListViewItemPresenter) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&ListViewItemPresenter> for super::super::UIElement {
    fn from(value: &ListViewItemPresenter) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::UIElement> for ListViewItemPresenter {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::UIElement> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::UIElement> for &ListViewItemPresenter {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::UIElement> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::super::UIElement>::into(self))
    }
}
impl ::core::convert::From<ListViewItemPresenter> for super::super::DependencyObject {
    fn from(value: ListViewItemPresenter) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&ListViewItemPresenter> for super::super::DependencyObject {
    fn from(value: &ListViewItemPresenter) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::DependencyObject> for ListViewItemPresenter {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::DependencyObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::DependencyObject> for &ListViewItemPresenter {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::DependencyObject> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for ListViewItemPresenter {}
unsafe impl ::core::marker::Sync for ListViewItemPresenter {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct ListViewItemPresenterCheckMode(pub i32);
impl ListViewItemPresenterCheckMode {
    pub const Inline: Self = Self(0i32);
    pub const Overlay: Self = Self(1i32);
}
impl ::core::marker::Copy for ListViewItemPresenterCheckMode {}
impl ::core::clone::Clone for ListViewItemPresenterCheckMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ListViewItemPresenterCheckMode {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for ListViewItemPresenterCheckMode {
    type Abi = Self;
}
impl ::core::fmt::Debug for ListViewItemPresenterCheckMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ListViewItemPresenterCheckMode").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ListViewItemPresenterCheckMode {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Controls.Primitives.ListViewItemPresenterCheckMode;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct ListViewItemPresenterSelectionIndicatorMode(pub i32);
impl ListViewItemPresenterSelectionIndicatorMode {
    pub const Inline: Self = Self(0i32);
    pub const Overlay: Self = Self(1i32);
}
impl ::core::marker::Copy for ListViewItemPresenterSelectionIndicatorMode {}
impl ::core::clone::Clone for ListViewItemPresenterSelectionIndicatorMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ListViewItemPresenterSelectionIndicatorMode {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for ListViewItemPresenterSelectionIndicatorMode {
    type Abi = Self;
}
impl ::core::fmt::Debug for ListViewItemPresenterSelectionIndicatorMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ListViewItemPresenterSelectionIndicatorMode").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ListViewItemPresenterSelectionIndicatorMode {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Controls.Primitives.ListViewItemPresenterSelectionIndicatorMode;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct ListViewItemTemplateSettings(::windows_core::IUnknown);
impl ListViewItemTemplateSettings {
    pub fn DragItemsCount(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows_core::Interface::vtable(this).DragItemsCount)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
}
impl ::core::clone::Clone for ListViewItemTemplateSettings {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ListViewItemTemplateSettings {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ListViewItemTemplateSettings {}
impl ::core::fmt::Debug for ListViewItemTemplateSettings {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ListViewItemTemplateSettings").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ListViewItemTemplateSettings {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Controls.Primitives.ListViewItemTemplateSettings;{67af84bf-8279-4686-9326-cd189f27575d})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for ListViewItemTemplateSettings {
    type Vtable = IListViewItemTemplateSettings_Vtbl;
    const IID: ::windows_core::GUID = <IListViewItemTemplateSettings as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for ListViewItemTemplateSettings {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.ListViewItemTemplateSettings";
}
impl ::core::convert::From<ListViewItemTemplateSettings> for ::windows_core::IUnknown {
    fn from(value: ListViewItemTemplateSettings) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ListViewItemTemplateSettings> for ::windows_core::IUnknown {
    fn from(value: &ListViewItemTemplateSettings) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ListViewItemTemplateSettings {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ListViewItemTemplateSettings {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ListViewItemTemplateSettings> for ::windows_core::IInspectable {
    fn from(value: ListViewItemTemplateSettings) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ListViewItemTemplateSettings> for ::windows_core::IInspectable {
    fn from(value: &ListViewItemTemplateSettings) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for ListViewItemTemplateSettings {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a ListViewItemTemplateSettings {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ListViewItemTemplateSettings> for super::super::DependencyObject {
    fn from(value: ListViewItemTemplateSettings) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&ListViewItemTemplateSettings> for super::super::DependencyObject {
    fn from(value: &ListViewItemTemplateSettings) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::DependencyObject> for ListViewItemTemplateSettings {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::DependencyObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::DependencyObject> for &ListViewItemTemplateSettings {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::DependencyObject> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for ListViewItemTemplateSettings {}
unsafe impl ::core::marker::Sync for ListViewItemTemplateSettings {}
#[repr(transparent)]
pub struct LoopingSelector(::windows_core::IUnknown);
impl LoopingSelector {
    pub fn ShouldLoop(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).ShouldLoop)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetShouldLoop(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetShouldLoop)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Items(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVector<::windows_core::IInspectable>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Items)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVector<::windows_core::IInspectable>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn SetItems<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IVector<::windows_core::IInspectable>>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetItems)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn SelectedIndex(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows_core::Interface::vtable(this).SelectedIndex)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    pub fn SetSelectedIndex(&self, value: i32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetSelectedIndex)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn SelectedItem(&self) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows_core::Interface::vtable(this).SelectedItem)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::IInspectable>(result__)
        }
    }
    pub fn SetSelectedItem<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::IInspectable>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetSelectedItem)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn ItemWidth(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows_core::Interface::vtable(this).ItemWidth)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    pub fn SetItemWidth(&self, value: i32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetItemWidth)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ItemHeight(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows_core::Interface::vtable(this).ItemHeight)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    pub fn SetItemHeight(&self, value: i32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetItemHeight)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ItemTemplate(&self) -> ::windows_core::Result<super::super::DataTemplate> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ItemTemplate)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DataTemplate>(result__)
        }
    }
    pub fn SetItemTemplate<'a, Param0: ::windows_core::IntoParam<'a, super::super::DataTemplate>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetItemTemplate)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn SelectionChanged<'a, Param0: ::windows_core::IntoParam<'a, super::SelectionChangedEventHandler>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).SelectionChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveSelectionChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveSelectionChanged)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn ShouldLoopProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::ILoopingSelectorStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ShouldLoopProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn ItemsProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::ILoopingSelectorStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ItemsProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn SelectedIndexProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::ILoopingSelectorStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SelectedIndexProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn SelectedItemProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::ILoopingSelectorStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SelectedItemProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn ItemWidthProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::ILoopingSelectorStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ItemWidthProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn ItemHeightProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::ILoopingSelectorStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ItemHeightProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn ItemTemplateProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::ILoopingSelectorStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ItemTemplateProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn ILoopingSelectorStatics<R, F: FnOnce(&ILoopingSelectorStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<LoopingSelector, ILoopingSelectorStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for LoopingSelector {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for LoopingSelector {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for LoopingSelector {}
impl ::core::fmt::Debug for LoopingSelector {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LoopingSelector").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for LoopingSelector {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Controls.Primitives.LoopingSelector;{4c9a3e04-4827-49d9-8806-093957b0fd21})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for LoopingSelector {
    type Vtable = ILoopingSelector_Vtbl;
    const IID: ::windows_core::GUID = <ILoopingSelector as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for LoopingSelector {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.LoopingSelector";
}
impl ::core::convert::From<LoopingSelector> for ::windows_core::IUnknown {
    fn from(value: LoopingSelector) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LoopingSelector> for ::windows_core::IUnknown {
    fn from(value: &LoopingSelector) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for LoopingSelector {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a LoopingSelector {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<LoopingSelector> for ::windows_core::IInspectable {
    fn from(value: LoopingSelector) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LoopingSelector> for ::windows_core::IInspectable {
    fn from(value: &LoopingSelector) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for LoopingSelector {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a LoopingSelector {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<LoopingSelector> for super::super::super::Composition::IAnimationObject {
    type Error = ::windows_core::Error;
    fn try_from(value: LoopingSelector) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<&LoopingSelector> for super::super::super::Composition::IAnimationObject {
    type Error = ::windows_core::Error;
    fn try_from(value: &LoopingSelector) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows_core::IntoParam<'a, super::super::super::Composition::IAnimationObject> for LoopingSelector {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::super::Composition::IAnimationObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows_core::IntoParam<'a, super::super::super::Composition::IAnimationObject> for &LoopingSelector {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::super::Composition::IAnimationObject> {
        ::core::convert::TryInto::<super::super::super::Composition::IAnimationObject>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<LoopingSelector> for super::super::super::Composition::IVisualElement {
    type Error = ::windows_core::Error;
    fn try_from(value: LoopingSelector) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<&LoopingSelector> for super::super::super::Composition::IVisualElement {
    type Error = ::windows_core::Error;
    fn try_from(value: &LoopingSelector) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows_core::IntoParam<'a, super::super::super::Composition::IVisualElement> for LoopingSelector {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::super::Composition::IVisualElement> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows_core::IntoParam<'a, super::super::super::Composition::IVisualElement> for &LoopingSelector {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::super::Composition::IVisualElement> {
        ::core::convert::TryInto::<super::super::super::Composition::IVisualElement>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::From<LoopingSelector> for super::Control {
    fn from(value: LoopingSelector) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&LoopingSelector> for super::Control {
    fn from(value: &LoopingSelector) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::Control> for LoopingSelector {
    fn into_param(self) -> ::windows_core::Param<'a, super::Control> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::Control> for &LoopingSelector {
    fn into_param(self) -> ::windows_core::Param<'a, super::Control> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::Control>::into(self))
    }
}
impl ::core::convert::From<LoopingSelector> for super::super::FrameworkElement {
    fn from(value: LoopingSelector) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&LoopingSelector> for super::super::FrameworkElement {
    fn from(value: &LoopingSelector) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::FrameworkElement> for LoopingSelector {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::FrameworkElement> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::FrameworkElement> for &LoopingSelector {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::FrameworkElement> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::super::FrameworkElement>::into(self))
    }
}
impl ::core::convert::From<LoopingSelector> for super::super::UIElement {
    fn from(value: LoopingSelector) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&LoopingSelector> for super::super::UIElement {
    fn from(value: &LoopingSelector) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::UIElement> for LoopingSelector {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::UIElement> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::UIElement> for &LoopingSelector {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::UIElement> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::super::UIElement>::into(self))
    }
}
impl ::core::convert::From<LoopingSelector> for super::super::DependencyObject {
    fn from(value: LoopingSelector) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&LoopingSelector> for super::super::DependencyObject {
    fn from(value: &LoopingSelector) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::DependencyObject> for LoopingSelector {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::DependencyObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::DependencyObject> for &LoopingSelector {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::DependencyObject> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for LoopingSelector {}
unsafe impl ::core::marker::Sync for LoopingSelector {}
#[repr(transparent)]
pub struct LoopingSelectorItem(::windows_core::IUnknown);
impl LoopingSelectorItem {}
impl ::core::clone::Clone for LoopingSelectorItem {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for LoopingSelectorItem {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for LoopingSelectorItem {}
impl ::core::fmt::Debug for LoopingSelectorItem {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LoopingSelectorItem").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for LoopingSelectorItem {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Controls.Primitives.LoopingSelectorItem;{c69714b9-27c6-4433-9d7c-0dbfb2f4344f})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for LoopingSelectorItem {
    type Vtable = ILoopingSelectorItem_Vtbl;
    const IID: ::windows_core::GUID = <ILoopingSelectorItem as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for LoopingSelectorItem {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.LoopingSelectorItem";
}
impl ::core::convert::From<LoopingSelectorItem> for ::windows_core::IUnknown {
    fn from(value: LoopingSelectorItem) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LoopingSelectorItem> for ::windows_core::IUnknown {
    fn from(value: &LoopingSelectorItem) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for LoopingSelectorItem {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a LoopingSelectorItem {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<LoopingSelectorItem> for ::windows_core::IInspectable {
    fn from(value: LoopingSelectorItem) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LoopingSelectorItem> for ::windows_core::IInspectable {
    fn from(value: &LoopingSelectorItem) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for LoopingSelectorItem {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a LoopingSelectorItem {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<LoopingSelectorItem> for super::super::super::Composition::IAnimationObject {
    type Error = ::windows_core::Error;
    fn try_from(value: LoopingSelectorItem) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<&LoopingSelectorItem> for super::super::super::Composition::IAnimationObject {
    type Error = ::windows_core::Error;
    fn try_from(value: &LoopingSelectorItem) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows_core::IntoParam<'a, super::super::super::Composition::IAnimationObject> for LoopingSelectorItem {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::super::Composition::IAnimationObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows_core::IntoParam<'a, super::super::super::Composition::IAnimationObject> for &LoopingSelectorItem {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::super::Composition::IAnimationObject> {
        ::core::convert::TryInto::<super::super::super::Composition::IAnimationObject>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<LoopingSelectorItem> for super::super::super::Composition::IVisualElement {
    type Error = ::windows_core::Error;
    fn try_from(value: LoopingSelectorItem) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<&LoopingSelectorItem> for super::super::super::Composition::IVisualElement {
    type Error = ::windows_core::Error;
    fn try_from(value: &LoopingSelectorItem) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows_core::IntoParam<'a, super::super::super::Composition::IVisualElement> for LoopingSelectorItem {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::super::Composition::IVisualElement> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows_core::IntoParam<'a, super::super::super::Composition::IVisualElement> for &LoopingSelectorItem {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::super::Composition::IVisualElement> {
        ::core::convert::TryInto::<super::super::super::Composition::IVisualElement>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::From<LoopingSelectorItem> for super::ContentControl {
    fn from(value: LoopingSelectorItem) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&LoopingSelectorItem> for super::ContentControl {
    fn from(value: &LoopingSelectorItem) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::ContentControl> for LoopingSelectorItem {
    fn into_param(self) -> ::windows_core::Param<'a, super::ContentControl> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::ContentControl> for &LoopingSelectorItem {
    fn into_param(self) -> ::windows_core::Param<'a, super::ContentControl> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::ContentControl>::into(self))
    }
}
impl ::core::convert::From<LoopingSelectorItem> for super::Control {
    fn from(value: LoopingSelectorItem) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&LoopingSelectorItem> for super::Control {
    fn from(value: &LoopingSelectorItem) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::Control> for LoopingSelectorItem {
    fn into_param(self) -> ::windows_core::Param<'a, super::Control> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::Control> for &LoopingSelectorItem {
    fn into_param(self) -> ::windows_core::Param<'a, super::Control> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::Control>::into(self))
    }
}
impl ::core::convert::From<LoopingSelectorItem> for super::super::FrameworkElement {
    fn from(value: LoopingSelectorItem) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&LoopingSelectorItem> for super::super::FrameworkElement {
    fn from(value: &LoopingSelectorItem) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::FrameworkElement> for LoopingSelectorItem {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::FrameworkElement> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::FrameworkElement> for &LoopingSelectorItem {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::FrameworkElement> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::super::FrameworkElement>::into(self))
    }
}
impl ::core::convert::From<LoopingSelectorItem> for super::super::UIElement {
    fn from(value: LoopingSelectorItem) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&LoopingSelectorItem> for super::super::UIElement {
    fn from(value: &LoopingSelectorItem) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::UIElement> for LoopingSelectorItem {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::UIElement> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::UIElement> for &LoopingSelectorItem {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::UIElement> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::super::UIElement>::into(self))
    }
}
impl ::core::convert::From<LoopingSelectorItem> for super::super::DependencyObject {
    fn from(value: LoopingSelectorItem) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&LoopingSelectorItem> for super::super::DependencyObject {
    fn from(value: &LoopingSelectorItem) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::DependencyObject> for LoopingSelectorItem {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::DependencyObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::DependencyObject> for &LoopingSelectorItem {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::DependencyObject> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for LoopingSelectorItem {}
unsafe impl ::core::marker::Sync for LoopingSelectorItem {}
#[repr(transparent)]
pub struct LoopingSelectorPanel(::windows_core::IUnknown);
impl LoopingSelectorPanel {
    pub fn AreHorizontalSnapPointsRegular(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IScrollSnapPointsInfo>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).AreHorizontalSnapPointsRegular)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn AreVerticalSnapPointsRegular(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IScrollSnapPointsInfo>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).AreVerticalSnapPointsRegular)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn HorizontalSnapPointsChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventHandler<::windows_core::IInspectable>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = &::windows_core::Interface::cast::<IScrollSnapPointsInfo>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).HorizontalSnapPointsChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveHorizontalSnapPointsChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IScrollSnapPointsInfo>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).RemoveHorizontalSnapPointsChanged)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn VerticalSnapPointsChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventHandler<::windows_core::IInspectable>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = &::windows_core::Interface::cast::<IScrollSnapPointsInfo>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).VerticalSnapPointsChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveVerticalSnapPointsChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IScrollSnapPointsInfo>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).RemoveVerticalSnapPointsChanged)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetIrregularSnapPoints(&self, orientation: super::Orientation, alignment: SnapPointsAlignment) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<f32>> {
        let this = &::windows_core::Interface::cast::<IScrollSnapPointsInfo>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetIrregularSnapPoints)(::windows_core::Interface::as_raw(this), orientation, alignment, result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<f32>>(result__)
        }
    }
    pub fn GetRegularSnapPoints(&self, orientation: super::Orientation, alignment: SnapPointsAlignment, offset: &mut f32) -> ::windows_core::Result<f32> {
        let this = &::windows_core::Interface::cast::<IScrollSnapPointsInfo>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f32>::zeroed();
            (::windows_core::Interface::vtable(this).GetRegularSnapPoints)(::windows_core::Interface::as_raw(this), orientation, alignment, offset, result__.as_mut_ptr()).from_abi::<f32>(result__)
        }
    }
}
impl ::core::clone::Clone for LoopingSelectorPanel {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for LoopingSelectorPanel {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for LoopingSelectorPanel {}
impl ::core::fmt::Debug for LoopingSelectorPanel {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LoopingSelectorPanel").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for LoopingSelectorPanel {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Controls.Primitives.LoopingSelectorPanel;{40a9ba70-1011-4778-87f7-6bfd20d6377d})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for LoopingSelectorPanel {
    type Vtable = ILoopingSelectorPanel_Vtbl;
    const IID: ::windows_core::GUID = <ILoopingSelectorPanel as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for LoopingSelectorPanel {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.LoopingSelectorPanel";
}
impl ::core::convert::From<LoopingSelectorPanel> for ::windows_core::IUnknown {
    fn from(value: LoopingSelectorPanel) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LoopingSelectorPanel> for ::windows_core::IUnknown {
    fn from(value: &LoopingSelectorPanel) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for LoopingSelectorPanel {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a LoopingSelectorPanel {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<LoopingSelectorPanel> for ::windows_core::IInspectable {
    fn from(value: LoopingSelectorPanel) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LoopingSelectorPanel> for ::windows_core::IInspectable {
    fn from(value: &LoopingSelectorPanel) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for LoopingSelectorPanel {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a LoopingSelectorPanel {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<LoopingSelectorPanel> for super::super::super::Composition::IAnimationObject {
    type Error = ::windows_core::Error;
    fn try_from(value: LoopingSelectorPanel) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<&LoopingSelectorPanel> for super::super::super::Composition::IAnimationObject {
    type Error = ::windows_core::Error;
    fn try_from(value: &LoopingSelectorPanel) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows_core::IntoParam<'a, super::super::super::Composition::IAnimationObject> for LoopingSelectorPanel {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::super::Composition::IAnimationObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows_core::IntoParam<'a, super::super::super::Composition::IAnimationObject> for &LoopingSelectorPanel {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::super::Composition::IAnimationObject> {
        ::core::convert::TryInto::<super::super::super::Composition::IAnimationObject>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<LoopingSelectorPanel> for IScrollSnapPointsInfo {
    type Error = ::windows_core::Error;
    fn try_from(value: LoopingSelectorPanel) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&LoopingSelectorPanel> for IScrollSnapPointsInfo {
    type Error = ::windows_core::Error;
    fn try_from(value: &LoopingSelectorPanel) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IScrollSnapPointsInfo> for LoopingSelectorPanel {
    fn into_param(self) -> ::windows_core::Param<'a, IScrollSnapPointsInfo> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IScrollSnapPointsInfo> for &LoopingSelectorPanel {
    fn into_param(self) -> ::windows_core::Param<'a, IScrollSnapPointsInfo> {
        ::core::convert::TryInto::<IScrollSnapPointsInfo>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<LoopingSelectorPanel> for super::super::super::Composition::IVisualElement {
    type Error = ::windows_core::Error;
    fn try_from(value: LoopingSelectorPanel) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<&LoopingSelectorPanel> for super::super::super::Composition::IVisualElement {
    type Error = ::windows_core::Error;
    fn try_from(value: &LoopingSelectorPanel) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows_core::IntoParam<'a, super::super::super::Composition::IVisualElement> for LoopingSelectorPanel {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::super::Composition::IVisualElement> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows_core::IntoParam<'a, super::super::super::Composition::IVisualElement> for &LoopingSelectorPanel {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::super::Composition::IVisualElement> {
        ::core::convert::TryInto::<super::super::super::Composition::IVisualElement>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::From<LoopingSelectorPanel> for super::Canvas {
    fn from(value: LoopingSelectorPanel) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&LoopingSelectorPanel> for super::Canvas {
    fn from(value: &LoopingSelectorPanel) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::Canvas> for LoopingSelectorPanel {
    fn into_param(self) -> ::windows_core::Param<'a, super::Canvas> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::Canvas> for &LoopingSelectorPanel {
    fn into_param(self) -> ::windows_core::Param<'a, super::Canvas> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::Canvas>::into(self))
    }
}
impl ::core::convert::From<LoopingSelectorPanel> for super::Panel {
    fn from(value: LoopingSelectorPanel) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&LoopingSelectorPanel> for super::Panel {
    fn from(value: &LoopingSelectorPanel) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::Panel> for LoopingSelectorPanel {
    fn into_param(self) -> ::windows_core::Param<'a, super::Panel> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::Panel> for &LoopingSelectorPanel {
    fn into_param(self) -> ::windows_core::Param<'a, super::Panel> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::Panel>::into(self))
    }
}
impl ::core::convert::From<LoopingSelectorPanel> for super::super::FrameworkElement {
    fn from(value: LoopingSelectorPanel) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&LoopingSelectorPanel> for super::super::FrameworkElement {
    fn from(value: &LoopingSelectorPanel) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::FrameworkElement> for LoopingSelectorPanel {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::FrameworkElement> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::FrameworkElement> for &LoopingSelectorPanel {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::FrameworkElement> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::super::FrameworkElement>::into(self))
    }
}
impl ::core::convert::From<LoopingSelectorPanel> for super::super::UIElement {
    fn from(value: LoopingSelectorPanel) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&LoopingSelectorPanel> for super::super::UIElement {
    fn from(value: &LoopingSelectorPanel) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::UIElement> for LoopingSelectorPanel {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::UIElement> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::UIElement> for &LoopingSelectorPanel {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::UIElement> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::super::UIElement>::into(self))
    }
}
impl ::core::convert::From<LoopingSelectorPanel> for super::super::DependencyObject {
    fn from(value: LoopingSelectorPanel) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&LoopingSelectorPanel> for super::super::DependencyObject {
    fn from(value: &LoopingSelectorPanel) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::DependencyObject> for LoopingSelectorPanel {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::DependencyObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::DependencyObject> for &LoopingSelectorPanel {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::DependencyObject> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for LoopingSelectorPanel {}
unsafe impl ::core::marker::Sync for LoopingSelectorPanel {}
#[repr(transparent)]
pub struct MenuFlyoutItemTemplateSettings(::windows_core::IUnknown);
impl MenuFlyoutItemTemplateSettings {
    pub fn KeyboardAcceleratorTextMinWidth(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).KeyboardAcceleratorTextMinWidth)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
}
impl ::core::clone::Clone for MenuFlyoutItemTemplateSettings {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MenuFlyoutItemTemplateSettings {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MenuFlyoutItemTemplateSettings {}
impl ::core::fmt::Debug for MenuFlyoutItemTemplateSettings {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MenuFlyoutItemTemplateSettings").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for MenuFlyoutItemTemplateSettings {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Controls.Primitives.MenuFlyoutItemTemplateSettings;{56ad1809-3a16-4147-81cb-d0b35c834e0f})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for MenuFlyoutItemTemplateSettings {
    type Vtable = IMenuFlyoutItemTemplateSettings_Vtbl;
    const IID: ::windows_core::GUID = <IMenuFlyoutItemTemplateSettings as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for MenuFlyoutItemTemplateSettings {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.MenuFlyoutItemTemplateSettings";
}
impl ::core::convert::From<MenuFlyoutItemTemplateSettings> for ::windows_core::IUnknown {
    fn from(value: MenuFlyoutItemTemplateSettings) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MenuFlyoutItemTemplateSettings> for ::windows_core::IUnknown {
    fn from(value: &MenuFlyoutItemTemplateSettings) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for MenuFlyoutItemTemplateSettings {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a MenuFlyoutItemTemplateSettings {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MenuFlyoutItemTemplateSettings> for ::windows_core::IInspectable {
    fn from(value: MenuFlyoutItemTemplateSettings) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MenuFlyoutItemTemplateSettings> for ::windows_core::IInspectable {
    fn from(value: &MenuFlyoutItemTemplateSettings) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for MenuFlyoutItemTemplateSettings {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a MenuFlyoutItemTemplateSettings {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MenuFlyoutItemTemplateSettings> for super::super::DependencyObject {
    fn from(value: MenuFlyoutItemTemplateSettings) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&MenuFlyoutItemTemplateSettings> for super::super::DependencyObject {
    fn from(value: &MenuFlyoutItemTemplateSettings) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::DependencyObject> for MenuFlyoutItemTemplateSettings {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::DependencyObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::DependencyObject> for &MenuFlyoutItemTemplateSettings {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::DependencyObject> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for MenuFlyoutItemTemplateSettings {}
unsafe impl ::core::marker::Sync for MenuFlyoutItemTemplateSettings {}
#[repr(transparent)]
pub struct MenuFlyoutPresenterTemplateSettings(::windows_core::IUnknown);
impl MenuFlyoutPresenterTemplateSettings {
    pub fn FlyoutContentMinWidth(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).FlyoutContentMinWidth)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
}
impl ::core::clone::Clone for MenuFlyoutPresenterTemplateSettings {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MenuFlyoutPresenterTemplateSettings {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MenuFlyoutPresenterTemplateSettings {}
impl ::core::fmt::Debug for MenuFlyoutPresenterTemplateSettings {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MenuFlyoutPresenterTemplateSettings").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for MenuFlyoutPresenterTemplateSettings {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Controls.Primitives.MenuFlyoutPresenterTemplateSettings;{d68fd00d-629d-4349-ac51-b877c80983b8})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for MenuFlyoutPresenterTemplateSettings {
    type Vtable = IMenuFlyoutPresenterTemplateSettings_Vtbl;
    const IID: ::windows_core::GUID = <IMenuFlyoutPresenterTemplateSettings as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for MenuFlyoutPresenterTemplateSettings {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.MenuFlyoutPresenterTemplateSettings";
}
impl ::core::convert::From<MenuFlyoutPresenterTemplateSettings> for ::windows_core::IUnknown {
    fn from(value: MenuFlyoutPresenterTemplateSettings) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MenuFlyoutPresenterTemplateSettings> for ::windows_core::IUnknown {
    fn from(value: &MenuFlyoutPresenterTemplateSettings) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for MenuFlyoutPresenterTemplateSettings {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a MenuFlyoutPresenterTemplateSettings {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MenuFlyoutPresenterTemplateSettings> for ::windows_core::IInspectable {
    fn from(value: MenuFlyoutPresenterTemplateSettings) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MenuFlyoutPresenterTemplateSettings> for ::windows_core::IInspectable {
    fn from(value: &MenuFlyoutPresenterTemplateSettings) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for MenuFlyoutPresenterTemplateSettings {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a MenuFlyoutPresenterTemplateSettings {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MenuFlyoutPresenterTemplateSettings> for super::super::DependencyObject {
    fn from(value: MenuFlyoutPresenterTemplateSettings) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&MenuFlyoutPresenterTemplateSettings> for super::super::DependencyObject {
    fn from(value: &MenuFlyoutPresenterTemplateSettings) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::DependencyObject> for MenuFlyoutPresenterTemplateSettings {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::DependencyObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::DependencyObject> for &MenuFlyoutPresenterTemplateSettings {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::DependencyObject> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for MenuFlyoutPresenterTemplateSettings {}
unsafe impl ::core::marker::Sync for MenuFlyoutPresenterTemplateSettings {}
#[repr(transparent)]
pub struct NavigationViewItemPresenter(::windows_core::IUnknown);
impl NavigationViewItemPresenter {
    pub fn Icon(&self) -> ::windows_core::Result<super::IconElement> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Icon)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::IconElement>(result__)
        }
    }
    pub fn SetIcon<'a, Param0: ::windows_core::IntoParam<'a, super::IconElement>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetIcon)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn new() -> ::windows_core::Result<NavigationViewItemPresenter> {
        Self::INavigationViewItemPresenterFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateInstance)(::windows_core::Interface::as_raw(this), ::core::ptr::null_mut(), &mut ::core::option::Option::<::windows_core::IInspectable>::None as *mut _ as _, result__.as_mut_ptr()).from_abi::<NavigationViewItemPresenter>(result__)
        })
    }
    pub fn compose<T: ::windows_core::Compose>(compose: T) -> ::windows_core::Result<NavigationViewItemPresenter> {
        Self::INavigationViewItemPresenterFactory(|this| unsafe {
            let (derived__, base__) = ::windows_core::Compose::compose(compose);
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateInstance)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(&derived__), base__ as *mut _ as _, result__.as_mut_ptr()).from_abi::<NavigationViewItemPresenter>(result__)
        })
    }
    pub fn IconProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::INavigationViewItemPresenterStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).IconProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn INavigationViewItemPresenterFactory<R, F: FnOnce(&INavigationViewItemPresenterFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<NavigationViewItemPresenter, INavigationViewItemPresenterFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn INavigationViewItemPresenterStatics<R, F: FnOnce(&INavigationViewItemPresenterStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<NavigationViewItemPresenter, INavigationViewItemPresenterStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for NavigationViewItemPresenter {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for NavigationViewItemPresenter {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for NavigationViewItemPresenter {}
impl ::core::fmt::Debug for NavigationViewItemPresenter {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NavigationViewItemPresenter").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for NavigationViewItemPresenter {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Controls.Primitives.NavigationViewItemPresenter;{9956d3fc-4693-59cb-b6bf-37249058be96})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for NavigationViewItemPresenter {
    type Vtable = INavigationViewItemPresenter_Vtbl;
    const IID: ::windows_core::GUID = <INavigationViewItemPresenter as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for NavigationViewItemPresenter {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.NavigationViewItemPresenter";
}
impl ::core::convert::From<NavigationViewItemPresenter> for ::windows_core::IUnknown {
    fn from(value: NavigationViewItemPresenter) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&NavigationViewItemPresenter> for ::windows_core::IUnknown {
    fn from(value: &NavigationViewItemPresenter) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for NavigationViewItemPresenter {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a NavigationViewItemPresenter {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<NavigationViewItemPresenter> for ::windows_core::IInspectable {
    fn from(value: NavigationViewItemPresenter) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&NavigationViewItemPresenter> for ::windows_core::IInspectable {
    fn from(value: &NavigationViewItemPresenter) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for NavigationViewItemPresenter {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a NavigationViewItemPresenter {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<NavigationViewItemPresenter> for super::super::super::Composition::IAnimationObject {
    type Error = ::windows_core::Error;
    fn try_from(value: NavigationViewItemPresenter) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<&NavigationViewItemPresenter> for super::super::super::Composition::IAnimationObject {
    type Error = ::windows_core::Error;
    fn try_from(value: &NavigationViewItemPresenter) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows_core::IntoParam<'a, super::super::super::Composition::IAnimationObject> for NavigationViewItemPresenter {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::super::Composition::IAnimationObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows_core::IntoParam<'a, super::super::super::Composition::IAnimationObject> for &NavigationViewItemPresenter {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::super::Composition::IAnimationObject> {
        ::core::convert::TryInto::<super::super::super::Composition::IAnimationObject>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<NavigationViewItemPresenter> for super::super::super::Composition::IVisualElement {
    type Error = ::windows_core::Error;
    fn try_from(value: NavigationViewItemPresenter) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<&NavigationViewItemPresenter> for super::super::super::Composition::IVisualElement {
    type Error = ::windows_core::Error;
    fn try_from(value: &NavigationViewItemPresenter) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows_core::IntoParam<'a, super::super::super::Composition::IVisualElement> for NavigationViewItemPresenter {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::super::Composition::IVisualElement> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows_core::IntoParam<'a, super::super::super::Composition::IVisualElement> for &NavigationViewItemPresenter {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::super::Composition::IVisualElement> {
        ::core::convert::TryInto::<super::super::super::Composition::IVisualElement>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::From<NavigationViewItemPresenter> for super::ContentControl {
    fn from(value: NavigationViewItemPresenter) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&NavigationViewItemPresenter> for super::ContentControl {
    fn from(value: &NavigationViewItemPresenter) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::ContentControl> for NavigationViewItemPresenter {
    fn into_param(self) -> ::windows_core::Param<'a, super::ContentControl> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::ContentControl> for &NavigationViewItemPresenter {
    fn into_param(self) -> ::windows_core::Param<'a, super::ContentControl> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::ContentControl>::into(self))
    }
}
impl ::core::convert::From<NavigationViewItemPresenter> for super::Control {
    fn from(value: NavigationViewItemPresenter) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&NavigationViewItemPresenter> for super::Control {
    fn from(value: &NavigationViewItemPresenter) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::Control> for NavigationViewItemPresenter {
    fn into_param(self) -> ::windows_core::Param<'a, super::Control> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::Control> for &NavigationViewItemPresenter {
    fn into_param(self) -> ::windows_core::Param<'a, super::Control> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::Control>::into(self))
    }
}
impl ::core::convert::From<NavigationViewItemPresenter> for super::super::FrameworkElement {
    fn from(value: NavigationViewItemPresenter) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&NavigationViewItemPresenter> for super::super::FrameworkElement {
    fn from(value: &NavigationViewItemPresenter) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::FrameworkElement> for NavigationViewItemPresenter {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::FrameworkElement> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::FrameworkElement> for &NavigationViewItemPresenter {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::FrameworkElement> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::super::FrameworkElement>::into(self))
    }
}
impl ::core::convert::From<NavigationViewItemPresenter> for super::super::UIElement {
    fn from(value: NavigationViewItemPresenter) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&NavigationViewItemPresenter> for super::super::UIElement {
    fn from(value: &NavigationViewItemPresenter) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::UIElement> for NavigationViewItemPresenter {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::UIElement> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::UIElement> for &NavigationViewItemPresenter {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::UIElement> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::super::UIElement>::into(self))
    }
}
impl ::core::convert::From<NavigationViewItemPresenter> for super::super::DependencyObject {
    fn from(value: NavigationViewItemPresenter) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&NavigationViewItemPresenter> for super::super::DependencyObject {
    fn from(value: &NavigationViewItemPresenter) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::DependencyObject> for NavigationViewItemPresenter {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::DependencyObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::DependencyObject> for &NavigationViewItemPresenter {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::DependencyObject> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for NavigationViewItemPresenter {}
unsafe impl ::core::marker::Sync for NavigationViewItemPresenter {}
#[repr(transparent)]
pub struct OrientedVirtualizingPanel(::windows_core::IUnknown);
impl OrientedVirtualizingPanel {
    pub fn GetInsertionIndexes<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Point>>(&self, position: Param0, first: &mut i32, second: &mut i32) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<super::IInsertionPanel>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).GetInsertionIndexes)(::windows_core::Interface::as_raw(this), position.into_param().abi(), first, second).ok() }
    }
    pub fn CanVerticallyScroll(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).CanVerticallyScroll)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetCanVerticallyScroll(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetCanVerticallyScroll)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn CanHorizontallyScroll(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).CanHorizontallyScroll)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetCanHorizontallyScroll(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetCanHorizontallyScroll)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ExtentWidth(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).ExtentWidth)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn ExtentHeight(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).ExtentHeight)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn ViewportWidth(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).ViewportWidth)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn ViewportHeight(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).ViewportHeight)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn HorizontalOffset(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).HorizontalOffset)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn VerticalOffset(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).VerticalOffset)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn ScrollOwner(&self) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows_core::Interface::vtable(this).ScrollOwner)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::IInspectable>(result__)
        }
    }
    pub fn SetScrollOwner<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::IInspectable>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetScrollOwner)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn LineUp(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).LineUp)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn LineDown(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).LineDown)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn LineLeft(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).LineLeft)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn LineRight(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).LineRight)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn PageUp(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).PageUp)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn PageDown(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).PageDown)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn PageLeft(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).PageLeft)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn PageRight(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).PageRight)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn MouseWheelUp(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).MouseWheelUp)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn MouseWheelDown(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).MouseWheelDown)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn MouseWheelLeft(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).MouseWheelLeft)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn MouseWheelRight(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).MouseWheelRight)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn SetHorizontalOffset(&self, offset: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetHorizontalOffset)(::windows_core::Interface::as_raw(this), offset).ok() }
    }
    pub fn SetVerticalOffset(&self, offset: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetVerticalOffset)(::windows_core::Interface::as_raw(this), offset).ok() }
    }
    pub fn MakeVisible<'a, Param0: ::windows_core::IntoParam<'a, super::super::UIElement>, Param1: ::windows_core::IntoParam<'a, ::winrt_foundation::Rect>>(&self, visual: Param0, rectangle: Param1) -> ::windows_core::Result<::winrt_foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::Rect>::zeroed();
            (::windows_core::Interface::vtable(this).MakeVisible)(::windows_core::Interface::as_raw(this), visual.into_param().abi(), rectangle.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Rect>(result__)
        }
    }
    pub fn AreHorizontalSnapPointsRegular(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IScrollSnapPointsInfo>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).AreHorizontalSnapPointsRegular)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn AreVerticalSnapPointsRegular(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IScrollSnapPointsInfo>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).AreVerticalSnapPointsRegular)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn HorizontalSnapPointsChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventHandler<::windows_core::IInspectable>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = &::windows_core::Interface::cast::<IScrollSnapPointsInfo>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).HorizontalSnapPointsChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveHorizontalSnapPointsChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IScrollSnapPointsInfo>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).RemoveHorizontalSnapPointsChanged)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn VerticalSnapPointsChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventHandler<::windows_core::IInspectable>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = &::windows_core::Interface::cast::<IScrollSnapPointsInfo>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).VerticalSnapPointsChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveVerticalSnapPointsChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IScrollSnapPointsInfo>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).RemoveVerticalSnapPointsChanged)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetIrregularSnapPoints(&self, orientation: super::Orientation, alignment: SnapPointsAlignment) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<f32>> {
        let this = &::windows_core::Interface::cast::<IScrollSnapPointsInfo>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetIrregularSnapPoints)(::windows_core::Interface::as_raw(this), orientation, alignment, result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<f32>>(result__)
        }
    }
    pub fn GetRegularSnapPoints(&self, orientation: super::Orientation, alignment: SnapPointsAlignment, offset: &mut f32) -> ::windows_core::Result<f32> {
        let this = &::windows_core::Interface::cast::<IScrollSnapPointsInfo>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f32>::zeroed();
            (::windows_core::Interface::vtable(this).GetRegularSnapPoints)(::windows_core::Interface::as_raw(this), orientation, alignment, offset, result__.as_mut_ptr()).from_abi::<f32>(result__)
        }
    }
}
impl ::core::clone::Clone for OrientedVirtualizingPanel {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for OrientedVirtualizingPanel {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for OrientedVirtualizingPanel {}
impl ::core::fmt::Debug for OrientedVirtualizingPanel {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("OrientedVirtualizingPanel").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for OrientedVirtualizingPanel {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Controls.Primitives.OrientedVirtualizingPanel;{f077b577-39bd-46ee-bdd7-0826beed71b8})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for OrientedVirtualizingPanel {
    type Vtable = IOrientedVirtualizingPanel_Vtbl;
    const IID: ::windows_core::GUID = <IOrientedVirtualizingPanel as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for OrientedVirtualizingPanel {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.OrientedVirtualizingPanel";
}
impl ::core::convert::From<OrientedVirtualizingPanel> for ::windows_core::IUnknown {
    fn from(value: OrientedVirtualizingPanel) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&OrientedVirtualizingPanel> for ::windows_core::IUnknown {
    fn from(value: &OrientedVirtualizingPanel) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for OrientedVirtualizingPanel {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a OrientedVirtualizingPanel {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<OrientedVirtualizingPanel> for ::windows_core::IInspectable {
    fn from(value: OrientedVirtualizingPanel) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&OrientedVirtualizingPanel> for ::windows_core::IInspectable {
    fn from(value: &OrientedVirtualizingPanel) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for OrientedVirtualizingPanel {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a OrientedVirtualizingPanel {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<OrientedVirtualizingPanel> for super::super::super::Composition::IAnimationObject {
    type Error = ::windows_core::Error;
    fn try_from(value: OrientedVirtualizingPanel) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<&OrientedVirtualizingPanel> for super::super::super::Composition::IAnimationObject {
    type Error = ::windows_core::Error;
    fn try_from(value: &OrientedVirtualizingPanel) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows_core::IntoParam<'a, super::super::super::Composition::IAnimationObject> for OrientedVirtualizingPanel {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::super::Composition::IAnimationObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows_core::IntoParam<'a, super::super::super::Composition::IAnimationObject> for &OrientedVirtualizingPanel {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::super::Composition::IAnimationObject> {
        ::core::convert::TryInto::<super::super::super::Composition::IAnimationObject>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<OrientedVirtualizingPanel> for super::IInsertionPanel {
    type Error = ::windows_core::Error;
    fn try_from(value: OrientedVirtualizingPanel) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&OrientedVirtualizingPanel> for super::IInsertionPanel {
    type Error = ::windows_core::Error;
    fn try_from(value: &OrientedVirtualizingPanel) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::IInsertionPanel> for OrientedVirtualizingPanel {
    fn into_param(self) -> ::windows_core::Param<'a, super::IInsertionPanel> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::IInsertionPanel> for &OrientedVirtualizingPanel {
    fn into_param(self) -> ::windows_core::Param<'a, super::IInsertionPanel> {
        ::core::convert::TryInto::<super::IInsertionPanel>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<OrientedVirtualizingPanel> for IScrollSnapPointsInfo {
    type Error = ::windows_core::Error;
    fn try_from(value: OrientedVirtualizingPanel) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&OrientedVirtualizingPanel> for IScrollSnapPointsInfo {
    type Error = ::windows_core::Error;
    fn try_from(value: &OrientedVirtualizingPanel) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IScrollSnapPointsInfo> for OrientedVirtualizingPanel {
    fn into_param(self) -> ::windows_core::Param<'a, IScrollSnapPointsInfo> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IScrollSnapPointsInfo> for &OrientedVirtualizingPanel {
    fn into_param(self) -> ::windows_core::Param<'a, IScrollSnapPointsInfo> {
        ::core::convert::TryInto::<IScrollSnapPointsInfo>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<OrientedVirtualizingPanel> for super::super::super::Composition::IVisualElement {
    type Error = ::windows_core::Error;
    fn try_from(value: OrientedVirtualizingPanel) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<&OrientedVirtualizingPanel> for super::super::super::Composition::IVisualElement {
    type Error = ::windows_core::Error;
    fn try_from(value: &OrientedVirtualizingPanel) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows_core::IntoParam<'a, super::super::super::Composition::IVisualElement> for OrientedVirtualizingPanel {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::super::Composition::IVisualElement> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows_core::IntoParam<'a, super::super::super::Composition::IVisualElement> for &OrientedVirtualizingPanel {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::super::Composition::IVisualElement> {
        ::core::convert::TryInto::<super::super::super::Composition::IVisualElement>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::From<OrientedVirtualizingPanel> for super::VirtualizingPanel {
    fn from(value: OrientedVirtualizingPanel) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&OrientedVirtualizingPanel> for super::VirtualizingPanel {
    fn from(value: &OrientedVirtualizingPanel) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::VirtualizingPanel> for OrientedVirtualizingPanel {
    fn into_param(self) -> ::windows_core::Param<'a, super::VirtualizingPanel> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::VirtualizingPanel> for &OrientedVirtualizingPanel {
    fn into_param(self) -> ::windows_core::Param<'a, super::VirtualizingPanel> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::VirtualizingPanel>::into(self))
    }
}
impl ::core::convert::From<OrientedVirtualizingPanel> for super::Panel {
    fn from(value: OrientedVirtualizingPanel) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&OrientedVirtualizingPanel> for super::Panel {
    fn from(value: &OrientedVirtualizingPanel) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::Panel> for OrientedVirtualizingPanel {
    fn into_param(self) -> ::windows_core::Param<'a, super::Panel> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::Panel> for &OrientedVirtualizingPanel {
    fn into_param(self) -> ::windows_core::Param<'a, super::Panel> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::Panel>::into(self))
    }
}
impl ::core::convert::From<OrientedVirtualizingPanel> for super::super::FrameworkElement {
    fn from(value: OrientedVirtualizingPanel) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&OrientedVirtualizingPanel> for super::super::FrameworkElement {
    fn from(value: &OrientedVirtualizingPanel) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::FrameworkElement> for OrientedVirtualizingPanel {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::FrameworkElement> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::FrameworkElement> for &OrientedVirtualizingPanel {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::FrameworkElement> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::super::FrameworkElement>::into(self))
    }
}
impl ::core::convert::From<OrientedVirtualizingPanel> for super::super::UIElement {
    fn from(value: OrientedVirtualizingPanel) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&OrientedVirtualizingPanel> for super::super::UIElement {
    fn from(value: &OrientedVirtualizingPanel) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::UIElement> for OrientedVirtualizingPanel {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::UIElement> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::UIElement> for &OrientedVirtualizingPanel {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::UIElement> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::super::UIElement>::into(self))
    }
}
impl ::core::convert::From<OrientedVirtualizingPanel> for super::super::DependencyObject {
    fn from(value: OrientedVirtualizingPanel) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&OrientedVirtualizingPanel> for super::super::DependencyObject {
    fn from(value: &OrientedVirtualizingPanel) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::DependencyObject> for OrientedVirtualizingPanel {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::DependencyObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::DependencyObject> for &OrientedVirtualizingPanel {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::DependencyObject> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for OrientedVirtualizingPanel {}
unsafe impl ::core::marker::Sync for OrientedVirtualizingPanel {}
#[repr(transparent)]
pub struct PickerFlyoutBase(::windows_core::IUnknown);
impl PickerFlyoutBase {
    pub fn TitleProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::IPickerFlyoutBaseStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TitleProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn GetTitle<'a, Param0: ::windows_core::IntoParam<'a, super::super::DependencyObject>>(element: Param0) -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IPickerFlyoutBaseStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).GetTitle)(::windows_core::Interface::as_raw(this), element.into_param().abi(), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn SetTitle<'a, Param0: ::windows_core::IntoParam<'a, super::super::DependencyObject>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(element: Param0, value: Param1) -> ::windows_core::Result<()> {
        Self::IPickerFlyoutBaseStatics(|this| unsafe { (::windows_core::Interface::vtable(this).SetTitle)(::windows_core::Interface::as_raw(this), element.into_param().abi(), value.into_param().abi()).ok() })
    }
    pub fn IPickerFlyoutBaseStatics<R, F: FnOnce(&IPickerFlyoutBaseStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<PickerFlyoutBase, IPickerFlyoutBaseStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for PickerFlyoutBase {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PickerFlyoutBase {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PickerFlyoutBase {}
impl ::core::fmt::Debug for PickerFlyoutBase {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PickerFlyoutBase").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PickerFlyoutBase {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Controls.Primitives.PickerFlyoutBase;{e33574ea-1076-44d1-9383-dc24ac5cff2a})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for PickerFlyoutBase {
    type Vtable = IPickerFlyoutBase_Vtbl;
    const IID: ::windows_core::GUID = <IPickerFlyoutBase as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for PickerFlyoutBase {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.PickerFlyoutBase";
}
impl ::core::convert::From<PickerFlyoutBase> for ::windows_core::IUnknown {
    fn from(value: PickerFlyoutBase) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PickerFlyoutBase> for ::windows_core::IUnknown {
    fn from(value: &PickerFlyoutBase) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for PickerFlyoutBase {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a PickerFlyoutBase {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PickerFlyoutBase> for ::windows_core::IInspectable {
    fn from(value: PickerFlyoutBase) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PickerFlyoutBase> for ::windows_core::IInspectable {
    fn from(value: &PickerFlyoutBase) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for PickerFlyoutBase {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a PickerFlyoutBase {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PickerFlyoutBase> for FlyoutBase {
    fn from(value: PickerFlyoutBase) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&PickerFlyoutBase> for FlyoutBase {
    fn from(value: &PickerFlyoutBase) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, FlyoutBase> for PickerFlyoutBase {
    fn into_param(self) -> ::windows_core::Param<'a, FlyoutBase> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, FlyoutBase> for &PickerFlyoutBase {
    fn into_param(self) -> ::windows_core::Param<'a, FlyoutBase> {
        ::windows_core::Param::Owned(::core::convert::Into::<FlyoutBase>::into(self))
    }
}
impl ::core::convert::From<PickerFlyoutBase> for super::super::DependencyObject {
    fn from(value: PickerFlyoutBase) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&PickerFlyoutBase> for super::super::DependencyObject {
    fn from(value: &PickerFlyoutBase) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::DependencyObject> for PickerFlyoutBase {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::DependencyObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::DependencyObject> for &PickerFlyoutBase {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::DependencyObject> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for PickerFlyoutBase {}
unsafe impl ::core::marker::Sync for PickerFlyoutBase {}
#[repr(transparent)]
pub struct PivotHeaderItem(::windows_core::IUnknown);
impl PivotHeaderItem {
    pub fn new() -> ::windows_core::Result<PivotHeaderItem> {
        Self::IPivotHeaderItemFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateInstance)(::windows_core::Interface::as_raw(this), ::core::ptr::null_mut(), &mut ::core::option::Option::<::windows_core::IInspectable>::None as *mut _ as _, result__.as_mut_ptr()).from_abi::<PivotHeaderItem>(result__)
        })
    }
    pub fn compose<T: ::windows_core::Compose>(compose: T) -> ::windows_core::Result<PivotHeaderItem> {
        Self::IPivotHeaderItemFactory(|this| unsafe {
            let (derived__, base__) = ::windows_core::Compose::compose(compose);
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateInstance)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(&derived__), base__ as *mut _ as _, result__.as_mut_ptr()).from_abi::<PivotHeaderItem>(result__)
        })
    }
    pub fn IPivotHeaderItemFactory<R, F: FnOnce(&IPivotHeaderItemFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<PivotHeaderItem, IPivotHeaderItemFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for PivotHeaderItem {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PivotHeaderItem {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PivotHeaderItem {}
impl ::core::fmt::Debug for PivotHeaderItem {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PivotHeaderItem").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PivotHeaderItem {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Controls.Primitives.PivotHeaderItem;{594572c2-82aa-410b-9e55-fd8e2c98862d})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for PivotHeaderItem {
    type Vtable = IPivotHeaderItem_Vtbl;
    const IID: ::windows_core::GUID = <IPivotHeaderItem as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for PivotHeaderItem {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.PivotHeaderItem";
}
impl ::core::convert::From<PivotHeaderItem> for ::windows_core::IUnknown {
    fn from(value: PivotHeaderItem) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PivotHeaderItem> for ::windows_core::IUnknown {
    fn from(value: &PivotHeaderItem) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for PivotHeaderItem {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a PivotHeaderItem {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PivotHeaderItem> for ::windows_core::IInspectable {
    fn from(value: PivotHeaderItem) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PivotHeaderItem> for ::windows_core::IInspectable {
    fn from(value: &PivotHeaderItem) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for PivotHeaderItem {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a PivotHeaderItem {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<PivotHeaderItem> for super::super::super::Composition::IAnimationObject {
    type Error = ::windows_core::Error;
    fn try_from(value: PivotHeaderItem) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<&PivotHeaderItem> for super::super::super::Composition::IAnimationObject {
    type Error = ::windows_core::Error;
    fn try_from(value: &PivotHeaderItem) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows_core::IntoParam<'a, super::super::super::Composition::IAnimationObject> for PivotHeaderItem {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::super::Composition::IAnimationObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows_core::IntoParam<'a, super::super::super::Composition::IAnimationObject> for &PivotHeaderItem {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::super::Composition::IAnimationObject> {
        ::core::convert::TryInto::<super::super::super::Composition::IAnimationObject>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<PivotHeaderItem> for super::super::super::Composition::IVisualElement {
    type Error = ::windows_core::Error;
    fn try_from(value: PivotHeaderItem) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<&PivotHeaderItem> for super::super::super::Composition::IVisualElement {
    type Error = ::windows_core::Error;
    fn try_from(value: &PivotHeaderItem) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows_core::IntoParam<'a, super::super::super::Composition::IVisualElement> for PivotHeaderItem {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::super::Composition::IVisualElement> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows_core::IntoParam<'a, super::super::super::Composition::IVisualElement> for &PivotHeaderItem {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::super::Composition::IVisualElement> {
        ::core::convert::TryInto::<super::super::super::Composition::IVisualElement>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::From<PivotHeaderItem> for super::ContentControl {
    fn from(value: PivotHeaderItem) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&PivotHeaderItem> for super::ContentControl {
    fn from(value: &PivotHeaderItem) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::ContentControl> for PivotHeaderItem {
    fn into_param(self) -> ::windows_core::Param<'a, super::ContentControl> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::ContentControl> for &PivotHeaderItem {
    fn into_param(self) -> ::windows_core::Param<'a, super::ContentControl> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::ContentControl>::into(self))
    }
}
impl ::core::convert::From<PivotHeaderItem> for super::Control {
    fn from(value: PivotHeaderItem) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&PivotHeaderItem> for super::Control {
    fn from(value: &PivotHeaderItem) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::Control> for PivotHeaderItem {
    fn into_param(self) -> ::windows_core::Param<'a, super::Control> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::Control> for &PivotHeaderItem {
    fn into_param(self) -> ::windows_core::Param<'a, super::Control> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::Control>::into(self))
    }
}
impl ::core::convert::From<PivotHeaderItem> for super::super::FrameworkElement {
    fn from(value: PivotHeaderItem) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&PivotHeaderItem> for super::super::FrameworkElement {
    fn from(value: &PivotHeaderItem) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::FrameworkElement> for PivotHeaderItem {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::FrameworkElement> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::FrameworkElement> for &PivotHeaderItem {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::FrameworkElement> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::super::FrameworkElement>::into(self))
    }
}
impl ::core::convert::From<PivotHeaderItem> for super::super::UIElement {
    fn from(value: PivotHeaderItem) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&PivotHeaderItem> for super::super::UIElement {
    fn from(value: &PivotHeaderItem) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::UIElement> for PivotHeaderItem {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::UIElement> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::UIElement> for &PivotHeaderItem {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::UIElement> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::super::UIElement>::into(self))
    }
}
impl ::core::convert::From<PivotHeaderItem> for super::super::DependencyObject {
    fn from(value: PivotHeaderItem) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&PivotHeaderItem> for super::super::DependencyObject {
    fn from(value: &PivotHeaderItem) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::DependencyObject> for PivotHeaderItem {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::DependencyObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::DependencyObject> for &PivotHeaderItem {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::DependencyObject> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for PivotHeaderItem {}
unsafe impl ::core::marker::Sync for PivotHeaderItem {}
#[repr(transparent)]
pub struct PivotHeaderPanel(::windows_core::IUnknown);
impl PivotHeaderPanel {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<PivotHeaderPanel, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for PivotHeaderPanel {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PivotHeaderPanel {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PivotHeaderPanel {}
impl ::core::fmt::Debug for PivotHeaderPanel {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PivotHeaderPanel").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PivotHeaderPanel {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Controls.Primitives.PivotHeaderPanel;{21484ebc-9241-4203-bd37-6c08fb096612})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for PivotHeaderPanel {
    type Vtable = IPivotHeaderPanel_Vtbl;
    const IID: ::windows_core::GUID = <IPivotHeaderPanel as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for PivotHeaderPanel {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.PivotHeaderPanel";
}
impl ::core::convert::From<PivotHeaderPanel> for ::windows_core::IUnknown {
    fn from(value: PivotHeaderPanel) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PivotHeaderPanel> for ::windows_core::IUnknown {
    fn from(value: &PivotHeaderPanel) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for PivotHeaderPanel {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a PivotHeaderPanel {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PivotHeaderPanel> for ::windows_core::IInspectable {
    fn from(value: PivotHeaderPanel) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PivotHeaderPanel> for ::windows_core::IInspectable {
    fn from(value: &PivotHeaderPanel) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for PivotHeaderPanel {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a PivotHeaderPanel {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<PivotHeaderPanel> for super::super::super::Composition::IAnimationObject {
    type Error = ::windows_core::Error;
    fn try_from(value: PivotHeaderPanel) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<&PivotHeaderPanel> for super::super::super::Composition::IAnimationObject {
    type Error = ::windows_core::Error;
    fn try_from(value: &PivotHeaderPanel) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows_core::IntoParam<'a, super::super::super::Composition::IAnimationObject> for PivotHeaderPanel {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::super::Composition::IAnimationObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows_core::IntoParam<'a, super::super::super::Composition::IAnimationObject> for &PivotHeaderPanel {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::super::Composition::IAnimationObject> {
        ::core::convert::TryInto::<super::super::super::Composition::IAnimationObject>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<PivotHeaderPanel> for super::super::super::Composition::IVisualElement {
    type Error = ::windows_core::Error;
    fn try_from(value: PivotHeaderPanel) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<&PivotHeaderPanel> for super::super::super::Composition::IVisualElement {
    type Error = ::windows_core::Error;
    fn try_from(value: &PivotHeaderPanel) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows_core::IntoParam<'a, super::super::super::Composition::IVisualElement> for PivotHeaderPanel {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::super::Composition::IVisualElement> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows_core::IntoParam<'a, super::super::super::Composition::IVisualElement> for &PivotHeaderPanel {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::super::Composition::IVisualElement> {
        ::core::convert::TryInto::<super::super::super::Composition::IVisualElement>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::From<PivotHeaderPanel> for super::Canvas {
    fn from(value: PivotHeaderPanel) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&PivotHeaderPanel> for super::Canvas {
    fn from(value: &PivotHeaderPanel) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::Canvas> for PivotHeaderPanel {
    fn into_param(self) -> ::windows_core::Param<'a, super::Canvas> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::Canvas> for &PivotHeaderPanel {
    fn into_param(self) -> ::windows_core::Param<'a, super::Canvas> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::Canvas>::into(self))
    }
}
impl ::core::convert::From<PivotHeaderPanel> for super::Panel {
    fn from(value: PivotHeaderPanel) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&PivotHeaderPanel> for super::Panel {
    fn from(value: &PivotHeaderPanel) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::Panel> for PivotHeaderPanel {
    fn into_param(self) -> ::windows_core::Param<'a, super::Panel> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::Panel> for &PivotHeaderPanel {
    fn into_param(self) -> ::windows_core::Param<'a, super::Panel> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::Panel>::into(self))
    }
}
impl ::core::convert::From<PivotHeaderPanel> for super::super::FrameworkElement {
    fn from(value: PivotHeaderPanel) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&PivotHeaderPanel> for super::super::FrameworkElement {
    fn from(value: &PivotHeaderPanel) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::FrameworkElement> for PivotHeaderPanel {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::FrameworkElement> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::FrameworkElement> for &PivotHeaderPanel {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::FrameworkElement> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::super::FrameworkElement>::into(self))
    }
}
impl ::core::convert::From<PivotHeaderPanel> for super::super::UIElement {
    fn from(value: PivotHeaderPanel) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&PivotHeaderPanel> for super::super::UIElement {
    fn from(value: &PivotHeaderPanel) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::UIElement> for PivotHeaderPanel {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::UIElement> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::UIElement> for &PivotHeaderPanel {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::UIElement> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::super::UIElement>::into(self))
    }
}
impl ::core::convert::From<PivotHeaderPanel> for super::super::DependencyObject {
    fn from(value: PivotHeaderPanel) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&PivotHeaderPanel> for super::super::DependencyObject {
    fn from(value: &PivotHeaderPanel) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::DependencyObject> for PivotHeaderPanel {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::DependencyObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::DependencyObject> for &PivotHeaderPanel {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::DependencyObject> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for PivotHeaderPanel {}
unsafe impl ::core::marker::Sync for PivotHeaderPanel {}
#[repr(transparent)]
pub struct PivotPanel(::windows_core::IUnknown);
impl PivotPanel {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<PivotPanel, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn AreHorizontalSnapPointsRegular(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IScrollSnapPointsInfo>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).AreHorizontalSnapPointsRegular)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn AreVerticalSnapPointsRegular(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IScrollSnapPointsInfo>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).AreVerticalSnapPointsRegular)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn HorizontalSnapPointsChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventHandler<::windows_core::IInspectable>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = &::windows_core::Interface::cast::<IScrollSnapPointsInfo>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).HorizontalSnapPointsChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveHorizontalSnapPointsChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IScrollSnapPointsInfo>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).RemoveHorizontalSnapPointsChanged)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn VerticalSnapPointsChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventHandler<::windows_core::IInspectable>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = &::windows_core::Interface::cast::<IScrollSnapPointsInfo>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).VerticalSnapPointsChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveVerticalSnapPointsChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IScrollSnapPointsInfo>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).RemoveVerticalSnapPointsChanged)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetIrregularSnapPoints(&self, orientation: super::Orientation, alignment: SnapPointsAlignment) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<f32>> {
        let this = &::windows_core::Interface::cast::<IScrollSnapPointsInfo>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetIrregularSnapPoints)(::windows_core::Interface::as_raw(this), orientation, alignment, result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<f32>>(result__)
        }
    }
    pub fn GetRegularSnapPoints(&self, orientation: super::Orientation, alignment: SnapPointsAlignment, offset: &mut f32) -> ::windows_core::Result<f32> {
        let this = &::windows_core::Interface::cast::<IScrollSnapPointsInfo>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f32>::zeroed();
            (::windows_core::Interface::vtable(this).GetRegularSnapPoints)(::windows_core::Interface::as_raw(this), orientation, alignment, offset, result__.as_mut_ptr()).from_abi::<f32>(result__)
        }
    }
}
impl ::core::clone::Clone for PivotPanel {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PivotPanel {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PivotPanel {}
impl ::core::fmt::Debug for PivotPanel {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PivotPanel").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PivotPanel {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Controls.Primitives.PivotPanel;{ad4ebe80-22a9-4ca3-9212-2773b6359ff3})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for PivotPanel {
    type Vtable = IPivotPanel_Vtbl;
    const IID: ::windows_core::GUID = <IPivotPanel as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for PivotPanel {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.PivotPanel";
}
impl ::core::convert::From<PivotPanel> for ::windows_core::IUnknown {
    fn from(value: PivotPanel) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PivotPanel> for ::windows_core::IUnknown {
    fn from(value: &PivotPanel) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for PivotPanel {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a PivotPanel {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PivotPanel> for ::windows_core::IInspectable {
    fn from(value: PivotPanel) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PivotPanel> for ::windows_core::IInspectable {
    fn from(value: &PivotPanel) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for PivotPanel {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a PivotPanel {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<PivotPanel> for super::super::super::Composition::IAnimationObject {
    type Error = ::windows_core::Error;
    fn try_from(value: PivotPanel) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<&PivotPanel> for super::super::super::Composition::IAnimationObject {
    type Error = ::windows_core::Error;
    fn try_from(value: &PivotPanel) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows_core::IntoParam<'a, super::super::super::Composition::IAnimationObject> for PivotPanel {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::super::Composition::IAnimationObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows_core::IntoParam<'a, super::super::super::Composition::IAnimationObject> for &PivotPanel {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::super::Composition::IAnimationObject> {
        ::core::convert::TryInto::<super::super::super::Composition::IAnimationObject>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<PivotPanel> for IScrollSnapPointsInfo {
    type Error = ::windows_core::Error;
    fn try_from(value: PivotPanel) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&PivotPanel> for IScrollSnapPointsInfo {
    type Error = ::windows_core::Error;
    fn try_from(value: &PivotPanel) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IScrollSnapPointsInfo> for PivotPanel {
    fn into_param(self) -> ::windows_core::Param<'a, IScrollSnapPointsInfo> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IScrollSnapPointsInfo> for &PivotPanel {
    fn into_param(self) -> ::windows_core::Param<'a, IScrollSnapPointsInfo> {
        ::core::convert::TryInto::<IScrollSnapPointsInfo>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<PivotPanel> for super::super::super::Composition::IVisualElement {
    type Error = ::windows_core::Error;
    fn try_from(value: PivotPanel) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<&PivotPanel> for super::super::super::Composition::IVisualElement {
    type Error = ::windows_core::Error;
    fn try_from(value: &PivotPanel) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows_core::IntoParam<'a, super::super::super::Composition::IVisualElement> for PivotPanel {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::super::Composition::IVisualElement> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows_core::IntoParam<'a, super::super::super::Composition::IVisualElement> for &PivotPanel {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::super::Composition::IVisualElement> {
        ::core::convert::TryInto::<super::super::super::Composition::IVisualElement>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::From<PivotPanel> for super::Panel {
    fn from(value: PivotPanel) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&PivotPanel> for super::Panel {
    fn from(value: &PivotPanel) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::Panel> for PivotPanel {
    fn into_param(self) -> ::windows_core::Param<'a, super::Panel> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::Panel> for &PivotPanel {
    fn into_param(self) -> ::windows_core::Param<'a, super::Panel> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::Panel>::into(self))
    }
}
impl ::core::convert::From<PivotPanel> for super::super::FrameworkElement {
    fn from(value: PivotPanel) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&PivotPanel> for super::super::FrameworkElement {
    fn from(value: &PivotPanel) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::FrameworkElement> for PivotPanel {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::FrameworkElement> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::FrameworkElement> for &PivotPanel {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::FrameworkElement> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::super::FrameworkElement>::into(self))
    }
}
impl ::core::convert::From<PivotPanel> for super::super::UIElement {
    fn from(value: PivotPanel) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&PivotPanel> for super::super::UIElement {
    fn from(value: &PivotPanel) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::UIElement> for PivotPanel {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::UIElement> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::UIElement> for &PivotPanel {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::UIElement> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::super::UIElement>::into(self))
    }
}
impl ::core::convert::From<PivotPanel> for super::super::DependencyObject {
    fn from(value: PivotPanel) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&PivotPanel> for super::super::DependencyObject {
    fn from(value: &PivotPanel) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::DependencyObject> for PivotPanel {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::DependencyObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::DependencyObject> for &PivotPanel {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::DependencyObject> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for PivotPanel {}
unsafe impl ::core::marker::Sync for PivotPanel {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PlacementMode(pub i32);
impl PlacementMode {
    pub const Bottom: Self = Self(2i32);
    pub const Left: Self = Self(9i32);
    pub const Mouse: Self = Self(7i32);
    pub const Right: Self = Self(4i32);
    pub const Top: Self = Self(10i32);
}
impl ::core::marker::Copy for PlacementMode {}
impl ::core::clone::Clone for PlacementMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PlacementMode {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for PlacementMode {
    type Abi = Self;
}
impl ::core::fmt::Debug for PlacementMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PlacementMode").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PlacementMode {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Controls.Primitives.PlacementMode;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct Popup(::windows_core::IUnknown);
impl Popup {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<Popup, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn Child(&self) -> ::windows_core::Result<super::super::UIElement> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Child)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::UIElement>(result__)
        }
    }
    pub fn SetChild<'a, Param0: ::windows_core::IntoParam<'a, super::super::UIElement>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetChild)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn IsOpen(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsOpen)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetIsOpen(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetIsOpen)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn HorizontalOffset(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).HorizontalOffset)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn SetHorizontalOffset(&self, value: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetHorizontalOffset)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn VerticalOffset(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).VerticalOffset)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn SetVerticalOffset(&self, value: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetVerticalOffset)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "UI_Xaml_Media_Animation"))]
    pub fn ChildTransitions(&self) -> ::windows_core::Result<super::super::Media::Animation::TransitionCollection> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ChildTransitions)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Media::Animation::TransitionCollection>(result__)
        }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "UI_Xaml_Media_Animation"))]
    pub fn SetChildTransitions<'a, Param0: ::windows_core::IntoParam<'a, super::super::Media::Animation::TransitionCollection>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetChildTransitions)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn IsLightDismissEnabled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsLightDismissEnabled)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetIsLightDismissEnabled(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetIsLightDismissEnabled)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Opened<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventHandler<::windows_core::IInspectable>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).Opened)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveOpened<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveOpened)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn Closed<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventHandler<::windows_core::IInspectable>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).Closed)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveClosed<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveClosed)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn LightDismissOverlayMode(&self) -> ::windows_core::Result<super::LightDismissOverlayMode> {
        let this = &::windows_core::Interface::cast::<IPopup2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::LightDismissOverlayMode>::zeroed();
            (::windows_core::Interface::vtable(this).LightDismissOverlayMode)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::LightDismissOverlayMode>(result__)
        }
    }
    pub fn SetLightDismissOverlayMode(&self, value: super::LightDismissOverlayMode) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IPopup2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetLightDismissOverlayMode)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ShouldConstrainToRootBounds(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IPopup3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).ShouldConstrainToRootBounds)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetShouldConstrainToRootBounds(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IPopup3>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetShouldConstrainToRootBounds)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsConstrainedToRootBounds(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IPopup3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsConstrainedToRootBounds)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn PlacementTarget(&self) -> ::windows_core::Result<super::super::FrameworkElement> {
        let this = &::windows_core::Interface::cast::<IPopup4>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).PlacementTarget)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::FrameworkElement>(result__)
        }
    }
    pub fn SetPlacementTarget<'a, Param0: ::windows_core::IntoParam<'a, super::super::FrameworkElement>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IPopup4>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetPlacementTarget)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn DesiredPlacement(&self) -> ::windows_core::Result<PopupPlacementMode> {
        let this = &::windows_core::Interface::cast::<IPopup4>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<PopupPlacementMode>::zeroed();
            (::windows_core::Interface::vtable(this).DesiredPlacement)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PopupPlacementMode>(result__)
        }
    }
    pub fn SetDesiredPlacement(&self, value: PopupPlacementMode) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IPopup4>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetDesiredPlacement)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ActualPlacement(&self) -> ::windows_core::Result<PopupPlacementMode> {
        let this = &::windows_core::Interface::cast::<IPopup4>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<PopupPlacementMode>::zeroed();
            (::windows_core::Interface::vtable(this).ActualPlacement)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PopupPlacementMode>(result__)
        }
    }
    pub fn ActualPlacementChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventHandler<::windows_core::IInspectable>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = &::windows_core::Interface::cast::<IPopup4>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).ActualPlacementChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveActualPlacementChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IPopup4>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).RemoveActualPlacementChanged)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn ChildProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::IPopupStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ChildProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn IsOpenProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::IPopupStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).IsOpenProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn HorizontalOffsetProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::IPopupStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).HorizontalOffsetProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn VerticalOffsetProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::IPopupStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).VerticalOffsetProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn ChildTransitionsProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::IPopupStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ChildTransitionsProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn IsLightDismissEnabledProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::IPopupStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).IsLightDismissEnabledProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn LightDismissOverlayModeProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::IPopupStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).LightDismissOverlayModeProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn ShouldConstrainToRootBoundsProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::IPopupStatics3(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ShouldConstrainToRootBoundsProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn PlacementTargetProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::IPopupStatics4(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).PlacementTargetProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn DesiredPlacementProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::IPopupStatics4(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).DesiredPlacementProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn IPopupStatics<R, F: FnOnce(&IPopupStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<Popup, IPopupStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IPopupStatics2<R, F: FnOnce(&IPopupStatics2) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<Popup, IPopupStatics2> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IPopupStatics3<R, F: FnOnce(&IPopupStatics3) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<Popup, IPopupStatics3> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IPopupStatics4<R, F: FnOnce(&IPopupStatics4) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<Popup, IPopupStatics4> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for Popup {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for Popup {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for Popup {}
impl ::core::fmt::Debug for Popup {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Popup").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for Popup {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Controls.Primitives.Popup;{62418240-e6d3-4705-a1dc-39156456ee29})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for Popup {
    type Vtable = IPopup_Vtbl;
    const IID: ::windows_core::GUID = <IPopup as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for Popup {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.Popup";
}
impl ::core::convert::From<Popup> for ::windows_core::IUnknown {
    fn from(value: Popup) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&Popup> for ::windows_core::IUnknown {
    fn from(value: &Popup) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for Popup {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a Popup {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<Popup> for ::windows_core::IInspectable {
    fn from(value: Popup) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&Popup> for ::windows_core::IInspectable {
    fn from(value: &Popup) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for Popup {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a Popup {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<Popup> for super::super::super::Composition::IAnimationObject {
    type Error = ::windows_core::Error;
    fn try_from(value: Popup) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<&Popup> for super::super::super::Composition::IAnimationObject {
    type Error = ::windows_core::Error;
    fn try_from(value: &Popup) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows_core::IntoParam<'a, super::super::super::Composition::IAnimationObject> for Popup {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::super::Composition::IAnimationObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows_core::IntoParam<'a, super::super::super::Composition::IAnimationObject> for &Popup {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::super::Composition::IAnimationObject> {
        ::core::convert::TryInto::<super::super::super::Composition::IAnimationObject>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<Popup> for super::super::super::Composition::IVisualElement {
    type Error = ::windows_core::Error;
    fn try_from(value: Popup) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<&Popup> for super::super::super::Composition::IVisualElement {
    type Error = ::windows_core::Error;
    fn try_from(value: &Popup) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows_core::IntoParam<'a, super::super::super::Composition::IVisualElement> for Popup {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::super::Composition::IVisualElement> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows_core::IntoParam<'a, super::super::super::Composition::IVisualElement> for &Popup {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::super::Composition::IVisualElement> {
        ::core::convert::TryInto::<super::super::super::Composition::IVisualElement>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::From<Popup> for super::super::FrameworkElement {
    fn from(value: Popup) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&Popup> for super::super::FrameworkElement {
    fn from(value: &Popup) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::FrameworkElement> for Popup {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::FrameworkElement> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::FrameworkElement> for &Popup {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::FrameworkElement> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::super::FrameworkElement>::into(self))
    }
}
impl ::core::convert::From<Popup> for super::super::UIElement {
    fn from(value: Popup) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&Popup> for super::super::UIElement {
    fn from(value: &Popup) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::UIElement> for Popup {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::UIElement> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::UIElement> for &Popup {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::UIElement> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::super::UIElement>::into(self))
    }
}
impl ::core::convert::From<Popup> for super::super::DependencyObject {
    fn from(value: Popup) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&Popup> for super::super::DependencyObject {
    fn from(value: &Popup) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::DependencyObject> for Popup {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::DependencyObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::DependencyObject> for &Popup {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::DependencyObject> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for Popup {}
unsafe impl ::core::marker::Sync for Popup {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PopupPlacementMode(pub i32);
impl PopupPlacementMode {
    pub const Auto: Self = Self(0i32);
    pub const Top: Self = Self(1i32);
    pub const Bottom: Self = Self(2i32);
    pub const Left: Self = Self(3i32);
    pub const Right: Self = Self(4i32);
    pub const TopEdgeAlignedLeft: Self = Self(5i32);
    pub const TopEdgeAlignedRight: Self = Self(6i32);
    pub const BottomEdgeAlignedLeft: Self = Self(7i32);
    pub const BottomEdgeAlignedRight: Self = Self(8i32);
    pub const LeftEdgeAlignedTop: Self = Self(9i32);
    pub const LeftEdgeAlignedBottom: Self = Self(10i32);
    pub const RightEdgeAlignedTop: Self = Self(11i32);
    pub const RightEdgeAlignedBottom: Self = Self(12i32);
}
impl ::core::marker::Copy for PopupPlacementMode {}
impl ::core::clone::Clone for PopupPlacementMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PopupPlacementMode {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for PopupPlacementMode {
    type Abi = Self;
}
impl ::core::fmt::Debug for PopupPlacementMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PopupPlacementMode").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PopupPlacementMode {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Controls.Primitives.PopupPlacementMode;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct ProgressBarTemplateSettings(::windows_core::IUnknown);
impl ProgressBarTemplateSettings {
    pub fn EllipseDiameter(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).EllipseDiameter)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn EllipseOffset(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).EllipseOffset)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn EllipseAnimationWellPosition(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).EllipseAnimationWellPosition)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn EllipseAnimationEndPosition(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).EllipseAnimationEndPosition)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn ContainerAnimationStartPosition(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).ContainerAnimationStartPosition)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn ContainerAnimationEndPosition(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).ContainerAnimationEndPosition)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn IndicatorLengthDelta(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).IndicatorLengthDelta)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
}
impl ::core::clone::Clone for ProgressBarTemplateSettings {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ProgressBarTemplateSettings {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ProgressBarTemplateSettings {}
impl ::core::fmt::Debug for ProgressBarTemplateSettings {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ProgressBarTemplateSettings").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ProgressBarTemplateSettings {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Controls.Primitives.ProgressBarTemplateSettings;{3fe2ea2a-e3f2-4c2b-9488-918d77d2bbe4})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for ProgressBarTemplateSettings {
    type Vtable = IProgressBarTemplateSettings_Vtbl;
    const IID: ::windows_core::GUID = <IProgressBarTemplateSettings as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for ProgressBarTemplateSettings {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.ProgressBarTemplateSettings";
}
impl ::core::convert::From<ProgressBarTemplateSettings> for ::windows_core::IUnknown {
    fn from(value: ProgressBarTemplateSettings) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ProgressBarTemplateSettings> for ::windows_core::IUnknown {
    fn from(value: &ProgressBarTemplateSettings) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ProgressBarTemplateSettings {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ProgressBarTemplateSettings {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ProgressBarTemplateSettings> for ::windows_core::IInspectable {
    fn from(value: ProgressBarTemplateSettings) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ProgressBarTemplateSettings> for ::windows_core::IInspectable {
    fn from(value: &ProgressBarTemplateSettings) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for ProgressBarTemplateSettings {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a ProgressBarTemplateSettings {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ProgressBarTemplateSettings> for super::super::DependencyObject {
    fn from(value: ProgressBarTemplateSettings) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&ProgressBarTemplateSettings> for super::super::DependencyObject {
    fn from(value: &ProgressBarTemplateSettings) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::DependencyObject> for ProgressBarTemplateSettings {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::DependencyObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::DependencyObject> for &ProgressBarTemplateSettings {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::DependencyObject> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for ProgressBarTemplateSettings {}
unsafe impl ::core::marker::Sync for ProgressBarTemplateSettings {}
#[repr(transparent)]
pub struct ProgressRingTemplateSettings(::windows_core::IUnknown);
impl ProgressRingTemplateSettings {
    pub fn EllipseDiameter(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).EllipseDiameter)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn EllipseOffset(&self) -> ::windows_core::Result<super::super::Thickness> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::Thickness>::zeroed();
            (::windows_core::Interface::vtable(this).EllipseOffset)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Thickness>(result__)
        }
    }
    pub fn MaxSideLength(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).MaxSideLength)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
}
impl ::core::clone::Clone for ProgressRingTemplateSettings {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ProgressRingTemplateSettings {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ProgressRingTemplateSettings {}
impl ::core::fmt::Debug for ProgressRingTemplateSettings {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ProgressRingTemplateSettings").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ProgressRingTemplateSettings {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Controls.Primitives.ProgressRingTemplateSettings;{b9b675ec-c723-42e6-83e9-9826272bdc0e})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for ProgressRingTemplateSettings {
    type Vtable = IProgressRingTemplateSettings_Vtbl;
    const IID: ::windows_core::GUID = <IProgressRingTemplateSettings as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for ProgressRingTemplateSettings {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.ProgressRingTemplateSettings";
}
impl ::core::convert::From<ProgressRingTemplateSettings> for ::windows_core::IUnknown {
    fn from(value: ProgressRingTemplateSettings) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ProgressRingTemplateSettings> for ::windows_core::IUnknown {
    fn from(value: &ProgressRingTemplateSettings) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ProgressRingTemplateSettings {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ProgressRingTemplateSettings {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ProgressRingTemplateSettings> for ::windows_core::IInspectable {
    fn from(value: ProgressRingTemplateSettings) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ProgressRingTemplateSettings> for ::windows_core::IInspectable {
    fn from(value: &ProgressRingTemplateSettings) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for ProgressRingTemplateSettings {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a ProgressRingTemplateSettings {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ProgressRingTemplateSettings> for super::super::DependencyObject {
    fn from(value: ProgressRingTemplateSettings) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&ProgressRingTemplateSettings> for super::super::DependencyObject {
    fn from(value: &ProgressRingTemplateSettings) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::DependencyObject> for ProgressRingTemplateSettings {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::DependencyObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::DependencyObject> for &ProgressRingTemplateSettings {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::DependencyObject> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for ProgressRingTemplateSettings {}
unsafe impl ::core::marker::Sync for ProgressRingTemplateSettings {}
#[repr(transparent)]
pub struct RangeBase(::windows_core::IUnknown);
impl RangeBase {
    pub fn Minimum(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).Minimum)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn SetMinimum(&self, value: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetMinimum)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Maximum(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).Maximum)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn SetMaximum(&self, value: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetMaximum)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn SmallChange(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).SmallChange)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn SetSmallChange(&self, value: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetSmallChange)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn LargeChange(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).LargeChange)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn SetLargeChange(&self, value: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetLargeChange)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Value(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).Value)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn SetValue(&self, value: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetValue)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ValueChanged<'a, Param0: ::windows_core::IntoParam<'a, RangeBaseValueChangedEventHandler>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).ValueChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveValueChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveValueChanged)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn MinimumProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::IRangeBaseStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).MinimumProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn MaximumProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::IRangeBaseStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).MaximumProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn SmallChangeProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::IRangeBaseStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SmallChangeProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn LargeChangeProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::IRangeBaseStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).LargeChangeProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn ValueProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::IRangeBaseStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ValueProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn IRangeBaseStatics<R, F: FnOnce(&IRangeBaseStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<RangeBase, IRangeBaseStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for RangeBase {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for RangeBase {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for RangeBase {}
impl ::core::fmt::Debug for RangeBase {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RangeBase").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for RangeBase {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Controls.Primitives.RangeBase;{fa002c1a-494e-46cf-91d4-e14a8d798675})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for RangeBase {
    type Vtable = IRangeBase_Vtbl;
    const IID: ::windows_core::GUID = <IRangeBase as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for RangeBase {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.RangeBase";
}
impl ::core::convert::From<RangeBase> for ::windows_core::IUnknown {
    fn from(value: RangeBase) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RangeBase> for ::windows_core::IUnknown {
    fn from(value: &RangeBase) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for RangeBase {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a RangeBase {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<RangeBase> for ::windows_core::IInspectable {
    fn from(value: RangeBase) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RangeBase> for ::windows_core::IInspectable {
    fn from(value: &RangeBase) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for RangeBase {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a RangeBase {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<RangeBase> for super::super::super::Composition::IAnimationObject {
    type Error = ::windows_core::Error;
    fn try_from(value: RangeBase) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<&RangeBase> for super::super::super::Composition::IAnimationObject {
    type Error = ::windows_core::Error;
    fn try_from(value: &RangeBase) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows_core::IntoParam<'a, super::super::super::Composition::IAnimationObject> for RangeBase {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::super::Composition::IAnimationObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows_core::IntoParam<'a, super::super::super::Composition::IAnimationObject> for &RangeBase {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::super::Composition::IAnimationObject> {
        ::core::convert::TryInto::<super::super::super::Composition::IAnimationObject>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<RangeBase> for super::super::super::Composition::IVisualElement {
    type Error = ::windows_core::Error;
    fn try_from(value: RangeBase) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<&RangeBase> for super::super::super::Composition::IVisualElement {
    type Error = ::windows_core::Error;
    fn try_from(value: &RangeBase) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows_core::IntoParam<'a, super::super::super::Composition::IVisualElement> for RangeBase {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::super::Composition::IVisualElement> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows_core::IntoParam<'a, super::super::super::Composition::IVisualElement> for &RangeBase {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::super::Composition::IVisualElement> {
        ::core::convert::TryInto::<super::super::super::Composition::IVisualElement>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::From<RangeBase> for super::Control {
    fn from(value: RangeBase) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&RangeBase> for super::Control {
    fn from(value: &RangeBase) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::Control> for RangeBase {
    fn into_param(self) -> ::windows_core::Param<'a, super::Control> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::Control> for &RangeBase {
    fn into_param(self) -> ::windows_core::Param<'a, super::Control> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::Control>::into(self))
    }
}
impl ::core::convert::From<RangeBase> for super::super::FrameworkElement {
    fn from(value: RangeBase) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&RangeBase> for super::super::FrameworkElement {
    fn from(value: &RangeBase) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::FrameworkElement> for RangeBase {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::FrameworkElement> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::FrameworkElement> for &RangeBase {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::FrameworkElement> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::super::FrameworkElement>::into(self))
    }
}
impl ::core::convert::From<RangeBase> for super::super::UIElement {
    fn from(value: RangeBase) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&RangeBase> for super::super::UIElement {
    fn from(value: &RangeBase) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::UIElement> for RangeBase {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::UIElement> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::UIElement> for &RangeBase {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::UIElement> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::super::UIElement>::into(self))
    }
}
impl ::core::convert::From<RangeBase> for super::super::DependencyObject {
    fn from(value: RangeBase) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&RangeBase> for super::super::DependencyObject {
    fn from(value: &RangeBase) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::DependencyObject> for RangeBase {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::DependencyObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::DependencyObject> for &RangeBase {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::DependencyObject> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for RangeBase {}
unsafe impl ::core::marker::Sync for RangeBase {}
#[repr(transparent)]
pub struct RangeBaseValueChangedEventArgs(::windows_core::IUnknown);
impl RangeBaseValueChangedEventArgs {
    pub fn OldValue(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).OldValue)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn NewValue(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).NewValue)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
}
impl ::core::clone::Clone for RangeBaseValueChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for RangeBaseValueChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for RangeBaseValueChangedEventArgs {}
impl ::core::fmt::Debug for RangeBaseValueChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RangeBaseValueChangedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for RangeBaseValueChangedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Controls.Primitives.RangeBaseValueChangedEventArgs;{a1921777-d5c1-4f9c-a7b0-0401b7e6dc5c})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for RangeBaseValueChangedEventArgs {
    type Vtable = IRangeBaseValueChangedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IRangeBaseValueChangedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for RangeBaseValueChangedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.RangeBaseValueChangedEventArgs";
}
impl ::core::convert::From<RangeBaseValueChangedEventArgs> for ::windows_core::IUnknown {
    fn from(value: RangeBaseValueChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RangeBaseValueChangedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &RangeBaseValueChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for RangeBaseValueChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a RangeBaseValueChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<RangeBaseValueChangedEventArgs> for ::windows_core::IInspectable {
    fn from(value: RangeBaseValueChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RangeBaseValueChangedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &RangeBaseValueChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for RangeBaseValueChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a RangeBaseValueChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<RangeBaseValueChangedEventArgs> for super::super::RoutedEventArgs {
    fn from(value: RangeBaseValueChangedEventArgs) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&RangeBaseValueChangedEventArgs> for super::super::RoutedEventArgs {
    fn from(value: &RangeBaseValueChangedEventArgs) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::RoutedEventArgs> for RangeBaseValueChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::RoutedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::RoutedEventArgs> for &RangeBaseValueChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::RoutedEventArgs> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::super::RoutedEventArgs>::into(self))
    }
}
unsafe impl ::core::marker::Send for RangeBaseValueChangedEventArgs {}
unsafe impl ::core::marker::Sync for RangeBaseValueChangedEventArgs {}
#[repr(transparent)]
pub struct RangeBaseValueChangedEventHandler(pub ::windows_core::IUnknown);
impl RangeBaseValueChangedEventHandler {
    pub fn new<F: FnMut(&::core::option::Option<::windows_core::IInspectable>, &::core::option::Option<RangeBaseValueChangedEventArgs>) -> ::windows_core::Result<()> + ::core::marker::Send + 'static>(invoke: F) -> Self {
        let com = RangeBaseValueChangedEventHandlerBox::<F> { vtable: &RangeBaseValueChangedEventHandlerBox::<F>::VTABLE, count: ::windows_core::RefCount::new(1), invoke };
        unsafe { ::core::mem::transmute(::windows_core::alloc::boxed::Box::new(com)) }
    }
    pub fn Invoke<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::IInspectable>, Param1: ::windows_core::IntoParam<'a, RangeBaseValueChangedEventArgs>>(&self, sender: Param0, e: Param1) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Invoke)(::windows_core::Interface::as_raw(this), sender.into_param().abi(), e.into_param().abi()).ok() }
    }
}
#[repr(C)]
struct RangeBaseValueChangedEventHandlerBox<F: FnMut(&::core::option::Option<::windows_core::IInspectable>, &::core::option::Option<RangeBaseValueChangedEventArgs>) -> ::windows_core::Result<()> + ::core::marker::Send + 'static> {
    vtable: *const RangeBaseValueChangedEventHandler_Vtbl,
    invoke: F,
    count: ::windows_core::RefCount,
}
impl<F: FnMut(&::core::option::Option<::windows_core::IInspectable>, &::core::option::Option<RangeBaseValueChangedEventArgs>) -> ::windows_core::Result<()> + ::core::marker::Send + 'static> RangeBaseValueChangedEventHandlerBox<F> {
    const VTABLE: RangeBaseValueChangedEventHandler_Vtbl = RangeBaseValueChangedEventHandler_Vtbl {
        base__: ::windows_core::IUnknownVtbl { QueryInterface: Self::QueryInterface, AddRef: Self::AddRef, Release: Self::Release },
        Invoke: Self::Invoke,
    };
    unsafe extern "system" fn QueryInterface(this: ::windows_core::RawPtr, iid: &::windows_core::GUID, interface: *mut *const ::core::ffi::c_void) -> ::windows_core::HRESULT {
        let this = this as *mut ::windows_core::RawPtr as *mut Self;
        *interface = if iid == &<RangeBaseValueChangedEventHandler as ::windows_core::Interface>::IID || iid == &<::windows_core::IUnknown as ::windows_core::Interface>::IID || iid == &<::windows_core::IAgileObject as ::windows_core::Interface>::IID { &mut (*this).vtable as *mut _ as _ } else { ::core::ptr::null_mut() };
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
    unsafe extern "system" fn Invoke(this: *mut ::core::ffi::c_void, sender: *mut ::core::ffi::c_void, e: ::windows_core::RawPtr) -> ::windows_core::HRESULT {
        let this = this as *mut ::windows_core::RawPtr as *mut Self;
        ((*this).invoke)(::core::mem::transmute(&sender), ::core::mem::transmute(&e)).into()
    }
}
impl ::core::clone::Clone for RangeBaseValueChangedEventHandler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for RangeBaseValueChangedEventHandler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for RangeBaseValueChangedEventHandler {}
impl ::core::fmt::Debug for RangeBaseValueChangedEventHandler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RangeBaseValueChangedEventHandler").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for RangeBaseValueChangedEventHandler {
    type Vtable = RangeBaseValueChangedEventHandler_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe3906fd9_4d1b_4ac8_a43c_c3b908742799);
}
unsafe impl ::windows_core::RuntimeType for RangeBaseValueChangedEventHandler {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"{e3906fd9-4d1b-4ac8-a43c-c3b908742799}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct RangeBaseValueChangedEventHandler_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub Invoke: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sender: *mut ::core::ffi::c_void, e: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct RepeatButton(::windows_core::IUnknown);
impl RepeatButton {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<RepeatButton, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn Delay(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows_core::Interface::vtable(this).Delay)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    pub fn SetDelay(&self, value: i32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetDelay)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Interval(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows_core::Interface::vtable(this).Interval)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    pub fn SetInterval(&self, value: i32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetInterval)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn DelayProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::IRepeatButtonStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).DelayProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn IntervalProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::IRepeatButtonStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).IntervalProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn IRepeatButtonStatics<R, F: FnOnce(&IRepeatButtonStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<RepeatButton, IRepeatButtonStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for RepeatButton {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for RepeatButton {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for RepeatButton {}
impl ::core::fmt::Debug for RepeatButton {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RepeatButton").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for RepeatButton {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Controls.Primitives.RepeatButton;{02200df9-021a-484a-a93b-0f31020314e5})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for RepeatButton {
    type Vtable = IRepeatButton_Vtbl;
    const IID: ::windows_core::GUID = <IRepeatButton as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for RepeatButton {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.RepeatButton";
}
impl ::core::convert::From<RepeatButton> for ::windows_core::IUnknown {
    fn from(value: RepeatButton) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RepeatButton> for ::windows_core::IUnknown {
    fn from(value: &RepeatButton) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for RepeatButton {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a RepeatButton {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<RepeatButton> for ::windows_core::IInspectable {
    fn from(value: RepeatButton) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RepeatButton> for ::windows_core::IInspectable {
    fn from(value: &RepeatButton) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for RepeatButton {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a RepeatButton {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<RepeatButton> for super::super::super::Composition::IAnimationObject {
    type Error = ::windows_core::Error;
    fn try_from(value: RepeatButton) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<&RepeatButton> for super::super::super::Composition::IAnimationObject {
    type Error = ::windows_core::Error;
    fn try_from(value: &RepeatButton) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows_core::IntoParam<'a, super::super::super::Composition::IAnimationObject> for RepeatButton {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::super::Composition::IAnimationObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows_core::IntoParam<'a, super::super::super::Composition::IAnimationObject> for &RepeatButton {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::super::Composition::IAnimationObject> {
        ::core::convert::TryInto::<super::super::super::Composition::IAnimationObject>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<RepeatButton> for super::super::super::Composition::IVisualElement {
    type Error = ::windows_core::Error;
    fn try_from(value: RepeatButton) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<&RepeatButton> for super::super::super::Composition::IVisualElement {
    type Error = ::windows_core::Error;
    fn try_from(value: &RepeatButton) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows_core::IntoParam<'a, super::super::super::Composition::IVisualElement> for RepeatButton {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::super::Composition::IVisualElement> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows_core::IntoParam<'a, super::super::super::Composition::IVisualElement> for &RepeatButton {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::super::Composition::IVisualElement> {
        ::core::convert::TryInto::<super::super::super::Composition::IVisualElement>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::From<RepeatButton> for ButtonBase {
    fn from(value: RepeatButton) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&RepeatButton> for ButtonBase {
    fn from(value: &RepeatButton) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, ButtonBase> for RepeatButton {
    fn into_param(self) -> ::windows_core::Param<'a, ButtonBase> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ButtonBase> for &RepeatButton {
    fn into_param(self) -> ::windows_core::Param<'a, ButtonBase> {
        ::windows_core::Param::Owned(::core::convert::Into::<ButtonBase>::into(self))
    }
}
impl ::core::convert::From<RepeatButton> for super::ContentControl {
    fn from(value: RepeatButton) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&RepeatButton> for super::ContentControl {
    fn from(value: &RepeatButton) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::ContentControl> for RepeatButton {
    fn into_param(self) -> ::windows_core::Param<'a, super::ContentControl> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::ContentControl> for &RepeatButton {
    fn into_param(self) -> ::windows_core::Param<'a, super::ContentControl> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::ContentControl>::into(self))
    }
}
impl ::core::convert::From<RepeatButton> for super::Control {
    fn from(value: RepeatButton) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&RepeatButton> for super::Control {
    fn from(value: &RepeatButton) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::Control> for RepeatButton {
    fn into_param(self) -> ::windows_core::Param<'a, super::Control> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::Control> for &RepeatButton {
    fn into_param(self) -> ::windows_core::Param<'a, super::Control> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::Control>::into(self))
    }
}
impl ::core::convert::From<RepeatButton> for super::super::FrameworkElement {
    fn from(value: RepeatButton) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&RepeatButton> for super::super::FrameworkElement {
    fn from(value: &RepeatButton) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::FrameworkElement> for RepeatButton {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::FrameworkElement> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::FrameworkElement> for &RepeatButton {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::FrameworkElement> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::super::FrameworkElement>::into(self))
    }
}
impl ::core::convert::From<RepeatButton> for super::super::UIElement {
    fn from(value: RepeatButton) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&RepeatButton> for super::super::UIElement {
    fn from(value: &RepeatButton) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::UIElement> for RepeatButton {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::UIElement> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::UIElement> for &RepeatButton {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::UIElement> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::super::UIElement>::into(self))
    }
}
impl ::core::convert::From<RepeatButton> for super::super::DependencyObject {
    fn from(value: RepeatButton) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&RepeatButton> for super::super::DependencyObject {
    fn from(value: &RepeatButton) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::DependencyObject> for RepeatButton {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::DependencyObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::DependencyObject> for &RepeatButton {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::DependencyObject> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for RepeatButton {}
unsafe impl ::core::marker::Sync for RepeatButton {}
#[repr(transparent)]
pub struct ScrollBar(::windows_core::IUnknown);
impl ScrollBar {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<ScrollBar, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn Orientation(&self) -> ::windows_core::Result<super::Orientation> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::Orientation>::zeroed();
            (::windows_core::Interface::vtable(this).Orientation)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::Orientation>(result__)
        }
    }
    pub fn SetOrientation(&self, value: super::Orientation) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetOrientation)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ViewportSize(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).ViewportSize)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn SetViewportSize(&self, value: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetViewportSize)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IndicatorMode(&self) -> ::windows_core::Result<ScrollingIndicatorMode> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ScrollingIndicatorMode>::zeroed();
            (::windows_core::Interface::vtable(this).IndicatorMode)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ScrollingIndicatorMode>(result__)
        }
    }
    pub fn SetIndicatorMode(&self, value: ScrollingIndicatorMode) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetIndicatorMode)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Scroll<'a, Param0: ::windows_core::IntoParam<'a, ScrollEventHandler>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).Scroll)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveScroll<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveScroll)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn OrientationProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::IScrollBarStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).OrientationProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn ViewportSizeProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::IScrollBarStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ViewportSizeProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn IndicatorModeProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::IScrollBarStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).IndicatorModeProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn IScrollBarStatics<R, F: FnOnce(&IScrollBarStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<ScrollBar, IScrollBarStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for ScrollBar {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ScrollBar {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ScrollBar {}
impl ::core::fmt::Debug for ScrollBar {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ScrollBar").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ScrollBar {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Controls.Primitives.ScrollBar;{f57ae6ca-d1a6-4b90-a4e9-54df1ba8d2ec})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for ScrollBar {
    type Vtable = IScrollBar_Vtbl;
    const IID: ::windows_core::GUID = <IScrollBar as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for ScrollBar {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.ScrollBar";
}
impl ::core::convert::From<ScrollBar> for ::windows_core::IUnknown {
    fn from(value: ScrollBar) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ScrollBar> for ::windows_core::IUnknown {
    fn from(value: &ScrollBar) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ScrollBar {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ScrollBar {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ScrollBar> for ::windows_core::IInspectable {
    fn from(value: ScrollBar) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ScrollBar> for ::windows_core::IInspectable {
    fn from(value: &ScrollBar) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for ScrollBar {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a ScrollBar {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<ScrollBar> for super::super::super::Composition::IAnimationObject {
    type Error = ::windows_core::Error;
    fn try_from(value: ScrollBar) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<&ScrollBar> for super::super::super::Composition::IAnimationObject {
    type Error = ::windows_core::Error;
    fn try_from(value: &ScrollBar) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows_core::IntoParam<'a, super::super::super::Composition::IAnimationObject> for ScrollBar {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::super::Composition::IAnimationObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows_core::IntoParam<'a, super::super::super::Composition::IAnimationObject> for &ScrollBar {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::super::Composition::IAnimationObject> {
        ::core::convert::TryInto::<super::super::super::Composition::IAnimationObject>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<ScrollBar> for super::super::super::Composition::IVisualElement {
    type Error = ::windows_core::Error;
    fn try_from(value: ScrollBar) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<&ScrollBar> for super::super::super::Composition::IVisualElement {
    type Error = ::windows_core::Error;
    fn try_from(value: &ScrollBar) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows_core::IntoParam<'a, super::super::super::Composition::IVisualElement> for ScrollBar {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::super::Composition::IVisualElement> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows_core::IntoParam<'a, super::super::super::Composition::IVisualElement> for &ScrollBar {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::super::Composition::IVisualElement> {
        ::core::convert::TryInto::<super::super::super::Composition::IVisualElement>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::From<ScrollBar> for RangeBase {
    fn from(value: ScrollBar) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&ScrollBar> for RangeBase {
    fn from(value: &ScrollBar) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, RangeBase> for ScrollBar {
    fn into_param(self) -> ::windows_core::Param<'a, RangeBase> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, RangeBase> for &ScrollBar {
    fn into_param(self) -> ::windows_core::Param<'a, RangeBase> {
        ::windows_core::Param::Owned(::core::convert::Into::<RangeBase>::into(self))
    }
}
impl ::core::convert::From<ScrollBar> for super::Control {
    fn from(value: ScrollBar) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&ScrollBar> for super::Control {
    fn from(value: &ScrollBar) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::Control> for ScrollBar {
    fn into_param(self) -> ::windows_core::Param<'a, super::Control> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::Control> for &ScrollBar {
    fn into_param(self) -> ::windows_core::Param<'a, super::Control> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::Control>::into(self))
    }
}
impl ::core::convert::From<ScrollBar> for super::super::FrameworkElement {
    fn from(value: ScrollBar) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&ScrollBar> for super::super::FrameworkElement {
    fn from(value: &ScrollBar) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::FrameworkElement> for ScrollBar {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::FrameworkElement> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::FrameworkElement> for &ScrollBar {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::FrameworkElement> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::super::FrameworkElement>::into(self))
    }
}
impl ::core::convert::From<ScrollBar> for super::super::UIElement {
    fn from(value: ScrollBar) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&ScrollBar> for super::super::UIElement {
    fn from(value: &ScrollBar) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::UIElement> for ScrollBar {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::UIElement> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::UIElement> for &ScrollBar {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::UIElement> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::super::UIElement>::into(self))
    }
}
impl ::core::convert::From<ScrollBar> for super::super::DependencyObject {
    fn from(value: ScrollBar) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&ScrollBar> for super::super::DependencyObject {
    fn from(value: &ScrollBar) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::DependencyObject> for ScrollBar {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::DependencyObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::DependencyObject> for &ScrollBar {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::DependencyObject> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for ScrollBar {}
unsafe impl ::core::marker::Sync for ScrollBar {}
#[repr(transparent)]
pub struct ScrollEventArgs(::windows_core::IUnknown);
impl ScrollEventArgs {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<ScrollEventArgs, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn NewValue(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).NewValue)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn ScrollEventType(&self) -> ::windows_core::Result<ScrollEventType> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ScrollEventType>::zeroed();
            (::windows_core::Interface::vtable(this).ScrollEventType)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ScrollEventType>(result__)
        }
    }
}
impl ::core::clone::Clone for ScrollEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ScrollEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ScrollEventArgs {}
impl ::core::fmt::Debug for ScrollEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ScrollEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ScrollEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Controls.Primitives.ScrollEventArgs;{c57e5168-3afe-448d-b752-2f364c75d743})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for ScrollEventArgs {
    type Vtable = IScrollEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IScrollEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for ScrollEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.ScrollEventArgs";
}
impl ::core::convert::From<ScrollEventArgs> for ::windows_core::IUnknown {
    fn from(value: ScrollEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ScrollEventArgs> for ::windows_core::IUnknown {
    fn from(value: &ScrollEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ScrollEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ScrollEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ScrollEventArgs> for ::windows_core::IInspectable {
    fn from(value: ScrollEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ScrollEventArgs> for ::windows_core::IInspectable {
    fn from(value: &ScrollEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for ScrollEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a ScrollEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ScrollEventArgs> for super::super::RoutedEventArgs {
    fn from(value: ScrollEventArgs) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&ScrollEventArgs> for super::super::RoutedEventArgs {
    fn from(value: &ScrollEventArgs) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::RoutedEventArgs> for ScrollEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::RoutedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::RoutedEventArgs> for &ScrollEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::RoutedEventArgs> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::super::RoutedEventArgs>::into(self))
    }
}
unsafe impl ::core::marker::Send for ScrollEventArgs {}
unsafe impl ::core::marker::Sync for ScrollEventArgs {}
#[repr(transparent)]
pub struct ScrollEventHandler(pub ::windows_core::IUnknown);
impl ScrollEventHandler {
    pub fn new<F: FnMut(&::core::option::Option<::windows_core::IInspectable>, &::core::option::Option<ScrollEventArgs>) -> ::windows_core::Result<()> + ::core::marker::Send + 'static>(invoke: F) -> Self {
        let com = ScrollEventHandlerBox::<F> { vtable: &ScrollEventHandlerBox::<F>::VTABLE, count: ::windows_core::RefCount::new(1), invoke };
        unsafe { ::core::mem::transmute(::windows_core::alloc::boxed::Box::new(com)) }
    }
    pub fn Invoke<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::IInspectable>, Param1: ::windows_core::IntoParam<'a, ScrollEventArgs>>(&self, sender: Param0, e: Param1) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Invoke)(::windows_core::Interface::as_raw(this), sender.into_param().abi(), e.into_param().abi()).ok() }
    }
}
#[repr(C)]
struct ScrollEventHandlerBox<F: FnMut(&::core::option::Option<::windows_core::IInspectable>, &::core::option::Option<ScrollEventArgs>) -> ::windows_core::Result<()> + ::core::marker::Send + 'static> {
    vtable: *const ScrollEventHandler_Vtbl,
    invoke: F,
    count: ::windows_core::RefCount,
}
impl<F: FnMut(&::core::option::Option<::windows_core::IInspectable>, &::core::option::Option<ScrollEventArgs>) -> ::windows_core::Result<()> + ::core::marker::Send + 'static> ScrollEventHandlerBox<F> {
    const VTABLE: ScrollEventHandler_Vtbl = ScrollEventHandler_Vtbl {
        base__: ::windows_core::IUnknownVtbl { QueryInterface: Self::QueryInterface, AddRef: Self::AddRef, Release: Self::Release },
        Invoke: Self::Invoke,
    };
    unsafe extern "system" fn QueryInterface(this: ::windows_core::RawPtr, iid: &::windows_core::GUID, interface: *mut *const ::core::ffi::c_void) -> ::windows_core::HRESULT {
        let this = this as *mut ::windows_core::RawPtr as *mut Self;
        *interface = if iid == &<ScrollEventHandler as ::windows_core::Interface>::IID || iid == &<::windows_core::IUnknown as ::windows_core::Interface>::IID || iid == &<::windows_core::IAgileObject as ::windows_core::Interface>::IID { &mut (*this).vtable as *mut _ as _ } else { ::core::ptr::null_mut() };
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
    unsafe extern "system" fn Invoke(this: *mut ::core::ffi::c_void, sender: *mut ::core::ffi::c_void, e: ::windows_core::RawPtr) -> ::windows_core::HRESULT {
        let this = this as *mut ::windows_core::RawPtr as *mut Self;
        ((*this).invoke)(::core::mem::transmute(&sender), ::core::mem::transmute(&e)).into()
    }
}
impl ::core::clone::Clone for ScrollEventHandler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ScrollEventHandler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ScrollEventHandler {}
impl ::core::fmt::Debug for ScrollEventHandler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ScrollEventHandler").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for ScrollEventHandler {
    type Vtable = ScrollEventHandler_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8860b0a4_a383_4c83_b306_a1c39d7db87f);
}
unsafe impl ::windows_core::RuntimeType for ScrollEventHandler {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"{8860b0a4-a383-4c83-b306-a1c39d7db87f}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ScrollEventHandler_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub Invoke: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sender: *mut ::core::ffi::c_void, e: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct ScrollEventType(pub i32);
impl ScrollEventType {
    pub const SmallDecrement: Self = Self(0i32);
    pub const SmallIncrement: Self = Self(1i32);
    pub const LargeDecrement: Self = Self(2i32);
    pub const LargeIncrement: Self = Self(3i32);
    pub const ThumbPosition: Self = Self(4i32);
    pub const ThumbTrack: Self = Self(5i32);
    pub const First: Self = Self(6i32);
    pub const Last: Self = Self(7i32);
    pub const EndScroll: Self = Self(8i32);
}
impl ::core::marker::Copy for ScrollEventType {}
impl ::core::clone::Clone for ScrollEventType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ScrollEventType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for ScrollEventType {
    type Abi = Self;
}
impl ::core::fmt::Debug for ScrollEventType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ScrollEventType").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ScrollEventType {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Controls.Primitives.ScrollEventType;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct ScrollingIndicatorMode(pub i32);
impl ScrollingIndicatorMode {
    pub const None: Self = Self(0i32);
    pub const TouchIndicator: Self = Self(1i32);
    pub const MouseIndicator: Self = Self(2i32);
}
impl ::core::marker::Copy for ScrollingIndicatorMode {}
impl ::core::clone::Clone for ScrollingIndicatorMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ScrollingIndicatorMode {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for ScrollingIndicatorMode {
    type Abi = Self;
}
impl ::core::fmt::Debug for ScrollingIndicatorMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ScrollingIndicatorMode").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ScrollingIndicatorMode {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Controls.Primitives.ScrollingIndicatorMode;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct Selector(::windows_core::IUnknown);
impl Selector {
    pub fn SelectedIndex(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows_core::Interface::vtable(this).SelectedIndex)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    pub fn SetSelectedIndex(&self, value: i32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetSelectedIndex)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn SelectedItem(&self) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows_core::Interface::vtable(this).SelectedItem)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::IInspectable>(result__)
        }
    }
    pub fn SetSelectedItem<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::IInspectable>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetSelectedItem)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn SelectedValue(&self) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows_core::Interface::vtable(this).SelectedValue)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::IInspectable>(result__)
        }
    }
    pub fn SetSelectedValue<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::IInspectable>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetSelectedValue)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn SelectedValuePath(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).SelectedValuePath)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetSelectedValuePath<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetSelectedValuePath)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn IsSynchronizedWithCurrentItem(&self) -> ::windows_core::Result<::winrt_foundation::IReference<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).IsSynchronizedWithCurrentItem)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IReference<bool>>(result__)
        }
    }
    pub fn SetIsSynchronizedWithCurrentItem<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::IReference<bool>>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetIsSynchronizedWithCurrentItem)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn SelectionChanged<'a, Param0: ::windows_core::IntoParam<'a, super::SelectionChangedEventHandler>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).SelectionChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveSelectionChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveSelectionChanged)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn SelectedIndexProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::ISelectorStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SelectedIndexProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn SelectedItemProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::ISelectorStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SelectedItemProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn SelectedValueProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::ISelectorStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SelectedValueProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn SelectedValuePathProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::ISelectorStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SelectedValuePathProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn IsSynchronizedWithCurrentItemProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::ISelectorStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).IsSynchronizedWithCurrentItemProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn GetIsSelectionActive<'a, Param0: ::windows_core::IntoParam<'a, super::super::DependencyObject>>(element: Param0) -> ::windows_core::Result<bool> {
        Self::ISelectorStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).GetIsSelectionActive)(::windows_core::Interface::as_raw(this), element.into_param().abi(), result__.as_mut_ptr()).from_abi::<bool>(result__)
        })
    }
    pub fn ISelectorStatics<R, F: FnOnce(&ISelectorStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<Selector, ISelectorStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for Selector {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for Selector {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for Selector {}
impl ::core::fmt::Debug for Selector {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Selector").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for Selector {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Controls.Primitives.Selector;{e30eb3a5-b36b-42dc-8527-cd25136c083c})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for Selector {
    type Vtable = ISelector_Vtbl;
    const IID: ::windows_core::GUID = <ISelector as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for Selector {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.Selector";
}
impl ::core::convert::From<Selector> for ::windows_core::IUnknown {
    fn from(value: Selector) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&Selector> for ::windows_core::IUnknown {
    fn from(value: &Selector) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for Selector {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a Selector {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<Selector> for ::windows_core::IInspectable {
    fn from(value: Selector) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&Selector> for ::windows_core::IInspectable {
    fn from(value: &Selector) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for Selector {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a Selector {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<Selector> for super::super::super::Composition::IAnimationObject {
    type Error = ::windows_core::Error;
    fn try_from(value: Selector) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<&Selector> for super::super::super::Composition::IAnimationObject {
    type Error = ::windows_core::Error;
    fn try_from(value: &Selector) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows_core::IntoParam<'a, super::super::super::Composition::IAnimationObject> for Selector {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::super::Composition::IAnimationObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows_core::IntoParam<'a, super::super::super::Composition::IAnimationObject> for &Selector {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::super::Composition::IAnimationObject> {
        ::core::convert::TryInto::<super::super::super::Composition::IAnimationObject>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<Selector> for super::IItemContainerMapping {
    type Error = ::windows_core::Error;
    fn try_from(value: Selector) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&Selector> for super::IItemContainerMapping {
    type Error = ::windows_core::Error;
    fn try_from(value: &Selector) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::IItemContainerMapping> for Selector {
    fn into_param(self) -> ::windows_core::Param<'a, super::IItemContainerMapping> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::IItemContainerMapping> for &Selector {
    fn into_param(self) -> ::windows_core::Param<'a, super::IItemContainerMapping> {
        ::core::convert::TryInto::<super::IItemContainerMapping>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<Selector> for super::super::super::Composition::IVisualElement {
    type Error = ::windows_core::Error;
    fn try_from(value: Selector) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<&Selector> for super::super::super::Composition::IVisualElement {
    type Error = ::windows_core::Error;
    fn try_from(value: &Selector) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows_core::IntoParam<'a, super::super::super::Composition::IVisualElement> for Selector {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::super::Composition::IVisualElement> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows_core::IntoParam<'a, super::super::super::Composition::IVisualElement> for &Selector {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::super::Composition::IVisualElement> {
        ::core::convert::TryInto::<super::super::super::Composition::IVisualElement>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::From<Selector> for super::ItemsControl {
    fn from(value: Selector) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&Selector> for super::ItemsControl {
    fn from(value: &Selector) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::ItemsControl> for Selector {
    fn into_param(self) -> ::windows_core::Param<'a, super::ItemsControl> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::ItemsControl> for &Selector {
    fn into_param(self) -> ::windows_core::Param<'a, super::ItemsControl> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::ItemsControl>::into(self))
    }
}
impl ::core::convert::From<Selector> for super::Control {
    fn from(value: Selector) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&Selector> for super::Control {
    fn from(value: &Selector) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::Control> for Selector {
    fn into_param(self) -> ::windows_core::Param<'a, super::Control> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::Control> for &Selector {
    fn into_param(self) -> ::windows_core::Param<'a, super::Control> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::Control>::into(self))
    }
}
impl ::core::convert::From<Selector> for super::super::FrameworkElement {
    fn from(value: Selector) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&Selector> for super::super::FrameworkElement {
    fn from(value: &Selector) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::FrameworkElement> for Selector {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::FrameworkElement> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::FrameworkElement> for &Selector {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::FrameworkElement> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::super::FrameworkElement>::into(self))
    }
}
impl ::core::convert::From<Selector> for super::super::UIElement {
    fn from(value: Selector) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&Selector> for super::super::UIElement {
    fn from(value: &Selector) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::UIElement> for Selector {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::UIElement> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::UIElement> for &Selector {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::UIElement> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::super::UIElement>::into(self))
    }
}
impl ::core::convert::From<Selector> for super::super::DependencyObject {
    fn from(value: Selector) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&Selector> for super::super::DependencyObject {
    fn from(value: &Selector) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::DependencyObject> for Selector {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::DependencyObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::DependencyObject> for &Selector {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::DependencyObject> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for Selector {}
unsafe impl ::core::marker::Sync for Selector {}
#[repr(transparent)]
pub struct SelectorItem(::windows_core::IUnknown);
impl SelectorItem {
    pub fn IsSelected(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsSelected)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetIsSelected(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetIsSelected)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsSelectedProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::ISelectorItemStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).IsSelectedProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn ISelectorItemStatics<R, F: FnOnce(&ISelectorItemStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<SelectorItem, ISelectorItemStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for SelectorItem {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SelectorItem {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SelectorItem {}
impl ::core::fmt::Debug for SelectorItem {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SelectorItem").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SelectorItem {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Controls.Primitives.SelectorItem;{541c8d6c-0283-4581-b945-2a64c28a0646})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for SelectorItem {
    type Vtable = ISelectorItem_Vtbl;
    const IID: ::windows_core::GUID = <ISelectorItem as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for SelectorItem {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.SelectorItem";
}
impl ::core::convert::From<SelectorItem> for ::windows_core::IUnknown {
    fn from(value: SelectorItem) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SelectorItem> for ::windows_core::IUnknown {
    fn from(value: &SelectorItem) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for SelectorItem {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a SelectorItem {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SelectorItem> for ::windows_core::IInspectable {
    fn from(value: SelectorItem) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SelectorItem> for ::windows_core::IInspectable {
    fn from(value: &SelectorItem) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for SelectorItem {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a SelectorItem {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<SelectorItem> for super::super::super::Composition::IAnimationObject {
    type Error = ::windows_core::Error;
    fn try_from(value: SelectorItem) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<&SelectorItem> for super::super::super::Composition::IAnimationObject {
    type Error = ::windows_core::Error;
    fn try_from(value: &SelectorItem) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows_core::IntoParam<'a, super::super::super::Composition::IAnimationObject> for SelectorItem {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::super::Composition::IAnimationObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows_core::IntoParam<'a, super::super::super::Composition::IAnimationObject> for &SelectorItem {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::super::Composition::IAnimationObject> {
        ::core::convert::TryInto::<super::super::super::Composition::IAnimationObject>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<SelectorItem> for super::super::super::Composition::IVisualElement {
    type Error = ::windows_core::Error;
    fn try_from(value: SelectorItem) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<&SelectorItem> for super::super::super::Composition::IVisualElement {
    type Error = ::windows_core::Error;
    fn try_from(value: &SelectorItem) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows_core::IntoParam<'a, super::super::super::Composition::IVisualElement> for SelectorItem {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::super::Composition::IVisualElement> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows_core::IntoParam<'a, super::super::super::Composition::IVisualElement> for &SelectorItem {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::super::Composition::IVisualElement> {
        ::core::convert::TryInto::<super::super::super::Composition::IVisualElement>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::From<SelectorItem> for super::ContentControl {
    fn from(value: SelectorItem) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&SelectorItem> for super::ContentControl {
    fn from(value: &SelectorItem) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::ContentControl> for SelectorItem {
    fn into_param(self) -> ::windows_core::Param<'a, super::ContentControl> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::ContentControl> for &SelectorItem {
    fn into_param(self) -> ::windows_core::Param<'a, super::ContentControl> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::ContentControl>::into(self))
    }
}
impl ::core::convert::From<SelectorItem> for super::Control {
    fn from(value: SelectorItem) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&SelectorItem> for super::Control {
    fn from(value: &SelectorItem) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::Control> for SelectorItem {
    fn into_param(self) -> ::windows_core::Param<'a, super::Control> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::Control> for &SelectorItem {
    fn into_param(self) -> ::windows_core::Param<'a, super::Control> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::Control>::into(self))
    }
}
impl ::core::convert::From<SelectorItem> for super::super::FrameworkElement {
    fn from(value: SelectorItem) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&SelectorItem> for super::super::FrameworkElement {
    fn from(value: &SelectorItem) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::FrameworkElement> for SelectorItem {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::FrameworkElement> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::FrameworkElement> for &SelectorItem {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::FrameworkElement> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::super::FrameworkElement>::into(self))
    }
}
impl ::core::convert::From<SelectorItem> for super::super::UIElement {
    fn from(value: SelectorItem) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&SelectorItem> for super::super::UIElement {
    fn from(value: &SelectorItem) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::UIElement> for SelectorItem {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::UIElement> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::UIElement> for &SelectorItem {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::UIElement> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::super::UIElement>::into(self))
    }
}
impl ::core::convert::From<SelectorItem> for super::super::DependencyObject {
    fn from(value: SelectorItem) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&SelectorItem> for super::super::DependencyObject {
    fn from(value: &SelectorItem) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::DependencyObject> for SelectorItem {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::DependencyObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::DependencyObject> for &SelectorItem {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::DependencyObject> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for SelectorItem {}
unsafe impl ::core::marker::Sync for SelectorItem {}
#[repr(transparent)]
pub struct SettingsFlyoutTemplateSettings(::windows_core::IUnknown);
impl SettingsFlyoutTemplateSettings {
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn HeaderBackground(&self) -> ::windows_core::Result<super::super::Media::Brush> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).HeaderBackground)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Media::Brush>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn HeaderForeground(&self) -> ::windows_core::Result<super::super::Media::Brush> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).HeaderForeground)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Media::Brush>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn BorderBrush(&self) -> ::windows_core::Result<super::super::Media::Brush> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).BorderBrush)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Media::Brush>(result__)
        }
    }
    pub fn BorderThickness(&self) -> ::windows_core::Result<super::super::Thickness> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::Thickness>::zeroed();
            (::windows_core::Interface::vtable(this).BorderThickness)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Thickness>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn IconSource(&self) -> ::windows_core::Result<super::super::Media::ImageSource> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).IconSource)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Media::ImageSource>(result__)
        }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "UI_Xaml_Media_Animation"))]
    pub fn ContentTransitions(&self) -> ::windows_core::Result<super::super::Media::Animation::TransitionCollection> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ContentTransitions)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Media::Animation::TransitionCollection>(result__)
        }
    }
}
impl ::core::clone::Clone for SettingsFlyoutTemplateSettings {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SettingsFlyoutTemplateSettings {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SettingsFlyoutTemplateSettings {}
impl ::core::fmt::Debug for SettingsFlyoutTemplateSettings {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SettingsFlyoutTemplateSettings").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SettingsFlyoutTemplateSettings {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Controls.Primitives.SettingsFlyoutTemplateSettings;{bcf14c10-cea7-43f1-9d68-57605ded69d4})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for SettingsFlyoutTemplateSettings {
    type Vtable = ISettingsFlyoutTemplateSettings_Vtbl;
    const IID: ::windows_core::GUID = <ISettingsFlyoutTemplateSettings as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for SettingsFlyoutTemplateSettings {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.SettingsFlyoutTemplateSettings";
}
impl ::core::convert::From<SettingsFlyoutTemplateSettings> for ::windows_core::IUnknown {
    fn from(value: SettingsFlyoutTemplateSettings) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SettingsFlyoutTemplateSettings> for ::windows_core::IUnknown {
    fn from(value: &SettingsFlyoutTemplateSettings) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for SettingsFlyoutTemplateSettings {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a SettingsFlyoutTemplateSettings {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SettingsFlyoutTemplateSettings> for ::windows_core::IInspectable {
    fn from(value: SettingsFlyoutTemplateSettings) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SettingsFlyoutTemplateSettings> for ::windows_core::IInspectable {
    fn from(value: &SettingsFlyoutTemplateSettings) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for SettingsFlyoutTemplateSettings {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a SettingsFlyoutTemplateSettings {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SettingsFlyoutTemplateSettings> for super::super::DependencyObject {
    fn from(value: SettingsFlyoutTemplateSettings) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&SettingsFlyoutTemplateSettings> for super::super::DependencyObject {
    fn from(value: &SettingsFlyoutTemplateSettings) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::DependencyObject> for SettingsFlyoutTemplateSettings {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::DependencyObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::DependencyObject> for &SettingsFlyoutTemplateSettings {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::DependencyObject> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for SettingsFlyoutTemplateSettings {}
unsafe impl ::core::marker::Sync for SettingsFlyoutTemplateSettings {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct SliderSnapsTo(pub i32);
impl SliderSnapsTo {
    pub const StepValues: Self = Self(0i32);
    pub const Ticks: Self = Self(1i32);
}
impl ::core::marker::Copy for SliderSnapsTo {}
impl ::core::clone::Clone for SliderSnapsTo {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SliderSnapsTo {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for SliderSnapsTo {
    type Abi = Self;
}
impl ::core::fmt::Debug for SliderSnapsTo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SliderSnapsTo").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SliderSnapsTo {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Controls.Primitives.SliderSnapsTo;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct SnapPointsAlignment(pub i32);
impl SnapPointsAlignment {
    pub const Near: Self = Self(0i32);
    pub const Center: Self = Self(1i32);
    pub const Far: Self = Self(2i32);
}
impl ::core::marker::Copy for SnapPointsAlignment {}
impl ::core::clone::Clone for SnapPointsAlignment {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SnapPointsAlignment {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for SnapPointsAlignment {
    type Abi = Self;
}
impl ::core::fmt::Debug for SnapPointsAlignment {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SnapPointsAlignment").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SnapPointsAlignment {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Controls.Primitives.SnapPointsAlignment;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct SplitViewTemplateSettings(::windows_core::IUnknown);
impl SplitViewTemplateSettings {
    pub fn OpenPaneLength(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).OpenPaneLength)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn NegativeOpenPaneLength(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).NegativeOpenPaneLength)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn OpenPaneLengthMinusCompactLength(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).OpenPaneLengthMinusCompactLength)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn NegativeOpenPaneLengthMinusCompactLength(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).NegativeOpenPaneLengthMinusCompactLength)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn OpenPaneGridLength(&self) -> ::windows_core::Result<super::super::GridLength> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::GridLength>::zeroed();
            (::windows_core::Interface::vtable(this).OpenPaneGridLength)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::GridLength>(result__)
        }
    }
    pub fn CompactPaneGridLength(&self) -> ::windows_core::Result<super::super::GridLength> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::GridLength>::zeroed();
            (::windows_core::Interface::vtable(this).CompactPaneGridLength)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::GridLength>(result__)
        }
    }
}
impl ::core::clone::Clone for SplitViewTemplateSettings {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SplitViewTemplateSettings {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SplitViewTemplateSettings {}
impl ::core::fmt::Debug for SplitViewTemplateSettings {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SplitViewTemplateSettings").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SplitViewTemplateSettings {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Controls.Primitives.SplitViewTemplateSettings;{c16ab5a7-4996-4443-b199-6b6b89124eab})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for SplitViewTemplateSettings {
    type Vtable = ISplitViewTemplateSettings_Vtbl;
    const IID: ::windows_core::GUID = <ISplitViewTemplateSettings as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for SplitViewTemplateSettings {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.SplitViewTemplateSettings";
}
impl ::core::convert::From<SplitViewTemplateSettings> for ::windows_core::IUnknown {
    fn from(value: SplitViewTemplateSettings) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SplitViewTemplateSettings> for ::windows_core::IUnknown {
    fn from(value: &SplitViewTemplateSettings) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for SplitViewTemplateSettings {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a SplitViewTemplateSettings {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SplitViewTemplateSettings> for ::windows_core::IInspectable {
    fn from(value: SplitViewTemplateSettings) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SplitViewTemplateSettings> for ::windows_core::IInspectable {
    fn from(value: &SplitViewTemplateSettings) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for SplitViewTemplateSettings {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a SplitViewTemplateSettings {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SplitViewTemplateSettings> for super::super::DependencyObject {
    fn from(value: SplitViewTemplateSettings) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&SplitViewTemplateSettings> for super::super::DependencyObject {
    fn from(value: &SplitViewTemplateSettings) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::DependencyObject> for SplitViewTemplateSettings {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::DependencyObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::DependencyObject> for &SplitViewTemplateSettings {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::DependencyObject> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for SplitViewTemplateSettings {}
unsafe impl ::core::marker::Sync for SplitViewTemplateSettings {}
#[repr(transparent)]
pub struct Thumb(::windows_core::IUnknown);
impl Thumb {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<Thumb, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IsDragging(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsDragging)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn DragStarted<'a, Param0: ::windows_core::IntoParam<'a, DragStartedEventHandler>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).DragStarted)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveDragStarted<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveDragStarted)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn DragDelta<'a, Param0: ::windows_core::IntoParam<'a, DragDeltaEventHandler>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).DragDelta)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveDragDelta<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveDragDelta)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn DragCompleted<'a, Param0: ::windows_core::IntoParam<'a, DragCompletedEventHandler>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).DragCompleted)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveDragCompleted<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveDragCompleted)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn CancelDrag(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).CancelDrag)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn IsDraggingProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::IThumbStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).IsDraggingProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn IThumbStatics<R, F: FnOnce(&IThumbStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<Thumb, IThumbStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for Thumb {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for Thumb {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for Thumb {}
impl ::core::fmt::Debug for Thumb {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Thumb").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for Thumb {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Controls.Primitives.Thumb;{e8b2b281-0d6a-45cf-b333-2402b037f099})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for Thumb {
    type Vtable = IThumb_Vtbl;
    const IID: ::windows_core::GUID = <IThumb as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for Thumb {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.Thumb";
}
impl ::core::convert::From<Thumb> for ::windows_core::IUnknown {
    fn from(value: Thumb) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&Thumb> for ::windows_core::IUnknown {
    fn from(value: &Thumb) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for Thumb {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a Thumb {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<Thumb> for ::windows_core::IInspectable {
    fn from(value: Thumb) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&Thumb> for ::windows_core::IInspectable {
    fn from(value: &Thumb) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for Thumb {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a Thumb {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<Thumb> for super::super::super::Composition::IAnimationObject {
    type Error = ::windows_core::Error;
    fn try_from(value: Thumb) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<&Thumb> for super::super::super::Composition::IAnimationObject {
    type Error = ::windows_core::Error;
    fn try_from(value: &Thumb) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows_core::IntoParam<'a, super::super::super::Composition::IAnimationObject> for Thumb {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::super::Composition::IAnimationObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows_core::IntoParam<'a, super::super::super::Composition::IAnimationObject> for &Thumb {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::super::Composition::IAnimationObject> {
        ::core::convert::TryInto::<super::super::super::Composition::IAnimationObject>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<Thumb> for super::super::super::Composition::IVisualElement {
    type Error = ::windows_core::Error;
    fn try_from(value: Thumb) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<&Thumb> for super::super::super::Composition::IVisualElement {
    type Error = ::windows_core::Error;
    fn try_from(value: &Thumb) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows_core::IntoParam<'a, super::super::super::Composition::IVisualElement> for Thumb {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::super::Composition::IVisualElement> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows_core::IntoParam<'a, super::super::super::Composition::IVisualElement> for &Thumb {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::super::Composition::IVisualElement> {
        ::core::convert::TryInto::<super::super::super::Composition::IVisualElement>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::From<Thumb> for super::Control {
    fn from(value: Thumb) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&Thumb> for super::Control {
    fn from(value: &Thumb) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::Control> for Thumb {
    fn into_param(self) -> ::windows_core::Param<'a, super::Control> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::Control> for &Thumb {
    fn into_param(self) -> ::windows_core::Param<'a, super::Control> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::Control>::into(self))
    }
}
impl ::core::convert::From<Thumb> for super::super::FrameworkElement {
    fn from(value: Thumb) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&Thumb> for super::super::FrameworkElement {
    fn from(value: &Thumb) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::FrameworkElement> for Thumb {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::FrameworkElement> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::FrameworkElement> for &Thumb {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::FrameworkElement> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::super::FrameworkElement>::into(self))
    }
}
impl ::core::convert::From<Thumb> for super::super::UIElement {
    fn from(value: Thumb) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&Thumb> for super::super::UIElement {
    fn from(value: &Thumb) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::UIElement> for Thumb {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::UIElement> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::UIElement> for &Thumb {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::UIElement> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::super::UIElement>::into(self))
    }
}
impl ::core::convert::From<Thumb> for super::super::DependencyObject {
    fn from(value: Thumb) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&Thumb> for super::super::DependencyObject {
    fn from(value: &Thumb) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::DependencyObject> for Thumb {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::DependencyObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::DependencyObject> for &Thumb {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::DependencyObject> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for Thumb {}
unsafe impl ::core::marker::Sync for Thumb {}
#[repr(transparent)]
pub struct TickBar(::windows_core::IUnknown);
impl TickBar {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<TickBar, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn Fill(&self) -> ::windows_core::Result<super::super::Media::Brush> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Fill)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Media::Brush>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetFill<'a, Param0: ::windows_core::IntoParam<'a, super::super::Media::Brush>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetFill)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn FillProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::ITickBarStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FillProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn ITickBarStatics<R, F: FnOnce(&ITickBarStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<TickBar, ITickBarStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for TickBar {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for TickBar {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for TickBar {}
impl ::core::fmt::Debug for TickBar {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TickBar").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for TickBar {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Controls.Primitives.TickBar;{994683fa-f1f6-487d-a5ac-c15921bfa995})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for TickBar {
    type Vtable = ITickBar_Vtbl;
    const IID: ::windows_core::GUID = <ITickBar as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for TickBar {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.TickBar";
}
impl ::core::convert::From<TickBar> for ::windows_core::IUnknown {
    fn from(value: TickBar) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&TickBar> for ::windows_core::IUnknown {
    fn from(value: &TickBar) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for TickBar {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a TickBar {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<TickBar> for ::windows_core::IInspectable {
    fn from(value: TickBar) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&TickBar> for ::windows_core::IInspectable {
    fn from(value: &TickBar) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for TickBar {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a TickBar {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<TickBar> for super::super::super::Composition::IAnimationObject {
    type Error = ::windows_core::Error;
    fn try_from(value: TickBar) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<&TickBar> for super::super::super::Composition::IAnimationObject {
    type Error = ::windows_core::Error;
    fn try_from(value: &TickBar) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows_core::IntoParam<'a, super::super::super::Composition::IAnimationObject> for TickBar {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::super::Composition::IAnimationObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows_core::IntoParam<'a, super::super::super::Composition::IAnimationObject> for &TickBar {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::super::Composition::IAnimationObject> {
        ::core::convert::TryInto::<super::super::super::Composition::IAnimationObject>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<TickBar> for super::super::super::Composition::IVisualElement {
    type Error = ::windows_core::Error;
    fn try_from(value: TickBar) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<&TickBar> for super::super::super::Composition::IVisualElement {
    type Error = ::windows_core::Error;
    fn try_from(value: &TickBar) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows_core::IntoParam<'a, super::super::super::Composition::IVisualElement> for TickBar {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::super::Composition::IVisualElement> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows_core::IntoParam<'a, super::super::super::Composition::IVisualElement> for &TickBar {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::super::Composition::IVisualElement> {
        ::core::convert::TryInto::<super::super::super::Composition::IVisualElement>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::From<TickBar> for super::super::FrameworkElement {
    fn from(value: TickBar) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&TickBar> for super::super::FrameworkElement {
    fn from(value: &TickBar) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::FrameworkElement> for TickBar {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::FrameworkElement> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::FrameworkElement> for &TickBar {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::FrameworkElement> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::super::FrameworkElement>::into(self))
    }
}
impl ::core::convert::From<TickBar> for super::super::UIElement {
    fn from(value: TickBar) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&TickBar> for super::super::UIElement {
    fn from(value: &TickBar) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::UIElement> for TickBar {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::UIElement> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::UIElement> for &TickBar {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::UIElement> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::super::UIElement>::into(self))
    }
}
impl ::core::convert::From<TickBar> for super::super::DependencyObject {
    fn from(value: TickBar) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&TickBar> for super::super::DependencyObject {
    fn from(value: &TickBar) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::DependencyObject> for TickBar {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::DependencyObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::DependencyObject> for &TickBar {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::DependencyObject> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for TickBar {}
unsafe impl ::core::marker::Sync for TickBar {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct TickPlacement(pub i32);
impl TickPlacement {
    pub const None: Self = Self(0i32);
    pub const TopLeft: Self = Self(1i32);
    pub const BottomRight: Self = Self(2i32);
    pub const Outside: Self = Self(3i32);
    pub const Inline: Self = Self(4i32);
}
impl ::core::marker::Copy for TickPlacement {}
impl ::core::clone::Clone for TickPlacement {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TickPlacement {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for TickPlacement {
    type Abi = Self;
}
impl ::core::fmt::Debug for TickPlacement {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TickPlacement").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for TickPlacement {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Controls.Primitives.TickPlacement;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct ToggleButton(::windows_core::IUnknown);
impl ToggleButton {
    pub fn IsChecked(&self) -> ::windows_core::Result<::winrt_foundation::IReference<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).IsChecked)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IReference<bool>>(result__)
        }
    }
    pub fn SetIsChecked<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::IReference<bool>>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetIsChecked)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn IsThreeState(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsThreeState)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetIsThreeState(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetIsThreeState)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Checked<'a, Param0: ::windows_core::IntoParam<'a, super::super::RoutedEventHandler>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).Checked)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveChecked<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveChecked)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn Unchecked<'a, Param0: ::windows_core::IntoParam<'a, super::super::RoutedEventHandler>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).Unchecked)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveUnchecked<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveUnchecked)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn Indeterminate<'a, Param0: ::windows_core::IntoParam<'a, super::super::RoutedEventHandler>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).Indeterminate)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveIndeterminate<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveIndeterminate)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn new() -> ::windows_core::Result<ToggleButton> {
        Self::IToggleButtonFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateInstance)(::windows_core::Interface::as_raw(this), ::core::ptr::null_mut(), &mut ::core::option::Option::<::windows_core::IInspectable>::None as *mut _ as _, result__.as_mut_ptr()).from_abi::<ToggleButton>(result__)
        })
    }
    pub fn compose<T: ::windows_core::Compose>(compose: T) -> ::windows_core::Result<ToggleButton> {
        Self::IToggleButtonFactory(|this| unsafe {
            let (derived__, base__) = ::windows_core::Compose::compose(compose);
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateInstance)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(&derived__), base__ as *mut _ as _, result__.as_mut_ptr()).from_abi::<ToggleButton>(result__)
        })
    }
    pub fn IsCheckedProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::IToggleButtonStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).IsCheckedProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn IsThreeStateProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::IToggleButtonStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).IsThreeStateProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn IToggleButtonFactory<R, F: FnOnce(&IToggleButtonFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<ToggleButton, IToggleButtonFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IToggleButtonStatics<R, F: FnOnce(&IToggleButtonStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<ToggleButton, IToggleButtonStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for ToggleButton {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ToggleButton {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ToggleButton {}
impl ::core::fmt::Debug for ToggleButton {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ToggleButton").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ToggleButton {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Controls.Primitives.ToggleButton;{589877fb-0fc7-4036-9d8b-127dfa75c16d})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for ToggleButton {
    type Vtable = IToggleButton_Vtbl;
    const IID: ::windows_core::GUID = <IToggleButton as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for ToggleButton {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.ToggleButton";
}
impl ::core::convert::From<ToggleButton> for ::windows_core::IUnknown {
    fn from(value: ToggleButton) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ToggleButton> for ::windows_core::IUnknown {
    fn from(value: &ToggleButton) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ToggleButton {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ToggleButton {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ToggleButton> for ::windows_core::IInspectable {
    fn from(value: ToggleButton) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ToggleButton> for ::windows_core::IInspectable {
    fn from(value: &ToggleButton) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for ToggleButton {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a ToggleButton {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<ToggleButton> for super::super::super::Composition::IAnimationObject {
    type Error = ::windows_core::Error;
    fn try_from(value: ToggleButton) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<&ToggleButton> for super::super::super::Composition::IAnimationObject {
    type Error = ::windows_core::Error;
    fn try_from(value: &ToggleButton) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows_core::IntoParam<'a, super::super::super::Composition::IAnimationObject> for ToggleButton {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::super::Composition::IAnimationObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows_core::IntoParam<'a, super::super::super::Composition::IAnimationObject> for &ToggleButton {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::super::Composition::IAnimationObject> {
        ::core::convert::TryInto::<super::super::super::Composition::IAnimationObject>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<ToggleButton> for super::super::super::Composition::IVisualElement {
    type Error = ::windows_core::Error;
    fn try_from(value: ToggleButton) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "UI_Composition")]
impl ::core::convert::TryFrom<&ToggleButton> for super::super::super::Composition::IVisualElement {
    type Error = ::windows_core::Error;
    fn try_from(value: &ToggleButton) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows_core::IntoParam<'a, super::super::super::Composition::IVisualElement> for ToggleButton {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::super::Composition::IVisualElement> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "UI_Composition")]
impl<'a> ::windows_core::IntoParam<'a, super::super::super::Composition::IVisualElement> for &ToggleButton {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::super::Composition::IVisualElement> {
        ::core::convert::TryInto::<super::super::super::Composition::IVisualElement>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::From<ToggleButton> for ButtonBase {
    fn from(value: ToggleButton) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&ToggleButton> for ButtonBase {
    fn from(value: &ToggleButton) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, ButtonBase> for ToggleButton {
    fn into_param(self) -> ::windows_core::Param<'a, ButtonBase> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ButtonBase> for &ToggleButton {
    fn into_param(self) -> ::windows_core::Param<'a, ButtonBase> {
        ::windows_core::Param::Owned(::core::convert::Into::<ButtonBase>::into(self))
    }
}
impl ::core::convert::From<ToggleButton> for super::ContentControl {
    fn from(value: ToggleButton) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&ToggleButton> for super::ContentControl {
    fn from(value: &ToggleButton) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::ContentControl> for ToggleButton {
    fn into_param(self) -> ::windows_core::Param<'a, super::ContentControl> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::ContentControl> for &ToggleButton {
    fn into_param(self) -> ::windows_core::Param<'a, super::ContentControl> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::ContentControl>::into(self))
    }
}
impl ::core::convert::From<ToggleButton> for super::Control {
    fn from(value: ToggleButton) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&ToggleButton> for super::Control {
    fn from(value: &ToggleButton) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::Control> for ToggleButton {
    fn into_param(self) -> ::windows_core::Param<'a, super::Control> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::Control> for &ToggleButton {
    fn into_param(self) -> ::windows_core::Param<'a, super::Control> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::Control>::into(self))
    }
}
impl ::core::convert::From<ToggleButton> for super::super::FrameworkElement {
    fn from(value: ToggleButton) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&ToggleButton> for super::super::FrameworkElement {
    fn from(value: &ToggleButton) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::FrameworkElement> for ToggleButton {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::FrameworkElement> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::FrameworkElement> for &ToggleButton {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::FrameworkElement> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::super::FrameworkElement>::into(self))
    }
}
impl ::core::convert::From<ToggleButton> for super::super::UIElement {
    fn from(value: ToggleButton) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&ToggleButton> for super::super::UIElement {
    fn from(value: &ToggleButton) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::UIElement> for ToggleButton {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::UIElement> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::UIElement> for &ToggleButton {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::UIElement> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::super::UIElement>::into(self))
    }
}
impl ::core::convert::From<ToggleButton> for super::super::DependencyObject {
    fn from(value: ToggleButton) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&ToggleButton> for super::super::DependencyObject {
    fn from(value: &ToggleButton) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::DependencyObject> for ToggleButton {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::DependencyObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::DependencyObject> for &ToggleButton {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::DependencyObject> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for ToggleButton {}
unsafe impl ::core::marker::Sync for ToggleButton {}
#[repr(transparent)]
pub struct ToggleSwitchTemplateSettings(::windows_core::IUnknown);
impl ToggleSwitchTemplateSettings {
    pub fn KnobCurrentToOnOffset(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).KnobCurrentToOnOffset)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn KnobCurrentToOffOffset(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).KnobCurrentToOffOffset)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn KnobOnToOffOffset(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).KnobOnToOffOffset)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn KnobOffToOnOffset(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).KnobOffToOnOffset)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn CurtainCurrentToOnOffset(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).CurtainCurrentToOnOffset)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn CurtainCurrentToOffOffset(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).CurtainCurrentToOffOffset)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn CurtainOnToOffOffset(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).CurtainOnToOffOffset)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn CurtainOffToOnOffset(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).CurtainOffToOnOffset)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
}
impl ::core::clone::Clone for ToggleSwitchTemplateSettings {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ToggleSwitchTemplateSettings {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ToggleSwitchTemplateSettings {}
impl ::core::fmt::Debug for ToggleSwitchTemplateSettings {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ToggleSwitchTemplateSettings").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ToggleSwitchTemplateSettings {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Controls.Primitives.ToggleSwitchTemplateSettings;{02b7bdcd-628a-4363-86e0-51d6e2e89e58})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for ToggleSwitchTemplateSettings {
    type Vtable = IToggleSwitchTemplateSettings_Vtbl;
    const IID: ::windows_core::GUID = <IToggleSwitchTemplateSettings as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for ToggleSwitchTemplateSettings {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.ToggleSwitchTemplateSettings";
}
impl ::core::convert::From<ToggleSwitchTemplateSettings> for ::windows_core::IUnknown {
    fn from(value: ToggleSwitchTemplateSettings) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ToggleSwitchTemplateSettings> for ::windows_core::IUnknown {
    fn from(value: &ToggleSwitchTemplateSettings) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ToggleSwitchTemplateSettings {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ToggleSwitchTemplateSettings {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ToggleSwitchTemplateSettings> for ::windows_core::IInspectable {
    fn from(value: ToggleSwitchTemplateSettings) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ToggleSwitchTemplateSettings> for ::windows_core::IInspectable {
    fn from(value: &ToggleSwitchTemplateSettings) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for ToggleSwitchTemplateSettings {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a ToggleSwitchTemplateSettings {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ToggleSwitchTemplateSettings> for super::super::DependencyObject {
    fn from(value: ToggleSwitchTemplateSettings) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&ToggleSwitchTemplateSettings> for super::super::DependencyObject {
    fn from(value: &ToggleSwitchTemplateSettings) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::DependencyObject> for ToggleSwitchTemplateSettings {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::DependencyObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::DependencyObject> for &ToggleSwitchTemplateSettings {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::DependencyObject> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for ToggleSwitchTemplateSettings {}
unsafe impl ::core::marker::Sync for ToggleSwitchTemplateSettings {}
#[repr(transparent)]
pub struct ToolTipTemplateSettings(::windows_core::IUnknown);
impl ToolTipTemplateSettings {
    pub fn FromHorizontalOffset(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).FromHorizontalOffset)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn FromVerticalOffset(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).FromVerticalOffset)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
}
impl ::core::clone::Clone for ToolTipTemplateSettings {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ToolTipTemplateSettings {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ToolTipTemplateSettings {}
impl ::core::fmt::Debug for ToolTipTemplateSettings {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ToolTipTemplateSettings").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ToolTipTemplateSettings {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Controls.Primitives.ToolTipTemplateSettings;{d4388247-0ec4-4506-affd-afac2225b48c})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for ToolTipTemplateSettings {
    type Vtable = IToolTipTemplateSettings_Vtbl;
    const IID: ::windows_core::GUID = <IToolTipTemplateSettings as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for ToolTipTemplateSettings {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.ToolTipTemplateSettings";
}
impl ::core::convert::From<ToolTipTemplateSettings> for ::windows_core::IUnknown {
    fn from(value: ToolTipTemplateSettings) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ToolTipTemplateSettings> for ::windows_core::IUnknown {
    fn from(value: &ToolTipTemplateSettings) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ToolTipTemplateSettings {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ToolTipTemplateSettings {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ToolTipTemplateSettings> for ::windows_core::IInspectable {
    fn from(value: ToolTipTemplateSettings) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ToolTipTemplateSettings> for ::windows_core::IInspectable {
    fn from(value: &ToolTipTemplateSettings) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for ToolTipTemplateSettings {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a ToolTipTemplateSettings {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ToolTipTemplateSettings> for super::super::DependencyObject {
    fn from(value: ToolTipTemplateSettings) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&ToolTipTemplateSettings> for super::super::DependencyObject {
    fn from(value: &ToolTipTemplateSettings) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::DependencyObject> for ToolTipTemplateSettings {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::DependencyObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::DependencyObject> for &ToolTipTemplateSettings {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::DependencyObject> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for ToolTipTemplateSettings {}
unsafe impl ::core::marker::Sync for ToolTipTemplateSettings {}
