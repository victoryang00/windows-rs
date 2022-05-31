#[cfg(feature = "Management")]
pub mod Management;
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct AdaptiveNotificationContentKind(pub i32);
impl AdaptiveNotificationContentKind {
    pub const Text: Self = Self(0i32);
}
impl ::core::marker::Copy for AdaptiveNotificationContentKind {}
impl ::core::clone::Clone for AdaptiveNotificationContentKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AdaptiveNotificationContentKind {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for AdaptiveNotificationContentKind {
    type Abi = Self;
}
impl ::core::fmt::Debug for AdaptiveNotificationContentKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AdaptiveNotificationContentKind").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AdaptiveNotificationContentKind {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.UI.Notifications.AdaptiveNotificationContentKind;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct AdaptiveNotificationText(::windows_core::IUnknown);
impl AdaptiveNotificationText {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<AdaptiveNotificationText, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn Kind(&self) -> ::windows_core::Result<AdaptiveNotificationContentKind> {
        let this = &::windows_core::Interface::cast::<IAdaptiveNotificationContent>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<AdaptiveNotificationContentKind>::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AdaptiveNotificationContentKind>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Hints(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IMap<::windows_core::HSTRING, ::windows_core::HSTRING>> {
        let this = &::windows_core::Interface::cast::<IAdaptiveNotificationContent>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Hints)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IMap<::windows_core::HSTRING, ::windows_core::HSTRING>>(result__)
        }
    }
    pub fn Text(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Text)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetText<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetText)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn Language(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Language)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetLanguage<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetLanguage)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
}
impl ::core::clone::Clone for AdaptiveNotificationText {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AdaptiveNotificationText {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AdaptiveNotificationText {}
impl ::core::fmt::Debug for AdaptiveNotificationText {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AdaptiveNotificationText").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AdaptiveNotificationText {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Notifications.AdaptiveNotificationText;{46d4a3be-609a-4326-a40b-bfde872034a3})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for AdaptiveNotificationText {
    type Vtable = IAdaptiveNotificationText_Vtbl;
    const IID: ::windows_core::GUID = <IAdaptiveNotificationText as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for AdaptiveNotificationText {
    const NAME: &'static str = "Windows.UI.Notifications.AdaptiveNotificationText";
}
impl ::core::convert::From<AdaptiveNotificationText> for ::windows_core::IUnknown {
    fn from(value: AdaptiveNotificationText) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AdaptiveNotificationText> for ::windows_core::IUnknown {
    fn from(value: &AdaptiveNotificationText) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for AdaptiveNotificationText {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a AdaptiveNotificationText {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AdaptiveNotificationText> for ::windows_core::IInspectable {
    fn from(value: AdaptiveNotificationText) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AdaptiveNotificationText> for ::windows_core::IInspectable {
    fn from(value: &AdaptiveNotificationText) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for AdaptiveNotificationText {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a AdaptiveNotificationText {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<AdaptiveNotificationText> for IAdaptiveNotificationContent {
    type Error = ::windows_core::Error;
    fn try_from(value: AdaptiveNotificationText) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&AdaptiveNotificationText> for IAdaptiveNotificationContent {
    type Error = ::windows_core::Error;
    fn try_from(value: &AdaptiveNotificationText) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IAdaptiveNotificationContent> for AdaptiveNotificationText {
    fn into_param(self) -> ::windows_core::Param<'a, IAdaptiveNotificationContent> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IAdaptiveNotificationContent> for &AdaptiveNotificationText {
    fn into_param(self) -> ::windows_core::Param<'a, IAdaptiveNotificationContent> {
        ::core::convert::TryInto::<IAdaptiveNotificationContent>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for AdaptiveNotificationText {}
unsafe impl ::core::marker::Sync for AdaptiveNotificationText {}
#[repr(transparent)]
pub struct BadgeNotification(::windows_core::IUnknown);
impl BadgeNotification {
    #[cfg(feature = "Data_Xml_Dom")]
    pub fn Content(&self) -> ::windows_core::Result<::winrt_data::Xml::Dom::XmlDocument> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Content)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_data::Xml::Dom::XmlDocument>(result__)
        }
    }
    pub fn SetExpirationTime<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::IReference<::winrt_foundation::DateTime>>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetExpirationTime)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn ExpirationTime(&self) -> ::windows_core::Result<::winrt_foundation::IReference<::winrt_foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ExpirationTime)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IReference<::winrt_foundation::DateTime>>(result__)
        }
    }
    #[cfg(feature = "Data_Xml_Dom")]
    pub fn CreateBadgeNotification<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_data::Xml::Dom::XmlDocument>>(content: Param0) -> ::windows_core::Result<BadgeNotification> {
        Self::IBadgeNotificationFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateBadgeNotification)(::windows_core::Interface::as_raw(this), content.into_param().abi(), result__.as_mut_ptr()).from_abi::<BadgeNotification>(result__)
        })
    }
    pub fn IBadgeNotificationFactory<R, F: FnOnce(&IBadgeNotificationFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<BadgeNotification, IBadgeNotificationFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for BadgeNotification {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for BadgeNotification {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for BadgeNotification {}
impl ::core::fmt::Debug for BadgeNotification {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BadgeNotification").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for BadgeNotification {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Notifications.BadgeNotification;{075cb4ca-d08a-4e2f-9233-7e289c1f7722})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for BadgeNotification {
    type Vtable = IBadgeNotification_Vtbl;
    const IID: ::windows_core::GUID = <IBadgeNotification as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for BadgeNotification {
    const NAME: &'static str = "Windows.UI.Notifications.BadgeNotification";
}
impl ::core::convert::From<BadgeNotification> for ::windows_core::IUnknown {
    fn from(value: BadgeNotification) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BadgeNotification> for ::windows_core::IUnknown {
    fn from(value: &BadgeNotification) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for BadgeNotification {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a BadgeNotification {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<BadgeNotification> for ::windows_core::IInspectable {
    fn from(value: BadgeNotification) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BadgeNotification> for ::windows_core::IInspectable {
    fn from(value: &BadgeNotification) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for BadgeNotification {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a BadgeNotification {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for BadgeNotification {}
unsafe impl ::core::marker::Sync for BadgeNotification {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct BadgeTemplateType(pub i32);
impl BadgeTemplateType {
    pub const BadgeGlyph: Self = Self(0i32);
    pub const BadgeNumber: Self = Self(1i32);
}
impl ::core::marker::Copy for BadgeTemplateType {}
impl ::core::clone::Clone for BadgeTemplateType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for BadgeTemplateType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for BadgeTemplateType {
    type Abi = Self;
}
impl ::core::fmt::Debug for BadgeTemplateType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BadgeTemplateType").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for BadgeTemplateType {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.UI.Notifications.BadgeTemplateType;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
pub struct BadgeUpdateManager;
impl BadgeUpdateManager {
    pub fn CreateBadgeUpdaterForApplication() -> ::windows_core::Result<BadgeUpdater> {
        Self::IBadgeUpdateManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateBadgeUpdaterForApplication)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<BadgeUpdater>(result__)
        })
    }
    pub fn CreateBadgeUpdaterForApplicationWithId<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(applicationid: Param0) -> ::windows_core::Result<BadgeUpdater> {
        Self::IBadgeUpdateManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateBadgeUpdaterForApplicationWithId)(::windows_core::Interface::as_raw(this), applicationid.into_param().abi(), result__.as_mut_ptr()).from_abi::<BadgeUpdater>(result__)
        })
    }
    pub fn CreateBadgeUpdaterForSecondaryTile<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(tileid: Param0) -> ::windows_core::Result<BadgeUpdater> {
        Self::IBadgeUpdateManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateBadgeUpdaterForSecondaryTile)(::windows_core::Interface::as_raw(this), tileid.into_param().abi(), result__.as_mut_ptr()).from_abi::<BadgeUpdater>(result__)
        })
    }
    #[cfg(feature = "Data_Xml_Dom")]
    pub fn GetTemplateContent(r#type: BadgeTemplateType) -> ::windows_core::Result<::winrt_data::Xml::Dom::XmlDocument> {
        Self::IBadgeUpdateManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetTemplateContent)(::windows_core::Interface::as_raw(this), r#type, result__.as_mut_ptr()).from_abi::<::winrt_data::Xml::Dom::XmlDocument>(result__)
        })
    }
    #[cfg(feature = "System")]
    pub fn GetForUser<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_system::User>>(user: Param0) -> ::windows_core::Result<BadgeUpdateManagerForUser> {
        Self::IBadgeUpdateManagerStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetForUser)(::windows_core::Interface::as_raw(this), user.into_param().abi(), result__.as_mut_ptr()).from_abi::<BadgeUpdateManagerForUser>(result__)
        })
    }
    pub fn IBadgeUpdateManagerStatics<R, F: FnOnce(&IBadgeUpdateManagerStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<BadgeUpdateManager, IBadgeUpdateManagerStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IBadgeUpdateManagerStatics2<R, F: FnOnce(&IBadgeUpdateManagerStatics2) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<BadgeUpdateManager, IBadgeUpdateManagerStatics2> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows_core::RuntimeName for BadgeUpdateManager {
    const NAME: &'static str = "Windows.UI.Notifications.BadgeUpdateManager";
}
#[repr(transparent)]
pub struct BadgeUpdateManagerForUser(::windows_core::IUnknown);
impl BadgeUpdateManagerForUser {
    pub fn CreateBadgeUpdaterForApplication(&self) -> ::windows_core::Result<BadgeUpdater> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateBadgeUpdaterForApplication)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<BadgeUpdater>(result__)
        }
    }
    pub fn CreateBadgeUpdaterForApplicationWithId<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, applicationid: Param0) -> ::windows_core::Result<BadgeUpdater> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateBadgeUpdaterForApplicationWithId)(::windows_core::Interface::as_raw(this), applicationid.into_param().abi(), result__.as_mut_ptr()).from_abi::<BadgeUpdater>(result__)
        }
    }
    pub fn CreateBadgeUpdaterForSecondaryTile<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, tileid: Param0) -> ::windows_core::Result<BadgeUpdater> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateBadgeUpdaterForSecondaryTile)(::windows_core::Interface::as_raw(this), tileid.into_param().abi(), result__.as_mut_ptr()).from_abi::<BadgeUpdater>(result__)
        }
    }
    #[cfg(feature = "System")]
    pub fn User(&self) -> ::windows_core::Result<::winrt_system::User> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).User)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_system::User>(result__)
        }
    }
}
impl ::core::clone::Clone for BadgeUpdateManagerForUser {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for BadgeUpdateManagerForUser {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for BadgeUpdateManagerForUser {}
impl ::core::fmt::Debug for BadgeUpdateManagerForUser {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BadgeUpdateManagerForUser").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for BadgeUpdateManagerForUser {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Notifications.BadgeUpdateManagerForUser;{996b21bc-0386-44e5-ba8d-0c1077a62e92})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for BadgeUpdateManagerForUser {
    type Vtable = IBadgeUpdateManagerForUser_Vtbl;
    const IID: ::windows_core::GUID = <IBadgeUpdateManagerForUser as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for BadgeUpdateManagerForUser {
    const NAME: &'static str = "Windows.UI.Notifications.BadgeUpdateManagerForUser";
}
impl ::core::convert::From<BadgeUpdateManagerForUser> for ::windows_core::IUnknown {
    fn from(value: BadgeUpdateManagerForUser) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BadgeUpdateManagerForUser> for ::windows_core::IUnknown {
    fn from(value: &BadgeUpdateManagerForUser) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for BadgeUpdateManagerForUser {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a BadgeUpdateManagerForUser {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<BadgeUpdateManagerForUser> for ::windows_core::IInspectable {
    fn from(value: BadgeUpdateManagerForUser) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BadgeUpdateManagerForUser> for ::windows_core::IInspectable {
    fn from(value: &BadgeUpdateManagerForUser) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for BadgeUpdateManagerForUser {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a BadgeUpdateManagerForUser {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for BadgeUpdateManagerForUser {}
unsafe impl ::core::marker::Sync for BadgeUpdateManagerForUser {}
#[repr(transparent)]
pub struct BadgeUpdater(::windows_core::IUnknown);
impl BadgeUpdater {
    pub fn Update<'a, Param0: ::windows_core::IntoParam<'a, BadgeNotification>>(&self, notification: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Update)(::windows_core::Interface::as_raw(this), notification.into_param().abi()).ok() }
    }
    pub fn Clear(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Clear)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn StartPeriodicUpdate<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Uri>>(&self, badgecontent: Param0, requestedinterval: PeriodicUpdateRecurrence) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).StartPeriodicUpdate)(::windows_core::Interface::as_raw(this), badgecontent.into_param().abi(), requestedinterval).ok() }
    }
    pub fn StartPeriodicUpdateAtTime<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Uri>, Param1: ::windows_core::IntoParam<'a, ::winrt_foundation::DateTime>>(&self, badgecontent: Param0, starttime: Param1, requestedinterval: PeriodicUpdateRecurrence) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).StartPeriodicUpdateAtTime)(::windows_core::Interface::as_raw(this), badgecontent.into_param().abi(), starttime.into_param().abi(), requestedinterval).ok() }
    }
    pub fn StopPeriodicUpdate(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).StopPeriodicUpdate)(::windows_core::Interface::as_raw(this)).ok() }
    }
}
impl ::core::clone::Clone for BadgeUpdater {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for BadgeUpdater {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for BadgeUpdater {}
impl ::core::fmt::Debug for BadgeUpdater {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BadgeUpdater").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for BadgeUpdater {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Notifications.BadgeUpdater;{b5fa1fd4-7562-4f6c-bfa3-1b6ed2e57f2f})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for BadgeUpdater {
    type Vtable = IBadgeUpdater_Vtbl;
    const IID: ::windows_core::GUID = <IBadgeUpdater as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for BadgeUpdater {
    const NAME: &'static str = "Windows.UI.Notifications.BadgeUpdater";
}
impl ::core::convert::From<BadgeUpdater> for ::windows_core::IUnknown {
    fn from(value: BadgeUpdater) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BadgeUpdater> for ::windows_core::IUnknown {
    fn from(value: &BadgeUpdater) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for BadgeUpdater {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a BadgeUpdater {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<BadgeUpdater> for ::windows_core::IInspectable {
    fn from(value: BadgeUpdater) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BadgeUpdater> for ::windows_core::IInspectable {
    fn from(value: &BadgeUpdater) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for BadgeUpdater {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a BadgeUpdater {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for BadgeUpdater {}
unsafe impl ::core::marker::Sync for BadgeUpdater {}
#[repr(transparent)]
pub struct IAdaptiveNotificationContent(::windows_core::IUnknown);
impl IAdaptiveNotificationContent {
    pub fn Kind(&self) -> ::windows_core::Result<AdaptiveNotificationContentKind> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<AdaptiveNotificationContentKind>::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AdaptiveNotificationContentKind>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Hints(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IMap<::windows_core::HSTRING, ::windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Hints)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IMap<::windows_core::HSTRING, ::windows_core::HSTRING>>(result__)
        }
    }
}
impl ::core::convert::From<IAdaptiveNotificationContent> for ::windows_core::IUnknown {
    fn from(value: IAdaptiveNotificationContent) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAdaptiveNotificationContent> for ::windows_core::IUnknown {
    fn from(value: &IAdaptiveNotificationContent) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IAdaptiveNotificationContent {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IAdaptiveNotificationContent {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IAdaptiveNotificationContent> for ::windows_core::IInspectable {
    fn from(value: IAdaptiveNotificationContent) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAdaptiveNotificationContent> for ::windows_core::IInspectable {
    fn from(value: &IAdaptiveNotificationContent) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for IAdaptiveNotificationContent {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a IAdaptiveNotificationContent {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IAdaptiveNotificationContent {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAdaptiveNotificationContent {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAdaptiveNotificationContent {}
impl ::core::fmt::Debug for IAdaptiveNotificationContent {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAdaptiveNotificationContent").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for IAdaptiveNotificationContent {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"{eb0dbe66-7448-448d-9db8-d78acd2abba9}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for IAdaptiveNotificationContent {
    type Vtable = IAdaptiveNotificationContent_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xeb0dbe66_7448_448d_9db8_d78acd2abba9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAdaptiveNotificationContent_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Kind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AdaptiveNotificationContentKind) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Hints: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Hints: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAdaptiveNotificationText(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAdaptiveNotificationText {
    type Vtable = IAdaptiveNotificationText_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x46d4a3be_609a_4326_a40b_bfde872034a3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAdaptiveNotificationText_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Text: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Language: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetLanguage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBadgeNotification(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IBadgeNotification {
    type Vtable = IBadgeNotification_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x075cb4ca_d08a_4e2f_9233_7e289c1f7722);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBadgeNotification_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Data_Xml_Dom")]
    pub Content: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Data_Xml_Dom"))]
    Content: usize,
    pub SetExpirationTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ExpirationTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBadgeNotificationFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IBadgeNotificationFactory {
    type Vtable = IBadgeNotificationFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xedf255ce_0618_4d59_948a_5a61040c52f9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBadgeNotificationFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Data_Xml_Dom")]
    pub CreateBadgeNotification: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, content: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Data_Xml_Dom"))]
    CreateBadgeNotification: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBadgeUpdateManagerForUser(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IBadgeUpdateManagerForUser {
    type Vtable = IBadgeUpdateManagerForUser_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x996b21bc_0386_44e5_ba8d_0c1077a62e92);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBadgeUpdateManagerForUser_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CreateBadgeUpdaterForApplication: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CreateBadgeUpdaterForApplicationWithId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, applicationid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CreateBadgeUpdaterForSecondaryTile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, tileid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "System")]
    pub User: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "System"))]
    User: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBadgeUpdateManagerStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IBadgeUpdateManagerStatics {
    type Vtable = IBadgeUpdateManagerStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x33400faa_6dd5_4105_aebc_9b50fca492da);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBadgeUpdateManagerStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CreateBadgeUpdaterForApplication: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CreateBadgeUpdaterForApplicationWithId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, applicationid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CreateBadgeUpdaterForSecondaryTile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, tileid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "Data_Xml_Dom")]
    pub GetTemplateContent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: BadgeTemplateType, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Data_Xml_Dom"))]
    GetTemplateContent: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBadgeUpdateManagerStatics2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IBadgeUpdateManagerStatics2 {
    type Vtable = IBadgeUpdateManagerStatics2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x979a35ce_f940_48bf_94e8_ca244d400b41);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBadgeUpdateManagerStatics2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "System")]
    pub GetForUser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "System"))]
    GetForUser: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBadgeUpdater(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IBadgeUpdater {
    type Vtable = IBadgeUpdater_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb5fa1fd4_7562_4f6c_bfa3_1b6ed2e57f2f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBadgeUpdater_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Update: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, notification: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Clear: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub StartPeriodicUpdate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, badgecontent: ::windows_core::RawPtr, requestedinterval: PeriodicUpdateRecurrence) -> ::windows_core::HRESULT,
    pub StartPeriodicUpdateAtTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, badgecontent: ::windows_core::RawPtr, starttime: ::winrt_foundation::DateTime, requestedinterval: PeriodicUpdateRecurrence) -> ::windows_core::HRESULT,
    pub StopPeriodicUpdate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IKnownAdaptiveNotificationHintsStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IKnownAdaptiveNotificationHintsStatics {
    type Vtable = IKnownAdaptiveNotificationHintsStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x06206598_d496_497d_8692_4f7d7c2770df);
}
#[repr(C)]
#[doc(hidden)]
pub struct IKnownAdaptiveNotificationHintsStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Style: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Wrap: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub MaxLines: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub MinLines: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub TextStacking: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Align: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IKnownAdaptiveNotificationTextStylesStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IKnownAdaptiveNotificationTextStylesStatics {
    type Vtable = IKnownAdaptiveNotificationTextStylesStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x202192d7_8996_45aa_8ba1_d461d72c2a1b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IKnownAdaptiveNotificationTextStylesStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Caption: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Body: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Base: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Subtitle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Title: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Subheader: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Header: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub TitleNumeral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SubheaderNumeral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub HeaderNumeral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub CaptionSubtle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub BodySubtle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub BaseSubtle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SubtitleSubtle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub TitleSubtle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SubheaderSubtle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SubheaderNumeralSubtle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub HeaderSubtle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub HeaderNumeralSubtle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IKnownNotificationBindingsStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IKnownNotificationBindingsStatics {
    type Vtable = IKnownNotificationBindingsStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x79427bae_a8b7_4d58_89ea_76a7b7bccded);
}
#[repr(C)]
#[doc(hidden)]
pub struct IKnownNotificationBindingsStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub ToastGeneric: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct INotification(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for INotification {
    type Vtable = INotification_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x108037fe_eb76_4f82_97bc_da07530a2e20);
}
#[repr(C)]
#[doc(hidden)]
pub struct INotification_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub ExpirationTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetExpirationTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Visual: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetVisual: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct INotificationBinding(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for INotificationBinding {
    type Vtable = INotificationBinding_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf29e4b85_0370_4ad3_b4ea_da9e35e7eabf);
}
#[repr(C)]
#[doc(hidden)]
pub struct INotificationBinding_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Template: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetTemplate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Language: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetLanguage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Hints: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Hints: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetTextElements: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetTextElements: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct INotificationData(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for INotificationData {
    type Vtable = INotificationData_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9ffd2312_9d6a_4aaf_b6ac_ff17f0c1f280);
}
#[repr(C)]
#[doc(hidden)]
pub struct INotificationData_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub Values: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Values: usize,
    pub SequenceNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub SetSequenceNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct INotificationDataFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for INotificationDataFactory {
    type Vtable = INotificationDataFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x23c1e33a_1c10_46fb_8040_dec384621cf8);
}
#[repr(C)]
#[doc(hidden)]
pub struct INotificationDataFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateNotificationDataWithValuesAndSequenceNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, initialvalues: ::windows_core::RawPtr, sequencenumber: u32, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateNotificationDataWithValuesAndSequenceNumber: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateNotificationDataWithValues: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, initialvalues: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateNotificationDataWithValues: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct INotificationVisual(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for INotificationVisual {
    type Vtable = INotificationVisual_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x68835b8e_aa56_4e11_86d3_5f9a6957bc5b);
}
#[repr(C)]
#[doc(hidden)]
pub struct INotificationVisual_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Language: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetLanguage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Bindings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Bindings: usize,
    pub GetBinding: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, templatename: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IScheduledTileNotification(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IScheduledTileNotification {
    type Vtable = IScheduledTileNotification_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0abca6d5_99dc_4c78_a11c_c9e7f86d7ef7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IScheduledTileNotification_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Data_Xml_Dom")]
    pub Content: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Data_Xml_Dom"))]
    Content: usize,
    pub DeliveryTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::DateTime) -> ::windows_core::HRESULT,
    pub SetExpirationTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ExpirationTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetTag: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Tag: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IScheduledTileNotificationFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IScheduledTileNotificationFactory {
    type Vtable = IScheduledTileNotificationFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3383138a_98c0_4c3b_bbd6_4a633c7cfc29);
}
#[repr(C)]
#[doc(hidden)]
pub struct IScheduledTileNotificationFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Data_Xml_Dom")]
    pub CreateScheduledTileNotification: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, content: ::windows_core::RawPtr, deliverytime: ::winrt_foundation::DateTime, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Data_Xml_Dom"))]
    CreateScheduledTileNotification: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IScheduledToastNotification(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IScheduledToastNotification {
    type Vtable = IScheduledToastNotification_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x79f577f8_0de7_48cd_9740_9b370490c838);
}
#[repr(C)]
#[doc(hidden)]
pub struct IScheduledToastNotification_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Data_Xml_Dom")]
    pub Content: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Data_Xml_Dom"))]
    Content: usize,
    pub DeliveryTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::DateTime) -> ::windows_core::HRESULT,
    pub SnoozeInterval: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub MaximumSnoozeCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub SetId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IScheduledToastNotification2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IScheduledToastNotification2 {
    type Vtable = IScheduledToastNotification2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa66ea09c_31b4_43b0_b5dd_7a40e85363b1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IScheduledToastNotification2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub SetTag: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Tag: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetGroup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Group: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetSuppressPopup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub SuppressPopup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IScheduledToastNotification3(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IScheduledToastNotification3 {
    type Vtable = IScheduledToastNotification3_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x98429e8b_bd32_4a3b_9d15_22aea49462a1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IScheduledToastNotification3_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub NotificationMirroring: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut NotificationMirroring) -> ::windows_core::HRESULT,
    pub SetNotificationMirroring: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: NotificationMirroring) -> ::windows_core::HRESULT,
    pub RemoteId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetRemoteId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IScheduledToastNotification4(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IScheduledToastNotification4 {
    type Vtable = IScheduledToastNotification4_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1d4761fd_bdef_4e4a_96be_0101369b58d2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IScheduledToastNotification4_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub ExpirationTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetExpirationTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IScheduledToastNotificationFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IScheduledToastNotificationFactory {
    type Vtable = IScheduledToastNotificationFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe7bed191_0bb9_4189_8394_31761b476fd7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IScheduledToastNotificationFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Data_Xml_Dom")]
    pub CreateScheduledToastNotification: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, content: ::windows_core::RawPtr, deliverytime: ::winrt_foundation::DateTime, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Data_Xml_Dom"))]
    CreateScheduledToastNotification: usize,
    #[cfg(feature = "Data_Xml_Dom")]
    pub CreateScheduledToastNotificationRecurring: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, content: ::windows_core::RawPtr, deliverytime: ::winrt_foundation::DateTime, snoozeinterval: ::winrt_foundation::TimeSpan, maximumsnoozecount: u32, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Data_Xml_Dom"))]
    CreateScheduledToastNotificationRecurring: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IScheduledToastNotificationShowingEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IScheduledToastNotificationShowingEventArgs {
    type Vtable = IScheduledToastNotificationShowingEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6173f6b4_412a_5e2c_a6ed_a0209aef9a09);
}
#[repr(C)]
#[doc(hidden)]
pub struct IScheduledToastNotificationShowingEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Cancel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetCancel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub ScheduledToastNotification: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IShownTileNotification(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IShownTileNotification {
    type Vtable = IShownTileNotification_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x342d8988_5af2_481a_a6a3_f2fdc78de88e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IShownTileNotification_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Arguments: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITileFlyoutNotification(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ITileFlyoutNotification {
    type Vtable = ITileFlyoutNotification_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9a53b261_c70c_42be_b2f3_f42aa97d34e5);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITileFlyoutNotification_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Data_Xml_Dom")]
    pub Content: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Data_Xml_Dom"))]
    Content: usize,
    pub SetExpirationTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ExpirationTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITileFlyoutNotificationFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ITileFlyoutNotificationFactory {
    type Vtable = ITileFlyoutNotificationFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xef556ff5_5226_4f2b_b278_88a35dfe569f);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITileFlyoutNotificationFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Data_Xml_Dom")]
    pub CreateTileFlyoutNotification: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, content: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Data_Xml_Dom"))]
    CreateTileFlyoutNotification: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITileFlyoutUpdateManagerStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ITileFlyoutUpdateManagerStatics {
    type Vtable = ITileFlyoutUpdateManagerStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x04363b0b_1ac0_4b99_88e7_ada83e953d48);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITileFlyoutUpdateManagerStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CreateTileFlyoutUpdaterForApplication: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CreateTileFlyoutUpdaterForApplicationWithId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, applicationid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CreateTileFlyoutUpdaterForSecondaryTile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, tileid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "Data_Xml_Dom")]
    pub GetTemplateContent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: TileFlyoutTemplateType, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Data_Xml_Dom"))]
    GetTemplateContent: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITileFlyoutUpdater(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ITileFlyoutUpdater {
    type Vtable = ITileFlyoutUpdater_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8d40c76a_c465_4052_a740_5c2654c1a089);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITileFlyoutUpdater_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Update: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, notification: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Clear: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub StartPeriodicUpdate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, tileflyoutcontent: ::windows_core::RawPtr, requestedinterval: PeriodicUpdateRecurrence) -> ::windows_core::HRESULT,
    pub StartPeriodicUpdateAtTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, tileflyoutcontent: ::windows_core::RawPtr, starttime: ::winrt_foundation::DateTime, requestedinterval: PeriodicUpdateRecurrence) -> ::windows_core::HRESULT,
    pub StopPeriodicUpdate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Setting: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut NotificationSetting) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITileNotification(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ITileNotification {
    type Vtable = ITileNotification_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xebaec8fa_50ec_4c18_b4d0_3af02e5540ab);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITileNotification_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Data_Xml_Dom")]
    pub Content: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Data_Xml_Dom"))]
    Content: usize,
    pub SetExpirationTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ExpirationTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetTag: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Tag: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITileNotificationFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ITileNotificationFactory {
    type Vtable = ITileNotificationFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc6abdd6e_4928_46c8_bdbf_81a047dea0d4);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITileNotificationFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Data_Xml_Dom")]
    pub CreateTileNotification: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, content: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Data_Xml_Dom"))]
    CreateTileNotification: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITileUpdateManagerForUser(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ITileUpdateManagerForUser {
    type Vtable = ITileUpdateManagerForUser_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x55141348_2ee2_4e2d_9cc1_216a20decc9f);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITileUpdateManagerForUser_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CreateTileUpdaterForApplication: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CreateTileUpdaterForApplicationWithId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, applicationid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CreateTileUpdaterForSecondaryTile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, tileid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "System")]
    pub User: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "System"))]
    User: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITileUpdateManagerStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ITileUpdateManagerStatics {
    type Vtable = ITileUpdateManagerStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xda159e5d_3ea9_4986_8d84_b09d5e12276d);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITileUpdateManagerStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CreateTileUpdaterForApplication: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CreateTileUpdaterForApplicationWithId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, applicationid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CreateTileUpdaterForSecondaryTile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, tileid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "Data_Xml_Dom")]
    pub GetTemplateContent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: TileTemplateType, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Data_Xml_Dom"))]
    GetTemplateContent: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITileUpdateManagerStatics2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ITileUpdateManagerStatics2 {
    type Vtable = ITileUpdateManagerStatics2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x731c1ddc_8e14_4b7c_a34b_9d22de76c84d);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITileUpdateManagerStatics2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "System")]
    pub GetForUser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "System"))]
    GetForUser: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITileUpdater(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ITileUpdater {
    type Vtable = ITileUpdater_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0942a48b_1d91_44ec_9243_c1e821c29a20);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITileUpdater_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Update: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, notification: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Clear: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub EnableNotificationQueue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, enable: bool) -> ::windows_core::HRESULT,
    pub Setting: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut NotificationSetting) -> ::windows_core::HRESULT,
    pub AddToSchedule: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, scheduledtile: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub RemoveFromSchedule: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, scheduledtile: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub GetScheduledTileNotifications: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetScheduledTileNotifications: usize,
    pub StartPeriodicUpdate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, tilecontent: ::windows_core::RawPtr, requestedinterval: PeriodicUpdateRecurrence) -> ::windows_core::HRESULT,
    pub StartPeriodicUpdateAtTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, tilecontent: ::windows_core::RawPtr, starttime: ::winrt_foundation::DateTime, requestedinterval: PeriodicUpdateRecurrence) -> ::windows_core::HRESULT,
    pub StopPeriodicUpdate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub StartPeriodicUpdateBatch: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, tilecontents: ::windows_core::RawPtr, requestedinterval: PeriodicUpdateRecurrence) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    StartPeriodicUpdateBatch: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub StartPeriodicUpdateBatchAtTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, tilecontents: ::windows_core::RawPtr, starttime: ::winrt_foundation::DateTime, requestedinterval: PeriodicUpdateRecurrence) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    StartPeriodicUpdateBatchAtTime: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITileUpdater2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ITileUpdater2 {
    type Vtable = ITileUpdater2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa2266e12_15ee_43ed_83f5_65b352bb1a84);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITileUpdater2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub EnableNotificationQueueForSquare150x150: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, enable: bool) -> ::windows_core::HRESULT,
    pub EnableNotificationQueueForWide310x150: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, enable: bool) -> ::windows_core::HRESULT,
    pub EnableNotificationQueueForSquare310x310: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, enable: bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IToastActivatedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IToastActivatedEventArgs {
    type Vtable = IToastActivatedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe3bf92f3_c197_436f_8265_0625824f8dac);
}
#[repr(C)]
#[doc(hidden)]
pub struct IToastActivatedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Arguments: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IToastActivatedEventArgs2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IToastActivatedEventArgs2 {
    type Vtable = IToastActivatedEventArgs2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xab7da512_cc61_568e_81be_304ac31038fa);
}
#[repr(C)]
#[doc(hidden)]
pub struct IToastActivatedEventArgs2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub UserInput: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    UserInput: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IToastCollection(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IToastCollection {
    type Vtable = IToastCollection_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0a8bc3b0_e0be_4858_bc2a_89dfe0b32863);
}
#[repr(C)]
#[doc(hidden)]
pub struct IToastCollection_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub DisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetDisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub LaunchArgs: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetLaunchArgs: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Icon: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetIcon: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IToastCollectionFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IToastCollectionFactory {
    type Vtable = IToastCollectionFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x164dd3d7_73c4_44f7_b4ff_fb6d4bf1f4c6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IToastCollectionFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CreateInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, collectionid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, displayname: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, launchargs: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, iconuri: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IToastCollectionManager(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IToastCollectionManager {
    type Vtable = IToastCollectionManager_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2a1821fe_179d_49bc_b79d_a527920d3665);
}
#[repr(C)]
#[doc(hidden)]
pub struct IToastCollectionManager_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub SaveToastCollectionAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, collection: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub FindAllToastCollectionsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FindAllToastCollectionsAsync: usize,
    pub GetToastCollectionAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, collectionid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub RemoveToastCollectionAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, collectionid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub RemoveAllToastCollectionsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "System")]
    pub User: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "System"))]
    User: usize,
    pub AppId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IToastDismissedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IToastDismissedEventArgs {
    type Vtable = IToastDismissedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3f89d935_d9cb_4538_a0f0_ffe7659938f8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IToastDismissedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Reason: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ToastDismissalReason) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IToastFailedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IToastFailedEventArgs {
    type Vtable = IToastFailedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x35176862_cfd4_44f8_ad64_f500fd896c3b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IToastFailedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub ErrorCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::HRESULT) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IToastNotification(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IToastNotification {
    type Vtable = IToastNotification_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x997e2675_059e_4e60_8b06_1760917c8b80);
}
#[repr(C)]
#[doc(hidden)]
pub struct IToastNotification_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Data_Xml_Dom")]
    pub Content: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Data_Xml_Dom"))]
    Content: usize,
    pub SetExpirationTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ExpirationTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Dismissed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveDismissed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub Activated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveActivated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub Failed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveFailed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IToastNotification2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IToastNotification2 {
    type Vtable = IToastNotification2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9dfb9fd1_143a_490e_90bf_b9fba7132de7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IToastNotification2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub SetTag: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Tag: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetGroup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Group: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetSuppressPopup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub SuppressPopup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IToastNotification3(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IToastNotification3 {
    type Vtable = IToastNotification3_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x31e8aed8_8141_4f99_bc0a_c4ed21297d77);
}
#[repr(C)]
#[doc(hidden)]
pub struct IToastNotification3_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub NotificationMirroring: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut NotificationMirroring) -> ::windows_core::HRESULT,
    pub SetNotificationMirroring: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: NotificationMirroring) -> ::windows_core::HRESULT,
    pub RemoteId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetRemoteId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IToastNotification4(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IToastNotification4 {
    type Vtable = IToastNotification4_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x15154935_28ea_4727_88e9_c58680e2d118);
}
#[repr(C)]
#[doc(hidden)]
pub struct IToastNotification4_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Data: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Priority: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ToastNotificationPriority) -> ::windows_core::HRESULT,
    pub SetPriority: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ToastNotificationPriority) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IToastNotification6(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IToastNotification6 {
    type Vtable = IToastNotification6_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x43ebfe53_89ae_5c1e_a279_3aecfe9b6f54);
}
#[repr(C)]
#[doc(hidden)]
pub struct IToastNotification6_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub ExpiresOnReboot: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetExpiresOnReboot: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IToastNotificationActionTriggerDetail(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IToastNotificationActionTriggerDetail {
    type Vtable = IToastNotificationActionTriggerDetail_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9445135a_38f3_42f6_96aa_7955b0f03da2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IToastNotificationActionTriggerDetail_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Argument: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub UserInput: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    UserInput: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IToastNotificationFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IToastNotificationFactory {
    type Vtable = IToastNotificationFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x04124b20_82c6_4229_b109_fd9ed4662b53);
}
#[repr(C)]
#[doc(hidden)]
pub struct IToastNotificationFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Data_Xml_Dom")]
    pub CreateToastNotification: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, content: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Data_Xml_Dom"))]
    CreateToastNotification: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IToastNotificationHistory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IToastNotificationHistory {
    type Vtable = IToastNotificationHistory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5caddc63_01d3_4c97_986f_0533483fee14);
}
#[repr(C)]
#[doc(hidden)]
pub struct IToastNotificationHistory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub RemoveGroup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, group: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub RemoveGroupWithId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, group: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, applicationid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub RemoveGroupedTagWithId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, tag: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, group: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, applicationid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub RemoveGroupedTag: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, tag: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, group: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Remove: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, tag: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Clear: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub ClearWithId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, applicationid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IToastNotificationHistory2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IToastNotificationHistory2 {
    type Vtable = IToastNotificationHistory2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3bc3d253_2f31_4092_9129_8ad5abf067da);
}
#[repr(C)]
#[doc(hidden)]
pub struct IToastNotificationHistory2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub GetHistory: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetHistory: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetHistoryWithId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, applicationid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetHistoryWithId: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IToastNotificationHistoryChangedTriggerDetail(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IToastNotificationHistoryChangedTriggerDetail {
    type Vtable = IToastNotificationHistoryChangedTriggerDetail_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xdb037ffa_0068_412c_9c83_267c37f65670);
}
#[repr(C)]
#[doc(hidden)]
pub struct IToastNotificationHistoryChangedTriggerDetail_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub ChangeType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ToastHistoryChangedType) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IToastNotificationHistoryChangedTriggerDetail2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IToastNotificationHistoryChangedTriggerDetail2 {
    type Vtable = IToastNotificationHistoryChangedTriggerDetail2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0b36e982_c871_49fb_babb_25bdbc4cc45b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IToastNotificationHistoryChangedTriggerDetail2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CollectionId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IToastNotificationManagerForUser(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IToastNotificationManagerForUser {
    type Vtable = IToastNotificationManagerForUser_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x79ab57f6_43fe_487b_8a7f_99567200ae94);
}
#[repr(C)]
#[doc(hidden)]
pub struct IToastNotificationManagerForUser_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CreateToastNotifier: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CreateToastNotifierWithId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, applicationid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub History: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "System")]
    pub User: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "System"))]
    User: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IToastNotificationManagerForUser2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IToastNotificationManagerForUser2 {
    type Vtable = IToastNotificationManagerForUser2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x679c64b7_81ab_42c2_8819_c958767753f4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IToastNotificationManagerForUser2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub GetToastNotifierForToastCollectionIdAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, collectionid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetHistoryForToastCollectionIdAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, collectionid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetToastCollectionManager: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetToastCollectionManagerWithAppId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, appid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IToastNotificationManagerStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IToastNotificationManagerStatics {
    type Vtable = IToastNotificationManagerStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x50ac103f_d235_4598_bbef_98fe4d1a3ad4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IToastNotificationManagerStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CreateToastNotifier: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CreateToastNotifierWithId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, applicationid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "Data_Xml_Dom")]
    pub GetTemplateContent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: ToastTemplateType, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Data_Xml_Dom"))]
    GetTemplateContent: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IToastNotificationManagerStatics2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IToastNotificationManagerStatics2 {
    type Vtable = IToastNotificationManagerStatics2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7ab93c52_0e48_4750_ba9d_1a4113981847);
}
#[repr(C)]
#[doc(hidden)]
pub struct IToastNotificationManagerStatics2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub History: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IToastNotificationManagerStatics4(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IToastNotificationManagerStatics4 {
    type Vtable = IToastNotificationManagerStatics4_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8f993fd3_e516_45fb_8130_398e93fa52c3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IToastNotificationManagerStatics4_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "System")]
    pub GetForUser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "System"))]
    GetForUser: usize,
    pub ConfigureNotificationMirroring: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: NotificationMirroring) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IToastNotificationManagerStatics5(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IToastNotificationManagerStatics5 {
    type Vtable = IToastNotificationManagerStatics5_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd6f5f569_d40d_407c_8989_88cab42cfd14);
}
#[repr(C)]
#[doc(hidden)]
pub struct IToastNotificationManagerStatics5_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub GetDefault: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IToastNotifier(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IToastNotifier {
    type Vtable = IToastNotifier_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x75927b93_03f3_41ec_91d3_6e5bac1b38e7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IToastNotifier_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Show: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, notification: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Hide: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, notification: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Setting: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut NotificationSetting) -> ::windows_core::HRESULT,
    pub AddToSchedule: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, scheduledtoast: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub RemoveFromSchedule: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, scheduledtoast: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub GetScheduledToastNotifications: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetScheduledToastNotifications: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IToastNotifier2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IToastNotifier2 {
    type Vtable = IToastNotifier2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x354389c6_7c01_4bd5_9c20_604340cd2b74);
}
#[repr(C)]
#[doc(hidden)]
pub struct IToastNotifier2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub UpdateWithTagAndGroup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, data: ::windows_core::RawPtr, tag: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, group: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut NotificationUpdateResult) -> ::windows_core::HRESULT,
    pub UpdateWithTag: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, data: ::windows_core::RawPtr, tag: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut NotificationUpdateResult) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IToastNotifier3(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IToastNotifier3 {
    type Vtable = IToastNotifier3_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xae75a04a_3b0c_51ad_b7e8_b08ab6052549);
}
#[repr(C)]
#[doc(hidden)]
pub struct IToastNotifier3_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub ScheduledToastNotificationShowing: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveScheduledToastNotificationShowing: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IUserNotification(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IUserNotification {
    type Vtable = IUserNotification_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xadf7e52f_4e53_42d5_9c33_eb5ea515b23e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserNotification_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Notification: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "ApplicationModel")]
    pub AppInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "ApplicationModel"))]
    AppInfo: usize,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub CreationTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::DateTime) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IUserNotificationChangedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IUserNotificationChangedEventArgs {
    type Vtable = IUserNotificationChangedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb6bd6839_79cf_4b25_82c0_0ce1eef81f8c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserNotificationChangedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub ChangeKind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut UserNotificationChangedKind) -> ::windows_core::HRESULT,
    pub UserNotificationId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
}
pub struct KnownAdaptiveNotificationHints;
impl KnownAdaptiveNotificationHints {
    pub fn Style() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IKnownAdaptiveNotificationHintsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Style)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn Wrap() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IKnownAdaptiveNotificationHintsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Wrap)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn MaxLines() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IKnownAdaptiveNotificationHintsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).MaxLines)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn MinLines() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IKnownAdaptiveNotificationHintsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).MinLines)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn TextStacking() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IKnownAdaptiveNotificationHintsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).TextStacking)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn Align() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IKnownAdaptiveNotificationHintsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Align)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn IKnownAdaptiveNotificationHintsStatics<R, F: FnOnce(&IKnownAdaptiveNotificationHintsStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<KnownAdaptiveNotificationHints, IKnownAdaptiveNotificationHintsStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows_core::RuntimeName for KnownAdaptiveNotificationHints {
    const NAME: &'static str = "Windows.UI.Notifications.KnownAdaptiveNotificationHints";
}
pub struct KnownAdaptiveNotificationTextStyles;
impl KnownAdaptiveNotificationTextStyles {
    pub fn Caption() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IKnownAdaptiveNotificationTextStylesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Caption)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn Body() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IKnownAdaptiveNotificationTextStylesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Body)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn Base() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IKnownAdaptiveNotificationTextStylesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Base)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn Subtitle() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IKnownAdaptiveNotificationTextStylesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Subtitle)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn Title() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IKnownAdaptiveNotificationTextStylesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Title)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn Subheader() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IKnownAdaptiveNotificationTextStylesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Subheader)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn Header() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IKnownAdaptiveNotificationTextStylesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Header)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn TitleNumeral() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IKnownAdaptiveNotificationTextStylesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).TitleNumeral)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn SubheaderNumeral() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IKnownAdaptiveNotificationTextStylesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).SubheaderNumeral)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn HeaderNumeral() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IKnownAdaptiveNotificationTextStylesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).HeaderNumeral)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn CaptionSubtle() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IKnownAdaptiveNotificationTextStylesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).CaptionSubtle)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn BodySubtle() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IKnownAdaptiveNotificationTextStylesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).BodySubtle)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn BaseSubtle() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IKnownAdaptiveNotificationTextStylesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).BaseSubtle)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn SubtitleSubtle() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IKnownAdaptiveNotificationTextStylesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).SubtitleSubtle)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn TitleSubtle() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IKnownAdaptiveNotificationTextStylesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).TitleSubtle)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn SubheaderSubtle() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IKnownAdaptiveNotificationTextStylesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).SubheaderSubtle)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn SubheaderNumeralSubtle() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IKnownAdaptiveNotificationTextStylesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).SubheaderNumeralSubtle)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn HeaderSubtle() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IKnownAdaptiveNotificationTextStylesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).HeaderSubtle)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn HeaderNumeralSubtle() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IKnownAdaptiveNotificationTextStylesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).HeaderNumeralSubtle)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn IKnownAdaptiveNotificationTextStylesStatics<R, F: FnOnce(&IKnownAdaptiveNotificationTextStylesStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<KnownAdaptiveNotificationTextStyles, IKnownAdaptiveNotificationTextStylesStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows_core::RuntimeName for KnownAdaptiveNotificationTextStyles {
    const NAME: &'static str = "Windows.UI.Notifications.KnownAdaptiveNotificationTextStyles";
}
pub struct KnownNotificationBindings;
impl KnownNotificationBindings {
    pub fn ToastGeneric() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IKnownNotificationBindingsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).ToastGeneric)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn IKnownNotificationBindingsStatics<R, F: FnOnce(&IKnownNotificationBindingsStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<KnownNotificationBindings, IKnownNotificationBindingsStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows_core::RuntimeName for KnownNotificationBindings {
    const NAME: &'static str = "Windows.UI.Notifications.KnownNotificationBindings";
}
#[repr(transparent)]
pub struct Notification(::windows_core::IUnknown);
impl Notification {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<Notification, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn ExpirationTime(&self) -> ::windows_core::Result<::winrt_foundation::IReference<::winrt_foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ExpirationTime)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IReference<::winrt_foundation::DateTime>>(result__)
        }
    }
    pub fn SetExpirationTime<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::IReference<::winrt_foundation::DateTime>>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetExpirationTime)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn Visual(&self) -> ::windows_core::Result<NotificationVisual> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Visual)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<NotificationVisual>(result__)
        }
    }
    pub fn SetVisual<'a, Param0: ::windows_core::IntoParam<'a, NotificationVisual>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetVisual)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
}
impl ::core::clone::Clone for Notification {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for Notification {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for Notification {}
impl ::core::fmt::Debug for Notification {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Notification").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for Notification {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Notifications.Notification;{108037fe-eb76-4f82-97bc-da07530a2e20})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for Notification {
    type Vtable = INotification_Vtbl;
    const IID: ::windows_core::GUID = <INotification as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for Notification {
    const NAME: &'static str = "Windows.UI.Notifications.Notification";
}
impl ::core::convert::From<Notification> for ::windows_core::IUnknown {
    fn from(value: Notification) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&Notification> for ::windows_core::IUnknown {
    fn from(value: &Notification) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for Notification {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a Notification {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<Notification> for ::windows_core::IInspectable {
    fn from(value: Notification) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&Notification> for ::windows_core::IInspectable {
    fn from(value: &Notification) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for Notification {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a Notification {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for Notification {}
unsafe impl ::core::marker::Sync for Notification {}
#[repr(transparent)]
pub struct NotificationBinding(::windows_core::IUnknown);
impl NotificationBinding {
    pub fn Template(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Template)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetTemplate<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetTemplate)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn Language(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Language)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetLanguage<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetLanguage)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Hints(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IMap<::windows_core::HSTRING, ::windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Hints)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IMap<::windows_core::HSTRING, ::windows_core::HSTRING>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetTextElements(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<AdaptiveNotificationText>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetTextElements)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<AdaptiveNotificationText>>(result__)
        }
    }
}
impl ::core::clone::Clone for NotificationBinding {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for NotificationBinding {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for NotificationBinding {}
impl ::core::fmt::Debug for NotificationBinding {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NotificationBinding").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for NotificationBinding {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Notifications.NotificationBinding;{f29e4b85-0370-4ad3-b4ea-da9e35e7eabf})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for NotificationBinding {
    type Vtable = INotificationBinding_Vtbl;
    const IID: ::windows_core::GUID = <INotificationBinding as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for NotificationBinding {
    const NAME: &'static str = "Windows.UI.Notifications.NotificationBinding";
}
impl ::core::convert::From<NotificationBinding> for ::windows_core::IUnknown {
    fn from(value: NotificationBinding) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&NotificationBinding> for ::windows_core::IUnknown {
    fn from(value: &NotificationBinding) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for NotificationBinding {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a NotificationBinding {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<NotificationBinding> for ::windows_core::IInspectable {
    fn from(value: NotificationBinding) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&NotificationBinding> for ::windows_core::IInspectable {
    fn from(value: &NotificationBinding) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for NotificationBinding {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a NotificationBinding {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for NotificationBinding {}
unsafe impl ::core::marker::Sync for NotificationBinding {}
#[repr(transparent)]
pub struct NotificationData(::windows_core::IUnknown);
impl NotificationData {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<NotificationData, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Values(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IMap<::windows_core::HSTRING, ::windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Values)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IMap<::windows_core::HSTRING, ::windows_core::HSTRING>>(result__)
        }
    }
    pub fn SequenceNumber(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).SequenceNumber)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn SetSequenceNumber(&self, value: u32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetSequenceNumber)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateNotificationDataWithValuesAndSequenceNumber<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<::winrt_foundation::Collections::IKeyValuePair<::windows_core::HSTRING, ::windows_core::HSTRING>>>>(initialvalues: Param0, sequencenumber: u32) -> ::windows_core::Result<NotificationData> {
        Self::INotificationDataFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateNotificationDataWithValuesAndSequenceNumber)(::windows_core::Interface::as_raw(this), initialvalues.into_param().abi(), sequencenumber, result__.as_mut_ptr()).from_abi::<NotificationData>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateNotificationDataWithValues<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<::winrt_foundation::Collections::IKeyValuePair<::windows_core::HSTRING, ::windows_core::HSTRING>>>>(initialvalues: Param0) -> ::windows_core::Result<NotificationData> {
        Self::INotificationDataFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateNotificationDataWithValues)(::windows_core::Interface::as_raw(this), initialvalues.into_param().abi(), result__.as_mut_ptr()).from_abi::<NotificationData>(result__)
        })
    }
    pub fn INotificationDataFactory<R, F: FnOnce(&INotificationDataFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<NotificationData, INotificationDataFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for NotificationData {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for NotificationData {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for NotificationData {}
impl ::core::fmt::Debug for NotificationData {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NotificationData").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for NotificationData {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Notifications.NotificationData;{9ffd2312-9d6a-4aaf-b6ac-ff17f0c1f280})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for NotificationData {
    type Vtable = INotificationData_Vtbl;
    const IID: ::windows_core::GUID = <INotificationData as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for NotificationData {
    const NAME: &'static str = "Windows.UI.Notifications.NotificationData";
}
impl ::core::convert::From<NotificationData> for ::windows_core::IUnknown {
    fn from(value: NotificationData) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&NotificationData> for ::windows_core::IUnknown {
    fn from(value: &NotificationData) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for NotificationData {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a NotificationData {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<NotificationData> for ::windows_core::IInspectable {
    fn from(value: NotificationData) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&NotificationData> for ::windows_core::IInspectable {
    fn from(value: &NotificationData) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for NotificationData {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a NotificationData {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for NotificationData {}
unsafe impl ::core::marker::Sync for NotificationData {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct NotificationKinds(pub u32);
impl NotificationKinds {
    pub const Unknown: Self = Self(0u32);
    pub const Toast: Self = Self(1u32);
}
impl ::core::marker::Copy for NotificationKinds {}
impl ::core::clone::Clone for NotificationKinds {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for NotificationKinds {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for NotificationKinds {
    type Abi = Self;
}
impl ::core::fmt::Debug for NotificationKinds {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NotificationKinds").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for NotificationKinds {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for NotificationKinds {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for NotificationKinds {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for NotificationKinds {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for NotificationKinds {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
unsafe impl ::windows_core::RuntimeType for NotificationKinds {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.UI.Notifications.NotificationKinds;u4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct NotificationMirroring(pub i32);
impl NotificationMirroring {
    pub const Allowed: Self = Self(0i32);
    pub const Disabled: Self = Self(1i32);
}
impl ::core::marker::Copy for NotificationMirroring {}
impl ::core::clone::Clone for NotificationMirroring {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for NotificationMirroring {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for NotificationMirroring {
    type Abi = Self;
}
impl ::core::fmt::Debug for NotificationMirroring {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NotificationMirroring").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for NotificationMirroring {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.UI.Notifications.NotificationMirroring;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct NotificationSetting(pub i32);
impl NotificationSetting {
    pub const Enabled: Self = Self(0i32);
    pub const DisabledForApplication: Self = Self(1i32);
    pub const DisabledForUser: Self = Self(2i32);
    pub const DisabledByGroupPolicy: Self = Self(3i32);
    pub const DisabledByManifest: Self = Self(4i32);
}
impl ::core::marker::Copy for NotificationSetting {}
impl ::core::clone::Clone for NotificationSetting {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for NotificationSetting {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for NotificationSetting {
    type Abi = Self;
}
impl ::core::fmt::Debug for NotificationSetting {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NotificationSetting").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for NotificationSetting {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.UI.Notifications.NotificationSetting;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct NotificationUpdateResult(pub i32);
impl NotificationUpdateResult {
    pub const Succeeded: Self = Self(0i32);
    pub const Failed: Self = Self(1i32);
    pub const NotificationNotFound: Self = Self(2i32);
}
impl ::core::marker::Copy for NotificationUpdateResult {}
impl ::core::clone::Clone for NotificationUpdateResult {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for NotificationUpdateResult {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for NotificationUpdateResult {
    type Abi = Self;
}
impl ::core::fmt::Debug for NotificationUpdateResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NotificationUpdateResult").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for NotificationUpdateResult {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.UI.Notifications.NotificationUpdateResult;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct NotificationVisual(::windows_core::IUnknown);
impl NotificationVisual {
    pub fn Language(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Language)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetLanguage<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetLanguage)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Bindings(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVector<NotificationBinding>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Bindings)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVector<NotificationBinding>>(result__)
        }
    }
    pub fn GetBinding<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, templatename: Param0) -> ::windows_core::Result<NotificationBinding> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetBinding)(::windows_core::Interface::as_raw(this), templatename.into_param().abi(), result__.as_mut_ptr()).from_abi::<NotificationBinding>(result__)
        }
    }
}
impl ::core::clone::Clone for NotificationVisual {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for NotificationVisual {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for NotificationVisual {}
impl ::core::fmt::Debug for NotificationVisual {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NotificationVisual").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for NotificationVisual {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Notifications.NotificationVisual;{68835b8e-aa56-4e11-86d3-5f9a6957bc5b})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for NotificationVisual {
    type Vtable = INotificationVisual_Vtbl;
    const IID: ::windows_core::GUID = <INotificationVisual as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for NotificationVisual {
    const NAME: &'static str = "Windows.UI.Notifications.NotificationVisual";
}
impl ::core::convert::From<NotificationVisual> for ::windows_core::IUnknown {
    fn from(value: NotificationVisual) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&NotificationVisual> for ::windows_core::IUnknown {
    fn from(value: &NotificationVisual) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for NotificationVisual {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a NotificationVisual {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<NotificationVisual> for ::windows_core::IInspectable {
    fn from(value: NotificationVisual) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&NotificationVisual> for ::windows_core::IInspectable {
    fn from(value: &NotificationVisual) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for NotificationVisual {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a NotificationVisual {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for NotificationVisual {}
unsafe impl ::core::marker::Sync for NotificationVisual {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PeriodicUpdateRecurrence(pub i32);
impl PeriodicUpdateRecurrence {
    pub const HalfHour: Self = Self(0i32);
    pub const Hour: Self = Self(1i32);
    pub const SixHours: Self = Self(2i32);
    pub const TwelveHours: Self = Self(3i32);
    pub const Daily: Self = Self(4i32);
}
impl ::core::marker::Copy for PeriodicUpdateRecurrence {}
impl ::core::clone::Clone for PeriodicUpdateRecurrence {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PeriodicUpdateRecurrence {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for PeriodicUpdateRecurrence {
    type Abi = Self;
}
impl ::core::fmt::Debug for PeriodicUpdateRecurrence {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PeriodicUpdateRecurrence").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PeriodicUpdateRecurrence {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.UI.Notifications.PeriodicUpdateRecurrence;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct ScheduledTileNotification(::windows_core::IUnknown);
impl ScheduledTileNotification {
    #[cfg(feature = "Data_Xml_Dom")]
    pub fn Content(&self) -> ::windows_core::Result<::winrt_data::Xml::Dom::XmlDocument> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Content)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_data::Xml::Dom::XmlDocument>(result__)
        }
    }
    pub fn DeliveryTime(&self) -> ::windows_core::Result<::winrt_foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::DateTime>::zeroed();
            (::windows_core::Interface::vtable(this).DeliveryTime)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::DateTime>(result__)
        }
    }
    pub fn SetExpirationTime<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::IReference<::winrt_foundation::DateTime>>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetExpirationTime)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn ExpirationTime(&self) -> ::windows_core::Result<::winrt_foundation::IReference<::winrt_foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ExpirationTime)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IReference<::winrt_foundation::DateTime>>(result__)
        }
    }
    pub fn SetTag<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetTag)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn Tag(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Tag)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetId<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetId)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn Id(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Id)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Data_Xml_Dom")]
    pub fn CreateScheduledTileNotification<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_data::Xml::Dom::XmlDocument>, Param1: ::windows_core::IntoParam<'a, ::winrt_foundation::DateTime>>(content: Param0, deliverytime: Param1) -> ::windows_core::Result<ScheduledTileNotification> {
        Self::IScheduledTileNotificationFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateScheduledTileNotification)(::windows_core::Interface::as_raw(this), content.into_param().abi(), deliverytime.into_param().abi(), result__.as_mut_ptr()).from_abi::<ScheduledTileNotification>(result__)
        })
    }
    pub fn IScheduledTileNotificationFactory<R, F: FnOnce(&IScheduledTileNotificationFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<ScheduledTileNotification, IScheduledTileNotificationFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for ScheduledTileNotification {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ScheduledTileNotification {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ScheduledTileNotification {}
impl ::core::fmt::Debug for ScheduledTileNotification {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ScheduledTileNotification").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ScheduledTileNotification {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Notifications.ScheduledTileNotification;{0abca6d5-99dc-4c78-a11c-c9e7f86d7ef7})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for ScheduledTileNotification {
    type Vtable = IScheduledTileNotification_Vtbl;
    const IID: ::windows_core::GUID = <IScheduledTileNotification as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for ScheduledTileNotification {
    const NAME: &'static str = "Windows.UI.Notifications.ScheduledTileNotification";
}
impl ::core::convert::From<ScheduledTileNotification> for ::windows_core::IUnknown {
    fn from(value: ScheduledTileNotification) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ScheduledTileNotification> for ::windows_core::IUnknown {
    fn from(value: &ScheduledTileNotification) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ScheduledTileNotification {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ScheduledTileNotification {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ScheduledTileNotification> for ::windows_core::IInspectable {
    fn from(value: ScheduledTileNotification) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ScheduledTileNotification> for ::windows_core::IInspectable {
    fn from(value: &ScheduledTileNotification) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for ScheduledTileNotification {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a ScheduledTileNotification {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for ScheduledTileNotification {}
unsafe impl ::core::marker::Sync for ScheduledTileNotification {}
#[repr(transparent)]
pub struct ScheduledToastNotification(::windows_core::IUnknown);
impl ScheduledToastNotification {
    #[cfg(feature = "Data_Xml_Dom")]
    pub fn Content(&self) -> ::windows_core::Result<::winrt_data::Xml::Dom::XmlDocument> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Content)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_data::Xml::Dom::XmlDocument>(result__)
        }
    }
    pub fn DeliveryTime(&self) -> ::windows_core::Result<::winrt_foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::DateTime>::zeroed();
            (::windows_core::Interface::vtable(this).DeliveryTime)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::DateTime>(result__)
        }
    }
    pub fn SnoozeInterval(&self) -> ::windows_core::Result<::winrt_foundation::IReference<::winrt_foundation::TimeSpan>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SnoozeInterval)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IReference<::winrt_foundation::TimeSpan>>(result__)
        }
    }
    pub fn MaximumSnoozeCount(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).MaximumSnoozeCount)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn SetId<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetId)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn Id(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Id)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetTag<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IScheduledToastNotification2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetTag)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn Tag(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IScheduledToastNotification2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Tag)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetGroup<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IScheduledToastNotification2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetGroup)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn Group(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IScheduledToastNotification2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Group)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetSuppressPopup(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IScheduledToastNotification2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetSuppressPopup)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn SuppressPopup(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IScheduledToastNotification2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).SuppressPopup)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn NotificationMirroring(&self) -> ::windows_core::Result<NotificationMirroring> {
        let this = &::windows_core::Interface::cast::<IScheduledToastNotification3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<NotificationMirroring>::zeroed();
            (::windows_core::Interface::vtable(this).NotificationMirroring)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<NotificationMirroring>(result__)
        }
    }
    pub fn SetNotificationMirroring(&self, value: NotificationMirroring) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IScheduledToastNotification3>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetNotificationMirroring)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn RemoteId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IScheduledToastNotification3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).RemoteId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetRemoteId<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IScheduledToastNotification3>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetRemoteId)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn ExpirationTime(&self) -> ::windows_core::Result<::winrt_foundation::IReference<::winrt_foundation::DateTime>> {
        let this = &::windows_core::Interface::cast::<IScheduledToastNotification4>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ExpirationTime)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IReference<::winrt_foundation::DateTime>>(result__)
        }
    }
    pub fn SetExpirationTime<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::IReference<::winrt_foundation::DateTime>>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IScheduledToastNotification4>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetExpirationTime)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Data_Xml_Dom")]
    pub fn CreateScheduledToastNotification<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_data::Xml::Dom::XmlDocument>, Param1: ::windows_core::IntoParam<'a, ::winrt_foundation::DateTime>>(content: Param0, deliverytime: Param1) -> ::windows_core::Result<ScheduledToastNotification> {
        Self::IScheduledToastNotificationFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateScheduledToastNotification)(::windows_core::Interface::as_raw(this), content.into_param().abi(), deliverytime.into_param().abi(), result__.as_mut_ptr()).from_abi::<ScheduledToastNotification>(result__)
        })
    }
    #[cfg(feature = "Data_Xml_Dom")]
    pub fn CreateScheduledToastNotificationRecurring<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_data::Xml::Dom::XmlDocument>, Param1: ::windows_core::IntoParam<'a, ::winrt_foundation::DateTime>, Param2: ::windows_core::IntoParam<'a, ::winrt_foundation::TimeSpan>>(content: Param0, deliverytime: Param1, snoozeinterval: Param2, maximumsnoozecount: u32) -> ::windows_core::Result<ScheduledToastNotification> {
        Self::IScheduledToastNotificationFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateScheduledToastNotificationRecurring)(::windows_core::Interface::as_raw(this), content.into_param().abi(), deliverytime.into_param().abi(), snoozeinterval.into_param().abi(), maximumsnoozecount, result__.as_mut_ptr()).from_abi::<ScheduledToastNotification>(result__)
        })
    }
    pub fn IScheduledToastNotificationFactory<R, F: FnOnce(&IScheduledToastNotificationFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<ScheduledToastNotification, IScheduledToastNotificationFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for ScheduledToastNotification {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ScheduledToastNotification {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ScheduledToastNotification {}
impl ::core::fmt::Debug for ScheduledToastNotification {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ScheduledToastNotification").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ScheduledToastNotification {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Notifications.ScheduledToastNotification;{79f577f8-0de7-48cd-9740-9b370490c838})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for ScheduledToastNotification {
    type Vtable = IScheduledToastNotification_Vtbl;
    const IID: ::windows_core::GUID = <IScheduledToastNotification as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for ScheduledToastNotification {
    const NAME: &'static str = "Windows.UI.Notifications.ScheduledToastNotification";
}
impl ::core::convert::From<ScheduledToastNotification> for ::windows_core::IUnknown {
    fn from(value: ScheduledToastNotification) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ScheduledToastNotification> for ::windows_core::IUnknown {
    fn from(value: &ScheduledToastNotification) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ScheduledToastNotification {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ScheduledToastNotification {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ScheduledToastNotification> for ::windows_core::IInspectable {
    fn from(value: ScheduledToastNotification) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ScheduledToastNotification> for ::windows_core::IInspectable {
    fn from(value: &ScheduledToastNotification) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for ScheduledToastNotification {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a ScheduledToastNotification {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for ScheduledToastNotification {}
unsafe impl ::core::marker::Sync for ScheduledToastNotification {}
#[repr(transparent)]
pub struct ScheduledToastNotificationShowingEventArgs(::windows_core::IUnknown);
impl ScheduledToastNotificationShowingEventArgs {
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
    pub fn ScheduledToastNotification(&self) -> ::windows_core::Result<ScheduledToastNotification> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ScheduledToastNotification)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ScheduledToastNotification>(result__)
        }
    }
    pub fn GetDeferral(&self) -> ::windows_core::Result<::winrt_foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetDeferral)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Deferral>(result__)
        }
    }
}
impl ::core::clone::Clone for ScheduledToastNotificationShowingEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ScheduledToastNotificationShowingEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ScheduledToastNotificationShowingEventArgs {}
impl ::core::fmt::Debug for ScheduledToastNotificationShowingEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ScheduledToastNotificationShowingEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ScheduledToastNotificationShowingEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Notifications.ScheduledToastNotificationShowingEventArgs;{6173f6b4-412a-5e2c-a6ed-a0209aef9a09})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for ScheduledToastNotificationShowingEventArgs {
    type Vtable = IScheduledToastNotificationShowingEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IScheduledToastNotificationShowingEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for ScheduledToastNotificationShowingEventArgs {
    const NAME: &'static str = "Windows.UI.Notifications.ScheduledToastNotificationShowingEventArgs";
}
impl ::core::convert::From<ScheduledToastNotificationShowingEventArgs> for ::windows_core::IUnknown {
    fn from(value: ScheduledToastNotificationShowingEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ScheduledToastNotificationShowingEventArgs> for ::windows_core::IUnknown {
    fn from(value: &ScheduledToastNotificationShowingEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ScheduledToastNotificationShowingEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ScheduledToastNotificationShowingEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ScheduledToastNotificationShowingEventArgs> for ::windows_core::IInspectable {
    fn from(value: ScheduledToastNotificationShowingEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ScheduledToastNotificationShowingEventArgs> for ::windows_core::IInspectable {
    fn from(value: &ScheduledToastNotificationShowingEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for ScheduledToastNotificationShowingEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a ScheduledToastNotificationShowingEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for ScheduledToastNotificationShowingEventArgs {}
unsafe impl ::core::marker::Sync for ScheduledToastNotificationShowingEventArgs {}
#[repr(transparent)]
pub struct ShownTileNotification(::windows_core::IUnknown);
impl ShownTileNotification {
    pub fn Arguments(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Arguments)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
}
impl ::core::clone::Clone for ShownTileNotification {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ShownTileNotification {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ShownTileNotification {}
impl ::core::fmt::Debug for ShownTileNotification {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ShownTileNotification").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ShownTileNotification {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Notifications.ShownTileNotification;{342d8988-5af2-481a-a6a3-f2fdc78de88e})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for ShownTileNotification {
    type Vtable = IShownTileNotification_Vtbl;
    const IID: ::windows_core::GUID = <IShownTileNotification as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for ShownTileNotification {
    const NAME: &'static str = "Windows.UI.Notifications.ShownTileNotification";
}
impl ::core::convert::From<ShownTileNotification> for ::windows_core::IUnknown {
    fn from(value: ShownTileNotification) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ShownTileNotification> for ::windows_core::IUnknown {
    fn from(value: &ShownTileNotification) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ShownTileNotification {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ShownTileNotification {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ShownTileNotification> for ::windows_core::IInspectable {
    fn from(value: ShownTileNotification) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ShownTileNotification> for ::windows_core::IInspectable {
    fn from(value: &ShownTileNotification) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for ShownTileNotification {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a ShownTileNotification {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for ShownTileNotification {}
unsafe impl ::core::marker::Sync for ShownTileNotification {}
#[repr(transparent)]
pub struct TileFlyoutNotification(::windows_core::IUnknown);
impl TileFlyoutNotification {
    #[cfg(feature = "Data_Xml_Dom")]
    pub fn Content(&self) -> ::windows_core::Result<::winrt_data::Xml::Dom::XmlDocument> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Content)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_data::Xml::Dom::XmlDocument>(result__)
        }
    }
    pub fn SetExpirationTime<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::IReference<::winrt_foundation::DateTime>>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetExpirationTime)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn ExpirationTime(&self) -> ::windows_core::Result<::winrt_foundation::IReference<::winrt_foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ExpirationTime)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IReference<::winrt_foundation::DateTime>>(result__)
        }
    }
    #[cfg(feature = "Data_Xml_Dom")]
    pub fn CreateTileFlyoutNotification<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_data::Xml::Dom::XmlDocument>>(content: Param0) -> ::windows_core::Result<TileFlyoutNotification> {
        Self::ITileFlyoutNotificationFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateTileFlyoutNotification)(::windows_core::Interface::as_raw(this), content.into_param().abi(), result__.as_mut_ptr()).from_abi::<TileFlyoutNotification>(result__)
        })
    }
    pub fn ITileFlyoutNotificationFactory<R, F: FnOnce(&ITileFlyoutNotificationFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<TileFlyoutNotification, ITileFlyoutNotificationFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for TileFlyoutNotification {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for TileFlyoutNotification {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for TileFlyoutNotification {}
impl ::core::fmt::Debug for TileFlyoutNotification {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TileFlyoutNotification").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for TileFlyoutNotification {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Notifications.TileFlyoutNotification;{9a53b261-c70c-42be-b2f3-f42aa97d34e5})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for TileFlyoutNotification {
    type Vtable = ITileFlyoutNotification_Vtbl;
    const IID: ::windows_core::GUID = <ITileFlyoutNotification as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for TileFlyoutNotification {
    const NAME: &'static str = "Windows.UI.Notifications.TileFlyoutNotification";
}
impl ::core::convert::From<TileFlyoutNotification> for ::windows_core::IUnknown {
    fn from(value: TileFlyoutNotification) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&TileFlyoutNotification> for ::windows_core::IUnknown {
    fn from(value: &TileFlyoutNotification) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for TileFlyoutNotification {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a TileFlyoutNotification {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<TileFlyoutNotification> for ::windows_core::IInspectable {
    fn from(value: TileFlyoutNotification) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&TileFlyoutNotification> for ::windows_core::IInspectable {
    fn from(value: &TileFlyoutNotification) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for TileFlyoutNotification {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a TileFlyoutNotification {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for TileFlyoutNotification {}
unsafe impl ::core::marker::Sync for TileFlyoutNotification {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct TileFlyoutTemplateType(pub i32);
impl TileFlyoutTemplateType {
    pub const TileFlyoutTemplate01: Self = Self(0i32);
}
impl ::core::marker::Copy for TileFlyoutTemplateType {}
impl ::core::clone::Clone for TileFlyoutTemplateType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TileFlyoutTemplateType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for TileFlyoutTemplateType {
    type Abi = Self;
}
impl ::core::fmt::Debug for TileFlyoutTemplateType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TileFlyoutTemplateType").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for TileFlyoutTemplateType {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.UI.Notifications.TileFlyoutTemplateType;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
pub struct TileFlyoutUpdateManager;
impl TileFlyoutUpdateManager {
    pub fn CreateTileFlyoutUpdaterForApplication() -> ::windows_core::Result<TileFlyoutUpdater> {
        Self::ITileFlyoutUpdateManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateTileFlyoutUpdaterForApplication)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<TileFlyoutUpdater>(result__)
        })
    }
    pub fn CreateTileFlyoutUpdaterForApplicationWithId<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(applicationid: Param0) -> ::windows_core::Result<TileFlyoutUpdater> {
        Self::ITileFlyoutUpdateManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateTileFlyoutUpdaterForApplicationWithId)(::windows_core::Interface::as_raw(this), applicationid.into_param().abi(), result__.as_mut_ptr()).from_abi::<TileFlyoutUpdater>(result__)
        })
    }
    pub fn CreateTileFlyoutUpdaterForSecondaryTile<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(tileid: Param0) -> ::windows_core::Result<TileFlyoutUpdater> {
        Self::ITileFlyoutUpdateManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateTileFlyoutUpdaterForSecondaryTile)(::windows_core::Interface::as_raw(this), tileid.into_param().abi(), result__.as_mut_ptr()).from_abi::<TileFlyoutUpdater>(result__)
        })
    }
    #[cfg(feature = "Data_Xml_Dom")]
    pub fn GetTemplateContent(r#type: TileFlyoutTemplateType) -> ::windows_core::Result<::winrt_data::Xml::Dom::XmlDocument> {
        Self::ITileFlyoutUpdateManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetTemplateContent)(::windows_core::Interface::as_raw(this), r#type, result__.as_mut_ptr()).from_abi::<::winrt_data::Xml::Dom::XmlDocument>(result__)
        })
    }
    pub fn ITileFlyoutUpdateManagerStatics<R, F: FnOnce(&ITileFlyoutUpdateManagerStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<TileFlyoutUpdateManager, ITileFlyoutUpdateManagerStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows_core::RuntimeName for TileFlyoutUpdateManager {
    const NAME: &'static str = "Windows.UI.Notifications.TileFlyoutUpdateManager";
}
#[repr(transparent)]
pub struct TileFlyoutUpdater(::windows_core::IUnknown);
impl TileFlyoutUpdater {
    pub fn Update<'a, Param0: ::windows_core::IntoParam<'a, TileFlyoutNotification>>(&self, notification: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Update)(::windows_core::Interface::as_raw(this), notification.into_param().abi()).ok() }
    }
    pub fn Clear(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Clear)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn StartPeriodicUpdate<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Uri>>(&self, tileflyoutcontent: Param0, requestedinterval: PeriodicUpdateRecurrence) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).StartPeriodicUpdate)(::windows_core::Interface::as_raw(this), tileflyoutcontent.into_param().abi(), requestedinterval).ok() }
    }
    pub fn StartPeriodicUpdateAtTime<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Uri>, Param1: ::windows_core::IntoParam<'a, ::winrt_foundation::DateTime>>(&self, tileflyoutcontent: Param0, starttime: Param1, requestedinterval: PeriodicUpdateRecurrence) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).StartPeriodicUpdateAtTime)(::windows_core::Interface::as_raw(this), tileflyoutcontent.into_param().abi(), starttime.into_param().abi(), requestedinterval).ok() }
    }
    pub fn StopPeriodicUpdate(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).StopPeriodicUpdate)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Setting(&self) -> ::windows_core::Result<NotificationSetting> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<NotificationSetting>::zeroed();
            (::windows_core::Interface::vtable(this).Setting)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<NotificationSetting>(result__)
        }
    }
}
impl ::core::clone::Clone for TileFlyoutUpdater {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for TileFlyoutUpdater {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for TileFlyoutUpdater {}
impl ::core::fmt::Debug for TileFlyoutUpdater {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TileFlyoutUpdater").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for TileFlyoutUpdater {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Notifications.TileFlyoutUpdater;{8d40c76a-c465-4052-a740-5c2654c1a089})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for TileFlyoutUpdater {
    type Vtable = ITileFlyoutUpdater_Vtbl;
    const IID: ::windows_core::GUID = <ITileFlyoutUpdater as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for TileFlyoutUpdater {
    const NAME: &'static str = "Windows.UI.Notifications.TileFlyoutUpdater";
}
impl ::core::convert::From<TileFlyoutUpdater> for ::windows_core::IUnknown {
    fn from(value: TileFlyoutUpdater) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&TileFlyoutUpdater> for ::windows_core::IUnknown {
    fn from(value: &TileFlyoutUpdater) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for TileFlyoutUpdater {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a TileFlyoutUpdater {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<TileFlyoutUpdater> for ::windows_core::IInspectable {
    fn from(value: TileFlyoutUpdater) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&TileFlyoutUpdater> for ::windows_core::IInspectable {
    fn from(value: &TileFlyoutUpdater) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for TileFlyoutUpdater {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a TileFlyoutUpdater {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(transparent)]
pub struct TileNotification(::windows_core::IUnknown);
impl TileNotification {
    #[cfg(feature = "Data_Xml_Dom")]
    pub fn Content(&self) -> ::windows_core::Result<::winrt_data::Xml::Dom::XmlDocument> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Content)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_data::Xml::Dom::XmlDocument>(result__)
        }
    }
    pub fn SetExpirationTime<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::IReference<::winrt_foundation::DateTime>>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetExpirationTime)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn ExpirationTime(&self) -> ::windows_core::Result<::winrt_foundation::IReference<::winrt_foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ExpirationTime)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IReference<::winrt_foundation::DateTime>>(result__)
        }
    }
    pub fn SetTag<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetTag)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn Tag(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Tag)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Data_Xml_Dom")]
    pub fn CreateTileNotification<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_data::Xml::Dom::XmlDocument>>(content: Param0) -> ::windows_core::Result<TileNotification> {
        Self::ITileNotificationFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateTileNotification)(::windows_core::Interface::as_raw(this), content.into_param().abi(), result__.as_mut_ptr()).from_abi::<TileNotification>(result__)
        })
    }
    pub fn ITileNotificationFactory<R, F: FnOnce(&ITileNotificationFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<TileNotification, ITileNotificationFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for TileNotification {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for TileNotification {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for TileNotification {}
impl ::core::fmt::Debug for TileNotification {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TileNotification").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for TileNotification {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Notifications.TileNotification;{ebaec8fa-50ec-4c18-b4d0-3af02e5540ab})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for TileNotification {
    type Vtable = ITileNotification_Vtbl;
    const IID: ::windows_core::GUID = <ITileNotification as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for TileNotification {
    const NAME: &'static str = "Windows.UI.Notifications.TileNotification";
}
impl ::core::convert::From<TileNotification> for ::windows_core::IUnknown {
    fn from(value: TileNotification) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&TileNotification> for ::windows_core::IUnknown {
    fn from(value: &TileNotification) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for TileNotification {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a TileNotification {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<TileNotification> for ::windows_core::IInspectable {
    fn from(value: TileNotification) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&TileNotification> for ::windows_core::IInspectable {
    fn from(value: &TileNotification) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for TileNotification {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a TileNotification {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for TileNotification {}
unsafe impl ::core::marker::Sync for TileNotification {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct TileTemplateType(pub i32);
impl TileTemplateType {
    pub const TileSquareImage: Self = Self(0i32);
    pub const TileSquareBlock: Self = Self(1i32);
    pub const TileSquareText01: Self = Self(2i32);
    pub const TileSquareText02: Self = Self(3i32);
    pub const TileSquareText03: Self = Self(4i32);
    pub const TileSquareText04: Self = Self(5i32);
    pub const TileSquarePeekImageAndText01: Self = Self(6i32);
    pub const TileSquarePeekImageAndText02: Self = Self(7i32);
    pub const TileSquarePeekImageAndText03: Self = Self(8i32);
    pub const TileSquarePeekImageAndText04: Self = Self(9i32);
    pub const TileWideImage: Self = Self(10i32);
    pub const TileWideImageCollection: Self = Self(11i32);
    pub const TileWideImageAndText01: Self = Self(12i32);
    pub const TileWideImageAndText02: Self = Self(13i32);
    pub const TileWideBlockAndText01: Self = Self(14i32);
    pub const TileWideBlockAndText02: Self = Self(15i32);
    pub const TileWidePeekImageCollection01: Self = Self(16i32);
    pub const TileWidePeekImageCollection02: Self = Self(17i32);
    pub const TileWidePeekImageCollection03: Self = Self(18i32);
    pub const TileWidePeekImageCollection04: Self = Self(19i32);
    pub const TileWidePeekImageCollection05: Self = Self(20i32);
    pub const TileWidePeekImageCollection06: Self = Self(21i32);
    pub const TileWidePeekImageAndText01: Self = Self(22i32);
    pub const TileWidePeekImageAndText02: Self = Self(23i32);
    pub const TileWidePeekImage01: Self = Self(24i32);
    pub const TileWidePeekImage02: Self = Self(25i32);
    pub const TileWidePeekImage03: Self = Self(26i32);
    pub const TileWidePeekImage04: Self = Self(27i32);
    pub const TileWidePeekImage05: Self = Self(28i32);
    pub const TileWidePeekImage06: Self = Self(29i32);
    pub const TileWideSmallImageAndText01: Self = Self(30i32);
    pub const TileWideSmallImageAndText02: Self = Self(31i32);
    pub const TileWideSmallImageAndText03: Self = Self(32i32);
    pub const TileWideSmallImageAndText04: Self = Self(33i32);
    pub const TileWideSmallImageAndText05: Self = Self(34i32);
    pub const TileWideText01: Self = Self(35i32);
    pub const TileWideText02: Self = Self(36i32);
    pub const TileWideText03: Self = Self(37i32);
    pub const TileWideText04: Self = Self(38i32);
    pub const TileWideText05: Self = Self(39i32);
    pub const TileWideText06: Self = Self(40i32);
    pub const TileWideText07: Self = Self(41i32);
    pub const TileWideText08: Self = Self(42i32);
    pub const TileWideText09: Self = Self(43i32);
    pub const TileWideText10: Self = Self(44i32);
    pub const TileWideText11: Self = Self(45i32);
    pub const TileSquare150x150Image: Self = Self(0i32);
    pub const TileSquare150x150Block: Self = Self(1i32);
    pub const TileSquare150x150Text01: Self = Self(2i32);
    pub const TileSquare150x150Text02: Self = Self(3i32);
    pub const TileSquare150x150Text03: Self = Self(4i32);
    pub const TileSquare150x150Text04: Self = Self(5i32);
    pub const TileSquare150x150PeekImageAndText01: Self = Self(6i32);
    pub const TileSquare150x150PeekImageAndText02: Self = Self(7i32);
    pub const TileSquare150x150PeekImageAndText03: Self = Self(8i32);
    pub const TileSquare150x150PeekImageAndText04: Self = Self(9i32);
    pub const TileWide310x150Image: Self = Self(10i32);
    pub const TileWide310x150ImageCollection: Self = Self(11i32);
    pub const TileWide310x150ImageAndText01: Self = Self(12i32);
    pub const TileWide310x150ImageAndText02: Self = Self(13i32);
    pub const TileWide310x150BlockAndText01: Self = Self(14i32);
    pub const TileWide310x150BlockAndText02: Self = Self(15i32);
    pub const TileWide310x150PeekImageCollection01: Self = Self(16i32);
    pub const TileWide310x150PeekImageCollection02: Self = Self(17i32);
    pub const TileWide310x150PeekImageCollection03: Self = Self(18i32);
    pub const TileWide310x150PeekImageCollection04: Self = Self(19i32);
    pub const TileWide310x150PeekImageCollection05: Self = Self(20i32);
    pub const TileWide310x150PeekImageCollection06: Self = Self(21i32);
    pub const TileWide310x150PeekImageAndText01: Self = Self(22i32);
    pub const TileWide310x150PeekImageAndText02: Self = Self(23i32);
    pub const TileWide310x150PeekImage01: Self = Self(24i32);
    pub const TileWide310x150PeekImage02: Self = Self(25i32);
    pub const TileWide310x150PeekImage03: Self = Self(26i32);
    pub const TileWide310x150PeekImage04: Self = Self(27i32);
    pub const TileWide310x150PeekImage05: Self = Self(28i32);
    pub const TileWide310x150PeekImage06: Self = Self(29i32);
    pub const TileWide310x150SmallImageAndText01: Self = Self(30i32);
    pub const TileWide310x150SmallImageAndText02: Self = Self(31i32);
    pub const TileWide310x150SmallImageAndText03: Self = Self(32i32);
    pub const TileWide310x150SmallImageAndText04: Self = Self(33i32);
    pub const TileWide310x150SmallImageAndText05: Self = Self(34i32);
    pub const TileWide310x150Text01: Self = Self(35i32);
    pub const TileWide310x150Text02: Self = Self(36i32);
    pub const TileWide310x150Text03: Self = Self(37i32);
    pub const TileWide310x150Text04: Self = Self(38i32);
    pub const TileWide310x150Text05: Self = Self(39i32);
    pub const TileWide310x150Text06: Self = Self(40i32);
    pub const TileWide310x150Text07: Self = Self(41i32);
    pub const TileWide310x150Text08: Self = Self(42i32);
    pub const TileWide310x150Text09: Self = Self(43i32);
    pub const TileWide310x150Text10: Self = Self(44i32);
    pub const TileWide310x150Text11: Self = Self(45i32);
    pub const TileSquare310x310BlockAndText01: Self = Self(46i32);
    pub const TileSquare310x310BlockAndText02: Self = Self(47i32);
    pub const TileSquare310x310Image: Self = Self(48i32);
    pub const TileSquare310x310ImageAndText01: Self = Self(49i32);
    pub const TileSquare310x310ImageAndText02: Self = Self(50i32);
    pub const TileSquare310x310ImageAndTextOverlay01: Self = Self(51i32);
    pub const TileSquare310x310ImageAndTextOverlay02: Self = Self(52i32);
    pub const TileSquare310x310ImageAndTextOverlay03: Self = Self(53i32);
    pub const TileSquare310x310ImageCollectionAndText01: Self = Self(54i32);
    pub const TileSquare310x310ImageCollectionAndText02: Self = Self(55i32);
    pub const TileSquare310x310ImageCollection: Self = Self(56i32);
    pub const TileSquare310x310SmallImagesAndTextList01: Self = Self(57i32);
    pub const TileSquare310x310SmallImagesAndTextList02: Self = Self(58i32);
    pub const TileSquare310x310SmallImagesAndTextList03: Self = Self(59i32);
    pub const TileSquare310x310SmallImagesAndTextList04: Self = Self(60i32);
    pub const TileSquare310x310Text01: Self = Self(61i32);
    pub const TileSquare310x310Text02: Self = Self(62i32);
    pub const TileSquare310x310Text03: Self = Self(63i32);
    pub const TileSquare310x310Text04: Self = Self(64i32);
    pub const TileSquare310x310Text05: Self = Self(65i32);
    pub const TileSquare310x310Text06: Self = Self(66i32);
    pub const TileSquare310x310Text07: Self = Self(67i32);
    pub const TileSquare310x310Text08: Self = Self(68i32);
    pub const TileSquare310x310TextList01: Self = Self(69i32);
    pub const TileSquare310x310TextList02: Self = Self(70i32);
    pub const TileSquare310x310TextList03: Self = Self(71i32);
    pub const TileSquare310x310SmallImageAndText01: Self = Self(72i32);
    pub const TileSquare310x310SmallImagesAndTextList05: Self = Self(73i32);
    pub const TileSquare310x310Text09: Self = Self(74i32);
    pub const TileSquare71x71IconWithBadge: Self = Self(75i32);
    pub const TileSquare150x150IconWithBadge: Self = Self(76i32);
    pub const TileWide310x150IconWithBadgeAndText: Self = Self(77i32);
    pub const TileSquare71x71Image: Self = Self(78i32);
    pub const TileTall150x310Image: Self = Self(79i32);
}
impl ::core::marker::Copy for TileTemplateType {}
impl ::core::clone::Clone for TileTemplateType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TileTemplateType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for TileTemplateType {
    type Abi = Self;
}
impl ::core::fmt::Debug for TileTemplateType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TileTemplateType").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for TileTemplateType {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.UI.Notifications.TileTemplateType;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
pub struct TileUpdateManager;
impl TileUpdateManager {
    pub fn CreateTileUpdaterForApplication() -> ::windows_core::Result<TileUpdater> {
        Self::ITileUpdateManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateTileUpdaterForApplication)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<TileUpdater>(result__)
        })
    }
    pub fn CreateTileUpdaterForApplicationWithId<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(applicationid: Param0) -> ::windows_core::Result<TileUpdater> {
        Self::ITileUpdateManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateTileUpdaterForApplicationWithId)(::windows_core::Interface::as_raw(this), applicationid.into_param().abi(), result__.as_mut_ptr()).from_abi::<TileUpdater>(result__)
        })
    }
    pub fn CreateTileUpdaterForSecondaryTile<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(tileid: Param0) -> ::windows_core::Result<TileUpdater> {
        Self::ITileUpdateManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateTileUpdaterForSecondaryTile)(::windows_core::Interface::as_raw(this), tileid.into_param().abi(), result__.as_mut_ptr()).from_abi::<TileUpdater>(result__)
        })
    }
    #[cfg(feature = "Data_Xml_Dom")]
    pub fn GetTemplateContent(r#type: TileTemplateType) -> ::windows_core::Result<::winrt_data::Xml::Dom::XmlDocument> {
        Self::ITileUpdateManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetTemplateContent)(::windows_core::Interface::as_raw(this), r#type, result__.as_mut_ptr()).from_abi::<::winrt_data::Xml::Dom::XmlDocument>(result__)
        })
    }
    #[cfg(feature = "System")]
    pub fn GetForUser<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_system::User>>(user: Param0) -> ::windows_core::Result<TileUpdateManagerForUser> {
        Self::ITileUpdateManagerStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetForUser)(::windows_core::Interface::as_raw(this), user.into_param().abi(), result__.as_mut_ptr()).from_abi::<TileUpdateManagerForUser>(result__)
        })
    }
    pub fn ITileUpdateManagerStatics<R, F: FnOnce(&ITileUpdateManagerStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<TileUpdateManager, ITileUpdateManagerStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn ITileUpdateManagerStatics2<R, F: FnOnce(&ITileUpdateManagerStatics2) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<TileUpdateManager, ITileUpdateManagerStatics2> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows_core::RuntimeName for TileUpdateManager {
    const NAME: &'static str = "Windows.UI.Notifications.TileUpdateManager";
}
#[repr(transparent)]
pub struct TileUpdateManagerForUser(::windows_core::IUnknown);
impl TileUpdateManagerForUser {
    pub fn CreateTileUpdaterForApplication(&self) -> ::windows_core::Result<TileUpdater> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateTileUpdaterForApplication)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<TileUpdater>(result__)
        }
    }
    pub fn CreateTileUpdaterForApplicationWithId<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, applicationid: Param0) -> ::windows_core::Result<TileUpdater> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateTileUpdaterForApplicationWithId)(::windows_core::Interface::as_raw(this), applicationid.into_param().abi(), result__.as_mut_ptr()).from_abi::<TileUpdater>(result__)
        }
    }
    pub fn CreateTileUpdaterForSecondaryTile<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, tileid: Param0) -> ::windows_core::Result<TileUpdater> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateTileUpdaterForSecondaryTile)(::windows_core::Interface::as_raw(this), tileid.into_param().abi(), result__.as_mut_ptr()).from_abi::<TileUpdater>(result__)
        }
    }
    #[cfg(feature = "System")]
    pub fn User(&self) -> ::windows_core::Result<::winrt_system::User> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).User)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_system::User>(result__)
        }
    }
}
impl ::core::clone::Clone for TileUpdateManagerForUser {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for TileUpdateManagerForUser {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for TileUpdateManagerForUser {}
impl ::core::fmt::Debug for TileUpdateManagerForUser {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TileUpdateManagerForUser").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for TileUpdateManagerForUser {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Notifications.TileUpdateManagerForUser;{55141348-2ee2-4e2d-9cc1-216a20decc9f})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for TileUpdateManagerForUser {
    type Vtable = ITileUpdateManagerForUser_Vtbl;
    const IID: ::windows_core::GUID = <ITileUpdateManagerForUser as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for TileUpdateManagerForUser {
    const NAME: &'static str = "Windows.UI.Notifications.TileUpdateManagerForUser";
}
impl ::core::convert::From<TileUpdateManagerForUser> for ::windows_core::IUnknown {
    fn from(value: TileUpdateManagerForUser) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&TileUpdateManagerForUser> for ::windows_core::IUnknown {
    fn from(value: &TileUpdateManagerForUser) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for TileUpdateManagerForUser {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a TileUpdateManagerForUser {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<TileUpdateManagerForUser> for ::windows_core::IInspectable {
    fn from(value: TileUpdateManagerForUser) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&TileUpdateManagerForUser> for ::windows_core::IInspectable {
    fn from(value: &TileUpdateManagerForUser) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for TileUpdateManagerForUser {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a TileUpdateManagerForUser {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for TileUpdateManagerForUser {}
unsafe impl ::core::marker::Sync for TileUpdateManagerForUser {}
#[repr(transparent)]
pub struct TileUpdater(::windows_core::IUnknown);
impl TileUpdater {
    pub fn Update<'a, Param0: ::windows_core::IntoParam<'a, TileNotification>>(&self, notification: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Update)(::windows_core::Interface::as_raw(this), notification.into_param().abi()).ok() }
    }
    pub fn Clear(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Clear)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn EnableNotificationQueue(&self, enable: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).EnableNotificationQueue)(::windows_core::Interface::as_raw(this), enable).ok() }
    }
    pub fn Setting(&self) -> ::windows_core::Result<NotificationSetting> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<NotificationSetting>::zeroed();
            (::windows_core::Interface::vtable(this).Setting)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<NotificationSetting>(result__)
        }
    }
    pub fn AddToSchedule<'a, Param0: ::windows_core::IntoParam<'a, ScheduledTileNotification>>(&self, scheduledtile: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).AddToSchedule)(::windows_core::Interface::as_raw(this), scheduledtile.into_param().abi()).ok() }
    }
    pub fn RemoveFromSchedule<'a, Param0: ::windows_core::IntoParam<'a, ScheduledTileNotification>>(&self, scheduledtile: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveFromSchedule)(::windows_core::Interface::as_raw(this), scheduledtile.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetScheduledTileNotifications(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<ScheduledTileNotification>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetScheduledTileNotifications)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<ScheduledTileNotification>>(result__)
        }
    }
    pub fn StartPeriodicUpdate<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Uri>>(&self, tilecontent: Param0, requestedinterval: PeriodicUpdateRecurrence) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).StartPeriodicUpdate)(::windows_core::Interface::as_raw(this), tilecontent.into_param().abi(), requestedinterval).ok() }
    }
    pub fn StartPeriodicUpdateAtTime<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Uri>, Param1: ::windows_core::IntoParam<'a, ::winrt_foundation::DateTime>>(&self, tilecontent: Param0, starttime: Param1, requestedinterval: PeriodicUpdateRecurrence) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).StartPeriodicUpdateAtTime)(::windows_core::Interface::as_raw(this), tilecontent.into_param().abi(), starttime.into_param().abi(), requestedinterval).ok() }
    }
    pub fn StopPeriodicUpdate(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).StopPeriodicUpdate)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn StartPeriodicUpdateBatch<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<::winrt_foundation::Uri>>>(&self, tilecontents: Param0, requestedinterval: PeriodicUpdateRecurrence) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).StartPeriodicUpdateBatch)(::windows_core::Interface::as_raw(this), tilecontents.into_param().abi(), requestedinterval).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn StartPeriodicUpdateBatchAtTime<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<::winrt_foundation::Uri>>, Param1: ::windows_core::IntoParam<'a, ::winrt_foundation::DateTime>>(&self, tilecontents: Param0, starttime: Param1, requestedinterval: PeriodicUpdateRecurrence) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).StartPeriodicUpdateBatchAtTime)(::windows_core::Interface::as_raw(this), tilecontents.into_param().abi(), starttime.into_param().abi(), requestedinterval).ok() }
    }
    pub fn EnableNotificationQueueForSquare150x150(&self, enable: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<ITileUpdater2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).EnableNotificationQueueForSquare150x150)(::windows_core::Interface::as_raw(this), enable).ok() }
    }
    pub fn EnableNotificationQueueForWide310x150(&self, enable: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<ITileUpdater2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).EnableNotificationQueueForWide310x150)(::windows_core::Interface::as_raw(this), enable).ok() }
    }
    pub fn EnableNotificationQueueForSquare310x310(&self, enable: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<ITileUpdater2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).EnableNotificationQueueForSquare310x310)(::windows_core::Interface::as_raw(this), enable).ok() }
    }
}
impl ::core::clone::Clone for TileUpdater {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for TileUpdater {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for TileUpdater {}
impl ::core::fmt::Debug for TileUpdater {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TileUpdater").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for TileUpdater {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Notifications.TileUpdater;{0942a48b-1d91-44ec-9243-c1e821c29a20})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for TileUpdater {
    type Vtable = ITileUpdater_Vtbl;
    const IID: ::windows_core::GUID = <ITileUpdater as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for TileUpdater {
    const NAME: &'static str = "Windows.UI.Notifications.TileUpdater";
}
impl ::core::convert::From<TileUpdater> for ::windows_core::IUnknown {
    fn from(value: TileUpdater) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&TileUpdater> for ::windows_core::IUnknown {
    fn from(value: &TileUpdater) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for TileUpdater {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a TileUpdater {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<TileUpdater> for ::windows_core::IInspectable {
    fn from(value: TileUpdater) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&TileUpdater> for ::windows_core::IInspectable {
    fn from(value: &TileUpdater) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for TileUpdater {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a TileUpdater {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for TileUpdater {}
unsafe impl ::core::marker::Sync for TileUpdater {}
#[repr(transparent)]
pub struct ToastActivatedEventArgs(::windows_core::IUnknown);
impl ToastActivatedEventArgs {
    pub fn Arguments(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Arguments)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn UserInput(&self) -> ::windows_core::Result<::winrt_foundation::Collections::ValueSet> {
        let this = &::windows_core::Interface::cast::<IToastActivatedEventArgs2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).UserInput)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::ValueSet>(result__)
        }
    }
}
impl ::core::clone::Clone for ToastActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ToastActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ToastActivatedEventArgs {}
impl ::core::fmt::Debug for ToastActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ToastActivatedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ToastActivatedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Notifications.ToastActivatedEventArgs;{e3bf92f3-c197-436f-8265-0625824f8dac})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for ToastActivatedEventArgs {
    type Vtable = IToastActivatedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IToastActivatedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for ToastActivatedEventArgs {
    const NAME: &'static str = "Windows.UI.Notifications.ToastActivatedEventArgs";
}
impl ::core::convert::From<ToastActivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: ToastActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ToastActivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &ToastActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ToastActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ToastActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ToastActivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: ToastActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ToastActivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &ToastActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for ToastActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a ToastActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(transparent)]
pub struct ToastCollection(::windows_core::IUnknown);
impl ToastCollection {
    pub fn Id(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Id)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn DisplayName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).DisplayName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetDisplayName<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetDisplayName)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn LaunchArgs(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).LaunchArgs)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetLaunchArgs<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetLaunchArgs)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn Icon(&self) -> ::windows_core::Result<::winrt_foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Icon)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Uri>(result__)
        }
    }
    pub fn SetIcon<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Uri>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetIcon)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn CreateInstance<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param2: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param3: ::windows_core::IntoParam<'a, ::winrt_foundation::Uri>>(collectionid: Param0, displayname: Param1, launchargs: Param2, iconuri: Param3) -> ::windows_core::Result<ToastCollection> {
        Self::IToastCollectionFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateInstance)(::windows_core::Interface::as_raw(this), collectionid.into_param().abi(), displayname.into_param().abi(), launchargs.into_param().abi(), iconuri.into_param().abi(), result__.as_mut_ptr()).from_abi::<ToastCollection>(result__)
        })
    }
    pub fn IToastCollectionFactory<R, F: FnOnce(&IToastCollectionFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<ToastCollection, IToastCollectionFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for ToastCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ToastCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ToastCollection {}
impl ::core::fmt::Debug for ToastCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ToastCollection").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ToastCollection {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Notifications.ToastCollection;{0a8bc3b0-e0be-4858-bc2a-89dfe0b32863})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for ToastCollection {
    type Vtable = IToastCollection_Vtbl;
    const IID: ::windows_core::GUID = <IToastCollection as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for ToastCollection {
    const NAME: &'static str = "Windows.UI.Notifications.ToastCollection";
}
impl ::core::convert::From<ToastCollection> for ::windows_core::IUnknown {
    fn from(value: ToastCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ToastCollection> for ::windows_core::IUnknown {
    fn from(value: &ToastCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ToastCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ToastCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ToastCollection> for ::windows_core::IInspectable {
    fn from(value: ToastCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ToastCollection> for ::windows_core::IInspectable {
    fn from(value: &ToastCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for ToastCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a ToastCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for ToastCollection {}
unsafe impl ::core::marker::Sync for ToastCollection {}
#[repr(transparent)]
pub struct ToastCollectionManager(::windows_core::IUnknown);
impl ToastCollectionManager {
    pub fn SaveToastCollectionAsync<'a, Param0: ::windows_core::IntoParam<'a, ToastCollection>>(&self, collection: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SaveToastCollectionAsync)(::windows_core::Interface::as_raw(this), collection.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn FindAllToastCollectionsAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<::winrt_foundation::Collections::IVectorView<ToastCollection>>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FindAllToastCollectionsAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<::winrt_foundation::Collections::IVectorView<ToastCollection>>>(result__)
        }
    }
    pub fn GetToastCollectionAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, collectionid: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<ToastCollection>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetToastCollectionAsync)(::windows_core::Interface::as_raw(this), collectionid.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<ToastCollection>>(result__)
        }
    }
    pub fn RemoveToastCollectionAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, collectionid: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RemoveToastCollectionAsync)(::windows_core::Interface::as_raw(this), collectionid.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncAction>(result__)
        }
    }
    pub fn RemoveAllToastCollectionsAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RemoveAllToastCollectionsAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "System")]
    pub fn User(&self) -> ::windows_core::Result<::winrt_system::User> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).User)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_system::User>(result__)
        }
    }
    pub fn AppId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).AppId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
}
impl ::core::clone::Clone for ToastCollectionManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ToastCollectionManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ToastCollectionManager {}
impl ::core::fmt::Debug for ToastCollectionManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ToastCollectionManager").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ToastCollectionManager {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Notifications.ToastCollectionManager;{2a1821fe-179d-49bc-b79d-a527920d3665})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for ToastCollectionManager {
    type Vtable = IToastCollectionManager_Vtbl;
    const IID: ::windows_core::GUID = <IToastCollectionManager as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for ToastCollectionManager {
    const NAME: &'static str = "Windows.UI.Notifications.ToastCollectionManager";
}
impl ::core::convert::From<ToastCollectionManager> for ::windows_core::IUnknown {
    fn from(value: ToastCollectionManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ToastCollectionManager> for ::windows_core::IUnknown {
    fn from(value: &ToastCollectionManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ToastCollectionManager {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ToastCollectionManager {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ToastCollectionManager> for ::windows_core::IInspectable {
    fn from(value: ToastCollectionManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ToastCollectionManager> for ::windows_core::IInspectable {
    fn from(value: &ToastCollectionManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for ToastCollectionManager {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a ToastCollectionManager {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for ToastCollectionManager {}
unsafe impl ::core::marker::Sync for ToastCollectionManager {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct ToastDismissalReason(pub i32);
impl ToastDismissalReason {
    pub const UserCanceled: Self = Self(0i32);
    pub const ApplicationHidden: Self = Self(1i32);
    pub const TimedOut: Self = Self(2i32);
}
impl ::core::marker::Copy for ToastDismissalReason {}
impl ::core::clone::Clone for ToastDismissalReason {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ToastDismissalReason {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for ToastDismissalReason {
    type Abi = Self;
}
impl ::core::fmt::Debug for ToastDismissalReason {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ToastDismissalReason").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ToastDismissalReason {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.UI.Notifications.ToastDismissalReason;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct ToastDismissedEventArgs(::windows_core::IUnknown);
impl ToastDismissedEventArgs {
    pub fn Reason(&self) -> ::windows_core::Result<ToastDismissalReason> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ToastDismissalReason>::zeroed();
            (::windows_core::Interface::vtable(this).Reason)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ToastDismissalReason>(result__)
        }
    }
}
impl ::core::clone::Clone for ToastDismissedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ToastDismissedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ToastDismissedEventArgs {}
impl ::core::fmt::Debug for ToastDismissedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ToastDismissedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ToastDismissedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Notifications.ToastDismissedEventArgs;{3f89d935-d9cb-4538-a0f0-ffe7659938f8})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for ToastDismissedEventArgs {
    type Vtable = IToastDismissedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IToastDismissedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for ToastDismissedEventArgs {
    const NAME: &'static str = "Windows.UI.Notifications.ToastDismissedEventArgs";
}
impl ::core::convert::From<ToastDismissedEventArgs> for ::windows_core::IUnknown {
    fn from(value: ToastDismissedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ToastDismissedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &ToastDismissedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ToastDismissedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ToastDismissedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ToastDismissedEventArgs> for ::windows_core::IInspectable {
    fn from(value: ToastDismissedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ToastDismissedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &ToastDismissedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for ToastDismissedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a ToastDismissedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for ToastDismissedEventArgs {}
unsafe impl ::core::marker::Sync for ToastDismissedEventArgs {}
#[repr(transparent)]
pub struct ToastFailedEventArgs(::windows_core::IUnknown);
impl ToastFailedEventArgs {
    pub fn ErrorCode(&self) -> ::windows_core::Result<::windows_core::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::HRESULT>::zeroed();
            (::windows_core::Interface::vtable(this).ErrorCode)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HRESULT>(result__)
        }
    }
}
impl ::core::clone::Clone for ToastFailedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ToastFailedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ToastFailedEventArgs {}
impl ::core::fmt::Debug for ToastFailedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ToastFailedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ToastFailedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Notifications.ToastFailedEventArgs;{35176862-cfd4-44f8-ad64-f500fd896c3b})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for ToastFailedEventArgs {
    type Vtable = IToastFailedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IToastFailedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for ToastFailedEventArgs {
    const NAME: &'static str = "Windows.UI.Notifications.ToastFailedEventArgs";
}
impl ::core::convert::From<ToastFailedEventArgs> for ::windows_core::IUnknown {
    fn from(value: ToastFailedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ToastFailedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &ToastFailedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ToastFailedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ToastFailedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ToastFailedEventArgs> for ::windows_core::IInspectable {
    fn from(value: ToastFailedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ToastFailedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &ToastFailedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for ToastFailedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a ToastFailedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for ToastFailedEventArgs {}
unsafe impl ::core::marker::Sync for ToastFailedEventArgs {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct ToastHistoryChangedType(pub i32);
impl ToastHistoryChangedType {
    pub const Cleared: Self = Self(0i32);
    pub const Removed: Self = Self(1i32);
    pub const Expired: Self = Self(2i32);
    pub const Added: Self = Self(3i32);
}
impl ::core::marker::Copy for ToastHistoryChangedType {}
impl ::core::clone::Clone for ToastHistoryChangedType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ToastHistoryChangedType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for ToastHistoryChangedType {
    type Abi = Self;
}
impl ::core::fmt::Debug for ToastHistoryChangedType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ToastHistoryChangedType").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ToastHistoryChangedType {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.UI.Notifications.ToastHistoryChangedType;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct ToastNotification(::windows_core::IUnknown);
impl ToastNotification {
    #[cfg(feature = "Data_Xml_Dom")]
    pub fn Content(&self) -> ::windows_core::Result<::winrt_data::Xml::Dom::XmlDocument> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Content)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_data::Xml::Dom::XmlDocument>(result__)
        }
    }
    pub fn SetExpirationTime<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::IReference<::winrt_foundation::DateTime>>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetExpirationTime)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn ExpirationTime(&self) -> ::windows_core::Result<::winrt_foundation::IReference<::winrt_foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ExpirationTime)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IReference<::winrt_foundation::DateTime>>(result__)
        }
    }
    pub fn Dismissed<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<ToastNotification, ToastDismissedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).Dismissed)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveDismissed<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveDismissed)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn Activated<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<ToastNotification, ::windows_core::IInspectable>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).Activated)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveActivated<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveActivated)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn Failed<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<ToastNotification, ToastFailedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).Failed)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveFailed<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveFailed)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn SetTag<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IToastNotification2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetTag)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn Tag(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IToastNotification2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Tag)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetGroup<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IToastNotification2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetGroup)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn Group(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IToastNotification2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Group)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetSuppressPopup(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IToastNotification2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetSuppressPopup)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn SuppressPopup(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IToastNotification2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).SuppressPopup)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn NotificationMirroring(&self) -> ::windows_core::Result<NotificationMirroring> {
        let this = &::windows_core::Interface::cast::<IToastNotification3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<NotificationMirroring>::zeroed();
            (::windows_core::Interface::vtable(this).NotificationMirroring)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<NotificationMirroring>(result__)
        }
    }
    pub fn SetNotificationMirroring(&self, value: NotificationMirroring) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IToastNotification3>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetNotificationMirroring)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn RemoteId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IToastNotification3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).RemoteId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetRemoteId<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IToastNotification3>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetRemoteId)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn Data(&self) -> ::windows_core::Result<NotificationData> {
        let this = &::windows_core::Interface::cast::<IToastNotification4>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Data)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<NotificationData>(result__)
        }
    }
    pub fn SetData<'a, Param0: ::windows_core::IntoParam<'a, NotificationData>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IToastNotification4>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetData)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn Priority(&self) -> ::windows_core::Result<ToastNotificationPriority> {
        let this = &::windows_core::Interface::cast::<IToastNotification4>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ToastNotificationPriority>::zeroed();
            (::windows_core::Interface::vtable(this).Priority)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ToastNotificationPriority>(result__)
        }
    }
    pub fn SetPriority(&self, value: ToastNotificationPriority) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IToastNotification4>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetPriority)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ExpiresOnReboot(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IToastNotification6>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).ExpiresOnReboot)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetExpiresOnReboot(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IToastNotification6>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetExpiresOnReboot)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "Data_Xml_Dom")]
    pub fn CreateToastNotification<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_data::Xml::Dom::XmlDocument>>(content: Param0) -> ::windows_core::Result<ToastNotification> {
        Self::IToastNotificationFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateToastNotification)(::windows_core::Interface::as_raw(this), content.into_param().abi(), result__.as_mut_ptr()).from_abi::<ToastNotification>(result__)
        })
    }
    pub fn IToastNotificationFactory<R, F: FnOnce(&IToastNotificationFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<ToastNotification, IToastNotificationFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for ToastNotification {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ToastNotification {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ToastNotification {}
impl ::core::fmt::Debug for ToastNotification {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ToastNotification").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ToastNotification {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Notifications.ToastNotification;{997e2675-059e-4e60-8b06-1760917c8b80})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for ToastNotification {
    type Vtable = IToastNotification_Vtbl;
    const IID: ::windows_core::GUID = <IToastNotification as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for ToastNotification {
    const NAME: &'static str = "Windows.UI.Notifications.ToastNotification";
}
impl ::core::convert::From<ToastNotification> for ::windows_core::IUnknown {
    fn from(value: ToastNotification) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ToastNotification> for ::windows_core::IUnknown {
    fn from(value: &ToastNotification) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ToastNotification {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ToastNotification {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ToastNotification> for ::windows_core::IInspectable {
    fn from(value: ToastNotification) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ToastNotification> for ::windows_core::IInspectable {
    fn from(value: &ToastNotification) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for ToastNotification {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a ToastNotification {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for ToastNotification {}
unsafe impl ::core::marker::Sync for ToastNotification {}
#[repr(transparent)]
pub struct ToastNotificationActionTriggerDetail(::windows_core::IUnknown);
impl ToastNotificationActionTriggerDetail {
    pub fn Argument(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Argument)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn UserInput(&self) -> ::windows_core::Result<::winrt_foundation::Collections::ValueSet> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).UserInput)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::ValueSet>(result__)
        }
    }
}
impl ::core::clone::Clone for ToastNotificationActionTriggerDetail {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ToastNotificationActionTriggerDetail {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ToastNotificationActionTriggerDetail {}
impl ::core::fmt::Debug for ToastNotificationActionTriggerDetail {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ToastNotificationActionTriggerDetail").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ToastNotificationActionTriggerDetail {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Notifications.ToastNotificationActionTriggerDetail;{9445135a-38f3-42f6-96aa-7955b0f03da2})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for ToastNotificationActionTriggerDetail {
    type Vtable = IToastNotificationActionTriggerDetail_Vtbl;
    const IID: ::windows_core::GUID = <IToastNotificationActionTriggerDetail as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for ToastNotificationActionTriggerDetail {
    const NAME: &'static str = "Windows.UI.Notifications.ToastNotificationActionTriggerDetail";
}
impl ::core::convert::From<ToastNotificationActionTriggerDetail> for ::windows_core::IUnknown {
    fn from(value: ToastNotificationActionTriggerDetail) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ToastNotificationActionTriggerDetail> for ::windows_core::IUnknown {
    fn from(value: &ToastNotificationActionTriggerDetail) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ToastNotificationActionTriggerDetail {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ToastNotificationActionTriggerDetail {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ToastNotificationActionTriggerDetail> for ::windows_core::IInspectable {
    fn from(value: ToastNotificationActionTriggerDetail) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ToastNotificationActionTriggerDetail> for ::windows_core::IInspectable {
    fn from(value: &ToastNotificationActionTriggerDetail) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for ToastNotificationActionTriggerDetail {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a ToastNotificationActionTriggerDetail {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(transparent)]
pub struct ToastNotificationHistory(::windows_core::IUnknown);
impl ToastNotificationHistory {
    pub fn RemoveGroup<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, group: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveGroup)(::windows_core::Interface::as_raw(this), group.into_param().abi()).ok() }
    }
    pub fn RemoveGroupWithId<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, group: Param0, applicationid: Param1) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveGroupWithId)(::windows_core::Interface::as_raw(this), group.into_param().abi(), applicationid.into_param().abi()).ok() }
    }
    pub fn RemoveGroupedTagWithId<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param2: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, tag: Param0, group: Param1, applicationid: Param2) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveGroupedTagWithId)(::windows_core::Interface::as_raw(this), tag.into_param().abi(), group.into_param().abi(), applicationid.into_param().abi()).ok() }
    }
    pub fn RemoveGroupedTag<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, tag: Param0, group: Param1) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveGroupedTag)(::windows_core::Interface::as_raw(this), tag.into_param().abi(), group.into_param().abi()).ok() }
    }
    pub fn Remove<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, tag: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Remove)(::windows_core::Interface::as_raw(this), tag.into_param().abi()).ok() }
    }
    pub fn Clear(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Clear)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn ClearWithId<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, applicationid: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).ClearWithId)(::windows_core::Interface::as_raw(this), applicationid.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetHistory(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<ToastNotification>> {
        let this = &::windows_core::Interface::cast::<IToastNotificationHistory2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetHistory)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<ToastNotification>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetHistoryWithId<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, applicationid: Param0) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<ToastNotification>> {
        let this = &::windows_core::Interface::cast::<IToastNotificationHistory2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetHistoryWithId)(::windows_core::Interface::as_raw(this), applicationid.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<ToastNotification>>(result__)
        }
    }
}
impl ::core::clone::Clone for ToastNotificationHistory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ToastNotificationHistory {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ToastNotificationHistory {}
impl ::core::fmt::Debug for ToastNotificationHistory {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ToastNotificationHistory").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ToastNotificationHistory {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Notifications.ToastNotificationHistory;{5caddc63-01d3-4c97-986f-0533483fee14})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for ToastNotificationHistory {
    type Vtable = IToastNotificationHistory_Vtbl;
    const IID: ::windows_core::GUID = <IToastNotificationHistory as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for ToastNotificationHistory {
    const NAME: &'static str = "Windows.UI.Notifications.ToastNotificationHistory";
}
impl ::core::convert::From<ToastNotificationHistory> for ::windows_core::IUnknown {
    fn from(value: ToastNotificationHistory) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ToastNotificationHistory> for ::windows_core::IUnknown {
    fn from(value: &ToastNotificationHistory) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ToastNotificationHistory {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ToastNotificationHistory {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ToastNotificationHistory> for ::windows_core::IInspectable {
    fn from(value: ToastNotificationHistory) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ToastNotificationHistory> for ::windows_core::IInspectable {
    fn from(value: &ToastNotificationHistory) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for ToastNotificationHistory {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a ToastNotificationHistory {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(transparent)]
pub struct ToastNotificationHistoryChangedTriggerDetail(::windows_core::IUnknown);
impl ToastNotificationHistoryChangedTriggerDetail {
    pub fn ChangeType(&self) -> ::windows_core::Result<ToastHistoryChangedType> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ToastHistoryChangedType>::zeroed();
            (::windows_core::Interface::vtable(this).ChangeType)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ToastHistoryChangedType>(result__)
        }
    }
    pub fn CollectionId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IToastNotificationHistoryChangedTriggerDetail2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).CollectionId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
}
impl ::core::clone::Clone for ToastNotificationHistoryChangedTriggerDetail {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ToastNotificationHistoryChangedTriggerDetail {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ToastNotificationHistoryChangedTriggerDetail {}
impl ::core::fmt::Debug for ToastNotificationHistoryChangedTriggerDetail {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ToastNotificationHistoryChangedTriggerDetail").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ToastNotificationHistoryChangedTriggerDetail {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Notifications.ToastNotificationHistoryChangedTriggerDetail;{db037ffa-0068-412c-9c83-267c37f65670})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for ToastNotificationHistoryChangedTriggerDetail {
    type Vtable = IToastNotificationHistoryChangedTriggerDetail_Vtbl;
    const IID: ::windows_core::GUID = <IToastNotificationHistoryChangedTriggerDetail as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for ToastNotificationHistoryChangedTriggerDetail {
    const NAME: &'static str = "Windows.UI.Notifications.ToastNotificationHistoryChangedTriggerDetail";
}
impl ::core::convert::From<ToastNotificationHistoryChangedTriggerDetail> for ::windows_core::IUnknown {
    fn from(value: ToastNotificationHistoryChangedTriggerDetail) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ToastNotificationHistoryChangedTriggerDetail> for ::windows_core::IUnknown {
    fn from(value: &ToastNotificationHistoryChangedTriggerDetail) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ToastNotificationHistoryChangedTriggerDetail {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ToastNotificationHistoryChangedTriggerDetail {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ToastNotificationHistoryChangedTriggerDetail> for ::windows_core::IInspectable {
    fn from(value: ToastNotificationHistoryChangedTriggerDetail) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ToastNotificationHistoryChangedTriggerDetail> for ::windows_core::IInspectable {
    fn from(value: &ToastNotificationHistoryChangedTriggerDetail) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for ToastNotificationHistoryChangedTriggerDetail {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a ToastNotificationHistoryChangedTriggerDetail {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
pub struct ToastNotificationManager;
impl ToastNotificationManager {
    pub fn CreateToastNotifier() -> ::windows_core::Result<ToastNotifier> {
        Self::IToastNotificationManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateToastNotifier)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ToastNotifier>(result__)
        })
    }
    pub fn CreateToastNotifierWithId<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(applicationid: Param0) -> ::windows_core::Result<ToastNotifier> {
        Self::IToastNotificationManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateToastNotifierWithId)(::windows_core::Interface::as_raw(this), applicationid.into_param().abi(), result__.as_mut_ptr()).from_abi::<ToastNotifier>(result__)
        })
    }
    #[cfg(feature = "Data_Xml_Dom")]
    pub fn GetTemplateContent(r#type: ToastTemplateType) -> ::windows_core::Result<::winrt_data::Xml::Dom::XmlDocument> {
        Self::IToastNotificationManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetTemplateContent)(::windows_core::Interface::as_raw(this), r#type, result__.as_mut_ptr()).from_abi::<::winrt_data::Xml::Dom::XmlDocument>(result__)
        })
    }
    pub fn History() -> ::windows_core::Result<ToastNotificationHistory> {
        Self::IToastNotificationManagerStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).History)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ToastNotificationHistory>(result__)
        })
    }
    #[cfg(feature = "System")]
    pub fn GetForUser<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_system::User>>(user: Param0) -> ::windows_core::Result<ToastNotificationManagerForUser> {
        Self::IToastNotificationManagerStatics4(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetForUser)(::windows_core::Interface::as_raw(this), user.into_param().abi(), result__.as_mut_ptr()).from_abi::<ToastNotificationManagerForUser>(result__)
        })
    }
    pub fn ConfigureNotificationMirroring(value: NotificationMirroring) -> ::windows_core::Result<()> {
        Self::IToastNotificationManagerStatics4(|this| unsafe { (::windows_core::Interface::vtable(this).ConfigureNotificationMirroring)(::windows_core::Interface::as_raw(this), value).ok() })
    }
    pub fn GetDefault() -> ::windows_core::Result<ToastNotificationManagerForUser> {
        Self::IToastNotificationManagerStatics5(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetDefault)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ToastNotificationManagerForUser>(result__)
        })
    }
    pub fn IToastNotificationManagerStatics<R, F: FnOnce(&IToastNotificationManagerStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<ToastNotificationManager, IToastNotificationManagerStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IToastNotificationManagerStatics2<R, F: FnOnce(&IToastNotificationManagerStatics2) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<ToastNotificationManager, IToastNotificationManagerStatics2> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IToastNotificationManagerStatics4<R, F: FnOnce(&IToastNotificationManagerStatics4) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<ToastNotificationManager, IToastNotificationManagerStatics4> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IToastNotificationManagerStatics5<R, F: FnOnce(&IToastNotificationManagerStatics5) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<ToastNotificationManager, IToastNotificationManagerStatics5> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows_core::RuntimeName for ToastNotificationManager {
    const NAME: &'static str = "Windows.UI.Notifications.ToastNotificationManager";
}
#[repr(transparent)]
pub struct ToastNotificationManagerForUser(::windows_core::IUnknown);
impl ToastNotificationManagerForUser {
    pub fn CreateToastNotifier(&self) -> ::windows_core::Result<ToastNotifier> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateToastNotifier)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ToastNotifier>(result__)
        }
    }
    pub fn CreateToastNotifierWithId<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, applicationid: Param0) -> ::windows_core::Result<ToastNotifier> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateToastNotifierWithId)(::windows_core::Interface::as_raw(this), applicationid.into_param().abi(), result__.as_mut_ptr()).from_abi::<ToastNotifier>(result__)
        }
    }
    pub fn History(&self) -> ::windows_core::Result<ToastNotificationHistory> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).History)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ToastNotificationHistory>(result__)
        }
    }
    #[cfg(feature = "System")]
    pub fn User(&self) -> ::windows_core::Result<::winrt_system::User> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).User)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_system::User>(result__)
        }
    }
    pub fn GetToastNotifierForToastCollectionIdAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, collectionid: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<ToastNotifier>> {
        let this = &::windows_core::Interface::cast::<IToastNotificationManagerForUser2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetToastNotifierForToastCollectionIdAsync)(::windows_core::Interface::as_raw(this), collectionid.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<ToastNotifier>>(result__)
        }
    }
    pub fn GetHistoryForToastCollectionIdAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, collectionid: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<ToastNotificationHistory>> {
        let this = &::windows_core::Interface::cast::<IToastNotificationManagerForUser2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetHistoryForToastCollectionIdAsync)(::windows_core::Interface::as_raw(this), collectionid.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<ToastNotificationHistory>>(result__)
        }
    }
    pub fn GetToastCollectionManager(&self) -> ::windows_core::Result<ToastCollectionManager> {
        let this = &::windows_core::Interface::cast::<IToastNotificationManagerForUser2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetToastCollectionManager)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ToastCollectionManager>(result__)
        }
    }
    pub fn GetToastCollectionManagerWithAppId<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, appid: Param0) -> ::windows_core::Result<ToastCollectionManager> {
        let this = &::windows_core::Interface::cast::<IToastNotificationManagerForUser2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetToastCollectionManagerWithAppId)(::windows_core::Interface::as_raw(this), appid.into_param().abi(), result__.as_mut_ptr()).from_abi::<ToastCollectionManager>(result__)
        }
    }
}
impl ::core::clone::Clone for ToastNotificationManagerForUser {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ToastNotificationManagerForUser {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ToastNotificationManagerForUser {}
impl ::core::fmt::Debug for ToastNotificationManagerForUser {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ToastNotificationManagerForUser").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ToastNotificationManagerForUser {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Notifications.ToastNotificationManagerForUser;{79ab57f6-43fe-487b-8a7f-99567200ae94})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for ToastNotificationManagerForUser {
    type Vtable = IToastNotificationManagerForUser_Vtbl;
    const IID: ::windows_core::GUID = <IToastNotificationManagerForUser as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for ToastNotificationManagerForUser {
    const NAME: &'static str = "Windows.UI.Notifications.ToastNotificationManagerForUser";
}
impl ::core::convert::From<ToastNotificationManagerForUser> for ::windows_core::IUnknown {
    fn from(value: ToastNotificationManagerForUser) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ToastNotificationManagerForUser> for ::windows_core::IUnknown {
    fn from(value: &ToastNotificationManagerForUser) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ToastNotificationManagerForUser {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ToastNotificationManagerForUser {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ToastNotificationManagerForUser> for ::windows_core::IInspectable {
    fn from(value: ToastNotificationManagerForUser) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ToastNotificationManagerForUser> for ::windows_core::IInspectable {
    fn from(value: &ToastNotificationManagerForUser) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for ToastNotificationManagerForUser {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a ToastNotificationManagerForUser {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for ToastNotificationManagerForUser {}
unsafe impl ::core::marker::Sync for ToastNotificationManagerForUser {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct ToastNotificationPriority(pub i32);
impl ToastNotificationPriority {
    pub const Default: Self = Self(0i32);
    pub const High: Self = Self(1i32);
}
impl ::core::marker::Copy for ToastNotificationPriority {}
impl ::core::clone::Clone for ToastNotificationPriority {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ToastNotificationPriority {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for ToastNotificationPriority {
    type Abi = Self;
}
impl ::core::fmt::Debug for ToastNotificationPriority {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ToastNotificationPriority").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ToastNotificationPriority {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.UI.Notifications.ToastNotificationPriority;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct ToastNotifier(::windows_core::IUnknown);
impl ToastNotifier {
    pub fn Show<'a, Param0: ::windows_core::IntoParam<'a, ToastNotification>>(&self, notification: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Show)(::windows_core::Interface::as_raw(this), notification.into_param().abi()).ok() }
    }
    pub fn Hide<'a, Param0: ::windows_core::IntoParam<'a, ToastNotification>>(&self, notification: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Hide)(::windows_core::Interface::as_raw(this), notification.into_param().abi()).ok() }
    }
    pub fn Setting(&self) -> ::windows_core::Result<NotificationSetting> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<NotificationSetting>::zeroed();
            (::windows_core::Interface::vtable(this).Setting)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<NotificationSetting>(result__)
        }
    }
    pub fn AddToSchedule<'a, Param0: ::windows_core::IntoParam<'a, ScheduledToastNotification>>(&self, scheduledtoast: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).AddToSchedule)(::windows_core::Interface::as_raw(this), scheduledtoast.into_param().abi()).ok() }
    }
    pub fn RemoveFromSchedule<'a, Param0: ::windows_core::IntoParam<'a, ScheduledToastNotification>>(&self, scheduledtoast: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveFromSchedule)(::windows_core::Interface::as_raw(this), scheduledtoast.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetScheduledToastNotifications(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<ScheduledToastNotification>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetScheduledToastNotifications)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<ScheduledToastNotification>>(result__)
        }
    }
    pub fn UpdateWithTagAndGroup<'a, Param0: ::windows_core::IntoParam<'a, NotificationData>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param2: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, data: Param0, tag: Param1, group: Param2) -> ::windows_core::Result<NotificationUpdateResult> {
        let this = &::windows_core::Interface::cast::<IToastNotifier2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<NotificationUpdateResult>::zeroed();
            (::windows_core::Interface::vtable(this).UpdateWithTagAndGroup)(::windows_core::Interface::as_raw(this), data.into_param().abi(), tag.into_param().abi(), group.into_param().abi(), result__.as_mut_ptr()).from_abi::<NotificationUpdateResult>(result__)
        }
    }
    pub fn UpdateWithTag<'a, Param0: ::windows_core::IntoParam<'a, NotificationData>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, data: Param0, tag: Param1) -> ::windows_core::Result<NotificationUpdateResult> {
        let this = &::windows_core::Interface::cast::<IToastNotifier2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<NotificationUpdateResult>::zeroed();
            (::windows_core::Interface::vtable(this).UpdateWithTag)(::windows_core::Interface::as_raw(this), data.into_param().abi(), tag.into_param().abi(), result__.as_mut_ptr()).from_abi::<NotificationUpdateResult>(result__)
        }
    }
    pub fn ScheduledToastNotificationShowing<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<ToastNotifier, ScheduledToastNotificationShowingEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = &::windows_core::Interface::cast::<IToastNotifier3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).ScheduledToastNotificationShowing)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveScheduledToastNotificationShowing<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IToastNotifier3>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).RemoveScheduledToastNotificationShowing)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
}
impl ::core::clone::Clone for ToastNotifier {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ToastNotifier {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ToastNotifier {}
impl ::core::fmt::Debug for ToastNotifier {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ToastNotifier").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ToastNotifier {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Notifications.ToastNotifier;{75927b93-03f3-41ec-91d3-6e5bac1b38e7})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for ToastNotifier {
    type Vtable = IToastNotifier_Vtbl;
    const IID: ::windows_core::GUID = <IToastNotifier as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for ToastNotifier {
    const NAME: &'static str = "Windows.UI.Notifications.ToastNotifier";
}
impl ::core::convert::From<ToastNotifier> for ::windows_core::IUnknown {
    fn from(value: ToastNotifier) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ToastNotifier> for ::windows_core::IUnknown {
    fn from(value: &ToastNotifier) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ToastNotifier {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ToastNotifier {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ToastNotifier> for ::windows_core::IInspectable {
    fn from(value: ToastNotifier) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ToastNotifier> for ::windows_core::IInspectable {
    fn from(value: &ToastNotifier) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for ToastNotifier {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a ToastNotifier {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for ToastNotifier {}
unsafe impl ::core::marker::Sync for ToastNotifier {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct ToastTemplateType(pub i32);
impl ToastTemplateType {
    pub const ToastImageAndText01: Self = Self(0i32);
    pub const ToastImageAndText02: Self = Self(1i32);
    pub const ToastImageAndText03: Self = Self(2i32);
    pub const ToastImageAndText04: Self = Self(3i32);
    pub const ToastText01: Self = Self(4i32);
    pub const ToastText02: Self = Self(5i32);
    pub const ToastText03: Self = Self(6i32);
    pub const ToastText04: Self = Self(7i32);
}
impl ::core::marker::Copy for ToastTemplateType {}
impl ::core::clone::Clone for ToastTemplateType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ToastTemplateType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for ToastTemplateType {
    type Abi = Self;
}
impl ::core::fmt::Debug for ToastTemplateType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ToastTemplateType").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ToastTemplateType {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.UI.Notifications.ToastTemplateType;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct UserNotification(::windows_core::IUnknown);
impl UserNotification {
    pub fn Notification(&self) -> ::windows_core::Result<Notification> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Notification)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<Notification>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel")]
    pub fn AppInfo(&self) -> ::windows_core::Result<::winrt_applicationmodel::AppInfo> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).AppInfo)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_applicationmodel::AppInfo>(result__)
        }
    }
    pub fn Id(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).Id)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn CreationTime(&self) -> ::windows_core::Result<::winrt_foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::DateTime>::zeroed();
            (::windows_core::Interface::vtable(this).CreationTime)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::DateTime>(result__)
        }
    }
}
impl ::core::clone::Clone for UserNotification {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for UserNotification {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for UserNotification {}
impl ::core::fmt::Debug for UserNotification {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UserNotification").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for UserNotification {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Notifications.UserNotification;{adf7e52f-4e53-42d5-9c33-eb5ea515b23e})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for UserNotification {
    type Vtable = IUserNotification_Vtbl;
    const IID: ::windows_core::GUID = <IUserNotification as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for UserNotification {
    const NAME: &'static str = "Windows.UI.Notifications.UserNotification";
}
impl ::core::convert::From<UserNotification> for ::windows_core::IUnknown {
    fn from(value: UserNotification) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UserNotification> for ::windows_core::IUnknown {
    fn from(value: &UserNotification) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for UserNotification {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a UserNotification {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<UserNotification> for ::windows_core::IInspectable {
    fn from(value: UserNotification) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UserNotification> for ::windows_core::IInspectable {
    fn from(value: &UserNotification) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for UserNotification {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a UserNotification {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for UserNotification {}
unsafe impl ::core::marker::Sync for UserNotification {}
#[repr(transparent)]
pub struct UserNotificationChangedEventArgs(::windows_core::IUnknown);
impl UserNotificationChangedEventArgs {
    pub fn ChangeKind(&self) -> ::windows_core::Result<UserNotificationChangedKind> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<UserNotificationChangedKind>::zeroed();
            (::windows_core::Interface::vtable(this).ChangeKind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<UserNotificationChangedKind>(result__)
        }
    }
    pub fn UserNotificationId(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).UserNotificationId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
}
impl ::core::clone::Clone for UserNotificationChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for UserNotificationChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for UserNotificationChangedEventArgs {}
impl ::core::fmt::Debug for UserNotificationChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UserNotificationChangedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for UserNotificationChangedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Notifications.UserNotificationChangedEventArgs;{b6bd6839-79cf-4b25-82c0-0ce1eef81f8c})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for UserNotificationChangedEventArgs {
    type Vtable = IUserNotificationChangedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IUserNotificationChangedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for UserNotificationChangedEventArgs {
    const NAME: &'static str = "Windows.UI.Notifications.UserNotificationChangedEventArgs";
}
impl ::core::convert::From<UserNotificationChangedEventArgs> for ::windows_core::IUnknown {
    fn from(value: UserNotificationChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UserNotificationChangedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &UserNotificationChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for UserNotificationChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a UserNotificationChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<UserNotificationChangedEventArgs> for ::windows_core::IInspectable {
    fn from(value: UserNotificationChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UserNotificationChangedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &UserNotificationChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for UserNotificationChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a UserNotificationChangedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for UserNotificationChangedEventArgs {}
unsafe impl ::core::marker::Sync for UserNotificationChangedEventArgs {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct UserNotificationChangedKind(pub i32);
impl UserNotificationChangedKind {
    pub const Added: Self = Self(0i32);
    pub const Removed: Self = Self(1i32);
}
impl ::core::marker::Copy for UserNotificationChangedKind {}
impl ::core::clone::Clone for UserNotificationChangedKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UserNotificationChangedKind {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for UserNotificationChangedKind {
    type Abi = Self;
}
impl ::core::fmt::Debug for UserNotificationChangedKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UserNotificationChangedKind").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for UserNotificationChangedKind {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.UI.Notifications.UserNotificationChangedKind;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
