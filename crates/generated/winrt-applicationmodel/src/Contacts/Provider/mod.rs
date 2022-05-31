#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct AddContactResult(pub i32);
impl AddContactResult {
    pub const Added: Self = Self(0i32);
    pub const AlreadyAdded: Self = Self(1i32);
    pub const Unavailable: Self = Self(2i32);
}
impl ::core::marker::Copy for AddContactResult {}
impl ::core::clone::Clone for AddContactResult {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AddContactResult {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for AddContactResult {
    type Abi = Self;
}
impl ::core::fmt::Debug for AddContactResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AddContactResult").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AddContactResult {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Contacts.Provider.AddContactResult;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct ContactPickerUI(::windows_core::IUnknown);
impl ContactPickerUI {
    #[cfg(feature = "deprecated")]
    pub fn AddContact<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, super::Contact>>(&self, id: Param0, contact: Param1) -> ::windows_core::Result<AddContactResult> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<AddContactResult>::zeroed();
            (::windows_core::Interface::vtable(this).AddContact)(::windows_core::Interface::as_raw(this), id.into_param().abi(), contact.into_param().abi(), result__.as_mut_ptr()).from_abi::<AddContactResult>(result__)
        }
    }
    pub fn RemoveContact<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, id: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveContact)(::windows_core::Interface::as_raw(this), id.into_param().abi()).ok() }
    }
    pub fn ContainsContact<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, id: Param0) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).ContainsContact)(::windows_core::Interface::as_raw(this), id.into_param().abi(), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub fn DesiredFields(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<::windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).DesiredFields)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<::windows_core::HSTRING>>(result__)
        }
    }
    pub fn SelectionMode(&self) -> ::windows_core::Result<super::ContactSelectionMode> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::ContactSelectionMode>::zeroed();
            (::windows_core::Interface::vtable(this).SelectionMode)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::ContactSelectionMode>(result__)
        }
    }
    pub fn ContactRemoved<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<ContactPickerUI, ContactRemovedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).ContactRemoved)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveContactRemoved<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveContactRemoved)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn AddContact2<'a, Param0: ::windows_core::IntoParam<'a, super::Contact>>(&self, contact: Param0) -> ::windows_core::Result<AddContactResult> {
        let this = &::windows_core::Interface::cast::<IContactPickerUI2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<AddContactResult>::zeroed();
            (::windows_core::Interface::vtable(this).AddContact)(::windows_core::Interface::as_raw(this), contact.into_param().abi(), result__.as_mut_ptr()).from_abi::<AddContactResult>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn DesiredFieldsWithContactFieldType(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVector<super::ContactFieldType>> {
        let this = &::windows_core::Interface::cast::<IContactPickerUI2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).DesiredFieldsWithContactFieldType)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVector<super::ContactFieldType>>(result__)
        }
    }
}
impl ::core::clone::Clone for ContactPickerUI {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ContactPickerUI {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ContactPickerUI {}
impl ::core::fmt::Debug for ContactPickerUI {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ContactPickerUI").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ContactPickerUI {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Contacts.Provider.ContactPickerUI;{e2cc1366-cf66-43c4-a96a-a5a112db4746})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for ContactPickerUI {
    type Vtable = IContactPickerUI_Vtbl;
    const IID: ::windows_core::GUID = <IContactPickerUI as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for ContactPickerUI {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.Provider.ContactPickerUI";
}
impl ::core::convert::From<ContactPickerUI> for ::windows_core::IUnknown {
    fn from(value: ContactPickerUI) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ContactPickerUI> for ::windows_core::IUnknown {
    fn from(value: &ContactPickerUI) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ContactPickerUI {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ContactPickerUI {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ContactPickerUI> for ::windows_core::IInspectable {
    fn from(value: ContactPickerUI) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ContactPickerUI> for ::windows_core::IInspectable {
    fn from(value: &ContactPickerUI) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for ContactPickerUI {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a ContactPickerUI {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(transparent)]
pub struct ContactRemovedEventArgs(::windows_core::IUnknown);
impl ContactRemovedEventArgs {
    pub fn Id(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Id)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
}
impl ::core::clone::Clone for ContactRemovedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ContactRemovedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ContactRemovedEventArgs {}
impl ::core::fmt::Debug for ContactRemovedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ContactRemovedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ContactRemovedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Contacts.Provider.ContactRemovedEventArgs;{6f354338-3302-4d13-ad8d-adcc0ff9e47c})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for ContactRemovedEventArgs {
    type Vtable = IContactRemovedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IContactRemovedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for ContactRemovedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.Provider.ContactRemovedEventArgs";
}
impl ::core::convert::From<ContactRemovedEventArgs> for ::windows_core::IUnknown {
    fn from(value: ContactRemovedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ContactRemovedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &ContactRemovedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ContactRemovedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ContactRemovedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ContactRemovedEventArgs> for ::windows_core::IInspectable {
    fn from(value: ContactRemovedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ContactRemovedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &ContactRemovedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for ContactRemovedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a ContactRemovedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IContactPickerUI(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IContactPickerUI {
    type Vtable = IContactPickerUI_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe2cc1366_cf66_43c4_a96a_a5a112db4746);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactPickerUI_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "deprecated")]
    pub AddContact: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, id: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, contact: ::windows_core::RawPtr, result__: *mut AddContactResult) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    AddContact: usize,
    pub RemoveContact: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, id: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub ContainsContact: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, id: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub DesiredFields: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "deprecated")))]
    DesiredFields: usize,
    pub SelectionMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::ContactSelectionMode) -> ::windows_core::HRESULT,
    pub ContactRemoved: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveContactRemoved: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IContactPickerUI2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IContactPickerUI2 {
    type Vtable = IContactPickerUI2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6e449e28_7b25_4999_9b0b_875400a1e8c8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactPickerUI2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub AddContact: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, contact: ::windows_core::RawPtr, result__: *mut AddContactResult) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub DesiredFieldsWithContactFieldType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    DesiredFieldsWithContactFieldType: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IContactRemovedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IContactRemovedEventArgs {
    type Vtable = IContactRemovedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6f354338_3302_4d13_ad8d_adcc0ff9e47c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactRemovedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
