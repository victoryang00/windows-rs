#[repr(transparent)]
pub struct AddDeleteThemeTransition(::windows_core::IUnknown);
impl AddDeleteThemeTransition {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<AddDeleteThemeTransition, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for AddDeleteThemeTransition {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AddDeleteThemeTransition {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AddDeleteThemeTransition {}
impl ::core::fmt::Debug for AddDeleteThemeTransition {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AddDeleteThemeTransition").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AddDeleteThemeTransition {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Animation.AddDeleteThemeTransition;{adec852e-4424-4dab-99c1-3a04e36a3c48})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for AddDeleteThemeTransition {
    type Vtable = IAddDeleteThemeTransition_Vtbl;
    const IID: ::windows_core::GUID = <IAddDeleteThemeTransition as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for AddDeleteThemeTransition {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.AddDeleteThemeTransition";
}
impl ::core::convert::From<AddDeleteThemeTransition> for ::windows_core::IUnknown {
    fn from(value: AddDeleteThemeTransition) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AddDeleteThemeTransition> for ::windows_core::IUnknown {
    fn from(value: &AddDeleteThemeTransition) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for AddDeleteThemeTransition {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a AddDeleteThemeTransition {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AddDeleteThemeTransition> for ::windows_core::IInspectable {
    fn from(value: AddDeleteThemeTransition) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AddDeleteThemeTransition> for ::windows_core::IInspectable {
    fn from(value: &AddDeleteThemeTransition) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for AddDeleteThemeTransition {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a AddDeleteThemeTransition {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AddDeleteThemeTransition> for Transition {
    fn from(value: AddDeleteThemeTransition) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&AddDeleteThemeTransition> for Transition {
    fn from(value: &AddDeleteThemeTransition) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, Transition> for AddDeleteThemeTransition {
    fn into_param(self) -> ::windows_core::Param<'a, Transition> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, Transition> for &AddDeleteThemeTransition {
    fn into_param(self) -> ::windows_core::Param<'a, Transition> {
        ::windows_core::Param::Owned(::core::convert::Into::<Transition>::into(self))
    }
}
impl ::core::convert::From<AddDeleteThemeTransition> for super::super::DependencyObject {
    fn from(value: AddDeleteThemeTransition) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&AddDeleteThemeTransition> for super::super::DependencyObject {
    fn from(value: &AddDeleteThemeTransition) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::DependencyObject> for AddDeleteThemeTransition {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::DependencyObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::DependencyObject> for &AddDeleteThemeTransition {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::DependencyObject> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for AddDeleteThemeTransition {}
unsafe impl ::core::marker::Sync for AddDeleteThemeTransition {}
#[repr(transparent)]
pub struct BackEase(::windows_core::IUnknown);
impl BackEase {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<BackEase, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn Amplitude(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).Amplitude)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn SetAmplitude(&self, value: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetAmplitude)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn AmplitudeProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::IBackEaseStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).AmplitudeProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn IBackEaseStatics<R, F: FnOnce(&IBackEaseStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<BackEase, IBackEaseStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for BackEase {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for BackEase {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for BackEase {}
impl ::core::fmt::Debug for BackEase {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BackEase").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for BackEase {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Animation.BackEase;{e47796e7-f805-4a8f-81c9-38e6472caa94})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for BackEase {
    type Vtable = IBackEase_Vtbl;
    const IID: ::windows_core::GUID = <IBackEase as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for BackEase {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.BackEase";
}
impl ::core::convert::From<BackEase> for ::windows_core::IUnknown {
    fn from(value: BackEase) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BackEase> for ::windows_core::IUnknown {
    fn from(value: &BackEase) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for BackEase {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a BackEase {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<BackEase> for ::windows_core::IInspectable {
    fn from(value: BackEase) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BackEase> for ::windows_core::IInspectable {
    fn from(value: &BackEase) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for BackEase {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a BackEase {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<BackEase> for EasingFunctionBase {
    fn from(value: BackEase) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&BackEase> for EasingFunctionBase {
    fn from(value: &BackEase) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, EasingFunctionBase> for BackEase {
    fn into_param(self) -> ::windows_core::Param<'a, EasingFunctionBase> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, EasingFunctionBase> for &BackEase {
    fn into_param(self) -> ::windows_core::Param<'a, EasingFunctionBase> {
        ::windows_core::Param::Owned(::core::convert::Into::<EasingFunctionBase>::into(self))
    }
}
impl ::core::convert::From<BackEase> for super::super::DependencyObject {
    fn from(value: BackEase) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&BackEase> for super::super::DependencyObject {
    fn from(value: &BackEase) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::DependencyObject> for BackEase {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::DependencyObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::DependencyObject> for &BackEase {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::DependencyObject> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for BackEase {}
unsafe impl ::core::marker::Sync for BackEase {}
#[repr(transparent)]
pub struct BasicConnectedAnimationConfiguration(::windows_core::IUnknown);
impl BasicConnectedAnimationConfiguration {
    pub fn new() -> ::windows_core::Result<BasicConnectedAnimationConfiguration> {
        Self::IBasicConnectedAnimationConfigurationFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateInstance)(::windows_core::Interface::as_raw(this), ::core::ptr::null_mut(), &mut ::core::option::Option::<::windows_core::IInspectable>::None as *mut _ as _, result__.as_mut_ptr()).from_abi::<BasicConnectedAnimationConfiguration>(result__)
        })
    }
    pub fn compose<T: ::windows_core::Compose>(compose: T) -> ::windows_core::Result<BasicConnectedAnimationConfiguration> {
        Self::IBasicConnectedAnimationConfigurationFactory(|this| unsafe {
            let (derived__, base__) = ::windows_core::Compose::compose(compose);
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateInstance)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(&derived__), base__ as *mut _ as _, result__.as_mut_ptr()).from_abi::<BasicConnectedAnimationConfiguration>(result__)
        })
    }
    pub fn IBasicConnectedAnimationConfigurationFactory<R, F: FnOnce(&IBasicConnectedAnimationConfigurationFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<BasicConnectedAnimationConfiguration, IBasicConnectedAnimationConfigurationFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for BasicConnectedAnimationConfiguration {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for BasicConnectedAnimationConfiguration {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for BasicConnectedAnimationConfiguration {}
impl ::core::fmt::Debug for BasicConnectedAnimationConfiguration {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BasicConnectedAnimationConfiguration").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for BasicConnectedAnimationConfiguration {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Animation.BasicConnectedAnimationConfiguration;{e675f9b5-a4d6-5353-83e6-c89e7cf8d456})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for BasicConnectedAnimationConfiguration {
    type Vtable = IBasicConnectedAnimationConfiguration_Vtbl;
    const IID: ::windows_core::GUID = <IBasicConnectedAnimationConfiguration as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for BasicConnectedAnimationConfiguration {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.BasicConnectedAnimationConfiguration";
}
impl ::core::convert::From<BasicConnectedAnimationConfiguration> for ::windows_core::IUnknown {
    fn from(value: BasicConnectedAnimationConfiguration) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BasicConnectedAnimationConfiguration> for ::windows_core::IUnknown {
    fn from(value: &BasicConnectedAnimationConfiguration) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for BasicConnectedAnimationConfiguration {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a BasicConnectedAnimationConfiguration {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<BasicConnectedAnimationConfiguration> for ::windows_core::IInspectable {
    fn from(value: BasicConnectedAnimationConfiguration) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BasicConnectedAnimationConfiguration> for ::windows_core::IInspectable {
    fn from(value: &BasicConnectedAnimationConfiguration) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for BasicConnectedAnimationConfiguration {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a BasicConnectedAnimationConfiguration {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<BasicConnectedAnimationConfiguration> for ConnectedAnimationConfiguration {
    fn from(value: BasicConnectedAnimationConfiguration) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&BasicConnectedAnimationConfiguration> for ConnectedAnimationConfiguration {
    fn from(value: &BasicConnectedAnimationConfiguration) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, ConnectedAnimationConfiguration> for BasicConnectedAnimationConfiguration {
    fn into_param(self) -> ::windows_core::Param<'a, ConnectedAnimationConfiguration> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ConnectedAnimationConfiguration> for &BasicConnectedAnimationConfiguration {
    fn into_param(self) -> ::windows_core::Param<'a, ConnectedAnimationConfiguration> {
        ::windows_core::Param::Owned(::core::convert::Into::<ConnectedAnimationConfiguration>::into(self))
    }
}
unsafe impl ::core::marker::Send for BasicConnectedAnimationConfiguration {}
unsafe impl ::core::marker::Sync for BasicConnectedAnimationConfiguration {}
#[repr(transparent)]
pub struct BeginStoryboard(::windows_core::IUnknown);
impl BeginStoryboard {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<BeginStoryboard, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn Storyboard(&self) -> ::windows_core::Result<Storyboard> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Storyboard)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<Storyboard>(result__)
        }
    }
    pub fn SetStoryboard<'a, Param0: ::windows_core::IntoParam<'a, Storyboard>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetStoryboard)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn StoryboardProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::IBeginStoryboardStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).StoryboardProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn IBeginStoryboardStatics<R, F: FnOnce(&IBeginStoryboardStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<BeginStoryboard, IBeginStoryboardStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for BeginStoryboard {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for BeginStoryboard {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for BeginStoryboard {}
impl ::core::fmt::Debug for BeginStoryboard {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BeginStoryboard").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for BeginStoryboard {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Animation.BeginStoryboard;{64189fcd-49ec-4e52-a6f6-55324c921053})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for BeginStoryboard {
    type Vtable = IBeginStoryboard_Vtbl;
    const IID: ::windows_core::GUID = <IBeginStoryboard as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for BeginStoryboard {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.BeginStoryboard";
}
impl ::core::convert::From<BeginStoryboard> for ::windows_core::IUnknown {
    fn from(value: BeginStoryboard) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BeginStoryboard> for ::windows_core::IUnknown {
    fn from(value: &BeginStoryboard) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for BeginStoryboard {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a BeginStoryboard {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<BeginStoryboard> for ::windows_core::IInspectable {
    fn from(value: BeginStoryboard) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BeginStoryboard> for ::windows_core::IInspectable {
    fn from(value: &BeginStoryboard) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for BeginStoryboard {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a BeginStoryboard {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<BeginStoryboard> for super::super::TriggerAction {
    fn from(value: BeginStoryboard) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&BeginStoryboard> for super::super::TriggerAction {
    fn from(value: &BeginStoryboard) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::TriggerAction> for BeginStoryboard {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::TriggerAction> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::TriggerAction> for &BeginStoryboard {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::TriggerAction> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::super::TriggerAction>::into(self))
    }
}
impl ::core::convert::From<BeginStoryboard> for super::super::DependencyObject {
    fn from(value: BeginStoryboard) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&BeginStoryboard> for super::super::DependencyObject {
    fn from(value: &BeginStoryboard) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::DependencyObject> for BeginStoryboard {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::DependencyObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::DependencyObject> for &BeginStoryboard {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::DependencyObject> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for BeginStoryboard {}
unsafe impl ::core::marker::Sync for BeginStoryboard {}
#[repr(transparent)]
pub struct BounceEase(::windows_core::IUnknown);
impl BounceEase {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<BounceEase, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn Bounces(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows_core::Interface::vtable(this).Bounces)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    pub fn SetBounces(&self, value: i32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetBounces)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Bounciness(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).Bounciness)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn SetBounciness(&self, value: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetBounciness)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn BouncesProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::IBounceEaseStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).BouncesProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn BouncinessProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::IBounceEaseStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).BouncinessProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn IBounceEaseStatics<R, F: FnOnce(&IBounceEaseStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<BounceEase, IBounceEaseStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for BounceEase {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for BounceEase {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for BounceEase {}
impl ::core::fmt::Debug for BounceEase {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BounceEase").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for BounceEase {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Animation.BounceEase;{2bf1464e-fc71-47ed-85a1-3ba9577718b4})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for BounceEase {
    type Vtable = IBounceEase_Vtbl;
    const IID: ::windows_core::GUID = <IBounceEase as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for BounceEase {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.BounceEase";
}
impl ::core::convert::From<BounceEase> for ::windows_core::IUnknown {
    fn from(value: BounceEase) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BounceEase> for ::windows_core::IUnknown {
    fn from(value: &BounceEase) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for BounceEase {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a BounceEase {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<BounceEase> for ::windows_core::IInspectable {
    fn from(value: BounceEase) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BounceEase> for ::windows_core::IInspectable {
    fn from(value: &BounceEase) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for BounceEase {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a BounceEase {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<BounceEase> for EasingFunctionBase {
    fn from(value: BounceEase) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&BounceEase> for EasingFunctionBase {
    fn from(value: &BounceEase) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, EasingFunctionBase> for BounceEase {
    fn into_param(self) -> ::windows_core::Param<'a, EasingFunctionBase> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, EasingFunctionBase> for &BounceEase {
    fn into_param(self) -> ::windows_core::Param<'a, EasingFunctionBase> {
        ::windows_core::Param::Owned(::core::convert::Into::<EasingFunctionBase>::into(self))
    }
}
impl ::core::convert::From<BounceEase> for super::super::DependencyObject {
    fn from(value: BounceEase) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&BounceEase> for super::super::DependencyObject {
    fn from(value: &BounceEase) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::DependencyObject> for BounceEase {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::DependencyObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::DependencyObject> for &BounceEase {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::DependencyObject> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for BounceEase {}
unsafe impl ::core::marker::Sync for BounceEase {}
#[repr(transparent)]
pub struct CircleEase(::windows_core::IUnknown);
impl CircleEase {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<CircleEase, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for CircleEase {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CircleEase {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CircleEase {}
impl ::core::fmt::Debug for CircleEase {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CircleEase").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for CircleEase {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Animation.CircleEase;{53a3bdb2-9177-4e6e-a043-5082d889ab1f})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for CircleEase {
    type Vtable = ICircleEase_Vtbl;
    const IID: ::windows_core::GUID = <ICircleEase as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for CircleEase {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.CircleEase";
}
impl ::core::convert::From<CircleEase> for ::windows_core::IUnknown {
    fn from(value: CircleEase) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CircleEase> for ::windows_core::IUnknown {
    fn from(value: &CircleEase) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for CircleEase {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a CircleEase {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<CircleEase> for ::windows_core::IInspectable {
    fn from(value: CircleEase) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CircleEase> for ::windows_core::IInspectable {
    fn from(value: &CircleEase) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for CircleEase {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a CircleEase {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<CircleEase> for EasingFunctionBase {
    fn from(value: CircleEase) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&CircleEase> for EasingFunctionBase {
    fn from(value: &CircleEase) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, EasingFunctionBase> for CircleEase {
    fn into_param(self) -> ::windows_core::Param<'a, EasingFunctionBase> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, EasingFunctionBase> for &CircleEase {
    fn into_param(self) -> ::windows_core::Param<'a, EasingFunctionBase> {
        ::windows_core::Param::Owned(::core::convert::Into::<EasingFunctionBase>::into(self))
    }
}
impl ::core::convert::From<CircleEase> for super::super::DependencyObject {
    fn from(value: CircleEase) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&CircleEase> for super::super::DependencyObject {
    fn from(value: &CircleEase) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::DependencyObject> for CircleEase {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::DependencyObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::DependencyObject> for &CircleEase {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::DependencyObject> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for CircleEase {}
unsafe impl ::core::marker::Sync for CircleEase {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct ClockState(pub i32);
impl ClockState {
    pub const Active: Self = Self(0i32);
    pub const Filling: Self = Self(1i32);
    pub const Stopped: Self = Self(2i32);
}
impl ::core::marker::Copy for ClockState {}
impl ::core::clone::Clone for ClockState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ClockState {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for ClockState {
    type Abi = Self;
}
impl ::core::fmt::Debug for ClockState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ClockState").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ClockState {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Media.Animation.ClockState;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct ColorAnimation(::windows_core::IUnknown);
impl ColorAnimation {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<ColorAnimation, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn From(&self) -> ::windows_core::Result<::winrt_foundation::IReference<super::super::super::Color>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).From)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IReference<super::super::super::Color>>(result__)
        }
    }
    pub fn SetFrom<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::IReference<super::super::super::Color>>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetFrom)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn To(&self) -> ::windows_core::Result<::winrt_foundation::IReference<super::super::super::Color>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).To)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IReference<super::super::super::Color>>(result__)
        }
    }
    pub fn SetTo<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::IReference<super::super::super::Color>>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetTo)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn By(&self) -> ::windows_core::Result<::winrt_foundation::IReference<super::super::super::Color>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).By)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IReference<super::super::super::Color>>(result__)
        }
    }
    pub fn SetBy<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::IReference<super::super::super::Color>>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetBy)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn EasingFunction(&self) -> ::windows_core::Result<EasingFunctionBase> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).EasingFunction)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<EasingFunctionBase>(result__)
        }
    }
    pub fn SetEasingFunction<'a, Param0: ::windows_core::IntoParam<'a, EasingFunctionBase>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetEasingFunction)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn EnableDependentAnimation(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).EnableDependentAnimation)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetEnableDependentAnimation(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetEnableDependentAnimation)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn FromProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::IColorAnimationStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FromProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn ToProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::IColorAnimationStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ToProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn ByProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::IColorAnimationStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ByProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn EasingFunctionProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::IColorAnimationStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).EasingFunctionProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn EnableDependentAnimationProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::IColorAnimationStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).EnableDependentAnimationProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn IColorAnimationStatics<R, F: FnOnce(&IColorAnimationStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<ColorAnimation, IColorAnimationStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for ColorAnimation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ColorAnimation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ColorAnimation {}
impl ::core::fmt::Debug for ColorAnimation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ColorAnimation").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ColorAnimation {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Animation.ColorAnimation;{b8ae8a15-0f63-4694-9467-bdafac1253ea})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for ColorAnimation {
    type Vtable = IColorAnimation_Vtbl;
    const IID: ::windows_core::GUID = <IColorAnimation as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for ColorAnimation {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.ColorAnimation";
}
impl ::core::convert::From<ColorAnimation> for ::windows_core::IUnknown {
    fn from(value: ColorAnimation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ColorAnimation> for ::windows_core::IUnknown {
    fn from(value: &ColorAnimation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ColorAnimation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ColorAnimation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ColorAnimation> for ::windows_core::IInspectable {
    fn from(value: ColorAnimation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ColorAnimation> for ::windows_core::IInspectable {
    fn from(value: &ColorAnimation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for ColorAnimation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a ColorAnimation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ColorAnimation> for Timeline {
    fn from(value: ColorAnimation) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&ColorAnimation> for Timeline {
    fn from(value: &ColorAnimation) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, Timeline> for ColorAnimation {
    fn into_param(self) -> ::windows_core::Param<'a, Timeline> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, Timeline> for &ColorAnimation {
    fn into_param(self) -> ::windows_core::Param<'a, Timeline> {
        ::windows_core::Param::Owned(::core::convert::Into::<Timeline>::into(self))
    }
}
impl ::core::convert::From<ColorAnimation> for super::super::DependencyObject {
    fn from(value: ColorAnimation) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&ColorAnimation> for super::super::DependencyObject {
    fn from(value: &ColorAnimation) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::DependencyObject> for ColorAnimation {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::DependencyObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::DependencyObject> for &ColorAnimation {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::DependencyObject> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for ColorAnimation {}
unsafe impl ::core::marker::Sync for ColorAnimation {}
#[repr(transparent)]
pub struct ColorAnimationUsingKeyFrames(::windows_core::IUnknown);
impl ColorAnimationUsingKeyFrames {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<ColorAnimationUsingKeyFrames, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn KeyFrames(&self) -> ::windows_core::Result<ColorKeyFrameCollection> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).KeyFrames)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ColorKeyFrameCollection>(result__)
        }
    }
    pub fn EnableDependentAnimation(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).EnableDependentAnimation)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetEnableDependentAnimation(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetEnableDependentAnimation)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn EnableDependentAnimationProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::IColorAnimationUsingKeyFramesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).EnableDependentAnimationProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn IColorAnimationUsingKeyFramesStatics<R, F: FnOnce(&IColorAnimationUsingKeyFramesStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<ColorAnimationUsingKeyFrames, IColorAnimationUsingKeyFramesStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for ColorAnimationUsingKeyFrames {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ColorAnimationUsingKeyFrames {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ColorAnimationUsingKeyFrames {}
impl ::core::fmt::Debug for ColorAnimationUsingKeyFrames {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ColorAnimationUsingKeyFrames").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ColorAnimationUsingKeyFrames {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Animation.ColorAnimationUsingKeyFrames;{f5c82640-13c3-42aa-9ae2-7e6b51c92f95})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for ColorAnimationUsingKeyFrames {
    type Vtable = IColorAnimationUsingKeyFrames_Vtbl;
    const IID: ::windows_core::GUID = <IColorAnimationUsingKeyFrames as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for ColorAnimationUsingKeyFrames {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.ColorAnimationUsingKeyFrames";
}
impl ::core::convert::From<ColorAnimationUsingKeyFrames> for ::windows_core::IUnknown {
    fn from(value: ColorAnimationUsingKeyFrames) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ColorAnimationUsingKeyFrames> for ::windows_core::IUnknown {
    fn from(value: &ColorAnimationUsingKeyFrames) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ColorAnimationUsingKeyFrames {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ColorAnimationUsingKeyFrames {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ColorAnimationUsingKeyFrames> for ::windows_core::IInspectable {
    fn from(value: ColorAnimationUsingKeyFrames) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ColorAnimationUsingKeyFrames> for ::windows_core::IInspectable {
    fn from(value: &ColorAnimationUsingKeyFrames) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for ColorAnimationUsingKeyFrames {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a ColorAnimationUsingKeyFrames {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ColorAnimationUsingKeyFrames> for Timeline {
    fn from(value: ColorAnimationUsingKeyFrames) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&ColorAnimationUsingKeyFrames> for Timeline {
    fn from(value: &ColorAnimationUsingKeyFrames) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, Timeline> for ColorAnimationUsingKeyFrames {
    fn into_param(self) -> ::windows_core::Param<'a, Timeline> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, Timeline> for &ColorAnimationUsingKeyFrames {
    fn into_param(self) -> ::windows_core::Param<'a, Timeline> {
        ::windows_core::Param::Owned(::core::convert::Into::<Timeline>::into(self))
    }
}
impl ::core::convert::From<ColorAnimationUsingKeyFrames> for super::super::DependencyObject {
    fn from(value: ColorAnimationUsingKeyFrames) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&ColorAnimationUsingKeyFrames> for super::super::DependencyObject {
    fn from(value: &ColorAnimationUsingKeyFrames) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::DependencyObject> for ColorAnimationUsingKeyFrames {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::DependencyObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::DependencyObject> for &ColorAnimationUsingKeyFrames {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::DependencyObject> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for ColorAnimationUsingKeyFrames {}
unsafe impl ::core::marker::Sync for ColorAnimationUsingKeyFrames {}
#[repr(transparent)]
pub struct ColorKeyFrame(::windows_core::IUnknown);
impl ColorKeyFrame {
    pub fn Value(&self) -> ::windows_core::Result<super::super::super::Color> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::super::Color>::zeroed();
            (::windows_core::Interface::vtable(this).Value)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Color>(result__)
        }
    }
    pub fn SetValue<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Color>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetValue)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn KeyTime(&self) -> ::windows_core::Result<KeyTime> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<KeyTime>::zeroed();
            (::windows_core::Interface::vtable(this).KeyTime)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<KeyTime>(result__)
        }
    }
    pub fn SetKeyTime<'a, Param0: ::windows_core::IntoParam<'a, KeyTime>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetKeyTime)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn ValueProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::IColorKeyFrameStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ValueProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn KeyTimeProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::IColorKeyFrameStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).KeyTimeProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn IColorKeyFrameStatics<R, F: FnOnce(&IColorKeyFrameStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<ColorKeyFrame, IColorKeyFrameStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for ColorKeyFrame {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ColorKeyFrame {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ColorKeyFrame {}
impl ::core::fmt::Debug for ColorKeyFrame {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ColorKeyFrame").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ColorKeyFrame {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Animation.ColorKeyFrame;{b51d82d9-0910-4589-a284-b0c9205858e9})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for ColorKeyFrame {
    type Vtable = IColorKeyFrame_Vtbl;
    const IID: ::windows_core::GUID = <IColorKeyFrame as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for ColorKeyFrame {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.ColorKeyFrame";
}
impl ::core::convert::From<ColorKeyFrame> for ::windows_core::IUnknown {
    fn from(value: ColorKeyFrame) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ColorKeyFrame> for ::windows_core::IUnknown {
    fn from(value: &ColorKeyFrame) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ColorKeyFrame {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ColorKeyFrame {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ColorKeyFrame> for ::windows_core::IInspectable {
    fn from(value: ColorKeyFrame) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ColorKeyFrame> for ::windows_core::IInspectable {
    fn from(value: &ColorKeyFrame) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for ColorKeyFrame {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a ColorKeyFrame {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ColorKeyFrame> for super::super::DependencyObject {
    fn from(value: ColorKeyFrame) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&ColorKeyFrame> for super::super::DependencyObject {
    fn from(value: &ColorKeyFrame) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::DependencyObject> for ColorKeyFrame {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::DependencyObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::DependencyObject> for &ColorKeyFrame {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::DependencyObject> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for ColorKeyFrame {}
unsafe impl ::core::marker::Sync for ColorKeyFrame {}
#[cfg(feature = "winrt-foundation")]
#[repr(transparent)]
pub struct ColorKeyFrameCollection(::windows_core::IUnknown);
#[cfg(feature = "winrt-foundation")]
impl ColorKeyFrameCollection {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<ColorKeyFrameCollection, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn First(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IIterator<ColorKeyFrame>> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IIterable<ColorKeyFrame>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).First)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IIterator<ColorKeyFrame>>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn GetAt(&self, index: u32) -> ::windows_core::Result<ColorKeyFrame> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetAt)(::windows_core::Interface::as_raw(this), index, result__.as_mut_ptr()).from_abi::<ColorKeyFrame>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn Size(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).Size)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn GetView(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<ColorKeyFrame>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetView)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<ColorKeyFrame>>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn IndexOf<'a, Param0: ::windows_core::IntoParam<'a, ColorKeyFrame>>(&self, value: Param0, index: &mut u32) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IndexOf)(::windows_core::Interface::as_raw(this), value.into_param().abi(), index, result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn SetAt<'a, Param1: ::windows_core::IntoParam<'a, ColorKeyFrame>>(&self, index: u32, value: Param1) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetAt)(::windows_core::Interface::as_raw(this), index, value.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn InsertAt<'a, Param1: ::windows_core::IntoParam<'a, ColorKeyFrame>>(&self, index: u32, value: Param1) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).InsertAt)(::windows_core::Interface::as_raw(this), index, value.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn RemoveAt(&self, index: u32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveAt)(::windows_core::Interface::as_raw(this), index).ok() }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn Append<'a, Param0: ::windows_core::IntoParam<'a, ColorKeyFrame>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Append)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn RemoveAtEnd(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveAtEnd)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn Clear(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Clear)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn GetMany(&self, startindex: u32, items: &mut [::core::option::Option<ColorKeyFrame>]) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).GetMany)(::windows_core::Interface::as_raw(this), startindex, items.len() as u32, ::core::mem::transmute_copy(&items), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn ReplaceAll(&self, items: &[::core::option::Option<ColorKeyFrame>]) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).ReplaceAll)(::windows_core::Interface::as_raw(this), items.len() as u32, ::core::mem::transmute(items.as_ptr())).ok() }
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::clone::Clone for ColorKeyFrameCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::cmp::PartialEq for ColorKeyFrameCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::cmp::Eq for ColorKeyFrameCollection {}
#[cfg(feature = "winrt-foundation")]
impl ::core::fmt::Debug for ColorKeyFrameCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ColorKeyFrameCollection").field(&self.0).finish()
    }
}
#[cfg(feature = "winrt-foundation")]
unsafe impl ::windows_core::RuntimeType for ColorKeyFrameCollection {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Animation.ColorKeyFrameCollection;pinterface({913337e9-11a1-4345-a3a2-4e7f956e222d};rc(Windows.UI.Xaml.Media.Animation.ColorKeyFrame;{b51d82d9-0910-4589-a284-b0c9205858e9})))");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[cfg(feature = "winrt-foundation")]
unsafe impl ::windows_core::Interface for ColorKeyFrameCollection {
    type Vtable = ::winrt_foundation::Collections::IVector_Vtbl<ColorKeyFrame>;
    const IID: ::windows_core::GUID = <::winrt_foundation::Collections::IVector<ColorKeyFrame> as ::windows_core::Interface>::IID;
}
#[cfg(feature = "winrt-foundation")]
impl ::windows_core::RuntimeName for ColorKeyFrameCollection {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.ColorKeyFrameCollection";
}
#[cfg(feature = "winrt-foundation")]
impl ::core::iter::IntoIterator for ColorKeyFrameCollection {
    type Item = ColorKeyFrame;
    type IntoIter = ::winrt_foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::iter::IntoIterator for &ColorKeyFrameCollection {
    type Item = ColorKeyFrame;
    type IntoIter = ::winrt_foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::winrt_foundation::Collections::VectorIterator::new(::core::convert::TryInto::try_into(self).ok())
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::From<ColorKeyFrameCollection> for ::windows_core::IUnknown {
    fn from(value: ColorKeyFrameCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::From<&ColorKeyFrameCollection> for ::windows_core::IUnknown {
    fn from(value: &ColorKeyFrameCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ColorKeyFrameCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ColorKeyFrameCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::From<ColorKeyFrameCollection> for ::windows_core::IInspectable {
    fn from(value: ColorKeyFrameCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::From<&ColorKeyFrameCollection> for ::windows_core::IInspectable {
    fn from(value: &ColorKeyFrameCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for ColorKeyFrameCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a ColorKeyFrameCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::TryFrom<ColorKeyFrameCollection> for ::winrt_foundation::Collections::IIterable<ColorKeyFrame> {
    type Error = ::windows_core::Error;
    fn try_from(value: ColorKeyFrameCollection) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::TryFrom<&ColorKeyFrameCollection> for ::winrt_foundation::Collections::IIterable<ColorKeyFrame> {
    type Error = ::windows_core::Error;
    fn try_from(value: &ColorKeyFrameCollection) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<ColorKeyFrame>> for ColorKeyFrameCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::Collections::IIterable<ColorKeyFrame>> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<ColorKeyFrame>> for &ColorKeyFrameCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::Collections::IIterable<ColorKeyFrame>> {
        ::core::convert::TryInto::<::winrt_foundation::Collections::IIterable<ColorKeyFrame>>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::TryFrom<ColorKeyFrameCollection> for ::winrt_foundation::Collections::IVector<ColorKeyFrame> {
    type Error = ::windows_core::Error;
    fn try_from(value: ColorKeyFrameCollection) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::TryFrom<&ColorKeyFrameCollection> for ::winrt_foundation::Collections::IVector<ColorKeyFrame> {
    type Error = ::windows_core::Error;
    fn try_from(value: &ColorKeyFrameCollection) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IVector<ColorKeyFrame>> for ColorKeyFrameCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::Collections::IVector<ColorKeyFrame>> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IVector<ColorKeyFrame>> for &ColorKeyFrameCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::Collections::IVector<ColorKeyFrame>> {
        ::core::convert::TryInto::<::winrt_foundation::Collections::IVector<ColorKeyFrame>>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "winrt-foundation")]
unsafe impl ::core::marker::Send for ColorKeyFrameCollection {}
#[cfg(feature = "winrt-foundation")]
unsafe impl ::core::marker::Sync for ColorKeyFrameCollection {}
#[repr(transparent)]
pub struct CommonNavigationTransitionInfo(::windows_core::IUnknown);
impl CommonNavigationTransitionInfo {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<CommonNavigationTransitionInfo, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IsStaggeringEnabled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsStaggeringEnabled)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetIsStaggeringEnabled(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetIsStaggeringEnabled)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsStaggeringEnabledProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::ICommonNavigationTransitionInfoStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).IsStaggeringEnabledProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn IsStaggerElementProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::ICommonNavigationTransitionInfoStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).IsStaggerElementProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn GetIsStaggerElement<'a, Param0: ::windows_core::IntoParam<'a, super::super::UIElement>>(element: Param0) -> ::windows_core::Result<bool> {
        Self::ICommonNavigationTransitionInfoStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).GetIsStaggerElement)(::windows_core::Interface::as_raw(this), element.into_param().abi(), result__.as_mut_ptr()).from_abi::<bool>(result__)
        })
    }
    pub fn SetIsStaggerElement<'a, Param0: ::windows_core::IntoParam<'a, super::super::UIElement>>(element: Param0, value: bool) -> ::windows_core::Result<()> {
        Self::ICommonNavigationTransitionInfoStatics(|this| unsafe { (::windows_core::Interface::vtable(this).SetIsStaggerElement)(::windows_core::Interface::as_raw(this), element.into_param().abi(), value).ok() })
    }
    pub fn ICommonNavigationTransitionInfoStatics<R, F: FnOnce(&ICommonNavigationTransitionInfoStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<CommonNavigationTransitionInfo, ICommonNavigationTransitionInfoStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for CommonNavigationTransitionInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CommonNavigationTransitionInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CommonNavigationTransitionInfo {}
impl ::core::fmt::Debug for CommonNavigationTransitionInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CommonNavigationTransitionInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for CommonNavigationTransitionInfo {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Animation.CommonNavigationTransitionInfo;{50345692-a555-4624-a361-0a91c1706473})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for CommonNavigationTransitionInfo {
    type Vtable = ICommonNavigationTransitionInfo_Vtbl;
    const IID: ::windows_core::GUID = <ICommonNavigationTransitionInfo as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for CommonNavigationTransitionInfo {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.CommonNavigationTransitionInfo";
}
impl ::core::convert::From<CommonNavigationTransitionInfo> for ::windows_core::IUnknown {
    fn from(value: CommonNavigationTransitionInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CommonNavigationTransitionInfo> for ::windows_core::IUnknown {
    fn from(value: &CommonNavigationTransitionInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for CommonNavigationTransitionInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a CommonNavigationTransitionInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<CommonNavigationTransitionInfo> for ::windows_core::IInspectable {
    fn from(value: CommonNavigationTransitionInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CommonNavigationTransitionInfo> for ::windows_core::IInspectable {
    fn from(value: &CommonNavigationTransitionInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for CommonNavigationTransitionInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a CommonNavigationTransitionInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<CommonNavigationTransitionInfo> for NavigationTransitionInfo {
    fn from(value: CommonNavigationTransitionInfo) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&CommonNavigationTransitionInfo> for NavigationTransitionInfo {
    fn from(value: &CommonNavigationTransitionInfo) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, NavigationTransitionInfo> for CommonNavigationTransitionInfo {
    fn into_param(self) -> ::windows_core::Param<'a, NavigationTransitionInfo> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, NavigationTransitionInfo> for &CommonNavigationTransitionInfo {
    fn into_param(self) -> ::windows_core::Param<'a, NavigationTransitionInfo> {
        ::windows_core::Param::Owned(::core::convert::Into::<NavigationTransitionInfo>::into(self))
    }
}
impl ::core::convert::From<CommonNavigationTransitionInfo> for super::super::DependencyObject {
    fn from(value: CommonNavigationTransitionInfo) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&CommonNavigationTransitionInfo> for super::super::DependencyObject {
    fn from(value: &CommonNavigationTransitionInfo) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::DependencyObject> for CommonNavigationTransitionInfo {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::DependencyObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::DependencyObject> for &CommonNavigationTransitionInfo {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::DependencyObject> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for CommonNavigationTransitionInfo {}
unsafe impl ::core::marker::Sync for CommonNavigationTransitionInfo {}
#[repr(transparent)]
pub struct ConnectedAnimation(::windows_core::IUnknown);
impl ConnectedAnimation {
    pub fn Completed<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<ConnectedAnimation, ::windows_core::IInspectable>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).Completed)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveCompleted<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveCompleted)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn TryStart<'a, Param0: ::windows_core::IntoParam<'a, super::super::UIElement>>(&self, destination: Param0) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).TryStart)(::windows_core::Interface::as_raw(this), destination.into_param().abi(), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn Cancel(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Cancel)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn IsScaleAnimationEnabled(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IConnectedAnimation2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsScaleAnimationEnabled)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetIsScaleAnimationEnabled(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IConnectedAnimation2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetIsScaleAnimationEnabled)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn TryStartWithCoordinatedElements<'a, Param0: ::windows_core::IntoParam<'a, super::super::UIElement>, Param1: ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<super::super::UIElement>>>(&self, destination: Param0, coordinatedelements: Param1) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IConnectedAnimation2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).TryStartWithCoordinatedElements)(::windows_core::Interface::as_raw(this), destination.into_param().abi(), coordinatedelements.into_param().abi(), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "winrt-ui")]
    pub fn SetAnimationComponent<'a, Param1: ::windows_core::IntoParam<'a, super::super::super::Composition::ICompositionAnimationBase>>(&self, component: ConnectedAnimationComponent, animation: Param1) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IConnectedAnimation2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetAnimationComponent)(::windows_core::Interface::as_raw(this), component, animation.into_param().abi()).ok() }
    }
    pub fn Configuration(&self) -> ::windows_core::Result<ConnectedAnimationConfiguration> {
        let this = &::windows_core::Interface::cast::<IConnectedAnimation3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Configuration)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ConnectedAnimationConfiguration>(result__)
        }
    }
    pub fn SetConfiguration<'a, Param0: ::windows_core::IntoParam<'a, ConnectedAnimationConfiguration>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IConnectedAnimation3>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetConfiguration)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
}
impl ::core::clone::Clone for ConnectedAnimation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ConnectedAnimation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ConnectedAnimation {}
impl ::core::fmt::Debug for ConnectedAnimation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ConnectedAnimation").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ConnectedAnimation {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Animation.ConnectedAnimation;{3518628c-f387-4c25-ac98-44e86c3cadf0})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for ConnectedAnimation {
    type Vtable = IConnectedAnimation_Vtbl;
    const IID: ::windows_core::GUID = <IConnectedAnimation as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for ConnectedAnimation {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.ConnectedAnimation";
}
impl ::core::convert::From<ConnectedAnimation> for ::windows_core::IUnknown {
    fn from(value: ConnectedAnimation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ConnectedAnimation> for ::windows_core::IUnknown {
    fn from(value: &ConnectedAnimation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ConnectedAnimation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ConnectedAnimation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ConnectedAnimation> for ::windows_core::IInspectable {
    fn from(value: ConnectedAnimation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ConnectedAnimation> for ::windows_core::IInspectable {
    fn from(value: &ConnectedAnimation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for ConnectedAnimation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a ConnectedAnimation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for ConnectedAnimation {}
unsafe impl ::core::marker::Sync for ConnectedAnimation {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct ConnectedAnimationComponent(pub i32);
impl ConnectedAnimationComponent {
    pub const OffsetX: Self = Self(0i32);
    pub const OffsetY: Self = Self(1i32);
    pub const CrossFade: Self = Self(2i32);
    pub const Scale: Self = Self(3i32);
}
impl ::core::marker::Copy for ConnectedAnimationComponent {}
impl ::core::clone::Clone for ConnectedAnimationComponent {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ConnectedAnimationComponent {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for ConnectedAnimationComponent {
    type Abi = Self;
}
impl ::core::fmt::Debug for ConnectedAnimationComponent {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ConnectedAnimationComponent").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ConnectedAnimationComponent {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Media.Animation.ConnectedAnimationComponent;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct ConnectedAnimationConfiguration(::windows_core::IUnknown);
impl ConnectedAnimationConfiguration {}
impl ::core::clone::Clone for ConnectedAnimationConfiguration {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ConnectedAnimationConfiguration {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ConnectedAnimationConfiguration {}
impl ::core::fmt::Debug for ConnectedAnimationConfiguration {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ConnectedAnimationConfiguration").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ConnectedAnimationConfiguration {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Animation.ConnectedAnimationConfiguration;{00218aae-cd8c-5651-92a0-c1db95c03998})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for ConnectedAnimationConfiguration {
    type Vtable = IConnectedAnimationConfiguration_Vtbl;
    const IID: ::windows_core::GUID = <IConnectedAnimationConfiguration as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for ConnectedAnimationConfiguration {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.ConnectedAnimationConfiguration";
}
impl ::core::convert::From<ConnectedAnimationConfiguration> for ::windows_core::IUnknown {
    fn from(value: ConnectedAnimationConfiguration) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ConnectedAnimationConfiguration> for ::windows_core::IUnknown {
    fn from(value: &ConnectedAnimationConfiguration) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ConnectedAnimationConfiguration {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ConnectedAnimationConfiguration {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ConnectedAnimationConfiguration> for ::windows_core::IInspectable {
    fn from(value: ConnectedAnimationConfiguration) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ConnectedAnimationConfiguration> for ::windows_core::IInspectable {
    fn from(value: &ConnectedAnimationConfiguration) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for ConnectedAnimationConfiguration {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a ConnectedAnimationConfiguration {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for ConnectedAnimationConfiguration {}
unsafe impl ::core::marker::Sync for ConnectedAnimationConfiguration {}
#[repr(transparent)]
pub struct ConnectedAnimationService(::windows_core::IUnknown);
impl ConnectedAnimationService {
    pub fn DefaultDuration(&self) -> ::windows_core::Result<::winrt_foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::TimeSpan>::zeroed();
            (::windows_core::Interface::vtable(this).DefaultDuration)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::TimeSpan>(result__)
        }
    }
    pub fn SetDefaultDuration<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TimeSpan>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetDefaultDuration)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-ui")]
    pub fn DefaultEasingFunction(&self) -> ::windows_core::Result<super::super::super::Composition::CompositionEasingFunction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).DefaultEasingFunction)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Composition::CompositionEasingFunction>(result__)
        }
    }
    #[cfg(feature = "winrt-ui")]
    pub fn SetDefaultEasingFunction<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Composition::CompositionEasingFunction>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetDefaultEasingFunction)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn PrepareToAnimate<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, super::super::UIElement>>(&self, key: Param0, source: Param1) -> ::windows_core::Result<ConnectedAnimation> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).PrepareToAnimate)(::windows_core::Interface::as_raw(this), key.into_param().abi(), source.into_param().abi(), result__.as_mut_ptr()).from_abi::<ConnectedAnimation>(result__)
        }
    }
    pub fn GetAnimation<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, key: Param0) -> ::windows_core::Result<ConnectedAnimation> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetAnimation)(::windows_core::Interface::as_raw(this), key.into_param().abi(), result__.as_mut_ptr()).from_abi::<ConnectedAnimation>(result__)
        }
    }
    pub fn GetForCurrentView() -> ::windows_core::Result<ConnectedAnimationService> {
        Self::IConnectedAnimationServiceStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetForCurrentView)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ConnectedAnimationService>(result__)
        })
    }
    pub fn IConnectedAnimationServiceStatics<R, F: FnOnce(&IConnectedAnimationServiceStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<ConnectedAnimationService, IConnectedAnimationServiceStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for ConnectedAnimationService {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ConnectedAnimationService {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ConnectedAnimationService {}
impl ::core::fmt::Debug for ConnectedAnimationService {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ConnectedAnimationService").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ConnectedAnimationService {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Animation.ConnectedAnimationService;{1c6875c9-19bb-4d47-b9aa-66c802dcb9ff})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for ConnectedAnimationService {
    type Vtable = IConnectedAnimationService_Vtbl;
    const IID: ::windows_core::GUID = <IConnectedAnimationService as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for ConnectedAnimationService {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.ConnectedAnimationService";
}
impl ::core::convert::From<ConnectedAnimationService> for ::windows_core::IUnknown {
    fn from(value: ConnectedAnimationService) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ConnectedAnimationService> for ::windows_core::IUnknown {
    fn from(value: &ConnectedAnimationService) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ConnectedAnimationService {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ConnectedAnimationService {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ConnectedAnimationService> for ::windows_core::IInspectable {
    fn from(value: ConnectedAnimationService) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ConnectedAnimationService> for ::windows_core::IInspectable {
    fn from(value: &ConnectedAnimationService) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for ConnectedAnimationService {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a ConnectedAnimationService {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for ConnectedAnimationService {}
unsafe impl ::core::marker::Sync for ConnectedAnimationService {}
#[repr(transparent)]
pub struct ContentThemeTransition(::windows_core::IUnknown);
impl ContentThemeTransition {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<ContentThemeTransition, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
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
    pub fn HorizontalOffsetProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::IContentThemeTransitionStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).HorizontalOffsetProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn VerticalOffsetProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::IContentThemeTransitionStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).VerticalOffsetProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn IContentThemeTransitionStatics<R, F: FnOnce(&IContentThemeTransitionStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<ContentThemeTransition, IContentThemeTransitionStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for ContentThemeTransition {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ContentThemeTransition {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ContentThemeTransition {}
impl ::core::fmt::Debug for ContentThemeTransition {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ContentThemeTransition").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ContentThemeTransition {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Animation.ContentThemeTransition;{f66fc5c3-5915-437d-8e3b-adf8e7f0ab57})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for ContentThemeTransition {
    type Vtable = IContentThemeTransition_Vtbl;
    const IID: ::windows_core::GUID = <IContentThemeTransition as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for ContentThemeTransition {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.ContentThemeTransition";
}
impl ::core::convert::From<ContentThemeTransition> for ::windows_core::IUnknown {
    fn from(value: ContentThemeTransition) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ContentThemeTransition> for ::windows_core::IUnknown {
    fn from(value: &ContentThemeTransition) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ContentThemeTransition {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ContentThemeTransition {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ContentThemeTransition> for ::windows_core::IInspectable {
    fn from(value: ContentThemeTransition) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ContentThemeTransition> for ::windows_core::IInspectable {
    fn from(value: &ContentThemeTransition) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for ContentThemeTransition {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a ContentThemeTransition {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ContentThemeTransition> for Transition {
    fn from(value: ContentThemeTransition) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&ContentThemeTransition> for Transition {
    fn from(value: &ContentThemeTransition) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, Transition> for ContentThemeTransition {
    fn into_param(self) -> ::windows_core::Param<'a, Transition> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, Transition> for &ContentThemeTransition {
    fn into_param(self) -> ::windows_core::Param<'a, Transition> {
        ::windows_core::Param::Owned(::core::convert::Into::<Transition>::into(self))
    }
}
impl ::core::convert::From<ContentThemeTransition> for super::super::DependencyObject {
    fn from(value: ContentThemeTransition) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&ContentThemeTransition> for super::super::DependencyObject {
    fn from(value: &ContentThemeTransition) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::DependencyObject> for ContentThemeTransition {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::DependencyObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::DependencyObject> for &ContentThemeTransition {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::DependencyObject> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for ContentThemeTransition {}
unsafe impl ::core::marker::Sync for ContentThemeTransition {}
#[repr(transparent)]
pub struct ContinuumNavigationTransitionInfo(::windows_core::IUnknown);
impl ContinuumNavigationTransitionInfo {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<ContinuumNavigationTransitionInfo, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn ExitElement(&self) -> ::windows_core::Result<super::super::UIElement> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ExitElement)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::UIElement>(result__)
        }
    }
    pub fn SetExitElement<'a, Param0: ::windows_core::IntoParam<'a, super::super::UIElement>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetExitElement)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn ExitElementProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::IContinuumNavigationTransitionInfoStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ExitElementProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn IsEntranceElementProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::IContinuumNavigationTransitionInfoStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).IsEntranceElementProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn GetIsEntranceElement<'a, Param0: ::windows_core::IntoParam<'a, super::super::UIElement>>(element: Param0) -> ::windows_core::Result<bool> {
        Self::IContinuumNavigationTransitionInfoStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).GetIsEntranceElement)(::windows_core::Interface::as_raw(this), element.into_param().abi(), result__.as_mut_ptr()).from_abi::<bool>(result__)
        })
    }
    pub fn SetIsEntranceElement<'a, Param0: ::windows_core::IntoParam<'a, super::super::UIElement>>(element: Param0, value: bool) -> ::windows_core::Result<()> {
        Self::IContinuumNavigationTransitionInfoStatics(|this| unsafe { (::windows_core::Interface::vtable(this).SetIsEntranceElement)(::windows_core::Interface::as_raw(this), element.into_param().abi(), value).ok() })
    }
    pub fn IsExitElementProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::IContinuumNavigationTransitionInfoStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).IsExitElementProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn GetIsExitElement<'a, Param0: ::windows_core::IntoParam<'a, super::super::UIElement>>(element: Param0) -> ::windows_core::Result<bool> {
        Self::IContinuumNavigationTransitionInfoStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).GetIsExitElement)(::windows_core::Interface::as_raw(this), element.into_param().abi(), result__.as_mut_ptr()).from_abi::<bool>(result__)
        })
    }
    pub fn SetIsExitElement<'a, Param0: ::windows_core::IntoParam<'a, super::super::UIElement>>(element: Param0, value: bool) -> ::windows_core::Result<()> {
        Self::IContinuumNavigationTransitionInfoStatics(|this| unsafe { (::windows_core::Interface::vtable(this).SetIsExitElement)(::windows_core::Interface::as_raw(this), element.into_param().abi(), value).ok() })
    }
    pub fn ExitElementContainerProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::IContinuumNavigationTransitionInfoStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ExitElementContainerProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    #[cfg(feature = "winrt-ui")]
    pub fn GetExitElementContainer<'a, Param0: ::windows_core::IntoParam<'a, super::super::Controls::ListViewBase>>(element: Param0) -> ::windows_core::Result<bool> {
        Self::IContinuumNavigationTransitionInfoStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).GetExitElementContainer)(::windows_core::Interface::as_raw(this), element.into_param().abi(), result__.as_mut_ptr()).from_abi::<bool>(result__)
        })
    }
    #[cfg(feature = "winrt-ui")]
    pub fn SetExitElementContainer<'a, Param0: ::windows_core::IntoParam<'a, super::super::Controls::ListViewBase>>(element: Param0, value: bool) -> ::windows_core::Result<()> {
        Self::IContinuumNavigationTransitionInfoStatics(|this| unsafe { (::windows_core::Interface::vtable(this).SetExitElementContainer)(::windows_core::Interface::as_raw(this), element.into_param().abi(), value).ok() })
    }
    pub fn IContinuumNavigationTransitionInfoStatics<R, F: FnOnce(&IContinuumNavigationTransitionInfoStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<ContinuumNavigationTransitionInfo, IContinuumNavigationTransitionInfoStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for ContinuumNavigationTransitionInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ContinuumNavigationTransitionInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ContinuumNavigationTransitionInfo {}
impl ::core::fmt::Debug for ContinuumNavigationTransitionInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ContinuumNavigationTransitionInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ContinuumNavigationTransitionInfo {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Animation.ContinuumNavigationTransitionInfo;{4be1dbad-8ba6-4004-8438-8a9017978543})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for ContinuumNavigationTransitionInfo {
    type Vtable = IContinuumNavigationTransitionInfo_Vtbl;
    const IID: ::windows_core::GUID = <IContinuumNavigationTransitionInfo as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for ContinuumNavigationTransitionInfo {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.ContinuumNavigationTransitionInfo";
}
impl ::core::convert::From<ContinuumNavigationTransitionInfo> for ::windows_core::IUnknown {
    fn from(value: ContinuumNavigationTransitionInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ContinuumNavigationTransitionInfo> for ::windows_core::IUnknown {
    fn from(value: &ContinuumNavigationTransitionInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ContinuumNavigationTransitionInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ContinuumNavigationTransitionInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ContinuumNavigationTransitionInfo> for ::windows_core::IInspectable {
    fn from(value: ContinuumNavigationTransitionInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ContinuumNavigationTransitionInfo> for ::windows_core::IInspectable {
    fn from(value: &ContinuumNavigationTransitionInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for ContinuumNavigationTransitionInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a ContinuumNavigationTransitionInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ContinuumNavigationTransitionInfo> for NavigationTransitionInfo {
    fn from(value: ContinuumNavigationTransitionInfo) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&ContinuumNavigationTransitionInfo> for NavigationTransitionInfo {
    fn from(value: &ContinuumNavigationTransitionInfo) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, NavigationTransitionInfo> for ContinuumNavigationTransitionInfo {
    fn into_param(self) -> ::windows_core::Param<'a, NavigationTransitionInfo> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, NavigationTransitionInfo> for &ContinuumNavigationTransitionInfo {
    fn into_param(self) -> ::windows_core::Param<'a, NavigationTransitionInfo> {
        ::windows_core::Param::Owned(::core::convert::Into::<NavigationTransitionInfo>::into(self))
    }
}
impl ::core::convert::From<ContinuumNavigationTransitionInfo> for super::super::DependencyObject {
    fn from(value: ContinuumNavigationTransitionInfo) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&ContinuumNavigationTransitionInfo> for super::super::DependencyObject {
    fn from(value: &ContinuumNavigationTransitionInfo) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::DependencyObject> for ContinuumNavigationTransitionInfo {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::DependencyObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::DependencyObject> for &ContinuumNavigationTransitionInfo {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::DependencyObject> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for ContinuumNavigationTransitionInfo {}
unsafe impl ::core::marker::Sync for ContinuumNavigationTransitionInfo {}
#[repr(transparent)]
pub struct CubicEase(::windows_core::IUnknown);
impl CubicEase {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<CubicEase, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for CubicEase {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CubicEase {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CubicEase {}
impl ::core::fmt::Debug for CubicEase {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CubicEase").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for CubicEase {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Animation.CubicEase;{1b94fc76-dad7-4354-b1a2-7969fbf6a70d})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for CubicEase {
    type Vtable = ICubicEase_Vtbl;
    const IID: ::windows_core::GUID = <ICubicEase as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for CubicEase {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.CubicEase";
}
impl ::core::convert::From<CubicEase> for ::windows_core::IUnknown {
    fn from(value: CubicEase) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CubicEase> for ::windows_core::IUnknown {
    fn from(value: &CubicEase) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for CubicEase {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a CubicEase {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<CubicEase> for ::windows_core::IInspectable {
    fn from(value: CubicEase) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CubicEase> for ::windows_core::IInspectable {
    fn from(value: &CubicEase) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for CubicEase {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a CubicEase {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<CubicEase> for EasingFunctionBase {
    fn from(value: CubicEase) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&CubicEase> for EasingFunctionBase {
    fn from(value: &CubicEase) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, EasingFunctionBase> for CubicEase {
    fn into_param(self) -> ::windows_core::Param<'a, EasingFunctionBase> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, EasingFunctionBase> for &CubicEase {
    fn into_param(self) -> ::windows_core::Param<'a, EasingFunctionBase> {
        ::windows_core::Param::Owned(::core::convert::Into::<EasingFunctionBase>::into(self))
    }
}
impl ::core::convert::From<CubicEase> for super::super::DependencyObject {
    fn from(value: CubicEase) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&CubicEase> for super::super::DependencyObject {
    fn from(value: &CubicEase) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::DependencyObject> for CubicEase {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::DependencyObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::DependencyObject> for &CubicEase {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::DependencyObject> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for CubicEase {}
unsafe impl ::core::marker::Sync for CubicEase {}
#[repr(transparent)]
pub struct DirectConnectedAnimationConfiguration(::windows_core::IUnknown);
impl DirectConnectedAnimationConfiguration {
    pub fn new() -> ::windows_core::Result<DirectConnectedAnimationConfiguration> {
        Self::IDirectConnectedAnimationConfigurationFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateInstance)(::windows_core::Interface::as_raw(this), ::core::ptr::null_mut(), &mut ::core::option::Option::<::windows_core::IInspectable>::None as *mut _ as _, result__.as_mut_ptr()).from_abi::<DirectConnectedAnimationConfiguration>(result__)
        })
    }
    pub fn compose<T: ::windows_core::Compose>(compose: T) -> ::windows_core::Result<DirectConnectedAnimationConfiguration> {
        Self::IDirectConnectedAnimationConfigurationFactory(|this| unsafe {
            let (derived__, base__) = ::windows_core::Compose::compose(compose);
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateInstance)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(&derived__), base__ as *mut _ as _, result__.as_mut_ptr()).from_abi::<DirectConnectedAnimationConfiguration>(result__)
        })
    }
    pub fn IDirectConnectedAnimationConfigurationFactory<R, F: FnOnce(&IDirectConnectedAnimationConfigurationFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<DirectConnectedAnimationConfiguration, IDirectConnectedAnimationConfigurationFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for DirectConnectedAnimationConfiguration {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for DirectConnectedAnimationConfiguration {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DirectConnectedAnimationConfiguration {}
impl ::core::fmt::Debug for DirectConnectedAnimationConfiguration {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DirectConnectedAnimationConfiguration").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for DirectConnectedAnimationConfiguration {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Animation.DirectConnectedAnimationConfiguration;{ee5d736f-5738-5d86-b770-151948cf365e})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for DirectConnectedAnimationConfiguration {
    type Vtable = IDirectConnectedAnimationConfiguration_Vtbl;
    const IID: ::windows_core::GUID = <IDirectConnectedAnimationConfiguration as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for DirectConnectedAnimationConfiguration {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.DirectConnectedAnimationConfiguration";
}
impl ::core::convert::From<DirectConnectedAnimationConfiguration> for ::windows_core::IUnknown {
    fn from(value: DirectConnectedAnimationConfiguration) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DirectConnectedAnimationConfiguration> for ::windows_core::IUnknown {
    fn from(value: &DirectConnectedAnimationConfiguration) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for DirectConnectedAnimationConfiguration {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a DirectConnectedAnimationConfiguration {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<DirectConnectedAnimationConfiguration> for ::windows_core::IInspectable {
    fn from(value: DirectConnectedAnimationConfiguration) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DirectConnectedAnimationConfiguration> for ::windows_core::IInspectable {
    fn from(value: &DirectConnectedAnimationConfiguration) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for DirectConnectedAnimationConfiguration {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a DirectConnectedAnimationConfiguration {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<DirectConnectedAnimationConfiguration> for ConnectedAnimationConfiguration {
    fn from(value: DirectConnectedAnimationConfiguration) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&DirectConnectedAnimationConfiguration> for ConnectedAnimationConfiguration {
    fn from(value: &DirectConnectedAnimationConfiguration) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, ConnectedAnimationConfiguration> for DirectConnectedAnimationConfiguration {
    fn into_param(self) -> ::windows_core::Param<'a, ConnectedAnimationConfiguration> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ConnectedAnimationConfiguration> for &DirectConnectedAnimationConfiguration {
    fn into_param(self) -> ::windows_core::Param<'a, ConnectedAnimationConfiguration> {
        ::windows_core::Param::Owned(::core::convert::Into::<ConnectedAnimationConfiguration>::into(self))
    }
}
unsafe impl ::core::marker::Send for DirectConnectedAnimationConfiguration {}
unsafe impl ::core::marker::Sync for DirectConnectedAnimationConfiguration {}
#[repr(transparent)]
pub struct DiscreteColorKeyFrame(::windows_core::IUnknown);
impl DiscreteColorKeyFrame {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<DiscreteColorKeyFrame, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for DiscreteColorKeyFrame {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for DiscreteColorKeyFrame {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DiscreteColorKeyFrame {}
impl ::core::fmt::Debug for DiscreteColorKeyFrame {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DiscreteColorKeyFrame").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for DiscreteColorKeyFrame {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Animation.DiscreteColorKeyFrame;{230c08f4-e062-4cb1-8e2a-14093d73ed8c})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for DiscreteColorKeyFrame {
    type Vtable = IDiscreteColorKeyFrame_Vtbl;
    const IID: ::windows_core::GUID = <IDiscreteColorKeyFrame as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for DiscreteColorKeyFrame {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.DiscreteColorKeyFrame";
}
impl ::core::convert::From<DiscreteColorKeyFrame> for ::windows_core::IUnknown {
    fn from(value: DiscreteColorKeyFrame) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DiscreteColorKeyFrame> for ::windows_core::IUnknown {
    fn from(value: &DiscreteColorKeyFrame) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for DiscreteColorKeyFrame {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a DiscreteColorKeyFrame {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<DiscreteColorKeyFrame> for ::windows_core::IInspectable {
    fn from(value: DiscreteColorKeyFrame) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DiscreteColorKeyFrame> for ::windows_core::IInspectable {
    fn from(value: &DiscreteColorKeyFrame) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for DiscreteColorKeyFrame {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a DiscreteColorKeyFrame {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<DiscreteColorKeyFrame> for ColorKeyFrame {
    fn from(value: DiscreteColorKeyFrame) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&DiscreteColorKeyFrame> for ColorKeyFrame {
    fn from(value: &DiscreteColorKeyFrame) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, ColorKeyFrame> for DiscreteColorKeyFrame {
    fn into_param(self) -> ::windows_core::Param<'a, ColorKeyFrame> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ColorKeyFrame> for &DiscreteColorKeyFrame {
    fn into_param(self) -> ::windows_core::Param<'a, ColorKeyFrame> {
        ::windows_core::Param::Owned(::core::convert::Into::<ColorKeyFrame>::into(self))
    }
}
impl ::core::convert::From<DiscreteColorKeyFrame> for super::super::DependencyObject {
    fn from(value: DiscreteColorKeyFrame) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&DiscreteColorKeyFrame> for super::super::DependencyObject {
    fn from(value: &DiscreteColorKeyFrame) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::DependencyObject> for DiscreteColorKeyFrame {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::DependencyObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::DependencyObject> for &DiscreteColorKeyFrame {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::DependencyObject> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for DiscreteColorKeyFrame {}
unsafe impl ::core::marker::Sync for DiscreteColorKeyFrame {}
#[repr(transparent)]
pub struct DiscreteDoubleKeyFrame(::windows_core::IUnknown);
impl DiscreteDoubleKeyFrame {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<DiscreteDoubleKeyFrame, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for DiscreteDoubleKeyFrame {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for DiscreteDoubleKeyFrame {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DiscreteDoubleKeyFrame {}
impl ::core::fmt::Debug for DiscreteDoubleKeyFrame {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DiscreteDoubleKeyFrame").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for DiscreteDoubleKeyFrame {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Animation.DiscreteDoubleKeyFrame;{f5f51f3a-ad11-49ce-8e1c-08fdf1447446})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for DiscreteDoubleKeyFrame {
    type Vtable = IDiscreteDoubleKeyFrame_Vtbl;
    const IID: ::windows_core::GUID = <IDiscreteDoubleKeyFrame as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for DiscreteDoubleKeyFrame {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.DiscreteDoubleKeyFrame";
}
impl ::core::convert::From<DiscreteDoubleKeyFrame> for ::windows_core::IUnknown {
    fn from(value: DiscreteDoubleKeyFrame) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DiscreteDoubleKeyFrame> for ::windows_core::IUnknown {
    fn from(value: &DiscreteDoubleKeyFrame) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for DiscreteDoubleKeyFrame {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a DiscreteDoubleKeyFrame {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<DiscreteDoubleKeyFrame> for ::windows_core::IInspectable {
    fn from(value: DiscreteDoubleKeyFrame) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DiscreteDoubleKeyFrame> for ::windows_core::IInspectable {
    fn from(value: &DiscreteDoubleKeyFrame) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for DiscreteDoubleKeyFrame {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a DiscreteDoubleKeyFrame {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<DiscreteDoubleKeyFrame> for DoubleKeyFrame {
    fn from(value: DiscreteDoubleKeyFrame) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&DiscreteDoubleKeyFrame> for DoubleKeyFrame {
    fn from(value: &DiscreteDoubleKeyFrame) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, DoubleKeyFrame> for DiscreteDoubleKeyFrame {
    fn into_param(self) -> ::windows_core::Param<'a, DoubleKeyFrame> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, DoubleKeyFrame> for &DiscreteDoubleKeyFrame {
    fn into_param(self) -> ::windows_core::Param<'a, DoubleKeyFrame> {
        ::windows_core::Param::Owned(::core::convert::Into::<DoubleKeyFrame>::into(self))
    }
}
impl ::core::convert::From<DiscreteDoubleKeyFrame> for super::super::DependencyObject {
    fn from(value: DiscreteDoubleKeyFrame) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&DiscreteDoubleKeyFrame> for super::super::DependencyObject {
    fn from(value: &DiscreteDoubleKeyFrame) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::DependencyObject> for DiscreteDoubleKeyFrame {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::DependencyObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::DependencyObject> for &DiscreteDoubleKeyFrame {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::DependencyObject> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for DiscreteDoubleKeyFrame {}
unsafe impl ::core::marker::Sync for DiscreteDoubleKeyFrame {}
#[repr(transparent)]
pub struct DiscreteObjectKeyFrame(::windows_core::IUnknown);
impl DiscreteObjectKeyFrame {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<DiscreteObjectKeyFrame, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for DiscreteObjectKeyFrame {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for DiscreteObjectKeyFrame {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DiscreteObjectKeyFrame {}
impl ::core::fmt::Debug for DiscreteObjectKeyFrame {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DiscreteObjectKeyFrame").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for DiscreteObjectKeyFrame {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Animation.DiscreteObjectKeyFrame;{c7dcde89-f12d-4a9c-8199-e7a9ece3a473})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for DiscreteObjectKeyFrame {
    type Vtable = IDiscreteObjectKeyFrame_Vtbl;
    const IID: ::windows_core::GUID = <IDiscreteObjectKeyFrame as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for DiscreteObjectKeyFrame {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.DiscreteObjectKeyFrame";
}
impl ::core::convert::From<DiscreteObjectKeyFrame> for ::windows_core::IUnknown {
    fn from(value: DiscreteObjectKeyFrame) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DiscreteObjectKeyFrame> for ::windows_core::IUnknown {
    fn from(value: &DiscreteObjectKeyFrame) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for DiscreteObjectKeyFrame {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a DiscreteObjectKeyFrame {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<DiscreteObjectKeyFrame> for ::windows_core::IInspectable {
    fn from(value: DiscreteObjectKeyFrame) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DiscreteObjectKeyFrame> for ::windows_core::IInspectable {
    fn from(value: &DiscreteObjectKeyFrame) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for DiscreteObjectKeyFrame {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a DiscreteObjectKeyFrame {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<DiscreteObjectKeyFrame> for ObjectKeyFrame {
    fn from(value: DiscreteObjectKeyFrame) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&DiscreteObjectKeyFrame> for ObjectKeyFrame {
    fn from(value: &DiscreteObjectKeyFrame) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, ObjectKeyFrame> for DiscreteObjectKeyFrame {
    fn into_param(self) -> ::windows_core::Param<'a, ObjectKeyFrame> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ObjectKeyFrame> for &DiscreteObjectKeyFrame {
    fn into_param(self) -> ::windows_core::Param<'a, ObjectKeyFrame> {
        ::windows_core::Param::Owned(::core::convert::Into::<ObjectKeyFrame>::into(self))
    }
}
impl ::core::convert::From<DiscreteObjectKeyFrame> for super::super::DependencyObject {
    fn from(value: DiscreteObjectKeyFrame) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&DiscreteObjectKeyFrame> for super::super::DependencyObject {
    fn from(value: &DiscreteObjectKeyFrame) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::DependencyObject> for DiscreteObjectKeyFrame {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::DependencyObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::DependencyObject> for &DiscreteObjectKeyFrame {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::DependencyObject> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for DiscreteObjectKeyFrame {}
unsafe impl ::core::marker::Sync for DiscreteObjectKeyFrame {}
#[repr(transparent)]
pub struct DiscretePointKeyFrame(::windows_core::IUnknown);
impl DiscretePointKeyFrame {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<DiscretePointKeyFrame, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for DiscretePointKeyFrame {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for DiscretePointKeyFrame {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DiscretePointKeyFrame {}
impl ::core::fmt::Debug for DiscretePointKeyFrame {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DiscretePointKeyFrame").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for DiscretePointKeyFrame {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Animation.DiscretePointKeyFrame;{e0a9070d-4c42-4a90-983a-75f5a83a2fbe})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for DiscretePointKeyFrame {
    type Vtable = IDiscretePointKeyFrame_Vtbl;
    const IID: ::windows_core::GUID = <IDiscretePointKeyFrame as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for DiscretePointKeyFrame {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.DiscretePointKeyFrame";
}
impl ::core::convert::From<DiscretePointKeyFrame> for ::windows_core::IUnknown {
    fn from(value: DiscretePointKeyFrame) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DiscretePointKeyFrame> for ::windows_core::IUnknown {
    fn from(value: &DiscretePointKeyFrame) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for DiscretePointKeyFrame {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a DiscretePointKeyFrame {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<DiscretePointKeyFrame> for ::windows_core::IInspectable {
    fn from(value: DiscretePointKeyFrame) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DiscretePointKeyFrame> for ::windows_core::IInspectable {
    fn from(value: &DiscretePointKeyFrame) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for DiscretePointKeyFrame {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a DiscretePointKeyFrame {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<DiscretePointKeyFrame> for PointKeyFrame {
    fn from(value: DiscretePointKeyFrame) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&DiscretePointKeyFrame> for PointKeyFrame {
    fn from(value: &DiscretePointKeyFrame) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, PointKeyFrame> for DiscretePointKeyFrame {
    fn into_param(self) -> ::windows_core::Param<'a, PointKeyFrame> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, PointKeyFrame> for &DiscretePointKeyFrame {
    fn into_param(self) -> ::windows_core::Param<'a, PointKeyFrame> {
        ::windows_core::Param::Owned(::core::convert::Into::<PointKeyFrame>::into(self))
    }
}
impl ::core::convert::From<DiscretePointKeyFrame> for super::super::DependencyObject {
    fn from(value: DiscretePointKeyFrame) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&DiscretePointKeyFrame> for super::super::DependencyObject {
    fn from(value: &DiscretePointKeyFrame) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::DependencyObject> for DiscretePointKeyFrame {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::DependencyObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::DependencyObject> for &DiscretePointKeyFrame {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::DependencyObject> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for DiscretePointKeyFrame {}
unsafe impl ::core::marker::Sync for DiscretePointKeyFrame {}
#[repr(transparent)]
pub struct DoubleAnimation(::windows_core::IUnknown);
impl DoubleAnimation {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<DoubleAnimation, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn From(&self) -> ::windows_core::Result<::winrt_foundation::IReference<f64>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).From)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IReference<f64>>(result__)
        }
    }
    pub fn SetFrom<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::IReference<f64>>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetFrom)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn To(&self) -> ::windows_core::Result<::winrt_foundation::IReference<f64>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).To)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IReference<f64>>(result__)
        }
    }
    pub fn SetTo<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::IReference<f64>>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetTo)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn By(&self) -> ::windows_core::Result<::winrt_foundation::IReference<f64>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).By)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IReference<f64>>(result__)
        }
    }
    pub fn SetBy<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::IReference<f64>>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetBy)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn EasingFunction(&self) -> ::windows_core::Result<EasingFunctionBase> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).EasingFunction)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<EasingFunctionBase>(result__)
        }
    }
    pub fn SetEasingFunction<'a, Param0: ::windows_core::IntoParam<'a, EasingFunctionBase>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetEasingFunction)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn EnableDependentAnimation(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).EnableDependentAnimation)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetEnableDependentAnimation(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetEnableDependentAnimation)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn FromProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::IDoubleAnimationStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FromProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn ToProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::IDoubleAnimationStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ToProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn ByProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::IDoubleAnimationStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ByProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn EasingFunctionProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::IDoubleAnimationStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).EasingFunctionProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn EnableDependentAnimationProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::IDoubleAnimationStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).EnableDependentAnimationProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn IDoubleAnimationStatics<R, F: FnOnce(&IDoubleAnimationStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<DoubleAnimation, IDoubleAnimationStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for DoubleAnimation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for DoubleAnimation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DoubleAnimation {}
impl ::core::fmt::Debug for DoubleAnimation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DoubleAnimation").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for DoubleAnimation {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Animation.DoubleAnimation;{7e9f3d59-0f07-4bc9-977d-03763ff8154f})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for DoubleAnimation {
    type Vtable = IDoubleAnimation_Vtbl;
    const IID: ::windows_core::GUID = <IDoubleAnimation as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for DoubleAnimation {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.DoubleAnimation";
}
impl ::core::convert::From<DoubleAnimation> for ::windows_core::IUnknown {
    fn from(value: DoubleAnimation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DoubleAnimation> for ::windows_core::IUnknown {
    fn from(value: &DoubleAnimation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for DoubleAnimation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a DoubleAnimation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<DoubleAnimation> for ::windows_core::IInspectable {
    fn from(value: DoubleAnimation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DoubleAnimation> for ::windows_core::IInspectable {
    fn from(value: &DoubleAnimation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for DoubleAnimation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a DoubleAnimation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<DoubleAnimation> for Timeline {
    fn from(value: DoubleAnimation) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&DoubleAnimation> for Timeline {
    fn from(value: &DoubleAnimation) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, Timeline> for DoubleAnimation {
    fn into_param(self) -> ::windows_core::Param<'a, Timeline> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, Timeline> for &DoubleAnimation {
    fn into_param(self) -> ::windows_core::Param<'a, Timeline> {
        ::windows_core::Param::Owned(::core::convert::Into::<Timeline>::into(self))
    }
}
impl ::core::convert::From<DoubleAnimation> for super::super::DependencyObject {
    fn from(value: DoubleAnimation) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&DoubleAnimation> for super::super::DependencyObject {
    fn from(value: &DoubleAnimation) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::DependencyObject> for DoubleAnimation {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::DependencyObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::DependencyObject> for &DoubleAnimation {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::DependencyObject> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for DoubleAnimation {}
unsafe impl ::core::marker::Sync for DoubleAnimation {}
#[repr(transparent)]
pub struct DoubleAnimationUsingKeyFrames(::windows_core::IUnknown);
impl DoubleAnimationUsingKeyFrames {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<DoubleAnimationUsingKeyFrames, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn KeyFrames(&self) -> ::windows_core::Result<DoubleKeyFrameCollection> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).KeyFrames)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<DoubleKeyFrameCollection>(result__)
        }
    }
    pub fn EnableDependentAnimation(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).EnableDependentAnimation)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetEnableDependentAnimation(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetEnableDependentAnimation)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn EnableDependentAnimationProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::IDoubleAnimationUsingKeyFramesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).EnableDependentAnimationProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn IDoubleAnimationUsingKeyFramesStatics<R, F: FnOnce(&IDoubleAnimationUsingKeyFramesStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<DoubleAnimationUsingKeyFrames, IDoubleAnimationUsingKeyFramesStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for DoubleAnimationUsingKeyFrames {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for DoubleAnimationUsingKeyFrames {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DoubleAnimationUsingKeyFrames {}
impl ::core::fmt::Debug for DoubleAnimationUsingKeyFrames {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DoubleAnimationUsingKeyFrames").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for DoubleAnimationUsingKeyFrames {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Animation.DoubleAnimationUsingKeyFrames;{4fee628f-bfee-4f75-83c2-a93b39488473})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for DoubleAnimationUsingKeyFrames {
    type Vtable = IDoubleAnimationUsingKeyFrames_Vtbl;
    const IID: ::windows_core::GUID = <IDoubleAnimationUsingKeyFrames as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for DoubleAnimationUsingKeyFrames {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.DoubleAnimationUsingKeyFrames";
}
impl ::core::convert::From<DoubleAnimationUsingKeyFrames> for ::windows_core::IUnknown {
    fn from(value: DoubleAnimationUsingKeyFrames) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DoubleAnimationUsingKeyFrames> for ::windows_core::IUnknown {
    fn from(value: &DoubleAnimationUsingKeyFrames) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for DoubleAnimationUsingKeyFrames {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a DoubleAnimationUsingKeyFrames {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<DoubleAnimationUsingKeyFrames> for ::windows_core::IInspectable {
    fn from(value: DoubleAnimationUsingKeyFrames) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DoubleAnimationUsingKeyFrames> for ::windows_core::IInspectable {
    fn from(value: &DoubleAnimationUsingKeyFrames) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for DoubleAnimationUsingKeyFrames {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a DoubleAnimationUsingKeyFrames {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<DoubleAnimationUsingKeyFrames> for Timeline {
    fn from(value: DoubleAnimationUsingKeyFrames) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&DoubleAnimationUsingKeyFrames> for Timeline {
    fn from(value: &DoubleAnimationUsingKeyFrames) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, Timeline> for DoubleAnimationUsingKeyFrames {
    fn into_param(self) -> ::windows_core::Param<'a, Timeline> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, Timeline> for &DoubleAnimationUsingKeyFrames {
    fn into_param(self) -> ::windows_core::Param<'a, Timeline> {
        ::windows_core::Param::Owned(::core::convert::Into::<Timeline>::into(self))
    }
}
impl ::core::convert::From<DoubleAnimationUsingKeyFrames> for super::super::DependencyObject {
    fn from(value: DoubleAnimationUsingKeyFrames) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&DoubleAnimationUsingKeyFrames> for super::super::DependencyObject {
    fn from(value: &DoubleAnimationUsingKeyFrames) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::DependencyObject> for DoubleAnimationUsingKeyFrames {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::DependencyObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::DependencyObject> for &DoubleAnimationUsingKeyFrames {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::DependencyObject> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for DoubleAnimationUsingKeyFrames {}
unsafe impl ::core::marker::Sync for DoubleAnimationUsingKeyFrames {}
#[repr(transparent)]
pub struct DoubleKeyFrame(::windows_core::IUnknown);
impl DoubleKeyFrame {
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
    pub fn KeyTime(&self) -> ::windows_core::Result<KeyTime> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<KeyTime>::zeroed();
            (::windows_core::Interface::vtable(this).KeyTime)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<KeyTime>(result__)
        }
    }
    pub fn SetKeyTime<'a, Param0: ::windows_core::IntoParam<'a, KeyTime>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetKeyTime)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn ValueProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::IDoubleKeyFrameStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ValueProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn KeyTimeProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::IDoubleKeyFrameStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).KeyTimeProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn IDoubleKeyFrameStatics<R, F: FnOnce(&IDoubleKeyFrameStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<DoubleKeyFrame, IDoubleKeyFrameStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for DoubleKeyFrame {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for DoubleKeyFrame {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DoubleKeyFrame {}
impl ::core::fmt::Debug for DoubleKeyFrame {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DoubleKeyFrame").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for DoubleKeyFrame {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Animation.DoubleKeyFrame;{674456fd-e81e-4f4e-b4ad-0acfed9ecd68})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for DoubleKeyFrame {
    type Vtable = IDoubleKeyFrame_Vtbl;
    const IID: ::windows_core::GUID = <IDoubleKeyFrame as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for DoubleKeyFrame {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.DoubleKeyFrame";
}
impl ::core::convert::From<DoubleKeyFrame> for ::windows_core::IUnknown {
    fn from(value: DoubleKeyFrame) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DoubleKeyFrame> for ::windows_core::IUnknown {
    fn from(value: &DoubleKeyFrame) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for DoubleKeyFrame {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a DoubleKeyFrame {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<DoubleKeyFrame> for ::windows_core::IInspectable {
    fn from(value: DoubleKeyFrame) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DoubleKeyFrame> for ::windows_core::IInspectable {
    fn from(value: &DoubleKeyFrame) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for DoubleKeyFrame {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a DoubleKeyFrame {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<DoubleKeyFrame> for super::super::DependencyObject {
    fn from(value: DoubleKeyFrame) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&DoubleKeyFrame> for super::super::DependencyObject {
    fn from(value: &DoubleKeyFrame) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::DependencyObject> for DoubleKeyFrame {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::DependencyObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::DependencyObject> for &DoubleKeyFrame {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::DependencyObject> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for DoubleKeyFrame {}
unsafe impl ::core::marker::Sync for DoubleKeyFrame {}
#[cfg(feature = "winrt-foundation")]
#[repr(transparent)]
pub struct DoubleKeyFrameCollection(::windows_core::IUnknown);
#[cfg(feature = "winrt-foundation")]
impl DoubleKeyFrameCollection {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<DoubleKeyFrameCollection, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn First(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IIterator<DoubleKeyFrame>> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IIterable<DoubleKeyFrame>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).First)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IIterator<DoubleKeyFrame>>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn GetAt(&self, index: u32) -> ::windows_core::Result<DoubleKeyFrame> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetAt)(::windows_core::Interface::as_raw(this), index, result__.as_mut_ptr()).from_abi::<DoubleKeyFrame>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn Size(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).Size)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn GetView(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<DoubleKeyFrame>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetView)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<DoubleKeyFrame>>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn IndexOf<'a, Param0: ::windows_core::IntoParam<'a, DoubleKeyFrame>>(&self, value: Param0, index: &mut u32) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IndexOf)(::windows_core::Interface::as_raw(this), value.into_param().abi(), index, result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn SetAt<'a, Param1: ::windows_core::IntoParam<'a, DoubleKeyFrame>>(&self, index: u32, value: Param1) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetAt)(::windows_core::Interface::as_raw(this), index, value.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn InsertAt<'a, Param1: ::windows_core::IntoParam<'a, DoubleKeyFrame>>(&self, index: u32, value: Param1) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).InsertAt)(::windows_core::Interface::as_raw(this), index, value.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn RemoveAt(&self, index: u32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveAt)(::windows_core::Interface::as_raw(this), index).ok() }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn Append<'a, Param0: ::windows_core::IntoParam<'a, DoubleKeyFrame>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Append)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn RemoveAtEnd(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveAtEnd)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn Clear(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Clear)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn GetMany(&self, startindex: u32, items: &mut [::core::option::Option<DoubleKeyFrame>]) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).GetMany)(::windows_core::Interface::as_raw(this), startindex, items.len() as u32, ::core::mem::transmute_copy(&items), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn ReplaceAll(&self, items: &[::core::option::Option<DoubleKeyFrame>]) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).ReplaceAll)(::windows_core::Interface::as_raw(this), items.len() as u32, ::core::mem::transmute(items.as_ptr())).ok() }
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::clone::Clone for DoubleKeyFrameCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::cmp::PartialEq for DoubleKeyFrameCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::cmp::Eq for DoubleKeyFrameCollection {}
#[cfg(feature = "winrt-foundation")]
impl ::core::fmt::Debug for DoubleKeyFrameCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DoubleKeyFrameCollection").field(&self.0).finish()
    }
}
#[cfg(feature = "winrt-foundation")]
unsafe impl ::windows_core::RuntimeType for DoubleKeyFrameCollection {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Animation.DoubleKeyFrameCollection;pinterface({913337e9-11a1-4345-a3a2-4e7f956e222d};rc(Windows.UI.Xaml.Media.Animation.DoubleKeyFrame;{674456fd-e81e-4f4e-b4ad-0acfed9ecd68})))");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[cfg(feature = "winrt-foundation")]
unsafe impl ::windows_core::Interface for DoubleKeyFrameCollection {
    type Vtable = ::winrt_foundation::Collections::IVector_Vtbl<DoubleKeyFrame>;
    const IID: ::windows_core::GUID = <::winrt_foundation::Collections::IVector<DoubleKeyFrame> as ::windows_core::Interface>::IID;
}
#[cfg(feature = "winrt-foundation")]
impl ::windows_core::RuntimeName for DoubleKeyFrameCollection {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.DoubleKeyFrameCollection";
}
#[cfg(feature = "winrt-foundation")]
impl ::core::iter::IntoIterator for DoubleKeyFrameCollection {
    type Item = DoubleKeyFrame;
    type IntoIter = ::winrt_foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::iter::IntoIterator for &DoubleKeyFrameCollection {
    type Item = DoubleKeyFrame;
    type IntoIter = ::winrt_foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::winrt_foundation::Collections::VectorIterator::new(::core::convert::TryInto::try_into(self).ok())
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::From<DoubleKeyFrameCollection> for ::windows_core::IUnknown {
    fn from(value: DoubleKeyFrameCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::From<&DoubleKeyFrameCollection> for ::windows_core::IUnknown {
    fn from(value: &DoubleKeyFrameCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for DoubleKeyFrameCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a DoubleKeyFrameCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::From<DoubleKeyFrameCollection> for ::windows_core::IInspectable {
    fn from(value: DoubleKeyFrameCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::From<&DoubleKeyFrameCollection> for ::windows_core::IInspectable {
    fn from(value: &DoubleKeyFrameCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for DoubleKeyFrameCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a DoubleKeyFrameCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::TryFrom<DoubleKeyFrameCollection> for ::winrt_foundation::Collections::IIterable<DoubleKeyFrame> {
    type Error = ::windows_core::Error;
    fn try_from(value: DoubleKeyFrameCollection) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::TryFrom<&DoubleKeyFrameCollection> for ::winrt_foundation::Collections::IIterable<DoubleKeyFrame> {
    type Error = ::windows_core::Error;
    fn try_from(value: &DoubleKeyFrameCollection) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<DoubleKeyFrame>> for DoubleKeyFrameCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::Collections::IIterable<DoubleKeyFrame>> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<DoubleKeyFrame>> for &DoubleKeyFrameCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::Collections::IIterable<DoubleKeyFrame>> {
        ::core::convert::TryInto::<::winrt_foundation::Collections::IIterable<DoubleKeyFrame>>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::TryFrom<DoubleKeyFrameCollection> for ::winrt_foundation::Collections::IVector<DoubleKeyFrame> {
    type Error = ::windows_core::Error;
    fn try_from(value: DoubleKeyFrameCollection) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::TryFrom<&DoubleKeyFrameCollection> for ::winrt_foundation::Collections::IVector<DoubleKeyFrame> {
    type Error = ::windows_core::Error;
    fn try_from(value: &DoubleKeyFrameCollection) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IVector<DoubleKeyFrame>> for DoubleKeyFrameCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::Collections::IVector<DoubleKeyFrame>> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IVector<DoubleKeyFrame>> for &DoubleKeyFrameCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::Collections::IVector<DoubleKeyFrame>> {
        ::core::convert::TryInto::<::winrt_foundation::Collections::IVector<DoubleKeyFrame>>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "winrt-foundation")]
unsafe impl ::core::marker::Send for DoubleKeyFrameCollection {}
#[cfg(feature = "winrt-foundation")]
unsafe impl ::core::marker::Sync for DoubleKeyFrameCollection {}
#[repr(transparent)]
pub struct DragItemThemeAnimation(::windows_core::IUnknown);
impl DragItemThemeAnimation {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<DragItemThemeAnimation, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn TargetName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).TargetName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetTargetName<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetTargetName)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn TargetNameProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::IDragItemThemeAnimationStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TargetNameProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn IDragItemThemeAnimationStatics<R, F: FnOnce(&IDragItemThemeAnimationStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<DragItemThemeAnimation, IDragItemThemeAnimationStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for DragItemThemeAnimation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for DragItemThemeAnimation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DragItemThemeAnimation {}
impl ::core::fmt::Debug for DragItemThemeAnimation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DragItemThemeAnimation").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for DragItemThemeAnimation {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Animation.DragItemThemeAnimation;{0c7d5db5-7ed6-4949-b4e6-a78c9f4f978d})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for DragItemThemeAnimation {
    type Vtable = IDragItemThemeAnimation_Vtbl;
    const IID: ::windows_core::GUID = <IDragItemThemeAnimation as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for DragItemThemeAnimation {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.DragItemThemeAnimation";
}
impl ::core::convert::From<DragItemThemeAnimation> for ::windows_core::IUnknown {
    fn from(value: DragItemThemeAnimation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DragItemThemeAnimation> for ::windows_core::IUnknown {
    fn from(value: &DragItemThemeAnimation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for DragItemThemeAnimation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a DragItemThemeAnimation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<DragItemThemeAnimation> for ::windows_core::IInspectable {
    fn from(value: DragItemThemeAnimation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DragItemThemeAnimation> for ::windows_core::IInspectable {
    fn from(value: &DragItemThemeAnimation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for DragItemThemeAnimation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a DragItemThemeAnimation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<DragItemThemeAnimation> for Timeline {
    fn from(value: DragItemThemeAnimation) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&DragItemThemeAnimation> for Timeline {
    fn from(value: &DragItemThemeAnimation) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, Timeline> for DragItemThemeAnimation {
    fn into_param(self) -> ::windows_core::Param<'a, Timeline> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, Timeline> for &DragItemThemeAnimation {
    fn into_param(self) -> ::windows_core::Param<'a, Timeline> {
        ::windows_core::Param::Owned(::core::convert::Into::<Timeline>::into(self))
    }
}
impl ::core::convert::From<DragItemThemeAnimation> for super::super::DependencyObject {
    fn from(value: DragItemThemeAnimation) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&DragItemThemeAnimation> for super::super::DependencyObject {
    fn from(value: &DragItemThemeAnimation) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::DependencyObject> for DragItemThemeAnimation {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::DependencyObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::DependencyObject> for &DragItemThemeAnimation {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::DependencyObject> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for DragItemThemeAnimation {}
unsafe impl ::core::marker::Sync for DragItemThemeAnimation {}
#[repr(transparent)]
pub struct DragOverThemeAnimation(::windows_core::IUnknown);
impl DragOverThemeAnimation {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<DragOverThemeAnimation, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn TargetName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).TargetName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetTargetName<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetTargetName)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn ToOffset(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).ToOffset)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn SetToOffset(&self, value: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetToOffset)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "winrt-ui")]
    pub fn Direction(&self) -> ::windows_core::Result<super::super::Controls::Primitives::AnimationDirection> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::Controls::Primitives::AnimationDirection>::zeroed();
            (::windows_core::Interface::vtable(this).Direction)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Controls::Primitives::AnimationDirection>(result__)
        }
    }
    #[cfg(feature = "winrt-ui")]
    pub fn SetDirection(&self, value: super::super::Controls::Primitives::AnimationDirection) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetDirection)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn TargetNameProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::IDragOverThemeAnimationStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TargetNameProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn ToOffsetProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::IDragOverThemeAnimationStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ToOffsetProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn DirectionProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::IDragOverThemeAnimationStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).DirectionProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn IDragOverThemeAnimationStatics<R, F: FnOnce(&IDragOverThemeAnimationStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<DragOverThemeAnimation, IDragOverThemeAnimationStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for DragOverThemeAnimation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for DragOverThemeAnimation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DragOverThemeAnimation {}
impl ::core::fmt::Debug for DragOverThemeAnimation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DragOverThemeAnimation").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for DragOverThemeAnimation {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Animation.DragOverThemeAnimation;{72f762f7-7e51-4a6b-b937-dc4b4c1c5458})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for DragOverThemeAnimation {
    type Vtable = IDragOverThemeAnimation_Vtbl;
    const IID: ::windows_core::GUID = <IDragOverThemeAnimation as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for DragOverThemeAnimation {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.DragOverThemeAnimation";
}
impl ::core::convert::From<DragOverThemeAnimation> for ::windows_core::IUnknown {
    fn from(value: DragOverThemeAnimation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DragOverThemeAnimation> for ::windows_core::IUnknown {
    fn from(value: &DragOverThemeAnimation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for DragOverThemeAnimation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a DragOverThemeAnimation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<DragOverThemeAnimation> for ::windows_core::IInspectable {
    fn from(value: DragOverThemeAnimation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DragOverThemeAnimation> for ::windows_core::IInspectable {
    fn from(value: &DragOverThemeAnimation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for DragOverThemeAnimation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a DragOverThemeAnimation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<DragOverThemeAnimation> for Timeline {
    fn from(value: DragOverThemeAnimation) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&DragOverThemeAnimation> for Timeline {
    fn from(value: &DragOverThemeAnimation) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, Timeline> for DragOverThemeAnimation {
    fn into_param(self) -> ::windows_core::Param<'a, Timeline> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, Timeline> for &DragOverThemeAnimation {
    fn into_param(self) -> ::windows_core::Param<'a, Timeline> {
        ::windows_core::Param::Owned(::core::convert::Into::<Timeline>::into(self))
    }
}
impl ::core::convert::From<DragOverThemeAnimation> for super::super::DependencyObject {
    fn from(value: DragOverThemeAnimation) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&DragOverThemeAnimation> for super::super::DependencyObject {
    fn from(value: &DragOverThemeAnimation) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::DependencyObject> for DragOverThemeAnimation {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::DependencyObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::DependencyObject> for &DragOverThemeAnimation {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::DependencyObject> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for DragOverThemeAnimation {}
unsafe impl ::core::marker::Sync for DragOverThemeAnimation {}
#[repr(transparent)]
pub struct DrillInNavigationTransitionInfo(::windows_core::IUnknown);
impl DrillInNavigationTransitionInfo {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<DrillInNavigationTransitionInfo, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for DrillInNavigationTransitionInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for DrillInNavigationTransitionInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DrillInNavigationTransitionInfo {}
impl ::core::fmt::Debug for DrillInNavigationTransitionInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DrillInNavigationTransitionInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for DrillInNavigationTransitionInfo {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Animation.DrillInNavigationTransitionInfo;{3b86201a-45d3-463b-939e-c8595f439bcc})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for DrillInNavigationTransitionInfo {
    type Vtable = IDrillInNavigationTransitionInfo_Vtbl;
    const IID: ::windows_core::GUID = <IDrillInNavigationTransitionInfo as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for DrillInNavigationTransitionInfo {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.DrillInNavigationTransitionInfo";
}
impl ::core::convert::From<DrillInNavigationTransitionInfo> for ::windows_core::IUnknown {
    fn from(value: DrillInNavigationTransitionInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DrillInNavigationTransitionInfo> for ::windows_core::IUnknown {
    fn from(value: &DrillInNavigationTransitionInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for DrillInNavigationTransitionInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a DrillInNavigationTransitionInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<DrillInNavigationTransitionInfo> for ::windows_core::IInspectable {
    fn from(value: DrillInNavigationTransitionInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DrillInNavigationTransitionInfo> for ::windows_core::IInspectable {
    fn from(value: &DrillInNavigationTransitionInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for DrillInNavigationTransitionInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a DrillInNavigationTransitionInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<DrillInNavigationTransitionInfo> for NavigationTransitionInfo {
    fn from(value: DrillInNavigationTransitionInfo) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&DrillInNavigationTransitionInfo> for NavigationTransitionInfo {
    fn from(value: &DrillInNavigationTransitionInfo) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, NavigationTransitionInfo> for DrillInNavigationTransitionInfo {
    fn into_param(self) -> ::windows_core::Param<'a, NavigationTransitionInfo> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, NavigationTransitionInfo> for &DrillInNavigationTransitionInfo {
    fn into_param(self) -> ::windows_core::Param<'a, NavigationTransitionInfo> {
        ::windows_core::Param::Owned(::core::convert::Into::<NavigationTransitionInfo>::into(self))
    }
}
impl ::core::convert::From<DrillInNavigationTransitionInfo> for super::super::DependencyObject {
    fn from(value: DrillInNavigationTransitionInfo) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&DrillInNavigationTransitionInfo> for super::super::DependencyObject {
    fn from(value: &DrillInNavigationTransitionInfo) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::DependencyObject> for DrillInNavigationTransitionInfo {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::DependencyObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::DependencyObject> for &DrillInNavigationTransitionInfo {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::DependencyObject> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for DrillInNavigationTransitionInfo {}
unsafe impl ::core::marker::Sync for DrillInNavigationTransitionInfo {}
#[repr(transparent)]
pub struct DrillInThemeAnimation(::windows_core::IUnknown);
impl DrillInThemeAnimation {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<DrillInThemeAnimation, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn EntranceTargetName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).EntranceTargetName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetEntranceTargetName<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetEntranceTargetName)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn EntranceTarget(&self) -> ::windows_core::Result<super::super::DependencyObject> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).EntranceTarget)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyObject>(result__)
        }
    }
    pub fn SetEntranceTarget<'a, Param0: ::windows_core::IntoParam<'a, super::super::DependencyObject>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetEntranceTarget)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn ExitTargetName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).ExitTargetName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetExitTargetName<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetExitTargetName)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn ExitTarget(&self) -> ::windows_core::Result<super::super::DependencyObject> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ExitTarget)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyObject>(result__)
        }
    }
    pub fn SetExitTarget<'a, Param0: ::windows_core::IntoParam<'a, super::super::DependencyObject>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetExitTarget)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn EntranceTargetNameProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::IDrillInThemeAnimationStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).EntranceTargetNameProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn EntranceTargetProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::IDrillInThemeAnimationStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).EntranceTargetProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn ExitTargetNameProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::IDrillInThemeAnimationStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ExitTargetNameProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn ExitTargetProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::IDrillInThemeAnimationStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ExitTargetProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn IDrillInThemeAnimationStatics<R, F: FnOnce(&IDrillInThemeAnimationStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<DrillInThemeAnimation, IDrillInThemeAnimationStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for DrillInThemeAnimation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for DrillInThemeAnimation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DrillInThemeAnimation {}
impl ::core::fmt::Debug for DrillInThemeAnimation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DrillInThemeAnimation").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for DrillInThemeAnimation {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Animation.DrillInThemeAnimation;{b090b824-f1d2-41b8-87ba-78034126594c})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for DrillInThemeAnimation {
    type Vtable = IDrillInThemeAnimation_Vtbl;
    const IID: ::windows_core::GUID = <IDrillInThemeAnimation as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for DrillInThemeAnimation {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.DrillInThemeAnimation";
}
impl ::core::convert::From<DrillInThemeAnimation> for ::windows_core::IUnknown {
    fn from(value: DrillInThemeAnimation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DrillInThemeAnimation> for ::windows_core::IUnknown {
    fn from(value: &DrillInThemeAnimation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for DrillInThemeAnimation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a DrillInThemeAnimation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<DrillInThemeAnimation> for ::windows_core::IInspectable {
    fn from(value: DrillInThemeAnimation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DrillInThemeAnimation> for ::windows_core::IInspectable {
    fn from(value: &DrillInThemeAnimation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for DrillInThemeAnimation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a DrillInThemeAnimation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<DrillInThemeAnimation> for Timeline {
    fn from(value: DrillInThemeAnimation) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&DrillInThemeAnimation> for Timeline {
    fn from(value: &DrillInThemeAnimation) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, Timeline> for DrillInThemeAnimation {
    fn into_param(self) -> ::windows_core::Param<'a, Timeline> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, Timeline> for &DrillInThemeAnimation {
    fn into_param(self) -> ::windows_core::Param<'a, Timeline> {
        ::windows_core::Param::Owned(::core::convert::Into::<Timeline>::into(self))
    }
}
impl ::core::convert::From<DrillInThemeAnimation> for super::super::DependencyObject {
    fn from(value: DrillInThemeAnimation) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&DrillInThemeAnimation> for super::super::DependencyObject {
    fn from(value: &DrillInThemeAnimation) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::DependencyObject> for DrillInThemeAnimation {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::DependencyObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::DependencyObject> for &DrillInThemeAnimation {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::DependencyObject> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for DrillInThemeAnimation {}
unsafe impl ::core::marker::Sync for DrillInThemeAnimation {}
#[repr(transparent)]
pub struct DrillOutThemeAnimation(::windows_core::IUnknown);
impl DrillOutThemeAnimation {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<DrillOutThemeAnimation, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn EntranceTargetName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).EntranceTargetName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetEntranceTargetName<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetEntranceTargetName)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn EntranceTarget(&self) -> ::windows_core::Result<super::super::DependencyObject> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).EntranceTarget)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyObject>(result__)
        }
    }
    pub fn SetEntranceTarget<'a, Param0: ::windows_core::IntoParam<'a, super::super::DependencyObject>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetEntranceTarget)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn ExitTargetName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).ExitTargetName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetExitTargetName<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetExitTargetName)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn ExitTarget(&self) -> ::windows_core::Result<super::super::DependencyObject> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ExitTarget)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyObject>(result__)
        }
    }
    pub fn SetExitTarget<'a, Param0: ::windows_core::IntoParam<'a, super::super::DependencyObject>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetExitTarget)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn EntranceTargetNameProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::IDrillOutThemeAnimationStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).EntranceTargetNameProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn EntranceTargetProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::IDrillOutThemeAnimationStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).EntranceTargetProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn ExitTargetNameProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::IDrillOutThemeAnimationStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ExitTargetNameProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn ExitTargetProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::IDrillOutThemeAnimationStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ExitTargetProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn IDrillOutThemeAnimationStatics<R, F: FnOnce(&IDrillOutThemeAnimationStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<DrillOutThemeAnimation, IDrillOutThemeAnimationStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for DrillOutThemeAnimation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for DrillOutThemeAnimation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DrillOutThemeAnimation {}
impl ::core::fmt::Debug for DrillOutThemeAnimation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DrillOutThemeAnimation").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for DrillOutThemeAnimation {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Animation.DrillOutThemeAnimation;{d890ccdf-06d3-4f7e-8e4a-4fb76e256139})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for DrillOutThemeAnimation {
    type Vtable = IDrillOutThemeAnimation_Vtbl;
    const IID: ::windows_core::GUID = <IDrillOutThemeAnimation as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for DrillOutThemeAnimation {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.DrillOutThemeAnimation";
}
impl ::core::convert::From<DrillOutThemeAnimation> for ::windows_core::IUnknown {
    fn from(value: DrillOutThemeAnimation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DrillOutThemeAnimation> for ::windows_core::IUnknown {
    fn from(value: &DrillOutThemeAnimation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for DrillOutThemeAnimation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a DrillOutThemeAnimation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<DrillOutThemeAnimation> for ::windows_core::IInspectable {
    fn from(value: DrillOutThemeAnimation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DrillOutThemeAnimation> for ::windows_core::IInspectable {
    fn from(value: &DrillOutThemeAnimation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for DrillOutThemeAnimation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a DrillOutThemeAnimation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<DrillOutThemeAnimation> for Timeline {
    fn from(value: DrillOutThemeAnimation) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&DrillOutThemeAnimation> for Timeline {
    fn from(value: &DrillOutThemeAnimation) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, Timeline> for DrillOutThemeAnimation {
    fn into_param(self) -> ::windows_core::Param<'a, Timeline> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, Timeline> for &DrillOutThemeAnimation {
    fn into_param(self) -> ::windows_core::Param<'a, Timeline> {
        ::windows_core::Param::Owned(::core::convert::Into::<Timeline>::into(self))
    }
}
impl ::core::convert::From<DrillOutThemeAnimation> for super::super::DependencyObject {
    fn from(value: DrillOutThemeAnimation) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&DrillOutThemeAnimation> for super::super::DependencyObject {
    fn from(value: &DrillOutThemeAnimation) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::DependencyObject> for DrillOutThemeAnimation {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::DependencyObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::DependencyObject> for &DrillOutThemeAnimation {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::DependencyObject> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for DrillOutThemeAnimation {}
unsafe impl ::core::marker::Sync for DrillOutThemeAnimation {}
#[repr(transparent)]
pub struct DropTargetItemThemeAnimation(::windows_core::IUnknown);
impl DropTargetItemThemeAnimation {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<DropTargetItemThemeAnimation, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn TargetName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).TargetName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetTargetName<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetTargetName)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn TargetNameProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::IDropTargetItemThemeAnimationStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TargetNameProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn IDropTargetItemThemeAnimationStatics<R, F: FnOnce(&IDropTargetItemThemeAnimationStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<DropTargetItemThemeAnimation, IDropTargetItemThemeAnimationStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for DropTargetItemThemeAnimation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for DropTargetItemThemeAnimation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DropTargetItemThemeAnimation {}
impl ::core::fmt::Debug for DropTargetItemThemeAnimation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DropTargetItemThemeAnimation").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for DropTargetItemThemeAnimation {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Animation.DropTargetItemThemeAnimation;{1881c968-1824-462b-87e8-c357212b977b})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for DropTargetItemThemeAnimation {
    type Vtable = IDropTargetItemThemeAnimation_Vtbl;
    const IID: ::windows_core::GUID = <IDropTargetItemThemeAnimation as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for DropTargetItemThemeAnimation {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.DropTargetItemThemeAnimation";
}
impl ::core::convert::From<DropTargetItemThemeAnimation> for ::windows_core::IUnknown {
    fn from(value: DropTargetItemThemeAnimation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DropTargetItemThemeAnimation> for ::windows_core::IUnknown {
    fn from(value: &DropTargetItemThemeAnimation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for DropTargetItemThemeAnimation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a DropTargetItemThemeAnimation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<DropTargetItemThemeAnimation> for ::windows_core::IInspectable {
    fn from(value: DropTargetItemThemeAnimation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DropTargetItemThemeAnimation> for ::windows_core::IInspectable {
    fn from(value: &DropTargetItemThemeAnimation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for DropTargetItemThemeAnimation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a DropTargetItemThemeAnimation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<DropTargetItemThemeAnimation> for Timeline {
    fn from(value: DropTargetItemThemeAnimation) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&DropTargetItemThemeAnimation> for Timeline {
    fn from(value: &DropTargetItemThemeAnimation) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, Timeline> for DropTargetItemThemeAnimation {
    fn into_param(self) -> ::windows_core::Param<'a, Timeline> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, Timeline> for &DropTargetItemThemeAnimation {
    fn into_param(self) -> ::windows_core::Param<'a, Timeline> {
        ::windows_core::Param::Owned(::core::convert::Into::<Timeline>::into(self))
    }
}
impl ::core::convert::From<DropTargetItemThemeAnimation> for super::super::DependencyObject {
    fn from(value: DropTargetItemThemeAnimation) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&DropTargetItemThemeAnimation> for super::super::DependencyObject {
    fn from(value: &DropTargetItemThemeAnimation) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::DependencyObject> for DropTargetItemThemeAnimation {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::DependencyObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::DependencyObject> for &DropTargetItemThemeAnimation {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::DependencyObject> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for DropTargetItemThemeAnimation {}
unsafe impl ::core::marker::Sync for DropTargetItemThemeAnimation {}
#[repr(transparent)]
pub struct EasingColorKeyFrame(::windows_core::IUnknown);
impl EasingColorKeyFrame {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<EasingColorKeyFrame, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn EasingFunction(&self) -> ::windows_core::Result<EasingFunctionBase> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).EasingFunction)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<EasingFunctionBase>(result__)
        }
    }
    pub fn SetEasingFunction<'a, Param0: ::windows_core::IntoParam<'a, EasingFunctionBase>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetEasingFunction)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn EasingFunctionProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::IEasingColorKeyFrameStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).EasingFunctionProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn IEasingColorKeyFrameStatics<R, F: FnOnce(&IEasingColorKeyFrameStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<EasingColorKeyFrame, IEasingColorKeyFrameStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for EasingColorKeyFrame {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for EasingColorKeyFrame {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for EasingColorKeyFrame {}
impl ::core::fmt::Debug for EasingColorKeyFrame {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EasingColorKeyFrame").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for EasingColorKeyFrame {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Animation.EasingColorKeyFrame;{c733d630-f4b9-4934-9bdd-27ac5ed1cfd8})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for EasingColorKeyFrame {
    type Vtable = IEasingColorKeyFrame_Vtbl;
    const IID: ::windows_core::GUID = <IEasingColorKeyFrame as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for EasingColorKeyFrame {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.EasingColorKeyFrame";
}
impl ::core::convert::From<EasingColorKeyFrame> for ::windows_core::IUnknown {
    fn from(value: EasingColorKeyFrame) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&EasingColorKeyFrame> for ::windows_core::IUnknown {
    fn from(value: &EasingColorKeyFrame) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for EasingColorKeyFrame {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a EasingColorKeyFrame {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<EasingColorKeyFrame> for ::windows_core::IInspectable {
    fn from(value: EasingColorKeyFrame) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&EasingColorKeyFrame> for ::windows_core::IInspectable {
    fn from(value: &EasingColorKeyFrame) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for EasingColorKeyFrame {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a EasingColorKeyFrame {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<EasingColorKeyFrame> for ColorKeyFrame {
    fn from(value: EasingColorKeyFrame) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&EasingColorKeyFrame> for ColorKeyFrame {
    fn from(value: &EasingColorKeyFrame) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, ColorKeyFrame> for EasingColorKeyFrame {
    fn into_param(self) -> ::windows_core::Param<'a, ColorKeyFrame> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ColorKeyFrame> for &EasingColorKeyFrame {
    fn into_param(self) -> ::windows_core::Param<'a, ColorKeyFrame> {
        ::windows_core::Param::Owned(::core::convert::Into::<ColorKeyFrame>::into(self))
    }
}
impl ::core::convert::From<EasingColorKeyFrame> for super::super::DependencyObject {
    fn from(value: EasingColorKeyFrame) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&EasingColorKeyFrame> for super::super::DependencyObject {
    fn from(value: &EasingColorKeyFrame) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::DependencyObject> for EasingColorKeyFrame {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::DependencyObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::DependencyObject> for &EasingColorKeyFrame {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::DependencyObject> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for EasingColorKeyFrame {}
unsafe impl ::core::marker::Sync for EasingColorKeyFrame {}
#[repr(transparent)]
pub struct EasingDoubleKeyFrame(::windows_core::IUnknown);
impl EasingDoubleKeyFrame {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<EasingDoubleKeyFrame, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn EasingFunction(&self) -> ::windows_core::Result<EasingFunctionBase> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).EasingFunction)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<EasingFunctionBase>(result__)
        }
    }
    pub fn SetEasingFunction<'a, Param0: ::windows_core::IntoParam<'a, EasingFunctionBase>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetEasingFunction)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn EasingFunctionProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::IEasingDoubleKeyFrameStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).EasingFunctionProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn IEasingDoubleKeyFrameStatics<R, F: FnOnce(&IEasingDoubleKeyFrameStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<EasingDoubleKeyFrame, IEasingDoubleKeyFrameStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for EasingDoubleKeyFrame {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for EasingDoubleKeyFrame {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for EasingDoubleKeyFrame {}
impl ::core::fmt::Debug for EasingDoubleKeyFrame {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EasingDoubleKeyFrame").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for EasingDoubleKeyFrame {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Animation.EasingDoubleKeyFrame;{965adb8d-9a54-4108-b4ff-b5a5212cb338})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for EasingDoubleKeyFrame {
    type Vtable = IEasingDoubleKeyFrame_Vtbl;
    const IID: ::windows_core::GUID = <IEasingDoubleKeyFrame as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for EasingDoubleKeyFrame {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.EasingDoubleKeyFrame";
}
impl ::core::convert::From<EasingDoubleKeyFrame> for ::windows_core::IUnknown {
    fn from(value: EasingDoubleKeyFrame) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&EasingDoubleKeyFrame> for ::windows_core::IUnknown {
    fn from(value: &EasingDoubleKeyFrame) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for EasingDoubleKeyFrame {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a EasingDoubleKeyFrame {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<EasingDoubleKeyFrame> for ::windows_core::IInspectable {
    fn from(value: EasingDoubleKeyFrame) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&EasingDoubleKeyFrame> for ::windows_core::IInspectable {
    fn from(value: &EasingDoubleKeyFrame) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for EasingDoubleKeyFrame {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a EasingDoubleKeyFrame {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<EasingDoubleKeyFrame> for DoubleKeyFrame {
    fn from(value: EasingDoubleKeyFrame) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&EasingDoubleKeyFrame> for DoubleKeyFrame {
    fn from(value: &EasingDoubleKeyFrame) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, DoubleKeyFrame> for EasingDoubleKeyFrame {
    fn into_param(self) -> ::windows_core::Param<'a, DoubleKeyFrame> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, DoubleKeyFrame> for &EasingDoubleKeyFrame {
    fn into_param(self) -> ::windows_core::Param<'a, DoubleKeyFrame> {
        ::windows_core::Param::Owned(::core::convert::Into::<DoubleKeyFrame>::into(self))
    }
}
impl ::core::convert::From<EasingDoubleKeyFrame> for super::super::DependencyObject {
    fn from(value: EasingDoubleKeyFrame) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&EasingDoubleKeyFrame> for super::super::DependencyObject {
    fn from(value: &EasingDoubleKeyFrame) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::DependencyObject> for EasingDoubleKeyFrame {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::DependencyObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::DependencyObject> for &EasingDoubleKeyFrame {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::DependencyObject> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for EasingDoubleKeyFrame {}
unsafe impl ::core::marker::Sync for EasingDoubleKeyFrame {}
#[repr(transparent)]
pub struct EasingFunctionBase(::windows_core::IUnknown);
impl EasingFunctionBase {
    pub fn EasingMode(&self) -> ::windows_core::Result<EasingMode> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<EasingMode>::zeroed();
            (::windows_core::Interface::vtable(this).EasingMode)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<EasingMode>(result__)
        }
    }
    pub fn SetEasingMode(&self, value: EasingMode) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetEasingMode)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Ease(&self, normalizedtime: f64) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).Ease)(::windows_core::Interface::as_raw(this), normalizedtime, result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn EasingModeProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::IEasingFunctionBaseStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).EasingModeProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn IEasingFunctionBaseStatics<R, F: FnOnce(&IEasingFunctionBaseStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<EasingFunctionBase, IEasingFunctionBaseStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for EasingFunctionBase {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for EasingFunctionBase {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for EasingFunctionBase {}
impl ::core::fmt::Debug for EasingFunctionBase {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EasingFunctionBase").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for EasingFunctionBase {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Animation.EasingFunctionBase;{c108383f-2c02-4151-8ecd-68ddaa3f0d9b})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for EasingFunctionBase {
    type Vtable = IEasingFunctionBase_Vtbl;
    const IID: ::windows_core::GUID = <IEasingFunctionBase as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for EasingFunctionBase {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.EasingFunctionBase";
}
impl ::core::convert::From<EasingFunctionBase> for ::windows_core::IUnknown {
    fn from(value: EasingFunctionBase) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&EasingFunctionBase> for ::windows_core::IUnknown {
    fn from(value: &EasingFunctionBase) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for EasingFunctionBase {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a EasingFunctionBase {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<EasingFunctionBase> for ::windows_core::IInspectable {
    fn from(value: EasingFunctionBase) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&EasingFunctionBase> for ::windows_core::IInspectable {
    fn from(value: &EasingFunctionBase) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for EasingFunctionBase {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a EasingFunctionBase {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<EasingFunctionBase> for super::super::DependencyObject {
    fn from(value: EasingFunctionBase) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&EasingFunctionBase> for super::super::DependencyObject {
    fn from(value: &EasingFunctionBase) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::DependencyObject> for EasingFunctionBase {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::DependencyObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::DependencyObject> for &EasingFunctionBase {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::DependencyObject> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for EasingFunctionBase {}
unsafe impl ::core::marker::Sync for EasingFunctionBase {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct EasingMode(pub i32);
impl EasingMode {
    pub const EaseOut: Self = Self(0i32);
    pub const EaseIn: Self = Self(1i32);
    pub const EaseInOut: Self = Self(2i32);
}
impl ::core::marker::Copy for EasingMode {}
impl ::core::clone::Clone for EasingMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for EasingMode {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for EasingMode {
    type Abi = Self;
}
impl ::core::fmt::Debug for EasingMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EasingMode").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for EasingMode {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Media.Animation.EasingMode;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct EasingPointKeyFrame(::windows_core::IUnknown);
impl EasingPointKeyFrame {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<EasingPointKeyFrame, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn EasingFunction(&self) -> ::windows_core::Result<EasingFunctionBase> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).EasingFunction)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<EasingFunctionBase>(result__)
        }
    }
    pub fn SetEasingFunction<'a, Param0: ::windows_core::IntoParam<'a, EasingFunctionBase>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetEasingFunction)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn EasingFunctionProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::IEasingPointKeyFrameStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).EasingFunctionProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn IEasingPointKeyFrameStatics<R, F: FnOnce(&IEasingPointKeyFrameStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<EasingPointKeyFrame, IEasingPointKeyFrameStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for EasingPointKeyFrame {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for EasingPointKeyFrame {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for EasingPointKeyFrame {}
impl ::core::fmt::Debug for EasingPointKeyFrame {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EasingPointKeyFrame").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for EasingPointKeyFrame {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Animation.EasingPointKeyFrame;{b3c91380-6868-4225-a70b-3981cc0b2947})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for EasingPointKeyFrame {
    type Vtable = IEasingPointKeyFrame_Vtbl;
    const IID: ::windows_core::GUID = <IEasingPointKeyFrame as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for EasingPointKeyFrame {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.EasingPointKeyFrame";
}
impl ::core::convert::From<EasingPointKeyFrame> for ::windows_core::IUnknown {
    fn from(value: EasingPointKeyFrame) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&EasingPointKeyFrame> for ::windows_core::IUnknown {
    fn from(value: &EasingPointKeyFrame) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for EasingPointKeyFrame {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a EasingPointKeyFrame {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<EasingPointKeyFrame> for ::windows_core::IInspectable {
    fn from(value: EasingPointKeyFrame) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&EasingPointKeyFrame> for ::windows_core::IInspectable {
    fn from(value: &EasingPointKeyFrame) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for EasingPointKeyFrame {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a EasingPointKeyFrame {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<EasingPointKeyFrame> for PointKeyFrame {
    fn from(value: EasingPointKeyFrame) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&EasingPointKeyFrame> for PointKeyFrame {
    fn from(value: &EasingPointKeyFrame) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, PointKeyFrame> for EasingPointKeyFrame {
    fn into_param(self) -> ::windows_core::Param<'a, PointKeyFrame> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, PointKeyFrame> for &EasingPointKeyFrame {
    fn into_param(self) -> ::windows_core::Param<'a, PointKeyFrame> {
        ::windows_core::Param::Owned(::core::convert::Into::<PointKeyFrame>::into(self))
    }
}
impl ::core::convert::From<EasingPointKeyFrame> for super::super::DependencyObject {
    fn from(value: EasingPointKeyFrame) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&EasingPointKeyFrame> for super::super::DependencyObject {
    fn from(value: &EasingPointKeyFrame) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::DependencyObject> for EasingPointKeyFrame {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::DependencyObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::DependencyObject> for &EasingPointKeyFrame {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::DependencyObject> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for EasingPointKeyFrame {}
unsafe impl ::core::marker::Sync for EasingPointKeyFrame {}
#[repr(transparent)]
pub struct EdgeUIThemeTransition(::windows_core::IUnknown);
impl EdgeUIThemeTransition {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<EdgeUIThemeTransition, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "winrt-ui")]
    pub fn Edge(&self) -> ::windows_core::Result<super::super::Controls::Primitives::EdgeTransitionLocation> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::Controls::Primitives::EdgeTransitionLocation>::zeroed();
            (::windows_core::Interface::vtable(this).Edge)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Controls::Primitives::EdgeTransitionLocation>(result__)
        }
    }
    #[cfg(feature = "winrt-ui")]
    pub fn SetEdge(&self, value: super::super::Controls::Primitives::EdgeTransitionLocation) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetEdge)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn EdgeProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::IEdgeUIThemeTransitionStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).EdgeProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn IEdgeUIThemeTransitionStatics<R, F: FnOnce(&IEdgeUIThemeTransitionStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<EdgeUIThemeTransition, IEdgeUIThemeTransitionStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for EdgeUIThemeTransition {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for EdgeUIThemeTransition {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for EdgeUIThemeTransition {}
impl ::core::fmt::Debug for EdgeUIThemeTransition {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EdgeUIThemeTransition").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for EdgeUIThemeTransition {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Animation.EdgeUIThemeTransition;{5c86c19b-49d7-19ec-cf19-83a73c6de75e})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for EdgeUIThemeTransition {
    type Vtable = IEdgeUIThemeTransition_Vtbl;
    const IID: ::windows_core::GUID = <IEdgeUIThemeTransition as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for EdgeUIThemeTransition {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.EdgeUIThemeTransition";
}
impl ::core::convert::From<EdgeUIThemeTransition> for ::windows_core::IUnknown {
    fn from(value: EdgeUIThemeTransition) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&EdgeUIThemeTransition> for ::windows_core::IUnknown {
    fn from(value: &EdgeUIThemeTransition) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for EdgeUIThemeTransition {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a EdgeUIThemeTransition {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<EdgeUIThemeTransition> for ::windows_core::IInspectable {
    fn from(value: EdgeUIThemeTransition) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&EdgeUIThemeTransition> for ::windows_core::IInspectable {
    fn from(value: &EdgeUIThemeTransition) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for EdgeUIThemeTransition {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a EdgeUIThemeTransition {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<EdgeUIThemeTransition> for Transition {
    fn from(value: EdgeUIThemeTransition) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&EdgeUIThemeTransition> for Transition {
    fn from(value: &EdgeUIThemeTransition) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, Transition> for EdgeUIThemeTransition {
    fn into_param(self) -> ::windows_core::Param<'a, Transition> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, Transition> for &EdgeUIThemeTransition {
    fn into_param(self) -> ::windows_core::Param<'a, Transition> {
        ::windows_core::Param::Owned(::core::convert::Into::<Transition>::into(self))
    }
}
impl ::core::convert::From<EdgeUIThemeTransition> for super::super::DependencyObject {
    fn from(value: EdgeUIThemeTransition) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&EdgeUIThemeTransition> for super::super::DependencyObject {
    fn from(value: &EdgeUIThemeTransition) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::DependencyObject> for EdgeUIThemeTransition {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::DependencyObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::DependencyObject> for &EdgeUIThemeTransition {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::DependencyObject> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for EdgeUIThemeTransition {}
unsafe impl ::core::marker::Sync for EdgeUIThemeTransition {}
#[repr(transparent)]
pub struct ElasticEase(::windows_core::IUnknown);
impl ElasticEase {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<ElasticEase, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn Oscillations(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows_core::Interface::vtable(this).Oscillations)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    pub fn SetOscillations(&self, value: i32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetOscillations)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Springiness(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).Springiness)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn SetSpringiness(&self, value: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetSpringiness)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn OscillationsProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::IElasticEaseStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).OscillationsProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn SpringinessProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::IElasticEaseStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SpringinessProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn IElasticEaseStatics<R, F: FnOnce(&IElasticEaseStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<ElasticEase, IElasticEaseStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for ElasticEase {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ElasticEase {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ElasticEase {}
impl ::core::fmt::Debug for ElasticEase {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ElasticEase").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ElasticEase {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Animation.ElasticEase;{ef5ba58c-b0b6-4a6c-9ca8-fb4233f12459})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for ElasticEase {
    type Vtable = IElasticEase_Vtbl;
    const IID: ::windows_core::GUID = <IElasticEase as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for ElasticEase {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.ElasticEase";
}
impl ::core::convert::From<ElasticEase> for ::windows_core::IUnknown {
    fn from(value: ElasticEase) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ElasticEase> for ::windows_core::IUnknown {
    fn from(value: &ElasticEase) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ElasticEase {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ElasticEase {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ElasticEase> for ::windows_core::IInspectable {
    fn from(value: ElasticEase) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ElasticEase> for ::windows_core::IInspectable {
    fn from(value: &ElasticEase) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for ElasticEase {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a ElasticEase {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ElasticEase> for EasingFunctionBase {
    fn from(value: ElasticEase) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&ElasticEase> for EasingFunctionBase {
    fn from(value: &ElasticEase) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, EasingFunctionBase> for ElasticEase {
    fn into_param(self) -> ::windows_core::Param<'a, EasingFunctionBase> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, EasingFunctionBase> for &ElasticEase {
    fn into_param(self) -> ::windows_core::Param<'a, EasingFunctionBase> {
        ::windows_core::Param::Owned(::core::convert::Into::<EasingFunctionBase>::into(self))
    }
}
impl ::core::convert::From<ElasticEase> for super::super::DependencyObject {
    fn from(value: ElasticEase) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&ElasticEase> for super::super::DependencyObject {
    fn from(value: &ElasticEase) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::DependencyObject> for ElasticEase {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::DependencyObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::DependencyObject> for &ElasticEase {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::DependencyObject> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for ElasticEase {}
unsafe impl ::core::marker::Sync for ElasticEase {}
#[repr(transparent)]
pub struct EntranceNavigationTransitionInfo(::windows_core::IUnknown);
impl EntranceNavigationTransitionInfo {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<EntranceNavigationTransitionInfo, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IsTargetElementProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::IEntranceNavigationTransitionInfoStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).IsTargetElementProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn GetIsTargetElement<'a, Param0: ::windows_core::IntoParam<'a, super::super::UIElement>>(element: Param0) -> ::windows_core::Result<bool> {
        Self::IEntranceNavigationTransitionInfoStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).GetIsTargetElement)(::windows_core::Interface::as_raw(this), element.into_param().abi(), result__.as_mut_ptr()).from_abi::<bool>(result__)
        })
    }
    pub fn SetIsTargetElement<'a, Param0: ::windows_core::IntoParam<'a, super::super::UIElement>>(element: Param0, value: bool) -> ::windows_core::Result<()> {
        Self::IEntranceNavigationTransitionInfoStatics(|this| unsafe { (::windows_core::Interface::vtable(this).SetIsTargetElement)(::windows_core::Interface::as_raw(this), element.into_param().abi(), value).ok() })
    }
    pub fn IEntranceNavigationTransitionInfoStatics<R, F: FnOnce(&IEntranceNavigationTransitionInfoStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<EntranceNavigationTransitionInfo, IEntranceNavigationTransitionInfoStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for EntranceNavigationTransitionInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for EntranceNavigationTransitionInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for EntranceNavigationTransitionInfo {}
impl ::core::fmt::Debug for EntranceNavigationTransitionInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EntranceNavigationTransitionInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for EntranceNavigationTransitionInfo {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Animation.EntranceNavigationTransitionInfo;{720a256b-1c8a-41ee-82ec-8a87c0cf47da})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for EntranceNavigationTransitionInfo {
    type Vtable = IEntranceNavigationTransitionInfo_Vtbl;
    const IID: ::windows_core::GUID = <IEntranceNavigationTransitionInfo as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for EntranceNavigationTransitionInfo {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.EntranceNavigationTransitionInfo";
}
impl ::core::convert::From<EntranceNavigationTransitionInfo> for ::windows_core::IUnknown {
    fn from(value: EntranceNavigationTransitionInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&EntranceNavigationTransitionInfo> for ::windows_core::IUnknown {
    fn from(value: &EntranceNavigationTransitionInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for EntranceNavigationTransitionInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a EntranceNavigationTransitionInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<EntranceNavigationTransitionInfo> for ::windows_core::IInspectable {
    fn from(value: EntranceNavigationTransitionInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&EntranceNavigationTransitionInfo> for ::windows_core::IInspectable {
    fn from(value: &EntranceNavigationTransitionInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for EntranceNavigationTransitionInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a EntranceNavigationTransitionInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<EntranceNavigationTransitionInfo> for NavigationTransitionInfo {
    fn from(value: EntranceNavigationTransitionInfo) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&EntranceNavigationTransitionInfo> for NavigationTransitionInfo {
    fn from(value: &EntranceNavigationTransitionInfo) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, NavigationTransitionInfo> for EntranceNavigationTransitionInfo {
    fn into_param(self) -> ::windows_core::Param<'a, NavigationTransitionInfo> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, NavigationTransitionInfo> for &EntranceNavigationTransitionInfo {
    fn into_param(self) -> ::windows_core::Param<'a, NavigationTransitionInfo> {
        ::windows_core::Param::Owned(::core::convert::Into::<NavigationTransitionInfo>::into(self))
    }
}
impl ::core::convert::From<EntranceNavigationTransitionInfo> for super::super::DependencyObject {
    fn from(value: EntranceNavigationTransitionInfo) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&EntranceNavigationTransitionInfo> for super::super::DependencyObject {
    fn from(value: &EntranceNavigationTransitionInfo) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::DependencyObject> for EntranceNavigationTransitionInfo {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::DependencyObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::DependencyObject> for &EntranceNavigationTransitionInfo {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::DependencyObject> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for EntranceNavigationTransitionInfo {}
unsafe impl ::core::marker::Sync for EntranceNavigationTransitionInfo {}
#[repr(transparent)]
pub struct EntranceThemeTransition(::windows_core::IUnknown);
impl EntranceThemeTransition {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<EntranceThemeTransition, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn FromHorizontalOffset(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).FromHorizontalOffset)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn SetFromHorizontalOffset(&self, value: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetFromHorizontalOffset)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn FromVerticalOffset(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).FromVerticalOffset)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn SetFromVerticalOffset(&self, value: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetFromVerticalOffset)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsStaggeringEnabled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsStaggeringEnabled)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetIsStaggeringEnabled(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetIsStaggeringEnabled)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn FromHorizontalOffsetProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::IEntranceThemeTransitionStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FromHorizontalOffsetProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn FromVerticalOffsetProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::IEntranceThemeTransitionStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FromVerticalOffsetProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn IsStaggeringEnabledProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::IEntranceThemeTransitionStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).IsStaggeringEnabledProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn IEntranceThemeTransitionStatics<R, F: FnOnce(&IEntranceThemeTransitionStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<EntranceThemeTransition, IEntranceThemeTransitionStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for EntranceThemeTransition {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for EntranceThemeTransition {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for EntranceThemeTransition {}
impl ::core::fmt::Debug for EntranceThemeTransition {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EntranceThemeTransition").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for EntranceThemeTransition {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Animation.EntranceThemeTransition;{07698c09-a8e3-419a-a01d-7410a0ae8ec8})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for EntranceThemeTransition {
    type Vtable = IEntranceThemeTransition_Vtbl;
    const IID: ::windows_core::GUID = <IEntranceThemeTransition as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for EntranceThemeTransition {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.EntranceThemeTransition";
}
impl ::core::convert::From<EntranceThemeTransition> for ::windows_core::IUnknown {
    fn from(value: EntranceThemeTransition) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&EntranceThemeTransition> for ::windows_core::IUnknown {
    fn from(value: &EntranceThemeTransition) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for EntranceThemeTransition {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a EntranceThemeTransition {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<EntranceThemeTransition> for ::windows_core::IInspectable {
    fn from(value: EntranceThemeTransition) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&EntranceThemeTransition> for ::windows_core::IInspectable {
    fn from(value: &EntranceThemeTransition) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for EntranceThemeTransition {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a EntranceThemeTransition {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<EntranceThemeTransition> for Transition {
    fn from(value: EntranceThemeTransition) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&EntranceThemeTransition> for Transition {
    fn from(value: &EntranceThemeTransition) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, Transition> for EntranceThemeTransition {
    fn into_param(self) -> ::windows_core::Param<'a, Transition> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, Transition> for &EntranceThemeTransition {
    fn into_param(self) -> ::windows_core::Param<'a, Transition> {
        ::windows_core::Param::Owned(::core::convert::Into::<Transition>::into(self))
    }
}
impl ::core::convert::From<EntranceThemeTransition> for super::super::DependencyObject {
    fn from(value: EntranceThemeTransition) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&EntranceThemeTransition> for super::super::DependencyObject {
    fn from(value: &EntranceThemeTransition) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::DependencyObject> for EntranceThemeTransition {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::DependencyObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::DependencyObject> for &EntranceThemeTransition {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::DependencyObject> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for EntranceThemeTransition {}
unsafe impl ::core::marker::Sync for EntranceThemeTransition {}
#[repr(transparent)]
pub struct ExponentialEase(::windows_core::IUnknown);
impl ExponentialEase {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<ExponentialEase, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn Exponent(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).Exponent)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn SetExponent(&self, value: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetExponent)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ExponentProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::IExponentialEaseStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ExponentProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn IExponentialEaseStatics<R, F: FnOnce(&IExponentialEaseStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<ExponentialEase, IExponentialEaseStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for ExponentialEase {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ExponentialEase {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ExponentialEase {}
impl ::core::fmt::Debug for ExponentialEase {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ExponentialEase").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ExponentialEase {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Animation.ExponentialEase;{7cb9e41d-f0bb-4bca-9da5-9ba3a11734c4})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for ExponentialEase {
    type Vtable = IExponentialEase_Vtbl;
    const IID: ::windows_core::GUID = <IExponentialEase as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for ExponentialEase {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.ExponentialEase";
}
impl ::core::convert::From<ExponentialEase> for ::windows_core::IUnknown {
    fn from(value: ExponentialEase) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ExponentialEase> for ::windows_core::IUnknown {
    fn from(value: &ExponentialEase) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ExponentialEase {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ExponentialEase {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ExponentialEase> for ::windows_core::IInspectable {
    fn from(value: ExponentialEase) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ExponentialEase> for ::windows_core::IInspectable {
    fn from(value: &ExponentialEase) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for ExponentialEase {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a ExponentialEase {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ExponentialEase> for EasingFunctionBase {
    fn from(value: ExponentialEase) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&ExponentialEase> for EasingFunctionBase {
    fn from(value: &ExponentialEase) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, EasingFunctionBase> for ExponentialEase {
    fn into_param(self) -> ::windows_core::Param<'a, EasingFunctionBase> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, EasingFunctionBase> for &ExponentialEase {
    fn into_param(self) -> ::windows_core::Param<'a, EasingFunctionBase> {
        ::windows_core::Param::Owned(::core::convert::Into::<EasingFunctionBase>::into(self))
    }
}
impl ::core::convert::From<ExponentialEase> for super::super::DependencyObject {
    fn from(value: ExponentialEase) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&ExponentialEase> for super::super::DependencyObject {
    fn from(value: &ExponentialEase) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::DependencyObject> for ExponentialEase {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::DependencyObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::DependencyObject> for &ExponentialEase {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::DependencyObject> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for ExponentialEase {}
unsafe impl ::core::marker::Sync for ExponentialEase {}
#[repr(transparent)]
pub struct FadeInThemeAnimation(::windows_core::IUnknown);
impl FadeInThemeAnimation {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<FadeInThemeAnimation, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn TargetName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).TargetName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetTargetName<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetTargetName)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn TargetNameProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::IFadeInThemeAnimationStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TargetNameProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn IFadeInThemeAnimationStatics<R, F: FnOnce(&IFadeInThemeAnimationStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<FadeInThemeAnimation, IFadeInThemeAnimationStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for FadeInThemeAnimation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for FadeInThemeAnimation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for FadeInThemeAnimation {}
impl ::core::fmt::Debug for FadeInThemeAnimation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FadeInThemeAnimation").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for FadeInThemeAnimation {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Animation.FadeInThemeAnimation;{6d4bc8f5-a918-4477-8078-554c68812ab8})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for FadeInThemeAnimation {
    type Vtable = IFadeInThemeAnimation_Vtbl;
    const IID: ::windows_core::GUID = <IFadeInThemeAnimation as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for FadeInThemeAnimation {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.FadeInThemeAnimation";
}
impl ::core::convert::From<FadeInThemeAnimation> for ::windows_core::IUnknown {
    fn from(value: FadeInThemeAnimation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&FadeInThemeAnimation> for ::windows_core::IUnknown {
    fn from(value: &FadeInThemeAnimation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for FadeInThemeAnimation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a FadeInThemeAnimation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<FadeInThemeAnimation> for ::windows_core::IInspectable {
    fn from(value: FadeInThemeAnimation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&FadeInThemeAnimation> for ::windows_core::IInspectable {
    fn from(value: &FadeInThemeAnimation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for FadeInThemeAnimation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a FadeInThemeAnimation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<FadeInThemeAnimation> for Timeline {
    fn from(value: FadeInThemeAnimation) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&FadeInThemeAnimation> for Timeline {
    fn from(value: &FadeInThemeAnimation) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, Timeline> for FadeInThemeAnimation {
    fn into_param(self) -> ::windows_core::Param<'a, Timeline> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, Timeline> for &FadeInThemeAnimation {
    fn into_param(self) -> ::windows_core::Param<'a, Timeline> {
        ::windows_core::Param::Owned(::core::convert::Into::<Timeline>::into(self))
    }
}
impl ::core::convert::From<FadeInThemeAnimation> for super::super::DependencyObject {
    fn from(value: FadeInThemeAnimation) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&FadeInThemeAnimation> for super::super::DependencyObject {
    fn from(value: &FadeInThemeAnimation) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::DependencyObject> for FadeInThemeAnimation {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::DependencyObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::DependencyObject> for &FadeInThemeAnimation {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::DependencyObject> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for FadeInThemeAnimation {}
unsafe impl ::core::marker::Sync for FadeInThemeAnimation {}
#[repr(transparent)]
pub struct FadeOutThemeAnimation(::windows_core::IUnknown);
impl FadeOutThemeAnimation {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<FadeOutThemeAnimation, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn TargetName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).TargetName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetTargetName<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetTargetName)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn TargetNameProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::IFadeOutThemeAnimationStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TargetNameProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn IFadeOutThemeAnimationStatics<R, F: FnOnce(&IFadeOutThemeAnimationStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<FadeOutThemeAnimation, IFadeOutThemeAnimationStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for FadeOutThemeAnimation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for FadeOutThemeAnimation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for FadeOutThemeAnimation {}
impl ::core::fmt::Debug for FadeOutThemeAnimation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FadeOutThemeAnimation").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for FadeOutThemeAnimation {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Animation.FadeOutThemeAnimation;{89276ba9-ffd4-45b6-9b9a-ced48951e712})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for FadeOutThemeAnimation {
    type Vtable = IFadeOutThemeAnimation_Vtbl;
    const IID: ::windows_core::GUID = <IFadeOutThemeAnimation as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for FadeOutThemeAnimation {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.FadeOutThemeAnimation";
}
impl ::core::convert::From<FadeOutThemeAnimation> for ::windows_core::IUnknown {
    fn from(value: FadeOutThemeAnimation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&FadeOutThemeAnimation> for ::windows_core::IUnknown {
    fn from(value: &FadeOutThemeAnimation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for FadeOutThemeAnimation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a FadeOutThemeAnimation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<FadeOutThemeAnimation> for ::windows_core::IInspectable {
    fn from(value: FadeOutThemeAnimation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&FadeOutThemeAnimation> for ::windows_core::IInspectable {
    fn from(value: &FadeOutThemeAnimation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for FadeOutThemeAnimation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a FadeOutThemeAnimation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<FadeOutThemeAnimation> for Timeline {
    fn from(value: FadeOutThemeAnimation) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&FadeOutThemeAnimation> for Timeline {
    fn from(value: &FadeOutThemeAnimation) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, Timeline> for FadeOutThemeAnimation {
    fn into_param(self) -> ::windows_core::Param<'a, Timeline> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, Timeline> for &FadeOutThemeAnimation {
    fn into_param(self) -> ::windows_core::Param<'a, Timeline> {
        ::windows_core::Param::Owned(::core::convert::Into::<Timeline>::into(self))
    }
}
impl ::core::convert::From<FadeOutThemeAnimation> for super::super::DependencyObject {
    fn from(value: FadeOutThemeAnimation) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&FadeOutThemeAnimation> for super::super::DependencyObject {
    fn from(value: &FadeOutThemeAnimation) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::DependencyObject> for FadeOutThemeAnimation {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::DependencyObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::DependencyObject> for &FadeOutThemeAnimation {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::DependencyObject> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for FadeOutThemeAnimation {}
unsafe impl ::core::marker::Sync for FadeOutThemeAnimation {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct FillBehavior(pub i32);
impl FillBehavior {
    pub const HoldEnd: Self = Self(0i32);
    pub const Stop: Self = Self(1i32);
}
impl ::core::marker::Copy for FillBehavior {}
impl ::core::clone::Clone for FillBehavior {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FillBehavior {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for FillBehavior {
    type Abi = Self;
}
impl ::core::fmt::Debug for FillBehavior {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FillBehavior").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for FillBehavior {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Media.Animation.FillBehavior;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct GravityConnectedAnimationConfiguration(::windows_core::IUnknown);
impl GravityConnectedAnimationConfiguration {
    pub fn IsShadowEnabled(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IGravityConnectedAnimationConfiguration2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsShadowEnabled)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetIsShadowEnabled(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IGravityConnectedAnimationConfiguration2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetIsShadowEnabled)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn new() -> ::windows_core::Result<GravityConnectedAnimationConfiguration> {
        Self::IGravityConnectedAnimationConfigurationFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateInstance)(::windows_core::Interface::as_raw(this), ::core::ptr::null_mut(), &mut ::core::option::Option::<::windows_core::IInspectable>::None as *mut _ as _, result__.as_mut_ptr()).from_abi::<GravityConnectedAnimationConfiguration>(result__)
        })
    }
    pub fn compose<T: ::windows_core::Compose>(compose: T) -> ::windows_core::Result<GravityConnectedAnimationConfiguration> {
        Self::IGravityConnectedAnimationConfigurationFactory(|this| unsafe {
            let (derived__, base__) = ::windows_core::Compose::compose(compose);
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateInstance)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(&derived__), base__ as *mut _ as _, result__.as_mut_ptr()).from_abi::<GravityConnectedAnimationConfiguration>(result__)
        })
    }
    pub fn IGravityConnectedAnimationConfigurationFactory<R, F: FnOnce(&IGravityConnectedAnimationConfigurationFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<GravityConnectedAnimationConfiguration, IGravityConnectedAnimationConfigurationFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for GravityConnectedAnimationConfiguration {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for GravityConnectedAnimationConfiguration {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GravityConnectedAnimationConfiguration {}
impl ::core::fmt::Debug for GravityConnectedAnimationConfiguration {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GravityConnectedAnimationConfiguration").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for GravityConnectedAnimationConfiguration {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Animation.GravityConnectedAnimationConfiguration;{c751a4b7-0459-5142-b891-aeaac1d41822})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for GravityConnectedAnimationConfiguration {
    type Vtable = IGravityConnectedAnimationConfiguration_Vtbl;
    const IID: ::windows_core::GUID = <IGravityConnectedAnimationConfiguration as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for GravityConnectedAnimationConfiguration {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.GravityConnectedAnimationConfiguration";
}
impl ::core::convert::From<GravityConnectedAnimationConfiguration> for ::windows_core::IUnknown {
    fn from(value: GravityConnectedAnimationConfiguration) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&GravityConnectedAnimationConfiguration> for ::windows_core::IUnknown {
    fn from(value: &GravityConnectedAnimationConfiguration) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for GravityConnectedAnimationConfiguration {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a GravityConnectedAnimationConfiguration {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<GravityConnectedAnimationConfiguration> for ::windows_core::IInspectable {
    fn from(value: GravityConnectedAnimationConfiguration) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&GravityConnectedAnimationConfiguration> for ::windows_core::IInspectable {
    fn from(value: &GravityConnectedAnimationConfiguration) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for GravityConnectedAnimationConfiguration {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a GravityConnectedAnimationConfiguration {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<GravityConnectedAnimationConfiguration> for ConnectedAnimationConfiguration {
    fn from(value: GravityConnectedAnimationConfiguration) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&GravityConnectedAnimationConfiguration> for ConnectedAnimationConfiguration {
    fn from(value: &GravityConnectedAnimationConfiguration) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, ConnectedAnimationConfiguration> for GravityConnectedAnimationConfiguration {
    fn into_param(self) -> ::windows_core::Param<'a, ConnectedAnimationConfiguration> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ConnectedAnimationConfiguration> for &GravityConnectedAnimationConfiguration {
    fn into_param(self) -> ::windows_core::Param<'a, ConnectedAnimationConfiguration> {
        ::windows_core::Param::Owned(::core::convert::Into::<ConnectedAnimationConfiguration>::into(self))
    }
}
unsafe impl ::core::marker::Send for GravityConnectedAnimationConfiguration {}
unsafe impl ::core::marker::Sync for GravityConnectedAnimationConfiguration {}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAddDeleteThemeTransition(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAddDeleteThemeTransition {
    type Vtable = IAddDeleteThemeTransition_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xadec852e_4424_4dab_99c1_3a04e36a3c48);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAddDeleteThemeTransition_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBackEase(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IBackEase {
    type Vtable = IBackEase_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe47796e7_f805_4a8f_81c9_38e6472caa94);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackEase_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Amplitude: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub SetAmplitude: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBackEaseStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IBackEaseStatics {
    type Vtable = IBackEaseStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3c70a2ff_a0a0_4786_926c_22321f8f25b7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackEaseStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub AmplitudeProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBasicConnectedAnimationConfiguration(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IBasicConnectedAnimationConfiguration {
    type Vtable = IBasicConnectedAnimationConfiguration_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe675f9b5_a4d6_5353_83e6_c89e7cf8d456);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBasicConnectedAnimationConfiguration_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBasicConnectedAnimationConfigurationFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IBasicConnectedAnimationConfigurationFactory {
    type Vtable = IBasicConnectedAnimationConfigurationFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x95e6844a_4377_503c_bee2_11dfcd5570e6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBasicConnectedAnimationConfigurationFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CreateInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBeginStoryboard(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IBeginStoryboard {
    type Vtable = IBeginStoryboard_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x64189fcd_49ec_4e52_a6f6_55324c921053);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBeginStoryboard_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Storyboard: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetStoryboard: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBeginStoryboardStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IBeginStoryboardStatics {
    type Vtable = IBeginStoryboardStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x12cff18c_aa91_4c4a_b82f_df34fc57f94b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBeginStoryboardStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub StoryboardProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBounceEase(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IBounceEase {
    type Vtable = IBounceEase_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2bf1464e_fc71_47ed_85a1_3ba9577718b4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBounceEase_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Bounces: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub SetBounces: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows_core::HRESULT,
    pub Bounciness: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub SetBounciness: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBounceEaseStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IBounceEaseStatics {
    type Vtable = IBounceEaseStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc0701da2_4f73_41c9_b2cb_2ea3105107ff);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBounceEaseStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub BouncesProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub BouncinessProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICircleEase(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICircleEase {
    type Vtable = ICircleEase_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x53a3bdb2_9177_4e6e_a043_5082d889ab1f);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICircleEase_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IColorAnimation(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IColorAnimation {
    type Vtable = IColorAnimation_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb8ae8a15_0f63_4694_9467_bdafac1253ea);
}
#[repr(C)]
#[doc(hidden)]
pub struct IColorAnimation_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub From: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetFrom: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub To: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetTo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub By: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetBy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub EasingFunction: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetEasingFunction: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub EnableDependentAnimation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetEnableDependentAnimation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IColorAnimationStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IColorAnimationStatics {
    type Vtable = IColorAnimationStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x55eaf6e2_87e3_4f48_958f_855b2f9ea9ec);
}
#[repr(C)]
#[doc(hidden)]
pub struct IColorAnimationStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub FromProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ToProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ByProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub EasingFunctionProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub EnableDependentAnimationProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IColorAnimationUsingKeyFrames(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IColorAnimationUsingKeyFrames {
    type Vtable = IColorAnimationUsingKeyFrames_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf5c82640_13c3_42aa_9ae2_7e6b51c92f95);
}
#[repr(C)]
#[doc(hidden)]
pub struct IColorAnimationUsingKeyFrames_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-foundation")]
    pub KeyFrames: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    KeyFrames: usize,
    pub EnableDependentAnimation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetEnableDependentAnimation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IColorAnimationUsingKeyFramesStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IColorAnimationUsingKeyFramesStatics {
    type Vtable = IColorAnimationUsingKeyFramesStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb4723cdc_96e9_48f9_8d92_9b648b2f1cc6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IColorAnimationUsingKeyFramesStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub EnableDependentAnimationProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IColorKeyFrame(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IColorKeyFrame {
    type Vtable = IColorKeyFrame_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb51d82d9_0910_4589_a284_b0c9205858e9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IColorKeyFrame_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Value: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Color) -> ::windows_core::HRESULT,
    pub SetValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::super::Color) -> ::windows_core::HRESULT,
    pub KeyTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut KeyTime) -> ::windows_core::HRESULT,
    pub SetKeyTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: KeyTime) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IColorKeyFrameFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IColorKeyFrameFactory {
    type Vtable = IColorKeyFrameFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x769bd88a_9cfb_4a7d_96c4_a1e7de6fdb4b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IColorKeyFrameFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CreateInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IColorKeyFrameStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IColorKeyFrameStatics {
    type Vtable = IColorKeyFrameStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc043ae99_210c_430f_9da5_df1082692055);
}
#[repr(C)]
#[doc(hidden)]
pub struct IColorKeyFrameStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub ValueProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub KeyTimeProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICommonNavigationTransitionInfo(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICommonNavigationTransitionInfo {
    type Vtable = ICommonNavigationTransitionInfo_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x50345692_a555_4624_a361_0a91c1706473);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICommonNavigationTransitionInfo_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub IsStaggeringEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetIsStaggeringEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICommonNavigationTransitionInfoStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICommonNavigationTransitionInfoStatics {
    type Vtable = ICommonNavigationTransitionInfoStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1e3efe33_50be_4443_883c_e5627201c2e5);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICommonNavigationTransitionInfoStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub IsStaggeringEnabledProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub IsStaggerElementProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetIsStaggerElement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows_core::RawPtr, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetIsStaggerElement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows_core::RawPtr, value: bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IConnectedAnimation(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IConnectedAnimation {
    type Vtable = IConnectedAnimation_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3518628c_f387_4c25_ac98_44e86c3cadf0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IConnectedAnimation_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Completed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub TryStart: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, destination: ::windows_core::RawPtr, result__: *mut bool) -> ::windows_core::HRESULT,
    pub Cancel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IConnectedAnimation2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IConnectedAnimation2 {
    type Vtable = IConnectedAnimation2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5d2f8e5c_584b_4ddd_b668_973891431459);
}
#[repr(C)]
#[doc(hidden)]
pub struct IConnectedAnimation2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub IsScaleAnimationEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetIsScaleAnimationEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-foundation")]
    pub TryStartWithCoordinatedElements: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, destination: ::windows_core::RawPtr, coordinatedelements: ::windows_core::RawPtr, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    TryStartWithCoordinatedElements: usize,
    #[cfg(feature = "winrt-ui")]
    pub SetAnimationComponent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, component: ConnectedAnimationComponent, animation: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-ui"))]
    SetAnimationComponent: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IConnectedAnimation3(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IConnectedAnimation3 {
    type Vtable = IConnectedAnimation3_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6e3040c6_0430_59c0_a80c_cceed2e778dd);
}
#[repr(C)]
#[doc(hidden)]
pub struct IConnectedAnimation3_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Configuration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetConfiguration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IConnectedAnimationConfiguration(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IConnectedAnimationConfiguration {
    type Vtable = IConnectedAnimationConfiguration_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x00218aae_cd8c_5651_92a0_c1db95c03998);
}
#[repr(C)]
#[doc(hidden)]
pub struct IConnectedAnimationConfiguration_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IConnectedAnimationConfigurationFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IConnectedAnimationConfigurationFactory {
    type Vtable = IConnectedAnimationConfigurationFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x30f9b84b_dd7e_593e_bf75_e959dc0ec52a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IConnectedAnimationConfigurationFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IConnectedAnimationService(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IConnectedAnimationService {
    type Vtable = IConnectedAnimationService_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1c6875c9_19bb_4d47_b9aa_66c802dcb9ff);
}
#[repr(C)]
#[doc(hidden)]
pub struct IConnectedAnimationService_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub DefaultDuration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::TimeSpan) -> ::windows_core::HRESULT,
    pub SetDefaultDuration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::winrt_foundation::TimeSpan) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-ui")]
    pub DefaultEasingFunction: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-ui"))]
    DefaultEasingFunction: usize,
    #[cfg(feature = "winrt-ui")]
    pub SetDefaultEasingFunction: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-ui"))]
    SetDefaultEasingFunction: usize,
    pub PrepareToAnimate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, source: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetAnimation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IConnectedAnimationServiceStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IConnectedAnimationServiceStatics {
    type Vtable = IConnectedAnimationServiceStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc7078ea5_d688_40e8_8f90_96a6279273d2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IConnectedAnimationServiceStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub GetForCurrentView: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IContentThemeTransition(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IContentThemeTransition {
    type Vtable = IContentThemeTransition_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf66fc5c3_5915_437d_8e3b_adf8e7f0ab57);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContentThemeTransition_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub HorizontalOffset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub SetHorizontalOffset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows_core::HRESULT,
    pub VerticalOffset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub SetVerticalOffset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IContentThemeTransitionStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IContentThemeTransitionStatics {
    type Vtable = IContentThemeTransitionStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0e8ee385_9a42_4459_afa9_337dc41e1587);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContentThemeTransitionStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub HorizontalOffsetProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub VerticalOffsetProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IContinuumNavigationTransitionInfo(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IContinuumNavigationTransitionInfo {
    type Vtable = IContinuumNavigationTransitionInfo_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4be1dbad_8ba6_4004_8438_8a9017978543);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContinuumNavigationTransitionInfo_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub ExitElement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetExitElement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IContinuumNavigationTransitionInfoStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IContinuumNavigationTransitionInfoStatics {
    type Vtable = IContinuumNavigationTransitionInfoStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3e25dd53_b18f_4bf1_b3bc_92f516f29903);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContinuumNavigationTransitionInfoStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub ExitElementProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub IsEntranceElementProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetIsEntranceElement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows_core::RawPtr, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetIsEntranceElement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows_core::RawPtr, value: bool) -> ::windows_core::HRESULT,
    pub IsExitElementProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetIsExitElement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows_core::RawPtr, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetIsExitElement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows_core::RawPtr, value: bool) -> ::windows_core::HRESULT,
    pub ExitElementContainerProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-ui")]
    pub GetExitElementContainer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows_core::RawPtr, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-ui"))]
    GetExitElementContainer: usize,
    #[cfg(feature = "winrt-ui")]
    pub SetExitElementContainer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows_core::RawPtr, value: bool) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-ui"))]
    SetExitElementContainer: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICubicEase(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICubicEase {
    type Vtable = ICubicEase_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1b94fc76_dad7_4354_b1a2_7969fbf6a70d);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICubicEase_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDirectConnectedAnimationConfiguration(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDirectConnectedAnimationConfiguration {
    type Vtable = IDirectConnectedAnimationConfiguration_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xee5d736f_5738_5d86_b770_151948cf365e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectConnectedAnimationConfiguration_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDirectConnectedAnimationConfigurationFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDirectConnectedAnimationConfigurationFactory {
    type Vtable = IDirectConnectedAnimationConfigurationFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x059263e9_d2b3_5a77_9cf4_e26d8b542608);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectConnectedAnimationConfigurationFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CreateInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDiscreteColorKeyFrame(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDiscreteColorKeyFrame {
    type Vtable = IDiscreteColorKeyFrame_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x230c08f4_e062_4cb1_8e2a_14093d73ed8c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDiscreteColorKeyFrame_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDiscreteDoubleKeyFrame(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDiscreteDoubleKeyFrame {
    type Vtable = IDiscreteDoubleKeyFrame_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf5f51f3a_ad11_49ce_8e1c_08fdf1447446);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDiscreteDoubleKeyFrame_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDiscreteObjectKeyFrame(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDiscreteObjectKeyFrame {
    type Vtable = IDiscreteObjectKeyFrame_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc7dcde89_f12d_4a9c_8199_e7a9ece3a473);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDiscreteObjectKeyFrame_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDiscretePointKeyFrame(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDiscretePointKeyFrame {
    type Vtable = IDiscretePointKeyFrame_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe0a9070d_4c42_4a90_983a_75f5a83a2fbe);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDiscretePointKeyFrame_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDoubleAnimation(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDoubleAnimation {
    type Vtable = IDoubleAnimation_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7e9f3d59_0f07_4bc9_977d_03763ff8154f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDoubleAnimation_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub From: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetFrom: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub To: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetTo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub By: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetBy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub EasingFunction: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetEasingFunction: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub EnableDependentAnimation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetEnableDependentAnimation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDoubleAnimationStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDoubleAnimationStatics {
    type Vtable = IDoubleAnimationStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe27a935d_f111_43b7_b824_832b58d7786b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDoubleAnimationStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub FromProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ToProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ByProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub EasingFunctionProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub EnableDependentAnimationProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDoubleAnimationUsingKeyFrames(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDoubleAnimationUsingKeyFrames {
    type Vtable = IDoubleAnimationUsingKeyFrames_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4fee628f_bfee_4f75_83c2_a93b39488473);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDoubleAnimationUsingKeyFrames_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-foundation")]
    pub KeyFrames: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    KeyFrames: usize,
    pub EnableDependentAnimation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetEnableDependentAnimation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDoubleAnimationUsingKeyFramesStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDoubleAnimationUsingKeyFramesStatics {
    type Vtable = IDoubleAnimationUsingKeyFramesStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x109bf2f6_c60f_49aa_abf6_f696d492116b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDoubleAnimationUsingKeyFramesStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub EnableDependentAnimationProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDoubleKeyFrame(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDoubleKeyFrame {
    type Vtable = IDoubleKeyFrame_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x674456fd_e81e_4f4e_b4ad_0acfed9ecd68);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDoubleKeyFrame_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Value: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub SetValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows_core::HRESULT,
    pub KeyTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut KeyTime) -> ::windows_core::HRESULT,
    pub SetKeyTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: KeyTime) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDoubleKeyFrameFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDoubleKeyFrameFactory {
    type Vtable = IDoubleKeyFrameFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xac97dec3_7538_40b9_b152_696f7fbf4722);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDoubleKeyFrameFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CreateInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDoubleKeyFrameStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDoubleKeyFrameStatics {
    type Vtable = IDoubleKeyFrameStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x324641b0_7d37_427a_adeb_43f38bb61a4d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDoubleKeyFrameStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub ValueProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub KeyTimeProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDragItemThemeAnimation(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDragItemThemeAnimation {
    type Vtable = IDragItemThemeAnimation_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0c7d5db5_7ed6_4949_b4e6_a78c9f4f978d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDragItemThemeAnimation_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub TargetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetTargetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDragItemThemeAnimationStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDragItemThemeAnimationStatics {
    type Vtable = IDragItemThemeAnimationStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6218b9f5_013a_4fb1_86fc_92bc4e8d0241);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDragItemThemeAnimationStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub TargetNameProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDragOverThemeAnimation(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDragOverThemeAnimation {
    type Vtable = IDragOverThemeAnimation_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x72f762f7_7e51_4a6b_b937_dc4b4c1c5458);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDragOverThemeAnimation_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub TargetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetTargetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub ToOffset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub SetToOffset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-ui")]
    pub Direction: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Controls::Primitives::AnimationDirection) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-ui"))]
    Direction: usize,
    #[cfg(feature = "winrt-ui")]
    pub SetDirection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Controls::Primitives::AnimationDirection) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-ui"))]
    SetDirection: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDragOverThemeAnimationStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDragOverThemeAnimationStatics {
    type Vtable = IDragOverThemeAnimationStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x146ffe57_3c9d_41d9_a5ff_8d7239516810);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDragOverThemeAnimationStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub TargetNameProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ToOffsetProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub DirectionProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDrillInNavigationTransitionInfo(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDrillInNavigationTransitionInfo {
    type Vtable = IDrillInNavigationTransitionInfo_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3b86201a_45d3_463b_939e_c8595f439bcc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDrillInNavigationTransitionInfo_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDrillInThemeAnimation(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDrillInThemeAnimation {
    type Vtable = IDrillInThemeAnimation_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb090b824_f1d2_41b8_87ba_78034126594c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDrillInThemeAnimation_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub EntranceTargetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetEntranceTargetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub EntranceTarget: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetEntranceTarget: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ExitTargetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetExitTargetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub ExitTarget: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetExitTarget: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDrillInThemeAnimationStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDrillInThemeAnimationStatics {
    type Vtable = IDrillInThemeAnimationStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc61fe488_a17a_4b11_b53b_a4f1a07d4ba9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDrillInThemeAnimationStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub EntranceTargetNameProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub EntranceTargetProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ExitTargetNameProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ExitTargetProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDrillOutThemeAnimation(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDrillOutThemeAnimation {
    type Vtable = IDrillOutThemeAnimation_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd890ccdf_06d3_4f7e_8e4a_4fb76e256139);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDrillOutThemeAnimation_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub EntranceTargetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetEntranceTargetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub EntranceTarget: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetEntranceTarget: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ExitTargetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetExitTargetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub ExitTarget: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetExitTarget: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDrillOutThemeAnimationStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDrillOutThemeAnimationStatics {
    type Vtable = IDrillOutThemeAnimationStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xbeb5db9b_2617_4888_80dd_72fa7bb6fac3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDrillOutThemeAnimationStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub EntranceTargetNameProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub EntranceTargetProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ExitTargetNameProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ExitTargetProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDropTargetItemThemeAnimation(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDropTargetItemThemeAnimation {
    type Vtable = IDropTargetItemThemeAnimation_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1881c968_1824_462b_87e8_c357212b977b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDropTargetItemThemeAnimation_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub TargetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetTargetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDropTargetItemThemeAnimationStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDropTargetItemThemeAnimationStatics {
    type Vtable = IDropTargetItemThemeAnimationStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xae80f486_2e56_4513_bf18_d77470164ae5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDropTargetItemThemeAnimationStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub TargetNameProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IEasingColorKeyFrame(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IEasingColorKeyFrame {
    type Vtable = IEasingColorKeyFrame_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc733d630_f4b9_4934_9bdd_27ac5ed1cfd8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEasingColorKeyFrame_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub EasingFunction: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetEasingFunction: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IEasingColorKeyFrameStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IEasingColorKeyFrameStatics {
    type Vtable = IEasingColorKeyFrameStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6f3837fc_8e3d_4522_9b0f_003db8609851);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEasingColorKeyFrameStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub EasingFunctionProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IEasingDoubleKeyFrame(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IEasingDoubleKeyFrame {
    type Vtable = IEasingDoubleKeyFrame_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x965adb8d_9a54_4108_b4ff_b5a5212cb338);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEasingDoubleKeyFrame_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub EasingFunction: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetEasingFunction: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IEasingDoubleKeyFrameStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IEasingDoubleKeyFrameStatics {
    type Vtable = IEasingDoubleKeyFrameStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc8d3d845_dbae_4e5b_8b84_d9537398e5b1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEasingDoubleKeyFrameStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub EasingFunctionProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IEasingFunctionBase(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IEasingFunctionBase {
    type Vtable = IEasingFunctionBase_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc108383f_2c02_4151_8ecd_68ddaa3f0d9b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEasingFunctionBase_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub EasingMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut EasingMode) -> ::windows_core::HRESULT,
    pub SetEasingMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: EasingMode) -> ::windows_core::HRESULT,
    pub Ease: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, normalizedtime: f64, result__: *mut f64) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IEasingFunctionBaseFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IEasingFunctionBaseFactory {
    type Vtable = IEasingFunctionBaseFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1830fe6a_f01b_43e0_b61f_b452a1c66fd2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEasingFunctionBaseFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IEasingFunctionBaseStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IEasingFunctionBaseStatics {
    type Vtable = IEasingFunctionBaseStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2a5031aa_2c50_4a1d_bb04_d75e07b71548);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEasingFunctionBaseStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub EasingModeProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IEasingPointKeyFrame(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IEasingPointKeyFrame {
    type Vtable = IEasingPointKeyFrame_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb3c91380_6868_4225_a70b_3981cc0b2947);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEasingPointKeyFrame_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub EasingFunction: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetEasingFunction: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IEasingPointKeyFrameStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IEasingPointKeyFrameStatics {
    type Vtable = IEasingPointKeyFrameStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe22dbfc4_080c_402c_a6b5_f48d0a98116b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEasingPointKeyFrameStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub EasingFunctionProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IEdgeUIThemeTransition(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IEdgeUIThemeTransition {
    type Vtable = IEdgeUIThemeTransition_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5c86c19b_49d7_19ec_cf19_83a73c6de75e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEdgeUIThemeTransition_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-ui")]
    pub Edge: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Controls::Primitives::EdgeTransitionLocation) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-ui"))]
    Edge: usize,
    #[cfg(feature = "winrt-ui")]
    pub SetEdge: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Controls::Primitives::EdgeTransitionLocation) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-ui"))]
    SetEdge: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IEdgeUIThemeTransitionStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IEdgeUIThemeTransitionStatics {
    type Vtable = IEdgeUIThemeTransitionStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x16a2b13b_4705_302b_27c6_2aac92f645ac);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEdgeUIThemeTransitionStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub EdgeProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IElasticEase(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IElasticEase {
    type Vtable = IElasticEase_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xef5ba58c_b0b6_4a6c_9ca8_fb4233f12459);
}
#[repr(C)]
#[doc(hidden)]
pub struct IElasticEase_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Oscillations: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub SetOscillations: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows_core::HRESULT,
    pub Springiness: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub SetSpringiness: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IElasticEaseStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IElasticEaseStatics {
    type Vtable = IElasticEaseStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa9f566ec_fe9c_4b2b_8e52_bb785d562185);
}
#[repr(C)]
#[doc(hidden)]
pub struct IElasticEaseStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub OscillationsProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SpringinessProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IEntranceNavigationTransitionInfo(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IEntranceNavigationTransitionInfo {
    type Vtable = IEntranceNavigationTransitionInfo_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x720a256b_1c8a_41ee_82ec_8a87c0cf47da);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEntranceNavigationTransitionInfo_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IEntranceNavigationTransitionInfoStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IEntranceNavigationTransitionInfoStatics {
    type Vtable = IEntranceNavigationTransitionInfoStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf948c27a_40c9_469f_8f33_bf45c8811f21);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEntranceNavigationTransitionInfoStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub IsTargetElementProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetIsTargetElement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows_core::RawPtr, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetIsTargetElement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows_core::RawPtr, value: bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IEntranceThemeTransition(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IEntranceThemeTransition {
    type Vtable = IEntranceThemeTransition_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x07698c09_a8e3_419a_a01d_7410a0ae8ec8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEntranceThemeTransition_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub FromHorizontalOffset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub SetFromHorizontalOffset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows_core::HRESULT,
    pub FromVerticalOffset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub SetFromVerticalOffset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows_core::HRESULT,
    pub IsStaggeringEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetIsStaggeringEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IEntranceThemeTransitionStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IEntranceThemeTransitionStatics {
    type Vtable = IEntranceThemeTransitionStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x37cc0577_ff98_4aed_b86e_5ec23702f877);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEntranceThemeTransitionStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub FromHorizontalOffsetProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub FromVerticalOffsetProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub IsStaggeringEnabledProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IExponentialEase(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IExponentialEase {
    type Vtable = IExponentialEase_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7cb9e41d_f0bb_4bca_9da5_9ba3a11734c4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IExponentialEase_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Exponent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub SetExponent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IExponentialEaseStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IExponentialEaseStatics {
    type Vtable = IExponentialEaseStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf37ee7e3_a761_4352_9ad6_70794567581a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IExponentialEaseStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub ExponentProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IFadeInThemeAnimation(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IFadeInThemeAnimation {
    type Vtable = IFadeInThemeAnimation_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6d4bc8f5_a918_4477_8078_554c68812ab8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFadeInThemeAnimation_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub TargetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetTargetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IFadeInThemeAnimationStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IFadeInThemeAnimationStatics {
    type Vtable = IFadeInThemeAnimationStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7f0117e1_bea9_4923_b23a_0ddf4d7b8737);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFadeInThemeAnimationStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub TargetNameProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IFadeOutThemeAnimation(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IFadeOutThemeAnimation {
    type Vtable = IFadeOutThemeAnimation_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x89276ba9_ffd4_45b6_9b9a_ced48951e712);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFadeOutThemeAnimation_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub TargetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetTargetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IFadeOutThemeAnimationStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IFadeOutThemeAnimationStatics {
    type Vtable = IFadeOutThemeAnimationStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xfe17a81a_4168_4f68_a28c_e5dd98cf680f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFadeOutThemeAnimationStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub TargetNameProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGravityConnectedAnimationConfiguration(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGravityConnectedAnimationConfiguration {
    type Vtable = IGravityConnectedAnimationConfiguration_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc751a4b7_0459_5142_b891_aeaac1d41822);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGravityConnectedAnimationConfiguration_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGravityConnectedAnimationConfiguration2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGravityConnectedAnimationConfiguration2 {
    type Vtable = IGravityConnectedAnimationConfiguration2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x62333add_aed4_5fed_95ff_d128acce8be4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGravityConnectedAnimationConfiguration2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub IsShadowEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetIsShadowEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGravityConnectedAnimationConfigurationFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGravityConnectedAnimationConfigurationFactory {
    type Vtable = IGravityConnectedAnimationConfigurationFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe822c41f_3656_5090_92f5_c217eaacb682);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGravityConnectedAnimationConfigurationFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CreateInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IKeySpline(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IKeySpline {
    type Vtable = IKeySpline_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x77a163bb_d5ca_4a32_ba0b_7dff988e58a0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IKeySpline_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub ControlPoint1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::Point) -> ::windows_core::HRESULT,
    pub SetControlPoint1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::winrt_foundation::Point) -> ::windows_core::HRESULT,
    pub ControlPoint2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::Point) -> ::windows_core::HRESULT,
    pub SetControlPoint2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::winrt_foundation::Point) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IKeyTimeHelper(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IKeyTimeHelper {
    type Vtable = IKeyTimeHelper_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3643e480_4823_466a_abe5_5e79c8ed77ed);
}
#[repr(C)]
#[doc(hidden)]
pub struct IKeyTimeHelper_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IKeyTimeHelperStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IKeyTimeHelperStatics {
    type Vtable = IKeyTimeHelperStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7fa2612c_22a9_45e9_9af7_c7416efff7a5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IKeyTimeHelperStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub FromTimeSpan: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, timespan: ::winrt_foundation::TimeSpan, result__: *mut KeyTime) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILinearColorKeyFrame(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ILinearColorKeyFrame {
    type Vtable = ILinearColorKeyFrame_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x66fdb6ef_ac81_4611_b1d2_61f545983f03);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILinearColorKeyFrame_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILinearDoubleKeyFrame(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ILinearDoubleKeyFrame {
    type Vtable = ILinearDoubleKeyFrame_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8efdf265_9a7b_431d_8f0c_14c56b5ea4d9);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILinearDoubleKeyFrame_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILinearPointKeyFrame(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ILinearPointKeyFrame {
    type Vtable = ILinearPointKeyFrame_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe7c9b8ef_af24_49ee_84f1_a86600a4e319);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILinearPointKeyFrame_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct INavigationThemeTransition(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for INavigationThemeTransition {
    type Vtable = INavigationThemeTransition_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8833848c_4eb7_41f2_8799_9eef0a213b73);
}
#[repr(C)]
#[doc(hidden)]
pub struct INavigationThemeTransition_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub DefaultNavigationTransitionInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetDefaultNavigationTransitionInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct INavigationThemeTransitionStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for INavigationThemeTransitionStatics {
    type Vtable = INavigationThemeTransitionStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xea2f06e0_5e60_4f8e_bcaf_431487a294ab);
}
#[repr(C)]
#[doc(hidden)]
pub struct INavigationThemeTransitionStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub DefaultNavigationTransitionInfoProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct INavigationTransitionInfo(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for INavigationTransitionInfo {
    type Vtable = INavigationTransitionInfo_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa9b05091_ae4a_4372_8625_21b7a8b98ca4);
}
#[repr(C)]
#[doc(hidden)]
pub struct INavigationTransitionInfo_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct INavigationTransitionInfoFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for INavigationTransitionInfoFactory {
    type Vtable = INavigationTransitionInfoFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xedf4f8d5_af63_4fab_9d4a_87927f82dd6b);
}
#[repr(C)]
#[doc(hidden)]
pub struct INavigationTransitionInfoFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CreateInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct INavigationTransitionInfoOverrides(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for INavigationTransitionInfoOverrides {
    type Vtable = INavigationTransitionInfoOverrides_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd9517e6a_a9d0_4bf7_9db0_4633a69daff2);
}
#[repr(C)]
#[doc(hidden)]
pub struct INavigationTransitionInfoOverrides_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub GetNavigationStateCore: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetNavigationStateCore: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, navigationstate: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IObjectAnimationUsingKeyFrames(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IObjectAnimationUsingKeyFrames {
    type Vtable = IObjectAnimationUsingKeyFrames_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x334a2d92_b74a_4c64_b9a6_58bcfa314f22);
}
#[repr(C)]
#[doc(hidden)]
pub struct IObjectAnimationUsingKeyFrames_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-foundation")]
    pub KeyFrames: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    KeyFrames: usize,
    pub EnableDependentAnimation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetEnableDependentAnimation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IObjectAnimationUsingKeyFramesStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IObjectAnimationUsingKeyFramesStatics {
    type Vtable = IObjectAnimationUsingKeyFramesStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xeb736182_6af1_49a3_97b6_783ed97400fe);
}
#[repr(C)]
#[doc(hidden)]
pub struct IObjectAnimationUsingKeyFramesStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub EnableDependentAnimationProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IObjectKeyFrame(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IObjectKeyFrame {
    type Vtable = IObjectKeyFrame_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9852a851_8593_48ee_a6a4_d5d4720f029a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IObjectKeyFrame_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Value: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub KeyTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut KeyTime) -> ::windows_core::HRESULT,
    pub SetKeyTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: KeyTime) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IObjectKeyFrameFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IObjectKeyFrameFactory {
    type Vtable = IObjectKeyFrameFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1626143e_3e6d_44d8_9b9a_04aea70f8492);
}
#[repr(C)]
#[doc(hidden)]
pub struct IObjectKeyFrameFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CreateInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IObjectKeyFrameStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IObjectKeyFrameStatics {
    type Vtable = IObjectKeyFrameStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2cd6ab00_5319_4286_8eed_4e755ea0cf9c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IObjectKeyFrameStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub ValueProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub KeyTimeProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPaneThemeTransition(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPaneThemeTransition {
    type Vtable = IPaneThemeTransition_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4708eb8e_4bfc_ee46_d4f9_708def3fbb2b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPaneThemeTransition_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-ui")]
    pub Edge: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Controls::Primitives::EdgeTransitionLocation) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-ui"))]
    Edge: usize,
    #[cfg(feature = "winrt-ui")]
    pub SetEdge: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Controls::Primitives::EdgeTransitionLocation) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-ui"))]
    SetEdge: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPaneThemeTransitionStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPaneThemeTransitionStatics {
    type Vtable = IPaneThemeTransitionStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x316b382f_4be4_1797_b45c_cd900bbe0caa);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPaneThemeTransitionStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub EdgeProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPointAnimation(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPointAnimation {
    type Vtable = IPointAnimation_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x30f04312_7726_4f88_b8e2_2fa54518963b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPointAnimation_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub From: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetFrom: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub To: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetTo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub By: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetBy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub EasingFunction: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetEasingFunction: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub EnableDependentAnimation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetEnableDependentAnimation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPointAnimationStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPointAnimationStatics {
    type Vtable = IPointAnimationStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2f99b356_e737_408b_a0fd_327826d32255);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPointAnimationStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub FromProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ToProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ByProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub EasingFunctionProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub EnableDependentAnimationProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPointAnimationUsingKeyFrames(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPointAnimationUsingKeyFrames {
    type Vtable = IPointAnimationUsingKeyFrames_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9b944f72_446a_41d0_a129_41a620f4595d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPointAnimationUsingKeyFrames_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-foundation")]
    pub KeyFrames: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    KeyFrames: usize,
    pub EnableDependentAnimation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetEnableDependentAnimation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPointAnimationUsingKeyFramesStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPointAnimationUsingKeyFramesStatics {
    type Vtable = IPointAnimationUsingKeyFramesStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5f454c87_2390_46ea_baa7_762f4bc30d04);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPointAnimationUsingKeyFramesStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub EnableDependentAnimationProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPointKeyFrame(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPointKeyFrame {
    type Vtable = IPointKeyFrame_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xfcc88d01_7f82_4dae_8026_7b7e086878b3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPointKeyFrame_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Value: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::Point) -> ::windows_core::HRESULT,
    pub SetValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::winrt_foundation::Point) -> ::windows_core::HRESULT,
    pub KeyTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut KeyTime) -> ::windows_core::HRESULT,
    pub SetKeyTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: KeyTime) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPointKeyFrameFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPointKeyFrameFactory {
    type Vtable = IPointKeyFrameFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xcb214bdf_426a_4392_8355_c2ae52852623);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPointKeyFrameFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CreateInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPointKeyFrameStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPointKeyFrameStatics {
    type Vtable = IPointKeyFrameStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x95cf1b27_7965_4bec_b9fb_fbe94b65518e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPointKeyFrameStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub ValueProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub KeyTimeProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPointerDownThemeAnimation(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPointerDownThemeAnimation {
    type Vtable = IPointerDownThemeAnimation_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb58e714e_c49d_4788_a233_0ae85d99dd5a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPointerDownThemeAnimation_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub TargetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetTargetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPointerDownThemeAnimationStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPointerDownThemeAnimationStatics {
    type Vtable = IPointerDownThemeAnimationStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x63a7cb7b_6d46_4494_b94a_e72f3b492a61);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPointerDownThemeAnimationStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub TargetNameProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPointerUpThemeAnimation(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPointerUpThemeAnimation {
    type Vtable = IPointerUpThemeAnimation_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe9e9d07d_6340_4828_ad12_690694b9910b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPointerUpThemeAnimation_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub TargetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetTargetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPointerUpThemeAnimationStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPointerUpThemeAnimationStatics {
    type Vtable = IPointerUpThemeAnimationStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7c618f9c_7992_4139_8bfc_0883b9727a7e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPointerUpThemeAnimationStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub TargetNameProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPopInThemeAnimation(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPopInThemeAnimation {
    type Vtable = IPopInThemeAnimation_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x196938c1_1c07_4c28_8847_f9f055b32855);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPopInThemeAnimation_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub TargetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetTargetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub FromHorizontalOffset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub SetFromHorizontalOffset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows_core::HRESULT,
    pub FromVerticalOffset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub SetFromVerticalOffset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPopInThemeAnimationStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPopInThemeAnimationStatics {
    type Vtable = IPopInThemeAnimationStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xefaa99d3_218a_4701_977f_f1bfae8ba649);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPopInThemeAnimationStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub TargetNameProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub FromHorizontalOffsetProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub FromVerticalOffsetProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPopOutThemeAnimation(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPopOutThemeAnimation {
    type Vtable = IPopOutThemeAnimation_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4786ab49_0e48_4e81_a2e5_cc5aa19e48d3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPopOutThemeAnimation_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub TargetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetTargetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPopOutThemeAnimationStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPopOutThemeAnimationStatics {
    type Vtable = IPopOutThemeAnimationStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1d492c09_03c1_4490_99dc_909feab357fb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPopOutThemeAnimationStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub TargetNameProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPopupThemeTransition(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPopupThemeTransition {
    type Vtable = IPopupThemeTransition_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x47843552_4283_545e_c791_268dca22ce4b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPopupThemeTransition_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub FromHorizontalOffset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub SetFromHorizontalOffset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows_core::HRESULT,
    pub FromVerticalOffset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub SetFromVerticalOffset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPopupThemeTransitionStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPopupThemeTransitionStatics {
    type Vtable = IPopupThemeTransitionStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe5a1640e_490d_1505_9f6b_8fafc044dec5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPopupThemeTransitionStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub FromHorizontalOffsetProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub FromVerticalOffsetProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPowerEase(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPowerEase {
    type Vtable = IPowerEase_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x69c80579_eedf_405b_8680_d9606880c937);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPowerEase_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Power: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub SetPower: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPowerEaseStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPowerEaseStatics {
    type Vtable = IPowerEaseStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa5955103_91a2_460c_9c41_d28f6a939bda);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPowerEaseStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub PowerProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IQuadraticEase(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IQuadraticEase {
    type Vtable = IQuadraticEase_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe1510e91_ef6d_44f0_803d_68d16de0ddfc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IQuadraticEase_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IQuarticEase(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IQuarticEase {
    type Vtable = IQuarticEase_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe8698814_fe42_4a05_b5b8_081f41157815);
}
#[repr(C)]
#[doc(hidden)]
pub struct IQuarticEase_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IQuinticEase(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IQuinticEase {
    type Vtable = IQuinticEase_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x92ee793b_3c49_4108_aa11_ab786603da21);
}
#[repr(C)]
#[doc(hidden)]
pub struct IQuinticEase_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IReorderThemeTransition(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IReorderThemeTransition {
    type Vtable = IReorderThemeTransition_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf2065c6c_d052_4ad1_8362_b71b36df7497);
}
#[repr(C)]
#[doc(hidden)]
pub struct IReorderThemeTransition_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRepeatBehaviorHelper(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IRepeatBehaviorHelper {
    type Vtable = IRepeatBehaviorHelper_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6863ab72_4997_47f9_87ad_37efb75993ea);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRepeatBehaviorHelper_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRepeatBehaviorHelperStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IRepeatBehaviorHelperStatics {
    type Vtable = IRepeatBehaviorHelperStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7a795033_79f3_4dd9_b267_9cf50fb51f84);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRepeatBehaviorHelperStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Forever: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut RepeatBehavior) -> ::windows_core::HRESULT,
    pub FromCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: f64, result__: *mut RepeatBehavior) -> ::windows_core::HRESULT,
    pub FromDuration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, duration: ::winrt_foundation::TimeSpan, result__: *mut RepeatBehavior) -> ::windows_core::HRESULT,
    pub GetHasCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, target: RepeatBehavior, result__: *mut bool) -> ::windows_core::HRESULT,
    pub GetHasDuration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, target: RepeatBehavior, result__: *mut bool) -> ::windows_core::HRESULT,
    pub Equals: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, target: RepeatBehavior, value: RepeatBehavior, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRepositionThemeAnimation(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IRepositionThemeAnimation {
    type Vtable = IRepositionThemeAnimation_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xecda24e8_8945_4949_a1bf_62109965a7e9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRepositionThemeAnimation_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub TargetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetTargetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub FromHorizontalOffset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub SetFromHorizontalOffset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows_core::HRESULT,
    pub FromVerticalOffset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub SetFromVerticalOffset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRepositionThemeAnimationStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IRepositionThemeAnimationStatics {
    type Vtable = IRepositionThemeAnimationStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4d92b1b1_860b_4bf9_a59d_1eb1ccbe8fe0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRepositionThemeAnimationStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub TargetNameProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub FromHorizontalOffsetProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub FromVerticalOffsetProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRepositionThemeTransition(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IRepositionThemeTransition {
    type Vtable = IRepositionThemeTransition_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x88329b82_98f3_455a_ac53_2e7083b6e22c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRepositionThemeTransition_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRepositionThemeTransition2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IRepositionThemeTransition2 {
    type Vtable = IRepositionThemeTransition2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xcebfe864_dbea_4404_8e6e_de55ada75239);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRepositionThemeTransition2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub IsStaggeringEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetIsStaggeringEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRepositionThemeTransitionStatics2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IRepositionThemeTransitionStatics2 {
    type Vtable = IRepositionThemeTransitionStatics2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9240e930_0a19_468b_8c2a_68fab4500027);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRepositionThemeTransitionStatics2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub IsStaggeringEnabledProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISineEase(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISineEase {
    type Vtable = ISineEase_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa9382962_230b_49da_9e0d_664987892343);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISineEase_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISlideNavigationTransitionInfo(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISlideNavigationTransitionInfo {
    type Vtable = ISlideNavigationTransitionInfo_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd6ac9d77_2e03_405f_80ed_e62beef3668f);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISlideNavigationTransitionInfo_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISlideNavigationTransitionInfo2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISlideNavigationTransitionInfo2 {
    type Vtable = ISlideNavigationTransitionInfo2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x90e2d9c0_5c81_5001_8013_4fbfea4bf139);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISlideNavigationTransitionInfo2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Effect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SlideNavigationTransitionEffect) -> ::windows_core::HRESULT,
    pub SetEffect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: SlideNavigationTransitionEffect) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISlideNavigationTransitionInfoStatics2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISlideNavigationTransitionInfoStatics2 {
    type Vtable = ISlideNavigationTransitionInfoStatics2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8a861baa_981a_5ace_9f85_cb7fde648a67);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISlideNavigationTransitionInfoStatics2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub EffectProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISplineColorKeyFrame(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISplineColorKeyFrame {
    type Vtable = ISplineColorKeyFrame_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1a4a5941_1fe0_473a_8efe_4316d8c86229);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISplineColorKeyFrame_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub KeySpline: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetKeySpline: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISplineColorKeyFrameStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISplineColorKeyFrameStatics {
    type Vtable = ISplineColorKeyFrameStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x61d1d997_8589_4f2f_8fbb_7d03edc98dd3);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISplineColorKeyFrameStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub KeySplineProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISplineDoubleKeyFrame(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISplineDoubleKeyFrame {
    type Vtable = ISplineDoubleKeyFrame_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x00d72d38_6b2b_4843_838e_c8b115eec801);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISplineDoubleKeyFrame_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub KeySpline: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetKeySpline: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISplineDoubleKeyFrameStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISplineDoubleKeyFrameStatics {
    type Vtable = ISplineDoubleKeyFrameStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x060a8ffc_975f_4e4e_9ec7_13c5aee02062);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISplineDoubleKeyFrameStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub KeySplineProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISplinePointKeyFrame(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISplinePointKeyFrame {
    type Vtable = ISplinePointKeyFrame_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0f19f306_7036_494f_bc3c_780df0cc524a);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISplinePointKeyFrame_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub KeySpline: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetKeySpline: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISplinePointKeyFrameStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISplinePointKeyFrameStatics {
    type Vtable = ISplinePointKeyFrameStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe97a32c2_0a7a_4766_95cb_0d692611cb4c);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISplinePointKeyFrameStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub KeySplineProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISplitCloseThemeAnimation(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISplitCloseThemeAnimation {
    type Vtable = ISplitCloseThemeAnimation_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4f799518_ff39_4e90_bb74_2abd56027402);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISplitCloseThemeAnimation_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub OpenedTargetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetOpenedTargetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub OpenedTarget: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetOpenedTarget: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ClosedTargetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetClosedTargetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub ClosedTarget: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetClosedTarget: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ContentTargetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetContentTargetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub ContentTarget: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetContentTarget: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub OpenedLength: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub SetOpenedLength: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows_core::HRESULT,
    pub ClosedLength: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub SetClosedLength: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows_core::HRESULT,
    pub OffsetFromCenter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub SetOffsetFromCenter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-ui")]
    pub ContentTranslationDirection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Controls::Primitives::AnimationDirection) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-ui"))]
    ContentTranslationDirection: usize,
    #[cfg(feature = "winrt-ui")]
    pub SetContentTranslationDirection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Controls::Primitives::AnimationDirection) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-ui"))]
    SetContentTranslationDirection: usize,
    pub ContentTranslationOffset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub SetContentTranslationOffset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISplitCloseThemeAnimationStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISplitCloseThemeAnimationStatics {
    type Vtable = ISplitCloseThemeAnimationStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7aa94de9_cc9b_4e90_a11a_0050a2216a9e);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISplitCloseThemeAnimationStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub OpenedTargetNameProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub OpenedTargetProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ClosedTargetNameProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ClosedTargetProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ContentTargetNameProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ContentTargetProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub OpenedLengthProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ClosedLengthProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub OffsetFromCenterProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ContentTranslationDirectionProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ContentTranslationOffsetProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISplitOpenThemeAnimation(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISplitOpenThemeAnimation {
    type Vtable = ISplitOpenThemeAnimation_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x785fd7aa_5456_4639_8fd2_26bae6a5ffe4);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISplitOpenThemeAnimation_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub OpenedTargetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetOpenedTargetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub OpenedTarget: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetOpenedTarget: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ClosedTargetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetClosedTargetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub ClosedTarget: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetClosedTarget: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ContentTargetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetContentTargetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub ContentTarget: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetContentTarget: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub OpenedLength: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub SetOpenedLength: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows_core::HRESULT,
    pub ClosedLength: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub SetClosedLength: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows_core::HRESULT,
    pub OffsetFromCenter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub SetOffsetFromCenter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-ui")]
    pub ContentTranslationDirection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Controls::Primitives::AnimationDirection) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-ui"))]
    ContentTranslationDirection: usize,
    #[cfg(feature = "winrt-ui")]
    pub SetContentTranslationDirection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Controls::Primitives::AnimationDirection) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-ui"))]
    SetContentTranslationDirection: usize,
    pub ContentTranslationOffset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub SetContentTranslationOffset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISplitOpenThemeAnimationStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISplitOpenThemeAnimationStatics {
    type Vtable = ISplitOpenThemeAnimationStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8d4cfa89_3a91_458d_b0fb_4cad625cbf8d);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISplitOpenThemeAnimationStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub OpenedTargetNameProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub OpenedTargetProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ClosedTargetNameProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ClosedTargetProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ContentTargetNameProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ContentTargetProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub OpenedLengthProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ClosedLengthProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub OffsetFromCenterProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ContentTranslationDirectionProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ContentTranslationOffsetProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStoryboard(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IStoryboard {
    type Vtable = IStoryboard_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd45c1e6e_3594_460e_981a_32271bd3aa06);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStoryboard_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-foundation")]
    pub Children: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    Children: usize,
    pub Seek: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, offset: ::winrt_foundation::TimeSpan) -> ::windows_core::HRESULT,
    pub Stop: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Begin: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Pause: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Resume: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetCurrentState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ClockState) -> ::windows_core::HRESULT,
    pub GetCurrentTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::TimeSpan) -> ::windows_core::HRESULT,
    pub SeekAlignedToLastTick: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, offset: ::winrt_foundation::TimeSpan) -> ::windows_core::HRESULT,
    pub SkipToFill: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStoryboardStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IStoryboardStatics {
    type Vtable = IStoryboardStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd82f07d8_73d5_4379_bd48_7e05184a8bad);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStoryboardStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub TargetPropertyProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetTargetProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows_core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetTargetProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows_core::RawPtr, path: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub TargetNameProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetTargetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows_core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetTargetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: ::windows_core::RawPtr, name: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetTarget: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, timeline: ::windows_core::RawPtr, target: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISuppressNavigationTransitionInfo(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISuppressNavigationTransitionInfo {
    type Vtable = ISuppressNavigationTransitionInfo_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x244d7b0c_b1b7_4871_9d3e_d56203a3a5b4);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISuppressNavigationTransitionInfo_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISwipeBackThemeAnimation(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISwipeBackThemeAnimation {
    type Vtable = ISwipeBackThemeAnimation_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa38a4214_0bca_4d2d_95f7_ceba57fbaf60);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISwipeBackThemeAnimation_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub TargetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetTargetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub FromHorizontalOffset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub SetFromHorizontalOffset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows_core::HRESULT,
    pub FromVerticalOffset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub SetFromVerticalOffset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISwipeBackThemeAnimationStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISwipeBackThemeAnimationStatics {
    type Vtable = ISwipeBackThemeAnimationStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x693f31bf_4da6_468a_8ce0_996c9aad42e0);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISwipeBackThemeAnimationStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub TargetNameProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub FromHorizontalOffsetProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub FromVerticalOffsetProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISwipeHintThemeAnimation(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISwipeHintThemeAnimation {
    type Vtable = ISwipeHintThemeAnimation_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xcdd067c0_580e_4e40_be98_f202d3d84365);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISwipeHintThemeAnimation_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub TargetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetTargetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub ToHorizontalOffset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub SetToHorizontalOffset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows_core::HRESULT,
    pub ToVerticalOffset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub SetToVerticalOffset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISwipeHintThemeAnimationStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISwipeHintThemeAnimationStatics {
    type Vtable = ISwipeHintThemeAnimationStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x23d61a57_9115_4d63_b04a_b89f1c744dc0);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISwipeHintThemeAnimationStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub TargetNameProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ToHorizontalOffsetProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ToVerticalOffsetProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITimeline(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ITimeline {
    type Vtable = ITimeline_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0bc465dc_be4d_4d0d_9549_2208b715f40d);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITimeline_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub AutoReverse: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetAutoReverse: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub BeginTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetBeginTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Duration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Duration) -> ::windows_core::HRESULT,
    pub SetDuration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Duration) -> ::windows_core::HRESULT,
    pub SpeedRatio: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub SetSpeedRatio: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows_core::HRESULT,
    pub FillBehavior: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut FillBehavior) -> ::windows_core::HRESULT,
    pub SetFillBehavior: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: FillBehavior) -> ::windows_core::HRESULT,
    pub RepeatBehavior: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut RepeatBehavior) -> ::windows_core::HRESULT,
    pub SetRepeatBehavior: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: RepeatBehavior) -> ::windows_core::HRESULT,
    pub Completed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITimelineFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ITimelineFactory {
    type Vtable = ITimelineFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1d56bb07_bda4_478b_8ada_eb04d580cd5e);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITimelineFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CreateInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITimelineStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ITimelineStatics {
    type Vtable = ITimelineStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa902ed4e_ef10_4d6f_9a40_93cb8895f4e5);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITimelineStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub AllowDependentAnimations: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetAllowDependentAnimations: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub AutoReverseProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub BeginTimeProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub DurationProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SpeedRatioProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub FillBehaviorProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub RepeatBehaviorProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITransition(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ITransition {
    type Vtable = ITransition_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3c677c7c_01d0_4dce_b333_976f93312b08);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITransition_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITransitionFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ITransitionFactory {
    type Vtable = ITransitionFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xdc9ab2cf_3bc9_44aa_b3fc_883a83233a2c);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITransitionFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
}
#[repr(transparent)]
pub struct KeySpline(::windows_core::IUnknown);
impl KeySpline {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<KeySpline, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn ControlPoint1(&self) -> ::windows_core::Result<::winrt_foundation::Point> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::Point>::zeroed();
            (::windows_core::Interface::vtable(this).ControlPoint1)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Point>(result__)
        }
    }
    pub fn SetControlPoint1<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Point>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetControlPoint1)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn ControlPoint2(&self) -> ::windows_core::Result<::winrt_foundation::Point> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::Point>::zeroed();
            (::windows_core::Interface::vtable(this).ControlPoint2)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Point>(result__)
        }
    }
    pub fn SetControlPoint2<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Point>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetControlPoint2)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
}
impl ::core::clone::Clone for KeySpline {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for KeySpline {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for KeySpline {}
impl ::core::fmt::Debug for KeySpline {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KeySpline").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for KeySpline {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Animation.KeySpline;{77a163bb-d5ca-4a32-ba0b-7dff988e58a0})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for KeySpline {
    type Vtable = IKeySpline_Vtbl;
    const IID: ::windows_core::GUID = <IKeySpline as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for KeySpline {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.KeySpline";
}
impl ::core::convert::From<KeySpline> for ::windows_core::IUnknown {
    fn from(value: KeySpline) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&KeySpline> for ::windows_core::IUnknown {
    fn from(value: &KeySpline) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for KeySpline {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a KeySpline {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<KeySpline> for ::windows_core::IInspectable {
    fn from(value: KeySpline) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&KeySpline> for ::windows_core::IInspectable {
    fn from(value: &KeySpline) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for KeySpline {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a KeySpline {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<KeySpline> for super::super::DependencyObject {
    fn from(value: KeySpline) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&KeySpline> for super::super::DependencyObject {
    fn from(value: &KeySpline) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::DependencyObject> for KeySpline {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::DependencyObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::DependencyObject> for &KeySpline {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::DependencyObject> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for KeySpline {}
unsafe impl ::core::marker::Sync for KeySpline {}
#[repr(C)]
pub struct KeyTime {
    pub TimeSpan: ::winrt_foundation::TimeSpan,
}
impl ::core::marker::Copy for KeyTime {}
impl ::core::clone::Clone for KeyTime {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for KeyTime {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KeyTime").field("TimeSpan", &self.TimeSpan).finish()
    }
}
unsafe impl ::windows_core::Abi for KeyTime {
    type Abi = Self;
}
unsafe impl ::windows_core::RuntimeType for KeyTime {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"struct(Windows.UI.Xaml.Media.Animation.KeyTime;struct(Windows.Foundation.TimeSpan;i8))");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
impl ::core::cmp::PartialEq for KeyTime {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<KeyTime>()) == 0 }
    }
}
impl ::core::cmp::Eq for KeyTime {}
impl ::core::default::Default for KeyTime {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
pub struct KeyTimeHelper(::windows_core::IUnknown);
impl KeyTimeHelper {
    pub fn FromTimeSpan<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TimeSpan>>(timespan: Param0) -> ::windows_core::Result<KeyTime> {
        Self::IKeyTimeHelperStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<KeyTime>::zeroed();
            (::windows_core::Interface::vtable(this).FromTimeSpan)(::windows_core::Interface::as_raw(this), timespan.into_param().abi(), result__.as_mut_ptr()).from_abi::<KeyTime>(result__)
        })
    }
    pub fn IKeyTimeHelperStatics<R, F: FnOnce(&IKeyTimeHelperStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<KeyTimeHelper, IKeyTimeHelperStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for KeyTimeHelper {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for KeyTimeHelper {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for KeyTimeHelper {}
impl ::core::fmt::Debug for KeyTimeHelper {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KeyTimeHelper").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for KeyTimeHelper {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Animation.KeyTimeHelper;{3643e480-4823-466a-abe5-5e79c8ed77ed})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for KeyTimeHelper {
    type Vtable = IKeyTimeHelper_Vtbl;
    const IID: ::windows_core::GUID = <IKeyTimeHelper as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for KeyTimeHelper {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.KeyTimeHelper";
}
impl ::core::convert::From<KeyTimeHelper> for ::windows_core::IUnknown {
    fn from(value: KeyTimeHelper) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&KeyTimeHelper> for ::windows_core::IUnknown {
    fn from(value: &KeyTimeHelper) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for KeyTimeHelper {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a KeyTimeHelper {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<KeyTimeHelper> for ::windows_core::IInspectable {
    fn from(value: KeyTimeHelper) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&KeyTimeHelper> for ::windows_core::IInspectable {
    fn from(value: &KeyTimeHelper) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for KeyTimeHelper {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a KeyTimeHelper {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for KeyTimeHelper {}
unsafe impl ::core::marker::Sync for KeyTimeHelper {}
#[repr(transparent)]
pub struct LinearColorKeyFrame(::windows_core::IUnknown);
impl LinearColorKeyFrame {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<LinearColorKeyFrame, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for LinearColorKeyFrame {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for LinearColorKeyFrame {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for LinearColorKeyFrame {}
impl ::core::fmt::Debug for LinearColorKeyFrame {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LinearColorKeyFrame").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for LinearColorKeyFrame {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Animation.LinearColorKeyFrame;{66fdb6ef-ac81-4611-b1d2-61f545983f03})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for LinearColorKeyFrame {
    type Vtable = ILinearColorKeyFrame_Vtbl;
    const IID: ::windows_core::GUID = <ILinearColorKeyFrame as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for LinearColorKeyFrame {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.LinearColorKeyFrame";
}
impl ::core::convert::From<LinearColorKeyFrame> for ::windows_core::IUnknown {
    fn from(value: LinearColorKeyFrame) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LinearColorKeyFrame> for ::windows_core::IUnknown {
    fn from(value: &LinearColorKeyFrame) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for LinearColorKeyFrame {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a LinearColorKeyFrame {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<LinearColorKeyFrame> for ::windows_core::IInspectable {
    fn from(value: LinearColorKeyFrame) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LinearColorKeyFrame> for ::windows_core::IInspectable {
    fn from(value: &LinearColorKeyFrame) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for LinearColorKeyFrame {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a LinearColorKeyFrame {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<LinearColorKeyFrame> for ColorKeyFrame {
    fn from(value: LinearColorKeyFrame) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&LinearColorKeyFrame> for ColorKeyFrame {
    fn from(value: &LinearColorKeyFrame) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, ColorKeyFrame> for LinearColorKeyFrame {
    fn into_param(self) -> ::windows_core::Param<'a, ColorKeyFrame> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ColorKeyFrame> for &LinearColorKeyFrame {
    fn into_param(self) -> ::windows_core::Param<'a, ColorKeyFrame> {
        ::windows_core::Param::Owned(::core::convert::Into::<ColorKeyFrame>::into(self))
    }
}
impl ::core::convert::From<LinearColorKeyFrame> for super::super::DependencyObject {
    fn from(value: LinearColorKeyFrame) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&LinearColorKeyFrame> for super::super::DependencyObject {
    fn from(value: &LinearColorKeyFrame) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::DependencyObject> for LinearColorKeyFrame {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::DependencyObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::DependencyObject> for &LinearColorKeyFrame {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::DependencyObject> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for LinearColorKeyFrame {}
unsafe impl ::core::marker::Sync for LinearColorKeyFrame {}
#[repr(transparent)]
pub struct LinearDoubleKeyFrame(::windows_core::IUnknown);
impl LinearDoubleKeyFrame {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<LinearDoubleKeyFrame, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for LinearDoubleKeyFrame {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for LinearDoubleKeyFrame {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for LinearDoubleKeyFrame {}
impl ::core::fmt::Debug for LinearDoubleKeyFrame {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LinearDoubleKeyFrame").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for LinearDoubleKeyFrame {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Animation.LinearDoubleKeyFrame;{8efdf265-9a7b-431d-8f0c-14c56b5ea4d9})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for LinearDoubleKeyFrame {
    type Vtable = ILinearDoubleKeyFrame_Vtbl;
    const IID: ::windows_core::GUID = <ILinearDoubleKeyFrame as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for LinearDoubleKeyFrame {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.LinearDoubleKeyFrame";
}
impl ::core::convert::From<LinearDoubleKeyFrame> for ::windows_core::IUnknown {
    fn from(value: LinearDoubleKeyFrame) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LinearDoubleKeyFrame> for ::windows_core::IUnknown {
    fn from(value: &LinearDoubleKeyFrame) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for LinearDoubleKeyFrame {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a LinearDoubleKeyFrame {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<LinearDoubleKeyFrame> for ::windows_core::IInspectable {
    fn from(value: LinearDoubleKeyFrame) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LinearDoubleKeyFrame> for ::windows_core::IInspectable {
    fn from(value: &LinearDoubleKeyFrame) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for LinearDoubleKeyFrame {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a LinearDoubleKeyFrame {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<LinearDoubleKeyFrame> for DoubleKeyFrame {
    fn from(value: LinearDoubleKeyFrame) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&LinearDoubleKeyFrame> for DoubleKeyFrame {
    fn from(value: &LinearDoubleKeyFrame) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, DoubleKeyFrame> for LinearDoubleKeyFrame {
    fn into_param(self) -> ::windows_core::Param<'a, DoubleKeyFrame> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, DoubleKeyFrame> for &LinearDoubleKeyFrame {
    fn into_param(self) -> ::windows_core::Param<'a, DoubleKeyFrame> {
        ::windows_core::Param::Owned(::core::convert::Into::<DoubleKeyFrame>::into(self))
    }
}
impl ::core::convert::From<LinearDoubleKeyFrame> for super::super::DependencyObject {
    fn from(value: LinearDoubleKeyFrame) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&LinearDoubleKeyFrame> for super::super::DependencyObject {
    fn from(value: &LinearDoubleKeyFrame) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::DependencyObject> for LinearDoubleKeyFrame {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::DependencyObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::DependencyObject> for &LinearDoubleKeyFrame {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::DependencyObject> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for LinearDoubleKeyFrame {}
unsafe impl ::core::marker::Sync for LinearDoubleKeyFrame {}
#[repr(transparent)]
pub struct LinearPointKeyFrame(::windows_core::IUnknown);
impl LinearPointKeyFrame {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<LinearPointKeyFrame, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for LinearPointKeyFrame {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for LinearPointKeyFrame {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for LinearPointKeyFrame {}
impl ::core::fmt::Debug for LinearPointKeyFrame {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LinearPointKeyFrame").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for LinearPointKeyFrame {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Animation.LinearPointKeyFrame;{e7c9b8ef-af24-49ee-84f1-a86600a4e319})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for LinearPointKeyFrame {
    type Vtable = ILinearPointKeyFrame_Vtbl;
    const IID: ::windows_core::GUID = <ILinearPointKeyFrame as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for LinearPointKeyFrame {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.LinearPointKeyFrame";
}
impl ::core::convert::From<LinearPointKeyFrame> for ::windows_core::IUnknown {
    fn from(value: LinearPointKeyFrame) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LinearPointKeyFrame> for ::windows_core::IUnknown {
    fn from(value: &LinearPointKeyFrame) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for LinearPointKeyFrame {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a LinearPointKeyFrame {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<LinearPointKeyFrame> for ::windows_core::IInspectable {
    fn from(value: LinearPointKeyFrame) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LinearPointKeyFrame> for ::windows_core::IInspectable {
    fn from(value: &LinearPointKeyFrame) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for LinearPointKeyFrame {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a LinearPointKeyFrame {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<LinearPointKeyFrame> for PointKeyFrame {
    fn from(value: LinearPointKeyFrame) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&LinearPointKeyFrame> for PointKeyFrame {
    fn from(value: &LinearPointKeyFrame) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, PointKeyFrame> for LinearPointKeyFrame {
    fn into_param(self) -> ::windows_core::Param<'a, PointKeyFrame> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, PointKeyFrame> for &LinearPointKeyFrame {
    fn into_param(self) -> ::windows_core::Param<'a, PointKeyFrame> {
        ::windows_core::Param::Owned(::core::convert::Into::<PointKeyFrame>::into(self))
    }
}
impl ::core::convert::From<LinearPointKeyFrame> for super::super::DependencyObject {
    fn from(value: LinearPointKeyFrame) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&LinearPointKeyFrame> for super::super::DependencyObject {
    fn from(value: &LinearPointKeyFrame) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::DependencyObject> for LinearPointKeyFrame {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::DependencyObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::DependencyObject> for &LinearPointKeyFrame {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::DependencyObject> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for LinearPointKeyFrame {}
unsafe impl ::core::marker::Sync for LinearPointKeyFrame {}
#[repr(transparent)]
pub struct NavigationThemeTransition(::windows_core::IUnknown);
impl NavigationThemeTransition {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<NavigationThemeTransition, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn DefaultNavigationTransitionInfo(&self) -> ::windows_core::Result<NavigationTransitionInfo> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).DefaultNavigationTransitionInfo)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<NavigationTransitionInfo>(result__)
        }
    }
    pub fn SetDefaultNavigationTransitionInfo<'a, Param0: ::windows_core::IntoParam<'a, NavigationTransitionInfo>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetDefaultNavigationTransitionInfo)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn DefaultNavigationTransitionInfoProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::INavigationThemeTransitionStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).DefaultNavigationTransitionInfoProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn INavigationThemeTransitionStatics<R, F: FnOnce(&INavigationThemeTransitionStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<NavigationThemeTransition, INavigationThemeTransitionStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for NavigationThemeTransition {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for NavigationThemeTransition {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for NavigationThemeTransition {}
impl ::core::fmt::Debug for NavigationThemeTransition {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NavigationThemeTransition").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for NavigationThemeTransition {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Animation.NavigationThemeTransition;{8833848c-4eb7-41f2-8799-9eef0a213b73})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for NavigationThemeTransition {
    type Vtable = INavigationThemeTransition_Vtbl;
    const IID: ::windows_core::GUID = <INavigationThemeTransition as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for NavigationThemeTransition {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.NavigationThemeTransition";
}
impl ::core::convert::From<NavigationThemeTransition> for ::windows_core::IUnknown {
    fn from(value: NavigationThemeTransition) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&NavigationThemeTransition> for ::windows_core::IUnknown {
    fn from(value: &NavigationThemeTransition) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for NavigationThemeTransition {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a NavigationThemeTransition {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<NavigationThemeTransition> for ::windows_core::IInspectable {
    fn from(value: NavigationThemeTransition) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&NavigationThemeTransition> for ::windows_core::IInspectable {
    fn from(value: &NavigationThemeTransition) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for NavigationThemeTransition {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a NavigationThemeTransition {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<NavigationThemeTransition> for Transition {
    fn from(value: NavigationThemeTransition) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&NavigationThemeTransition> for Transition {
    fn from(value: &NavigationThemeTransition) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, Transition> for NavigationThemeTransition {
    fn into_param(self) -> ::windows_core::Param<'a, Transition> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, Transition> for &NavigationThemeTransition {
    fn into_param(self) -> ::windows_core::Param<'a, Transition> {
        ::windows_core::Param::Owned(::core::convert::Into::<Transition>::into(self))
    }
}
impl ::core::convert::From<NavigationThemeTransition> for super::super::DependencyObject {
    fn from(value: NavigationThemeTransition) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&NavigationThemeTransition> for super::super::DependencyObject {
    fn from(value: &NavigationThemeTransition) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::DependencyObject> for NavigationThemeTransition {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::DependencyObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::DependencyObject> for &NavigationThemeTransition {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::DependencyObject> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for NavigationThemeTransition {}
unsafe impl ::core::marker::Sync for NavigationThemeTransition {}
#[repr(transparent)]
pub struct NavigationTransitionInfo(::windows_core::IUnknown);
impl NavigationTransitionInfo {}
impl ::core::clone::Clone for NavigationTransitionInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for NavigationTransitionInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for NavigationTransitionInfo {}
impl ::core::fmt::Debug for NavigationTransitionInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NavigationTransitionInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for NavigationTransitionInfo {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Animation.NavigationTransitionInfo;{a9b05091-ae4a-4372-8625-21b7a8b98ca4})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for NavigationTransitionInfo {
    type Vtable = INavigationTransitionInfo_Vtbl;
    const IID: ::windows_core::GUID = <INavigationTransitionInfo as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for NavigationTransitionInfo {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.NavigationTransitionInfo";
}
impl ::core::convert::From<NavigationTransitionInfo> for ::windows_core::IUnknown {
    fn from(value: NavigationTransitionInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&NavigationTransitionInfo> for ::windows_core::IUnknown {
    fn from(value: &NavigationTransitionInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for NavigationTransitionInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a NavigationTransitionInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<NavigationTransitionInfo> for ::windows_core::IInspectable {
    fn from(value: NavigationTransitionInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&NavigationTransitionInfo> for ::windows_core::IInspectable {
    fn from(value: &NavigationTransitionInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for NavigationTransitionInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a NavigationTransitionInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<NavigationTransitionInfo> for super::super::DependencyObject {
    fn from(value: NavigationTransitionInfo) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&NavigationTransitionInfo> for super::super::DependencyObject {
    fn from(value: &NavigationTransitionInfo) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::DependencyObject> for NavigationTransitionInfo {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::DependencyObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::DependencyObject> for &NavigationTransitionInfo {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::DependencyObject> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for NavigationTransitionInfo {}
unsafe impl ::core::marker::Sync for NavigationTransitionInfo {}
#[repr(transparent)]
pub struct ObjectAnimationUsingKeyFrames(::windows_core::IUnknown);
impl ObjectAnimationUsingKeyFrames {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<ObjectAnimationUsingKeyFrames, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn KeyFrames(&self) -> ::windows_core::Result<ObjectKeyFrameCollection> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).KeyFrames)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ObjectKeyFrameCollection>(result__)
        }
    }
    pub fn EnableDependentAnimation(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).EnableDependentAnimation)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetEnableDependentAnimation(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetEnableDependentAnimation)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn EnableDependentAnimationProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::IObjectAnimationUsingKeyFramesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).EnableDependentAnimationProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn IObjectAnimationUsingKeyFramesStatics<R, F: FnOnce(&IObjectAnimationUsingKeyFramesStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<ObjectAnimationUsingKeyFrames, IObjectAnimationUsingKeyFramesStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for ObjectAnimationUsingKeyFrames {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ObjectAnimationUsingKeyFrames {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ObjectAnimationUsingKeyFrames {}
impl ::core::fmt::Debug for ObjectAnimationUsingKeyFrames {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ObjectAnimationUsingKeyFrames").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ObjectAnimationUsingKeyFrames {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Animation.ObjectAnimationUsingKeyFrames;{334a2d92-b74a-4c64-b9a6-58bcfa314f22})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for ObjectAnimationUsingKeyFrames {
    type Vtable = IObjectAnimationUsingKeyFrames_Vtbl;
    const IID: ::windows_core::GUID = <IObjectAnimationUsingKeyFrames as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for ObjectAnimationUsingKeyFrames {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.ObjectAnimationUsingKeyFrames";
}
impl ::core::convert::From<ObjectAnimationUsingKeyFrames> for ::windows_core::IUnknown {
    fn from(value: ObjectAnimationUsingKeyFrames) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ObjectAnimationUsingKeyFrames> for ::windows_core::IUnknown {
    fn from(value: &ObjectAnimationUsingKeyFrames) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ObjectAnimationUsingKeyFrames {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ObjectAnimationUsingKeyFrames {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ObjectAnimationUsingKeyFrames> for ::windows_core::IInspectable {
    fn from(value: ObjectAnimationUsingKeyFrames) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ObjectAnimationUsingKeyFrames> for ::windows_core::IInspectable {
    fn from(value: &ObjectAnimationUsingKeyFrames) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for ObjectAnimationUsingKeyFrames {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a ObjectAnimationUsingKeyFrames {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ObjectAnimationUsingKeyFrames> for Timeline {
    fn from(value: ObjectAnimationUsingKeyFrames) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&ObjectAnimationUsingKeyFrames> for Timeline {
    fn from(value: &ObjectAnimationUsingKeyFrames) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, Timeline> for ObjectAnimationUsingKeyFrames {
    fn into_param(self) -> ::windows_core::Param<'a, Timeline> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, Timeline> for &ObjectAnimationUsingKeyFrames {
    fn into_param(self) -> ::windows_core::Param<'a, Timeline> {
        ::windows_core::Param::Owned(::core::convert::Into::<Timeline>::into(self))
    }
}
impl ::core::convert::From<ObjectAnimationUsingKeyFrames> for super::super::DependencyObject {
    fn from(value: ObjectAnimationUsingKeyFrames) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&ObjectAnimationUsingKeyFrames> for super::super::DependencyObject {
    fn from(value: &ObjectAnimationUsingKeyFrames) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::DependencyObject> for ObjectAnimationUsingKeyFrames {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::DependencyObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::DependencyObject> for &ObjectAnimationUsingKeyFrames {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::DependencyObject> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for ObjectAnimationUsingKeyFrames {}
unsafe impl ::core::marker::Sync for ObjectAnimationUsingKeyFrames {}
#[repr(transparent)]
pub struct ObjectKeyFrame(::windows_core::IUnknown);
impl ObjectKeyFrame {
    pub fn Value(&self) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows_core::Interface::vtable(this).Value)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::IInspectable>(result__)
        }
    }
    pub fn SetValue<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::IInspectable>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetValue)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn KeyTime(&self) -> ::windows_core::Result<KeyTime> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<KeyTime>::zeroed();
            (::windows_core::Interface::vtable(this).KeyTime)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<KeyTime>(result__)
        }
    }
    pub fn SetKeyTime<'a, Param0: ::windows_core::IntoParam<'a, KeyTime>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetKeyTime)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn ValueProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::IObjectKeyFrameStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ValueProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn KeyTimeProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::IObjectKeyFrameStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).KeyTimeProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn IObjectKeyFrameStatics<R, F: FnOnce(&IObjectKeyFrameStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<ObjectKeyFrame, IObjectKeyFrameStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for ObjectKeyFrame {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ObjectKeyFrame {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ObjectKeyFrame {}
impl ::core::fmt::Debug for ObjectKeyFrame {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ObjectKeyFrame").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ObjectKeyFrame {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Animation.ObjectKeyFrame;{9852a851-8593-48ee-a6a4-d5d4720f029a})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for ObjectKeyFrame {
    type Vtable = IObjectKeyFrame_Vtbl;
    const IID: ::windows_core::GUID = <IObjectKeyFrame as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for ObjectKeyFrame {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.ObjectKeyFrame";
}
impl ::core::convert::From<ObjectKeyFrame> for ::windows_core::IUnknown {
    fn from(value: ObjectKeyFrame) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ObjectKeyFrame> for ::windows_core::IUnknown {
    fn from(value: &ObjectKeyFrame) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ObjectKeyFrame {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ObjectKeyFrame {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ObjectKeyFrame> for ::windows_core::IInspectable {
    fn from(value: ObjectKeyFrame) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ObjectKeyFrame> for ::windows_core::IInspectable {
    fn from(value: &ObjectKeyFrame) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for ObjectKeyFrame {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a ObjectKeyFrame {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ObjectKeyFrame> for super::super::DependencyObject {
    fn from(value: ObjectKeyFrame) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&ObjectKeyFrame> for super::super::DependencyObject {
    fn from(value: &ObjectKeyFrame) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::DependencyObject> for ObjectKeyFrame {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::DependencyObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::DependencyObject> for &ObjectKeyFrame {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::DependencyObject> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for ObjectKeyFrame {}
unsafe impl ::core::marker::Sync for ObjectKeyFrame {}
#[cfg(feature = "winrt-foundation")]
#[repr(transparent)]
pub struct ObjectKeyFrameCollection(::windows_core::IUnknown);
#[cfg(feature = "winrt-foundation")]
impl ObjectKeyFrameCollection {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<ObjectKeyFrameCollection, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn First(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IIterator<ObjectKeyFrame>> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IIterable<ObjectKeyFrame>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).First)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IIterator<ObjectKeyFrame>>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn GetAt(&self, index: u32) -> ::windows_core::Result<ObjectKeyFrame> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetAt)(::windows_core::Interface::as_raw(this), index, result__.as_mut_ptr()).from_abi::<ObjectKeyFrame>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn Size(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).Size)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn GetView(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<ObjectKeyFrame>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetView)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<ObjectKeyFrame>>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn IndexOf<'a, Param0: ::windows_core::IntoParam<'a, ObjectKeyFrame>>(&self, value: Param0, index: &mut u32) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IndexOf)(::windows_core::Interface::as_raw(this), value.into_param().abi(), index, result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn SetAt<'a, Param1: ::windows_core::IntoParam<'a, ObjectKeyFrame>>(&self, index: u32, value: Param1) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetAt)(::windows_core::Interface::as_raw(this), index, value.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn InsertAt<'a, Param1: ::windows_core::IntoParam<'a, ObjectKeyFrame>>(&self, index: u32, value: Param1) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).InsertAt)(::windows_core::Interface::as_raw(this), index, value.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn RemoveAt(&self, index: u32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveAt)(::windows_core::Interface::as_raw(this), index).ok() }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn Append<'a, Param0: ::windows_core::IntoParam<'a, ObjectKeyFrame>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Append)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn RemoveAtEnd(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveAtEnd)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn Clear(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Clear)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn GetMany(&self, startindex: u32, items: &mut [::core::option::Option<ObjectKeyFrame>]) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).GetMany)(::windows_core::Interface::as_raw(this), startindex, items.len() as u32, ::core::mem::transmute_copy(&items), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn ReplaceAll(&self, items: &[::core::option::Option<ObjectKeyFrame>]) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).ReplaceAll)(::windows_core::Interface::as_raw(this), items.len() as u32, ::core::mem::transmute(items.as_ptr())).ok() }
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::clone::Clone for ObjectKeyFrameCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::cmp::PartialEq for ObjectKeyFrameCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::cmp::Eq for ObjectKeyFrameCollection {}
#[cfg(feature = "winrt-foundation")]
impl ::core::fmt::Debug for ObjectKeyFrameCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ObjectKeyFrameCollection").field(&self.0).finish()
    }
}
#[cfg(feature = "winrt-foundation")]
unsafe impl ::windows_core::RuntimeType for ObjectKeyFrameCollection {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Animation.ObjectKeyFrameCollection;pinterface({913337e9-11a1-4345-a3a2-4e7f956e222d};rc(Windows.UI.Xaml.Media.Animation.ObjectKeyFrame;{9852a851-8593-48ee-a6a4-d5d4720f029a})))");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[cfg(feature = "winrt-foundation")]
unsafe impl ::windows_core::Interface for ObjectKeyFrameCollection {
    type Vtable = ::winrt_foundation::Collections::IVector_Vtbl<ObjectKeyFrame>;
    const IID: ::windows_core::GUID = <::winrt_foundation::Collections::IVector<ObjectKeyFrame> as ::windows_core::Interface>::IID;
}
#[cfg(feature = "winrt-foundation")]
impl ::windows_core::RuntimeName for ObjectKeyFrameCollection {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.ObjectKeyFrameCollection";
}
#[cfg(feature = "winrt-foundation")]
impl ::core::iter::IntoIterator for ObjectKeyFrameCollection {
    type Item = ObjectKeyFrame;
    type IntoIter = ::winrt_foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::iter::IntoIterator for &ObjectKeyFrameCollection {
    type Item = ObjectKeyFrame;
    type IntoIter = ::winrt_foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::winrt_foundation::Collections::VectorIterator::new(::core::convert::TryInto::try_into(self).ok())
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::From<ObjectKeyFrameCollection> for ::windows_core::IUnknown {
    fn from(value: ObjectKeyFrameCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::From<&ObjectKeyFrameCollection> for ::windows_core::IUnknown {
    fn from(value: &ObjectKeyFrameCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ObjectKeyFrameCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ObjectKeyFrameCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::From<ObjectKeyFrameCollection> for ::windows_core::IInspectable {
    fn from(value: ObjectKeyFrameCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::From<&ObjectKeyFrameCollection> for ::windows_core::IInspectable {
    fn from(value: &ObjectKeyFrameCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for ObjectKeyFrameCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a ObjectKeyFrameCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::TryFrom<ObjectKeyFrameCollection> for ::winrt_foundation::Collections::IIterable<ObjectKeyFrame> {
    type Error = ::windows_core::Error;
    fn try_from(value: ObjectKeyFrameCollection) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::TryFrom<&ObjectKeyFrameCollection> for ::winrt_foundation::Collections::IIterable<ObjectKeyFrame> {
    type Error = ::windows_core::Error;
    fn try_from(value: &ObjectKeyFrameCollection) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<ObjectKeyFrame>> for ObjectKeyFrameCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::Collections::IIterable<ObjectKeyFrame>> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<ObjectKeyFrame>> for &ObjectKeyFrameCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::Collections::IIterable<ObjectKeyFrame>> {
        ::core::convert::TryInto::<::winrt_foundation::Collections::IIterable<ObjectKeyFrame>>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::TryFrom<ObjectKeyFrameCollection> for ::winrt_foundation::Collections::IVector<ObjectKeyFrame> {
    type Error = ::windows_core::Error;
    fn try_from(value: ObjectKeyFrameCollection) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::TryFrom<&ObjectKeyFrameCollection> for ::winrt_foundation::Collections::IVector<ObjectKeyFrame> {
    type Error = ::windows_core::Error;
    fn try_from(value: &ObjectKeyFrameCollection) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IVector<ObjectKeyFrame>> for ObjectKeyFrameCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::Collections::IVector<ObjectKeyFrame>> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IVector<ObjectKeyFrame>> for &ObjectKeyFrameCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::Collections::IVector<ObjectKeyFrame>> {
        ::core::convert::TryInto::<::winrt_foundation::Collections::IVector<ObjectKeyFrame>>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "winrt-foundation")]
unsafe impl ::core::marker::Send for ObjectKeyFrameCollection {}
#[cfg(feature = "winrt-foundation")]
unsafe impl ::core::marker::Sync for ObjectKeyFrameCollection {}
#[repr(transparent)]
pub struct PaneThemeTransition(::windows_core::IUnknown);
impl PaneThemeTransition {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<PaneThemeTransition, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "winrt-ui")]
    pub fn Edge(&self) -> ::windows_core::Result<super::super::Controls::Primitives::EdgeTransitionLocation> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::Controls::Primitives::EdgeTransitionLocation>::zeroed();
            (::windows_core::Interface::vtable(this).Edge)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Controls::Primitives::EdgeTransitionLocation>(result__)
        }
    }
    #[cfg(feature = "winrt-ui")]
    pub fn SetEdge(&self, value: super::super::Controls::Primitives::EdgeTransitionLocation) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetEdge)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn EdgeProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::IPaneThemeTransitionStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).EdgeProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn IPaneThemeTransitionStatics<R, F: FnOnce(&IPaneThemeTransitionStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<PaneThemeTransition, IPaneThemeTransitionStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for PaneThemeTransition {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PaneThemeTransition {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PaneThemeTransition {}
impl ::core::fmt::Debug for PaneThemeTransition {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PaneThemeTransition").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PaneThemeTransition {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Animation.PaneThemeTransition;{4708eb8e-4bfc-ee46-d4f9-708def3fbb2b})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for PaneThemeTransition {
    type Vtable = IPaneThemeTransition_Vtbl;
    const IID: ::windows_core::GUID = <IPaneThemeTransition as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for PaneThemeTransition {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.PaneThemeTransition";
}
impl ::core::convert::From<PaneThemeTransition> for ::windows_core::IUnknown {
    fn from(value: PaneThemeTransition) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PaneThemeTransition> for ::windows_core::IUnknown {
    fn from(value: &PaneThemeTransition) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for PaneThemeTransition {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a PaneThemeTransition {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PaneThemeTransition> for ::windows_core::IInspectable {
    fn from(value: PaneThemeTransition) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PaneThemeTransition> for ::windows_core::IInspectable {
    fn from(value: &PaneThemeTransition) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for PaneThemeTransition {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a PaneThemeTransition {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PaneThemeTransition> for Transition {
    fn from(value: PaneThemeTransition) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&PaneThemeTransition> for Transition {
    fn from(value: &PaneThemeTransition) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, Transition> for PaneThemeTransition {
    fn into_param(self) -> ::windows_core::Param<'a, Transition> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, Transition> for &PaneThemeTransition {
    fn into_param(self) -> ::windows_core::Param<'a, Transition> {
        ::windows_core::Param::Owned(::core::convert::Into::<Transition>::into(self))
    }
}
impl ::core::convert::From<PaneThemeTransition> for super::super::DependencyObject {
    fn from(value: PaneThemeTransition) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&PaneThemeTransition> for super::super::DependencyObject {
    fn from(value: &PaneThemeTransition) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::DependencyObject> for PaneThemeTransition {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::DependencyObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::DependencyObject> for &PaneThemeTransition {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::DependencyObject> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for PaneThemeTransition {}
unsafe impl ::core::marker::Sync for PaneThemeTransition {}
#[repr(transparent)]
pub struct PointAnimation(::windows_core::IUnknown);
impl PointAnimation {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<PointAnimation, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn From(&self) -> ::windows_core::Result<::winrt_foundation::IReference<::winrt_foundation::Point>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).From)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IReference<::winrt_foundation::Point>>(result__)
        }
    }
    pub fn SetFrom<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::IReference<::winrt_foundation::Point>>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetFrom)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn To(&self) -> ::windows_core::Result<::winrt_foundation::IReference<::winrt_foundation::Point>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).To)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IReference<::winrt_foundation::Point>>(result__)
        }
    }
    pub fn SetTo<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::IReference<::winrt_foundation::Point>>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetTo)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn By(&self) -> ::windows_core::Result<::winrt_foundation::IReference<::winrt_foundation::Point>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).By)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IReference<::winrt_foundation::Point>>(result__)
        }
    }
    pub fn SetBy<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::IReference<::winrt_foundation::Point>>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetBy)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn EasingFunction(&self) -> ::windows_core::Result<EasingFunctionBase> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).EasingFunction)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<EasingFunctionBase>(result__)
        }
    }
    pub fn SetEasingFunction<'a, Param0: ::windows_core::IntoParam<'a, EasingFunctionBase>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetEasingFunction)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn EnableDependentAnimation(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).EnableDependentAnimation)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetEnableDependentAnimation(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetEnableDependentAnimation)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn FromProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::IPointAnimationStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FromProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn ToProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::IPointAnimationStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ToProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn ByProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::IPointAnimationStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ByProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn EasingFunctionProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::IPointAnimationStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).EasingFunctionProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn EnableDependentAnimationProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::IPointAnimationStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).EnableDependentAnimationProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn IPointAnimationStatics<R, F: FnOnce(&IPointAnimationStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<PointAnimation, IPointAnimationStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for PointAnimation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PointAnimation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PointAnimation {}
impl ::core::fmt::Debug for PointAnimation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PointAnimation").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PointAnimation {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Animation.PointAnimation;{30f04312-7726-4f88-b8e2-2fa54518963b})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for PointAnimation {
    type Vtable = IPointAnimation_Vtbl;
    const IID: ::windows_core::GUID = <IPointAnimation as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for PointAnimation {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.PointAnimation";
}
impl ::core::convert::From<PointAnimation> for ::windows_core::IUnknown {
    fn from(value: PointAnimation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PointAnimation> for ::windows_core::IUnknown {
    fn from(value: &PointAnimation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for PointAnimation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a PointAnimation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PointAnimation> for ::windows_core::IInspectable {
    fn from(value: PointAnimation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PointAnimation> for ::windows_core::IInspectable {
    fn from(value: &PointAnimation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for PointAnimation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a PointAnimation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PointAnimation> for Timeline {
    fn from(value: PointAnimation) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&PointAnimation> for Timeline {
    fn from(value: &PointAnimation) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, Timeline> for PointAnimation {
    fn into_param(self) -> ::windows_core::Param<'a, Timeline> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, Timeline> for &PointAnimation {
    fn into_param(self) -> ::windows_core::Param<'a, Timeline> {
        ::windows_core::Param::Owned(::core::convert::Into::<Timeline>::into(self))
    }
}
impl ::core::convert::From<PointAnimation> for super::super::DependencyObject {
    fn from(value: PointAnimation) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&PointAnimation> for super::super::DependencyObject {
    fn from(value: &PointAnimation) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::DependencyObject> for PointAnimation {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::DependencyObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::DependencyObject> for &PointAnimation {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::DependencyObject> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for PointAnimation {}
unsafe impl ::core::marker::Sync for PointAnimation {}
#[repr(transparent)]
pub struct PointAnimationUsingKeyFrames(::windows_core::IUnknown);
impl PointAnimationUsingKeyFrames {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<PointAnimationUsingKeyFrames, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn KeyFrames(&self) -> ::windows_core::Result<PointKeyFrameCollection> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).KeyFrames)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PointKeyFrameCollection>(result__)
        }
    }
    pub fn EnableDependentAnimation(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).EnableDependentAnimation)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetEnableDependentAnimation(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetEnableDependentAnimation)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn EnableDependentAnimationProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::IPointAnimationUsingKeyFramesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).EnableDependentAnimationProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn IPointAnimationUsingKeyFramesStatics<R, F: FnOnce(&IPointAnimationUsingKeyFramesStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<PointAnimationUsingKeyFrames, IPointAnimationUsingKeyFramesStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for PointAnimationUsingKeyFrames {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PointAnimationUsingKeyFrames {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PointAnimationUsingKeyFrames {}
impl ::core::fmt::Debug for PointAnimationUsingKeyFrames {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PointAnimationUsingKeyFrames").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PointAnimationUsingKeyFrames {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Animation.PointAnimationUsingKeyFrames;{9b944f72-446a-41d0-a129-41a620f4595d})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for PointAnimationUsingKeyFrames {
    type Vtable = IPointAnimationUsingKeyFrames_Vtbl;
    const IID: ::windows_core::GUID = <IPointAnimationUsingKeyFrames as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for PointAnimationUsingKeyFrames {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.PointAnimationUsingKeyFrames";
}
impl ::core::convert::From<PointAnimationUsingKeyFrames> for ::windows_core::IUnknown {
    fn from(value: PointAnimationUsingKeyFrames) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PointAnimationUsingKeyFrames> for ::windows_core::IUnknown {
    fn from(value: &PointAnimationUsingKeyFrames) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for PointAnimationUsingKeyFrames {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a PointAnimationUsingKeyFrames {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PointAnimationUsingKeyFrames> for ::windows_core::IInspectable {
    fn from(value: PointAnimationUsingKeyFrames) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PointAnimationUsingKeyFrames> for ::windows_core::IInspectable {
    fn from(value: &PointAnimationUsingKeyFrames) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for PointAnimationUsingKeyFrames {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a PointAnimationUsingKeyFrames {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PointAnimationUsingKeyFrames> for Timeline {
    fn from(value: PointAnimationUsingKeyFrames) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&PointAnimationUsingKeyFrames> for Timeline {
    fn from(value: &PointAnimationUsingKeyFrames) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, Timeline> for PointAnimationUsingKeyFrames {
    fn into_param(self) -> ::windows_core::Param<'a, Timeline> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, Timeline> for &PointAnimationUsingKeyFrames {
    fn into_param(self) -> ::windows_core::Param<'a, Timeline> {
        ::windows_core::Param::Owned(::core::convert::Into::<Timeline>::into(self))
    }
}
impl ::core::convert::From<PointAnimationUsingKeyFrames> for super::super::DependencyObject {
    fn from(value: PointAnimationUsingKeyFrames) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&PointAnimationUsingKeyFrames> for super::super::DependencyObject {
    fn from(value: &PointAnimationUsingKeyFrames) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::DependencyObject> for PointAnimationUsingKeyFrames {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::DependencyObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::DependencyObject> for &PointAnimationUsingKeyFrames {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::DependencyObject> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for PointAnimationUsingKeyFrames {}
unsafe impl ::core::marker::Sync for PointAnimationUsingKeyFrames {}
#[repr(transparent)]
pub struct PointKeyFrame(::windows_core::IUnknown);
impl PointKeyFrame {
    pub fn Value(&self) -> ::windows_core::Result<::winrt_foundation::Point> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::Point>::zeroed();
            (::windows_core::Interface::vtable(this).Value)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Point>(result__)
        }
    }
    pub fn SetValue<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Point>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetValue)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn KeyTime(&self) -> ::windows_core::Result<KeyTime> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<KeyTime>::zeroed();
            (::windows_core::Interface::vtable(this).KeyTime)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<KeyTime>(result__)
        }
    }
    pub fn SetKeyTime<'a, Param0: ::windows_core::IntoParam<'a, KeyTime>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetKeyTime)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn ValueProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::IPointKeyFrameStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ValueProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn KeyTimeProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::IPointKeyFrameStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).KeyTimeProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn IPointKeyFrameStatics<R, F: FnOnce(&IPointKeyFrameStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<PointKeyFrame, IPointKeyFrameStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for PointKeyFrame {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PointKeyFrame {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PointKeyFrame {}
impl ::core::fmt::Debug for PointKeyFrame {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PointKeyFrame").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PointKeyFrame {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Animation.PointKeyFrame;{fcc88d01-7f82-4dae-8026-7b7e086878b3})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for PointKeyFrame {
    type Vtable = IPointKeyFrame_Vtbl;
    const IID: ::windows_core::GUID = <IPointKeyFrame as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for PointKeyFrame {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.PointKeyFrame";
}
impl ::core::convert::From<PointKeyFrame> for ::windows_core::IUnknown {
    fn from(value: PointKeyFrame) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PointKeyFrame> for ::windows_core::IUnknown {
    fn from(value: &PointKeyFrame) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for PointKeyFrame {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a PointKeyFrame {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PointKeyFrame> for ::windows_core::IInspectable {
    fn from(value: PointKeyFrame) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PointKeyFrame> for ::windows_core::IInspectable {
    fn from(value: &PointKeyFrame) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for PointKeyFrame {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a PointKeyFrame {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PointKeyFrame> for super::super::DependencyObject {
    fn from(value: PointKeyFrame) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&PointKeyFrame> for super::super::DependencyObject {
    fn from(value: &PointKeyFrame) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::DependencyObject> for PointKeyFrame {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::DependencyObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::DependencyObject> for &PointKeyFrame {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::DependencyObject> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for PointKeyFrame {}
unsafe impl ::core::marker::Sync for PointKeyFrame {}
#[cfg(feature = "winrt-foundation")]
#[repr(transparent)]
pub struct PointKeyFrameCollection(::windows_core::IUnknown);
#[cfg(feature = "winrt-foundation")]
impl PointKeyFrameCollection {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<PointKeyFrameCollection, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn First(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IIterator<PointKeyFrame>> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IIterable<PointKeyFrame>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).First)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IIterator<PointKeyFrame>>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn GetAt(&self, index: u32) -> ::windows_core::Result<PointKeyFrame> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetAt)(::windows_core::Interface::as_raw(this), index, result__.as_mut_ptr()).from_abi::<PointKeyFrame>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn Size(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).Size)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn GetView(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<PointKeyFrame>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetView)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<PointKeyFrame>>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn IndexOf<'a, Param0: ::windows_core::IntoParam<'a, PointKeyFrame>>(&self, value: Param0, index: &mut u32) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IndexOf)(::windows_core::Interface::as_raw(this), value.into_param().abi(), index, result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn SetAt<'a, Param1: ::windows_core::IntoParam<'a, PointKeyFrame>>(&self, index: u32, value: Param1) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetAt)(::windows_core::Interface::as_raw(this), index, value.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn InsertAt<'a, Param1: ::windows_core::IntoParam<'a, PointKeyFrame>>(&self, index: u32, value: Param1) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).InsertAt)(::windows_core::Interface::as_raw(this), index, value.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn RemoveAt(&self, index: u32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveAt)(::windows_core::Interface::as_raw(this), index).ok() }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn Append<'a, Param0: ::windows_core::IntoParam<'a, PointKeyFrame>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Append)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn RemoveAtEnd(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveAtEnd)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn Clear(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Clear)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn GetMany(&self, startindex: u32, items: &mut [::core::option::Option<PointKeyFrame>]) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).GetMany)(::windows_core::Interface::as_raw(this), startindex, items.len() as u32, ::core::mem::transmute_copy(&items), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn ReplaceAll(&self, items: &[::core::option::Option<PointKeyFrame>]) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).ReplaceAll)(::windows_core::Interface::as_raw(this), items.len() as u32, ::core::mem::transmute(items.as_ptr())).ok() }
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::clone::Clone for PointKeyFrameCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::cmp::PartialEq for PointKeyFrameCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::cmp::Eq for PointKeyFrameCollection {}
#[cfg(feature = "winrt-foundation")]
impl ::core::fmt::Debug for PointKeyFrameCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PointKeyFrameCollection").field(&self.0).finish()
    }
}
#[cfg(feature = "winrt-foundation")]
unsafe impl ::windows_core::RuntimeType for PointKeyFrameCollection {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Animation.PointKeyFrameCollection;pinterface({913337e9-11a1-4345-a3a2-4e7f956e222d};rc(Windows.UI.Xaml.Media.Animation.PointKeyFrame;{fcc88d01-7f82-4dae-8026-7b7e086878b3})))");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[cfg(feature = "winrt-foundation")]
unsafe impl ::windows_core::Interface for PointKeyFrameCollection {
    type Vtable = ::winrt_foundation::Collections::IVector_Vtbl<PointKeyFrame>;
    const IID: ::windows_core::GUID = <::winrt_foundation::Collections::IVector<PointKeyFrame> as ::windows_core::Interface>::IID;
}
#[cfg(feature = "winrt-foundation")]
impl ::windows_core::RuntimeName for PointKeyFrameCollection {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.PointKeyFrameCollection";
}
#[cfg(feature = "winrt-foundation")]
impl ::core::iter::IntoIterator for PointKeyFrameCollection {
    type Item = PointKeyFrame;
    type IntoIter = ::winrt_foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::iter::IntoIterator for &PointKeyFrameCollection {
    type Item = PointKeyFrame;
    type IntoIter = ::winrt_foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::winrt_foundation::Collections::VectorIterator::new(::core::convert::TryInto::try_into(self).ok())
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::From<PointKeyFrameCollection> for ::windows_core::IUnknown {
    fn from(value: PointKeyFrameCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::From<&PointKeyFrameCollection> for ::windows_core::IUnknown {
    fn from(value: &PointKeyFrameCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for PointKeyFrameCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a PointKeyFrameCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::From<PointKeyFrameCollection> for ::windows_core::IInspectable {
    fn from(value: PointKeyFrameCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::From<&PointKeyFrameCollection> for ::windows_core::IInspectable {
    fn from(value: &PointKeyFrameCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for PointKeyFrameCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a PointKeyFrameCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::TryFrom<PointKeyFrameCollection> for ::winrt_foundation::Collections::IIterable<PointKeyFrame> {
    type Error = ::windows_core::Error;
    fn try_from(value: PointKeyFrameCollection) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::TryFrom<&PointKeyFrameCollection> for ::winrt_foundation::Collections::IIterable<PointKeyFrame> {
    type Error = ::windows_core::Error;
    fn try_from(value: &PointKeyFrameCollection) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<PointKeyFrame>> for PointKeyFrameCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::Collections::IIterable<PointKeyFrame>> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<PointKeyFrame>> for &PointKeyFrameCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::Collections::IIterable<PointKeyFrame>> {
        ::core::convert::TryInto::<::winrt_foundation::Collections::IIterable<PointKeyFrame>>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::TryFrom<PointKeyFrameCollection> for ::winrt_foundation::Collections::IVector<PointKeyFrame> {
    type Error = ::windows_core::Error;
    fn try_from(value: PointKeyFrameCollection) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::TryFrom<&PointKeyFrameCollection> for ::winrt_foundation::Collections::IVector<PointKeyFrame> {
    type Error = ::windows_core::Error;
    fn try_from(value: &PointKeyFrameCollection) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IVector<PointKeyFrame>> for PointKeyFrameCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::Collections::IVector<PointKeyFrame>> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IVector<PointKeyFrame>> for &PointKeyFrameCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::Collections::IVector<PointKeyFrame>> {
        ::core::convert::TryInto::<::winrt_foundation::Collections::IVector<PointKeyFrame>>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "winrt-foundation")]
unsafe impl ::core::marker::Send for PointKeyFrameCollection {}
#[cfg(feature = "winrt-foundation")]
unsafe impl ::core::marker::Sync for PointKeyFrameCollection {}
#[repr(transparent)]
pub struct PointerDownThemeAnimation(::windows_core::IUnknown);
impl PointerDownThemeAnimation {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<PointerDownThemeAnimation, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn TargetName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).TargetName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetTargetName<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetTargetName)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn TargetNameProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::IPointerDownThemeAnimationStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TargetNameProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn IPointerDownThemeAnimationStatics<R, F: FnOnce(&IPointerDownThemeAnimationStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<PointerDownThemeAnimation, IPointerDownThemeAnimationStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for PointerDownThemeAnimation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PointerDownThemeAnimation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PointerDownThemeAnimation {}
impl ::core::fmt::Debug for PointerDownThemeAnimation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PointerDownThemeAnimation").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PointerDownThemeAnimation {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Animation.PointerDownThemeAnimation;{b58e714e-c49d-4788-a233-0ae85d99dd5a})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for PointerDownThemeAnimation {
    type Vtable = IPointerDownThemeAnimation_Vtbl;
    const IID: ::windows_core::GUID = <IPointerDownThemeAnimation as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for PointerDownThemeAnimation {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.PointerDownThemeAnimation";
}
impl ::core::convert::From<PointerDownThemeAnimation> for ::windows_core::IUnknown {
    fn from(value: PointerDownThemeAnimation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PointerDownThemeAnimation> for ::windows_core::IUnknown {
    fn from(value: &PointerDownThemeAnimation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for PointerDownThemeAnimation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a PointerDownThemeAnimation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PointerDownThemeAnimation> for ::windows_core::IInspectable {
    fn from(value: PointerDownThemeAnimation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PointerDownThemeAnimation> for ::windows_core::IInspectable {
    fn from(value: &PointerDownThemeAnimation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for PointerDownThemeAnimation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a PointerDownThemeAnimation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PointerDownThemeAnimation> for Timeline {
    fn from(value: PointerDownThemeAnimation) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&PointerDownThemeAnimation> for Timeline {
    fn from(value: &PointerDownThemeAnimation) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, Timeline> for PointerDownThemeAnimation {
    fn into_param(self) -> ::windows_core::Param<'a, Timeline> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, Timeline> for &PointerDownThemeAnimation {
    fn into_param(self) -> ::windows_core::Param<'a, Timeline> {
        ::windows_core::Param::Owned(::core::convert::Into::<Timeline>::into(self))
    }
}
impl ::core::convert::From<PointerDownThemeAnimation> for super::super::DependencyObject {
    fn from(value: PointerDownThemeAnimation) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&PointerDownThemeAnimation> for super::super::DependencyObject {
    fn from(value: &PointerDownThemeAnimation) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::DependencyObject> for PointerDownThemeAnimation {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::DependencyObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::DependencyObject> for &PointerDownThemeAnimation {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::DependencyObject> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for PointerDownThemeAnimation {}
unsafe impl ::core::marker::Sync for PointerDownThemeAnimation {}
#[repr(transparent)]
pub struct PointerUpThemeAnimation(::windows_core::IUnknown);
impl PointerUpThemeAnimation {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<PointerUpThemeAnimation, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn TargetName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).TargetName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetTargetName<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetTargetName)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn TargetNameProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::IPointerUpThemeAnimationStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TargetNameProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn IPointerUpThemeAnimationStatics<R, F: FnOnce(&IPointerUpThemeAnimationStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<PointerUpThemeAnimation, IPointerUpThemeAnimationStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for PointerUpThemeAnimation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PointerUpThemeAnimation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PointerUpThemeAnimation {}
impl ::core::fmt::Debug for PointerUpThemeAnimation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PointerUpThemeAnimation").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PointerUpThemeAnimation {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Animation.PointerUpThemeAnimation;{e9e9d07d-6340-4828-ad12-690694b9910b})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for PointerUpThemeAnimation {
    type Vtable = IPointerUpThemeAnimation_Vtbl;
    const IID: ::windows_core::GUID = <IPointerUpThemeAnimation as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for PointerUpThemeAnimation {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.PointerUpThemeAnimation";
}
impl ::core::convert::From<PointerUpThemeAnimation> for ::windows_core::IUnknown {
    fn from(value: PointerUpThemeAnimation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PointerUpThemeAnimation> for ::windows_core::IUnknown {
    fn from(value: &PointerUpThemeAnimation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for PointerUpThemeAnimation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a PointerUpThemeAnimation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PointerUpThemeAnimation> for ::windows_core::IInspectable {
    fn from(value: PointerUpThemeAnimation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PointerUpThemeAnimation> for ::windows_core::IInspectable {
    fn from(value: &PointerUpThemeAnimation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for PointerUpThemeAnimation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a PointerUpThemeAnimation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PointerUpThemeAnimation> for Timeline {
    fn from(value: PointerUpThemeAnimation) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&PointerUpThemeAnimation> for Timeline {
    fn from(value: &PointerUpThemeAnimation) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, Timeline> for PointerUpThemeAnimation {
    fn into_param(self) -> ::windows_core::Param<'a, Timeline> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, Timeline> for &PointerUpThemeAnimation {
    fn into_param(self) -> ::windows_core::Param<'a, Timeline> {
        ::windows_core::Param::Owned(::core::convert::Into::<Timeline>::into(self))
    }
}
impl ::core::convert::From<PointerUpThemeAnimation> for super::super::DependencyObject {
    fn from(value: PointerUpThemeAnimation) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&PointerUpThemeAnimation> for super::super::DependencyObject {
    fn from(value: &PointerUpThemeAnimation) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::DependencyObject> for PointerUpThemeAnimation {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::DependencyObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::DependencyObject> for &PointerUpThemeAnimation {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::DependencyObject> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for PointerUpThemeAnimation {}
unsafe impl ::core::marker::Sync for PointerUpThemeAnimation {}
#[repr(transparent)]
pub struct PopInThemeAnimation(::windows_core::IUnknown);
impl PopInThemeAnimation {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<PopInThemeAnimation, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn TargetName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).TargetName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetTargetName<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetTargetName)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn FromHorizontalOffset(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).FromHorizontalOffset)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn SetFromHorizontalOffset(&self, value: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetFromHorizontalOffset)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn FromVerticalOffset(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).FromVerticalOffset)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn SetFromVerticalOffset(&self, value: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetFromVerticalOffset)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn TargetNameProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::IPopInThemeAnimationStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TargetNameProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn FromHorizontalOffsetProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::IPopInThemeAnimationStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FromHorizontalOffsetProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn FromVerticalOffsetProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::IPopInThemeAnimationStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FromVerticalOffsetProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn IPopInThemeAnimationStatics<R, F: FnOnce(&IPopInThemeAnimationStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<PopInThemeAnimation, IPopInThemeAnimationStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for PopInThemeAnimation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PopInThemeAnimation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PopInThemeAnimation {}
impl ::core::fmt::Debug for PopInThemeAnimation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PopInThemeAnimation").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PopInThemeAnimation {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Animation.PopInThemeAnimation;{196938c1-1c07-4c28-8847-f9f055b32855})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for PopInThemeAnimation {
    type Vtable = IPopInThemeAnimation_Vtbl;
    const IID: ::windows_core::GUID = <IPopInThemeAnimation as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for PopInThemeAnimation {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.PopInThemeAnimation";
}
impl ::core::convert::From<PopInThemeAnimation> for ::windows_core::IUnknown {
    fn from(value: PopInThemeAnimation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PopInThemeAnimation> for ::windows_core::IUnknown {
    fn from(value: &PopInThemeAnimation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for PopInThemeAnimation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a PopInThemeAnimation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PopInThemeAnimation> for ::windows_core::IInspectable {
    fn from(value: PopInThemeAnimation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PopInThemeAnimation> for ::windows_core::IInspectable {
    fn from(value: &PopInThemeAnimation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for PopInThemeAnimation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a PopInThemeAnimation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PopInThemeAnimation> for Timeline {
    fn from(value: PopInThemeAnimation) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&PopInThemeAnimation> for Timeline {
    fn from(value: &PopInThemeAnimation) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, Timeline> for PopInThemeAnimation {
    fn into_param(self) -> ::windows_core::Param<'a, Timeline> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, Timeline> for &PopInThemeAnimation {
    fn into_param(self) -> ::windows_core::Param<'a, Timeline> {
        ::windows_core::Param::Owned(::core::convert::Into::<Timeline>::into(self))
    }
}
impl ::core::convert::From<PopInThemeAnimation> for super::super::DependencyObject {
    fn from(value: PopInThemeAnimation) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&PopInThemeAnimation> for super::super::DependencyObject {
    fn from(value: &PopInThemeAnimation) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::DependencyObject> for PopInThemeAnimation {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::DependencyObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::DependencyObject> for &PopInThemeAnimation {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::DependencyObject> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for PopInThemeAnimation {}
unsafe impl ::core::marker::Sync for PopInThemeAnimation {}
#[repr(transparent)]
pub struct PopOutThemeAnimation(::windows_core::IUnknown);
impl PopOutThemeAnimation {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<PopOutThemeAnimation, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn TargetName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).TargetName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetTargetName<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetTargetName)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn TargetNameProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::IPopOutThemeAnimationStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TargetNameProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn IPopOutThemeAnimationStatics<R, F: FnOnce(&IPopOutThemeAnimationStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<PopOutThemeAnimation, IPopOutThemeAnimationStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for PopOutThemeAnimation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PopOutThemeAnimation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PopOutThemeAnimation {}
impl ::core::fmt::Debug for PopOutThemeAnimation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PopOutThemeAnimation").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PopOutThemeAnimation {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Animation.PopOutThemeAnimation;{4786ab49-0e48-4e81-a2e5-cc5aa19e48d3})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for PopOutThemeAnimation {
    type Vtable = IPopOutThemeAnimation_Vtbl;
    const IID: ::windows_core::GUID = <IPopOutThemeAnimation as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for PopOutThemeAnimation {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.PopOutThemeAnimation";
}
impl ::core::convert::From<PopOutThemeAnimation> for ::windows_core::IUnknown {
    fn from(value: PopOutThemeAnimation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PopOutThemeAnimation> for ::windows_core::IUnknown {
    fn from(value: &PopOutThemeAnimation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for PopOutThemeAnimation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a PopOutThemeAnimation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PopOutThemeAnimation> for ::windows_core::IInspectable {
    fn from(value: PopOutThemeAnimation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PopOutThemeAnimation> for ::windows_core::IInspectable {
    fn from(value: &PopOutThemeAnimation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for PopOutThemeAnimation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a PopOutThemeAnimation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PopOutThemeAnimation> for Timeline {
    fn from(value: PopOutThemeAnimation) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&PopOutThemeAnimation> for Timeline {
    fn from(value: &PopOutThemeAnimation) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, Timeline> for PopOutThemeAnimation {
    fn into_param(self) -> ::windows_core::Param<'a, Timeline> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, Timeline> for &PopOutThemeAnimation {
    fn into_param(self) -> ::windows_core::Param<'a, Timeline> {
        ::windows_core::Param::Owned(::core::convert::Into::<Timeline>::into(self))
    }
}
impl ::core::convert::From<PopOutThemeAnimation> for super::super::DependencyObject {
    fn from(value: PopOutThemeAnimation) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&PopOutThemeAnimation> for super::super::DependencyObject {
    fn from(value: &PopOutThemeAnimation) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::DependencyObject> for PopOutThemeAnimation {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::DependencyObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::DependencyObject> for &PopOutThemeAnimation {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::DependencyObject> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for PopOutThemeAnimation {}
unsafe impl ::core::marker::Sync for PopOutThemeAnimation {}
#[repr(transparent)]
pub struct PopupThemeTransition(::windows_core::IUnknown);
impl PopupThemeTransition {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<PopupThemeTransition, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn FromHorizontalOffset(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).FromHorizontalOffset)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn SetFromHorizontalOffset(&self, value: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetFromHorizontalOffset)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn FromVerticalOffset(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).FromVerticalOffset)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn SetFromVerticalOffset(&self, value: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetFromVerticalOffset)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn FromHorizontalOffsetProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::IPopupThemeTransitionStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FromHorizontalOffsetProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn FromVerticalOffsetProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::IPopupThemeTransitionStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FromVerticalOffsetProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn IPopupThemeTransitionStatics<R, F: FnOnce(&IPopupThemeTransitionStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<PopupThemeTransition, IPopupThemeTransitionStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for PopupThemeTransition {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PopupThemeTransition {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PopupThemeTransition {}
impl ::core::fmt::Debug for PopupThemeTransition {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PopupThemeTransition").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PopupThemeTransition {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Animation.PopupThemeTransition;{47843552-4283-545e-c791-268dca22ce4b})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for PopupThemeTransition {
    type Vtable = IPopupThemeTransition_Vtbl;
    const IID: ::windows_core::GUID = <IPopupThemeTransition as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for PopupThemeTransition {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.PopupThemeTransition";
}
impl ::core::convert::From<PopupThemeTransition> for ::windows_core::IUnknown {
    fn from(value: PopupThemeTransition) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PopupThemeTransition> for ::windows_core::IUnknown {
    fn from(value: &PopupThemeTransition) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for PopupThemeTransition {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a PopupThemeTransition {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PopupThemeTransition> for ::windows_core::IInspectable {
    fn from(value: PopupThemeTransition) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PopupThemeTransition> for ::windows_core::IInspectable {
    fn from(value: &PopupThemeTransition) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for PopupThemeTransition {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a PopupThemeTransition {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PopupThemeTransition> for Transition {
    fn from(value: PopupThemeTransition) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&PopupThemeTransition> for Transition {
    fn from(value: &PopupThemeTransition) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, Transition> for PopupThemeTransition {
    fn into_param(self) -> ::windows_core::Param<'a, Transition> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, Transition> for &PopupThemeTransition {
    fn into_param(self) -> ::windows_core::Param<'a, Transition> {
        ::windows_core::Param::Owned(::core::convert::Into::<Transition>::into(self))
    }
}
impl ::core::convert::From<PopupThemeTransition> for super::super::DependencyObject {
    fn from(value: PopupThemeTransition) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&PopupThemeTransition> for super::super::DependencyObject {
    fn from(value: &PopupThemeTransition) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::DependencyObject> for PopupThemeTransition {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::DependencyObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::DependencyObject> for &PopupThemeTransition {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::DependencyObject> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for PopupThemeTransition {}
unsafe impl ::core::marker::Sync for PopupThemeTransition {}
#[repr(transparent)]
pub struct PowerEase(::windows_core::IUnknown);
impl PowerEase {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<PowerEase, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn Power(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).Power)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn SetPower(&self, value: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetPower)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn PowerProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::IPowerEaseStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).PowerProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn IPowerEaseStatics<R, F: FnOnce(&IPowerEaseStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<PowerEase, IPowerEaseStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for PowerEase {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PowerEase {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PowerEase {}
impl ::core::fmt::Debug for PowerEase {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PowerEase").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PowerEase {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Animation.PowerEase;{69c80579-eedf-405b-8680-d9606880c937})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for PowerEase {
    type Vtable = IPowerEase_Vtbl;
    const IID: ::windows_core::GUID = <IPowerEase as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for PowerEase {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.PowerEase";
}
impl ::core::convert::From<PowerEase> for ::windows_core::IUnknown {
    fn from(value: PowerEase) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PowerEase> for ::windows_core::IUnknown {
    fn from(value: &PowerEase) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for PowerEase {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a PowerEase {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PowerEase> for ::windows_core::IInspectable {
    fn from(value: PowerEase) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PowerEase> for ::windows_core::IInspectable {
    fn from(value: &PowerEase) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for PowerEase {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a PowerEase {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PowerEase> for EasingFunctionBase {
    fn from(value: PowerEase) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&PowerEase> for EasingFunctionBase {
    fn from(value: &PowerEase) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, EasingFunctionBase> for PowerEase {
    fn into_param(self) -> ::windows_core::Param<'a, EasingFunctionBase> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, EasingFunctionBase> for &PowerEase {
    fn into_param(self) -> ::windows_core::Param<'a, EasingFunctionBase> {
        ::windows_core::Param::Owned(::core::convert::Into::<EasingFunctionBase>::into(self))
    }
}
impl ::core::convert::From<PowerEase> for super::super::DependencyObject {
    fn from(value: PowerEase) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&PowerEase> for super::super::DependencyObject {
    fn from(value: &PowerEase) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::DependencyObject> for PowerEase {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::DependencyObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::DependencyObject> for &PowerEase {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::DependencyObject> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for PowerEase {}
unsafe impl ::core::marker::Sync for PowerEase {}
#[repr(transparent)]
pub struct QuadraticEase(::windows_core::IUnknown);
impl QuadraticEase {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<QuadraticEase, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for QuadraticEase {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for QuadraticEase {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for QuadraticEase {}
impl ::core::fmt::Debug for QuadraticEase {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("QuadraticEase").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for QuadraticEase {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Animation.QuadraticEase;{e1510e91-ef6d-44f0-803d-68d16de0ddfc})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for QuadraticEase {
    type Vtable = IQuadraticEase_Vtbl;
    const IID: ::windows_core::GUID = <IQuadraticEase as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for QuadraticEase {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.QuadraticEase";
}
impl ::core::convert::From<QuadraticEase> for ::windows_core::IUnknown {
    fn from(value: QuadraticEase) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&QuadraticEase> for ::windows_core::IUnknown {
    fn from(value: &QuadraticEase) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for QuadraticEase {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a QuadraticEase {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<QuadraticEase> for ::windows_core::IInspectable {
    fn from(value: QuadraticEase) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&QuadraticEase> for ::windows_core::IInspectable {
    fn from(value: &QuadraticEase) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for QuadraticEase {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a QuadraticEase {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<QuadraticEase> for EasingFunctionBase {
    fn from(value: QuadraticEase) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&QuadraticEase> for EasingFunctionBase {
    fn from(value: &QuadraticEase) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, EasingFunctionBase> for QuadraticEase {
    fn into_param(self) -> ::windows_core::Param<'a, EasingFunctionBase> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, EasingFunctionBase> for &QuadraticEase {
    fn into_param(self) -> ::windows_core::Param<'a, EasingFunctionBase> {
        ::windows_core::Param::Owned(::core::convert::Into::<EasingFunctionBase>::into(self))
    }
}
impl ::core::convert::From<QuadraticEase> for super::super::DependencyObject {
    fn from(value: QuadraticEase) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&QuadraticEase> for super::super::DependencyObject {
    fn from(value: &QuadraticEase) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::DependencyObject> for QuadraticEase {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::DependencyObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::DependencyObject> for &QuadraticEase {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::DependencyObject> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for QuadraticEase {}
unsafe impl ::core::marker::Sync for QuadraticEase {}
#[repr(transparent)]
pub struct QuarticEase(::windows_core::IUnknown);
impl QuarticEase {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<QuarticEase, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for QuarticEase {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for QuarticEase {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for QuarticEase {}
impl ::core::fmt::Debug for QuarticEase {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("QuarticEase").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for QuarticEase {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Animation.QuarticEase;{e8698814-fe42-4a05-b5b8-081f41157815})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for QuarticEase {
    type Vtable = IQuarticEase_Vtbl;
    const IID: ::windows_core::GUID = <IQuarticEase as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for QuarticEase {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.QuarticEase";
}
impl ::core::convert::From<QuarticEase> for ::windows_core::IUnknown {
    fn from(value: QuarticEase) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&QuarticEase> for ::windows_core::IUnknown {
    fn from(value: &QuarticEase) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for QuarticEase {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a QuarticEase {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<QuarticEase> for ::windows_core::IInspectable {
    fn from(value: QuarticEase) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&QuarticEase> for ::windows_core::IInspectable {
    fn from(value: &QuarticEase) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for QuarticEase {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a QuarticEase {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<QuarticEase> for EasingFunctionBase {
    fn from(value: QuarticEase) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&QuarticEase> for EasingFunctionBase {
    fn from(value: &QuarticEase) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, EasingFunctionBase> for QuarticEase {
    fn into_param(self) -> ::windows_core::Param<'a, EasingFunctionBase> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, EasingFunctionBase> for &QuarticEase {
    fn into_param(self) -> ::windows_core::Param<'a, EasingFunctionBase> {
        ::windows_core::Param::Owned(::core::convert::Into::<EasingFunctionBase>::into(self))
    }
}
impl ::core::convert::From<QuarticEase> for super::super::DependencyObject {
    fn from(value: QuarticEase) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&QuarticEase> for super::super::DependencyObject {
    fn from(value: &QuarticEase) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::DependencyObject> for QuarticEase {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::DependencyObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::DependencyObject> for &QuarticEase {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::DependencyObject> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for QuarticEase {}
unsafe impl ::core::marker::Sync for QuarticEase {}
#[repr(transparent)]
pub struct QuinticEase(::windows_core::IUnknown);
impl QuinticEase {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<QuinticEase, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for QuinticEase {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for QuinticEase {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for QuinticEase {}
impl ::core::fmt::Debug for QuinticEase {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("QuinticEase").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for QuinticEase {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Animation.QuinticEase;{92ee793b-3c49-4108-aa11-ab786603da21})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for QuinticEase {
    type Vtable = IQuinticEase_Vtbl;
    const IID: ::windows_core::GUID = <IQuinticEase as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for QuinticEase {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.QuinticEase";
}
impl ::core::convert::From<QuinticEase> for ::windows_core::IUnknown {
    fn from(value: QuinticEase) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&QuinticEase> for ::windows_core::IUnknown {
    fn from(value: &QuinticEase) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for QuinticEase {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a QuinticEase {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<QuinticEase> for ::windows_core::IInspectable {
    fn from(value: QuinticEase) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&QuinticEase> for ::windows_core::IInspectable {
    fn from(value: &QuinticEase) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for QuinticEase {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a QuinticEase {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<QuinticEase> for EasingFunctionBase {
    fn from(value: QuinticEase) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&QuinticEase> for EasingFunctionBase {
    fn from(value: &QuinticEase) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, EasingFunctionBase> for QuinticEase {
    fn into_param(self) -> ::windows_core::Param<'a, EasingFunctionBase> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, EasingFunctionBase> for &QuinticEase {
    fn into_param(self) -> ::windows_core::Param<'a, EasingFunctionBase> {
        ::windows_core::Param::Owned(::core::convert::Into::<EasingFunctionBase>::into(self))
    }
}
impl ::core::convert::From<QuinticEase> for super::super::DependencyObject {
    fn from(value: QuinticEase) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&QuinticEase> for super::super::DependencyObject {
    fn from(value: &QuinticEase) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::DependencyObject> for QuinticEase {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::DependencyObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::DependencyObject> for &QuinticEase {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::DependencyObject> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for QuinticEase {}
unsafe impl ::core::marker::Sync for QuinticEase {}
#[repr(transparent)]
pub struct ReorderThemeTransition(::windows_core::IUnknown);
impl ReorderThemeTransition {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<ReorderThemeTransition, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for ReorderThemeTransition {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ReorderThemeTransition {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ReorderThemeTransition {}
impl ::core::fmt::Debug for ReorderThemeTransition {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ReorderThemeTransition").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ReorderThemeTransition {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Animation.ReorderThemeTransition;{f2065c6c-d052-4ad1-8362-b71b36df7497})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for ReorderThemeTransition {
    type Vtable = IReorderThemeTransition_Vtbl;
    const IID: ::windows_core::GUID = <IReorderThemeTransition as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for ReorderThemeTransition {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.ReorderThemeTransition";
}
impl ::core::convert::From<ReorderThemeTransition> for ::windows_core::IUnknown {
    fn from(value: ReorderThemeTransition) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ReorderThemeTransition> for ::windows_core::IUnknown {
    fn from(value: &ReorderThemeTransition) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ReorderThemeTransition {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ReorderThemeTransition {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ReorderThemeTransition> for ::windows_core::IInspectable {
    fn from(value: ReorderThemeTransition) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ReorderThemeTransition> for ::windows_core::IInspectable {
    fn from(value: &ReorderThemeTransition) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for ReorderThemeTransition {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a ReorderThemeTransition {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ReorderThemeTransition> for Transition {
    fn from(value: ReorderThemeTransition) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&ReorderThemeTransition> for Transition {
    fn from(value: &ReorderThemeTransition) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, Transition> for ReorderThemeTransition {
    fn into_param(self) -> ::windows_core::Param<'a, Transition> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, Transition> for &ReorderThemeTransition {
    fn into_param(self) -> ::windows_core::Param<'a, Transition> {
        ::windows_core::Param::Owned(::core::convert::Into::<Transition>::into(self))
    }
}
impl ::core::convert::From<ReorderThemeTransition> for super::super::DependencyObject {
    fn from(value: ReorderThemeTransition) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&ReorderThemeTransition> for super::super::DependencyObject {
    fn from(value: &ReorderThemeTransition) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::DependencyObject> for ReorderThemeTransition {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::DependencyObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::DependencyObject> for &ReorderThemeTransition {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::DependencyObject> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for ReorderThemeTransition {}
unsafe impl ::core::marker::Sync for ReorderThemeTransition {}
#[repr(C)]
pub struct RepeatBehavior {
    pub Count: f64,
    pub Duration: ::winrt_foundation::TimeSpan,
    pub Type: RepeatBehaviorType,
}
impl ::core::marker::Copy for RepeatBehavior {}
impl ::core::clone::Clone for RepeatBehavior {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for RepeatBehavior {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RepeatBehavior").field("Count", &self.Count).field("Duration", &self.Duration).field("Type", &self.Type).finish()
    }
}
unsafe impl ::windows_core::Abi for RepeatBehavior {
    type Abi = Self;
}
unsafe impl ::windows_core::RuntimeType for RepeatBehavior {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"struct(Windows.UI.Xaml.Media.Animation.RepeatBehavior;f8;struct(Windows.Foundation.TimeSpan;i8);enum(Windows.UI.Xaml.Media.Animation.RepeatBehaviorType;i4))");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
impl ::core::cmp::PartialEq for RepeatBehavior {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RepeatBehavior>()) == 0 }
    }
}
impl ::core::cmp::Eq for RepeatBehavior {}
impl ::core::default::Default for RepeatBehavior {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
pub struct RepeatBehaviorHelper(::windows_core::IUnknown);
impl RepeatBehaviorHelper {
    pub fn Forever() -> ::windows_core::Result<RepeatBehavior> {
        Self::IRepeatBehaviorHelperStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<RepeatBehavior>::zeroed();
            (::windows_core::Interface::vtable(this).Forever)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<RepeatBehavior>(result__)
        })
    }
    pub fn FromCount(count: f64) -> ::windows_core::Result<RepeatBehavior> {
        Self::IRepeatBehaviorHelperStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<RepeatBehavior>::zeroed();
            (::windows_core::Interface::vtable(this).FromCount)(::windows_core::Interface::as_raw(this), count, result__.as_mut_ptr()).from_abi::<RepeatBehavior>(result__)
        })
    }
    pub fn FromDuration<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TimeSpan>>(duration: Param0) -> ::windows_core::Result<RepeatBehavior> {
        Self::IRepeatBehaviorHelperStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<RepeatBehavior>::zeroed();
            (::windows_core::Interface::vtable(this).FromDuration)(::windows_core::Interface::as_raw(this), duration.into_param().abi(), result__.as_mut_ptr()).from_abi::<RepeatBehavior>(result__)
        })
    }
    pub fn GetHasCount<'a, Param0: ::windows_core::IntoParam<'a, RepeatBehavior>>(target: Param0) -> ::windows_core::Result<bool> {
        Self::IRepeatBehaviorHelperStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).GetHasCount)(::windows_core::Interface::as_raw(this), target.into_param().abi(), result__.as_mut_ptr()).from_abi::<bool>(result__)
        })
    }
    pub fn GetHasDuration<'a, Param0: ::windows_core::IntoParam<'a, RepeatBehavior>>(target: Param0) -> ::windows_core::Result<bool> {
        Self::IRepeatBehaviorHelperStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).GetHasDuration)(::windows_core::Interface::as_raw(this), target.into_param().abi(), result__.as_mut_ptr()).from_abi::<bool>(result__)
        })
    }
    pub fn Equals<'a, Param0: ::windows_core::IntoParam<'a, RepeatBehavior>, Param1: ::windows_core::IntoParam<'a, RepeatBehavior>>(target: Param0, value: Param1) -> ::windows_core::Result<bool> {
        Self::IRepeatBehaviorHelperStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).Equals)(::windows_core::Interface::as_raw(this), target.into_param().abi(), value.into_param().abi(), result__.as_mut_ptr()).from_abi::<bool>(result__)
        })
    }
    pub fn IRepeatBehaviorHelperStatics<R, F: FnOnce(&IRepeatBehaviorHelperStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<RepeatBehaviorHelper, IRepeatBehaviorHelperStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for RepeatBehaviorHelper {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for RepeatBehaviorHelper {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for RepeatBehaviorHelper {}
impl ::core::fmt::Debug for RepeatBehaviorHelper {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RepeatBehaviorHelper").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for RepeatBehaviorHelper {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Animation.RepeatBehaviorHelper;{6863ab72-4997-47f9-87ad-37efb75993ea})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for RepeatBehaviorHelper {
    type Vtable = IRepeatBehaviorHelper_Vtbl;
    const IID: ::windows_core::GUID = <IRepeatBehaviorHelper as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for RepeatBehaviorHelper {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.RepeatBehaviorHelper";
}
impl ::core::convert::From<RepeatBehaviorHelper> for ::windows_core::IUnknown {
    fn from(value: RepeatBehaviorHelper) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RepeatBehaviorHelper> for ::windows_core::IUnknown {
    fn from(value: &RepeatBehaviorHelper) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for RepeatBehaviorHelper {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a RepeatBehaviorHelper {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<RepeatBehaviorHelper> for ::windows_core::IInspectable {
    fn from(value: RepeatBehaviorHelper) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RepeatBehaviorHelper> for ::windows_core::IInspectable {
    fn from(value: &RepeatBehaviorHelper) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for RepeatBehaviorHelper {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a RepeatBehaviorHelper {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for RepeatBehaviorHelper {}
unsafe impl ::core::marker::Sync for RepeatBehaviorHelper {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct RepeatBehaviorType(pub i32);
impl RepeatBehaviorType {
    pub const Count: Self = Self(0i32);
    pub const Duration: Self = Self(1i32);
    pub const Forever: Self = Self(2i32);
}
impl ::core::marker::Copy for RepeatBehaviorType {}
impl ::core::clone::Clone for RepeatBehaviorType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RepeatBehaviorType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for RepeatBehaviorType {
    type Abi = Self;
}
impl ::core::fmt::Debug for RepeatBehaviorType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RepeatBehaviorType").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for RepeatBehaviorType {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Media.Animation.RepeatBehaviorType;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct RepositionThemeAnimation(::windows_core::IUnknown);
impl RepositionThemeAnimation {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<RepositionThemeAnimation, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn TargetName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).TargetName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetTargetName<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetTargetName)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn FromHorizontalOffset(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).FromHorizontalOffset)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn SetFromHorizontalOffset(&self, value: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetFromHorizontalOffset)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn FromVerticalOffset(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).FromVerticalOffset)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn SetFromVerticalOffset(&self, value: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetFromVerticalOffset)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn TargetNameProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::IRepositionThemeAnimationStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TargetNameProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn FromHorizontalOffsetProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::IRepositionThemeAnimationStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FromHorizontalOffsetProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn FromVerticalOffsetProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::IRepositionThemeAnimationStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FromVerticalOffsetProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn IRepositionThemeAnimationStatics<R, F: FnOnce(&IRepositionThemeAnimationStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<RepositionThemeAnimation, IRepositionThemeAnimationStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for RepositionThemeAnimation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for RepositionThemeAnimation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for RepositionThemeAnimation {}
impl ::core::fmt::Debug for RepositionThemeAnimation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RepositionThemeAnimation").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for RepositionThemeAnimation {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Animation.RepositionThemeAnimation;{ecda24e8-8945-4949-a1bf-62109965a7e9})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for RepositionThemeAnimation {
    type Vtable = IRepositionThemeAnimation_Vtbl;
    const IID: ::windows_core::GUID = <IRepositionThemeAnimation as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for RepositionThemeAnimation {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.RepositionThemeAnimation";
}
impl ::core::convert::From<RepositionThemeAnimation> for ::windows_core::IUnknown {
    fn from(value: RepositionThemeAnimation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RepositionThemeAnimation> for ::windows_core::IUnknown {
    fn from(value: &RepositionThemeAnimation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for RepositionThemeAnimation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a RepositionThemeAnimation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<RepositionThemeAnimation> for ::windows_core::IInspectable {
    fn from(value: RepositionThemeAnimation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RepositionThemeAnimation> for ::windows_core::IInspectable {
    fn from(value: &RepositionThemeAnimation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for RepositionThemeAnimation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a RepositionThemeAnimation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<RepositionThemeAnimation> for Timeline {
    fn from(value: RepositionThemeAnimation) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&RepositionThemeAnimation> for Timeline {
    fn from(value: &RepositionThemeAnimation) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, Timeline> for RepositionThemeAnimation {
    fn into_param(self) -> ::windows_core::Param<'a, Timeline> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, Timeline> for &RepositionThemeAnimation {
    fn into_param(self) -> ::windows_core::Param<'a, Timeline> {
        ::windows_core::Param::Owned(::core::convert::Into::<Timeline>::into(self))
    }
}
impl ::core::convert::From<RepositionThemeAnimation> for super::super::DependencyObject {
    fn from(value: RepositionThemeAnimation) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&RepositionThemeAnimation> for super::super::DependencyObject {
    fn from(value: &RepositionThemeAnimation) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::DependencyObject> for RepositionThemeAnimation {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::DependencyObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::DependencyObject> for &RepositionThemeAnimation {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::DependencyObject> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for RepositionThemeAnimation {}
unsafe impl ::core::marker::Sync for RepositionThemeAnimation {}
#[repr(transparent)]
pub struct RepositionThemeTransition(::windows_core::IUnknown);
impl RepositionThemeTransition {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<RepositionThemeTransition, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IsStaggeringEnabled(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IRepositionThemeTransition2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsStaggeringEnabled)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetIsStaggeringEnabled(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IRepositionThemeTransition2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetIsStaggeringEnabled)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsStaggeringEnabledProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::IRepositionThemeTransitionStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).IsStaggeringEnabledProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn IRepositionThemeTransitionStatics2<R, F: FnOnce(&IRepositionThemeTransitionStatics2) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<RepositionThemeTransition, IRepositionThemeTransitionStatics2> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for RepositionThemeTransition {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for RepositionThemeTransition {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for RepositionThemeTransition {}
impl ::core::fmt::Debug for RepositionThemeTransition {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RepositionThemeTransition").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for RepositionThemeTransition {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Animation.RepositionThemeTransition;{88329b82-98f3-455a-ac53-2e7083b6e22c})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for RepositionThemeTransition {
    type Vtable = IRepositionThemeTransition_Vtbl;
    const IID: ::windows_core::GUID = <IRepositionThemeTransition as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for RepositionThemeTransition {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.RepositionThemeTransition";
}
impl ::core::convert::From<RepositionThemeTransition> for ::windows_core::IUnknown {
    fn from(value: RepositionThemeTransition) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RepositionThemeTransition> for ::windows_core::IUnknown {
    fn from(value: &RepositionThemeTransition) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for RepositionThemeTransition {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a RepositionThemeTransition {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<RepositionThemeTransition> for ::windows_core::IInspectable {
    fn from(value: RepositionThemeTransition) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RepositionThemeTransition> for ::windows_core::IInspectable {
    fn from(value: &RepositionThemeTransition) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for RepositionThemeTransition {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a RepositionThemeTransition {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<RepositionThemeTransition> for Transition {
    fn from(value: RepositionThemeTransition) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&RepositionThemeTransition> for Transition {
    fn from(value: &RepositionThemeTransition) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, Transition> for RepositionThemeTransition {
    fn into_param(self) -> ::windows_core::Param<'a, Transition> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, Transition> for &RepositionThemeTransition {
    fn into_param(self) -> ::windows_core::Param<'a, Transition> {
        ::windows_core::Param::Owned(::core::convert::Into::<Transition>::into(self))
    }
}
impl ::core::convert::From<RepositionThemeTransition> for super::super::DependencyObject {
    fn from(value: RepositionThemeTransition) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&RepositionThemeTransition> for super::super::DependencyObject {
    fn from(value: &RepositionThemeTransition) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::DependencyObject> for RepositionThemeTransition {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::DependencyObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::DependencyObject> for &RepositionThemeTransition {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::DependencyObject> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for RepositionThemeTransition {}
unsafe impl ::core::marker::Sync for RepositionThemeTransition {}
#[repr(transparent)]
pub struct SineEase(::windows_core::IUnknown);
impl SineEase {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<SineEase, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for SineEase {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SineEase {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SineEase {}
impl ::core::fmt::Debug for SineEase {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SineEase").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SineEase {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Animation.SineEase;{a9382962-230b-49da-9e0d-664987892343})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for SineEase {
    type Vtable = ISineEase_Vtbl;
    const IID: ::windows_core::GUID = <ISineEase as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for SineEase {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.SineEase";
}
impl ::core::convert::From<SineEase> for ::windows_core::IUnknown {
    fn from(value: SineEase) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SineEase> for ::windows_core::IUnknown {
    fn from(value: &SineEase) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for SineEase {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a SineEase {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SineEase> for ::windows_core::IInspectable {
    fn from(value: SineEase) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SineEase> for ::windows_core::IInspectable {
    fn from(value: &SineEase) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for SineEase {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a SineEase {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SineEase> for EasingFunctionBase {
    fn from(value: SineEase) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&SineEase> for EasingFunctionBase {
    fn from(value: &SineEase) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, EasingFunctionBase> for SineEase {
    fn into_param(self) -> ::windows_core::Param<'a, EasingFunctionBase> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, EasingFunctionBase> for &SineEase {
    fn into_param(self) -> ::windows_core::Param<'a, EasingFunctionBase> {
        ::windows_core::Param::Owned(::core::convert::Into::<EasingFunctionBase>::into(self))
    }
}
impl ::core::convert::From<SineEase> for super::super::DependencyObject {
    fn from(value: SineEase) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&SineEase> for super::super::DependencyObject {
    fn from(value: &SineEase) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::DependencyObject> for SineEase {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::DependencyObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::DependencyObject> for &SineEase {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::DependencyObject> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for SineEase {}
unsafe impl ::core::marker::Sync for SineEase {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct SlideNavigationTransitionEffect(pub i32);
impl SlideNavigationTransitionEffect {
    pub const FromBottom: Self = Self(0i32);
    pub const FromLeft: Self = Self(1i32);
    pub const FromRight: Self = Self(2i32);
}
impl ::core::marker::Copy for SlideNavigationTransitionEffect {}
impl ::core::clone::Clone for SlideNavigationTransitionEffect {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SlideNavigationTransitionEffect {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for SlideNavigationTransitionEffect {
    type Abi = Self;
}
impl ::core::fmt::Debug for SlideNavigationTransitionEffect {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SlideNavigationTransitionEffect").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SlideNavigationTransitionEffect {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Media.Animation.SlideNavigationTransitionEffect;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct SlideNavigationTransitionInfo(::windows_core::IUnknown);
impl SlideNavigationTransitionInfo {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<SlideNavigationTransitionInfo, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn Effect(&self) -> ::windows_core::Result<SlideNavigationTransitionEffect> {
        let this = &::windows_core::Interface::cast::<ISlideNavigationTransitionInfo2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<SlideNavigationTransitionEffect>::zeroed();
            (::windows_core::Interface::vtable(this).Effect)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SlideNavigationTransitionEffect>(result__)
        }
    }
    pub fn SetEffect(&self, value: SlideNavigationTransitionEffect) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<ISlideNavigationTransitionInfo2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetEffect)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn EffectProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::ISlideNavigationTransitionInfoStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).EffectProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn ISlideNavigationTransitionInfoStatics2<R, F: FnOnce(&ISlideNavigationTransitionInfoStatics2) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<SlideNavigationTransitionInfo, ISlideNavigationTransitionInfoStatics2> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for SlideNavigationTransitionInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SlideNavigationTransitionInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SlideNavigationTransitionInfo {}
impl ::core::fmt::Debug for SlideNavigationTransitionInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SlideNavigationTransitionInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SlideNavigationTransitionInfo {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Animation.SlideNavigationTransitionInfo;{d6ac9d77-2e03-405f-80ed-e62beef3668f})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for SlideNavigationTransitionInfo {
    type Vtable = ISlideNavigationTransitionInfo_Vtbl;
    const IID: ::windows_core::GUID = <ISlideNavigationTransitionInfo as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for SlideNavigationTransitionInfo {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.SlideNavigationTransitionInfo";
}
impl ::core::convert::From<SlideNavigationTransitionInfo> for ::windows_core::IUnknown {
    fn from(value: SlideNavigationTransitionInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SlideNavigationTransitionInfo> for ::windows_core::IUnknown {
    fn from(value: &SlideNavigationTransitionInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for SlideNavigationTransitionInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a SlideNavigationTransitionInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SlideNavigationTransitionInfo> for ::windows_core::IInspectable {
    fn from(value: SlideNavigationTransitionInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SlideNavigationTransitionInfo> for ::windows_core::IInspectable {
    fn from(value: &SlideNavigationTransitionInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for SlideNavigationTransitionInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a SlideNavigationTransitionInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SlideNavigationTransitionInfo> for NavigationTransitionInfo {
    fn from(value: SlideNavigationTransitionInfo) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&SlideNavigationTransitionInfo> for NavigationTransitionInfo {
    fn from(value: &SlideNavigationTransitionInfo) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, NavigationTransitionInfo> for SlideNavigationTransitionInfo {
    fn into_param(self) -> ::windows_core::Param<'a, NavigationTransitionInfo> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, NavigationTransitionInfo> for &SlideNavigationTransitionInfo {
    fn into_param(self) -> ::windows_core::Param<'a, NavigationTransitionInfo> {
        ::windows_core::Param::Owned(::core::convert::Into::<NavigationTransitionInfo>::into(self))
    }
}
impl ::core::convert::From<SlideNavigationTransitionInfo> for super::super::DependencyObject {
    fn from(value: SlideNavigationTransitionInfo) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&SlideNavigationTransitionInfo> for super::super::DependencyObject {
    fn from(value: &SlideNavigationTransitionInfo) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::DependencyObject> for SlideNavigationTransitionInfo {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::DependencyObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::DependencyObject> for &SlideNavigationTransitionInfo {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::DependencyObject> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for SlideNavigationTransitionInfo {}
unsafe impl ::core::marker::Sync for SlideNavigationTransitionInfo {}
#[repr(transparent)]
pub struct SplineColorKeyFrame(::windows_core::IUnknown);
impl SplineColorKeyFrame {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<SplineColorKeyFrame, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn KeySpline(&self) -> ::windows_core::Result<KeySpline> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).KeySpline)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<KeySpline>(result__)
        }
    }
    pub fn SetKeySpline<'a, Param0: ::windows_core::IntoParam<'a, KeySpline>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetKeySpline)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn KeySplineProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::ISplineColorKeyFrameStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).KeySplineProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn ISplineColorKeyFrameStatics<R, F: FnOnce(&ISplineColorKeyFrameStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<SplineColorKeyFrame, ISplineColorKeyFrameStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for SplineColorKeyFrame {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SplineColorKeyFrame {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SplineColorKeyFrame {}
impl ::core::fmt::Debug for SplineColorKeyFrame {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SplineColorKeyFrame").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SplineColorKeyFrame {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Animation.SplineColorKeyFrame;{1a4a5941-1fe0-473a-8efe-4316d8c86229})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for SplineColorKeyFrame {
    type Vtable = ISplineColorKeyFrame_Vtbl;
    const IID: ::windows_core::GUID = <ISplineColorKeyFrame as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for SplineColorKeyFrame {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.SplineColorKeyFrame";
}
impl ::core::convert::From<SplineColorKeyFrame> for ::windows_core::IUnknown {
    fn from(value: SplineColorKeyFrame) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SplineColorKeyFrame> for ::windows_core::IUnknown {
    fn from(value: &SplineColorKeyFrame) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for SplineColorKeyFrame {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a SplineColorKeyFrame {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SplineColorKeyFrame> for ::windows_core::IInspectable {
    fn from(value: SplineColorKeyFrame) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SplineColorKeyFrame> for ::windows_core::IInspectable {
    fn from(value: &SplineColorKeyFrame) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for SplineColorKeyFrame {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a SplineColorKeyFrame {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SplineColorKeyFrame> for ColorKeyFrame {
    fn from(value: SplineColorKeyFrame) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&SplineColorKeyFrame> for ColorKeyFrame {
    fn from(value: &SplineColorKeyFrame) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, ColorKeyFrame> for SplineColorKeyFrame {
    fn into_param(self) -> ::windows_core::Param<'a, ColorKeyFrame> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ColorKeyFrame> for &SplineColorKeyFrame {
    fn into_param(self) -> ::windows_core::Param<'a, ColorKeyFrame> {
        ::windows_core::Param::Owned(::core::convert::Into::<ColorKeyFrame>::into(self))
    }
}
impl ::core::convert::From<SplineColorKeyFrame> for super::super::DependencyObject {
    fn from(value: SplineColorKeyFrame) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&SplineColorKeyFrame> for super::super::DependencyObject {
    fn from(value: &SplineColorKeyFrame) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::DependencyObject> for SplineColorKeyFrame {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::DependencyObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::DependencyObject> for &SplineColorKeyFrame {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::DependencyObject> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for SplineColorKeyFrame {}
unsafe impl ::core::marker::Sync for SplineColorKeyFrame {}
#[repr(transparent)]
pub struct SplineDoubleKeyFrame(::windows_core::IUnknown);
impl SplineDoubleKeyFrame {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<SplineDoubleKeyFrame, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn KeySpline(&self) -> ::windows_core::Result<KeySpline> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).KeySpline)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<KeySpline>(result__)
        }
    }
    pub fn SetKeySpline<'a, Param0: ::windows_core::IntoParam<'a, KeySpline>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetKeySpline)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn KeySplineProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::ISplineDoubleKeyFrameStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).KeySplineProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn ISplineDoubleKeyFrameStatics<R, F: FnOnce(&ISplineDoubleKeyFrameStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<SplineDoubleKeyFrame, ISplineDoubleKeyFrameStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for SplineDoubleKeyFrame {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SplineDoubleKeyFrame {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SplineDoubleKeyFrame {}
impl ::core::fmt::Debug for SplineDoubleKeyFrame {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SplineDoubleKeyFrame").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SplineDoubleKeyFrame {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Animation.SplineDoubleKeyFrame;{00d72d38-6b2b-4843-838e-c8b115eec801})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for SplineDoubleKeyFrame {
    type Vtable = ISplineDoubleKeyFrame_Vtbl;
    const IID: ::windows_core::GUID = <ISplineDoubleKeyFrame as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for SplineDoubleKeyFrame {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.SplineDoubleKeyFrame";
}
impl ::core::convert::From<SplineDoubleKeyFrame> for ::windows_core::IUnknown {
    fn from(value: SplineDoubleKeyFrame) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SplineDoubleKeyFrame> for ::windows_core::IUnknown {
    fn from(value: &SplineDoubleKeyFrame) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for SplineDoubleKeyFrame {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a SplineDoubleKeyFrame {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SplineDoubleKeyFrame> for ::windows_core::IInspectable {
    fn from(value: SplineDoubleKeyFrame) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SplineDoubleKeyFrame> for ::windows_core::IInspectable {
    fn from(value: &SplineDoubleKeyFrame) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for SplineDoubleKeyFrame {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a SplineDoubleKeyFrame {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SplineDoubleKeyFrame> for DoubleKeyFrame {
    fn from(value: SplineDoubleKeyFrame) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&SplineDoubleKeyFrame> for DoubleKeyFrame {
    fn from(value: &SplineDoubleKeyFrame) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, DoubleKeyFrame> for SplineDoubleKeyFrame {
    fn into_param(self) -> ::windows_core::Param<'a, DoubleKeyFrame> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, DoubleKeyFrame> for &SplineDoubleKeyFrame {
    fn into_param(self) -> ::windows_core::Param<'a, DoubleKeyFrame> {
        ::windows_core::Param::Owned(::core::convert::Into::<DoubleKeyFrame>::into(self))
    }
}
impl ::core::convert::From<SplineDoubleKeyFrame> for super::super::DependencyObject {
    fn from(value: SplineDoubleKeyFrame) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&SplineDoubleKeyFrame> for super::super::DependencyObject {
    fn from(value: &SplineDoubleKeyFrame) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::DependencyObject> for SplineDoubleKeyFrame {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::DependencyObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::DependencyObject> for &SplineDoubleKeyFrame {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::DependencyObject> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for SplineDoubleKeyFrame {}
unsafe impl ::core::marker::Sync for SplineDoubleKeyFrame {}
#[repr(transparent)]
pub struct SplinePointKeyFrame(::windows_core::IUnknown);
impl SplinePointKeyFrame {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<SplinePointKeyFrame, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn KeySpline(&self) -> ::windows_core::Result<KeySpline> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).KeySpline)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<KeySpline>(result__)
        }
    }
    pub fn SetKeySpline<'a, Param0: ::windows_core::IntoParam<'a, KeySpline>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetKeySpline)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn KeySplineProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::ISplinePointKeyFrameStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).KeySplineProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn ISplinePointKeyFrameStatics<R, F: FnOnce(&ISplinePointKeyFrameStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<SplinePointKeyFrame, ISplinePointKeyFrameStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for SplinePointKeyFrame {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SplinePointKeyFrame {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SplinePointKeyFrame {}
impl ::core::fmt::Debug for SplinePointKeyFrame {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SplinePointKeyFrame").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SplinePointKeyFrame {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Animation.SplinePointKeyFrame;{0f19f306-7036-494f-bc3c-780df0cc524a})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for SplinePointKeyFrame {
    type Vtable = ISplinePointKeyFrame_Vtbl;
    const IID: ::windows_core::GUID = <ISplinePointKeyFrame as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for SplinePointKeyFrame {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.SplinePointKeyFrame";
}
impl ::core::convert::From<SplinePointKeyFrame> for ::windows_core::IUnknown {
    fn from(value: SplinePointKeyFrame) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SplinePointKeyFrame> for ::windows_core::IUnknown {
    fn from(value: &SplinePointKeyFrame) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for SplinePointKeyFrame {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a SplinePointKeyFrame {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SplinePointKeyFrame> for ::windows_core::IInspectable {
    fn from(value: SplinePointKeyFrame) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SplinePointKeyFrame> for ::windows_core::IInspectable {
    fn from(value: &SplinePointKeyFrame) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for SplinePointKeyFrame {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a SplinePointKeyFrame {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SplinePointKeyFrame> for PointKeyFrame {
    fn from(value: SplinePointKeyFrame) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&SplinePointKeyFrame> for PointKeyFrame {
    fn from(value: &SplinePointKeyFrame) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, PointKeyFrame> for SplinePointKeyFrame {
    fn into_param(self) -> ::windows_core::Param<'a, PointKeyFrame> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, PointKeyFrame> for &SplinePointKeyFrame {
    fn into_param(self) -> ::windows_core::Param<'a, PointKeyFrame> {
        ::windows_core::Param::Owned(::core::convert::Into::<PointKeyFrame>::into(self))
    }
}
impl ::core::convert::From<SplinePointKeyFrame> for super::super::DependencyObject {
    fn from(value: SplinePointKeyFrame) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&SplinePointKeyFrame> for super::super::DependencyObject {
    fn from(value: &SplinePointKeyFrame) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::DependencyObject> for SplinePointKeyFrame {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::DependencyObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::DependencyObject> for &SplinePointKeyFrame {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::DependencyObject> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for SplinePointKeyFrame {}
unsafe impl ::core::marker::Sync for SplinePointKeyFrame {}
#[repr(transparent)]
pub struct SplitCloseThemeAnimation(::windows_core::IUnknown);
impl SplitCloseThemeAnimation {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<SplitCloseThemeAnimation, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn OpenedTargetName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).OpenedTargetName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetOpenedTargetName<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetOpenedTargetName)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn OpenedTarget(&self) -> ::windows_core::Result<super::super::DependencyObject> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).OpenedTarget)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyObject>(result__)
        }
    }
    pub fn SetOpenedTarget<'a, Param0: ::windows_core::IntoParam<'a, super::super::DependencyObject>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetOpenedTarget)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn ClosedTargetName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).ClosedTargetName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetClosedTargetName<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetClosedTargetName)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn ClosedTarget(&self) -> ::windows_core::Result<super::super::DependencyObject> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ClosedTarget)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyObject>(result__)
        }
    }
    pub fn SetClosedTarget<'a, Param0: ::windows_core::IntoParam<'a, super::super::DependencyObject>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetClosedTarget)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn ContentTargetName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).ContentTargetName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetContentTargetName<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetContentTargetName)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn ContentTarget(&self) -> ::windows_core::Result<super::super::DependencyObject> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ContentTarget)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyObject>(result__)
        }
    }
    pub fn SetContentTarget<'a, Param0: ::windows_core::IntoParam<'a, super::super::DependencyObject>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetContentTarget)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn OpenedLength(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).OpenedLength)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn SetOpenedLength(&self, value: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetOpenedLength)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ClosedLength(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).ClosedLength)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn SetClosedLength(&self, value: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetClosedLength)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn OffsetFromCenter(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).OffsetFromCenter)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn SetOffsetFromCenter(&self, value: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetOffsetFromCenter)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "winrt-ui")]
    pub fn ContentTranslationDirection(&self) -> ::windows_core::Result<super::super::Controls::Primitives::AnimationDirection> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::Controls::Primitives::AnimationDirection>::zeroed();
            (::windows_core::Interface::vtable(this).ContentTranslationDirection)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Controls::Primitives::AnimationDirection>(result__)
        }
    }
    #[cfg(feature = "winrt-ui")]
    pub fn SetContentTranslationDirection(&self, value: super::super::Controls::Primitives::AnimationDirection) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetContentTranslationDirection)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ContentTranslationOffset(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).ContentTranslationOffset)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn SetContentTranslationOffset(&self, value: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetContentTranslationOffset)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn OpenedTargetNameProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::ISplitCloseThemeAnimationStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).OpenedTargetNameProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn OpenedTargetProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::ISplitCloseThemeAnimationStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).OpenedTargetProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn ClosedTargetNameProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::ISplitCloseThemeAnimationStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ClosedTargetNameProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn ClosedTargetProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::ISplitCloseThemeAnimationStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ClosedTargetProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn ContentTargetNameProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::ISplitCloseThemeAnimationStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ContentTargetNameProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn ContentTargetProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::ISplitCloseThemeAnimationStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ContentTargetProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn OpenedLengthProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::ISplitCloseThemeAnimationStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).OpenedLengthProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn ClosedLengthProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::ISplitCloseThemeAnimationStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ClosedLengthProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn OffsetFromCenterProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::ISplitCloseThemeAnimationStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).OffsetFromCenterProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn ContentTranslationDirectionProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::ISplitCloseThemeAnimationStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ContentTranslationDirectionProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn ContentTranslationOffsetProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::ISplitCloseThemeAnimationStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ContentTranslationOffsetProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn ISplitCloseThemeAnimationStatics<R, F: FnOnce(&ISplitCloseThemeAnimationStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<SplitCloseThemeAnimation, ISplitCloseThemeAnimationStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for SplitCloseThemeAnimation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SplitCloseThemeAnimation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SplitCloseThemeAnimation {}
impl ::core::fmt::Debug for SplitCloseThemeAnimation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SplitCloseThemeAnimation").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SplitCloseThemeAnimation {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Animation.SplitCloseThemeAnimation;{4f799518-ff39-4e90-bb74-2abd56027402})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for SplitCloseThemeAnimation {
    type Vtable = ISplitCloseThemeAnimation_Vtbl;
    const IID: ::windows_core::GUID = <ISplitCloseThemeAnimation as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for SplitCloseThemeAnimation {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.SplitCloseThemeAnimation";
}
impl ::core::convert::From<SplitCloseThemeAnimation> for ::windows_core::IUnknown {
    fn from(value: SplitCloseThemeAnimation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SplitCloseThemeAnimation> for ::windows_core::IUnknown {
    fn from(value: &SplitCloseThemeAnimation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for SplitCloseThemeAnimation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a SplitCloseThemeAnimation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SplitCloseThemeAnimation> for ::windows_core::IInspectable {
    fn from(value: SplitCloseThemeAnimation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SplitCloseThemeAnimation> for ::windows_core::IInspectable {
    fn from(value: &SplitCloseThemeAnimation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for SplitCloseThemeAnimation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a SplitCloseThemeAnimation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SplitCloseThemeAnimation> for Timeline {
    fn from(value: SplitCloseThemeAnimation) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&SplitCloseThemeAnimation> for Timeline {
    fn from(value: &SplitCloseThemeAnimation) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, Timeline> for SplitCloseThemeAnimation {
    fn into_param(self) -> ::windows_core::Param<'a, Timeline> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, Timeline> for &SplitCloseThemeAnimation {
    fn into_param(self) -> ::windows_core::Param<'a, Timeline> {
        ::windows_core::Param::Owned(::core::convert::Into::<Timeline>::into(self))
    }
}
impl ::core::convert::From<SplitCloseThemeAnimation> for super::super::DependencyObject {
    fn from(value: SplitCloseThemeAnimation) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&SplitCloseThemeAnimation> for super::super::DependencyObject {
    fn from(value: &SplitCloseThemeAnimation) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::DependencyObject> for SplitCloseThemeAnimation {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::DependencyObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::DependencyObject> for &SplitCloseThemeAnimation {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::DependencyObject> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for SplitCloseThemeAnimation {}
unsafe impl ::core::marker::Sync for SplitCloseThemeAnimation {}
#[repr(transparent)]
pub struct SplitOpenThemeAnimation(::windows_core::IUnknown);
impl SplitOpenThemeAnimation {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<SplitOpenThemeAnimation, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn OpenedTargetName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).OpenedTargetName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetOpenedTargetName<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetOpenedTargetName)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn OpenedTarget(&self) -> ::windows_core::Result<super::super::DependencyObject> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).OpenedTarget)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyObject>(result__)
        }
    }
    pub fn SetOpenedTarget<'a, Param0: ::windows_core::IntoParam<'a, super::super::DependencyObject>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetOpenedTarget)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn ClosedTargetName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).ClosedTargetName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetClosedTargetName<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetClosedTargetName)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn ClosedTarget(&self) -> ::windows_core::Result<super::super::DependencyObject> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ClosedTarget)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyObject>(result__)
        }
    }
    pub fn SetClosedTarget<'a, Param0: ::windows_core::IntoParam<'a, super::super::DependencyObject>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetClosedTarget)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn ContentTargetName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).ContentTargetName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetContentTargetName<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetContentTargetName)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn ContentTarget(&self) -> ::windows_core::Result<super::super::DependencyObject> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ContentTarget)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyObject>(result__)
        }
    }
    pub fn SetContentTarget<'a, Param0: ::windows_core::IntoParam<'a, super::super::DependencyObject>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetContentTarget)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn OpenedLength(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).OpenedLength)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn SetOpenedLength(&self, value: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetOpenedLength)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ClosedLength(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).ClosedLength)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn SetClosedLength(&self, value: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetClosedLength)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn OffsetFromCenter(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).OffsetFromCenter)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn SetOffsetFromCenter(&self, value: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetOffsetFromCenter)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "winrt-ui")]
    pub fn ContentTranslationDirection(&self) -> ::windows_core::Result<super::super::Controls::Primitives::AnimationDirection> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::Controls::Primitives::AnimationDirection>::zeroed();
            (::windows_core::Interface::vtable(this).ContentTranslationDirection)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Controls::Primitives::AnimationDirection>(result__)
        }
    }
    #[cfg(feature = "winrt-ui")]
    pub fn SetContentTranslationDirection(&self, value: super::super::Controls::Primitives::AnimationDirection) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetContentTranslationDirection)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ContentTranslationOffset(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).ContentTranslationOffset)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn SetContentTranslationOffset(&self, value: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetContentTranslationOffset)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn OpenedTargetNameProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::ISplitOpenThemeAnimationStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).OpenedTargetNameProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn OpenedTargetProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::ISplitOpenThemeAnimationStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).OpenedTargetProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn ClosedTargetNameProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::ISplitOpenThemeAnimationStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ClosedTargetNameProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn ClosedTargetProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::ISplitOpenThemeAnimationStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ClosedTargetProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn ContentTargetNameProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::ISplitOpenThemeAnimationStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ContentTargetNameProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn ContentTargetProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::ISplitOpenThemeAnimationStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ContentTargetProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn OpenedLengthProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::ISplitOpenThemeAnimationStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).OpenedLengthProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn ClosedLengthProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::ISplitOpenThemeAnimationStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ClosedLengthProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn OffsetFromCenterProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::ISplitOpenThemeAnimationStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).OffsetFromCenterProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn ContentTranslationDirectionProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::ISplitOpenThemeAnimationStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ContentTranslationDirectionProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn ContentTranslationOffsetProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::ISplitOpenThemeAnimationStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ContentTranslationOffsetProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn ISplitOpenThemeAnimationStatics<R, F: FnOnce(&ISplitOpenThemeAnimationStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<SplitOpenThemeAnimation, ISplitOpenThemeAnimationStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for SplitOpenThemeAnimation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SplitOpenThemeAnimation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SplitOpenThemeAnimation {}
impl ::core::fmt::Debug for SplitOpenThemeAnimation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SplitOpenThemeAnimation").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SplitOpenThemeAnimation {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Animation.SplitOpenThemeAnimation;{785fd7aa-5456-4639-8fd2-26bae6a5ffe4})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for SplitOpenThemeAnimation {
    type Vtable = ISplitOpenThemeAnimation_Vtbl;
    const IID: ::windows_core::GUID = <ISplitOpenThemeAnimation as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for SplitOpenThemeAnimation {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.SplitOpenThemeAnimation";
}
impl ::core::convert::From<SplitOpenThemeAnimation> for ::windows_core::IUnknown {
    fn from(value: SplitOpenThemeAnimation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SplitOpenThemeAnimation> for ::windows_core::IUnknown {
    fn from(value: &SplitOpenThemeAnimation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for SplitOpenThemeAnimation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a SplitOpenThemeAnimation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SplitOpenThemeAnimation> for ::windows_core::IInspectable {
    fn from(value: SplitOpenThemeAnimation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SplitOpenThemeAnimation> for ::windows_core::IInspectable {
    fn from(value: &SplitOpenThemeAnimation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for SplitOpenThemeAnimation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a SplitOpenThemeAnimation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SplitOpenThemeAnimation> for Timeline {
    fn from(value: SplitOpenThemeAnimation) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&SplitOpenThemeAnimation> for Timeline {
    fn from(value: &SplitOpenThemeAnimation) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, Timeline> for SplitOpenThemeAnimation {
    fn into_param(self) -> ::windows_core::Param<'a, Timeline> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, Timeline> for &SplitOpenThemeAnimation {
    fn into_param(self) -> ::windows_core::Param<'a, Timeline> {
        ::windows_core::Param::Owned(::core::convert::Into::<Timeline>::into(self))
    }
}
impl ::core::convert::From<SplitOpenThemeAnimation> for super::super::DependencyObject {
    fn from(value: SplitOpenThemeAnimation) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&SplitOpenThemeAnimation> for super::super::DependencyObject {
    fn from(value: &SplitOpenThemeAnimation) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::DependencyObject> for SplitOpenThemeAnimation {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::DependencyObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::DependencyObject> for &SplitOpenThemeAnimation {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::DependencyObject> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for SplitOpenThemeAnimation {}
unsafe impl ::core::marker::Sync for SplitOpenThemeAnimation {}
#[repr(transparent)]
pub struct Storyboard(::windows_core::IUnknown);
impl Storyboard {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<Storyboard, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn Children(&self) -> ::windows_core::Result<TimelineCollection> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Children)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<TimelineCollection>(result__)
        }
    }
    pub fn Seek<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TimeSpan>>(&self, offset: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Seek)(::windows_core::Interface::as_raw(this), offset.into_param().abi()).ok() }
    }
    pub fn Stop(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Stop)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Begin(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Begin)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Pause(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Pause)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Resume(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Resume)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn GetCurrentState(&self) -> ::windows_core::Result<ClockState> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ClockState>::zeroed();
            (::windows_core::Interface::vtable(this).GetCurrentState)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ClockState>(result__)
        }
    }
    pub fn GetCurrentTime(&self) -> ::windows_core::Result<::winrt_foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::TimeSpan>::zeroed();
            (::windows_core::Interface::vtable(this).GetCurrentTime)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::TimeSpan>(result__)
        }
    }
    pub fn SeekAlignedToLastTick<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TimeSpan>>(&self, offset: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SeekAlignedToLastTick)(::windows_core::Interface::as_raw(this), offset.into_param().abi()).ok() }
    }
    pub fn SkipToFill(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SkipToFill)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn TargetPropertyProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::IStoryboardStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TargetPropertyProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn GetTargetProperty<'a, Param0: ::windows_core::IntoParam<'a, Timeline>>(element: Param0) -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IStoryboardStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).GetTargetProperty)(::windows_core::Interface::as_raw(this), element.into_param().abi(), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn SetTargetProperty<'a, Param0: ::windows_core::IntoParam<'a, Timeline>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(element: Param0, path: Param1) -> ::windows_core::Result<()> {
        Self::IStoryboardStatics(|this| unsafe { (::windows_core::Interface::vtable(this).SetTargetProperty)(::windows_core::Interface::as_raw(this), element.into_param().abi(), path.into_param().abi()).ok() })
    }
    pub fn TargetNameProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::IStoryboardStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TargetNameProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn GetTargetName<'a, Param0: ::windows_core::IntoParam<'a, Timeline>>(element: Param0) -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IStoryboardStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).GetTargetName)(::windows_core::Interface::as_raw(this), element.into_param().abi(), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn SetTargetName<'a, Param0: ::windows_core::IntoParam<'a, Timeline>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(element: Param0, name: Param1) -> ::windows_core::Result<()> {
        Self::IStoryboardStatics(|this| unsafe { (::windows_core::Interface::vtable(this).SetTargetName)(::windows_core::Interface::as_raw(this), element.into_param().abi(), name.into_param().abi()).ok() })
    }
    pub fn SetTarget<'a, Param0: ::windows_core::IntoParam<'a, Timeline>, Param1: ::windows_core::IntoParam<'a, super::super::DependencyObject>>(timeline: Param0, target: Param1) -> ::windows_core::Result<()> {
        Self::IStoryboardStatics(|this| unsafe { (::windows_core::Interface::vtable(this).SetTarget)(::windows_core::Interface::as_raw(this), timeline.into_param().abi(), target.into_param().abi()).ok() })
    }
    pub fn IStoryboardStatics<R, F: FnOnce(&IStoryboardStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<Storyboard, IStoryboardStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for Storyboard {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for Storyboard {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for Storyboard {}
impl ::core::fmt::Debug for Storyboard {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Storyboard").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for Storyboard {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Animation.Storyboard;{d45c1e6e-3594-460e-981a-32271bd3aa06})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for Storyboard {
    type Vtable = IStoryboard_Vtbl;
    const IID: ::windows_core::GUID = <IStoryboard as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for Storyboard {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.Storyboard";
}
impl ::core::convert::From<Storyboard> for ::windows_core::IUnknown {
    fn from(value: Storyboard) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&Storyboard> for ::windows_core::IUnknown {
    fn from(value: &Storyboard) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for Storyboard {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a Storyboard {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<Storyboard> for ::windows_core::IInspectable {
    fn from(value: Storyboard) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&Storyboard> for ::windows_core::IInspectable {
    fn from(value: &Storyboard) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for Storyboard {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a Storyboard {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<Storyboard> for Timeline {
    fn from(value: Storyboard) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&Storyboard> for Timeline {
    fn from(value: &Storyboard) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, Timeline> for Storyboard {
    fn into_param(self) -> ::windows_core::Param<'a, Timeline> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, Timeline> for &Storyboard {
    fn into_param(self) -> ::windows_core::Param<'a, Timeline> {
        ::windows_core::Param::Owned(::core::convert::Into::<Timeline>::into(self))
    }
}
impl ::core::convert::From<Storyboard> for super::super::DependencyObject {
    fn from(value: Storyboard) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&Storyboard> for super::super::DependencyObject {
    fn from(value: &Storyboard) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::DependencyObject> for Storyboard {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::DependencyObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::DependencyObject> for &Storyboard {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::DependencyObject> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for Storyboard {}
unsafe impl ::core::marker::Sync for Storyboard {}
#[repr(transparent)]
pub struct SuppressNavigationTransitionInfo(::windows_core::IUnknown);
impl SuppressNavigationTransitionInfo {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<SuppressNavigationTransitionInfo, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for SuppressNavigationTransitionInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SuppressNavigationTransitionInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SuppressNavigationTransitionInfo {}
impl ::core::fmt::Debug for SuppressNavigationTransitionInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SuppressNavigationTransitionInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SuppressNavigationTransitionInfo {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Animation.SuppressNavigationTransitionInfo;{244d7b0c-b1b7-4871-9d3e-d56203a3a5b4})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for SuppressNavigationTransitionInfo {
    type Vtable = ISuppressNavigationTransitionInfo_Vtbl;
    const IID: ::windows_core::GUID = <ISuppressNavigationTransitionInfo as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for SuppressNavigationTransitionInfo {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.SuppressNavigationTransitionInfo";
}
impl ::core::convert::From<SuppressNavigationTransitionInfo> for ::windows_core::IUnknown {
    fn from(value: SuppressNavigationTransitionInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SuppressNavigationTransitionInfo> for ::windows_core::IUnknown {
    fn from(value: &SuppressNavigationTransitionInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for SuppressNavigationTransitionInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a SuppressNavigationTransitionInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SuppressNavigationTransitionInfo> for ::windows_core::IInspectable {
    fn from(value: SuppressNavigationTransitionInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SuppressNavigationTransitionInfo> for ::windows_core::IInspectable {
    fn from(value: &SuppressNavigationTransitionInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for SuppressNavigationTransitionInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a SuppressNavigationTransitionInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SuppressNavigationTransitionInfo> for NavigationTransitionInfo {
    fn from(value: SuppressNavigationTransitionInfo) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&SuppressNavigationTransitionInfo> for NavigationTransitionInfo {
    fn from(value: &SuppressNavigationTransitionInfo) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, NavigationTransitionInfo> for SuppressNavigationTransitionInfo {
    fn into_param(self) -> ::windows_core::Param<'a, NavigationTransitionInfo> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, NavigationTransitionInfo> for &SuppressNavigationTransitionInfo {
    fn into_param(self) -> ::windows_core::Param<'a, NavigationTransitionInfo> {
        ::windows_core::Param::Owned(::core::convert::Into::<NavigationTransitionInfo>::into(self))
    }
}
impl ::core::convert::From<SuppressNavigationTransitionInfo> for super::super::DependencyObject {
    fn from(value: SuppressNavigationTransitionInfo) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&SuppressNavigationTransitionInfo> for super::super::DependencyObject {
    fn from(value: &SuppressNavigationTransitionInfo) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::DependencyObject> for SuppressNavigationTransitionInfo {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::DependencyObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::DependencyObject> for &SuppressNavigationTransitionInfo {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::DependencyObject> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for SuppressNavigationTransitionInfo {}
unsafe impl ::core::marker::Sync for SuppressNavigationTransitionInfo {}
#[repr(transparent)]
pub struct SwipeBackThemeAnimation(::windows_core::IUnknown);
impl SwipeBackThemeAnimation {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<SwipeBackThemeAnimation, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn TargetName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).TargetName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetTargetName<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetTargetName)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn FromHorizontalOffset(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).FromHorizontalOffset)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn SetFromHorizontalOffset(&self, value: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetFromHorizontalOffset)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn FromVerticalOffset(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).FromVerticalOffset)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn SetFromVerticalOffset(&self, value: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetFromVerticalOffset)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn TargetNameProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::ISwipeBackThemeAnimationStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TargetNameProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn FromHorizontalOffsetProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::ISwipeBackThemeAnimationStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FromHorizontalOffsetProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn FromVerticalOffsetProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::ISwipeBackThemeAnimationStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FromVerticalOffsetProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn ISwipeBackThemeAnimationStatics<R, F: FnOnce(&ISwipeBackThemeAnimationStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<SwipeBackThemeAnimation, ISwipeBackThemeAnimationStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for SwipeBackThemeAnimation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SwipeBackThemeAnimation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SwipeBackThemeAnimation {}
impl ::core::fmt::Debug for SwipeBackThemeAnimation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SwipeBackThemeAnimation").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SwipeBackThemeAnimation {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Animation.SwipeBackThemeAnimation;{a38a4214-0bca-4d2d-95f7-ceba57fbaf60})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for SwipeBackThemeAnimation {
    type Vtable = ISwipeBackThemeAnimation_Vtbl;
    const IID: ::windows_core::GUID = <ISwipeBackThemeAnimation as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for SwipeBackThemeAnimation {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.SwipeBackThemeAnimation";
}
impl ::core::convert::From<SwipeBackThemeAnimation> for ::windows_core::IUnknown {
    fn from(value: SwipeBackThemeAnimation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SwipeBackThemeAnimation> for ::windows_core::IUnknown {
    fn from(value: &SwipeBackThemeAnimation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for SwipeBackThemeAnimation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a SwipeBackThemeAnimation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SwipeBackThemeAnimation> for ::windows_core::IInspectable {
    fn from(value: SwipeBackThemeAnimation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SwipeBackThemeAnimation> for ::windows_core::IInspectable {
    fn from(value: &SwipeBackThemeAnimation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for SwipeBackThemeAnimation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a SwipeBackThemeAnimation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SwipeBackThemeAnimation> for Timeline {
    fn from(value: SwipeBackThemeAnimation) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&SwipeBackThemeAnimation> for Timeline {
    fn from(value: &SwipeBackThemeAnimation) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, Timeline> for SwipeBackThemeAnimation {
    fn into_param(self) -> ::windows_core::Param<'a, Timeline> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, Timeline> for &SwipeBackThemeAnimation {
    fn into_param(self) -> ::windows_core::Param<'a, Timeline> {
        ::windows_core::Param::Owned(::core::convert::Into::<Timeline>::into(self))
    }
}
impl ::core::convert::From<SwipeBackThemeAnimation> for super::super::DependencyObject {
    fn from(value: SwipeBackThemeAnimation) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&SwipeBackThemeAnimation> for super::super::DependencyObject {
    fn from(value: &SwipeBackThemeAnimation) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::DependencyObject> for SwipeBackThemeAnimation {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::DependencyObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::DependencyObject> for &SwipeBackThemeAnimation {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::DependencyObject> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for SwipeBackThemeAnimation {}
unsafe impl ::core::marker::Sync for SwipeBackThemeAnimation {}
#[repr(transparent)]
pub struct SwipeHintThemeAnimation(::windows_core::IUnknown);
impl SwipeHintThemeAnimation {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<SwipeHintThemeAnimation, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn TargetName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).TargetName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetTargetName<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetTargetName)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn ToHorizontalOffset(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).ToHorizontalOffset)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn SetToHorizontalOffset(&self, value: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetToHorizontalOffset)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ToVerticalOffset(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).ToVerticalOffset)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn SetToVerticalOffset(&self, value: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetToVerticalOffset)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn TargetNameProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::ISwipeHintThemeAnimationStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TargetNameProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn ToHorizontalOffsetProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::ISwipeHintThemeAnimationStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ToHorizontalOffsetProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn ToVerticalOffsetProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::ISwipeHintThemeAnimationStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ToVerticalOffsetProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn ISwipeHintThemeAnimationStatics<R, F: FnOnce(&ISwipeHintThemeAnimationStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<SwipeHintThemeAnimation, ISwipeHintThemeAnimationStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for SwipeHintThemeAnimation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SwipeHintThemeAnimation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SwipeHintThemeAnimation {}
impl ::core::fmt::Debug for SwipeHintThemeAnimation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SwipeHintThemeAnimation").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SwipeHintThemeAnimation {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Animation.SwipeHintThemeAnimation;{cdd067c0-580e-4e40-be98-f202d3d84365})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for SwipeHintThemeAnimation {
    type Vtable = ISwipeHintThemeAnimation_Vtbl;
    const IID: ::windows_core::GUID = <ISwipeHintThemeAnimation as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for SwipeHintThemeAnimation {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.SwipeHintThemeAnimation";
}
impl ::core::convert::From<SwipeHintThemeAnimation> for ::windows_core::IUnknown {
    fn from(value: SwipeHintThemeAnimation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SwipeHintThemeAnimation> for ::windows_core::IUnknown {
    fn from(value: &SwipeHintThemeAnimation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for SwipeHintThemeAnimation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a SwipeHintThemeAnimation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SwipeHintThemeAnimation> for ::windows_core::IInspectable {
    fn from(value: SwipeHintThemeAnimation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SwipeHintThemeAnimation> for ::windows_core::IInspectable {
    fn from(value: &SwipeHintThemeAnimation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for SwipeHintThemeAnimation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a SwipeHintThemeAnimation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SwipeHintThemeAnimation> for Timeline {
    fn from(value: SwipeHintThemeAnimation) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&SwipeHintThemeAnimation> for Timeline {
    fn from(value: &SwipeHintThemeAnimation) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, Timeline> for SwipeHintThemeAnimation {
    fn into_param(self) -> ::windows_core::Param<'a, Timeline> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, Timeline> for &SwipeHintThemeAnimation {
    fn into_param(self) -> ::windows_core::Param<'a, Timeline> {
        ::windows_core::Param::Owned(::core::convert::Into::<Timeline>::into(self))
    }
}
impl ::core::convert::From<SwipeHintThemeAnimation> for super::super::DependencyObject {
    fn from(value: SwipeHintThemeAnimation) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&SwipeHintThemeAnimation> for super::super::DependencyObject {
    fn from(value: &SwipeHintThemeAnimation) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::DependencyObject> for SwipeHintThemeAnimation {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::DependencyObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::DependencyObject> for &SwipeHintThemeAnimation {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::DependencyObject> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for SwipeHintThemeAnimation {}
unsafe impl ::core::marker::Sync for SwipeHintThemeAnimation {}
#[repr(transparent)]
pub struct Timeline(::windows_core::IUnknown);
impl Timeline {
    pub fn AutoReverse(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).AutoReverse)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetAutoReverse(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetAutoReverse)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn BeginTime(&self) -> ::windows_core::Result<::winrt_foundation::IReference<::winrt_foundation::TimeSpan>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).BeginTime)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IReference<::winrt_foundation::TimeSpan>>(result__)
        }
    }
    pub fn SetBeginTime<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::IReference<::winrt_foundation::TimeSpan>>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetBeginTime)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn Duration(&self) -> ::windows_core::Result<super::super::Duration> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::Duration>::zeroed();
            (::windows_core::Interface::vtable(this).Duration)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Duration>(result__)
        }
    }
    pub fn SetDuration<'a, Param0: ::windows_core::IntoParam<'a, super::super::Duration>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetDuration)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn SpeedRatio(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).SpeedRatio)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn SetSpeedRatio(&self, value: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetSpeedRatio)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn FillBehavior(&self) -> ::windows_core::Result<FillBehavior> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<FillBehavior>::zeroed();
            (::windows_core::Interface::vtable(this).FillBehavior)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<FillBehavior>(result__)
        }
    }
    pub fn SetFillBehavior(&self, value: FillBehavior) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetFillBehavior)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn RepeatBehavior(&self) -> ::windows_core::Result<RepeatBehavior> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<RepeatBehavior>::zeroed();
            (::windows_core::Interface::vtable(this).RepeatBehavior)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<RepeatBehavior>(result__)
        }
    }
    pub fn SetRepeatBehavior<'a, Param0: ::windows_core::IntoParam<'a, RepeatBehavior>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetRepeatBehavior)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn Completed<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventHandler<::windows_core::IInspectable>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).Completed)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveCompleted<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveCompleted)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn AllowDependentAnimations() -> ::windows_core::Result<bool> {
        Self::ITimelineStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).AllowDependentAnimations)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        })
    }
    pub fn SetAllowDependentAnimations(value: bool) -> ::windows_core::Result<()> {
        Self::ITimelineStatics(|this| unsafe { (::windows_core::Interface::vtable(this).SetAllowDependentAnimations)(::windows_core::Interface::as_raw(this), value).ok() })
    }
    pub fn AutoReverseProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::ITimelineStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).AutoReverseProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn BeginTimeProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::ITimelineStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).BeginTimeProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn DurationProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::ITimelineStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).DurationProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn SpeedRatioProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::ITimelineStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SpeedRatioProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn FillBehaviorProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::ITimelineStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FillBehaviorProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn RepeatBehaviorProperty() -> ::windows_core::Result<super::super::DependencyProperty> {
        Self::ITimelineStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RepeatBehaviorProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::DependencyProperty>(result__)
        })
    }
    pub fn ITimelineStatics<R, F: FnOnce(&ITimelineStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<Timeline, ITimelineStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for Timeline {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for Timeline {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for Timeline {}
impl ::core::fmt::Debug for Timeline {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Timeline").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for Timeline {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Animation.Timeline;{0bc465dc-be4d-4d0d-9549-2208b715f40d})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for Timeline {
    type Vtable = ITimeline_Vtbl;
    const IID: ::windows_core::GUID = <ITimeline as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for Timeline {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.Timeline";
}
impl ::core::convert::From<Timeline> for ::windows_core::IUnknown {
    fn from(value: Timeline) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&Timeline> for ::windows_core::IUnknown {
    fn from(value: &Timeline) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for Timeline {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a Timeline {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<Timeline> for ::windows_core::IInspectable {
    fn from(value: Timeline) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&Timeline> for ::windows_core::IInspectable {
    fn from(value: &Timeline) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for Timeline {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a Timeline {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<Timeline> for super::super::DependencyObject {
    fn from(value: Timeline) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&Timeline> for super::super::DependencyObject {
    fn from(value: &Timeline) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::DependencyObject> for Timeline {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::DependencyObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::DependencyObject> for &Timeline {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::DependencyObject> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for Timeline {}
unsafe impl ::core::marker::Sync for Timeline {}
#[cfg(feature = "winrt-foundation")]
#[repr(transparent)]
pub struct TimelineCollection(::windows_core::IUnknown);
#[cfg(feature = "winrt-foundation")]
impl TimelineCollection {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<TimelineCollection, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn First(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IIterator<Timeline>> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IIterable<Timeline>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).First)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IIterator<Timeline>>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn GetAt(&self, index: u32) -> ::windows_core::Result<Timeline> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetAt)(::windows_core::Interface::as_raw(this), index, result__.as_mut_ptr()).from_abi::<Timeline>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn Size(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).Size)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn GetView(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<Timeline>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetView)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<Timeline>>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn IndexOf<'a, Param0: ::windows_core::IntoParam<'a, Timeline>>(&self, value: Param0, index: &mut u32) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IndexOf)(::windows_core::Interface::as_raw(this), value.into_param().abi(), index, result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn SetAt<'a, Param1: ::windows_core::IntoParam<'a, Timeline>>(&self, index: u32, value: Param1) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetAt)(::windows_core::Interface::as_raw(this), index, value.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn InsertAt<'a, Param1: ::windows_core::IntoParam<'a, Timeline>>(&self, index: u32, value: Param1) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).InsertAt)(::windows_core::Interface::as_raw(this), index, value.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn RemoveAt(&self, index: u32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveAt)(::windows_core::Interface::as_raw(this), index).ok() }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn Append<'a, Param0: ::windows_core::IntoParam<'a, Timeline>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Append)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn RemoveAtEnd(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveAtEnd)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn Clear(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Clear)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn GetMany(&self, startindex: u32, items: &mut [::core::option::Option<Timeline>]) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).GetMany)(::windows_core::Interface::as_raw(this), startindex, items.len() as u32, ::core::mem::transmute_copy(&items), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn ReplaceAll(&self, items: &[::core::option::Option<Timeline>]) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).ReplaceAll)(::windows_core::Interface::as_raw(this), items.len() as u32, ::core::mem::transmute(items.as_ptr())).ok() }
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::clone::Clone for TimelineCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::cmp::PartialEq for TimelineCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::cmp::Eq for TimelineCollection {}
#[cfg(feature = "winrt-foundation")]
impl ::core::fmt::Debug for TimelineCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TimelineCollection").field(&self.0).finish()
    }
}
#[cfg(feature = "winrt-foundation")]
unsafe impl ::windows_core::RuntimeType for TimelineCollection {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Animation.TimelineCollection;pinterface({913337e9-11a1-4345-a3a2-4e7f956e222d};rc(Windows.UI.Xaml.Media.Animation.Timeline;{0bc465dc-be4d-4d0d-9549-2208b715f40d})))");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[cfg(feature = "winrt-foundation")]
unsafe impl ::windows_core::Interface for TimelineCollection {
    type Vtable = ::winrt_foundation::Collections::IVector_Vtbl<Timeline>;
    const IID: ::windows_core::GUID = <::winrt_foundation::Collections::IVector<Timeline> as ::windows_core::Interface>::IID;
}
#[cfg(feature = "winrt-foundation")]
impl ::windows_core::RuntimeName for TimelineCollection {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.TimelineCollection";
}
#[cfg(feature = "winrt-foundation")]
impl ::core::iter::IntoIterator for TimelineCollection {
    type Item = Timeline;
    type IntoIter = ::winrt_foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::iter::IntoIterator for &TimelineCollection {
    type Item = Timeline;
    type IntoIter = ::winrt_foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::winrt_foundation::Collections::VectorIterator::new(::core::convert::TryInto::try_into(self).ok())
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::From<TimelineCollection> for ::windows_core::IUnknown {
    fn from(value: TimelineCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::From<&TimelineCollection> for ::windows_core::IUnknown {
    fn from(value: &TimelineCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for TimelineCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a TimelineCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::From<TimelineCollection> for ::windows_core::IInspectable {
    fn from(value: TimelineCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::From<&TimelineCollection> for ::windows_core::IInspectable {
    fn from(value: &TimelineCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for TimelineCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a TimelineCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::TryFrom<TimelineCollection> for ::winrt_foundation::Collections::IIterable<Timeline> {
    type Error = ::windows_core::Error;
    fn try_from(value: TimelineCollection) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::TryFrom<&TimelineCollection> for ::winrt_foundation::Collections::IIterable<Timeline> {
    type Error = ::windows_core::Error;
    fn try_from(value: &TimelineCollection) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<Timeline>> for TimelineCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::Collections::IIterable<Timeline>> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<Timeline>> for &TimelineCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::Collections::IIterable<Timeline>> {
        ::core::convert::TryInto::<::winrt_foundation::Collections::IIterable<Timeline>>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::TryFrom<TimelineCollection> for ::winrt_foundation::Collections::IVector<Timeline> {
    type Error = ::windows_core::Error;
    fn try_from(value: TimelineCollection) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::TryFrom<&TimelineCollection> for ::winrt_foundation::Collections::IVector<Timeline> {
    type Error = ::windows_core::Error;
    fn try_from(value: &TimelineCollection) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IVector<Timeline>> for TimelineCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::Collections::IVector<Timeline>> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IVector<Timeline>> for &TimelineCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::Collections::IVector<Timeline>> {
        ::core::convert::TryInto::<::winrt_foundation::Collections::IVector<Timeline>>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "winrt-foundation")]
unsafe impl ::core::marker::Send for TimelineCollection {}
#[cfg(feature = "winrt-foundation")]
unsafe impl ::core::marker::Sync for TimelineCollection {}
#[repr(transparent)]
pub struct Transition(::windows_core::IUnknown);
impl Transition {}
impl ::core::clone::Clone for Transition {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for Transition {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for Transition {}
impl ::core::fmt::Debug for Transition {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Transition").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for Transition {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Animation.Transition;{3c677c7c-01d0-4dce-b333-976f93312b08})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for Transition {
    type Vtable = ITransition_Vtbl;
    const IID: ::windows_core::GUID = <ITransition as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for Transition {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.Transition";
}
impl ::core::convert::From<Transition> for ::windows_core::IUnknown {
    fn from(value: Transition) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&Transition> for ::windows_core::IUnknown {
    fn from(value: &Transition) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for Transition {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a Transition {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<Transition> for ::windows_core::IInspectable {
    fn from(value: Transition) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&Transition> for ::windows_core::IInspectable {
    fn from(value: &Transition) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for Transition {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a Transition {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<Transition> for super::super::DependencyObject {
    fn from(value: Transition) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&Transition> for super::super::DependencyObject {
    fn from(value: &Transition) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::DependencyObject> for Transition {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::DependencyObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::super::DependencyObject> for &Transition {
    fn into_param(self) -> ::windows_core::Param<'a, super::super::DependencyObject> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for Transition {}
unsafe impl ::core::marker::Sync for Transition {}
#[cfg(feature = "winrt-foundation")]
#[repr(transparent)]
pub struct TransitionCollection(::windows_core::IUnknown);
#[cfg(feature = "winrt-foundation")]
impl TransitionCollection {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<TransitionCollection, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn First(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IIterator<Transition>> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IIterable<Transition>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).First)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IIterator<Transition>>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn GetAt(&self, index: u32) -> ::windows_core::Result<Transition> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetAt)(::windows_core::Interface::as_raw(this), index, result__.as_mut_ptr()).from_abi::<Transition>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn Size(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).Size)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn GetView(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<Transition>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetView)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<Transition>>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn IndexOf<'a, Param0: ::windows_core::IntoParam<'a, Transition>>(&self, value: Param0, index: &mut u32) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IndexOf)(::windows_core::Interface::as_raw(this), value.into_param().abi(), index, result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn SetAt<'a, Param1: ::windows_core::IntoParam<'a, Transition>>(&self, index: u32, value: Param1) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetAt)(::windows_core::Interface::as_raw(this), index, value.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn InsertAt<'a, Param1: ::windows_core::IntoParam<'a, Transition>>(&self, index: u32, value: Param1) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).InsertAt)(::windows_core::Interface::as_raw(this), index, value.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn RemoveAt(&self, index: u32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveAt)(::windows_core::Interface::as_raw(this), index).ok() }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn Append<'a, Param0: ::windows_core::IntoParam<'a, Transition>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Append)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn RemoveAtEnd(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveAtEnd)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn Clear(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Clear)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn GetMany(&self, startindex: u32, items: &mut [::core::option::Option<Transition>]) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).GetMany)(::windows_core::Interface::as_raw(this), startindex, items.len() as u32, ::core::mem::transmute_copy(&items), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn ReplaceAll(&self, items: &[::core::option::Option<Transition>]) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).ReplaceAll)(::windows_core::Interface::as_raw(this), items.len() as u32, ::core::mem::transmute(items.as_ptr())).ok() }
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::clone::Clone for TransitionCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::cmp::PartialEq for TransitionCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::cmp::Eq for TransitionCollection {}
#[cfg(feature = "winrt-foundation")]
impl ::core::fmt::Debug for TransitionCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TransitionCollection").field(&self.0).finish()
    }
}
#[cfg(feature = "winrt-foundation")]
unsafe impl ::windows_core::RuntimeType for TransitionCollection {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Media.Animation.TransitionCollection;pinterface({913337e9-11a1-4345-a3a2-4e7f956e222d};rc(Windows.UI.Xaml.Media.Animation.Transition;{3c677c7c-01d0-4dce-b333-976f93312b08})))");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[cfg(feature = "winrt-foundation")]
unsafe impl ::windows_core::Interface for TransitionCollection {
    type Vtable = ::winrt_foundation::Collections::IVector_Vtbl<Transition>;
    const IID: ::windows_core::GUID = <::winrt_foundation::Collections::IVector<Transition> as ::windows_core::Interface>::IID;
}
#[cfg(feature = "winrt-foundation")]
impl ::windows_core::RuntimeName for TransitionCollection {
    const NAME: &'static str = "Windows.UI.Xaml.Media.Animation.TransitionCollection";
}
#[cfg(feature = "winrt-foundation")]
impl ::core::iter::IntoIterator for TransitionCollection {
    type Item = Transition;
    type IntoIter = ::winrt_foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::iter::IntoIterator for &TransitionCollection {
    type Item = Transition;
    type IntoIter = ::winrt_foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::winrt_foundation::Collections::VectorIterator::new(::core::convert::TryInto::try_into(self).ok())
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::From<TransitionCollection> for ::windows_core::IUnknown {
    fn from(value: TransitionCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::From<&TransitionCollection> for ::windows_core::IUnknown {
    fn from(value: &TransitionCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for TransitionCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a TransitionCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::From<TransitionCollection> for ::windows_core::IInspectable {
    fn from(value: TransitionCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::From<&TransitionCollection> for ::windows_core::IInspectable {
    fn from(value: &TransitionCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for TransitionCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a TransitionCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::TryFrom<TransitionCollection> for ::winrt_foundation::Collections::IIterable<Transition> {
    type Error = ::windows_core::Error;
    fn try_from(value: TransitionCollection) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::TryFrom<&TransitionCollection> for ::winrt_foundation::Collections::IIterable<Transition> {
    type Error = ::windows_core::Error;
    fn try_from(value: &TransitionCollection) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<Transition>> for TransitionCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::Collections::IIterable<Transition>> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<Transition>> for &TransitionCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::Collections::IIterable<Transition>> {
        ::core::convert::TryInto::<::winrt_foundation::Collections::IIterable<Transition>>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::TryFrom<TransitionCollection> for ::winrt_foundation::Collections::IVector<Transition> {
    type Error = ::windows_core::Error;
    fn try_from(value: TransitionCollection) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::convert::TryFrom<&TransitionCollection> for ::winrt_foundation::Collections::IVector<Transition> {
    type Error = ::windows_core::Error;
    fn try_from(value: &TransitionCollection) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IVector<Transition>> for TransitionCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::Collections::IVector<Transition>> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "winrt-foundation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IVector<Transition>> for &TransitionCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::Collections::IVector<Transition>> {
        ::core::convert::TryInto::<::winrt_foundation::Collections::IVector<Transition>>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "winrt-foundation")]
unsafe impl ::core::marker::Send for TransitionCollection {}
#[cfg(feature = "winrt-foundation")]
unsafe impl ::core::marker::Sync for TransitionCollection {}
