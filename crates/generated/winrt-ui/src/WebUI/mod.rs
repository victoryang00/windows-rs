#[cfg(feature = "Core")]
pub mod Core;
#[repr(transparent)]
pub struct ActivatedDeferral(::windows_core::IUnknown);
impl ActivatedDeferral {
    pub fn Complete(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Complete)(::windows_core::Interface::as_raw(this)).ok() }
    }
}
impl ::core::clone::Clone for ActivatedDeferral {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ActivatedDeferral {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ActivatedDeferral {}
impl ::core::fmt::Debug for ActivatedDeferral {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ActivatedDeferral").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ActivatedDeferral {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.WebUI.ActivatedDeferral;{c3bd1978-a431-49d8-a76a-395a4e03dcf3})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for ActivatedDeferral {
    type Vtable = IActivatedDeferral_Vtbl;
    const IID: ::windows_core::GUID = <IActivatedDeferral as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for ActivatedDeferral {
    const NAME: &'static str = "Windows.UI.WebUI.ActivatedDeferral";
}
impl ::core::convert::From<ActivatedDeferral> for ::windows_core::IUnknown {
    fn from(value: ActivatedDeferral) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ActivatedDeferral> for ::windows_core::IUnknown {
    fn from(value: &ActivatedDeferral) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ActivatedDeferral {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ActivatedDeferral {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ActivatedDeferral> for ::windows_core::IInspectable {
    fn from(value: ActivatedDeferral) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ActivatedDeferral> for ::windows_core::IInspectable {
    fn from(value: &ActivatedDeferral) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for ActivatedDeferral {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a ActivatedDeferral {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
#[repr(transparent)]
pub struct ActivatedEventHandler(pub ::windows_core::IUnknown);
#[cfg(feature = "ApplicationModel_Activation")]
impl ActivatedEventHandler {
    pub fn new<F: FnMut(&::core::option::Option<::windows_core::IInspectable>, &::core::option::Option<::winrt_applicationmodel::Activation::IActivatedEventArgs>) -> ::windows_core::Result<()> + ::core::marker::Send + 'static>(invoke: F) -> Self {
        let com = ActivatedEventHandlerBox::<F> { vtable: &ActivatedEventHandlerBox::<F>::VTABLE, count: ::windows_core::RefCount::new(1), invoke };
        unsafe { ::core::mem::transmute(::windows_core::alloc::boxed::Box::new(com)) }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Invoke<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::IInspectable>, Param1: ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgs>>(&self, sender: Param0, eventargs: Param1) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Invoke)(::windows_core::Interface::as_raw(this), sender.into_param().abi(), eventargs.into_param().abi()).ok() }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
#[repr(C)]
struct ActivatedEventHandlerBox<F: FnMut(&::core::option::Option<::windows_core::IInspectable>, &::core::option::Option<::winrt_applicationmodel::Activation::IActivatedEventArgs>) -> ::windows_core::Result<()> + ::core::marker::Send + 'static> {
    vtable: *const ActivatedEventHandler_Vtbl,
    invoke: F,
    count: ::windows_core::RefCount,
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<F: FnMut(&::core::option::Option<::windows_core::IInspectable>, &::core::option::Option<::winrt_applicationmodel::Activation::IActivatedEventArgs>) -> ::windows_core::Result<()> + ::core::marker::Send + 'static> ActivatedEventHandlerBox<F> {
    const VTABLE: ActivatedEventHandler_Vtbl = ActivatedEventHandler_Vtbl {
        base__: ::windows_core::IUnknownVtbl { QueryInterface: Self::QueryInterface, AddRef: Self::AddRef, Release: Self::Release },
        Invoke: Self::Invoke,
    };
    unsafe extern "system" fn QueryInterface(this: ::windows_core::RawPtr, iid: &::windows_core::GUID, interface: *mut *const ::core::ffi::c_void) -> ::windows_core::HRESULT {
        let this = this as *mut ::windows_core::RawPtr as *mut Self;
        *interface = if iid == &<ActivatedEventHandler as ::windows_core::Interface>::IID || iid == &<::windows_core::IUnknown as ::windows_core::Interface>::IID || iid == &<::windows_core::IAgileObject as ::windows_core::Interface>::IID { &mut (*this).vtable as *mut _ as _ } else { ::core::ptr::null_mut() };
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
    unsafe extern "system" fn Invoke(this: *mut ::core::ffi::c_void, sender: *mut ::core::ffi::c_void, eventargs: ::windows_core::RawPtr) -> ::windows_core::HRESULT {
        let this = this as *mut ::windows_core::RawPtr as *mut Self;
        ((*this).invoke)(::core::mem::transmute(&sender), ::core::mem::transmute(&eventargs)).into()
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::clone::Clone for ActivatedEventHandler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::cmp::PartialEq for ActivatedEventHandler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::cmp::Eq for ActivatedEventHandler {}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::fmt::Debug for ActivatedEventHandler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ActivatedEventHandler").field(&self.0).finish()
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows_core::Interface for ActivatedEventHandler {
    type Vtable = ActivatedEventHandler_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x50f1e730_c5d1_4b6b_9adb_8a11756be29c);
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows_core::RuntimeType for ActivatedEventHandler {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"{50f1e730-c5d1-4b6b-9adb-8a11756be29c}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
#[repr(C)]
#[doc(hidden)]
pub struct ActivatedEventHandler_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    #[cfg(feature = "ApplicationModel_Activation")]
    pub Invoke: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sender: *mut ::core::ffi::c_void, eventargs: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Activation"))]
    Invoke: usize,
}
#[repr(transparent)]
pub struct ActivatedOperation(::windows_core::IUnknown);
impl ActivatedOperation {
    pub fn GetDeferral(&self) -> ::windows_core::Result<ActivatedDeferral> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetDeferral)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivatedDeferral>(result__)
        }
    }
}
impl ::core::clone::Clone for ActivatedOperation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ActivatedOperation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ActivatedOperation {}
impl ::core::fmt::Debug for ActivatedOperation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ActivatedOperation").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ActivatedOperation {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.WebUI.ActivatedOperation;{b6a0b4bc-c6ca-42fd-9818-71904e45fed7})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for ActivatedOperation {
    type Vtable = IActivatedOperation_Vtbl;
    const IID: ::windows_core::GUID = <IActivatedOperation as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for ActivatedOperation {
    const NAME: &'static str = "Windows.UI.WebUI.ActivatedOperation";
}
impl ::core::convert::From<ActivatedOperation> for ::windows_core::IUnknown {
    fn from(value: ActivatedOperation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ActivatedOperation> for ::windows_core::IUnknown {
    fn from(value: &ActivatedOperation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ActivatedOperation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ActivatedOperation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ActivatedOperation> for ::windows_core::IInspectable {
    fn from(value: ActivatedOperation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ActivatedOperation> for ::windows_core::IInspectable {
    fn from(value: &ActivatedOperation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for ActivatedOperation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a ActivatedOperation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
#[repr(transparent)]
pub struct BackgroundActivatedEventArgs(::windows_core::IUnknown);
#[cfg(feature = "ApplicationModel_Activation")]
impl BackgroundActivatedEventArgs {
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "ApplicationModel_Background"))]
    pub fn TaskInstance(&self) -> ::windows_core::Result<::winrt_applicationmodel::Background::IBackgroundTaskInstance> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TaskInstance)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_applicationmodel::Background::IBackgroundTaskInstance>(result__)
        }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::clone::Clone for BackgroundActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::cmp::PartialEq for BackgroundActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::cmp::Eq for BackgroundActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::fmt::Debug for BackgroundActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BackgroundActivatedEventArgs").field(&self.0).finish()
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows_core::RuntimeType for BackgroundActivatedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.WebUI.BackgroundActivatedEventArgs;{ab14bee0-e760-440e-a91c-44796de3a92d})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows_core::Interface for BackgroundActivatedEventArgs {
    type Vtable = ::winrt_applicationmodel::Activation::IBackgroundActivatedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <::winrt_applicationmodel::Activation::IBackgroundActivatedEventArgs as ::windows_core::Interface>::IID;
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::RuntimeName for BackgroundActivatedEventArgs {
    const NAME: &'static str = "Windows.UI.WebUI.BackgroundActivatedEventArgs";
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<BackgroundActivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: BackgroundActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<&BackgroundActivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &BackgroundActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for BackgroundActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a BackgroundActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<BackgroundActivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: BackgroundActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<&BackgroundActivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &BackgroundActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for BackgroundActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a BackgroundActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<BackgroundActivatedEventArgs> for ::winrt_applicationmodel::Activation::IBackgroundActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: BackgroundActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&BackgroundActivatedEventArgs> for ::winrt_applicationmodel::Activation::IBackgroundActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &BackgroundActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IBackgroundActivatedEventArgs> for BackgroundActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IBackgroundActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IBackgroundActivatedEventArgs> for &BackgroundActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IBackgroundActivatedEventArgs> {
        ::core::convert::TryInto::<::winrt_applicationmodel::Activation::IBackgroundActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::core::marker::Send for BackgroundActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::core::marker::Sync for BackgroundActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
#[repr(transparent)]
pub struct BackgroundActivatedEventHandler(pub ::windows_core::IUnknown);
#[cfg(feature = "ApplicationModel_Activation")]
impl BackgroundActivatedEventHandler {
    pub fn new<F: FnMut(&::core::option::Option<::windows_core::IInspectable>, &::core::option::Option<::winrt_applicationmodel::Activation::IBackgroundActivatedEventArgs>) -> ::windows_core::Result<()> + ::core::marker::Send + 'static>(invoke: F) -> Self {
        let com = BackgroundActivatedEventHandlerBox::<F> { vtable: &BackgroundActivatedEventHandlerBox::<F>::VTABLE, count: ::windows_core::RefCount::new(1), invoke };
        unsafe { ::core::mem::transmute(::windows_core::alloc::boxed::Box::new(com)) }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Invoke<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::IInspectable>, Param1: ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IBackgroundActivatedEventArgs>>(&self, sender: Param0, eventargs: Param1) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Invoke)(::windows_core::Interface::as_raw(this), sender.into_param().abi(), eventargs.into_param().abi()).ok() }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
#[repr(C)]
struct BackgroundActivatedEventHandlerBox<F: FnMut(&::core::option::Option<::windows_core::IInspectable>, &::core::option::Option<::winrt_applicationmodel::Activation::IBackgroundActivatedEventArgs>) -> ::windows_core::Result<()> + ::core::marker::Send + 'static> {
    vtable: *const BackgroundActivatedEventHandler_Vtbl,
    invoke: F,
    count: ::windows_core::RefCount,
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<F: FnMut(&::core::option::Option<::windows_core::IInspectable>, &::core::option::Option<::winrt_applicationmodel::Activation::IBackgroundActivatedEventArgs>) -> ::windows_core::Result<()> + ::core::marker::Send + 'static> BackgroundActivatedEventHandlerBox<F> {
    const VTABLE: BackgroundActivatedEventHandler_Vtbl = BackgroundActivatedEventHandler_Vtbl {
        base__: ::windows_core::IUnknownVtbl { QueryInterface: Self::QueryInterface, AddRef: Self::AddRef, Release: Self::Release },
        Invoke: Self::Invoke,
    };
    unsafe extern "system" fn QueryInterface(this: ::windows_core::RawPtr, iid: &::windows_core::GUID, interface: *mut *const ::core::ffi::c_void) -> ::windows_core::HRESULT {
        let this = this as *mut ::windows_core::RawPtr as *mut Self;
        *interface = if iid == &<BackgroundActivatedEventHandler as ::windows_core::Interface>::IID || iid == &<::windows_core::IUnknown as ::windows_core::Interface>::IID || iid == &<::windows_core::IAgileObject as ::windows_core::Interface>::IID { &mut (*this).vtable as *mut _ as _ } else { ::core::ptr::null_mut() };
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
    unsafe extern "system" fn Invoke(this: *mut ::core::ffi::c_void, sender: *mut ::core::ffi::c_void, eventargs: ::windows_core::RawPtr) -> ::windows_core::HRESULT {
        let this = this as *mut ::windows_core::RawPtr as *mut Self;
        ((*this).invoke)(::core::mem::transmute(&sender), ::core::mem::transmute(&eventargs)).into()
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::clone::Clone for BackgroundActivatedEventHandler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::cmp::PartialEq for BackgroundActivatedEventHandler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::cmp::Eq for BackgroundActivatedEventHandler {}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::fmt::Debug for BackgroundActivatedEventHandler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BackgroundActivatedEventHandler").field(&self.0).finish()
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows_core::Interface for BackgroundActivatedEventHandler {
    type Vtable = BackgroundActivatedEventHandler_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xedb19fbb_0761_47cc_9a77_24d7072965ca);
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows_core::RuntimeType for BackgroundActivatedEventHandler {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"{edb19fbb-0761-47cc-9a77-24d7072965ca}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
#[repr(C)]
#[doc(hidden)]
pub struct BackgroundActivatedEventHandler_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    #[cfg(feature = "ApplicationModel_Activation")]
    pub Invoke: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sender: *mut ::core::ffi::c_void, eventargs: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Activation"))]
    Invoke: usize,
}
#[cfg(feature = "ApplicationModel")]
#[repr(transparent)]
pub struct EnteredBackgroundEventArgs(::windows_core::IUnknown);
#[cfg(feature = "ApplicationModel")]
impl EnteredBackgroundEventArgs {
    #[cfg(feature = "ApplicationModel")]
    pub fn GetDeferral(&self) -> ::windows_core::Result<::winrt_foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetDeferral)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Deferral>(result__)
        }
    }
}
#[cfg(feature = "ApplicationModel")]
impl ::core::clone::Clone for EnteredBackgroundEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "ApplicationModel")]
impl ::core::cmp::PartialEq for EnteredBackgroundEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "ApplicationModel")]
impl ::core::cmp::Eq for EnteredBackgroundEventArgs {}
#[cfg(feature = "ApplicationModel")]
impl ::core::fmt::Debug for EnteredBackgroundEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EnteredBackgroundEventArgs").field(&self.0).finish()
    }
}
#[cfg(feature = "ApplicationModel")]
unsafe impl ::windows_core::RuntimeType for EnteredBackgroundEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.WebUI.EnteredBackgroundEventArgs;{f722dcc2-9827-403d-aaed-ecca9ac17398})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[cfg(feature = "ApplicationModel")]
unsafe impl ::windows_core::Interface for EnteredBackgroundEventArgs {
    type Vtable = ::winrt_applicationmodel::IEnteredBackgroundEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <::winrt_applicationmodel::IEnteredBackgroundEventArgs as ::windows_core::Interface>::IID;
}
#[cfg(feature = "ApplicationModel")]
impl ::windows_core::RuntimeName for EnteredBackgroundEventArgs {
    const NAME: &'static str = "Windows.UI.WebUI.EnteredBackgroundEventArgs";
}
#[cfg(feature = "ApplicationModel")]
impl ::core::convert::From<EnteredBackgroundEventArgs> for ::windows_core::IUnknown {
    fn from(value: EnteredBackgroundEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel")]
impl ::core::convert::From<&EnteredBackgroundEventArgs> for ::windows_core::IUnknown {
    fn from(value: &EnteredBackgroundEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for EnteredBackgroundEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "ApplicationModel")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a EnteredBackgroundEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "ApplicationModel")]
impl ::core::convert::From<EnteredBackgroundEventArgs> for ::windows_core::IInspectable {
    fn from(value: EnteredBackgroundEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel")]
impl ::core::convert::From<&EnteredBackgroundEventArgs> for ::windows_core::IInspectable {
    fn from(value: &EnteredBackgroundEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for EnteredBackgroundEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "ApplicationModel")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a EnteredBackgroundEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "ApplicationModel")]
impl ::core::convert::TryFrom<EnteredBackgroundEventArgs> for ::winrt_applicationmodel::IEnteredBackgroundEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: EnteredBackgroundEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel")]
impl ::core::convert::TryFrom<&EnteredBackgroundEventArgs> for ::winrt_applicationmodel::IEnteredBackgroundEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &EnteredBackgroundEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::IEnteredBackgroundEventArgs> for EnteredBackgroundEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::IEnteredBackgroundEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::IEnteredBackgroundEventArgs> for &EnteredBackgroundEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::IEnteredBackgroundEventArgs> {
        ::core::convert::TryInto::<::winrt_applicationmodel::IEnteredBackgroundEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "ApplicationModel")]
unsafe impl ::core::marker::Send for EnteredBackgroundEventArgs {}
#[cfg(feature = "ApplicationModel")]
unsafe impl ::core::marker::Sync for EnteredBackgroundEventArgs {}
#[cfg(feature = "ApplicationModel")]
#[repr(transparent)]
pub struct EnteredBackgroundEventHandler(pub ::windows_core::IUnknown);
#[cfg(feature = "ApplicationModel")]
impl EnteredBackgroundEventHandler {
    pub fn new<F: FnMut(&::core::option::Option<::windows_core::IInspectable>, &::core::option::Option<::winrt_applicationmodel::IEnteredBackgroundEventArgs>) -> ::windows_core::Result<()> + ::core::marker::Send + 'static>(invoke: F) -> Self {
        let com = EnteredBackgroundEventHandlerBox::<F> { vtable: &EnteredBackgroundEventHandlerBox::<F>::VTABLE, count: ::windows_core::RefCount::new(1), invoke };
        unsafe { ::core::mem::transmute(::windows_core::alloc::boxed::Box::new(com)) }
    }
    #[cfg(feature = "ApplicationModel")]
    pub fn Invoke<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::IInspectable>, Param1: ::windows_core::IntoParam<'a, ::winrt_applicationmodel::IEnteredBackgroundEventArgs>>(&self, sender: Param0, e: Param1) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Invoke)(::windows_core::Interface::as_raw(this), sender.into_param().abi(), e.into_param().abi()).ok() }
    }
}
#[cfg(feature = "ApplicationModel")]
#[repr(C)]
struct EnteredBackgroundEventHandlerBox<F: FnMut(&::core::option::Option<::windows_core::IInspectable>, &::core::option::Option<::winrt_applicationmodel::IEnteredBackgroundEventArgs>) -> ::windows_core::Result<()> + ::core::marker::Send + 'static> {
    vtable: *const EnteredBackgroundEventHandler_Vtbl,
    invoke: F,
    count: ::windows_core::RefCount,
}
#[cfg(feature = "ApplicationModel")]
impl<F: FnMut(&::core::option::Option<::windows_core::IInspectable>, &::core::option::Option<::winrt_applicationmodel::IEnteredBackgroundEventArgs>) -> ::windows_core::Result<()> + ::core::marker::Send + 'static> EnteredBackgroundEventHandlerBox<F> {
    const VTABLE: EnteredBackgroundEventHandler_Vtbl = EnteredBackgroundEventHandler_Vtbl {
        base__: ::windows_core::IUnknownVtbl { QueryInterface: Self::QueryInterface, AddRef: Self::AddRef, Release: Self::Release },
        Invoke: Self::Invoke,
    };
    unsafe extern "system" fn QueryInterface(this: ::windows_core::RawPtr, iid: &::windows_core::GUID, interface: *mut *const ::core::ffi::c_void) -> ::windows_core::HRESULT {
        let this = this as *mut ::windows_core::RawPtr as *mut Self;
        *interface = if iid == &<EnteredBackgroundEventHandler as ::windows_core::Interface>::IID || iid == &<::windows_core::IUnknown as ::windows_core::Interface>::IID || iid == &<::windows_core::IAgileObject as ::windows_core::Interface>::IID { &mut (*this).vtable as *mut _ as _ } else { ::core::ptr::null_mut() };
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
#[cfg(feature = "ApplicationModel")]
impl ::core::clone::Clone for EnteredBackgroundEventHandler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "ApplicationModel")]
impl ::core::cmp::PartialEq for EnteredBackgroundEventHandler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "ApplicationModel")]
impl ::core::cmp::Eq for EnteredBackgroundEventHandler {}
#[cfg(feature = "ApplicationModel")]
impl ::core::fmt::Debug for EnteredBackgroundEventHandler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EnteredBackgroundEventHandler").field(&self.0).finish()
    }
}
#[cfg(feature = "ApplicationModel")]
unsafe impl ::windows_core::Interface for EnteredBackgroundEventHandler {
    type Vtable = EnteredBackgroundEventHandler_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2b09a173_b68e_4def_88c1_8de84e5aab2f);
}
#[cfg(feature = "ApplicationModel")]
unsafe impl ::windows_core::RuntimeType for EnteredBackgroundEventHandler {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"{2b09a173-b68e-4def-88c1-8de84e5aab2f}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[cfg(feature = "ApplicationModel")]
#[repr(C)]
#[doc(hidden)]
pub struct EnteredBackgroundEventHandler_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    #[cfg(feature = "ApplicationModel")]
    pub Invoke: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sender: *mut ::core::ffi::c_void, e: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "ApplicationModel"))]
    Invoke: usize,
}
#[repr(transparent)]
pub struct HtmlPrintDocumentSource(::windows_core::IUnknown);
impl HtmlPrintDocumentSource {
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::IClosable>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Content(&self) -> ::windows_core::Result<PrintContent> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<PrintContent>::zeroed();
            (::windows_core::Interface::vtable(this).Content)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PrintContent>(result__)
        }
    }
    pub fn SetContent(&self, value: PrintContent) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetContent)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn LeftMargin(&self) -> ::windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f32>::zeroed();
            (::windows_core::Interface::vtable(this).LeftMargin)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f32>(result__)
        }
    }
    pub fn SetLeftMargin(&self, value: f32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetLeftMargin)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn TopMargin(&self) -> ::windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f32>::zeroed();
            (::windows_core::Interface::vtable(this).TopMargin)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f32>(result__)
        }
    }
    pub fn SetTopMargin(&self, value: f32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetTopMargin)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn RightMargin(&self) -> ::windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f32>::zeroed();
            (::windows_core::Interface::vtable(this).RightMargin)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f32>(result__)
        }
    }
    pub fn SetRightMargin(&self, value: f32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetRightMargin)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn BottomMargin(&self) -> ::windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f32>::zeroed();
            (::windows_core::Interface::vtable(this).BottomMargin)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f32>(result__)
        }
    }
    pub fn SetBottomMargin(&self, value: f32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetBottomMargin)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn EnableHeaderFooter(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).EnableHeaderFooter)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetEnableHeaderFooter(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetEnableHeaderFooter)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ShrinkToFit(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).ShrinkToFit)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetShrinkToFit(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetShrinkToFit)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn PercentScale(&self) -> ::windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f32>::zeroed();
            (::windows_core::Interface::vtable(this).PercentScale)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f32>(result__)
        }
    }
    pub fn SetPercentScale(&self, scalepercent: f32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetPercentScale)(::windows_core::Interface::as_raw(this), scalepercent).ok() }
    }
    pub fn PageRange(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).PageRange)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn TrySetPageRange<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, strpagerange: Param0) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).TrySetPageRange)(::windows_core::Interface::as_raw(this), strpagerange.into_param().abi(), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
}
impl ::core::clone::Clone for HtmlPrintDocumentSource {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for HtmlPrintDocumentSource {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HtmlPrintDocumentSource {}
impl ::core::fmt::Debug for HtmlPrintDocumentSource {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HtmlPrintDocumentSource").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for HtmlPrintDocumentSource {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.WebUI.HtmlPrintDocumentSource;{cea6469a-0e05-467a-abc9-36ec1d4cdcb6})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for HtmlPrintDocumentSource {
    type Vtable = IHtmlPrintDocumentSource_Vtbl;
    const IID: ::windows_core::GUID = <IHtmlPrintDocumentSource as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for HtmlPrintDocumentSource {
    const NAME: &'static str = "Windows.UI.WebUI.HtmlPrintDocumentSource";
}
impl ::core::convert::From<HtmlPrintDocumentSource> for ::windows_core::IUnknown {
    fn from(value: HtmlPrintDocumentSource) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HtmlPrintDocumentSource> for ::windows_core::IUnknown {
    fn from(value: &HtmlPrintDocumentSource) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for HtmlPrintDocumentSource {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a HtmlPrintDocumentSource {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<HtmlPrintDocumentSource> for ::windows_core::IInspectable {
    fn from(value: HtmlPrintDocumentSource) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HtmlPrintDocumentSource> for ::windows_core::IInspectable {
    fn from(value: &HtmlPrintDocumentSource) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for HtmlPrintDocumentSource {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a HtmlPrintDocumentSource {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<HtmlPrintDocumentSource> for ::winrt_foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: HtmlPrintDocumentSource) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&HtmlPrintDocumentSource> for ::winrt_foundation::IClosable {
    type Error = ::windows_core::Error;
    fn try_from(value: &HtmlPrintDocumentSource) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IClosable> for HtmlPrintDocumentSource {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IClosable> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IClosable> for &HtmlPrintDocumentSource {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IClosable> {
        ::core::convert::TryInto::<::winrt_foundation::IClosable>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "Graphics_Printing")]
impl ::core::convert::TryFrom<HtmlPrintDocumentSource> for ::winrt_graphics::Printing::IPrintDocumentSource {
    type Error = ::windows_core::Error;
    fn try_from(value: HtmlPrintDocumentSource) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Graphics_Printing")]
impl ::core::convert::TryFrom<&HtmlPrintDocumentSource> for ::winrt_graphics::Printing::IPrintDocumentSource {
    type Error = ::windows_core::Error;
    fn try_from(value: &HtmlPrintDocumentSource) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "Graphics_Printing")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_graphics::Printing::IPrintDocumentSource> for HtmlPrintDocumentSource {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_graphics::Printing::IPrintDocumentSource> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Graphics_Printing")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_graphics::Printing::IPrintDocumentSource> for &HtmlPrintDocumentSource {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_graphics::Printing::IPrintDocumentSource> {
        ::core::convert::TryInto::<::winrt_graphics::Printing::IPrintDocumentSource>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for HtmlPrintDocumentSource {}
unsafe impl ::core::marker::Sync for HtmlPrintDocumentSource {}
#[doc(hidden)]
#[repr(transparent)]
pub struct IActivatedDeferral(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IActivatedDeferral {
    type Vtable = IActivatedDeferral_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc3bd1978_a431_49d8_a76a_395a4e03dcf3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IActivatedDeferral_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Complete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IActivatedEventArgsDeferral(::windows_core::IUnknown);
impl IActivatedEventArgsDeferral {
    pub fn ActivatedOperation(&self) -> ::windows_core::Result<ActivatedOperation> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ActivatedOperation)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivatedOperation>(result__)
        }
    }
}
impl ::core::convert::From<IActivatedEventArgsDeferral> for ::windows_core::IUnknown {
    fn from(value: IActivatedEventArgsDeferral) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IActivatedEventArgsDeferral> for ::windows_core::IUnknown {
    fn from(value: &IActivatedEventArgsDeferral) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IActivatedEventArgsDeferral {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IActivatedEventArgsDeferral {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IActivatedEventArgsDeferral> for ::windows_core::IInspectable {
    fn from(value: IActivatedEventArgsDeferral) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IActivatedEventArgsDeferral> for ::windows_core::IInspectable {
    fn from(value: &IActivatedEventArgsDeferral) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for IActivatedEventArgsDeferral {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a IActivatedEventArgsDeferral {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IActivatedEventArgsDeferral {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IActivatedEventArgsDeferral {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IActivatedEventArgsDeferral {}
impl ::core::fmt::Debug for IActivatedEventArgsDeferral {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IActivatedEventArgsDeferral").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for IActivatedEventArgsDeferral {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"{ca6d5f74-63c2-44a6-b97b-d9a03c20bc9b}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for IActivatedEventArgsDeferral {
    type Vtable = IActivatedEventArgsDeferral_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xca6d5f74_63c2_44a6_b97b_d9a03c20bc9b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IActivatedEventArgsDeferral_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub ActivatedOperation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IActivatedOperation(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IActivatedOperation {
    type Vtable = IActivatedOperation_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb6a0b4bc_c6ca_42fd_9818_71904e45fed7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IActivatedOperation_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHtmlPrintDocumentSource(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IHtmlPrintDocumentSource {
    type Vtable = IHtmlPrintDocumentSource_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xcea6469a_0e05_467a_abc9_36ec1d4cdcb6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHtmlPrintDocumentSource_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Content: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PrintContent) -> ::windows_core::HRESULT,
    pub SetContent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: PrintContent) -> ::windows_core::HRESULT,
    pub LeftMargin: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows_core::HRESULT,
    pub SetLeftMargin: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f32) -> ::windows_core::HRESULT,
    pub TopMargin: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows_core::HRESULT,
    pub SetTopMargin: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f32) -> ::windows_core::HRESULT,
    pub RightMargin: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows_core::HRESULT,
    pub SetRightMargin: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f32) -> ::windows_core::HRESULT,
    pub BottomMargin: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows_core::HRESULT,
    pub SetBottomMargin: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f32) -> ::windows_core::HRESULT,
    pub EnableHeaderFooter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetEnableHeaderFooter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub ShrinkToFit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetShrinkToFit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub PercentScale: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows_core::HRESULT,
    pub SetPercentScale: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, scalepercent: f32) -> ::windows_core::HRESULT,
    pub PageRange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub TrySetPageRange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strpagerange: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct INewWebUIViewCreatedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for INewWebUIViewCreatedEventArgs {
    type Vtable = INewWebUIViewCreatedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe8e1b216_be2b_4c9e_85e7_083143ec4be7);
}
#[repr(C)]
#[doc(hidden)]
pub struct INewWebUIViewCreatedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub WebUIView: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "ApplicationModel_Activation")]
    pub ActivatedEventArgs: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Activation"))]
    ActivatedEventArgs: usize,
    pub HasPendingNavigate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWebUIActivationStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWebUIActivationStatics {
    type Vtable = IWebUIActivationStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x351b86bd_43b3_482b_85db_35d87b517ad9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebUIActivationStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "ApplicationModel_Activation")]
    pub Activated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Activation"))]
    Activated: usize,
    pub RemoveActivated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(feature = "ApplicationModel")]
    pub Suspending: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "ApplicationModel"))]
    Suspending: usize,
    pub RemoveSuspending: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub Resuming: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveResuming: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub Navigated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveNavigated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWebUIActivationStatics2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWebUIActivationStatics2 {
    type Vtable = IWebUIActivationStatics2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc8e88696_4d78_4aa4_8f06_2a9eadc6c40a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebUIActivationStatics2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "ApplicationModel")]
    pub LeavingBackground: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "ApplicationModel"))]
    LeavingBackground: usize,
    pub RemoveLeavingBackground: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(feature = "ApplicationModel")]
    pub EnteredBackground: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "ApplicationModel"))]
    EnteredBackground: usize,
    pub RemoveEnteredBackground: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub EnablePrelaunch: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWebUIActivationStatics3(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWebUIActivationStatics3 {
    type Vtable = IWebUIActivationStatics3_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x91abb686_1af5_4445_b49f_9459f40fc8de);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebUIActivationStatics3_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "ApplicationModel_Core")]
    pub RequestRestartAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, launcharguments: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Core"))]
    RequestRestartAsync: usize,
    #[cfg(all(feature = "ApplicationModel_Core", feature = "System"))]
    pub RequestRestartForUserAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: ::windows_core::RawPtr, launcharguments: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel_Core", feature = "System")))]
    RequestRestartForUserAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWebUIActivationStatics4(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWebUIActivationStatics4 {
    type Vtable = IWebUIActivationStatics4_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5e391429_183f_478d_8a25_67f80d03935b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebUIActivationStatics4_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub NewWebUIViewCreated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveNewWebUIViewCreated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(feature = "ApplicationModel_Activation")]
    pub BackgroundActivated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Activation"))]
    BackgroundActivated: usize,
    pub RemoveBackgroundActivated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IWebUIBackgroundTaskInstance(::windows_core::IUnknown);
impl IWebUIBackgroundTaskInstance {
    pub fn Succeeded(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).Succeeded)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetSucceeded(&self, succeeded: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetSucceeded)(::windows_core::Interface::as_raw(this), succeeded).ok() }
    }
}
impl ::core::convert::From<IWebUIBackgroundTaskInstance> for ::windows_core::IUnknown {
    fn from(value: IWebUIBackgroundTaskInstance) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWebUIBackgroundTaskInstance> for ::windows_core::IUnknown {
    fn from(value: &IWebUIBackgroundTaskInstance) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IWebUIBackgroundTaskInstance {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IWebUIBackgroundTaskInstance {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWebUIBackgroundTaskInstance> for ::windows_core::IInspectable {
    fn from(value: IWebUIBackgroundTaskInstance) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWebUIBackgroundTaskInstance> for ::windows_core::IInspectable {
    fn from(value: &IWebUIBackgroundTaskInstance) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for IWebUIBackgroundTaskInstance {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a IWebUIBackgroundTaskInstance {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWebUIBackgroundTaskInstance {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWebUIBackgroundTaskInstance {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWebUIBackgroundTaskInstance {}
impl ::core::fmt::Debug for IWebUIBackgroundTaskInstance {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWebUIBackgroundTaskInstance").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for IWebUIBackgroundTaskInstance {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"{23f12c25-e2f7-4741-bc9c-394595de24dc}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for IWebUIBackgroundTaskInstance {
    type Vtable = IWebUIBackgroundTaskInstance_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x23f12c25_e2f7_4741_bc9c_394595de24dc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebUIBackgroundTaskInstance_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Succeeded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetSucceeded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, succeeded: bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWebUIBackgroundTaskInstanceStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWebUIBackgroundTaskInstanceStatics {
    type Vtable = IWebUIBackgroundTaskInstanceStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9c7a5291_19ae_4ca3_b94b_fe4ec744a740);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebUIBackgroundTaskInstanceStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Current: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWebUINavigatedDeferral(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWebUINavigatedDeferral {
    type Vtable = IWebUINavigatedDeferral_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd804204d_831f_46e2_b432_3afce211f962);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebUINavigatedDeferral_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Complete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IWebUINavigatedEventArgs(::windows_core::IUnknown);
impl IWebUINavigatedEventArgs {
    pub fn NavigatedOperation(&self) -> ::windows_core::Result<WebUINavigatedOperation> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).NavigatedOperation)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<WebUINavigatedOperation>(result__)
        }
    }
}
impl ::core::convert::From<IWebUINavigatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: IWebUINavigatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWebUINavigatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &IWebUINavigatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IWebUINavigatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IWebUINavigatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWebUINavigatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: IWebUINavigatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWebUINavigatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &IWebUINavigatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for IWebUINavigatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a IWebUINavigatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWebUINavigatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWebUINavigatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWebUINavigatedEventArgs {}
impl ::core::fmt::Debug for IWebUINavigatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWebUINavigatedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for IWebUINavigatedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"{a75841b8-2499-4030-a69d-15d2d9cfe524}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for IWebUINavigatedEventArgs {
    type Vtable = IWebUINavigatedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa75841b8_2499_4030_a69d_15d2d9cfe524);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebUINavigatedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub NavigatedOperation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWebUINavigatedOperation(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWebUINavigatedOperation {
    type Vtable = IWebUINavigatedOperation_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7a965f08_8182_4a89_ab67_8492e8750d4b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebUINavigatedOperation_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWebUIView(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWebUIView {
    type Vtable = IWebUIView_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6783f64f_52da_4fd7_be69_8ef6284b423c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebUIView_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub ApplicationViewId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub Closed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveClosed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(feature = "ApplicationModel_Activation")]
    pub Activated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Activation"))]
    Activated: usize,
    pub RemoveActivated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub IgnoreApplicationContentUriRulesNavigationRestrictions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetIgnoreApplicationContentUriRulesNavigationRestrictions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWebUIViewStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWebUIViewStatics {
    type Vtable = IWebUIViewStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb591e668_8e59_44f9_8803_1b24c9149d30);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebUIViewStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CreateAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CreateWithUriAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uri: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[cfg(feature = "ApplicationModel")]
#[repr(transparent)]
pub struct LeavingBackgroundEventArgs(::windows_core::IUnknown);
#[cfg(feature = "ApplicationModel")]
impl LeavingBackgroundEventArgs {
    #[cfg(feature = "ApplicationModel")]
    pub fn GetDeferral(&self) -> ::windows_core::Result<::winrt_foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetDeferral)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Deferral>(result__)
        }
    }
}
#[cfg(feature = "ApplicationModel")]
impl ::core::clone::Clone for LeavingBackgroundEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "ApplicationModel")]
impl ::core::cmp::PartialEq for LeavingBackgroundEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "ApplicationModel")]
impl ::core::cmp::Eq for LeavingBackgroundEventArgs {}
#[cfg(feature = "ApplicationModel")]
impl ::core::fmt::Debug for LeavingBackgroundEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LeavingBackgroundEventArgs").field(&self.0).finish()
    }
}
#[cfg(feature = "ApplicationModel")]
unsafe impl ::windows_core::RuntimeType for LeavingBackgroundEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.WebUI.LeavingBackgroundEventArgs;{39c6ec9a-ae6e-46f9-a07a-cfc23f88733e})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[cfg(feature = "ApplicationModel")]
unsafe impl ::windows_core::Interface for LeavingBackgroundEventArgs {
    type Vtable = ::winrt_applicationmodel::ILeavingBackgroundEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <::winrt_applicationmodel::ILeavingBackgroundEventArgs as ::windows_core::Interface>::IID;
}
#[cfg(feature = "ApplicationModel")]
impl ::windows_core::RuntimeName for LeavingBackgroundEventArgs {
    const NAME: &'static str = "Windows.UI.WebUI.LeavingBackgroundEventArgs";
}
#[cfg(feature = "ApplicationModel")]
impl ::core::convert::From<LeavingBackgroundEventArgs> for ::windows_core::IUnknown {
    fn from(value: LeavingBackgroundEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel")]
impl ::core::convert::From<&LeavingBackgroundEventArgs> for ::windows_core::IUnknown {
    fn from(value: &LeavingBackgroundEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for LeavingBackgroundEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "ApplicationModel")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a LeavingBackgroundEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "ApplicationModel")]
impl ::core::convert::From<LeavingBackgroundEventArgs> for ::windows_core::IInspectable {
    fn from(value: LeavingBackgroundEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel")]
impl ::core::convert::From<&LeavingBackgroundEventArgs> for ::windows_core::IInspectable {
    fn from(value: &LeavingBackgroundEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for LeavingBackgroundEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "ApplicationModel")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a LeavingBackgroundEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "ApplicationModel")]
impl ::core::convert::TryFrom<LeavingBackgroundEventArgs> for ::winrt_applicationmodel::ILeavingBackgroundEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: LeavingBackgroundEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel")]
impl ::core::convert::TryFrom<&LeavingBackgroundEventArgs> for ::winrt_applicationmodel::ILeavingBackgroundEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &LeavingBackgroundEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::ILeavingBackgroundEventArgs> for LeavingBackgroundEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::ILeavingBackgroundEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::ILeavingBackgroundEventArgs> for &LeavingBackgroundEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::ILeavingBackgroundEventArgs> {
        ::core::convert::TryInto::<::winrt_applicationmodel::ILeavingBackgroundEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "ApplicationModel")]
unsafe impl ::core::marker::Send for LeavingBackgroundEventArgs {}
#[cfg(feature = "ApplicationModel")]
unsafe impl ::core::marker::Sync for LeavingBackgroundEventArgs {}
#[cfg(feature = "ApplicationModel")]
#[repr(transparent)]
pub struct LeavingBackgroundEventHandler(pub ::windows_core::IUnknown);
#[cfg(feature = "ApplicationModel")]
impl LeavingBackgroundEventHandler {
    pub fn new<F: FnMut(&::core::option::Option<::windows_core::IInspectable>, &::core::option::Option<::winrt_applicationmodel::ILeavingBackgroundEventArgs>) -> ::windows_core::Result<()> + ::core::marker::Send + 'static>(invoke: F) -> Self {
        let com = LeavingBackgroundEventHandlerBox::<F> { vtable: &LeavingBackgroundEventHandlerBox::<F>::VTABLE, count: ::windows_core::RefCount::new(1), invoke };
        unsafe { ::core::mem::transmute(::windows_core::alloc::boxed::Box::new(com)) }
    }
    #[cfg(feature = "ApplicationModel")]
    pub fn Invoke<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::IInspectable>, Param1: ::windows_core::IntoParam<'a, ::winrt_applicationmodel::ILeavingBackgroundEventArgs>>(&self, sender: Param0, e: Param1) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Invoke)(::windows_core::Interface::as_raw(this), sender.into_param().abi(), e.into_param().abi()).ok() }
    }
}
#[cfg(feature = "ApplicationModel")]
#[repr(C)]
struct LeavingBackgroundEventHandlerBox<F: FnMut(&::core::option::Option<::windows_core::IInspectable>, &::core::option::Option<::winrt_applicationmodel::ILeavingBackgroundEventArgs>) -> ::windows_core::Result<()> + ::core::marker::Send + 'static> {
    vtable: *const LeavingBackgroundEventHandler_Vtbl,
    invoke: F,
    count: ::windows_core::RefCount,
}
#[cfg(feature = "ApplicationModel")]
impl<F: FnMut(&::core::option::Option<::windows_core::IInspectable>, &::core::option::Option<::winrt_applicationmodel::ILeavingBackgroundEventArgs>) -> ::windows_core::Result<()> + ::core::marker::Send + 'static> LeavingBackgroundEventHandlerBox<F> {
    const VTABLE: LeavingBackgroundEventHandler_Vtbl = LeavingBackgroundEventHandler_Vtbl {
        base__: ::windows_core::IUnknownVtbl { QueryInterface: Self::QueryInterface, AddRef: Self::AddRef, Release: Self::Release },
        Invoke: Self::Invoke,
    };
    unsafe extern "system" fn QueryInterface(this: ::windows_core::RawPtr, iid: &::windows_core::GUID, interface: *mut *const ::core::ffi::c_void) -> ::windows_core::HRESULT {
        let this = this as *mut ::windows_core::RawPtr as *mut Self;
        *interface = if iid == &<LeavingBackgroundEventHandler as ::windows_core::Interface>::IID || iid == &<::windows_core::IUnknown as ::windows_core::Interface>::IID || iid == &<::windows_core::IAgileObject as ::windows_core::Interface>::IID { &mut (*this).vtable as *mut _ as _ } else { ::core::ptr::null_mut() };
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
#[cfg(feature = "ApplicationModel")]
impl ::core::clone::Clone for LeavingBackgroundEventHandler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "ApplicationModel")]
impl ::core::cmp::PartialEq for LeavingBackgroundEventHandler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "ApplicationModel")]
impl ::core::cmp::Eq for LeavingBackgroundEventHandler {}
#[cfg(feature = "ApplicationModel")]
impl ::core::fmt::Debug for LeavingBackgroundEventHandler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LeavingBackgroundEventHandler").field(&self.0).finish()
    }
}
#[cfg(feature = "ApplicationModel")]
unsafe impl ::windows_core::Interface for LeavingBackgroundEventHandler {
    type Vtable = LeavingBackgroundEventHandler_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x00b4ccd9_7a9c_4b6b_9ac4_13474f268bc4);
}
#[cfg(feature = "ApplicationModel")]
unsafe impl ::windows_core::RuntimeType for LeavingBackgroundEventHandler {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"{00b4ccd9-7a9c-4b6b-9ac4-13474f268bc4}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[cfg(feature = "ApplicationModel")]
#[repr(C)]
#[doc(hidden)]
pub struct LeavingBackgroundEventHandler_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    #[cfg(feature = "ApplicationModel")]
    pub Invoke: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sender: *mut ::core::ffi::c_void, e: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "ApplicationModel"))]
    Invoke: usize,
}
#[repr(transparent)]
pub struct NavigatedEventHandler(pub ::windows_core::IUnknown);
impl NavigatedEventHandler {
    pub fn new<F: FnMut(&::core::option::Option<::windows_core::IInspectable>, &::core::option::Option<IWebUINavigatedEventArgs>) -> ::windows_core::Result<()> + ::core::marker::Send + 'static>(invoke: F) -> Self {
        let com = NavigatedEventHandlerBox::<F> { vtable: &NavigatedEventHandlerBox::<F>::VTABLE, count: ::windows_core::RefCount::new(1), invoke };
        unsafe { ::core::mem::transmute(::windows_core::alloc::boxed::Box::new(com)) }
    }
    pub fn Invoke<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::IInspectable>, Param1: ::windows_core::IntoParam<'a, IWebUINavigatedEventArgs>>(&self, sender: Param0, e: Param1) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Invoke)(::windows_core::Interface::as_raw(this), sender.into_param().abi(), e.into_param().abi()).ok() }
    }
}
#[repr(C)]
struct NavigatedEventHandlerBox<F: FnMut(&::core::option::Option<::windows_core::IInspectable>, &::core::option::Option<IWebUINavigatedEventArgs>) -> ::windows_core::Result<()> + ::core::marker::Send + 'static> {
    vtable: *const NavigatedEventHandler_Vtbl,
    invoke: F,
    count: ::windows_core::RefCount,
}
impl<F: FnMut(&::core::option::Option<::windows_core::IInspectable>, &::core::option::Option<IWebUINavigatedEventArgs>) -> ::windows_core::Result<()> + ::core::marker::Send + 'static> NavigatedEventHandlerBox<F> {
    const VTABLE: NavigatedEventHandler_Vtbl = NavigatedEventHandler_Vtbl {
        base__: ::windows_core::IUnknownVtbl { QueryInterface: Self::QueryInterface, AddRef: Self::AddRef, Release: Self::Release },
        Invoke: Self::Invoke,
    };
    unsafe extern "system" fn QueryInterface(this: ::windows_core::RawPtr, iid: &::windows_core::GUID, interface: *mut *const ::core::ffi::c_void) -> ::windows_core::HRESULT {
        let this = this as *mut ::windows_core::RawPtr as *mut Self;
        *interface = if iid == &<NavigatedEventHandler as ::windows_core::Interface>::IID || iid == &<::windows_core::IUnknown as ::windows_core::Interface>::IID || iid == &<::windows_core::IAgileObject as ::windows_core::Interface>::IID { &mut (*this).vtable as *mut _ as _ } else { ::core::ptr::null_mut() };
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
impl ::core::clone::Clone for NavigatedEventHandler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for NavigatedEventHandler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for NavigatedEventHandler {}
impl ::core::fmt::Debug for NavigatedEventHandler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NavigatedEventHandler").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for NavigatedEventHandler {
    type Vtable = NavigatedEventHandler_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7af46fe6_40ca_4e49_a7d6_dbdb330cd1a3);
}
unsafe impl ::windows_core::RuntimeType for NavigatedEventHandler {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"{7af46fe6-40ca-4e49-a7d6-dbdb330cd1a3}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct NavigatedEventHandler_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub Invoke: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sender: *mut ::core::ffi::c_void, e: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct NewWebUIViewCreatedEventArgs(::windows_core::IUnknown);
impl NewWebUIViewCreatedEventArgs {
    pub fn WebUIView(&self) -> ::windows_core::Result<WebUIView> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).WebUIView)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<WebUIView>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn ActivatedEventArgs(&self) -> ::windows_core::Result<::winrt_applicationmodel::Activation::IActivatedEventArgs> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ActivatedEventArgs)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_applicationmodel::Activation::IActivatedEventArgs>(result__)
        }
    }
    pub fn HasPendingNavigate(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).HasPendingNavigate)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
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
impl ::core::clone::Clone for NewWebUIViewCreatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for NewWebUIViewCreatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for NewWebUIViewCreatedEventArgs {}
impl ::core::fmt::Debug for NewWebUIViewCreatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NewWebUIViewCreatedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for NewWebUIViewCreatedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.WebUI.NewWebUIViewCreatedEventArgs;{e8e1b216-be2b-4c9e-85e7-083143ec4be7})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for NewWebUIViewCreatedEventArgs {
    type Vtable = INewWebUIViewCreatedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <INewWebUIViewCreatedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for NewWebUIViewCreatedEventArgs {
    const NAME: &'static str = "Windows.UI.WebUI.NewWebUIViewCreatedEventArgs";
}
impl ::core::convert::From<NewWebUIViewCreatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: NewWebUIViewCreatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&NewWebUIViewCreatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &NewWebUIViewCreatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for NewWebUIViewCreatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a NewWebUIViewCreatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<NewWebUIViewCreatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: NewWebUIViewCreatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&NewWebUIViewCreatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &NewWebUIViewCreatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for NewWebUIViewCreatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a NewWebUIViewCreatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PrintContent(pub i32);
impl PrintContent {
    pub const AllPages: Self = Self(0i32);
    pub const CurrentPage: Self = Self(1i32);
    pub const CustomPageRange: Self = Self(2i32);
    pub const CurrentSelection: Self = Self(3i32);
}
impl ::core::marker::Copy for PrintContent {}
impl ::core::clone::Clone for PrintContent {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PrintContent {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for PrintContent {
    type Abi = Self;
}
impl ::core::fmt::Debug for PrintContent {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PrintContent").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PrintContent {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.UI.WebUI.PrintContent;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct ResumingEventHandler(pub ::windows_core::IUnknown);
impl ResumingEventHandler {
    pub fn new<F: FnMut(&::core::option::Option<::windows_core::IInspectable>) -> ::windows_core::Result<()> + ::core::marker::Send + 'static>(invoke: F) -> Self {
        let com = ResumingEventHandlerBox::<F> { vtable: &ResumingEventHandlerBox::<F>::VTABLE, count: ::windows_core::RefCount::new(1), invoke };
        unsafe { ::core::mem::transmute(::windows_core::alloc::boxed::Box::new(com)) }
    }
    pub fn Invoke<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::IInspectable>>(&self, sender: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Invoke)(::windows_core::Interface::as_raw(this), sender.into_param().abi()).ok() }
    }
}
#[repr(C)]
struct ResumingEventHandlerBox<F: FnMut(&::core::option::Option<::windows_core::IInspectable>) -> ::windows_core::Result<()> + ::core::marker::Send + 'static> {
    vtable: *const ResumingEventHandler_Vtbl,
    invoke: F,
    count: ::windows_core::RefCount,
}
impl<F: FnMut(&::core::option::Option<::windows_core::IInspectable>) -> ::windows_core::Result<()> + ::core::marker::Send + 'static> ResumingEventHandlerBox<F> {
    const VTABLE: ResumingEventHandler_Vtbl = ResumingEventHandler_Vtbl {
        base__: ::windows_core::IUnknownVtbl { QueryInterface: Self::QueryInterface, AddRef: Self::AddRef, Release: Self::Release },
        Invoke: Self::Invoke,
    };
    unsafe extern "system" fn QueryInterface(this: ::windows_core::RawPtr, iid: &::windows_core::GUID, interface: *mut *const ::core::ffi::c_void) -> ::windows_core::HRESULT {
        let this = this as *mut ::windows_core::RawPtr as *mut Self;
        *interface = if iid == &<ResumingEventHandler as ::windows_core::Interface>::IID || iid == &<::windows_core::IUnknown as ::windows_core::Interface>::IID || iid == &<::windows_core::IAgileObject as ::windows_core::Interface>::IID { &mut (*this).vtable as *mut _ as _ } else { ::core::ptr::null_mut() };
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
impl ::core::clone::Clone for ResumingEventHandler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ResumingEventHandler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ResumingEventHandler {}
impl ::core::fmt::Debug for ResumingEventHandler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ResumingEventHandler").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for ResumingEventHandler {
    type Vtable = ResumingEventHandler_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x26599ba9_a22d_4806_a728_acadc1d075fa);
}
unsafe impl ::windows_core::RuntimeType for ResumingEventHandler {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"{26599ba9-a22d-4806-a728-acadc1d075fa}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ResumingEventHandler_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub Invoke: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sender: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[cfg(feature = "ApplicationModel")]
#[repr(transparent)]
pub struct SuspendingDeferral(::windows_core::IUnknown);
#[cfg(feature = "ApplicationModel")]
impl SuspendingDeferral {
    #[cfg(feature = "ApplicationModel")]
    pub fn Complete(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Complete)(::windows_core::Interface::as_raw(this)).ok() }
    }
}
#[cfg(feature = "ApplicationModel")]
impl ::core::clone::Clone for SuspendingDeferral {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "ApplicationModel")]
impl ::core::cmp::PartialEq for SuspendingDeferral {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "ApplicationModel")]
impl ::core::cmp::Eq for SuspendingDeferral {}
#[cfg(feature = "ApplicationModel")]
impl ::core::fmt::Debug for SuspendingDeferral {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SuspendingDeferral").field(&self.0).finish()
    }
}
#[cfg(feature = "ApplicationModel")]
unsafe impl ::windows_core::RuntimeType for SuspendingDeferral {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.WebUI.SuspendingDeferral;{59140509-8bc9-4eb4-b636-dabdc4f46f66})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[cfg(feature = "ApplicationModel")]
unsafe impl ::windows_core::Interface for SuspendingDeferral {
    type Vtable = ::winrt_applicationmodel::ISuspendingDeferral_Vtbl;
    const IID: ::windows_core::GUID = <::winrt_applicationmodel::ISuspendingDeferral as ::windows_core::Interface>::IID;
}
#[cfg(feature = "ApplicationModel")]
impl ::windows_core::RuntimeName for SuspendingDeferral {
    const NAME: &'static str = "Windows.UI.WebUI.SuspendingDeferral";
}
#[cfg(feature = "ApplicationModel")]
impl ::core::convert::From<SuspendingDeferral> for ::windows_core::IUnknown {
    fn from(value: SuspendingDeferral) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel")]
impl ::core::convert::From<&SuspendingDeferral> for ::windows_core::IUnknown {
    fn from(value: &SuspendingDeferral) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for SuspendingDeferral {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "ApplicationModel")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a SuspendingDeferral {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "ApplicationModel")]
impl ::core::convert::From<SuspendingDeferral> for ::windows_core::IInspectable {
    fn from(value: SuspendingDeferral) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel")]
impl ::core::convert::From<&SuspendingDeferral> for ::windows_core::IInspectable {
    fn from(value: &SuspendingDeferral) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for SuspendingDeferral {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "ApplicationModel")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a SuspendingDeferral {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "ApplicationModel")]
impl ::core::convert::TryFrom<SuspendingDeferral> for ::winrt_applicationmodel::ISuspendingDeferral {
    type Error = ::windows_core::Error;
    fn try_from(value: SuspendingDeferral) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel")]
impl ::core::convert::TryFrom<&SuspendingDeferral> for ::winrt_applicationmodel::ISuspendingDeferral {
    type Error = ::windows_core::Error;
    fn try_from(value: &SuspendingDeferral) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::ISuspendingDeferral> for SuspendingDeferral {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::ISuspendingDeferral> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::ISuspendingDeferral> for &SuspendingDeferral {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::ISuspendingDeferral> {
        ::core::convert::TryInto::<::winrt_applicationmodel::ISuspendingDeferral>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "ApplicationModel")]
#[repr(transparent)]
pub struct SuspendingEventArgs(::windows_core::IUnknown);
#[cfg(feature = "ApplicationModel")]
impl SuspendingEventArgs {
    #[cfg(feature = "ApplicationModel")]
    pub fn SuspendingOperation(&self) -> ::windows_core::Result<::winrt_applicationmodel::SuspendingOperation> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SuspendingOperation)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_applicationmodel::SuspendingOperation>(result__)
        }
    }
}
#[cfg(feature = "ApplicationModel")]
impl ::core::clone::Clone for SuspendingEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "ApplicationModel")]
impl ::core::cmp::PartialEq for SuspendingEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "ApplicationModel")]
impl ::core::cmp::Eq for SuspendingEventArgs {}
#[cfg(feature = "ApplicationModel")]
impl ::core::fmt::Debug for SuspendingEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SuspendingEventArgs").field(&self.0).finish()
    }
}
#[cfg(feature = "ApplicationModel")]
unsafe impl ::windows_core::RuntimeType for SuspendingEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.WebUI.SuspendingEventArgs;{96061c05-2dba-4d08-b0bd-2b30a131c6aa})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[cfg(feature = "ApplicationModel")]
unsafe impl ::windows_core::Interface for SuspendingEventArgs {
    type Vtable = ::winrt_applicationmodel::ISuspendingEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <::winrt_applicationmodel::ISuspendingEventArgs as ::windows_core::Interface>::IID;
}
#[cfg(feature = "ApplicationModel")]
impl ::windows_core::RuntimeName for SuspendingEventArgs {
    const NAME: &'static str = "Windows.UI.WebUI.SuspendingEventArgs";
}
#[cfg(feature = "ApplicationModel")]
impl ::core::convert::From<SuspendingEventArgs> for ::windows_core::IUnknown {
    fn from(value: SuspendingEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel")]
impl ::core::convert::From<&SuspendingEventArgs> for ::windows_core::IUnknown {
    fn from(value: &SuspendingEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for SuspendingEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "ApplicationModel")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a SuspendingEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "ApplicationModel")]
impl ::core::convert::From<SuspendingEventArgs> for ::windows_core::IInspectable {
    fn from(value: SuspendingEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel")]
impl ::core::convert::From<&SuspendingEventArgs> for ::windows_core::IInspectable {
    fn from(value: &SuspendingEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for SuspendingEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "ApplicationModel")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a SuspendingEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "ApplicationModel")]
impl ::core::convert::TryFrom<SuspendingEventArgs> for ::winrt_applicationmodel::ISuspendingEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: SuspendingEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel")]
impl ::core::convert::TryFrom<&SuspendingEventArgs> for ::winrt_applicationmodel::ISuspendingEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &SuspendingEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::ISuspendingEventArgs> for SuspendingEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::ISuspendingEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::ISuspendingEventArgs> for &SuspendingEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::ISuspendingEventArgs> {
        ::core::convert::TryInto::<::winrt_applicationmodel::ISuspendingEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "ApplicationModel")]
#[repr(transparent)]
pub struct SuspendingEventHandler(pub ::windows_core::IUnknown);
#[cfg(feature = "ApplicationModel")]
impl SuspendingEventHandler {
    pub fn new<F: FnMut(&::core::option::Option<::windows_core::IInspectable>, &::core::option::Option<::winrt_applicationmodel::ISuspendingEventArgs>) -> ::windows_core::Result<()> + ::core::marker::Send + 'static>(invoke: F) -> Self {
        let com = SuspendingEventHandlerBox::<F> { vtable: &SuspendingEventHandlerBox::<F>::VTABLE, count: ::windows_core::RefCount::new(1), invoke };
        unsafe { ::core::mem::transmute(::windows_core::alloc::boxed::Box::new(com)) }
    }
    #[cfg(feature = "ApplicationModel")]
    pub fn Invoke<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::IInspectable>, Param1: ::windows_core::IntoParam<'a, ::winrt_applicationmodel::ISuspendingEventArgs>>(&self, sender: Param0, e: Param1) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Invoke)(::windows_core::Interface::as_raw(this), sender.into_param().abi(), e.into_param().abi()).ok() }
    }
}
#[cfg(feature = "ApplicationModel")]
#[repr(C)]
struct SuspendingEventHandlerBox<F: FnMut(&::core::option::Option<::windows_core::IInspectable>, &::core::option::Option<::winrt_applicationmodel::ISuspendingEventArgs>) -> ::windows_core::Result<()> + ::core::marker::Send + 'static> {
    vtable: *const SuspendingEventHandler_Vtbl,
    invoke: F,
    count: ::windows_core::RefCount,
}
#[cfg(feature = "ApplicationModel")]
impl<F: FnMut(&::core::option::Option<::windows_core::IInspectable>, &::core::option::Option<::winrt_applicationmodel::ISuspendingEventArgs>) -> ::windows_core::Result<()> + ::core::marker::Send + 'static> SuspendingEventHandlerBox<F> {
    const VTABLE: SuspendingEventHandler_Vtbl = SuspendingEventHandler_Vtbl {
        base__: ::windows_core::IUnknownVtbl { QueryInterface: Self::QueryInterface, AddRef: Self::AddRef, Release: Self::Release },
        Invoke: Self::Invoke,
    };
    unsafe extern "system" fn QueryInterface(this: ::windows_core::RawPtr, iid: &::windows_core::GUID, interface: *mut *const ::core::ffi::c_void) -> ::windows_core::HRESULT {
        let this = this as *mut ::windows_core::RawPtr as *mut Self;
        *interface = if iid == &<SuspendingEventHandler as ::windows_core::Interface>::IID || iid == &<::windows_core::IUnknown as ::windows_core::Interface>::IID || iid == &<::windows_core::IAgileObject as ::windows_core::Interface>::IID { &mut (*this).vtable as *mut _ as _ } else { ::core::ptr::null_mut() };
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
#[cfg(feature = "ApplicationModel")]
impl ::core::clone::Clone for SuspendingEventHandler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "ApplicationModel")]
impl ::core::cmp::PartialEq for SuspendingEventHandler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "ApplicationModel")]
impl ::core::cmp::Eq for SuspendingEventHandler {}
#[cfg(feature = "ApplicationModel")]
impl ::core::fmt::Debug for SuspendingEventHandler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SuspendingEventHandler").field(&self.0).finish()
    }
}
#[cfg(feature = "ApplicationModel")]
unsafe impl ::windows_core::Interface for SuspendingEventHandler {
    type Vtable = SuspendingEventHandler_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x509c429c_78e2_4883_abc8_8960dcde1b5c);
}
#[cfg(feature = "ApplicationModel")]
unsafe impl ::windows_core::RuntimeType for SuspendingEventHandler {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"{509c429c-78e2-4883-abc8-8960dcde1b5c}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[cfg(feature = "ApplicationModel")]
#[repr(C)]
#[doc(hidden)]
pub struct SuspendingEventHandler_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    #[cfg(feature = "ApplicationModel")]
    pub Invoke: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sender: *mut ::core::ffi::c_void, e: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "ApplicationModel"))]
    Invoke: usize,
}
#[cfg(feature = "ApplicationModel")]
#[repr(transparent)]
pub struct SuspendingOperation(::windows_core::IUnknown);
#[cfg(feature = "ApplicationModel")]
impl SuspendingOperation {
    #[cfg(feature = "ApplicationModel")]
    pub fn GetDeferral(&self) -> ::windows_core::Result<::winrt_applicationmodel::SuspendingDeferral> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetDeferral)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_applicationmodel::SuspendingDeferral>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel")]
    pub fn Deadline(&self) -> ::windows_core::Result<::winrt_foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::DateTime>::zeroed();
            (::windows_core::Interface::vtable(this).Deadline)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::DateTime>(result__)
        }
    }
}
#[cfg(feature = "ApplicationModel")]
impl ::core::clone::Clone for SuspendingOperation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "ApplicationModel")]
impl ::core::cmp::PartialEq for SuspendingOperation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "ApplicationModel")]
impl ::core::cmp::Eq for SuspendingOperation {}
#[cfg(feature = "ApplicationModel")]
impl ::core::fmt::Debug for SuspendingOperation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SuspendingOperation").field(&self.0).finish()
    }
}
#[cfg(feature = "ApplicationModel")]
unsafe impl ::windows_core::RuntimeType for SuspendingOperation {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.WebUI.SuspendingOperation;{9da4ca41-20e1-4e9b-9f65-a9f435340c3a})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[cfg(feature = "ApplicationModel")]
unsafe impl ::windows_core::Interface for SuspendingOperation {
    type Vtable = ::winrt_applicationmodel::ISuspendingOperation_Vtbl;
    const IID: ::windows_core::GUID = <::winrt_applicationmodel::ISuspendingOperation as ::windows_core::Interface>::IID;
}
#[cfg(feature = "ApplicationModel")]
impl ::windows_core::RuntimeName for SuspendingOperation {
    const NAME: &'static str = "Windows.UI.WebUI.SuspendingOperation";
}
#[cfg(feature = "ApplicationModel")]
impl ::core::convert::From<SuspendingOperation> for ::windows_core::IUnknown {
    fn from(value: SuspendingOperation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel")]
impl ::core::convert::From<&SuspendingOperation> for ::windows_core::IUnknown {
    fn from(value: &SuspendingOperation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for SuspendingOperation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "ApplicationModel")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a SuspendingOperation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "ApplicationModel")]
impl ::core::convert::From<SuspendingOperation> for ::windows_core::IInspectable {
    fn from(value: SuspendingOperation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel")]
impl ::core::convert::From<&SuspendingOperation> for ::windows_core::IInspectable {
    fn from(value: &SuspendingOperation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for SuspendingOperation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "ApplicationModel")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a SuspendingOperation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "ApplicationModel")]
impl ::core::convert::TryFrom<SuspendingOperation> for ::winrt_applicationmodel::ISuspendingOperation {
    type Error = ::windows_core::Error;
    fn try_from(value: SuspendingOperation) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel")]
impl ::core::convert::TryFrom<&SuspendingOperation> for ::winrt_applicationmodel::ISuspendingOperation {
    type Error = ::windows_core::Error;
    fn try_from(value: &SuspendingOperation) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::ISuspendingOperation> for SuspendingOperation {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::ISuspendingOperation> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::ISuspendingOperation> for &SuspendingOperation {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::ISuspendingOperation> {
        ::core::convert::TryInto::<::winrt_applicationmodel::ISuspendingOperation>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
pub struct WebUIApplication;
impl WebUIApplication {
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Activated<'a, Param0: ::windows_core::IntoParam<'a, ActivatedEventHandler>>(handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        Self::IWebUIActivationStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).Activated)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        })
    }
    pub fn RemoveActivated<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(token: Param0) -> ::windows_core::Result<()> {
        Self::IWebUIActivationStatics(|this| unsafe { (::windows_core::Interface::vtable(this).RemoveActivated)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() })
    }
    #[cfg(feature = "ApplicationModel")]
    pub fn Suspending<'a, Param0: ::windows_core::IntoParam<'a, SuspendingEventHandler>>(handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        Self::IWebUIActivationStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).Suspending)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        })
    }
    pub fn RemoveSuspending<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(token: Param0) -> ::windows_core::Result<()> {
        Self::IWebUIActivationStatics(|this| unsafe { (::windows_core::Interface::vtable(this).RemoveSuspending)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() })
    }
    pub fn Resuming<'a, Param0: ::windows_core::IntoParam<'a, ResumingEventHandler>>(handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        Self::IWebUIActivationStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).Resuming)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        })
    }
    pub fn RemoveResuming<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(token: Param0) -> ::windows_core::Result<()> {
        Self::IWebUIActivationStatics(|this| unsafe { (::windows_core::Interface::vtable(this).RemoveResuming)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() })
    }
    pub fn Navigated<'a, Param0: ::windows_core::IntoParam<'a, NavigatedEventHandler>>(handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        Self::IWebUIActivationStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).Navigated)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        })
    }
    pub fn RemoveNavigated<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(token: Param0) -> ::windows_core::Result<()> {
        Self::IWebUIActivationStatics(|this| unsafe { (::windows_core::Interface::vtable(this).RemoveNavigated)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() })
    }
    #[cfg(feature = "ApplicationModel")]
    pub fn LeavingBackground<'a, Param0: ::windows_core::IntoParam<'a, LeavingBackgroundEventHandler>>(handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        Self::IWebUIActivationStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).LeavingBackground)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        })
    }
    pub fn RemoveLeavingBackground<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(token: Param0) -> ::windows_core::Result<()> {
        Self::IWebUIActivationStatics2(|this| unsafe { (::windows_core::Interface::vtable(this).RemoveLeavingBackground)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() })
    }
    #[cfg(feature = "ApplicationModel")]
    pub fn EnteredBackground<'a, Param0: ::windows_core::IntoParam<'a, EnteredBackgroundEventHandler>>(handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        Self::IWebUIActivationStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).EnteredBackground)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        })
    }
    pub fn RemoveEnteredBackground<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(token: Param0) -> ::windows_core::Result<()> {
        Self::IWebUIActivationStatics2(|this| unsafe { (::windows_core::Interface::vtable(this).RemoveEnteredBackground)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() })
    }
    pub fn EnablePrelaunch(value: bool) -> ::windows_core::Result<()> {
        Self::IWebUIActivationStatics2(|this| unsafe { (::windows_core::Interface::vtable(this).EnablePrelaunch)(::windows_core::Interface::as_raw(this), value).ok() })
    }
    #[cfg(feature = "ApplicationModel_Core")]
    pub fn RequestRestartAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(launcharguments: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<::winrt_applicationmodel::Core::AppRestartFailureReason>> {
        Self::IWebUIActivationStatics3(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RequestRestartAsync)(::windows_core::Interface::as_raw(this), launcharguments.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<::winrt_applicationmodel::Core::AppRestartFailureReason>>(result__)
        })
    }
    #[cfg(all(feature = "ApplicationModel_Core", feature = "System"))]
    pub fn RequestRestartForUserAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_system::User>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(user: Param0, launcharguments: Param1) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<::winrt_applicationmodel::Core::AppRestartFailureReason>> {
        Self::IWebUIActivationStatics3(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RequestRestartForUserAsync)(::windows_core::Interface::as_raw(this), user.into_param().abi(), launcharguments.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<::winrt_applicationmodel::Core::AppRestartFailureReason>>(result__)
        })
    }
    pub fn NewWebUIViewCreated<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventHandler<NewWebUIViewCreatedEventArgs>>>(handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        Self::IWebUIActivationStatics4(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).NewWebUIViewCreated)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        })
    }
    pub fn RemoveNewWebUIViewCreated<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(token: Param0) -> ::windows_core::Result<()> {
        Self::IWebUIActivationStatics4(|this| unsafe { (::windows_core::Interface::vtable(this).RemoveNewWebUIViewCreated)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() })
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn BackgroundActivated<'a, Param0: ::windows_core::IntoParam<'a, BackgroundActivatedEventHandler>>(handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        Self::IWebUIActivationStatics4(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).BackgroundActivated)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        })
    }
    pub fn RemoveBackgroundActivated<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(token: Param0) -> ::windows_core::Result<()> {
        Self::IWebUIActivationStatics4(|this| unsafe { (::windows_core::Interface::vtable(this).RemoveBackgroundActivated)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() })
    }
    pub fn IWebUIActivationStatics<R, F: FnOnce(&IWebUIActivationStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<WebUIApplication, IWebUIActivationStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IWebUIActivationStatics2<R, F: FnOnce(&IWebUIActivationStatics2) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<WebUIApplication, IWebUIActivationStatics2> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IWebUIActivationStatics3<R, F: FnOnce(&IWebUIActivationStatics3) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<WebUIApplication, IWebUIActivationStatics3> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IWebUIActivationStatics4<R, F: FnOnce(&IWebUIActivationStatics4) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<WebUIApplication, IWebUIActivationStatics4> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows_core::RuntimeName for WebUIApplication {
    const NAME: &'static str = "Windows.UI.WebUI.WebUIApplication";
}
#[cfg(feature = "ApplicationModel_Activation")]
#[repr(transparent)]
pub struct WebUIAppointmentsProviderAddAppointmentActivatedEventArgs(::windows_core::IUnknown);
#[cfg(feature = "ApplicationModel_Activation")]
impl WebUIAppointmentsProviderAddAppointmentActivatedEventArgs {
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Kind(&self) -> ::windows_core::Result<::winrt_applicationmodel::Activation::ActivationKind> {
        let this = &::windows_core::Interface::cast::<::winrt_applicationmodel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_applicationmodel::Activation::ActivationKind>::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_applicationmodel::Activation::ActivationKind>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<::winrt_applicationmodel::Activation::ApplicationExecutionState> {
        let this = &::windows_core::Interface::cast::<::winrt_applicationmodel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_applicationmodel::Activation::ApplicationExecutionState>::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_applicationmodel::Activation::ApplicationExecutionState>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn SplashScreen(&self) -> ::windows_core::Result<::winrt_applicationmodel::Activation::SplashScreen> {
        let this = &::windows_core::Interface::cast::<::winrt_applicationmodel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_applicationmodel::Activation::SplashScreen>(result__)
        }
    }
    pub fn ActivatedOperation(&self) -> ::windows_core::Result<ActivatedOperation> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgsDeferral>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ActivatedOperation)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivatedOperation>(result__)
        }
    }
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "System"))]
    pub fn User(&self) -> ::windows_core::Result<::winrt_system::User> {
        let this = &::windows_core::Interface::cast::<::winrt_applicationmodel::Activation::IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).User)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_system::User>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Verb(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<::winrt_applicationmodel::Activation::IAppointmentsProviderActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Verb)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "ApplicationModel_Appointments_AppointmentsProvider"))]
    pub fn AddAppointmentOperation(&self) -> ::windows_core::Result<::winrt_applicationmodel::Appointments::AppointmentsProvider::AddAppointmentOperation> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).AddAppointmentOperation)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_applicationmodel::Appointments::AppointmentsProvider::AddAppointmentOperation>(result__)
        }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::clone::Clone for WebUIAppointmentsProviderAddAppointmentActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::cmp::PartialEq for WebUIAppointmentsProviderAddAppointmentActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::cmp::Eq for WebUIAppointmentsProviderAddAppointmentActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::fmt::Debug for WebUIAppointmentsProviderAddAppointmentActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WebUIAppointmentsProviderAddAppointmentActivatedEventArgs").field(&self.0).finish()
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows_core::RuntimeType for WebUIAppointmentsProviderAddAppointmentActivatedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.WebUI.WebUIAppointmentsProviderAddAppointmentActivatedEventArgs;{a2861367-cee5-4e4d-9ed7-41c34ec18b02})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows_core::Interface for WebUIAppointmentsProviderAddAppointmentActivatedEventArgs {
    type Vtable = ::winrt_applicationmodel::Activation::IAppointmentsProviderAddAppointmentActivatedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <::winrt_applicationmodel::Activation::IAppointmentsProviderAddAppointmentActivatedEventArgs as ::windows_core::Interface>::IID;
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::RuntimeName for WebUIAppointmentsProviderAddAppointmentActivatedEventArgs {
    const NAME: &'static str = "Windows.UI.WebUI.WebUIAppointmentsProviderAddAppointmentActivatedEventArgs";
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<WebUIAppointmentsProviderAddAppointmentActivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: WebUIAppointmentsProviderAddAppointmentActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<&WebUIAppointmentsProviderAddAppointmentActivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &WebUIAppointmentsProviderAddAppointmentActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for WebUIAppointmentsProviderAddAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a WebUIAppointmentsProviderAddAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<WebUIAppointmentsProviderAddAppointmentActivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: WebUIAppointmentsProviderAddAppointmentActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<&WebUIAppointmentsProviderAddAppointmentActivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &WebUIAppointmentsProviderAddAppointmentActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for WebUIAppointmentsProviderAddAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a WebUIAppointmentsProviderAddAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIAppointmentsProviderAddAppointmentActivatedEventArgs> for ::winrt_applicationmodel::Activation::IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: WebUIAppointmentsProviderAddAppointmentActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIAppointmentsProviderAddAppointmentActivatedEventArgs> for ::winrt_applicationmodel::Activation::IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &WebUIAppointmentsProviderAddAppointmentActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgs> for WebUIAppointmentsProviderAddAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgs> for &WebUIAppointmentsProviderAddAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgs> {
        ::core::convert::TryInto::<::winrt_applicationmodel::Activation::IActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIAppointmentsProviderAddAppointmentActivatedEventArgs> for IActivatedEventArgsDeferral {
    type Error = ::windows_core::Error;
    fn try_from(value: WebUIAppointmentsProviderAddAppointmentActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIAppointmentsProviderAddAppointmentActivatedEventArgs> for IActivatedEventArgsDeferral {
    type Error = ::windows_core::Error;
    fn try_from(value: &WebUIAppointmentsProviderAddAppointmentActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgsDeferral> for WebUIAppointmentsProviderAddAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgsDeferral> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgsDeferral> for &WebUIAppointmentsProviderAddAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgsDeferral> {
        ::core::convert::TryInto::<IActivatedEventArgsDeferral>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIAppointmentsProviderAddAppointmentActivatedEventArgs> for ::winrt_applicationmodel::Activation::IActivatedEventArgsWithUser {
    type Error = ::windows_core::Error;
    fn try_from(value: WebUIAppointmentsProviderAddAppointmentActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIAppointmentsProviderAddAppointmentActivatedEventArgs> for ::winrt_applicationmodel::Activation::IActivatedEventArgsWithUser {
    type Error = ::windows_core::Error;
    fn try_from(value: &WebUIAppointmentsProviderAddAppointmentActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgsWithUser> for WebUIAppointmentsProviderAddAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgsWithUser> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgsWithUser> for &WebUIAppointmentsProviderAddAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgsWithUser> {
        ::core::convert::TryInto::<::winrt_applicationmodel::Activation::IActivatedEventArgsWithUser>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIAppointmentsProviderAddAppointmentActivatedEventArgs> for ::winrt_applicationmodel::Activation::IAppointmentsProviderActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: WebUIAppointmentsProviderAddAppointmentActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIAppointmentsProviderAddAppointmentActivatedEventArgs> for ::winrt_applicationmodel::Activation::IAppointmentsProviderActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &WebUIAppointmentsProviderAddAppointmentActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IAppointmentsProviderActivatedEventArgs> for WebUIAppointmentsProviderAddAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IAppointmentsProviderActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IAppointmentsProviderActivatedEventArgs> for &WebUIAppointmentsProviderAddAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IAppointmentsProviderActivatedEventArgs> {
        ::core::convert::TryInto::<::winrt_applicationmodel::Activation::IAppointmentsProviderActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIAppointmentsProviderAddAppointmentActivatedEventArgs> for ::winrt_applicationmodel::Activation::IAppointmentsProviderAddAppointmentActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: WebUIAppointmentsProviderAddAppointmentActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIAppointmentsProviderAddAppointmentActivatedEventArgs> for ::winrt_applicationmodel::Activation::IAppointmentsProviderAddAppointmentActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &WebUIAppointmentsProviderAddAppointmentActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IAppointmentsProviderAddAppointmentActivatedEventArgs> for WebUIAppointmentsProviderAddAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IAppointmentsProviderAddAppointmentActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IAppointmentsProviderAddAppointmentActivatedEventArgs> for &WebUIAppointmentsProviderAddAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IAppointmentsProviderAddAppointmentActivatedEventArgs> {
        ::core::convert::TryInto::<::winrt_applicationmodel::Activation::IAppointmentsProviderAddAppointmentActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
#[repr(transparent)]
pub struct WebUIAppointmentsProviderRemoveAppointmentActivatedEventArgs(::windows_core::IUnknown);
#[cfg(feature = "ApplicationModel_Activation")]
impl WebUIAppointmentsProviderRemoveAppointmentActivatedEventArgs {
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Kind(&self) -> ::windows_core::Result<::winrt_applicationmodel::Activation::ActivationKind> {
        let this = &::windows_core::Interface::cast::<::winrt_applicationmodel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_applicationmodel::Activation::ActivationKind>::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_applicationmodel::Activation::ActivationKind>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<::winrt_applicationmodel::Activation::ApplicationExecutionState> {
        let this = &::windows_core::Interface::cast::<::winrt_applicationmodel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_applicationmodel::Activation::ApplicationExecutionState>::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_applicationmodel::Activation::ApplicationExecutionState>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn SplashScreen(&self) -> ::windows_core::Result<::winrt_applicationmodel::Activation::SplashScreen> {
        let this = &::windows_core::Interface::cast::<::winrt_applicationmodel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_applicationmodel::Activation::SplashScreen>(result__)
        }
    }
    pub fn ActivatedOperation(&self) -> ::windows_core::Result<ActivatedOperation> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgsDeferral>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ActivatedOperation)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivatedOperation>(result__)
        }
    }
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "System"))]
    pub fn User(&self) -> ::windows_core::Result<::winrt_system::User> {
        let this = &::windows_core::Interface::cast::<::winrt_applicationmodel::Activation::IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).User)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_system::User>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Verb(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<::winrt_applicationmodel::Activation::IAppointmentsProviderActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Verb)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "ApplicationModel_Appointments_AppointmentsProvider"))]
    pub fn RemoveAppointmentOperation(&self) -> ::windows_core::Result<::winrt_applicationmodel::Appointments::AppointmentsProvider::RemoveAppointmentOperation> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RemoveAppointmentOperation)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_applicationmodel::Appointments::AppointmentsProvider::RemoveAppointmentOperation>(result__)
        }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::clone::Clone for WebUIAppointmentsProviderRemoveAppointmentActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::cmp::PartialEq for WebUIAppointmentsProviderRemoveAppointmentActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::cmp::Eq for WebUIAppointmentsProviderRemoveAppointmentActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::fmt::Debug for WebUIAppointmentsProviderRemoveAppointmentActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WebUIAppointmentsProviderRemoveAppointmentActivatedEventArgs").field(&self.0).finish()
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows_core::RuntimeType for WebUIAppointmentsProviderRemoveAppointmentActivatedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.WebUI.WebUIAppointmentsProviderRemoveAppointmentActivatedEventArgs;{751f3ab8-0b8e-451c-9f15-966e699bac25})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows_core::Interface for WebUIAppointmentsProviderRemoveAppointmentActivatedEventArgs {
    type Vtable = ::winrt_applicationmodel::Activation::IAppointmentsProviderRemoveAppointmentActivatedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <::winrt_applicationmodel::Activation::IAppointmentsProviderRemoveAppointmentActivatedEventArgs as ::windows_core::Interface>::IID;
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::RuntimeName for WebUIAppointmentsProviderRemoveAppointmentActivatedEventArgs {
    const NAME: &'static str = "Windows.UI.WebUI.WebUIAppointmentsProviderRemoveAppointmentActivatedEventArgs";
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<WebUIAppointmentsProviderRemoveAppointmentActivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: WebUIAppointmentsProviderRemoveAppointmentActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<&WebUIAppointmentsProviderRemoveAppointmentActivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &WebUIAppointmentsProviderRemoveAppointmentActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for WebUIAppointmentsProviderRemoveAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a WebUIAppointmentsProviderRemoveAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<WebUIAppointmentsProviderRemoveAppointmentActivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: WebUIAppointmentsProviderRemoveAppointmentActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<&WebUIAppointmentsProviderRemoveAppointmentActivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &WebUIAppointmentsProviderRemoveAppointmentActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for WebUIAppointmentsProviderRemoveAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a WebUIAppointmentsProviderRemoveAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIAppointmentsProviderRemoveAppointmentActivatedEventArgs> for ::winrt_applicationmodel::Activation::IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: WebUIAppointmentsProviderRemoveAppointmentActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIAppointmentsProviderRemoveAppointmentActivatedEventArgs> for ::winrt_applicationmodel::Activation::IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &WebUIAppointmentsProviderRemoveAppointmentActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgs> for WebUIAppointmentsProviderRemoveAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgs> for &WebUIAppointmentsProviderRemoveAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgs> {
        ::core::convert::TryInto::<::winrt_applicationmodel::Activation::IActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIAppointmentsProviderRemoveAppointmentActivatedEventArgs> for IActivatedEventArgsDeferral {
    type Error = ::windows_core::Error;
    fn try_from(value: WebUIAppointmentsProviderRemoveAppointmentActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIAppointmentsProviderRemoveAppointmentActivatedEventArgs> for IActivatedEventArgsDeferral {
    type Error = ::windows_core::Error;
    fn try_from(value: &WebUIAppointmentsProviderRemoveAppointmentActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgsDeferral> for WebUIAppointmentsProviderRemoveAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgsDeferral> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgsDeferral> for &WebUIAppointmentsProviderRemoveAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgsDeferral> {
        ::core::convert::TryInto::<IActivatedEventArgsDeferral>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIAppointmentsProviderRemoveAppointmentActivatedEventArgs> for ::winrt_applicationmodel::Activation::IActivatedEventArgsWithUser {
    type Error = ::windows_core::Error;
    fn try_from(value: WebUIAppointmentsProviderRemoveAppointmentActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIAppointmentsProviderRemoveAppointmentActivatedEventArgs> for ::winrt_applicationmodel::Activation::IActivatedEventArgsWithUser {
    type Error = ::windows_core::Error;
    fn try_from(value: &WebUIAppointmentsProviderRemoveAppointmentActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgsWithUser> for WebUIAppointmentsProviderRemoveAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgsWithUser> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgsWithUser> for &WebUIAppointmentsProviderRemoveAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgsWithUser> {
        ::core::convert::TryInto::<::winrt_applicationmodel::Activation::IActivatedEventArgsWithUser>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIAppointmentsProviderRemoveAppointmentActivatedEventArgs> for ::winrt_applicationmodel::Activation::IAppointmentsProviderActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: WebUIAppointmentsProviderRemoveAppointmentActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIAppointmentsProviderRemoveAppointmentActivatedEventArgs> for ::winrt_applicationmodel::Activation::IAppointmentsProviderActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &WebUIAppointmentsProviderRemoveAppointmentActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IAppointmentsProviderActivatedEventArgs> for WebUIAppointmentsProviderRemoveAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IAppointmentsProviderActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IAppointmentsProviderActivatedEventArgs> for &WebUIAppointmentsProviderRemoveAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IAppointmentsProviderActivatedEventArgs> {
        ::core::convert::TryInto::<::winrt_applicationmodel::Activation::IAppointmentsProviderActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIAppointmentsProviderRemoveAppointmentActivatedEventArgs> for ::winrt_applicationmodel::Activation::IAppointmentsProviderRemoveAppointmentActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: WebUIAppointmentsProviderRemoveAppointmentActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIAppointmentsProviderRemoveAppointmentActivatedEventArgs> for ::winrt_applicationmodel::Activation::IAppointmentsProviderRemoveAppointmentActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &WebUIAppointmentsProviderRemoveAppointmentActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IAppointmentsProviderRemoveAppointmentActivatedEventArgs> for WebUIAppointmentsProviderRemoveAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IAppointmentsProviderRemoveAppointmentActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IAppointmentsProviderRemoveAppointmentActivatedEventArgs> for &WebUIAppointmentsProviderRemoveAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IAppointmentsProviderRemoveAppointmentActivatedEventArgs> {
        ::core::convert::TryInto::<::winrt_applicationmodel::Activation::IAppointmentsProviderRemoveAppointmentActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
#[repr(transparent)]
pub struct WebUIAppointmentsProviderReplaceAppointmentActivatedEventArgs(::windows_core::IUnknown);
#[cfg(feature = "ApplicationModel_Activation")]
impl WebUIAppointmentsProviderReplaceAppointmentActivatedEventArgs {
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Kind(&self) -> ::windows_core::Result<::winrt_applicationmodel::Activation::ActivationKind> {
        let this = &::windows_core::Interface::cast::<::winrt_applicationmodel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_applicationmodel::Activation::ActivationKind>::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_applicationmodel::Activation::ActivationKind>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<::winrt_applicationmodel::Activation::ApplicationExecutionState> {
        let this = &::windows_core::Interface::cast::<::winrt_applicationmodel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_applicationmodel::Activation::ApplicationExecutionState>::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_applicationmodel::Activation::ApplicationExecutionState>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn SplashScreen(&self) -> ::windows_core::Result<::winrt_applicationmodel::Activation::SplashScreen> {
        let this = &::windows_core::Interface::cast::<::winrt_applicationmodel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_applicationmodel::Activation::SplashScreen>(result__)
        }
    }
    pub fn ActivatedOperation(&self) -> ::windows_core::Result<ActivatedOperation> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgsDeferral>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ActivatedOperation)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivatedOperation>(result__)
        }
    }
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "System"))]
    pub fn User(&self) -> ::windows_core::Result<::winrt_system::User> {
        let this = &::windows_core::Interface::cast::<::winrt_applicationmodel::Activation::IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).User)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_system::User>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Verb(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<::winrt_applicationmodel::Activation::IAppointmentsProviderActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Verb)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "ApplicationModel_Appointments_AppointmentsProvider"))]
    pub fn ReplaceAppointmentOperation(&self) -> ::windows_core::Result<::winrt_applicationmodel::Appointments::AppointmentsProvider::ReplaceAppointmentOperation> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ReplaceAppointmentOperation)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_applicationmodel::Appointments::AppointmentsProvider::ReplaceAppointmentOperation>(result__)
        }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::clone::Clone for WebUIAppointmentsProviderReplaceAppointmentActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::cmp::PartialEq for WebUIAppointmentsProviderReplaceAppointmentActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::cmp::Eq for WebUIAppointmentsProviderReplaceAppointmentActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::fmt::Debug for WebUIAppointmentsProviderReplaceAppointmentActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WebUIAppointmentsProviderReplaceAppointmentActivatedEventArgs").field(&self.0).finish()
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows_core::RuntimeType for WebUIAppointmentsProviderReplaceAppointmentActivatedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.WebUI.WebUIAppointmentsProviderReplaceAppointmentActivatedEventArgs;{1551b7d4-a981-4067-8a62-0524e4ade121})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows_core::Interface for WebUIAppointmentsProviderReplaceAppointmentActivatedEventArgs {
    type Vtable = ::winrt_applicationmodel::Activation::IAppointmentsProviderReplaceAppointmentActivatedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <::winrt_applicationmodel::Activation::IAppointmentsProviderReplaceAppointmentActivatedEventArgs as ::windows_core::Interface>::IID;
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::RuntimeName for WebUIAppointmentsProviderReplaceAppointmentActivatedEventArgs {
    const NAME: &'static str = "Windows.UI.WebUI.WebUIAppointmentsProviderReplaceAppointmentActivatedEventArgs";
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<WebUIAppointmentsProviderReplaceAppointmentActivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: WebUIAppointmentsProviderReplaceAppointmentActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<&WebUIAppointmentsProviderReplaceAppointmentActivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &WebUIAppointmentsProviderReplaceAppointmentActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for WebUIAppointmentsProviderReplaceAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a WebUIAppointmentsProviderReplaceAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<WebUIAppointmentsProviderReplaceAppointmentActivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: WebUIAppointmentsProviderReplaceAppointmentActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<&WebUIAppointmentsProviderReplaceAppointmentActivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &WebUIAppointmentsProviderReplaceAppointmentActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for WebUIAppointmentsProviderReplaceAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a WebUIAppointmentsProviderReplaceAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIAppointmentsProviderReplaceAppointmentActivatedEventArgs> for ::winrt_applicationmodel::Activation::IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: WebUIAppointmentsProviderReplaceAppointmentActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIAppointmentsProviderReplaceAppointmentActivatedEventArgs> for ::winrt_applicationmodel::Activation::IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &WebUIAppointmentsProviderReplaceAppointmentActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgs> for WebUIAppointmentsProviderReplaceAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgs> for &WebUIAppointmentsProviderReplaceAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgs> {
        ::core::convert::TryInto::<::winrt_applicationmodel::Activation::IActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIAppointmentsProviderReplaceAppointmentActivatedEventArgs> for IActivatedEventArgsDeferral {
    type Error = ::windows_core::Error;
    fn try_from(value: WebUIAppointmentsProviderReplaceAppointmentActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIAppointmentsProviderReplaceAppointmentActivatedEventArgs> for IActivatedEventArgsDeferral {
    type Error = ::windows_core::Error;
    fn try_from(value: &WebUIAppointmentsProviderReplaceAppointmentActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgsDeferral> for WebUIAppointmentsProviderReplaceAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgsDeferral> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgsDeferral> for &WebUIAppointmentsProviderReplaceAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgsDeferral> {
        ::core::convert::TryInto::<IActivatedEventArgsDeferral>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIAppointmentsProviderReplaceAppointmentActivatedEventArgs> for ::winrt_applicationmodel::Activation::IActivatedEventArgsWithUser {
    type Error = ::windows_core::Error;
    fn try_from(value: WebUIAppointmentsProviderReplaceAppointmentActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIAppointmentsProviderReplaceAppointmentActivatedEventArgs> for ::winrt_applicationmodel::Activation::IActivatedEventArgsWithUser {
    type Error = ::windows_core::Error;
    fn try_from(value: &WebUIAppointmentsProviderReplaceAppointmentActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgsWithUser> for WebUIAppointmentsProviderReplaceAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgsWithUser> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgsWithUser> for &WebUIAppointmentsProviderReplaceAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgsWithUser> {
        ::core::convert::TryInto::<::winrt_applicationmodel::Activation::IActivatedEventArgsWithUser>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIAppointmentsProviderReplaceAppointmentActivatedEventArgs> for ::winrt_applicationmodel::Activation::IAppointmentsProviderActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: WebUIAppointmentsProviderReplaceAppointmentActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIAppointmentsProviderReplaceAppointmentActivatedEventArgs> for ::winrt_applicationmodel::Activation::IAppointmentsProviderActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &WebUIAppointmentsProviderReplaceAppointmentActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IAppointmentsProviderActivatedEventArgs> for WebUIAppointmentsProviderReplaceAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IAppointmentsProviderActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IAppointmentsProviderActivatedEventArgs> for &WebUIAppointmentsProviderReplaceAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IAppointmentsProviderActivatedEventArgs> {
        ::core::convert::TryInto::<::winrt_applicationmodel::Activation::IAppointmentsProviderActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIAppointmentsProviderReplaceAppointmentActivatedEventArgs> for ::winrt_applicationmodel::Activation::IAppointmentsProviderReplaceAppointmentActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: WebUIAppointmentsProviderReplaceAppointmentActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIAppointmentsProviderReplaceAppointmentActivatedEventArgs> for ::winrt_applicationmodel::Activation::IAppointmentsProviderReplaceAppointmentActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &WebUIAppointmentsProviderReplaceAppointmentActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IAppointmentsProviderReplaceAppointmentActivatedEventArgs> for WebUIAppointmentsProviderReplaceAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IAppointmentsProviderReplaceAppointmentActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IAppointmentsProviderReplaceAppointmentActivatedEventArgs> for &WebUIAppointmentsProviderReplaceAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IAppointmentsProviderReplaceAppointmentActivatedEventArgs> {
        ::core::convert::TryInto::<::winrt_applicationmodel::Activation::IAppointmentsProviderReplaceAppointmentActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
#[repr(transparent)]
pub struct WebUIAppointmentsProviderShowAppointmentDetailsActivatedEventArgs(::windows_core::IUnknown);
#[cfg(feature = "ApplicationModel_Activation")]
impl WebUIAppointmentsProviderShowAppointmentDetailsActivatedEventArgs {
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Kind(&self) -> ::windows_core::Result<::winrt_applicationmodel::Activation::ActivationKind> {
        let this = &::windows_core::Interface::cast::<::winrt_applicationmodel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_applicationmodel::Activation::ActivationKind>::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_applicationmodel::Activation::ActivationKind>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<::winrt_applicationmodel::Activation::ApplicationExecutionState> {
        let this = &::windows_core::Interface::cast::<::winrt_applicationmodel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_applicationmodel::Activation::ApplicationExecutionState>::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_applicationmodel::Activation::ApplicationExecutionState>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn SplashScreen(&self) -> ::windows_core::Result<::winrt_applicationmodel::Activation::SplashScreen> {
        let this = &::windows_core::Interface::cast::<::winrt_applicationmodel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_applicationmodel::Activation::SplashScreen>(result__)
        }
    }
    pub fn ActivatedOperation(&self) -> ::windows_core::Result<ActivatedOperation> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgsDeferral>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ActivatedOperation)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivatedOperation>(result__)
        }
    }
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "System"))]
    pub fn User(&self) -> ::windows_core::Result<::winrt_system::User> {
        let this = &::windows_core::Interface::cast::<::winrt_applicationmodel::Activation::IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).User)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_system::User>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Verb(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<::winrt_applicationmodel::Activation::IAppointmentsProviderActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Verb)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn InstanceStartDate(&self) -> ::windows_core::Result<::winrt_foundation::IReference<::winrt_foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).InstanceStartDate)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IReference<::winrt_foundation::DateTime>>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn LocalId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).LocalId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn RoamingId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).RoamingId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::clone::Clone for WebUIAppointmentsProviderShowAppointmentDetailsActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::cmp::PartialEq for WebUIAppointmentsProviderShowAppointmentDetailsActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::cmp::Eq for WebUIAppointmentsProviderShowAppointmentDetailsActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::fmt::Debug for WebUIAppointmentsProviderShowAppointmentDetailsActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WebUIAppointmentsProviderShowAppointmentDetailsActivatedEventArgs").field(&self.0).finish()
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows_core::RuntimeType for WebUIAppointmentsProviderShowAppointmentDetailsActivatedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.WebUI.WebUIAppointmentsProviderShowAppointmentDetailsActivatedEventArgs;{3958f065-9841-4ca5-999b-885198b9ef2a})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows_core::Interface for WebUIAppointmentsProviderShowAppointmentDetailsActivatedEventArgs {
    type Vtable = ::winrt_applicationmodel::Activation::IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <::winrt_applicationmodel::Activation::IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs as ::windows_core::Interface>::IID;
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::RuntimeName for WebUIAppointmentsProviderShowAppointmentDetailsActivatedEventArgs {
    const NAME: &'static str = "Windows.UI.WebUI.WebUIAppointmentsProviderShowAppointmentDetailsActivatedEventArgs";
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<WebUIAppointmentsProviderShowAppointmentDetailsActivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: WebUIAppointmentsProviderShowAppointmentDetailsActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<&WebUIAppointmentsProviderShowAppointmentDetailsActivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &WebUIAppointmentsProviderShowAppointmentDetailsActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for WebUIAppointmentsProviderShowAppointmentDetailsActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a WebUIAppointmentsProviderShowAppointmentDetailsActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<WebUIAppointmentsProviderShowAppointmentDetailsActivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: WebUIAppointmentsProviderShowAppointmentDetailsActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<&WebUIAppointmentsProviderShowAppointmentDetailsActivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &WebUIAppointmentsProviderShowAppointmentDetailsActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for WebUIAppointmentsProviderShowAppointmentDetailsActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a WebUIAppointmentsProviderShowAppointmentDetailsActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIAppointmentsProviderShowAppointmentDetailsActivatedEventArgs> for ::winrt_applicationmodel::Activation::IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: WebUIAppointmentsProviderShowAppointmentDetailsActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIAppointmentsProviderShowAppointmentDetailsActivatedEventArgs> for ::winrt_applicationmodel::Activation::IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &WebUIAppointmentsProviderShowAppointmentDetailsActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgs> for WebUIAppointmentsProviderShowAppointmentDetailsActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgs> for &WebUIAppointmentsProviderShowAppointmentDetailsActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgs> {
        ::core::convert::TryInto::<::winrt_applicationmodel::Activation::IActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIAppointmentsProviderShowAppointmentDetailsActivatedEventArgs> for IActivatedEventArgsDeferral {
    type Error = ::windows_core::Error;
    fn try_from(value: WebUIAppointmentsProviderShowAppointmentDetailsActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIAppointmentsProviderShowAppointmentDetailsActivatedEventArgs> for IActivatedEventArgsDeferral {
    type Error = ::windows_core::Error;
    fn try_from(value: &WebUIAppointmentsProviderShowAppointmentDetailsActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgsDeferral> for WebUIAppointmentsProviderShowAppointmentDetailsActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgsDeferral> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgsDeferral> for &WebUIAppointmentsProviderShowAppointmentDetailsActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgsDeferral> {
        ::core::convert::TryInto::<IActivatedEventArgsDeferral>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIAppointmentsProviderShowAppointmentDetailsActivatedEventArgs> for ::winrt_applicationmodel::Activation::IActivatedEventArgsWithUser {
    type Error = ::windows_core::Error;
    fn try_from(value: WebUIAppointmentsProviderShowAppointmentDetailsActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIAppointmentsProviderShowAppointmentDetailsActivatedEventArgs> for ::winrt_applicationmodel::Activation::IActivatedEventArgsWithUser {
    type Error = ::windows_core::Error;
    fn try_from(value: &WebUIAppointmentsProviderShowAppointmentDetailsActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgsWithUser> for WebUIAppointmentsProviderShowAppointmentDetailsActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgsWithUser> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgsWithUser> for &WebUIAppointmentsProviderShowAppointmentDetailsActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgsWithUser> {
        ::core::convert::TryInto::<::winrt_applicationmodel::Activation::IActivatedEventArgsWithUser>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIAppointmentsProviderShowAppointmentDetailsActivatedEventArgs> for ::winrt_applicationmodel::Activation::IAppointmentsProviderActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: WebUIAppointmentsProviderShowAppointmentDetailsActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIAppointmentsProviderShowAppointmentDetailsActivatedEventArgs> for ::winrt_applicationmodel::Activation::IAppointmentsProviderActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &WebUIAppointmentsProviderShowAppointmentDetailsActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IAppointmentsProviderActivatedEventArgs> for WebUIAppointmentsProviderShowAppointmentDetailsActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IAppointmentsProviderActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IAppointmentsProviderActivatedEventArgs> for &WebUIAppointmentsProviderShowAppointmentDetailsActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IAppointmentsProviderActivatedEventArgs> {
        ::core::convert::TryInto::<::winrt_applicationmodel::Activation::IAppointmentsProviderActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIAppointmentsProviderShowAppointmentDetailsActivatedEventArgs> for ::winrt_applicationmodel::Activation::IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: WebUIAppointmentsProviderShowAppointmentDetailsActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIAppointmentsProviderShowAppointmentDetailsActivatedEventArgs> for ::winrt_applicationmodel::Activation::IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &WebUIAppointmentsProviderShowAppointmentDetailsActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs> for WebUIAppointmentsProviderShowAppointmentDetailsActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs> for &WebUIAppointmentsProviderShowAppointmentDetailsActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs> {
        ::core::convert::TryInto::<::winrt_applicationmodel::Activation::IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
#[repr(transparent)]
pub struct WebUIAppointmentsProviderShowTimeFrameActivatedEventArgs(::windows_core::IUnknown);
#[cfg(feature = "ApplicationModel_Activation")]
impl WebUIAppointmentsProviderShowTimeFrameActivatedEventArgs {
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Kind(&self) -> ::windows_core::Result<::winrt_applicationmodel::Activation::ActivationKind> {
        let this = &::windows_core::Interface::cast::<::winrt_applicationmodel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_applicationmodel::Activation::ActivationKind>::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_applicationmodel::Activation::ActivationKind>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<::winrt_applicationmodel::Activation::ApplicationExecutionState> {
        let this = &::windows_core::Interface::cast::<::winrt_applicationmodel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_applicationmodel::Activation::ApplicationExecutionState>::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_applicationmodel::Activation::ApplicationExecutionState>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn SplashScreen(&self) -> ::windows_core::Result<::winrt_applicationmodel::Activation::SplashScreen> {
        let this = &::windows_core::Interface::cast::<::winrt_applicationmodel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_applicationmodel::Activation::SplashScreen>(result__)
        }
    }
    pub fn ActivatedOperation(&self) -> ::windows_core::Result<ActivatedOperation> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgsDeferral>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ActivatedOperation)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivatedOperation>(result__)
        }
    }
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "System"))]
    pub fn User(&self) -> ::windows_core::Result<::winrt_system::User> {
        let this = &::windows_core::Interface::cast::<::winrt_applicationmodel::Activation::IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).User)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_system::User>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Verb(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<::winrt_applicationmodel::Activation::IAppointmentsProviderActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Verb)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn TimeToShow(&self) -> ::windows_core::Result<::winrt_foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::DateTime>::zeroed();
            (::windows_core::Interface::vtable(this).TimeToShow)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::DateTime>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Duration(&self) -> ::windows_core::Result<::winrt_foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::TimeSpan>::zeroed();
            (::windows_core::Interface::vtable(this).Duration)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::TimeSpan>(result__)
        }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::clone::Clone for WebUIAppointmentsProviderShowTimeFrameActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::cmp::PartialEq for WebUIAppointmentsProviderShowTimeFrameActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::cmp::Eq for WebUIAppointmentsProviderShowTimeFrameActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::fmt::Debug for WebUIAppointmentsProviderShowTimeFrameActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WebUIAppointmentsProviderShowTimeFrameActivatedEventArgs").field(&self.0).finish()
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows_core::RuntimeType for WebUIAppointmentsProviderShowTimeFrameActivatedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.WebUI.WebUIAppointmentsProviderShowTimeFrameActivatedEventArgs;{9baeaba6-0e0b-49aa-babc-12b1dc774986})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows_core::Interface for WebUIAppointmentsProviderShowTimeFrameActivatedEventArgs {
    type Vtable = ::winrt_applicationmodel::Activation::IAppointmentsProviderShowTimeFrameActivatedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <::winrt_applicationmodel::Activation::IAppointmentsProviderShowTimeFrameActivatedEventArgs as ::windows_core::Interface>::IID;
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::RuntimeName for WebUIAppointmentsProviderShowTimeFrameActivatedEventArgs {
    const NAME: &'static str = "Windows.UI.WebUI.WebUIAppointmentsProviderShowTimeFrameActivatedEventArgs";
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<WebUIAppointmentsProviderShowTimeFrameActivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: WebUIAppointmentsProviderShowTimeFrameActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<&WebUIAppointmentsProviderShowTimeFrameActivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &WebUIAppointmentsProviderShowTimeFrameActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for WebUIAppointmentsProviderShowTimeFrameActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a WebUIAppointmentsProviderShowTimeFrameActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<WebUIAppointmentsProviderShowTimeFrameActivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: WebUIAppointmentsProviderShowTimeFrameActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<&WebUIAppointmentsProviderShowTimeFrameActivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &WebUIAppointmentsProviderShowTimeFrameActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for WebUIAppointmentsProviderShowTimeFrameActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a WebUIAppointmentsProviderShowTimeFrameActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIAppointmentsProviderShowTimeFrameActivatedEventArgs> for ::winrt_applicationmodel::Activation::IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: WebUIAppointmentsProviderShowTimeFrameActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIAppointmentsProviderShowTimeFrameActivatedEventArgs> for ::winrt_applicationmodel::Activation::IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &WebUIAppointmentsProviderShowTimeFrameActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgs> for WebUIAppointmentsProviderShowTimeFrameActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgs> for &WebUIAppointmentsProviderShowTimeFrameActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgs> {
        ::core::convert::TryInto::<::winrt_applicationmodel::Activation::IActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIAppointmentsProviderShowTimeFrameActivatedEventArgs> for IActivatedEventArgsDeferral {
    type Error = ::windows_core::Error;
    fn try_from(value: WebUIAppointmentsProviderShowTimeFrameActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIAppointmentsProviderShowTimeFrameActivatedEventArgs> for IActivatedEventArgsDeferral {
    type Error = ::windows_core::Error;
    fn try_from(value: &WebUIAppointmentsProviderShowTimeFrameActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgsDeferral> for WebUIAppointmentsProviderShowTimeFrameActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgsDeferral> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgsDeferral> for &WebUIAppointmentsProviderShowTimeFrameActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgsDeferral> {
        ::core::convert::TryInto::<IActivatedEventArgsDeferral>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIAppointmentsProviderShowTimeFrameActivatedEventArgs> for ::winrt_applicationmodel::Activation::IActivatedEventArgsWithUser {
    type Error = ::windows_core::Error;
    fn try_from(value: WebUIAppointmentsProviderShowTimeFrameActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIAppointmentsProviderShowTimeFrameActivatedEventArgs> for ::winrt_applicationmodel::Activation::IActivatedEventArgsWithUser {
    type Error = ::windows_core::Error;
    fn try_from(value: &WebUIAppointmentsProviderShowTimeFrameActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgsWithUser> for WebUIAppointmentsProviderShowTimeFrameActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgsWithUser> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgsWithUser> for &WebUIAppointmentsProviderShowTimeFrameActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgsWithUser> {
        ::core::convert::TryInto::<::winrt_applicationmodel::Activation::IActivatedEventArgsWithUser>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIAppointmentsProviderShowTimeFrameActivatedEventArgs> for ::winrt_applicationmodel::Activation::IAppointmentsProviderActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: WebUIAppointmentsProviderShowTimeFrameActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIAppointmentsProviderShowTimeFrameActivatedEventArgs> for ::winrt_applicationmodel::Activation::IAppointmentsProviderActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &WebUIAppointmentsProviderShowTimeFrameActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IAppointmentsProviderActivatedEventArgs> for WebUIAppointmentsProviderShowTimeFrameActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IAppointmentsProviderActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IAppointmentsProviderActivatedEventArgs> for &WebUIAppointmentsProviderShowTimeFrameActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IAppointmentsProviderActivatedEventArgs> {
        ::core::convert::TryInto::<::winrt_applicationmodel::Activation::IAppointmentsProviderActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIAppointmentsProviderShowTimeFrameActivatedEventArgs> for ::winrt_applicationmodel::Activation::IAppointmentsProviderShowTimeFrameActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: WebUIAppointmentsProviderShowTimeFrameActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIAppointmentsProviderShowTimeFrameActivatedEventArgs> for ::winrt_applicationmodel::Activation::IAppointmentsProviderShowTimeFrameActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &WebUIAppointmentsProviderShowTimeFrameActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IAppointmentsProviderShowTimeFrameActivatedEventArgs> for WebUIAppointmentsProviderShowTimeFrameActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IAppointmentsProviderShowTimeFrameActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IAppointmentsProviderShowTimeFrameActivatedEventArgs> for &WebUIAppointmentsProviderShowTimeFrameActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IAppointmentsProviderShowTimeFrameActivatedEventArgs> {
        ::core::convert::TryInto::<::winrt_applicationmodel::Activation::IAppointmentsProviderShowTimeFrameActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
pub struct WebUIBackgroundTaskInstance;
impl WebUIBackgroundTaskInstance {
    pub fn Current() -> ::windows_core::Result<IWebUIBackgroundTaskInstance> {
        Self::IWebUIBackgroundTaskInstanceStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Current)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<IWebUIBackgroundTaskInstance>(result__)
        })
    }
    pub fn IWebUIBackgroundTaskInstanceStatics<R, F: FnOnce(&IWebUIBackgroundTaskInstanceStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<WebUIBackgroundTaskInstance, IWebUIBackgroundTaskInstanceStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows_core::RuntimeName for WebUIBackgroundTaskInstance {
    const NAME: &'static str = "Windows.UI.WebUI.WebUIBackgroundTaskInstance";
}
#[repr(transparent)]
pub struct WebUIBackgroundTaskInstanceRuntimeClass(::windows_core::IUnknown);
impl WebUIBackgroundTaskInstanceRuntimeClass {
    #[cfg(feature = "ApplicationModel_Background")]
    pub fn InstanceId(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let this = &::windows_core::Interface::cast::<::winrt_applicationmodel::Background::IBackgroundTaskInstance>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::GUID>::zeroed();
            (::windows_core::Interface::vtable(this).InstanceId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::GUID>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Background")]
    pub fn Task(&self) -> ::windows_core::Result<::winrt_applicationmodel::Background::BackgroundTaskRegistration> {
        let this = &::windows_core::Interface::cast::<::winrt_applicationmodel::Background::IBackgroundTaskInstance>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Task)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_applicationmodel::Background::BackgroundTaskRegistration>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Background")]
    pub fn Progress(&self) -> ::windows_core::Result<u32> {
        let this = &::windows_core::Interface::cast::<::winrt_applicationmodel::Background::IBackgroundTaskInstance>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).Progress)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Background")]
    pub fn SetProgress(&self, value: u32) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_applicationmodel::Background::IBackgroundTaskInstance>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetProgress)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "ApplicationModel_Background")]
    pub fn TriggerDetails(&self) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = &::windows_core::Interface::cast::<::winrt_applicationmodel::Background::IBackgroundTaskInstance>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows_core::Interface::vtable(this).TriggerDetails)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::IInspectable>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Background")]
    pub fn Canceled<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Background::BackgroundTaskCanceledEventHandler>>(&self, cancelhandler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = &::windows_core::Interface::cast::<::winrt_applicationmodel::Background::IBackgroundTaskInstance>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).Canceled)(::windows_core::Interface::as_raw(this), cancelhandler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Background")]
    pub fn RemoveCanceled<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_applicationmodel::Background::IBackgroundTaskInstance>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).RemoveCanceled)(::windows_core::Interface::as_raw(this), cookie.into_param().abi()).ok() }
    }
    #[cfg(feature = "ApplicationModel_Background")]
    pub fn SuspendedCount(&self) -> ::windows_core::Result<u32> {
        let this = &::windows_core::Interface::cast::<::winrt_applicationmodel::Background::IBackgroundTaskInstance>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).SuspendedCount)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Background")]
    pub fn GetDeferral(&self) -> ::windows_core::Result<::winrt_applicationmodel::Background::BackgroundTaskDeferral> {
        let this = &::windows_core::Interface::cast::<::winrt_applicationmodel::Background::IBackgroundTaskInstance>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetDeferral)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_applicationmodel::Background::BackgroundTaskDeferral>(result__)
        }
    }
    pub fn Succeeded(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).Succeeded)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetSucceeded(&self, succeeded: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetSucceeded)(::windows_core::Interface::as_raw(this), succeeded).ok() }
    }
}
impl ::core::clone::Clone for WebUIBackgroundTaskInstanceRuntimeClass {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for WebUIBackgroundTaskInstanceRuntimeClass {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for WebUIBackgroundTaskInstanceRuntimeClass {}
impl ::core::fmt::Debug for WebUIBackgroundTaskInstanceRuntimeClass {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WebUIBackgroundTaskInstanceRuntimeClass").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for WebUIBackgroundTaskInstanceRuntimeClass {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.WebUI.WebUIBackgroundTaskInstanceRuntimeClass;{23f12c25-e2f7-4741-bc9c-394595de24dc})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for WebUIBackgroundTaskInstanceRuntimeClass {
    type Vtable = IWebUIBackgroundTaskInstance_Vtbl;
    const IID: ::windows_core::GUID = <IWebUIBackgroundTaskInstance as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for WebUIBackgroundTaskInstanceRuntimeClass {
    const NAME: &'static str = "Windows.UI.WebUI.WebUIBackgroundTaskInstanceRuntimeClass";
}
impl ::core::convert::From<WebUIBackgroundTaskInstanceRuntimeClass> for ::windows_core::IUnknown {
    fn from(value: WebUIBackgroundTaskInstanceRuntimeClass) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WebUIBackgroundTaskInstanceRuntimeClass> for ::windows_core::IUnknown {
    fn from(value: &WebUIBackgroundTaskInstanceRuntimeClass) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for WebUIBackgroundTaskInstanceRuntimeClass {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a WebUIBackgroundTaskInstanceRuntimeClass {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<WebUIBackgroundTaskInstanceRuntimeClass> for ::windows_core::IInspectable {
    fn from(value: WebUIBackgroundTaskInstanceRuntimeClass) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WebUIBackgroundTaskInstanceRuntimeClass> for ::windows_core::IInspectable {
    fn from(value: &WebUIBackgroundTaskInstanceRuntimeClass) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for WebUIBackgroundTaskInstanceRuntimeClass {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a WebUIBackgroundTaskInstanceRuntimeClass {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "ApplicationModel_Background")]
impl ::core::convert::TryFrom<WebUIBackgroundTaskInstanceRuntimeClass> for ::winrt_applicationmodel::Background::IBackgroundTaskInstance {
    type Error = ::windows_core::Error;
    fn try_from(value: WebUIBackgroundTaskInstanceRuntimeClass) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Background")]
impl ::core::convert::TryFrom<&WebUIBackgroundTaskInstanceRuntimeClass> for ::winrt_applicationmodel::Background::IBackgroundTaskInstance {
    type Error = ::windows_core::Error;
    fn try_from(value: &WebUIBackgroundTaskInstanceRuntimeClass) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Background")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Background::IBackgroundTaskInstance> for WebUIBackgroundTaskInstanceRuntimeClass {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Background::IBackgroundTaskInstance> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Background")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Background::IBackgroundTaskInstance> for &WebUIBackgroundTaskInstanceRuntimeClass {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Background::IBackgroundTaskInstance> {
        ::core::convert::TryInto::<::winrt_applicationmodel::Background::IBackgroundTaskInstance>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<WebUIBackgroundTaskInstanceRuntimeClass> for IWebUIBackgroundTaskInstance {
    type Error = ::windows_core::Error;
    fn try_from(value: WebUIBackgroundTaskInstanceRuntimeClass) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&WebUIBackgroundTaskInstanceRuntimeClass> for IWebUIBackgroundTaskInstance {
    type Error = ::windows_core::Error;
    fn try_from(value: &WebUIBackgroundTaskInstanceRuntimeClass) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IWebUIBackgroundTaskInstance> for WebUIBackgroundTaskInstanceRuntimeClass {
    fn into_param(self) -> ::windows_core::Param<'a, IWebUIBackgroundTaskInstance> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IWebUIBackgroundTaskInstance> for &WebUIBackgroundTaskInstanceRuntimeClass {
    fn into_param(self) -> ::windows_core::Param<'a, IWebUIBackgroundTaskInstance> {
        ::core::convert::TryInto::<IWebUIBackgroundTaskInstance>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
#[repr(transparent)]
pub struct WebUIBarcodeScannerPreviewActivatedEventArgs(::windows_core::IUnknown);
#[cfg(feature = "ApplicationModel_Activation")]
impl WebUIBarcodeScannerPreviewActivatedEventArgs {
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Kind(&self) -> ::windows_core::Result<::winrt_applicationmodel::Activation::ActivationKind> {
        let this = &::windows_core::Interface::cast::<::winrt_applicationmodel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_applicationmodel::Activation::ActivationKind>::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_applicationmodel::Activation::ActivationKind>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<::winrt_applicationmodel::Activation::ApplicationExecutionState> {
        let this = &::windows_core::Interface::cast::<::winrt_applicationmodel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_applicationmodel::Activation::ApplicationExecutionState>::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_applicationmodel::Activation::ApplicationExecutionState>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn SplashScreen(&self) -> ::windows_core::Result<::winrt_applicationmodel::Activation::SplashScreen> {
        let this = &::windows_core::Interface::cast::<::winrt_applicationmodel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_applicationmodel::Activation::SplashScreen>(result__)
        }
    }
    pub fn ActivatedOperation(&self) -> ::windows_core::Result<ActivatedOperation> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgsDeferral>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ActivatedOperation)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivatedOperation>(result__)
        }
    }
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "System"))]
    pub fn User(&self) -> ::windows_core::Result<::winrt_system::User> {
        let this = &::windows_core::Interface::cast::<::winrt_applicationmodel::Activation::IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).User)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_system::User>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn ConnectionId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).ConnectionId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::clone::Clone for WebUIBarcodeScannerPreviewActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::cmp::PartialEq for WebUIBarcodeScannerPreviewActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::cmp::Eq for WebUIBarcodeScannerPreviewActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::fmt::Debug for WebUIBarcodeScannerPreviewActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WebUIBarcodeScannerPreviewActivatedEventArgs").field(&self.0).finish()
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows_core::RuntimeType for WebUIBarcodeScannerPreviewActivatedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.WebUI.WebUIBarcodeScannerPreviewActivatedEventArgs;{6772797c-99bf-4349-af22-e4123560371c})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows_core::Interface for WebUIBarcodeScannerPreviewActivatedEventArgs {
    type Vtable = ::winrt_applicationmodel::Activation::IBarcodeScannerPreviewActivatedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <::winrt_applicationmodel::Activation::IBarcodeScannerPreviewActivatedEventArgs as ::windows_core::Interface>::IID;
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::RuntimeName for WebUIBarcodeScannerPreviewActivatedEventArgs {
    const NAME: &'static str = "Windows.UI.WebUI.WebUIBarcodeScannerPreviewActivatedEventArgs";
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<WebUIBarcodeScannerPreviewActivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: WebUIBarcodeScannerPreviewActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<&WebUIBarcodeScannerPreviewActivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &WebUIBarcodeScannerPreviewActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for WebUIBarcodeScannerPreviewActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a WebUIBarcodeScannerPreviewActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<WebUIBarcodeScannerPreviewActivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: WebUIBarcodeScannerPreviewActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<&WebUIBarcodeScannerPreviewActivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &WebUIBarcodeScannerPreviewActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for WebUIBarcodeScannerPreviewActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a WebUIBarcodeScannerPreviewActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIBarcodeScannerPreviewActivatedEventArgs> for ::winrt_applicationmodel::Activation::IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: WebUIBarcodeScannerPreviewActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIBarcodeScannerPreviewActivatedEventArgs> for ::winrt_applicationmodel::Activation::IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &WebUIBarcodeScannerPreviewActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgs> for WebUIBarcodeScannerPreviewActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgs> for &WebUIBarcodeScannerPreviewActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgs> {
        ::core::convert::TryInto::<::winrt_applicationmodel::Activation::IActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIBarcodeScannerPreviewActivatedEventArgs> for IActivatedEventArgsDeferral {
    type Error = ::windows_core::Error;
    fn try_from(value: WebUIBarcodeScannerPreviewActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIBarcodeScannerPreviewActivatedEventArgs> for IActivatedEventArgsDeferral {
    type Error = ::windows_core::Error;
    fn try_from(value: &WebUIBarcodeScannerPreviewActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgsDeferral> for WebUIBarcodeScannerPreviewActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgsDeferral> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgsDeferral> for &WebUIBarcodeScannerPreviewActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgsDeferral> {
        ::core::convert::TryInto::<IActivatedEventArgsDeferral>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIBarcodeScannerPreviewActivatedEventArgs> for ::winrt_applicationmodel::Activation::IActivatedEventArgsWithUser {
    type Error = ::windows_core::Error;
    fn try_from(value: WebUIBarcodeScannerPreviewActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIBarcodeScannerPreviewActivatedEventArgs> for ::winrt_applicationmodel::Activation::IActivatedEventArgsWithUser {
    type Error = ::windows_core::Error;
    fn try_from(value: &WebUIBarcodeScannerPreviewActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgsWithUser> for WebUIBarcodeScannerPreviewActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgsWithUser> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgsWithUser> for &WebUIBarcodeScannerPreviewActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgsWithUser> {
        ::core::convert::TryInto::<::winrt_applicationmodel::Activation::IActivatedEventArgsWithUser>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIBarcodeScannerPreviewActivatedEventArgs> for ::winrt_applicationmodel::Activation::IBarcodeScannerPreviewActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: WebUIBarcodeScannerPreviewActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIBarcodeScannerPreviewActivatedEventArgs> for ::winrt_applicationmodel::Activation::IBarcodeScannerPreviewActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &WebUIBarcodeScannerPreviewActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IBarcodeScannerPreviewActivatedEventArgs> for WebUIBarcodeScannerPreviewActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IBarcodeScannerPreviewActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IBarcodeScannerPreviewActivatedEventArgs> for &WebUIBarcodeScannerPreviewActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IBarcodeScannerPreviewActivatedEventArgs> {
        ::core::convert::TryInto::<::winrt_applicationmodel::Activation::IBarcodeScannerPreviewActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::core::marker::Send for WebUIBarcodeScannerPreviewActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::core::marker::Sync for WebUIBarcodeScannerPreviewActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
#[repr(transparent)]
pub struct WebUICachedFileUpdaterActivatedEventArgs(::windows_core::IUnknown);
#[cfg(feature = "ApplicationModel_Activation")]
impl WebUICachedFileUpdaterActivatedEventArgs {
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Kind(&self) -> ::windows_core::Result<::winrt_applicationmodel::Activation::ActivationKind> {
        let this = &::windows_core::Interface::cast::<::winrt_applicationmodel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_applicationmodel::Activation::ActivationKind>::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_applicationmodel::Activation::ActivationKind>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<::winrt_applicationmodel::Activation::ApplicationExecutionState> {
        let this = &::windows_core::Interface::cast::<::winrt_applicationmodel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_applicationmodel::Activation::ApplicationExecutionState>::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_applicationmodel::Activation::ApplicationExecutionState>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn SplashScreen(&self) -> ::windows_core::Result<::winrt_applicationmodel::Activation::SplashScreen> {
        let this = &::windows_core::Interface::cast::<::winrt_applicationmodel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_applicationmodel::Activation::SplashScreen>(result__)
        }
    }
    pub fn ActivatedOperation(&self) -> ::windows_core::Result<ActivatedOperation> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgsDeferral>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ActivatedOperation)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivatedOperation>(result__)
        }
    }
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "System"))]
    pub fn User(&self) -> ::windows_core::Result<::winrt_system::User> {
        let this = &::windows_core::Interface::cast::<::winrt_applicationmodel::Activation::IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).User)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_system::User>(result__)
        }
    }
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "Storage_Provider"))]
    pub fn CachedFileUpdaterUI(&self) -> ::windows_core::Result<::winrt_storage::Provider::CachedFileUpdaterUI> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CachedFileUpdaterUI)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_storage::Provider::CachedFileUpdaterUI>(result__)
        }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::clone::Clone for WebUICachedFileUpdaterActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::cmp::PartialEq for WebUICachedFileUpdaterActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::cmp::Eq for WebUICachedFileUpdaterActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::fmt::Debug for WebUICachedFileUpdaterActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WebUICachedFileUpdaterActivatedEventArgs").field(&self.0).finish()
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows_core::RuntimeType for WebUICachedFileUpdaterActivatedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.WebUI.WebUICachedFileUpdaterActivatedEventArgs;{d06eb1c7-3805-4ecb-b757-6cf15e26fef3})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows_core::Interface for WebUICachedFileUpdaterActivatedEventArgs {
    type Vtable = ::winrt_applicationmodel::Activation::ICachedFileUpdaterActivatedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <::winrt_applicationmodel::Activation::ICachedFileUpdaterActivatedEventArgs as ::windows_core::Interface>::IID;
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::RuntimeName for WebUICachedFileUpdaterActivatedEventArgs {
    const NAME: &'static str = "Windows.UI.WebUI.WebUICachedFileUpdaterActivatedEventArgs";
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<WebUICachedFileUpdaterActivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: WebUICachedFileUpdaterActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<&WebUICachedFileUpdaterActivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &WebUICachedFileUpdaterActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for WebUICachedFileUpdaterActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a WebUICachedFileUpdaterActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<WebUICachedFileUpdaterActivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: WebUICachedFileUpdaterActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<&WebUICachedFileUpdaterActivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &WebUICachedFileUpdaterActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for WebUICachedFileUpdaterActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a WebUICachedFileUpdaterActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUICachedFileUpdaterActivatedEventArgs> for ::winrt_applicationmodel::Activation::IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: WebUICachedFileUpdaterActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUICachedFileUpdaterActivatedEventArgs> for ::winrt_applicationmodel::Activation::IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &WebUICachedFileUpdaterActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgs> for WebUICachedFileUpdaterActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgs> for &WebUICachedFileUpdaterActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgs> {
        ::core::convert::TryInto::<::winrt_applicationmodel::Activation::IActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUICachedFileUpdaterActivatedEventArgs> for IActivatedEventArgsDeferral {
    type Error = ::windows_core::Error;
    fn try_from(value: WebUICachedFileUpdaterActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUICachedFileUpdaterActivatedEventArgs> for IActivatedEventArgsDeferral {
    type Error = ::windows_core::Error;
    fn try_from(value: &WebUICachedFileUpdaterActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgsDeferral> for WebUICachedFileUpdaterActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgsDeferral> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgsDeferral> for &WebUICachedFileUpdaterActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgsDeferral> {
        ::core::convert::TryInto::<IActivatedEventArgsDeferral>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUICachedFileUpdaterActivatedEventArgs> for ::winrt_applicationmodel::Activation::IActivatedEventArgsWithUser {
    type Error = ::windows_core::Error;
    fn try_from(value: WebUICachedFileUpdaterActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUICachedFileUpdaterActivatedEventArgs> for ::winrt_applicationmodel::Activation::IActivatedEventArgsWithUser {
    type Error = ::windows_core::Error;
    fn try_from(value: &WebUICachedFileUpdaterActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgsWithUser> for WebUICachedFileUpdaterActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgsWithUser> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgsWithUser> for &WebUICachedFileUpdaterActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgsWithUser> {
        ::core::convert::TryInto::<::winrt_applicationmodel::Activation::IActivatedEventArgsWithUser>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUICachedFileUpdaterActivatedEventArgs> for ::winrt_applicationmodel::Activation::ICachedFileUpdaterActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: WebUICachedFileUpdaterActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUICachedFileUpdaterActivatedEventArgs> for ::winrt_applicationmodel::Activation::ICachedFileUpdaterActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &WebUICachedFileUpdaterActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::ICachedFileUpdaterActivatedEventArgs> for WebUICachedFileUpdaterActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::ICachedFileUpdaterActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::ICachedFileUpdaterActivatedEventArgs> for &WebUICachedFileUpdaterActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::ICachedFileUpdaterActivatedEventArgs> {
        ::core::convert::TryInto::<::winrt_applicationmodel::Activation::ICachedFileUpdaterActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
#[repr(transparent)]
pub struct WebUICameraSettingsActivatedEventArgs(::windows_core::IUnknown);
#[cfg(feature = "ApplicationModel_Activation")]
impl WebUICameraSettingsActivatedEventArgs {
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Kind(&self) -> ::windows_core::Result<::winrt_applicationmodel::Activation::ActivationKind> {
        let this = &::windows_core::Interface::cast::<::winrt_applicationmodel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_applicationmodel::Activation::ActivationKind>::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_applicationmodel::Activation::ActivationKind>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<::winrt_applicationmodel::Activation::ApplicationExecutionState> {
        let this = &::windows_core::Interface::cast::<::winrt_applicationmodel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_applicationmodel::Activation::ApplicationExecutionState>::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_applicationmodel::Activation::ApplicationExecutionState>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn SplashScreen(&self) -> ::windows_core::Result<::winrt_applicationmodel::Activation::SplashScreen> {
        let this = &::windows_core::Interface::cast::<::winrt_applicationmodel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_applicationmodel::Activation::SplashScreen>(result__)
        }
    }
    pub fn ActivatedOperation(&self) -> ::windows_core::Result<ActivatedOperation> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgsDeferral>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ActivatedOperation)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivatedOperation>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn VideoDeviceController(&self) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows_core::Interface::vtable(this).VideoDeviceController)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::IInspectable>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn VideoDeviceExtension(&self) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows_core::Interface::vtable(this).VideoDeviceExtension)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::IInspectable>(result__)
        }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::clone::Clone for WebUICameraSettingsActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::cmp::PartialEq for WebUICameraSettingsActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::cmp::Eq for WebUICameraSettingsActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::fmt::Debug for WebUICameraSettingsActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WebUICameraSettingsActivatedEventArgs").field(&self.0).finish()
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows_core::RuntimeType for WebUICameraSettingsActivatedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.WebUI.WebUICameraSettingsActivatedEventArgs;{fb67a508-2dad-490a-9170-dca036eb114b})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows_core::Interface for WebUICameraSettingsActivatedEventArgs {
    type Vtable = ::winrt_applicationmodel::Activation::ICameraSettingsActivatedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <::winrt_applicationmodel::Activation::ICameraSettingsActivatedEventArgs as ::windows_core::Interface>::IID;
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::RuntimeName for WebUICameraSettingsActivatedEventArgs {
    const NAME: &'static str = "Windows.UI.WebUI.WebUICameraSettingsActivatedEventArgs";
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<WebUICameraSettingsActivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: WebUICameraSettingsActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<&WebUICameraSettingsActivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &WebUICameraSettingsActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for WebUICameraSettingsActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a WebUICameraSettingsActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<WebUICameraSettingsActivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: WebUICameraSettingsActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<&WebUICameraSettingsActivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &WebUICameraSettingsActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for WebUICameraSettingsActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a WebUICameraSettingsActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUICameraSettingsActivatedEventArgs> for ::winrt_applicationmodel::Activation::IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: WebUICameraSettingsActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUICameraSettingsActivatedEventArgs> for ::winrt_applicationmodel::Activation::IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &WebUICameraSettingsActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgs> for WebUICameraSettingsActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgs> for &WebUICameraSettingsActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgs> {
        ::core::convert::TryInto::<::winrt_applicationmodel::Activation::IActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUICameraSettingsActivatedEventArgs> for IActivatedEventArgsDeferral {
    type Error = ::windows_core::Error;
    fn try_from(value: WebUICameraSettingsActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUICameraSettingsActivatedEventArgs> for IActivatedEventArgsDeferral {
    type Error = ::windows_core::Error;
    fn try_from(value: &WebUICameraSettingsActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgsDeferral> for WebUICameraSettingsActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgsDeferral> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgsDeferral> for &WebUICameraSettingsActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgsDeferral> {
        ::core::convert::TryInto::<IActivatedEventArgsDeferral>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUICameraSettingsActivatedEventArgs> for ::winrt_applicationmodel::Activation::ICameraSettingsActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: WebUICameraSettingsActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUICameraSettingsActivatedEventArgs> for ::winrt_applicationmodel::Activation::ICameraSettingsActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &WebUICameraSettingsActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::ICameraSettingsActivatedEventArgs> for WebUICameraSettingsActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::ICameraSettingsActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::ICameraSettingsActivatedEventArgs> for &WebUICameraSettingsActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::ICameraSettingsActivatedEventArgs> {
        ::core::convert::TryInto::<::winrt_applicationmodel::Activation::ICameraSettingsActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
#[repr(transparent)]
pub struct WebUICommandLineActivatedEventArgs(::windows_core::IUnknown);
#[cfg(feature = "ApplicationModel_Activation")]
impl WebUICommandLineActivatedEventArgs {
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Kind(&self) -> ::windows_core::Result<::winrt_applicationmodel::Activation::ActivationKind> {
        let this = &::windows_core::Interface::cast::<::winrt_applicationmodel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_applicationmodel::Activation::ActivationKind>::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_applicationmodel::Activation::ActivationKind>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<::winrt_applicationmodel::Activation::ApplicationExecutionState> {
        let this = &::windows_core::Interface::cast::<::winrt_applicationmodel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_applicationmodel::Activation::ApplicationExecutionState>::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_applicationmodel::Activation::ApplicationExecutionState>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn SplashScreen(&self) -> ::windows_core::Result<::winrt_applicationmodel::Activation::SplashScreen> {
        let this = &::windows_core::Interface::cast::<::winrt_applicationmodel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_applicationmodel::Activation::SplashScreen>(result__)
        }
    }
    pub fn ActivatedOperation(&self) -> ::windows_core::Result<ActivatedOperation> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgsDeferral>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ActivatedOperation)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivatedOperation>(result__)
        }
    }
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "System"))]
    pub fn User(&self) -> ::windows_core::Result<::winrt_system::User> {
        let this = &::windows_core::Interface::cast::<::winrt_applicationmodel::Activation::IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).User)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_system::User>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Operation(&self) -> ::windows_core::Result<::winrt_applicationmodel::Activation::CommandLineActivationOperation> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Operation)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_applicationmodel::Activation::CommandLineActivationOperation>(result__)
        }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::clone::Clone for WebUICommandLineActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::cmp::PartialEq for WebUICommandLineActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::cmp::Eq for WebUICommandLineActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::fmt::Debug for WebUICommandLineActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WebUICommandLineActivatedEventArgs").field(&self.0).finish()
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows_core::RuntimeType for WebUICommandLineActivatedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.WebUI.WebUICommandLineActivatedEventArgs;{4506472c-006a-48eb-8afb-d07ab25e3366})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows_core::Interface for WebUICommandLineActivatedEventArgs {
    type Vtable = ::winrt_applicationmodel::Activation::ICommandLineActivatedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <::winrt_applicationmodel::Activation::ICommandLineActivatedEventArgs as ::windows_core::Interface>::IID;
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::RuntimeName for WebUICommandLineActivatedEventArgs {
    const NAME: &'static str = "Windows.UI.WebUI.WebUICommandLineActivatedEventArgs";
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<WebUICommandLineActivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: WebUICommandLineActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<&WebUICommandLineActivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &WebUICommandLineActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for WebUICommandLineActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a WebUICommandLineActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<WebUICommandLineActivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: WebUICommandLineActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<&WebUICommandLineActivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &WebUICommandLineActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for WebUICommandLineActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a WebUICommandLineActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUICommandLineActivatedEventArgs> for ::winrt_applicationmodel::Activation::IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: WebUICommandLineActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUICommandLineActivatedEventArgs> for ::winrt_applicationmodel::Activation::IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &WebUICommandLineActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgs> for WebUICommandLineActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgs> for &WebUICommandLineActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgs> {
        ::core::convert::TryInto::<::winrt_applicationmodel::Activation::IActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUICommandLineActivatedEventArgs> for IActivatedEventArgsDeferral {
    type Error = ::windows_core::Error;
    fn try_from(value: WebUICommandLineActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUICommandLineActivatedEventArgs> for IActivatedEventArgsDeferral {
    type Error = ::windows_core::Error;
    fn try_from(value: &WebUICommandLineActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgsDeferral> for WebUICommandLineActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgsDeferral> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgsDeferral> for &WebUICommandLineActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgsDeferral> {
        ::core::convert::TryInto::<IActivatedEventArgsDeferral>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUICommandLineActivatedEventArgs> for ::winrt_applicationmodel::Activation::IActivatedEventArgsWithUser {
    type Error = ::windows_core::Error;
    fn try_from(value: WebUICommandLineActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUICommandLineActivatedEventArgs> for ::winrt_applicationmodel::Activation::IActivatedEventArgsWithUser {
    type Error = ::windows_core::Error;
    fn try_from(value: &WebUICommandLineActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgsWithUser> for WebUICommandLineActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgsWithUser> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgsWithUser> for &WebUICommandLineActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgsWithUser> {
        ::core::convert::TryInto::<::winrt_applicationmodel::Activation::IActivatedEventArgsWithUser>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUICommandLineActivatedEventArgs> for ::winrt_applicationmodel::Activation::ICommandLineActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: WebUICommandLineActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUICommandLineActivatedEventArgs> for ::winrt_applicationmodel::Activation::ICommandLineActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &WebUICommandLineActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::ICommandLineActivatedEventArgs> for WebUICommandLineActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::ICommandLineActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::ICommandLineActivatedEventArgs> for &WebUICommandLineActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::ICommandLineActivatedEventArgs> {
        ::core::convert::TryInto::<::winrt_applicationmodel::Activation::ICommandLineActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::core::marker::Send for WebUICommandLineActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::core::marker::Sync for WebUICommandLineActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
#[repr(transparent)]
pub struct WebUIContactCallActivatedEventArgs(::windows_core::IUnknown);
#[cfg(feature = "ApplicationModel_Activation")]
impl WebUIContactCallActivatedEventArgs {
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Kind(&self) -> ::windows_core::Result<::winrt_applicationmodel::Activation::ActivationKind> {
        let this = &::windows_core::Interface::cast::<::winrt_applicationmodel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_applicationmodel::Activation::ActivationKind>::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_applicationmodel::Activation::ActivationKind>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<::winrt_applicationmodel::Activation::ApplicationExecutionState> {
        let this = &::windows_core::Interface::cast::<::winrt_applicationmodel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_applicationmodel::Activation::ApplicationExecutionState>::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_applicationmodel::Activation::ApplicationExecutionState>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn SplashScreen(&self) -> ::windows_core::Result<::winrt_applicationmodel::Activation::SplashScreen> {
        let this = &::windows_core::Interface::cast::<::winrt_applicationmodel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_applicationmodel::Activation::SplashScreen>(result__)
        }
    }
    pub fn ActivatedOperation(&self) -> ::windows_core::Result<ActivatedOperation> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgsDeferral>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ActivatedOperation)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivatedOperation>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Verb(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<::winrt_applicationmodel::Activation::IContactActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Verb)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn ServiceId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).ServiceId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn ServiceUserId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).ServiceUserId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "ApplicationModel_Contacts"))]
    pub fn Contact(&self) -> ::windows_core::Result<::winrt_applicationmodel::Contacts::Contact> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Contact)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_applicationmodel::Contacts::Contact>(result__)
        }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::clone::Clone for WebUIContactCallActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::cmp::PartialEq for WebUIContactCallActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::cmp::Eq for WebUIContactCallActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::fmt::Debug for WebUIContactCallActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WebUIContactCallActivatedEventArgs").field(&self.0).finish()
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows_core::RuntimeType for WebUIContactCallActivatedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.WebUI.WebUIContactCallActivatedEventArgs;{c2df14c7-30eb-41c6-b3bc-5b1694f9dab3})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows_core::Interface for WebUIContactCallActivatedEventArgs {
    type Vtable = ::winrt_applicationmodel::Activation::IContactCallActivatedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <::winrt_applicationmodel::Activation::IContactCallActivatedEventArgs as ::windows_core::Interface>::IID;
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::RuntimeName for WebUIContactCallActivatedEventArgs {
    const NAME: &'static str = "Windows.UI.WebUI.WebUIContactCallActivatedEventArgs";
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<WebUIContactCallActivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: WebUIContactCallActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<&WebUIContactCallActivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &WebUIContactCallActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for WebUIContactCallActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a WebUIContactCallActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<WebUIContactCallActivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: WebUIContactCallActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<&WebUIContactCallActivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &WebUIContactCallActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for WebUIContactCallActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a WebUIContactCallActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIContactCallActivatedEventArgs> for ::winrt_applicationmodel::Activation::IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: WebUIContactCallActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIContactCallActivatedEventArgs> for ::winrt_applicationmodel::Activation::IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &WebUIContactCallActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgs> for WebUIContactCallActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgs> for &WebUIContactCallActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgs> {
        ::core::convert::TryInto::<::winrt_applicationmodel::Activation::IActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIContactCallActivatedEventArgs> for IActivatedEventArgsDeferral {
    type Error = ::windows_core::Error;
    fn try_from(value: WebUIContactCallActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIContactCallActivatedEventArgs> for IActivatedEventArgsDeferral {
    type Error = ::windows_core::Error;
    fn try_from(value: &WebUIContactCallActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgsDeferral> for WebUIContactCallActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgsDeferral> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgsDeferral> for &WebUIContactCallActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgsDeferral> {
        ::core::convert::TryInto::<IActivatedEventArgsDeferral>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIContactCallActivatedEventArgs> for ::winrt_applicationmodel::Activation::IContactActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: WebUIContactCallActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIContactCallActivatedEventArgs> for ::winrt_applicationmodel::Activation::IContactActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &WebUIContactCallActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IContactActivatedEventArgs> for WebUIContactCallActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IContactActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IContactActivatedEventArgs> for &WebUIContactCallActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IContactActivatedEventArgs> {
        ::core::convert::TryInto::<::winrt_applicationmodel::Activation::IContactActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIContactCallActivatedEventArgs> for ::winrt_applicationmodel::Activation::IContactCallActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: WebUIContactCallActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIContactCallActivatedEventArgs> for ::winrt_applicationmodel::Activation::IContactCallActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &WebUIContactCallActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IContactCallActivatedEventArgs> for WebUIContactCallActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IContactCallActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IContactCallActivatedEventArgs> for &WebUIContactCallActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IContactCallActivatedEventArgs> {
        ::core::convert::TryInto::<::winrt_applicationmodel::Activation::IContactCallActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
#[repr(transparent)]
pub struct WebUIContactMapActivatedEventArgs(::windows_core::IUnknown);
#[cfg(feature = "ApplicationModel_Activation")]
impl WebUIContactMapActivatedEventArgs {
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Kind(&self) -> ::windows_core::Result<::winrt_applicationmodel::Activation::ActivationKind> {
        let this = &::windows_core::Interface::cast::<::winrt_applicationmodel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_applicationmodel::Activation::ActivationKind>::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_applicationmodel::Activation::ActivationKind>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<::winrt_applicationmodel::Activation::ApplicationExecutionState> {
        let this = &::windows_core::Interface::cast::<::winrt_applicationmodel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_applicationmodel::Activation::ApplicationExecutionState>::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_applicationmodel::Activation::ApplicationExecutionState>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn SplashScreen(&self) -> ::windows_core::Result<::winrt_applicationmodel::Activation::SplashScreen> {
        let this = &::windows_core::Interface::cast::<::winrt_applicationmodel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_applicationmodel::Activation::SplashScreen>(result__)
        }
    }
    pub fn ActivatedOperation(&self) -> ::windows_core::Result<ActivatedOperation> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgsDeferral>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ActivatedOperation)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivatedOperation>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Verb(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<::winrt_applicationmodel::Activation::IContactActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Verb)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "ApplicationModel_Contacts"))]
    pub fn Address(&self) -> ::windows_core::Result<::winrt_applicationmodel::Contacts::ContactAddress> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Address)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_applicationmodel::Contacts::ContactAddress>(result__)
        }
    }
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "ApplicationModel_Contacts"))]
    pub fn Contact(&self) -> ::windows_core::Result<::winrt_applicationmodel::Contacts::Contact> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Contact)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_applicationmodel::Contacts::Contact>(result__)
        }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::clone::Clone for WebUIContactMapActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::cmp::PartialEq for WebUIContactMapActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::cmp::Eq for WebUIContactMapActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::fmt::Debug for WebUIContactMapActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WebUIContactMapActivatedEventArgs").field(&self.0).finish()
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows_core::RuntimeType for WebUIContactMapActivatedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.WebUI.WebUIContactMapActivatedEventArgs;{b32bf870-eee7-4ad2-aaf1-a87effcf00a4})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows_core::Interface for WebUIContactMapActivatedEventArgs {
    type Vtable = ::winrt_applicationmodel::Activation::IContactMapActivatedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <::winrt_applicationmodel::Activation::IContactMapActivatedEventArgs as ::windows_core::Interface>::IID;
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::RuntimeName for WebUIContactMapActivatedEventArgs {
    const NAME: &'static str = "Windows.UI.WebUI.WebUIContactMapActivatedEventArgs";
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<WebUIContactMapActivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: WebUIContactMapActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<&WebUIContactMapActivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &WebUIContactMapActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for WebUIContactMapActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a WebUIContactMapActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<WebUIContactMapActivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: WebUIContactMapActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<&WebUIContactMapActivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &WebUIContactMapActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for WebUIContactMapActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a WebUIContactMapActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIContactMapActivatedEventArgs> for ::winrt_applicationmodel::Activation::IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: WebUIContactMapActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIContactMapActivatedEventArgs> for ::winrt_applicationmodel::Activation::IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &WebUIContactMapActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgs> for WebUIContactMapActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgs> for &WebUIContactMapActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgs> {
        ::core::convert::TryInto::<::winrt_applicationmodel::Activation::IActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIContactMapActivatedEventArgs> for IActivatedEventArgsDeferral {
    type Error = ::windows_core::Error;
    fn try_from(value: WebUIContactMapActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIContactMapActivatedEventArgs> for IActivatedEventArgsDeferral {
    type Error = ::windows_core::Error;
    fn try_from(value: &WebUIContactMapActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgsDeferral> for WebUIContactMapActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgsDeferral> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgsDeferral> for &WebUIContactMapActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgsDeferral> {
        ::core::convert::TryInto::<IActivatedEventArgsDeferral>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIContactMapActivatedEventArgs> for ::winrt_applicationmodel::Activation::IContactActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: WebUIContactMapActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIContactMapActivatedEventArgs> for ::winrt_applicationmodel::Activation::IContactActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &WebUIContactMapActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IContactActivatedEventArgs> for WebUIContactMapActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IContactActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IContactActivatedEventArgs> for &WebUIContactMapActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IContactActivatedEventArgs> {
        ::core::convert::TryInto::<::winrt_applicationmodel::Activation::IContactActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIContactMapActivatedEventArgs> for ::winrt_applicationmodel::Activation::IContactMapActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: WebUIContactMapActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIContactMapActivatedEventArgs> for ::winrt_applicationmodel::Activation::IContactMapActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &WebUIContactMapActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IContactMapActivatedEventArgs> for WebUIContactMapActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IContactMapActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IContactMapActivatedEventArgs> for &WebUIContactMapActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IContactMapActivatedEventArgs> {
        ::core::convert::TryInto::<::winrt_applicationmodel::Activation::IContactMapActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
#[repr(transparent)]
pub struct WebUIContactMessageActivatedEventArgs(::windows_core::IUnknown);
#[cfg(feature = "ApplicationModel_Activation")]
impl WebUIContactMessageActivatedEventArgs {
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Kind(&self) -> ::windows_core::Result<::winrt_applicationmodel::Activation::ActivationKind> {
        let this = &::windows_core::Interface::cast::<::winrt_applicationmodel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_applicationmodel::Activation::ActivationKind>::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_applicationmodel::Activation::ActivationKind>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<::winrt_applicationmodel::Activation::ApplicationExecutionState> {
        let this = &::windows_core::Interface::cast::<::winrt_applicationmodel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_applicationmodel::Activation::ApplicationExecutionState>::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_applicationmodel::Activation::ApplicationExecutionState>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn SplashScreen(&self) -> ::windows_core::Result<::winrt_applicationmodel::Activation::SplashScreen> {
        let this = &::windows_core::Interface::cast::<::winrt_applicationmodel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_applicationmodel::Activation::SplashScreen>(result__)
        }
    }
    pub fn ActivatedOperation(&self) -> ::windows_core::Result<ActivatedOperation> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgsDeferral>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ActivatedOperation)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivatedOperation>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Verb(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<::winrt_applicationmodel::Activation::IContactActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Verb)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn ServiceId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).ServiceId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn ServiceUserId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).ServiceUserId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "ApplicationModel_Contacts"))]
    pub fn Contact(&self) -> ::windows_core::Result<::winrt_applicationmodel::Contacts::Contact> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Contact)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_applicationmodel::Contacts::Contact>(result__)
        }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::clone::Clone for WebUIContactMessageActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::cmp::PartialEq for WebUIContactMessageActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::cmp::Eq for WebUIContactMessageActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::fmt::Debug for WebUIContactMessageActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WebUIContactMessageActivatedEventArgs").field(&self.0).finish()
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows_core::RuntimeType for WebUIContactMessageActivatedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.WebUI.WebUIContactMessageActivatedEventArgs;{de598db2-0e03-43b0-bf56-bcc40b3162df})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows_core::Interface for WebUIContactMessageActivatedEventArgs {
    type Vtable = ::winrt_applicationmodel::Activation::IContactMessageActivatedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <::winrt_applicationmodel::Activation::IContactMessageActivatedEventArgs as ::windows_core::Interface>::IID;
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::RuntimeName for WebUIContactMessageActivatedEventArgs {
    const NAME: &'static str = "Windows.UI.WebUI.WebUIContactMessageActivatedEventArgs";
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<WebUIContactMessageActivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: WebUIContactMessageActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<&WebUIContactMessageActivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &WebUIContactMessageActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for WebUIContactMessageActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a WebUIContactMessageActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<WebUIContactMessageActivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: WebUIContactMessageActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<&WebUIContactMessageActivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &WebUIContactMessageActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for WebUIContactMessageActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a WebUIContactMessageActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIContactMessageActivatedEventArgs> for ::winrt_applicationmodel::Activation::IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: WebUIContactMessageActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIContactMessageActivatedEventArgs> for ::winrt_applicationmodel::Activation::IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &WebUIContactMessageActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgs> for WebUIContactMessageActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgs> for &WebUIContactMessageActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgs> {
        ::core::convert::TryInto::<::winrt_applicationmodel::Activation::IActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIContactMessageActivatedEventArgs> for IActivatedEventArgsDeferral {
    type Error = ::windows_core::Error;
    fn try_from(value: WebUIContactMessageActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIContactMessageActivatedEventArgs> for IActivatedEventArgsDeferral {
    type Error = ::windows_core::Error;
    fn try_from(value: &WebUIContactMessageActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgsDeferral> for WebUIContactMessageActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgsDeferral> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgsDeferral> for &WebUIContactMessageActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgsDeferral> {
        ::core::convert::TryInto::<IActivatedEventArgsDeferral>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIContactMessageActivatedEventArgs> for ::winrt_applicationmodel::Activation::IContactActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: WebUIContactMessageActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIContactMessageActivatedEventArgs> for ::winrt_applicationmodel::Activation::IContactActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &WebUIContactMessageActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IContactActivatedEventArgs> for WebUIContactMessageActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IContactActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IContactActivatedEventArgs> for &WebUIContactMessageActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IContactActivatedEventArgs> {
        ::core::convert::TryInto::<::winrt_applicationmodel::Activation::IContactActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIContactMessageActivatedEventArgs> for ::winrt_applicationmodel::Activation::IContactMessageActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: WebUIContactMessageActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIContactMessageActivatedEventArgs> for ::winrt_applicationmodel::Activation::IContactMessageActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &WebUIContactMessageActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IContactMessageActivatedEventArgs> for WebUIContactMessageActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IContactMessageActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IContactMessageActivatedEventArgs> for &WebUIContactMessageActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IContactMessageActivatedEventArgs> {
        ::core::convert::TryInto::<::winrt_applicationmodel::Activation::IContactMessageActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
#[repr(transparent)]
pub struct WebUIContactPanelActivatedEventArgs(::windows_core::IUnknown);
#[cfg(feature = "ApplicationModel_Activation")]
impl WebUIContactPanelActivatedEventArgs {
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Kind(&self) -> ::windows_core::Result<::winrt_applicationmodel::Activation::ActivationKind> {
        let this = &::windows_core::Interface::cast::<::winrt_applicationmodel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_applicationmodel::Activation::ActivationKind>::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_applicationmodel::Activation::ActivationKind>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<::winrt_applicationmodel::Activation::ApplicationExecutionState> {
        let this = &::windows_core::Interface::cast::<::winrt_applicationmodel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_applicationmodel::Activation::ApplicationExecutionState>::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_applicationmodel::Activation::ApplicationExecutionState>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn SplashScreen(&self) -> ::windows_core::Result<::winrt_applicationmodel::Activation::SplashScreen> {
        let this = &::windows_core::Interface::cast::<::winrt_applicationmodel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_applicationmodel::Activation::SplashScreen>(result__)
        }
    }
    pub fn ActivatedOperation(&self) -> ::windows_core::Result<ActivatedOperation> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgsDeferral>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ActivatedOperation)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivatedOperation>(result__)
        }
    }
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "System"))]
    pub fn User(&self) -> ::windows_core::Result<::winrt_system::User> {
        let this = &::windows_core::Interface::cast::<::winrt_applicationmodel::Activation::IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).User)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_system::User>(result__)
        }
    }
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "ApplicationModel_Contacts"))]
    pub fn ContactPanel(&self) -> ::windows_core::Result<::winrt_applicationmodel::Contacts::ContactPanel> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ContactPanel)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_applicationmodel::Contacts::ContactPanel>(result__)
        }
    }
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "ApplicationModel_Contacts"))]
    pub fn Contact(&self) -> ::windows_core::Result<::winrt_applicationmodel::Contacts::Contact> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Contact)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_applicationmodel::Contacts::Contact>(result__)
        }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::clone::Clone for WebUIContactPanelActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::cmp::PartialEq for WebUIContactPanelActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::cmp::Eq for WebUIContactPanelActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::fmt::Debug for WebUIContactPanelActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WebUIContactPanelActivatedEventArgs").field(&self.0).finish()
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows_core::RuntimeType for WebUIContactPanelActivatedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.WebUI.WebUIContactPanelActivatedEventArgs;{52bb63e4-d3d4-4b63-8051-4af2082cab80})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows_core::Interface for WebUIContactPanelActivatedEventArgs {
    type Vtable = ::winrt_applicationmodel::Activation::IContactPanelActivatedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <::winrt_applicationmodel::Activation::IContactPanelActivatedEventArgs as ::windows_core::Interface>::IID;
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::RuntimeName for WebUIContactPanelActivatedEventArgs {
    const NAME: &'static str = "Windows.UI.WebUI.WebUIContactPanelActivatedEventArgs";
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<WebUIContactPanelActivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: WebUIContactPanelActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<&WebUIContactPanelActivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &WebUIContactPanelActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for WebUIContactPanelActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a WebUIContactPanelActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<WebUIContactPanelActivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: WebUIContactPanelActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<&WebUIContactPanelActivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &WebUIContactPanelActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for WebUIContactPanelActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a WebUIContactPanelActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIContactPanelActivatedEventArgs> for ::winrt_applicationmodel::Activation::IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: WebUIContactPanelActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIContactPanelActivatedEventArgs> for ::winrt_applicationmodel::Activation::IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &WebUIContactPanelActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgs> for WebUIContactPanelActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgs> for &WebUIContactPanelActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgs> {
        ::core::convert::TryInto::<::winrt_applicationmodel::Activation::IActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIContactPanelActivatedEventArgs> for IActivatedEventArgsDeferral {
    type Error = ::windows_core::Error;
    fn try_from(value: WebUIContactPanelActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIContactPanelActivatedEventArgs> for IActivatedEventArgsDeferral {
    type Error = ::windows_core::Error;
    fn try_from(value: &WebUIContactPanelActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgsDeferral> for WebUIContactPanelActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgsDeferral> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgsDeferral> for &WebUIContactPanelActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgsDeferral> {
        ::core::convert::TryInto::<IActivatedEventArgsDeferral>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIContactPanelActivatedEventArgs> for ::winrt_applicationmodel::Activation::IActivatedEventArgsWithUser {
    type Error = ::windows_core::Error;
    fn try_from(value: WebUIContactPanelActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIContactPanelActivatedEventArgs> for ::winrt_applicationmodel::Activation::IActivatedEventArgsWithUser {
    type Error = ::windows_core::Error;
    fn try_from(value: &WebUIContactPanelActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgsWithUser> for WebUIContactPanelActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgsWithUser> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgsWithUser> for &WebUIContactPanelActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgsWithUser> {
        ::core::convert::TryInto::<::winrt_applicationmodel::Activation::IActivatedEventArgsWithUser>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIContactPanelActivatedEventArgs> for ::winrt_applicationmodel::Activation::IContactPanelActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: WebUIContactPanelActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIContactPanelActivatedEventArgs> for ::winrt_applicationmodel::Activation::IContactPanelActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &WebUIContactPanelActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IContactPanelActivatedEventArgs> for WebUIContactPanelActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IContactPanelActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IContactPanelActivatedEventArgs> for &WebUIContactPanelActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IContactPanelActivatedEventArgs> {
        ::core::convert::TryInto::<::winrt_applicationmodel::Activation::IContactPanelActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::core::marker::Send for WebUIContactPanelActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::core::marker::Sync for WebUIContactPanelActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
#[repr(transparent)]
pub struct WebUIContactPickerActivatedEventArgs(::windows_core::IUnknown);
#[cfg(feature = "ApplicationModel_Activation")]
impl WebUIContactPickerActivatedEventArgs {
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Kind(&self) -> ::windows_core::Result<::winrt_applicationmodel::Activation::ActivationKind> {
        let this = &::windows_core::Interface::cast::<::winrt_applicationmodel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_applicationmodel::Activation::ActivationKind>::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_applicationmodel::Activation::ActivationKind>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<::winrt_applicationmodel::Activation::ApplicationExecutionState> {
        let this = &::windows_core::Interface::cast::<::winrt_applicationmodel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_applicationmodel::Activation::ApplicationExecutionState>::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_applicationmodel::Activation::ApplicationExecutionState>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn SplashScreen(&self) -> ::windows_core::Result<::winrt_applicationmodel::Activation::SplashScreen> {
        let this = &::windows_core::Interface::cast::<::winrt_applicationmodel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_applicationmodel::Activation::SplashScreen>(result__)
        }
    }
    pub fn ActivatedOperation(&self) -> ::windows_core::Result<ActivatedOperation> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgsDeferral>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ActivatedOperation)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivatedOperation>(result__)
        }
    }
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "ApplicationModel_Contacts_Provider"))]
    pub fn ContactPickerUI(&self) -> ::windows_core::Result<::winrt_applicationmodel::Contacts::Provider::ContactPickerUI> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ContactPickerUI)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_applicationmodel::Contacts::Provider::ContactPickerUI>(result__)
        }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::clone::Clone for WebUIContactPickerActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::cmp::PartialEq for WebUIContactPickerActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::cmp::Eq for WebUIContactPickerActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::fmt::Debug for WebUIContactPickerActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WebUIContactPickerActivatedEventArgs").field(&self.0).finish()
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows_core::RuntimeType for WebUIContactPickerActivatedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.WebUI.WebUIContactPickerActivatedEventArgs;{ce57aae7-6449-45a7-971f-d113be7a8936})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows_core::Interface for WebUIContactPickerActivatedEventArgs {
    type Vtable = ::winrt_applicationmodel::Activation::IContactPickerActivatedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <::winrt_applicationmodel::Activation::IContactPickerActivatedEventArgs as ::windows_core::Interface>::IID;
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::RuntimeName for WebUIContactPickerActivatedEventArgs {
    const NAME: &'static str = "Windows.UI.WebUI.WebUIContactPickerActivatedEventArgs";
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<WebUIContactPickerActivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: WebUIContactPickerActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<&WebUIContactPickerActivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &WebUIContactPickerActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for WebUIContactPickerActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a WebUIContactPickerActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<WebUIContactPickerActivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: WebUIContactPickerActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<&WebUIContactPickerActivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &WebUIContactPickerActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for WebUIContactPickerActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a WebUIContactPickerActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIContactPickerActivatedEventArgs> for ::winrt_applicationmodel::Activation::IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: WebUIContactPickerActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIContactPickerActivatedEventArgs> for ::winrt_applicationmodel::Activation::IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &WebUIContactPickerActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgs> for WebUIContactPickerActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgs> for &WebUIContactPickerActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgs> {
        ::core::convert::TryInto::<::winrt_applicationmodel::Activation::IActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIContactPickerActivatedEventArgs> for IActivatedEventArgsDeferral {
    type Error = ::windows_core::Error;
    fn try_from(value: WebUIContactPickerActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIContactPickerActivatedEventArgs> for IActivatedEventArgsDeferral {
    type Error = ::windows_core::Error;
    fn try_from(value: &WebUIContactPickerActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgsDeferral> for WebUIContactPickerActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgsDeferral> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgsDeferral> for &WebUIContactPickerActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgsDeferral> {
        ::core::convert::TryInto::<IActivatedEventArgsDeferral>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIContactPickerActivatedEventArgs> for ::winrt_applicationmodel::Activation::IContactPickerActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: WebUIContactPickerActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIContactPickerActivatedEventArgs> for ::winrt_applicationmodel::Activation::IContactPickerActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &WebUIContactPickerActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IContactPickerActivatedEventArgs> for WebUIContactPickerActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IContactPickerActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IContactPickerActivatedEventArgs> for &WebUIContactPickerActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IContactPickerActivatedEventArgs> {
        ::core::convert::TryInto::<::winrt_applicationmodel::Activation::IContactPickerActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
#[repr(transparent)]
pub struct WebUIContactPostActivatedEventArgs(::windows_core::IUnknown);
#[cfg(feature = "ApplicationModel_Activation")]
impl WebUIContactPostActivatedEventArgs {
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Kind(&self) -> ::windows_core::Result<::winrt_applicationmodel::Activation::ActivationKind> {
        let this = &::windows_core::Interface::cast::<::winrt_applicationmodel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_applicationmodel::Activation::ActivationKind>::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_applicationmodel::Activation::ActivationKind>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<::winrt_applicationmodel::Activation::ApplicationExecutionState> {
        let this = &::windows_core::Interface::cast::<::winrt_applicationmodel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_applicationmodel::Activation::ApplicationExecutionState>::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_applicationmodel::Activation::ApplicationExecutionState>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn SplashScreen(&self) -> ::windows_core::Result<::winrt_applicationmodel::Activation::SplashScreen> {
        let this = &::windows_core::Interface::cast::<::winrt_applicationmodel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_applicationmodel::Activation::SplashScreen>(result__)
        }
    }
    pub fn ActivatedOperation(&self) -> ::windows_core::Result<ActivatedOperation> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgsDeferral>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ActivatedOperation)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivatedOperation>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Verb(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<::winrt_applicationmodel::Activation::IContactActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Verb)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn ServiceId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).ServiceId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn ServiceUserId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).ServiceUserId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "ApplicationModel_Contacts"))]
    pub fn Contact(&self) -> ::windows_core::Result<::winrt_applicationmodel::Contacts::Contact> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Contact)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_applicationmodel::Contacts::Contact>(result__)
        }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::clone::Clone for WebUIContactPostActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::cmp::PartialEq for WebUIContactPostActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::cmp::Eq for WebUIContactPostActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::fmt::Debug for WebUIContactPostActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WebUIContactPostActivatedEventArgs").field(&self.0).finish()
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows_core::RuntimeType for WebUIContactPostActivatedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.WebUI.WebUIContactPostActivatedEventArgs;{b35a3c67-f1e7-4655-ad6e-4857588f552f})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows_core::Interface for WebUIContactPostActivatedEventArgs {
    type Vtable = ::winrt_applicationmodel::Activation::IContactPostActivatedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <::winrt_applicationmodel::Activation::IContactPostActivatedEventArgs as ::windows_core::Interface>::IID;
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::RuntimeName for WebUIContactPostActivatedEventArgs {
    const NAME: &'static str = "Windows.UI.WebUI.WebUIContactPostActivatedEventArgs";
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<WebUIContactPostActivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: WebUIContactPostActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<&WebUIContactPostActivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &WebUIContactPostActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for WebUIContactPostActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a WebUIContactPostActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<WebUIContactPostActivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: WebUIContactPostActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<&WebUIContactPostActivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &WebUIContactPostActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for WebUIContactPostActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a WebUIContactPostActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIContactPostActivatedEventArgs> for ::winrt_applicationmodel::Activation::IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: WebUIContactPostActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIContactPostActivatedEventArgs> for ::winrt_applicationmodel::Activation::IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &WebUIContactPostActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgs> for WebUIContactPostActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgs> for &WebUIContactPostActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgs> {
        ::core::convert::TryInto::<::winrt_applicationmodel::Activation::IActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIContactPostActivatedEventArgs> for IActivatedEventArgsDeferral {
    type Error = ::windows_core::Error;
    fn try_from(value: WebUIContactPostActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIContactPostActivatedEventArgs> for IActivatedEventArgsDeferral {
    type Error = ::windows_core::Error;
    fn try_from(value: &WebUIContactPostActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgsDeferral> for WebUIContactPostActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgsDeferral> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgsDeferral> for &WebUIContactPostActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgsDeferral> {
        ::core::convert::TryInto::<IActivatedEventArgsDeferral>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIContactPostActivatedEventArgs> for ::winrt_applicationmodel::Activation::IContactActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: WebUIContactPostActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIContactPostActivatedEventArgs> for ::winrt_applicationmodel::Activation::IContactActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &WebUIContactPostActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IContactActivatedEventArgs> for WebUIContactPostActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IContactActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IContactActivatedEventArgs> for &WebUIContactPostActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IContactActivatedEventArgs> {
        ::core::convert::TryInto::<::winrt_applicationmodel::Activation::IContactActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIContactPostActivatedEventArgs> for ::winrt_applicationmodel::Activation::IContactPostActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: WebUIContactPostActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIContactPostActivatedEventArgs> for ::winrt_applicationmodel::Activation::IContactPostActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &WebUIContactPostActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IContactPostActivatedEventArgs> for WebUIContactPostActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IContactPostActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IContactPostActivatedEventArgs> for &WebUIContactPostActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IContactPostActivatedEventArgs> {
        ::core::convert::TryInto::<::winrt_applicationmodel::Activation::IContactPostActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
#[repr(transparent)]
pub struct WebUIContactVideoCallActivatedEventArgs(::windows_core::IUnknown);
#[cfg(feature = "ApplicationModel_Activation")]
impl WebUIContactVideoCallActivatedEventArgs {
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Kind(&self) -> ::windows_core::Result<::winrt_applicationmodel::Activation::ActivationKind> {
        let this = &::windows_core::Interface::cast::<::winrt_applicationmodel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_applicationmodel::Activation::ActivationKind>::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_applicationmodel::Activation::ActivationKind>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<::winrt_applicationmodel::Activation::ApplicationExecutionState> {
        let this = &::windows_core::Interface::cast::<::winrt_applicationmodel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_applicationmodel::Activation::ApplicationExecutionState>::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_applicationmodel::Activation::ApplicationExecutionState>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn SplashScreen(&self) -> ::windows_core::Result<::winrt_applicationmodel::Activation::SplashScreen> {
        let this = &::windows_core::Interface::cast::<::winrt_applicationmodel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_applicationmodel::Activation::SplashScreen>(result__)
        }
    }
    pub fn ActivatedOperation(&self) -> ::windows_core::Result<ActivatedOperation> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgsDeferral>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ActivatedOperation)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivatedOperation>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Verb(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<::winrt_applicationmodel::Activation::IContactActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Verb)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn ServiceId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).ServiceId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn ServiceUserId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).ServiceUserId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "ApplicationModel_Contacts"))]
    pub fn Contact(&self) -> ::windows_core::Result<::winrt_applicationmodel::Contacts::Contact> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Contact)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_applicationmodel::Contacts::Contact>(result__)
        }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::clone::Clone for WebUIContactVideoCallActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::cmp::PartialEq for WebUIContactVideoCallActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::cmp::Eq for WebUIContactVideoCallActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::fmt::Debug for WebUIContactVideoCallActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WebUIContactVideoCallActivatedEventArgs").field(&self.0).finish()
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows_core::RuntimeType for WebUIContactVideoCallActivatedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.WebUI.WebUIContactVideoCallActivatedEventArgs;{61079db8-e3e7-4b4f-858d-5c63a96ef684})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows_core::Interface for WebUIContactVideoCallActivatedEventArgs {
    type Vtable = ::winrt_applicationmodel::Activation::IContactVideoCallActivatedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <::winrt_applicationmodel::Activation::IContactVideoCallActivatedEventArgs as ::windows_core::Interface>::IID;
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::RuntimeName for WebUIContactVideoCallActivatedEventArgs {
    const NAME: &'static str = "Windows.UI.WebUI.WebUIContactVideoCallActivatedEventArgs";
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<WebUIContactVideoCallActivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: WebUIContactVideoCallActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<&WebUIContactVideoCallActivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &WebUIContactVideoCallActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for WebUIContactVideoCallActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a WebUIContactVideoCallActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<WebUIContactVideoCallActivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: WebUIContactVideoCallActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<&WebUIContactVideoCallActivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &WebUIContactVideoCallActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for WebUIContactVideoCallActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a WebUIContactVideoCallActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIContactVideoCallActivatedEventArgs> for ::winrt_applicationmodel::Activation::IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: WebUIContactVideoCallActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIContactVideoCallActivatedEventArgs> for ::winrt_applicationmodel::Activation::IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &WebUIContactVideoCallActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgs> for WebUIContactVideoCallActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgs> for &WebUIContactVideoCallActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgs> {
        ::core::convert::TryInto::<::winrt_applicationmodel::Activation::IActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIContactVideoCallActivatedEventArgs> for IActivatedEventArgsDeferral {
    type Error = ::windows_core::Error;
    fn try_from(value: WebUIContactVideoCallActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIContactVideoCallActivatedEventArgs> for IActivatedEventArgsDeferral {
    type Error = ::windows_core::Error;
    fn try_from(value: &WebUIContactVideoCallActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgsDeferral> for WebUIContactVideoCallActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgsDeferral> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgsDeferral> for &WebUIContactVideoCallActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgsDeferral> {
        ::core::convert::TryInto::<IActivatedEventArgsDeferral>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIContactVideoCallActivatedEventArgs> for ::winrt_applicationmodel::Activation::IContactActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: WebUIContactVideoCallActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIContactVideoCallActivatedEventArgs> for ::winrt_applicationmodel::Activation::IContactActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &WebUIContactVideoCallActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IContactActivatedEventArgs> for WebUIContactVideoCallActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IContactActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IContactActivatedEventArgs> for &WebUIContactVideoCallActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IContactActivatedEventArgs> {
        ::core::convert::TryInto::<::winrt_applicationmodel::Activation::IContactActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIContactVideoCallActivatedEventArgs> for ::winrt_applicationmodel::Activation::IContactVideoCallActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: WebUIContactVideoCallActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIContactVideoCallActivatedEventArgs> for ::winrt_applicationmodel::Activation::IContactVideoCallActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &WebUIContactVideoCallActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IContactVideoCallActivatedEventArgs> for WebUIContactVideoCallActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IContactVideoCallActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IContactVideoCallActivatedEventArgs> for &WebUIContactVideoCallActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IContactVideoCallActivatedEventArgs> {
        ::core::convert::TryInto::<::winrt_applicationmodel::Activation::IContactVideoCallActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
#[repr(transparent)]
pub struct WebUIDeviceActivatedEventArgs(::windows_core::IUnknown);
#[cfg(feature = "ApplicationModel_Activation")]
impl WebUIDeviceActivatedEventArgs {
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Kind(&self) -> ::windows_core::Result<::winrt_applicationmodel::Activation::ActivationKind> {
        let this = &::windows_core::Interface::cast::<::winrt_applicationmodel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_applicationmodel::Activation::ActivationKind>::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_applicationmodel::Activation::ActivationKind>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<::winrt_applicationmodel::Activation::ApplicationExecutionState> {
        let this = &::windows_core::Interface::cast::<::winrt_applicationmodel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_applicationmodel::Activation::ApplicationExecutionState>::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_applicationmodel::Activation::ApplicationExecutionState>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn SplashScreen(&self) -> ::windows_core::Result<::winrt_applicationmodel::Activation::SplashScreen> {
        let this = &::windows_core::Interface::cast::<::winrt_applicationmodel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_applicationmodel::Activation::SplashScreen>(result__)
        }
    }
    pub fn ActivatedOperation(&self) -> ::windows_core::Result<ActivatedOperation> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgsDeferral>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ActivatedOperation)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivatedOperation>(result__)
        }
    }
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "System"))]
    pub fn User(&self) -> ::windows_core::Result<::winrt_system::User> {
        let this = &::windows_core::Interface::cast::<::winrt_applicationmodel::Activation::IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).User)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_system::User>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn CurrentlyShownApplicationViewId(&self) -> ::windows_core::Result<i32> {
        let this = &::windows_core::Interface::cast::<::winrt_applicationmodel::Activation::IApplicationViewActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows_core::Interface::vtable(this).CurrentlyShownApplicationViewId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn DeviceInformationId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).DeviceInformationId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Verb(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Verb)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::clone::Clone for WebUIDeviceActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::cmp::PartialEq for WebUIDeviceActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::cmp::Eq for WebUIDeviceActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::fmt::Debug for WebUIDeviceActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WebUIDeviceActivatedEventArgs").field(&self.0).finish()
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows_core::RuntimeType for WebUIDeviceActivatedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.WebUI.WebUIDeviceActivatedEventArgs;{cd50b9a9-ce10-44d2-8234-c355a073ef33})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows_core::Interface for WebUIDeviceActivatedEventArgs {
    type Vtable = ::winrt_applicationmodel::Activation::IDeviceActivatedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <::winrt_applicationmodel::Activation::IDeviceActivatedEventArgs as ::windows_core::Interface>::IID;
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::RuntimeName for WebUIDeviceActivatedEventArgs {
    const NAME: &'static str = "Windows.UI.WebUI.WebUIDeviceActivatedEventArgs";
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<WebUIDeviceActivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: WebUIDeviceActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<&WebUIDeviceActivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &WebUIDeviceActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for WebUIDeviceActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a WebUIDeviceActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<WebUIDeviceActivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: WebUIDeviceActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<&WebUIDeviceActivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &WebUIDeviceActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for WebUIDeviceActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a WebUIDeviceActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIDeviceActivatedEventArgs> for ::winrt_applicationmodel::Activation::IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: WebUIDeviceActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIDeviceActivatedEventArgs> for ::winrt_applicationmodel::Activation::IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &WebUIDeviceActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgs> for WebUIDeviceActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgs> for &WebUIDeviceActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgs> {
        ::core::convert::TryInto::<::winrt_applicationmodel::Activation::IActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIDeviceActivatedEventArgs> for IActivatedEventArgsDeferral {
    type Error = ::windows_core::Error;
    fn try_from(value: WebUIDeviceActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIDeviceActivatedEventArgs> for IActivatedEventArgsDeferral {
    type Error = ::windows_core::Error;
    fn try_from(value: &WebUIDeviceActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgsDeferral> for WebUIDeviceActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgsDeferral> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgsDeferral> for &WebUIDeviceActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgsDeferral> {
        ::core::convert::TryInto::<IActivatedEventArgsDeferral>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIDeviceActivatedEventArgs> for ::winrt_applicationmodel::Activation::IActivatedEventArgsWithUser {
    type Error = ::windows_core::Error;
    fn try_from(value: WebUIDeviceActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIDeviceActivatedEventArgs> for ::winrt_applicationmodel::Activation::IActivatedEventArgsWithUser {
    type Error = ::windows_core::Error;
    fn try_from(value: &WebUIDeviceActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgsWithUser> for WebUIDeviceActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgsWithUser> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgsWithUser> for &WebUIDeviceActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgsWithUser> {
        ::core::convert::TryInto::<::winrt_applicationmodel::Activation::IActivatedEventArgsWithUser>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIDeviceActivatedEventArgs> for ::winrt_applicationmodel::Activation::IApplicationViewActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: WebUIDeviceActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIDeviceActivatedEventArgs> for ::winrt_applicationmodel::Activation::IApplicationViewActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &WebUIDeviceActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IApplicationViewActivatedEventArgs> for WebUIDeviceActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IApplicationViewActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IApplicationViewActivatedEventArgs> for &WebUIDeviceActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IApplicationViewActivatedEventArgs> {
        ::core::convert::TryInto::<::winrt_applicationmodel::Activation::IApplicationViewActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIDeviceActivatedEventArgs> for ::winrt_applicationmodel::Activation::IDeviceActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: WebUIDeviceActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIDeviceActivatedEventArgs> for ::winrt_applicationmodel::Activation::IDeviceActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &WebUIDeviceActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IDeviceActivatedEventArgs> for WebUIDeviceActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IDeviceActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IDeviceActivatedEventArgs> for &WebUIDeviceActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IDeviceActivatedEventArgs> {
        ::core::convert::TryInto::<::winrt_applicationmodel::Activation::IDeviceActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
#[repr(transparent)]
pub struct WebUIDevicePairingActivatedEventArgs(::windows_core::IUnknown);
#[cfg(feature = "ApplicationModel_Activation")]
impl WebUIDevicePairingActivatedEventArgs {
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Kind(&self) -> ::windows_core::Result<::winrt_applicationmodel::Activation::ActivationKind> {
        let this = &::windows_core::Interface::cast::<::winrt_applicationmodel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_applicationmodel::Activation::ActivationKind>::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_applicationmodel::Activation::ActivationKind>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<::winrt_applicationmodel::Activation::ApplicationExecutionState> {
        let this = &::windows_core::Interface::cast::<::winrt_applicationmodel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_applicationmodel::Activation::ApplicationExecutionState>::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_applicationmodel::Activation::ApplicationExecutionState>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn SplashScreen(&self) -> ::windows_core::Result<::winrt_applicationmodel::Activation::SplashScreen> {
        let this = &::windows_core::Interface::cast::<::winrt_applicationmodel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_applicationmodel::Activation::SplashScreen>(result__)
        }
    }
    pub fn ActivatedOperation(&self) -> ::windows_core::Result<ActivatedOperation> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgsDeferral>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ActivatedOperation)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivatedOperation>(result__)
        }
    }
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "System"))]
    pub fn User(&self) -> ::windows_core::Result<::winrt_system::User> {
        let this = &::windows_core::Interface::cast::<::winrt_applicationmodel::Activation::IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).User)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_system::User>(result__)
        }
    }
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "Devices_Enumeration"))]
    pub fn DeviceInformation(&self) -> ::windows_core::Result<::winrt_devices::Enumeration::DeviceInformation> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).DeviceInformation)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_devices::Enumeration::DeviceInformation>(result__)
        }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::clone::Clone for WebUIDevicePairingActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::cmp::PartialEq for WebUIDevicePairingActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::cmp::Eq for WebUIDevicePairingActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::fmt::Debug for WebUIDevicePairingActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WebUIDevicePairingActivatedEventArgs").field(&self.0).finish()
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows_core::RuntimeType for WebUIDevicePairingActivatedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.WebUI.WebUIDevicePairingActivatedEventArgs;{eba0d1e4-ecc6-4148-94ed-f4b37ec05b3e})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows_core::Interface for WebUIDevicePairingActivatedEventArgs {
    type Vtable = ::winrt_applicationmodel::Activation::IDevicePairingActivatedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <::winrt_applicationmodel::Activation::IDevicePairingActivatedEventArgs as ::windows_core::Interface>::IID;
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::RuntimeName for WebUIDevicePairingActivatedEventArgs {
    const NAME: &'static str = "Windows.UI.WebUI.WebUIDevicePairingActivatedEventArgs";
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<WebUIDevicePairingActivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: WebUIDevicePairingActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<&WebUIDevicePairingActivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &WebUIDevicePairingActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for WebUIDevicePairingActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a WebUIDevicePairingActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<WebUIDevicePairingActivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: WebUIDevicePairingActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<&WebUIDevicePairingActivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &WebUIDevicePairingActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for WebUIDevicePairingActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a WebUIDevicePairingActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIDevicePairingActivatedEventArgs> for ::winrt_applicationmodel::Activation::IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: WebUIDevicePairingActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIDevicePairingActivatedEventArgs> for ::winrt_applicationmodel::Activation::IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &WebUIDevicePairingActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgs> for WebUIDevicePairingActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgs> for &WebUIDevicePairingActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgs> {
        ::core::convert::TryInto::<::winrt_applicationmodel::Activation::IActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIDevicePairingActivatedEventArgs> for IActivatedEventArgsDeferral {
    type Error = ::windows_core::Error;
    fn try_from(value: WebUIDevicePairingActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIDevicePairingActivatedEventArgs> for IActivatedEventArgsDeferral {
    type Error = ::windows_core::Error;
    fn try_from(value: &WebUIDevicePairingActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgsDeferral> for WebUIDevicePairingActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgsDeferral> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgsDeferral> for &WebUIDevicePairingActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgsDeferral> {
        ::core::convert::TryInto::<IActivatedEventArgsDeferral>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIDevicePairingActivatedEventArgs> for ::winrt_applicationmodel::Activation::IActivatedEventArgsWithUser {
    type Error = ::windows_core::Error;
    fn try_from(value: WebUIDevicePairingActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIDevicePairingActivatedEventArgs> for ::winrt_applicationmodel::Activation::IActivatedEventArgsWithUser {
    type Error = ::windows_core::Error;
    fn try_from(value: &WebUIDevicePairingActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgsWithUser> for WebUIDevicePairingActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgsWithUser> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgsWithUser> for &WebUIDevicePairingActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgsWithUser> {
        ::core::convert::TryInto::<::winrt_applicationmodel::Activation::IActivatedEventArgsWithUser>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIDevicePairingActivatedEventArgs> for ::winrt_applicationmodel::Activation::IDevicePairingActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: WebUIDevicePairingActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIDevicePairingActivatedEventArgs> for ::winrt_applicationmodel::Activation::IDevicePairingActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &WebUIDevicePairingActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IDevicePairingActivatedEventArgs> for WebUIDevicePairingActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IDevicePairingActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IDevicePairingActivatedEventArgs> for &WebUIDevicePairingActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IDevicePairingActivatedEventArgs> {
        ::core::convert::TryInto::<::winrt_applicationmodel::Activation::IDevicePairingActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
#[repr(transparent)]
pub struct WebUIDialReceiverActivatedEventArgs(::windows_core::IUnknown);
#[cfg(feature = "ApplicationModel_Activation")]
impl WebUIDialReceiverActivatedEventArgs {
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Kind(&self) -> ::windows_core::Result<::winrt_applicationmodel::Activation::ActivationKind> {
        let this = &::windows_core::Interface::cast::<::winrt_applicationmodel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_applicationmodel::Activation::ActivationKind>::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_applicationmodel::Activation::ActivationKind>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<::winrt_applicationmodel::Activation::ApplicationExecutionState> {
        let this = &::windows_core::Interface::cast::<::winrt_applicationmodel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_applicationmodel::Activation::ApplicationExecutionState>::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_applicationmodel::Activation::ApplicationExecutionState>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn SplashScreen(&self) -> ::windows_core::Result<::winrt_applicationmodel::Activation::SplashScreen> {
        let this = &::windows_core::Interface::cast::<::winrt_applicationmodel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_applicationmodel::Activation::SplashScreen>(result__)
        }
    }
    pub fn ActivatedOperation(&self) -> ::windows_core::Result<ActivatedOperation> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgsDeferral>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ActivatedOperation)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivatedOperation>(result__)
        }
    }
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "System"))]
    pub fn User(&self) -> ::windows_core::Result<::winrt_system::User> {
        let this = &::windows_core::Interface::cast::<::winrt_applicationmodel::Activation::IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).User)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_system::User>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn CurrentlyShownApplicationViewId(&self) -> ::windows_core::Result<i32> {
        let this = &::windows_core::Interface::cast::<::winrt_applicationmodel::Activation::IApplicationViewActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows_core::Interface::vtable(this).CurrentlyShownApplicationViewId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn AppName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).AppName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Arguments(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<::winrt_applicationmodel::Activation::ILaunchActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Arguments)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn TileId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<::winrt_applicationmodel::Activation::ILaunchActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).TileId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::clone::Clone for WebUIDialReceiverActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::cmp::PartialEq for WebUIDialReceiverActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::cmp::Eq for WebUIDialReceiverActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::fmt::Debug for WebUIDialReceiverActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WebUIDialReceiverActivatedEventArgs").field(&self.0).finish()
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows_core::RuntimeType for WebUIDialReceiverActivatedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.WebUI.WebUIDialReceiverActivatedEventArgs;{fb777ed7-85ee-456e-a44d-85d730e70aed})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows_core::Interface for WebUIDialReceiverActivatedEventArgs {
    type Vtable = ::winrt_applicationmodel::Activation::IDialReceiverActivatedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <::winrt_applicationmodel::Activation::IDialReceiverActivatedEventArgs as ::windows_core::Interface>::IID;
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::RuntimeName for WebUIDialReceiverActivatedEventArgs {
    const NAME: &'static str = "Windows.UI.WebUI.WebUIDialReceiverActivatedEventArgs";
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<WebUIDialReceiverActivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: WebUIDialReceiverActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<&WebUIDialReceiverActivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &WebUIDialReceiverActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for WebUIDialReceiverActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a WebUIDialReceiverActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<WebUIDialReceiverActivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: WebUIDialReceiverActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<&WebUIDialReceiverActivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &WebUIDialReceiverActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for WebUIDialReceiverActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a WebUIDialReceiverActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIDialReceiverActivatedEventArgs> for ::winrt_applicationmodel::Activation::IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: WebUIDialReceiverActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIDialReceiverActivatedEventArgs> for ::winrt_applicationmodel::Activation::IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &WebUIDialReceiverActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgs> for WebUIDialReceiverActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgs> for &WebUIDialReceiverActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgs> {
        ::core::convert::TryInto::<::winrt_applicationmodel::Activation::IActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIDialReceiverActivatedEventArgs> for IActivatedEventArgsDeferral {
    type Error = ::windows_core::Error;
    fn try_from(value: WebUIDialReceiverActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIDialReceiverActivatedEventArgs> for IActivatedEventArgsDeferral {
    type Error = ::windows_core::Error;
    fn try_from(value: &WebUIDialReceiverActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgsDeferral> for WebUIDialReceiverActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgsDeferral> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgsDeferral> for &WebUIDialReceiverActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgsDeferral> {
        ::core::convert::TryInto::<IActivatedEventArgsDeferral>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIDialReceiverActivatedEventArgs> for ::winrt_applicationmodel::Activation::IActivatedEventArgsWithUser {
    type Error = ::windows_core::Error;
    fn try_from(value: WebUIDialReceiverActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIDialReceiverActivatedEventArgs> for ::winrt_applicationmodel::Activation::IActivatedEventArgsWithUser {
    type Error = ::windows_core::Error;
    fn try_from(value: &WebUIDialReceiverActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgsWithUser> for WebUIDialReceiverActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgsWithUser> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgsWithUser> for &WebUIDialReceiverActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgsWithUser> {
        ::core::convert::TryInto::<::winrt_applicationmodel::Activation::IActivatedEventArgsWithUser>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIDialReceiverActivatedEventArgs> for ::winrt_applicationmodel::Activation::IApplicationViewActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: WebUIDialReceiverActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIDialReceiverActivatedEventArgs> for ::winrt_applicationmodel::Activation::IApplicationViewActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &WebUIDialReceiverActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IApplicationViewActivatedEventArgs> for WebUIDialReceiverActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IApplicationViewActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IApplicationViewActivatedEventArgs> for &WebUIDialReceiverActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IApplicationViewActivatedEventArgs> {
        ::core::convert::TryInto::<::winrt_applicationmodel::Activation::IApplicationViewActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIDialReceiverActivatedEventArgs> for ::winrt_applicationmodel::Activation::IDialReceiverActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: WebUIDialReceiverActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIDialReceiverActivatedEventArgs> for ::winrt_applicationmodel::Activation::IDialReceiverActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &WebUIDialReceiverActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IDialReceiverActivatedEventArgs> for WebUIDialReceiverActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IDialReceiverActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IDialReceiverActivatedEventArgs> for &WebUIDialReceiverActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IDialReceiverActivatedEventArgs> {
        ::core::convert::TryInto::<::winrt_applicationmodel::Activation::IDialReceiverActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIDialReceiverActivatedEventArgs> for ::winrt_applicationmodel::Activation::ILaunchActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: WebUIDialReceiverActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIDialReceiverActivatedEventArgs> for ::winrt_applicationmodel::Activation::ILaunchActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &WebUIDialReceiverActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::ILaunchActivatedEventArgs> for WebUIDialReceiverActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::ILaunchActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::ILaunchActivatedEventArgs> for &WebUIDialReceiverActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::ILaunchActivatedEventArgs> {
        ::core::convert::TryInto::<::winrt_applicationmodel::Activation::ILaunchActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
#[repr(transparent)]
pub struct WebUIFileActivatedEventArgs(::windows_core::IUnknown);
#[cfg(feature = "ApplicationModel_Activation")]
impl WebUIFileActivatedEventArgs {
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Kind(&self) -> ::windows_core::Result<::winrt_applicationmodel::Activation::ActivationKind> {
        let this = &::windows_core::Interface::cast::<::winrt_applicationmodel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_applicationmodel::Activation::ActivationKind>::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_applicationmodel::Activation::ActivationKind>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<::winrt_applicationmodel::Activation::ApplicationExecutionState> {
        let this = &::windows_core::Interface::cast::<::winrt_applicationmodel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_applicationmodel::Activation::ApplicationExecutionState>::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_applicationmodel::Activation::ApplicationExecutionState>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn SplashScreen(&self) -> ::windows_core::Result<::winrt_applicationmodel::Activation::SplashScreen> {
        let this = &::windows_core::Interface::cast::<::winrt_applicationmodel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_applicationmodel::Activation::SplashScreen>(result__)
        }
    }
    pub fn ActivatedOperation(&self) -> ::windows_core::Result<ActivatedOperation> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgsDeferral>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ActivatedOperation)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivatedOperation>(result__)
        }
    }
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "System"))]
    pub fn User(&self) -> ::windows_core::Result<::winrt_system::User> {
        let this = &::windows_core::Interface::cast::<::winrt_applicationmodel::Activation::IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).User)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_system::User>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn CurrentlyShownApplicationViewId(&self) -> ::windows_core::Result<i32> {
        let this = &::windows_core::Interface::cast::<::winrt_applicationmodel::Activation::IApplicationViewActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows_core::Interface::vtable(this).CurrentlyShownApplicationViewId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "Foundation_Collections", feature = "Storage"))]
    pub fn Files(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<::winrt_storage::IStorageItem>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Files)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<::winrt_storage::IStorageItem>>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Verb(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Verb)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "Storage_Search"))]
    pub fn NeighboringFilesQuery(&self) -> ::windows_core::Result<::winrt_storage::Search::StorageFileQueryResult> {
        let this = &::windows_core::Interface::cast::<::winrt_applicationmodel::Activation::IFileActivatedEventArgsWithNeighboringFiles>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).NeighboringFilesQuery)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_storage::Search::StorageFileQueryResult>(result__)
        }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::clone::Clone for WebUIFileActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::cmp::PartialEq for WebUIFileActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::cmp::Eq for WebUIFileActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::fmt::Debug for WebUIFileActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WebUIFileActivatedEventArgs").field(&self.0).finish()
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows_core::RuntimeType for WebUIFileActivatedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.WebUI.WebUIFileActivatedEventArgs;{bb2afc33-93b1-42ed-8b26-236dd9c78496})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows_core::Interface for WebUIFileActivatedEventArgs {
    type Vtable = ::winrt_applicationmodel::Activation::IFileActivatedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <::winrt_applicationmodel::Activation::IFileActivatedEventArgs as ::windows_core::Interface>::IID;
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::RuntimeName for WebUIFileActivatedEventArgs {
    const NAME: &'static str = "Windows.UI.WebUI.WebUIFileActivatedEventArgs";
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<WebUIFileActivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: WebUIFileActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<&WebUIFileActivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &WebUIFileActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for WebUIFileActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a WebUIFileActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<WebUIFileActivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: WebUIFileActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<&WebUIFileActivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &WebUIFileActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for WebUIFileActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a WebUIFileActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIFileActivatedEventArgs> for ::winrt_applicationmodel::Activation::IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: WebUIFileActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIFileActivatedEventArgs> for ::winrt_applicationmodel::Activation::IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &WebUIFileActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgs> for WebUIFileActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgs> for &WebUIFileActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgs> {
        ::core::convert::TryInto::<::winrt_applicationmodel::Activation::IActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIFileActivatedEventArgs> for IActivatedEventArgsDeferral {
    type Error = ::windows_core::Error;
    fn try_from(value: WebUIFileActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIFileActivatedEventArgs> for IActivatedEventArgsDeferral {
    type Error = ::windows_core::Error;
    fn try_from(value: &WebUIFileActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgsDeferral> for WebUIFileActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgsDeferral> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgsDeferral> for &WebUIFileActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgsDeferral> {
        ::core::convert::TryInto::<IActivatedEventArgsDeferral>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIFileActivatedEventArgs> for ::winrt_applicationmodel::Activation::IActivatedEventArgsWithUser {
    type Error = ::windows_core::Error;
    fn try_from(value: WebUIFileActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIFileActivatedEventArgs> for ::winrt_applicationmodel::Activation::IActivatedEventArgsWithUser {
    type Error = ::windows_core::Error;
    fn try_from(value: &WebUIFileActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgsWithUser> for WebUIFileActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgsWithUser> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgsWithUser> for &WebUIFileActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgsWithUser> {
        ::core::convert::TryInto::<::winrt_applicationmodel::Activation::IActivatedEventArgsWithUser>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIFileActivatedEventArgs> for ::winrt_applicationmodel::Activation::IApplicationViewActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: WebUIFileActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIFileActivatedEventArgs> for ::winrt_applicationmodel::Activation::IApplicationViewActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &WebUIFileActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IApplicationViewActivatedEventArgs> for WebUIFileActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IApplicationViewActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IApplicationViewActivatedEventArgs> for &WebUIFileActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IApplicationViewActivatedEventArgs> {
        ::core::convert::TryInto::<::winrt_applicationmodel::Activation::IApplicationViewActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIFileActivatedEventArgs> for ::winrt_applicationmodel::Activation::IFileActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: WebUIFileActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIFileActivatedEventArgs> for ::winrt_applicationmodel::Activation::IFileActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &WebUIFileActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IFileActivatedEventArgs> for WebUIFileActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IFileActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IFileActivatedEventArgs> for &WebUIFileActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IFileActivatedEventArgs> {
        ::core::convert::TryInto::<::winrt_applicationmodel::Activation::IFileActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIFileActivatedEventArgs> for ::winrt_applicationmodel::Activation::IFileActivatedEventArgsWithNeighboringFiles {
    type Error = ::windows_core::Error;
    fn try_from(value: WebUIFileActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIFileActivatedEventArgs> for ::winrt_applicationmodel::Activation::IFileActivatedEventArgsWithNeighboringFiles {
    type Error = ::windows_core::Error;
    fn try_from(value: &WebUIFileActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IFileActivatedEventArgsWithNeighboringFiles> for WebUIFileActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IFileActivatedEventArgsWithNeighboringFiles> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IFileActivatedEventArgsWithNeighboringFiles> for &WebUIFileActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IFileActivatedEventArgsWithNeighboringFiles> {
        ::core::convert::TryInto::<::winrt_applicationmodel::Activation::IFileActivatedEventArgsWithNeighboringFiles>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
#[repr(transparent)]
pub struct WebUIFileOpenPickerActivatedEventArgs(::windows_core::IUnknown);
#[cfg(feature = "ApplicationModel_Activation")]
impl WebUIFileOpenPickerActivatedEventArgs {
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Kind(&self) -> ::windows_core::Result<::winrt_applicationmodel::Activation::ActivationKind> {
        let this = &::windows_core::Interface::cast::<::winrt_applicationmodel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_applicationmodel::Activation::ActivationKind>::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_applicationmodel::Activation::ActivationKind>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<::winrt_applicationmodel::Activation::ApplicationExecutionState> {
        let this = &::windows_core::Interface::cast::<::winrt_applicationmodel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_applicationmodel::Activation::ApplicationExecutionState>::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_applicationmodel::Activation::ApplicationExecutionState>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn SplashScreen(&self) -> ::windows_core::Result<::winrt_applicationmodel::Activation::SplashScreen> {
        let this = &::windows_core::Interface::cast::<::winrt_applicationmodel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_applicationmodel::Activation::SplashScreen>(result__)
        }
    }
    pub fn ActivatedOperation(&self) -> ::windows_core::Result<ActivatedOperation> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgsDeferral>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ActivatedOperation)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivatedOperation>(result__)
        }
    }
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "System"))]
    pub fn User(&self) -> ::windows_core::Result<::winrt_system::User> {
        let this = &::windows_core::Interface::cast::<::winrt_applicationmodel::Activation::IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).User)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_system::User>(result__)
        }
    }
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "Storage_Pickers_Provider"))]
    pub fn FileOpenPickerUI(&self) -> ::windows_core::Result<::winrt_storage::Pickers::Provider::FileOpenPickerUI> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FileOpenPickerUI)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_storage::Pickers::Provider::FileOpenPickerUI>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn CallerPackageFamilyName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<::winrt_applicationmodel::Activation::IFileOpenPickerActivatedEventArgs2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).CallerPackageFamilyName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::clone::Clone for WebUIFileOpenPickerActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::cmp::PartialEq for WebUIFileOpenPickerActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::cmp::Eq for WebUIFileOpenPickerActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::fmt::Debug for WebUIFileOpenPickerActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WebUIFileOpenPickerActivatedEventArgs").field(&self.0).finish()
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows_core::RuntimeType for WebUIFileOpenPickerActivatedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.WebUI.WebUIFileOpenPickerActivatedEventArgs;{72827082-5525-4bf2-bc09-1f5095d4964d})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows_core::Interface for WebUIFileOpenPickerActivatedEventArgs {
    type Vtable = ::winrt_applicationmodel::Activation::IFileOpenPickerActivatedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <::winrt_applicationmodel::Activation::IFileOpenPickerActivatedEventArgs as ::windows_core::Interface>::IID;
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::RuntimeName for WebUIFileOpenPickerActivatedEventArgs {
    const NAME: &'static str = "Windows.UI.WebUI.WebUIFileOpenPickerActivatedEventArgs";
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<WebUIFileOpenPickerActivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: WebUIFileOpenPickerActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<&WebUIFileOpenPickerActivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &WebUIFileOpenPickerActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for WebUIFileOpenPickerActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a WebUIFileOpenPickerActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<WebUIFileOpenPickerActivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: WebUIFileOpenPickerActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<&WebUIFileOpenPickerActivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &WebUIFileOpenPickerActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for WebUIFileOpenPickerActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a WebUIFileOpenPickerActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIFileOpenPickerActivatedEventArgs> for ::winrt_applicationmodel::Activation::IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: WebUIFileOpenPickerActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIFileOpenPickerActivatedEventArgs> for ::winrt_applicationmodel::Activation::IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &WebUIFileOpenPickerActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgs> for WebUIFileOpenPickerActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgs> for &WebUIFileOpenPickerActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgs> {
        ::core::convert::TryInto::<::winrt_applicationmodel::Activation::IActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIFileOpenPickerActivatedEventArgs> for IActivatedEventArgsDeferral {
    type Error = ::windows_core::Error;
    fn try_from(value: WebUIFileOpenPickerActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIFileOpenPickerActivatedEventArgs> for IActivatedEventArgsDeferral {
    type Error = ::windows_core::Error;
    fn try_from(value: &WebUIFileOpenPickerActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgsDeferral> for WebUIFileOpenPickerActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgsDeferral> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgsDeferral> for &WebUIFileOpenPickerActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgsDeferral> {
        ::core::convert::TryInto::<IActivatedEventArgsDeferral>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIFileOpenPickerActivatedEventArgs> for ::winrt_applicationmodel::Activation::IActivatedEventArgsWithUser {
    type Error = ::windows_core::Error;
    fn try_from(value: WebUIFileOpenPickerActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIFileOpenPickerActivatedEventArgs> for ::winrt_applicationmodel::Activation::IActivatedEventArgsWithUser {
    type Error = ::windows_core::Error;
    fn try_from(value: &WebUIFileOpenPickerActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgsWithUser> for WebUIFileOpenPickerActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgsWithUser> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgsWithUser> for &WebUIFileOpenPickerActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgsWithUser> {
        ::core::convert::TryInto::<::winrt_applicationmodel::Activation::IActivatedEventArgsWithUser>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIFileOpenPickerActivatedEventArgs> for ::winrt_applicationmodel::Activation::IFileOpenPickerActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: WebUIFileOpenPickerActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIFileOpenPickerActivatedEventArgs> for ::winrt_applicationmodel::Activation::IFileOpenPickerActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &WebUIFileOpenPickerActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IFileOpenPickerActivatedEventArgs> for WebUIFileOpenPickerActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IFileOpenPickerActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IFileOpenPickerActivatedEventArgs> for &WebUIFileOpenPickerActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IFileOpenPickerActivatedEventArgs> {
        ::core::convert::TryInto::<::winrt_applicationmodel::Activation::IFileOpenPickerActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIFileOpenPickerActivatedEventArgs> for ::winrt_applicationmodel::Activation::IFileOpenPickerActivatedEventArgs2 {
    type Error = ::windows_core::Error;
    fn try_from(value: WebUIFileOpenPickerActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIFileOpenPickerActivatedEventArgs> for ::winrt_applicationmodel::Activation::IFileOpenPickerActivatedEventArgs2 {
    type Error = ::windows_core::Error;
    fn try_from(value: &WebUIFileOpenPickerActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IFileOpenPickerActivatedEventArgs2> for WebUIFileOpenPickerActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IFileOpenPickerActivatedEventArgs2> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IFileOpenPickerActivatedEventArgs2> for &WebUIFileOpenPickerActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IFileOpenPickerActivatedEventArgs2> {
        ::core::convert::TryInto::<::winrt_applicationmodel::Activation::IFileOpenPickerActivatedEventArgs2>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "deprecated"))]
#[repr(transparent)]
pub struct WebUIFileOpenPickerContinuationEventArgs(::windows_core::IUnknown);
#[cfg(all(feature = "ApplicationModel_Activation", feature = "deprecated"))]
impl WebUIFileOpenPickerContinuationEventArgs {
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Kind(&self) -> ::windows_core::Result<::winrt_applicationmodel::Activation::ActivationKind> {
        let this = &::windows_core::Interface::cast::<::winrt_applicationmodel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_applicationmodel::Activation::ActivationKind>::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_applicationmodel::Activation::ActivationKind>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<::winrt_applicationmodel::Activation::ApplicationExecutionState> {
        let this = &::windows_core::Interface::cast::<::winrt_applicationmodel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_applicationmodel::Activation::ApplicationExecutionState>::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_applicationmodel::Activation::ApplicationExecutionState>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn SplashScreen(&self) -> ::windows_core::Result<::winrt_applicationmodel::Activation::SplashScreen> {
        let this = &::windows_core::Interface::cast::<::winrt_applicationmodel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_applicationmodel::Activation::SplashScreen>(result__)
        }
    }
    pub fn ActivatedOperation(&self) -> ::windows_core::Result<ActivatedOperation> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgsDeferral>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ActivatedOperation)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivatedOperation>(result__)
        }
    }
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "System"))]
    pub fn User(&self) -> ::windows_core::Result<::winrt_system::User> {
        let this = &::windows_core::Interface::cast::<::winrt_applicationmodel::Activation::IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).User)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_system::User>(result__)
        }
    }
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "Foundation_Collections"))]
    pub fn ContinuationData(&self) -> ::windows_core::Result<::winrt_foundation::Collections::ValueSet> {
        let this = &::windows_core::Interface::cast::<::winrt_applicationmodel::Activation::IContinuationActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ContinuationData)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::ValueSet>(result__)
        }
    }
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "Foundation_Collections", feature = "Storage", feature = "deprecated"))]
    pub fn Files(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<::winrt_storage::StorageFile>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Files)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<::winrt_storage::StorageFile>>(result__)
        }
    }
}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "deprecated"))]
impl ::core::clone::Clone for WebUIFileOpenPickerContinuationEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "deprecated"))]
impl ::core::cmp::PartialEq for WebUIFileOpenPickerContinuationEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "deprecated"))]
impl ::core::cmp::Eq for WebUIFileOpenPickerContinuationEventArgs {}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "deprecated"))]
impl ::core::fmt::Debug for WebUIFileOpenPickerContinuationEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WebUIFileOpenPickerContinuationEventArgs").field(&self.0).finish()
    }
}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "deprecated"))]
unsafe impl ::windows_core::RuntimeType for WebUIFileOpenPickerContinuationEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.WebUI.WebUIFileOpenPickerContinuationEventArgs;{f0fa3f3a-d4e8-4ad3-9c34-2308f32fcec9})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "deprecated"))]
unsafe impl ::windows_core::Interface for WebUIFileOpenPickerContinuationEventArgs {
    type Vtable = ::winrt_applicationmodel::Activation::IFileOpenPickerContinuationEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <::winrt_applicationmodel::Activation::IFileOpenPickerContinuationEventArgs as ::windows_core::Interface>::IID;
}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "deprecated"))]
impl ::windows_core::RuntimeName for WebUIFileOpenPickerContinuationEventArgs {
    const NAME: &'static str = "Windows.UI.WebUI.WebUIFileOpenPickerContinuationEventArgs";
}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "deprecated"))]
impl ::core::convert::From<WebUIFileOpenPickerContinuationEventArgs> for ::windows_core::IUnknown {
    fn from(value: WebUIFileOpenPickerContinuationEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "deprecated"))]
impl ::core::convert::From<&WebUIFileOpenPickerContinuationEventArgs> for ::windows_core::IUnknown {
    fn from(value: &WebUIFileOpenPickerContinuationEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "deprecated"))]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for WebUIFileOpenPickerContinuationEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "deprecated"))]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a WebUIFileOpenPickerContinuationEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "deprecated"))]
impl ::core::convert::From<WebUIFileOpenPickerContinuationEventArgs> for ::windows_core::IInspectable {
    fn from(value: WebUIFileOpenPickerContinuationEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "deprecated"))]
impl ::core::convert::From<&WebUIFileOpenPickerContinuationEventArgs> for ::windows_core::IInspectable {
    fn from(value: &WebUIFileOpenPickerContinuationEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "deprecated"))]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for WebUIFileOpenPickerContinuationEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "deprecated"))]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a WebUIFileOpenPickerContinuationEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "deprecated"))]
impl ::core::convert::TryFrom<WebUIFileOpenPickerContinuationEventArgs> for ::winrt_applicationmodel::Activation::IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: WebUIFileOpenPickerContinuationEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "deprecated"))]
impl ::core::convert::TryFrom<&WebUIFileOpenPickerContinuationEventArgs> for ::winrt_applicationmodel::Activation::IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &WebUIFileOpenPickerContinuationEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "deprecated"))]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgs> for WebUIFileOpenPickerContinuationEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "deprecated"))]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgs> for &WebUIFileOpenPickerContinuationEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgs> {
        ::core::convert::TryInto::<::winrt_applicationmodel::Activation::IActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "deprecated"))]
impl ::core::convert::TryFrom<WebUIFileOpenPickerContinuationEventArgs> for IActivatedEventArgsDeferral {
    type Error = ::windows_core::Error;
    fn try_from(value: WebUIFileOpenPickerContinuationEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "deprecated"))]
impl ::core::convert::TryFrom<&WebUIFileOpenPickerContinuationEventArgs> for IActivatedEventArgsDeferral {
    type Error = ::windows_core::Error;
    fn try_from(value: &WebUIFileOpenPickerContinuationEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "deprecated"))]
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgsDeferral> for WebUIFileOpenPickerContinuationEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgsDeferral> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "deprecated"))]
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgsDeferral> for &WebUIFileOpenPickerContinuationEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgsDeferral> {
        ::core::convert::TryInto::<IActivatedEventArgsDeferral>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "deprecated"))]
impl ::core::convert::TryFrom<WebUIFileOpenPickerContinuationEventArgs> for ::winrt_applicationmodel::Activation::IActivatedEventArgsWithUser {
    type Error = ::windows_core::Error;
    fn try_from(value: WebUIFileOpenPickerContinuationEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "deprecated"))]
impl ::core::convert::TryFrom<&WebUIFileOpenPickerContinuationEventArgs> for ::winrt_applicationmodel::Activation::IActivatedEventArgsWithUser {
    type Error = ::windows_core::Error;
    fn try_from(value: &WebUIFileOpenPickerContinuationEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "deprecated"))]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgsWithUser> for WebUIFileOpenPickerContinuationEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgsWithUser> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "deprecated"))]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgsWithUser> for &WebUIFileOpenPickerContinuationEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgsWithUser> {
        ::core::convert::TryInto::<::winrt_applicationmodel::Activation::IActivatedEventArgsWithUser>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "deprecated"))]
impl ::core::convert::TryFrom<WebUIFileOpenPickerContinuationEventArgs> for ::winrt_applicationmodel::Activation::IContinuationActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: WebUIFileOpenPickerContinuationEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "deprecated"))]
impl ::core::convert::TryFrom<&WebUIFileOpenPickerContinuationEventArgs> for ::winrt_applicationmodel::Activation::IContinuationActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &WebUIFileOpenPickerContinuationEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "deprecated"))]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IContinuationActivatedEventArgs> for WebUIFileOpenPickerContinuationEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IContinuationActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "deprecated"))]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IContinuationActivatedEventArgs> for &WebUIFileOpenPickerContinuationEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IContinuationActivatedEventArgs> {
        ::core::convert::TryInto::<::winrt_applicationmodel::Activation::IContinuationActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "deprecated"))]
impl ::core::convert::TryFrom<WebUIFileOpenPickerContinuationEventArgs> for ::winrt_applicationmodel::Activation::IFileOpenPickerContinuationEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: WebUIFileOpenPickerContinuationEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "deprecated"))]
impl ::core::convert::TryFrom<&WebUIFileOpenPickerContinuationEventArgs> for ::winrt_applicationmodel::Activation::IFileOpenPickerContinuationEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &WebUIFileOpenPickerContinuationEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "deprecated"))]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IFileOpenPickerContinuationEventArgs> for WebUIFileOpenPickerContinuationEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IFileOpenPickerContinuationEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "deprecated"))]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IFileOpenPickerContinuationEventArgs> for &WebUIFileOpenPickerContinuationEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IFileOpenPickerContinuationEventArgs> {
        ::core::convert::TryInto::<::winrt_applicationmodel::Activation::IFileOpenPickerContinuationEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
#[repr(transparent)]
pub struct WebUIFileSavePickerActivatedEventArgs(::windows_core::IUnknown);
#[cfg(feature = "ApplicationModel_Activation")]
impl WebUIFileSavePickerActivatedEventArgs {
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Kind(&self) -> ::windows_core::Result<::winrt_applicationmodel::Activation::ActivationKind> {
        let this = &::windows_core::Interface::cast::<::winrt_applicationmodel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_applicationmodel::Activation::ActivationKind>::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_applicationmodel::Activation::ActivationKind>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<::winrt_applicationmodel::Activation::ApplicationExecutionState> {
        let this = &::windows_core::Interface::cast::<::winrt_applicationmodel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_applicationmodel::Activation::ApplicationExecutionState>::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_applicationmodel::Activation::ApplicationExecutionState>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn SplashScreen(&self) -> ::windows_core::Result<::winrt_applicationmodel::Activation::SplashScreen> {
        let this = &::windows_core::Interface::cast::<::winrt_applicationmodel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_applicationmodel::Activation::SplashScreen>(result__)
        }
    }
    pub fn ActivatedOperation(&self) -> ::windows_core::Result<ActivatedOperation> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgsDeferral>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ActivatedOperation)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivatedOperation>(result__)
        }
    }
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "System"))]
    pub fn User(&self) -> ::windows_core::Result<::winrt_system::User> {
        let this = &::windows_core::Interface::cast::<::winrt_applicationmodel::Activation::IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).User)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_system::User>(result__)
        }
    }
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "Storage_Pickers_Provider"))]
    pub fn FileSavePickerUI(&self) -> ::windows_core::Result<::winrt_storage::Pickers::Provider::FileSavePickerUI> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FileSavePickerUI)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_storage::Pickers::Provider::FileSavePickerUI>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn CallerPackageFamilyName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<::winrt_applicationmodel::Activation::IFileSavePickerActivatedEventArgs2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).CallerPackageFamilyName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn EnterpriseId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<::winrt_applicationmodel::Activation::IFileSavePickerActivatedEventArgs2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).EnterpriseId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::clone::Clone for WebUIFileSavePickerActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::cmp::PartialEq for WebUIFileSavePickerActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::cmp::Eq for WebUIFileSavePickerActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::fmt::Debug for WebUIFileSavePickerActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WebUIFileSavePickerActivatedEventArgs").field(&self.0).finish()
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows_core::RuntimeType for WebUIFileSavePickerActivatedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.WebUI.WebUIFileSavePickerActivatedEventArgs;{81c19cf1-74e6-4387-82eb-bb8fd64b4346})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows_core::Interface for WebUIFileSavePickerActivatedEventArgs {
    type Vtable = ::winrt_applicationmodel::Activation::IFileSavePickerActivatedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <::winrt_applicationmodel::Activation::IFileSavePickerActivatedEventArgs as ::windows_core::Interface>::IID;
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::RuntimeName for WebUIFileSavePickerActivatedEventArgs {
    const NAME: &'static str = "Windows.UI.WebUI.WebUIFileSavePickerActivatedEventArgs";
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<WebUIFileSavePickerActivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: WebUIFileSavePickerActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<&WebUIFileSavePickerActivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &WebUIFileSavePickerActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for WebUIFileSavePickerActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a WebUIFileSavePickerActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<WebUIFileSavePickerActivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: WebUIFileSavePickerActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<&WebUIFileSavePickerActivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &WebUIFileSavePickerActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for WebUIFileSavePickerActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a WebUIFileSavePickerActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIFileSavePickerActivatedEventArgs> for ::winrt_applicationmodel::Activation::IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: WebUIFileSavePickerActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIFileSavePickerActivatedEventArgs> for ::winrt_applicationmodel::Activation::IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &WebUIFileSavePickerActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgs> for WebUIFileSavePickerActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgs> for &WebUIFileSavePickerActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgs> {
        ::core::convert::TryInto::<::winrt_applicationmodel::Activation::IActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIFileSavePickerActivatedEventArgs> for IActivatedEventArgsDeferral {
    type Error = ::windows_core::Error;
    fn try_from(value: WebUIFileSavePickerActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIFileSavePickerActivatedEventArgs> for IActivatedEventArgsDeferral {
    type Error = ::windows_core::Error;
    fn try_from(value: &WebUIFileSavePickerActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgsDeferral> for WebUIFileSavePickerActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgsDeferral> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgsDeferral> for &WebUIFileSavePickerActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgsDeferral> {
        ::core::convert::TryInto::<IActivatedEventArgsDeferral>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIFileSavePickerActivatedEventArgs> for ::winrt_applicationmodel::Activation::IActivatedEventArgsWithUser {
    type Error = ::windows_core::Error;
    fn try_from(value: WebUIFileSavePickerActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIFileSavePickerActivatedEventArgs> for ::winrt_applicationmodel::Activation::IActivatedEventArgsWithUser {
    type Error = ::windows_core::Error;
    fn try_from(value: &WebUIFileSavePickerActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgsWithUser> for WebUIFileSavePickerActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgsWithUser> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgsWithUser> for &WebUIFileSavePickerActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgsWithUser> {
        ::core::convert::TryInto::<::winrt_applicationmodel::Activation::IActivatedEventArgsWithUser>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIFileSavePickerActivatedEventArgs> for ::winrt_applicationmodel::Activation::IFileSavePickerActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: WebUIFileSavePickerActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIFileSavePickerActivatedEventArgs> for ::winrt_applicationmodel::Activation::IFileSavePickerActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &WebUIFileSavePickerActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IFileSavePickerActivatedEventArgs> for WebUIFileSavePickerActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IFileSavePickerActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IFileSavePickerActivatedEventArgs> for &WebUIFileSavePickerActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IFileSavePickerActivatedEventArgs> {
        ::core::convert::TryInto::<::winrt_applicationmodel::Activation::IFileSavePickerActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIFileSavePickerActivatedEventArgs> for ::winrt_applicationmodel::Activation::IFileSavePickerActivatedEventArgs2 {
    type Error = ::windows_core::Error;
    fn try_from(value: WebUIFileSavePickerActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIFileSavePickerActivatedEventArgs> for ::winrt_applicationmodel::Activation::IFileSavePickerActivatedEventArgs2 {
    type Error = ::windows_core::Error;
    fn try_from(value: &WebUIFileSavePickerActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IFileSavePickerActivatedEventArgs2> for WebUIFileSavePickerActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IFileSavePickerActivatedEventArgs2> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IFileSavePickerActivatedEventArgs2> for &WebUIFileSavePickerActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IFileSavePickerActivatedEventArgs2> {
        ::core::convert::TryInto::<::winrt_applicationmodel::Activation::IFileSavePickerActivatedEventArgs2>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "deprecated"))]
#[repr(transparent)]
pub struct WebUIFileSavePickerContinuationEventArgs(::windows_core::IUnknown);
#[cfg(all(feature = "ApplicationModel_Activation", feature = "deprecated"))]
impl WebUIFileSavePickerContinuationEventArgs {
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Kind(&self) -> ::windows_core::Result<::winrt_applicationmodel::Activation::ActivationKind> {
        let this = &::windows_core::Interface::cast::<::winrt_applicationmodel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_applicationmodel::Activation::ActivationKind>::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_applicationmodel::Activation::ActivationKind>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<::winrt_applicationmodel::Activation::ApplicationExecutionState> {
        let this = &::windows_core::Interface::cast::<::winrt_applicationmodel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_applicationmodel::Activation::ApplicationExecutionState>::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_applicationmodel::Activation::ApplicationExecutionState>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn SplashScreen(&self) -> ::windows_core::Result<::winrt_applicationmodel::Activation::SplashScreen> {
        let this = &::windows_core::Interface::cast::<::winrt_applicationmodel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_applicationmodel::Activation::SplashScreen>(result__)
        }
    }
    pub fn ActivatedOperation(&self) -> ::windows_core::Result<ActivatedOperation> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgsDeferral>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ActivatedOperation)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivatedOperation>(result__)
        }
    }
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "System"))]
    pub fn User(&self) -> ::windows_core::Result<::winrt_system::User> {
        let this = &::windows_core::Interface::cast::<::winrt_applicationmodel::Activation::IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).User)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_system::User>(result__)
        }
    }
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "Foundation_Collections"))]
    pub fn ContinuationData(&self) -> ::windows_core::Result<::winrt_foundation::Collections::ValueSet> {
        let this = &::windows_core::Interface::cast::<::winrt_applicationmodel::Activation::IContinuationActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ContinuationData)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::ValueSet>(result__)
        }
    }
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "Storage", feature = "deprecated"))]
    pub fn File(&self) -> ::windows_core::Result<::winrt_storage::StorageFile> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).File)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_storage::StorageFile>(result__)
        }
    }
}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "deprecated"))]
impl ::core::clone::Clone for WebUIFileSavePickerContinuationEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "deprecated"))]
impl ::core::cmp::PartialEq for WebUIFileSavePickerContinuationEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "deprecated"))]
impl ::core::cmp::Eq for WebUIFileSavePickerContinuationEventArgs {}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "deprecated"))]
impl ::core::fmt::Debug for WebUIFileSavePickerContinuationEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WebUIFileSavePickerContinuationEventArgs").field(&self.0).finish()
    }
}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "deprecated"))]
unsafe impl ::windows_core::RuntimeType for WebUIFileSavePickerContinuationEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.WebUI.WebUIFileSavePickerContinuationEventArgs;{2c846fe1-3bad-4f33-8c8b-e46fae824b4b})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "deprecated"))]
unsafe impl ::windows_core::Interface for WebUIFileSavePickerContinuationEventArgs {
    type Vtable = ::winrt_applicationmodel::Activation::IFileSavePickerContinuationEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <::winrt_applicationmodel::Activation::IFileSavePickerContinuationEventArgs as ::windows_core::Interface>::IID;
}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "deprecated"))]
impl ::windows_core::RuntimeName for WebUIFileSavePickerContinuationEventArgs {
    const NAME: &'static str = "Windows.UI.WebUI.WebUIFileSavePickerContinuationEventArgs";
}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "deprecated"))]
impl ::core::convert::From<WebUIFileSavePickerContinuationEventArgs> for ::windows_core::IUnknown {
    fn from(value: WebUIFileSavePickerContinuationEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "deprecated"))]
impl ::core::convert::From<&WebUIFileSavePickerContinuationEventArgs> for ::windows_core::IUnknown {
    fn from(value: &WebUIFileSavePickerContinuationEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "deprecated"))]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for WebUIFileSavePickerContinuationEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "deprecated"))]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a WebUIFileSavePickerContinuationEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "deprecated"))]
impl ::core::convert::From<WebUIFileSavePickerContinuationEventArgs> for ::windows_core::IInspectable {
    fn from(value: WebUIFileSavePickerContinuationEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "deprecated"))]
impl ::core::convert::From<&WebUIFileSavePickerContinuationEventArgs> for ::windows_core::IInspectable {
    fn from(value: &WebUIFileSavePickerContinuationEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "deprecated"))]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for WebUIFileSavePickerContinuationEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "deprecated"))]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a WebUIFileSavePickerContinuationEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "deprecated"))]
impl ::core::convert::TryFrom<WebUIFileSavePickerContinuationEventArgs> for ::winrt_applicationmodel::Activation::IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: WebUIFileSavePickerContinuationEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "deprecated"))]
impl ::core::convert::TryFrom<&WebUIFileSavePickerContinuationEventArgs> for ::winrt_applicationmodel::Activation::IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &WebUIFileSavePickerContinuationEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "deprecated"))]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgs> for WebUIFileSavePickerContinuationEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "deprecated"))]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgs> for &WebUIFileSavePickerContinuationEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgs> {
        ::core::convert::TryInto::<::winrt_applicationmodel::Activation::IActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "deprecated"))]
impl ::core::convert::TryFrom<WebUIFileSavePickerContinuationEventArgs> for IActivatedEventArgsDeferral {
    type Error = ::windows_core::Error;
    fn try_from(value: WebUIFileSavePickerContinuationEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "deprecated"))]
impl ::core::convert::TryFrom<&WebUIFileSavePickerContinuationEventArgs> for IActivatedEventArgsDeferral {
    type Error = ::windows_core::Error;
    fn try_from(value: &WebUIFileSavePickerContinuationEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "deprecated"))]
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgsDeferral> for WebUIFileSavePickerContinuationEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgsDeferral> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "deprecated"))]
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgsDeferral> for &WebUIFileSavePickerContinuationEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgsDeferral> {
        ::core::convert::TryInto::<IActivatedEventArgsDeferral>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "deprecated"))]
impl ::core::convert::TryFrom<WebUIFileSavePickerContinuationEventArgs> for ::winrt_applicationmodel::Activation::IActivatedEventArgsWithUser {
    type Error = ::windows_core::Error;
    fn try_from(value: WebUIFileSavePickerContinuationEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "deprecated"))]
impl ::core::convert::TryFrom<&WebUIFileSavePickerContinuationEventArgs> for ::winrt_applicationmodel::Activation::IActivatedEventArgsWithUser {
    type Error = ::windows_core::Error;
    fn try_from(value: &WebUIFileSavePickerContinuationEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "deprecated"))]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgsWithUser> for WebUIFileSavePickerContinuationEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgsWithUser> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "deprecated"))]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgsWithUser> for &WebUIFileSavePickerContinuationEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgsWithUser> {
        ::core::convert::TryInto::<::winrt_applicationmodel::Activation::IActivatedEventArgsWithUser>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "deprecated"))]
impl ::core::convert::TryFrom<WebUIFileSavePickerContinuationEventArgs> for ::winrt_applicationmodel::Activation::IContinuationActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: WebUIFileSavePickerContinuationEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "deprecated"))]
impl ::core::convert::TryFrom<&WebUIFileSavePickerContinuationEventArgs> for ::winrt_applicationmodel::Activation::IContinuationActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &WebUIFileSavePickerContinuationEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "deprecated"))]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IContinuationActivatedEventArgs> for WebUIFileSavePickerContinuationEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IContinuationActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "deprecated"))]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IContinuationActivatedEventArgs> for &WebUIFileSavePickerContinuationEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IContinuationActivatedEventArgs> {
        ::core::convert::TryInto::<::winrt_applicationmodel::Activation::IContinuationActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "deprecated"))]
impl ::core::convert::TryFrom<WebUIFileSavePickerContinuationEventArgs> for ::winrt_applicationmodel::Activation::IFileSavePickerContinuationEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: WebUIFileSavePickerContinuationEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "deprecated"))]
impl ::core::convert::TryFrom<&WebUIFileSavePickerContinuationEventArgs> for ::winrt_applicationmodel::Activation::IFileSavePickerContinuationEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &WebUIFileSavePickerContinuationEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "deprecated"))]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IFileSavePickerContinuationEventArgs> for WebUIFileSavePickerContinuationEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IFileSavePickerContinuationEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "deprecated"))]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IFileSavePickerContinuationEventArgs> for &WebUIFileSavePickerContinuationEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IFileSavePickerContinuationEventArgs> {
        ::core::convert::TryInto::<::winrt_applicationmodel::Activation::IFileSavePickerContinuationEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "deprecated"))]
#[repr(transparent)]
pub struct WebUIFolderPickerContinuationEventArgs(::windows_core::IUnknown);
#[cfg(all(feature = "ApplicationModel_Activation", feature = "deprecated"))]
impl WebUIFolderPickerContinuationEventArgs {
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Kind(&self) -> ::windows_core::Result<::winrt_applicationmodel::Activation::ActivationKind> {
        let this = &::windows_core::Interface::cast::<::winrt_applicationmodel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_applicationmodel::Activation::ActivationKind>::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_applicationmodel::Activation::ActivationKind>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<::winrt_applicationmodel::Activation::ApplicationExecutionState> {
        let this = &::windows_core::Interface::cast::<::winrt_applicationmodel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_applicationmodel::Activation::ApplicationExecutionState>::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_applicationmodel::Activation::ApplicationExecutionState>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn SplashScreen(&self) -> ::windows_core::Result<::winrt_applicationmodel::Activation::SplashScreen> {
        let this = &::windows_core::Interface::cast::<::winrt_applicationmodel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_applicationmodel::Activation::SplashScreen>(result__)
        }
    }
    pub fn ActivatedOperation(&self) -> ::windows_core::Result<ActivatedOperation> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgsDeferral>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ActivatedOperation)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivatedOperation>(result__)
        }
    }
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "System"))]
    pub fn User(&self) -> ::windows_core::Result<::winrt_system::User> {
        let this = &::windows_core::Interface::cast::<::winrt_applicationmodel::Activation::IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).User)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_system::User>(result__)
        }
    }
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "Foundation_Collections"))]
    pub fn ContinuationData(&self) -> ::windows_core::Result<::winrt_foundation::Collections::ValueSet> {
        let this = &::windows_core::Interface::cast::<::winrt_applicationmodel::Activation::IContinuationActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ContinuationData)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::ValueSet>(result__)
        }
    }
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "Storage", feature = "deprecated"))]
    pub fn Folder(&self) -> ::windows_core::Result<::winrt_storage::StorageFolder> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Folder)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_storage::StorageFolder>(result__)
        }
    }
}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "deprecated"))]
impl ::core::clone::Clone for WebUIFolderPickerContinuationEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "deprecated"))]
impl ::core::cmp::PartialEq for WebUIFolderPickerContinuationEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "deprecated"))]
impl ::core::cmp::Eq for WebUIFolderPickerContinuationEventArgs {}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "deprecated"))]
impl ::core::fmt::Debug for WebUIFolderPickerContinuationEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WebUIFolderPickerContinuationEventArgs").field(&self.0).finish()
    }
}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "deprecated"))]
unsafe impl ::windows_core::RuntimeType for WebUIFolderPickerContinuationEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.WebUI.WebUIFolderPickerContinuationEventArgs;{51882366-9f4b-498f-beb0-42684f6e1c29})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "deprecated"))]
unsafe impl ::windows_core::Interface for WebUIFolderPickerContinuationEventArgs {
    type Vtable = ::winrt_applicationmodel::Activation::IFolderPickerContinuationEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <::winrt_applicationmodel::Activation::IFolderPickerContinuationEventArgs as ::windows_core::Interface>::IID;
}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "deprecated"))]
impl ::windows_core::RuntimeName for WebUIFolderPickerContinuationEventArgs {
    const NAME: &'static str = "Windows.UI.WebUI.WebUIFolderPickerContinuationEventArgs";
}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "deprecated"))]
impl ::core::convert::From<WebUIFolderPickerContinuationEventArgs> for ::windows_core::IUnknown {
    fn from(value: WebUIFolderPickerContinuationEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "deprecated"))]
impl ::core::convert::From<&WebUIFolderPickerContinuationEventArgs> for ::windows_core::IUnknown {
    fn from(value: &WebUIFolderPickerContinuationEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "deprecated"))]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for WebUIFolderPickerContinuationEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "deprecated"))]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a WebUIFolderPickerContinuationEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "deprecated"))]
impl ::core::convert::From<WebUIFolderPickerContinuationEventArgs> for ::windows_core::IInspectable {
    fn from(value: WebUIFolderPickerContinuationEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "deprecated"))]
impl ::core::convert::From<&WebUIFolderPickerContinuationEventArgs> for ::windows_core::IInspectable {
    fn from(value: &WebUIFolderPickerContinuationEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "deprecated"))]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for WebUIFolderPickerContinuationEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "deprecated"))]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a WebUIFolderPickerContinuationEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "deprecated"))]
impl ::core::convert::TryFrom<WebUIFolderPickerContinuationEventArgs> for ::winrt_applicationmodel::Activation::IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: WebUIFolderPickerContinuationEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "deprecated"))]
impl ::core::convert::TryFrom<&WebUIFolderPickerContinuationEventArgs> for ::winrt_applicationmodel::Activation::IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &WebUIFolderPickerContinuationEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "deprecated"))]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgs> for WebUIFolderPickerContinuationEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "deprecated"))]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgs> for &WebUIFolderPickerContinuationEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgs> {
        ::core::convert::TryInto::<::winrt_applicationmodel::Activation::IActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "deprecated"))]
impl ::core::convert::TryFrom<WebUIFolderPickerContinuationEventArgs> for IActivatedEventArgsDeferral {
    type Error = ::windows_core::Error;
    fn try_from(value: WebUIFolderPickerContinuationEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "deprecated"))]
impl ::core::convert::TryFrom<&WebUIFolderPickerContinuationEventArgs> for IActivatedEventArgsDeferral {
    type Error = ::windows_core::Error;
    fn try_from(value: &WebUIFolderPickerContinuationEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "deprecated"))]
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgsDeferral> for WebUIFolderPickerContinuationEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgsDeferral> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "deprecated"))]
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgsDeferral> for &WebUIFolderPickerContinuationEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgsDeferral> {
        ::core::convert::TryInto::<IActivatedEventArgsDeferral>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "deprecated"))]
impl ::core::convert::TryFrom<WebUIFolderPickerContinuationEventArgs> for ::winrt_applicationmodel::Activation::IActivatedEventArgsWithUser {
    type Error = ::windows_core::Error;
    fn try_from(value: WebUIFolderPickerContinuationEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "deprecated"))]
impl ::core::convert::TryFrom<&WebUIFolderPickerContinuationEventArgs> for ::winrt_applicationmodel::Activation::IActivatedEventArgsWithUser {
    type Error = ::windows_core::Error;
    fn try_from(value: &WebUIFolderPickerContinuationEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "deprecated"))]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgsWithUser> for WebUIFolderPickerContinuationEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgsWithUser> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "deprecated"))]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgsWithUser> for &WebUIFolderPickerContinuationEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgsWithUser> {
        ::core::convert::TryInto::<::winrt_applicationmodel::Activation::IActivatedEventArgsWithUser>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "deprecated"))]
impl ::core::convert::TryFrom<WebUIFolderPickerContinuationEventArgs> for ::winrt_applicationmodel::Activation::IContinuationActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: WebUIFolderPickerContinuationEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "deprecated"))]
impl ::core::convert::TryFrom<&WebUIFolderPickerContinuationEventArgs> for ::winrt_applicationmodel::Activation::IContinuationActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &WebUIFolderPickerContinuationEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "deprecated"))]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IContinuationActivatedEventArgs> for WebUIFolderPickerContinuationEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IContinuationActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "deprecated"))]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IContinuationActivatedEventArgs> for &WebUIFolderPickerContinuationEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IContinuationActivatedEventArgs> {
        ::core::convert::TryInto::<::winrt_applicationmodel::Activation::IContinuationActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "deprecated"))]
impl ::core::convert::TryFrom<WebUIFolderPickerContinuationEventArgs> for ::winrt_applicationmodel::Activation::IFolderPickerContinuationEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: WebUIFolderPickerContinuationEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "deprecated"))]
impl ::core::convert::TryFrom<&WebUIFolderPickerContinuationEventArgs> for ::winrt_applicationmodel::Activation::IFolderPickerContinuationEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &WebUIFolderPickerContinuationEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "deprecated"))]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IFolderPickerContinuationEventArgs> for WebUIFolderPickerContinuationEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IFolderPickerContinuationEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "deprecated"))]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IFolderPickerContinuationEventArgs> for &WebUIFolderPickerContinuationEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IFolderPickerContinuationEventArgs> {
        ::core::convert::TryInto::<::winrt_applicationmodel::Activation::IFolderPickerContinuationEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
#[repr(transparent)]
pub struct WebUILaunchActivatedEventArgs(::windows_core::IUnknown);
#[cfg(feature = "ApplicationModel_Activation")]
impl WebUILaunchActivatedEventArgs {
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Kind(&self) -> ::windows_core::Result<::winrt_applicationmodel::Activation::ActivationKind> {
        let this = &::windows_core::Interface::cast::<::winrt_applicationmodel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_applicationmodel::Activation::ActivationKind>::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_applicationmodel::Activation::ActivationKind>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<::winrt_applicationmodel::Activation::ApplicationExecutionState> {
        let this = &::windows_core::Interface::cast::<::winrt_applicationmodel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_applicationmodel::Activation::ApplicationExecutionState>::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_applicationmodel::Activation::ApplicationExecutionState>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn SplashScreen(&self) -> ::windows_core::Result<::winrt_applicationmodel::Activation::SplashScreen> {
        let this = &::windows_core::Interface::cast::<::winrt_applicationmodel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_applicationmodel::Activation::SplashScreen>(result__)
        }
    }
    pub fn ActivatedOperation(&self) -> ::windows_core::Result<ActivatedOperation> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgsDeferral>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ActivatedOperation)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivatedOperation>(result__)
        }
    }
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "System"))]
    pub fn User(&self) -> ::windows_core::Result<::winrt_system::User> {
        let this = &::windows_core::Interface::cast::<::winrt_applicationmodel::Activation::IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).User)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_system::User>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn CurrentlyShownApplicationViewId(&self) -> ::windows_core::Result<i32> {
        let this = &::windows_core::Interface::cast::<::winrt_applicationmodel::Activation::IApplicationViewActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows_core::Interface::vtable(this).CurrentlyShownApplicationViewId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Arguments(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Arguments)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn TileId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).TileId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn TileActivatedInfo(&self) -> ::windows_core::Result<::winrt_applicationmodel::Activation::TileActivatedInfo> {
        let this = &::windows_core::Interface::cast::<::winrt_applicationmodel::Activation::ILaunchActivatedEventArgs2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TileActivatedInfo)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_applicationmodel::Activation::TileActivatedInfo>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn PrelaunchActivated(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<::winrt_applicationmodel::Activation::IPrelaunchActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).PrelaunchActivated)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::clone::Clone for WebUILaunchActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::cmp::PartialEq for WebUILaunchActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::cmp::Eq for WebUILaunchActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::fmt::Debug for WebUILaunchActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WebUILaunchActivatedEventArgs").field(&self.0).finish()
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows_core::RuntimeType for WebUILaunchActivatedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.WebUI.WebUILaunchActivatedEventArgs;{fbc93e26-a14a-4b4f-82b0-33bed920af52})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows_core::Interface for WebUILaunchActivatedEventArgs {
    type Vtable = ::winrt_applicationmodel::Activation::ILaunchActivatedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <::winrt_applicationmodel::Activation::ILaunchActivatedEventArgs as ::windows_core::Interface>::IID;
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::RuntimeName for WebUILaunchActivatedEventArgs {
    const NAME: &'static str = "Windows.UI.WebUI.WebUILaunchActivatedEventArgs";
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<WebUILaunchActivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: WebUILaunchActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<&WebUILaunchActivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &WebUILaunchActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for WebUILaunchActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a WebUILaunchActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<WebUILaunchActivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: WebUILaunchActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<&WebUILaunchActivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &WebUILaunchActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for WebUILaunchActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a WebUILaunchActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUILaunchActivatedEventArgs> for ::winrt_applicationmodel::Activation::IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: WebUILaunchActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUILaunchActivatedEventArgs> for ::winrt_applicationmodel::Activation::IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &WebUILaunchActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgs> for WebUILaunchActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgs> for &WebUILaunchActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgs> {
        ::core::convert::TryInto::<::winrt_applicationmodel::Activation::IActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUILaunchActivatedEventArgs> for IActivatedEventArgsDeferral {
    type Error = ::windows_core::Error;
    fn try_from(value: WebUILaunchActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUILaunchActivatedEventArgs> for IActivatedEventArgsDeferral {
    type Error = ::windows_core::Error;
    fn try_from(value: &WebUILaunchActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgsDeferral> for WebUILaunchActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgsDeferral> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgsDeferral> for &WebUILaunchActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgsDeferral> {
        ::core::convert::TryInto::<IActivatedEventArgsDeferral>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUILaunchActivatedEventArgs> for ::winrt_applicationmodel::Activation::IActivatedEventArgsWithUser {
    type Error = ::windows_core::Error;
    fn try_from(value: WebUILaunchActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUILaunchActivatedEventArgs> for ::winrt_applicationmodel::Activation::IActivatedEventArgsWithUser {
    type Error = ::windows_core::Error;
    fn try_from(value: &WebUILaunchActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgsWithUser> for WebUILaunchActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgsWithUser> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgsWithUser> for &WebUILaunchActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgsWithUser> {
        ::core::convert::TryInto::<::winrt_applicationmodel::Activation::IActivatedEventArgsWithUser>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUILaunchActivatedEventArgs> for ::winrt_applicationmodel::Activation::IApplicationViewActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: WebUILaunchActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUILaunchActivatedEventArgs> for ::winrt_applicationmodel::Activation::IApplicationViewActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &WebUILaunchActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IApplicationViewActivatedEventArgs> for WebUILaunchActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IApplicationViewActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IApplicationViewActivatedEventArgs> for &WebUILaunchActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IApplicationViewActivatedEventArgs> {
        ::core::convert::TryInto::<::winrt_applicationmodel::Activation::IApplicationViewActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUILaunchActivatedEventArgs> for ::winrt_applicationmodel::Activation::ILaunchActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: WebUILaunchActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUILaunchActivatedEventArgs> for ::winrt_applicationmodel::Activation::ILaunchActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &WebUILaunchActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::ILaunchActivatedEventArgs> for WebUILaunchActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::ILaunchActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::ILaunchActivatedEventArgs> for &WebUILaunchActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::ILaunchActivatedEventArgs> {
        ::core::convert::TryInto::<::winrt_applicationmodel::Activation::ILaunchActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUILaunchActivatedEventArgs> for ::winrt_applicationmodel::Activation::ILaunchActivatedEventArgs2 {
    type Error = ::windows_core::Error;
    fn try_from(value: WebUILaunchActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUILaunchActivatedEventArgs> for ::winrt_applicationmodel::Activation::ILaunchActivatedEventArgs2 {
    type Error = ::windows_core::Error;
    fn try_from(value: &WebUILaunchActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::ILaunchActivatedEventArgs2> for WebUILaunchActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::ILaunchActivatedEventArgs2> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::ILaunchActivatedEventArgs2> for &WebUILaunchActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::ILaunchActivatedEventArgs2> {
        ::core::convert::TryInto::<::winrt_applicationmodel::Activation::ILaunchActivatedEventArgs2>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUILaunchActivatedEventArgs> for ::winrt_applicationmodel::Activation::IPrelaunchActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: WebUILaunchActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUILaunchActivatedEventArgs> for ::winrt_applicationmodel::Activation::IPrelaunchActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &WebUILaunchActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IPrelaunchActivatedEventArgs> for WebUILaunchActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IPrelaunchActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IPrelaunchActivatedEventArgs> for &WebUILaunchActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IPrelaunchActivatedEventArgs> {
        ::core::convert::TryInto::<::winrt_applicationmodel::Activation::IPrelaunchActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
#[repr(transparent)]
pub struct WebUILockScreenActivatedEventArgs(::windows_core::IUnknown);
#[cfg(feature = "ApplicationModel_Activation")]
impl WebUILockScreenActivatedEventArgs {
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Kind(&self) -> ::windows_core::Result<::winrt_applicationmodel::Activation::ActivationKind> {
        let this = &::windows_core::Interface::cast::<::winrt_applicationmodel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_applicationmodel::Activation::ActivationKind>::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_applicationmodel::Activation::ActivationKind>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<::winrt_applicationmodel::Activation::ApplicationExecutionState> {
        let this = &::windows_core::Interface::cast::<::winrt_applicationmodel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_applicationmodel::Activation::ApplicationExecutionState>::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_applicationmodel::Activation::ApplicationExecutionState>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn SplashScreen(&self) -> ::windows_core::Result<::winrt_applicationmodel::Activation::SplashScreen> {
        let this = &::windows_core::Interface::cast::<::winrt_applicationmodel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_applicationmodel::Activation::SplashScreen>(result__)
        }
    }
    pub fn ActivatedOperation(&self) -> ::windows_core::Result<ActivatedOperation> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgsDeferral>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ActivatedOperation)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivatedOperation>(result__)
        }
    }
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "System"))]
    pub fn User(&self) -> ::windows_core::Result<::winrt_system::User> {
        let this = &::windows_core::Interface::cast::<::winrt_applicationmodel::Activation::IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).User)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_system::User>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn CurrentlyShownApplicationViewId(&self) -> ::windows_core::Result<i32> {
        let this = &::windows_core::Interface::cast::<::winrt_applicationmodel::Activation::IApplicationViewActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows_core::Interface::vtable(this).CurrentlyShownApplicationViewId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Info(&self) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows_core::Interface::vtable(this).Info)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::IInspectable>(result__)
        }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::clone::Clone for WebUILockScreenActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::cmp::PartialEq for WebUILockScreenActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::cmp::Eq for WebUILockScreenActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::fmt::Debug for WebUILockScreenActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WebUILockScreenActivatedEventArgs").field(&self.0).finish()
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows_core::RuntimeType for WebUILockScreenActivatedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.WebUI.WebUILockScreenActivatedEventArgs;{3ca77966-6108-4a41-8220-ee7d133c8532})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows_core::Interface for WebUILockScreenActivatedEventArgs {
    type Vtable = ::winrt_applicationmodel::Activation::ILockScreenActivatedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <::winrt_applicationmodel::Activation::ILockScreenActivatedEventArgs as ::windows_core::Interface>::IID;
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::RuntimeName for WebUILockScreenActivatedEventArgs {
    const NAME: &'static str = "Windows.UI.WebUI.WebUILockScreenActivatedEventArgs";
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<WebUILockScreenActivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: WebUILockScreenActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<&WebUILockScreenActivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &WebUILockScreenActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for WebUILockScreenActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a WebUILockScreenActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<WebUILockScreenActivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: WebUILockScreenActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<&WebUILockScreenActivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &WebUILockScreenActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for WebUILockScreenActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a WebUILockScreenActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUILockScreenActivatedEventArgs> for ::winrt_applicationmodel::Activation::IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: WebUILockScreenActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUILockScreenActivatedEventArgs> for ::winrt_applicationmodel::Activation::IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &WebUILockScreenActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgs> for WebUILockScreenActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgs> for &WebUILockScreenActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgs> {
        ::core::convert::TryInto::<::winrt_applicationmodel::Activation::IActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUILockScreenActivatedEventArgs> for IActivatedEventArgsDeferral {
    type Error = ::windows_core::Error;
    fn try_from(value: WebUILockScreenActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUILockScreenActivatedEventArgs> for IActivatedEventArgsDeferral {
    type Error = ::windows_core::Error;
    fn try_from(value: &WebUILockScreenActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgsDeferral> for WebUILockScreenActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgsDeferral> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgsDeferral> for &WebUILockScreenActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgsDeferral> {
        ::core::convert::TryInto::<IActivatedEventArgsDeferral>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUILockScreenActivatedEventArgs> for ::winrt_applicationmodel::Activation::IActivatedEventArgsWithUser {
    type Error = ::windows_core::Error;
    fn try_from(value: WebUILockScreenActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUILockScreenActivatedEventArgs> for ::winrt_applicationmodel::Activation::IActivatedEventArgsWithUser {
    type Error = ::windows_core::Error;
    fn try_from(value: &WebUILockScreenActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgsWithUser> for WebUILockScreenActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgsWithUser> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgsWithUser> for &WebUILockScreenActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgsWithUser> {
        ::core::convert::TryInto::<::winrt_applicationmodel::Activation::IActivatedEventArgsWithUser>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUILockScreenActivatedEventArgs> for ::winrt_applicationmodel::Activation::IApplicationViewActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: WebUILockScreenActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUILockScreenActivatedEventArgs> for ::winrt_applicationmodel::Activation::IApplicationViewActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &WebUILockScreenActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IApplicationViewActivatedEventArgs> for WebUILockScreenActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IApplicationViewActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IApplicationViewActivatedEventArgs> for &WebUILockScreenActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IApplicationViewActivatedEventArgs> {
        ::core::convert::TryInto::<::winrt_applicationmodel::Activation::IApplicationViewActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUILockScreenActivatedEventArgs> for ::winrt_applicationmodel::Activation::ILockScreenActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: WebUILockScreenActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUILockScreenActivatedEventArgs> for ::winrt_applicationmodel::Activation::ILockScreenActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &WebUILockScreenActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::ILockScreenActivatedEventArgs> for WebUILockScreenActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::ILockScreenActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::ILockScreenActivatedEventArgs> for &WebUILockScreenActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::ILockScreenActivatedEventArgs> {
        ::core::convert::TryInto::<::winrt_applicationmodel::Activation::ILockScreenActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
#[repr(transparent)]
pub struct WebUILockScreenCallActivatedEventArgs(::windows_core::IUnknown);
#[cfg(feature = "ApplicationModel_Activation")]
impl WebUILockScreenCallActivatedEventArgs {
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Kind(&self) -> ::windows_core::Result<::winrt_applicationmodel::Activation::ActivationKind> {
        let this = &::windows_core::Interface::cast::<::winrt_applicationmodel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_applicationmodel::Activation::ActivationKind>::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_applicationmodel::Activation::ActivationKind>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<::winrt_applicationmodel::Activation::ApplicationExecutionState> {
        let this = &::windows_core::Interface::cast::<::winrt_applicationmodel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_applicationmodel::Activation::ApplicationExecutionState>::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_applicationmodel::Activation::ApplicationExecutionState>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn SplashScreen(&self) -> ::windows_core::Result<::winrt_applicationmodel::Activation::SplashScreen> {
        let this = &::windows_core::Interface::cast::<::winrt_applicationmodel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_applicationmodel::Activation::SplashScreen>(result__)
        }
    }
    pub fn ActivatedOperation(&self) -> ::windows_core::Result<ActivatedOperation> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgsDeferral>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ActivatedOperation)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivatedOperation>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn CurrentlyShownApplicationViewId(&self) -> ::windows_core::Result<i32> {
        let this = &::windows_core::Interface::cast::<::winrt_applicationmodel::Activation::IApplicationViewActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows_core::Interface::vtable(this).CurrentlyShownApplicationViewId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Arguments(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<::winrt_applicationmodel::Activation::ILaunchActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Arguments)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn TileId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<::winrt_applicationmodel::Activation::ILaunchActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).TileId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "ApplicationModel_Calls"))]
    pub fn CallUI(&self) -> ::windows_core::Result<::winrt_applicationmodel::Calls::LockScreenCallUI> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CallUI)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_applicationmodel::Calls::LockScreenCallUI>(result__)
        }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::clone::Clone for WebUILockScreenCallActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::cmp::PartialEq for WebUILockScreenCallActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::cmp::Eq for WebUILockScreenCallActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::fmt::Debug for WebUILockScreenCallActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WebUILockScreenCallActivatedEventArgs").field(&self.0).finish()
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows_core::RuntimeType for WebUILockScreenCallActivatedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.WebUI.WebUILockScreenCallActivatedEventArgs;{06f37fbe-b5f2-448b-b13e-e328ac1c516a})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows_core::Interface for WebUILockScreenCallActivatedEventArgs {
    type Vtable = ::winrt_applicationmodel::Activation::ILockScreenCallActivatedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <::winrt_applicationmodel::Activation::ILockScreenCallActivatedEventArgs as ::windows_core::Interface>::IID;
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::RuntimeName for WebUILockScreenCallActivatedEventArgs {
    const NAME: &'static str = "Windows.UI.WebUI.WebUILockScreenCallActivatedEventArgs";
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<WebUILockScreenCallActivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: WebUILockScreenCallActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<&WebUILockScreenCallActivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &WebUILockScreenCallActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for WebUILockScreenCallActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a WebUILockScreenCallActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<WebUILockScreenCallActivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: WebUILockScreenCallActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<&WebUILockScreenCallActivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &WebUILockScreenCallActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for WebUILockScreenCallActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a WebUILockScreenCallActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUILockScreenCallActivatedEventArgs> for ::winrt_applicationmodel::Activation::IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: WebUILockScreenCallActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUILockScreenCallActivatedEventArgs> for ::winrt_applicationmodel::Activation::IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &WebUILockScreenCallActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgs> for WebUILockScreenCallActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgs> for &WebUILockScreenCallActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgs> {
        ::core::convert::TryInto::<::winrt_applicationmodel::Activation::IActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUILockScreenCallActivatedEventArgs> for IActivatedEventArgsDeferral {
    type Error = ::windows_core::Error;
    fn try_from(value: WebUILockScreenCallActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUILockScreenCallActivatedEventArgs> for IActivatedEventArgsDeferral {
    type Error = ::windows_core::Error;
    fn try_from(value: &WebUILockScreenCallActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgsDeferral> for WebUILockScreenCallActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgsDeferral> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgsDeferral> for &WebUILockScreenCallActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgsDeferral> {
        ::core::convert::TryInto::<IActivatedEventArgsDeferral>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUILockScreenCallActivatedEventArgs> for ::winrt_applicationmodel::Activation::IApplicationViewActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: WebUILockScreenCallActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUILockScreenCallActivatedEventArgs> for ::winrt_applicationmodel::Activation::IApplicationViewActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &WebUILockScreenCallActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IApplicationViewActivatedEventArgs> for WebUILockScreenCallActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IApplicationViewActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IApplicationViewActivatedEventArgs> for &WebUILockScreenCallActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IApplicationViewActivatedEventArgs> {
        ::core::convert::TryInto::<::winrt_applicationmodel::Activation::IApplicationViewActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUILockScreenCallActivatedEventArgs> for ::winrt_applicationmodel::Activation::ILaunchActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: WebUILockScreenCallActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUILockScreenCallActivatedEventArgs> for ::winrt_applicationmodel::Activation::ILaunchActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &WebUILockScreenCallActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::ILaunchActivatedEventArgs> for WebUILockScreenCallActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::ILaunchActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::ILaunchActivatedEventArgs> for &WebUILockScreenCallActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::ILaunchActivatedEventArgs> {
        ::core::convert::TryInto::<::winrt_applicationmodel::Activation::ILaunchActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUILockScreenCallActivatedEventArgs> for ::winrt_applicationmodel::Activation::ILockScreenCallActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: WebUILockScreenCallActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUILockScreenCallActivatedEventArgs> for ::winrt_applicationmodel::Activation::ILockScreenCallActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &WebUILockScreenCallActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::ILockScreenCallActivatedEventArgs> for WebUILockScreenCallActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::ILockScreenCallActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::ILockScreenCallActivatedEventArgs> for &WebUILockScreenCallActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::ILockScreenCallActivatedEventArgs> {
        ::core::convert::TryInto::<::winrt_applicationmodel::Activation::ILockScreenCallActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
#[repr(transparent)]
pub struct WebUILockScreenComponentActivatedEventArgs(::windows_core::IUnknown);
#[cfg(feature = "ApplicationModel_Activation")]
impl WebUILockScreenComponentActivatedEventArgs {
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Kind(&self) -> ::windows_core::Result<::winrt_applicationmodel::Activation::ActivationKind> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_applicationmodel::Activation::ActivationKind>::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_applicationmodel::Activation::ActivationKind>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<::winrt_applicationmodel::Activation::ApplicationExecutionState> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_applicationmodel::Activation::ApplicationExecutionState>::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_applicationmodel::Activation::ApplicationExecutionState>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn SplashScreen(&self) -> ::windows_core::Result<::winrt_applicationmodel::Activation::SplashScreen> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_applicationmodel::Activation::SplashScreen>(result__)
        }
    }
    pub fn ActivatedOperation(&self) -> ::windows_core::Result<ActivatedOperation> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgsDeferral>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ActivatedOperation)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivatedOperation>(result__)
        }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::clone::Clone for WebUILockScreenComponentActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::cmp::PartialEq for WebUILockScreenComponentActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::cmp::Eq for WebUILockScreenComponentActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::fmt::Debug for WebUILockScreenComponentActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WebUILockScreenComponentActivatedEventArgs").field(&self.0).finish()
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows_core::RuntimeType for WebUILockScreenComponentActivatedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.WebUI.WebUILockScreenComponentActivatedEventArgs;{cf651713-cd08-4fd8-b697-a281b6544e2e})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows_core::Interface for WebUILockScreenComponentActivatedEventArgs {
    type Vtable = ::winrt_applicationmodel::Activation::IActivatedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <::winrt_applicationmodel::Activation::IActivatedEventArgs as ::windows_core::Interface>::IID;
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::RuntimeName for WebUILockScreenComponentActivatedEventArgs {
    const NAME: &'static str = "Windows.UI.WebUI.WebUILockScreenComponentActivatedEventArgs";
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<WebUILockScreenComponentActivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: WebUILockScreenComponentActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<&WebUILockScreenComponentActivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &WebUILockScreenComponentActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for WebUILockScreenComponentActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a WebUILockScreenComponentActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<WebUILockScreenComponentActivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: WebUILockScreenComponentActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<&WebUILockScreenComponentActivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &WebUILockScreenComponentActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for WebUILockScreenComponentActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a WebUILockScreenComponentActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUILockScreenComponentActivatedEventArgs> for ::winrt_applicationmodel::Activation::IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: WebUILockScreenComponentActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUILockScreenComponentActivatedEventArgs> for ::winrt_applicationmodel::Activation::IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &WebUILockScreenComponentActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgs> for WebUILockScreenComponentActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgs> for &WebUILockScreenComponentActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgs> {
        ::core::convert::TryInto::<::winrt_applicationmodel::Activation::IActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUILockScreenComponentActivatedEventArgs> for IActivatedEventArgsDeferral {
    type Error = ::windows_core::Error;
    fn try_from(value: WebUILockScreenComponentActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUILockScreenComponentActivatedEventArgs> for IActivatedEventArgsDeferral {
    type Error = ::windows_core::Error;
    fn try_from(value: &WebUILockScreenComponentActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgsDeferral> for WebUILockScreenComponentActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgsDeferral> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgsDeferral> for &WebUILockScreenComponentActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgsDeferral> {
        ::core::convert::TryInto::<IActivatedEventArgsDeferral>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[repr(transparent)]
pub struct WebUINavigatedDeferral(::windows_core::IUnknown);
impl WebUINavigatedDeferral {
    pub fn Complete(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Complete)(::windows_core::Interface::as_raw(this)).ok() }
    }
}
impl ::core::clone::Clone for WebUINavigatedDeferral {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for WebUINavigatedDeferral {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for WebUINavigatedDeferral {}
impl ::core::fmt::Debug for WebUINavigatedDeferral {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WebUINavigatedDeferral").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for WebUINavigatedDeferral {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.WebUI.WebUINavigatedDeferral;{d804204d-831f-46e2-b432-3afce211f962})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for WebUINavigatedDeferral {
    type Vtable = IWebUINavigatedDeferral_Vtbl;
    const IID: ::windows_core::GUID = <IWebUINavigatedDeferral as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for WebUINavigatedDeferral {
    const NAME: &'static str = "Windows.UI.WebUI.WebUINavigatedDeferral";
}
impl ::core::convert::From<WebUINavigatedDeferral> for ::windows_core::IUnknown {
    fn from(value: WebUINavigatedDeferral) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WebUINavigatedDeferral> for ::windows_core::IUnknown {
    fn from(value: &WebUINavigatedDeferral) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for WebUINavigatedDeferral {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a WebUINavigatedDeferral {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<WebUINavigatedDeferral> for ::windows_core::IInspectable {
    fn from(value: WebUINavigatedDeferral) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WebUINavigatedDeferral> for ::windows_core::IInspectable {
    fn from(value: &WebUINavigatedDeferral) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for WebUINavigatedDeferral {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a WebUINavigatedDeferral {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(transparent)]
pub struct WebUINavigatedEventArgs(::windows_core::IUnknown);
impl WebUINavigatedEventArgs {
    pub fn NavigatedOperation(&self) -> ::windows_core::Result<WebUINavigatedOperation> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).NavigatedOperation)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<WebUINavigatedOperation>(result__)
        }
    }
}
impl ::core::clone::Clone for WebUINavigatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for WebUINavigatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for WebUINavigatedEventArgs {}
impl ::core::fmt::Debug for WebUINavigatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WebUINavigatedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for WebUINavigatedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.WebUI.WebUINavigatedEventArgs;{a75841b8-2499-4030-a69d-15d2d9cfe524})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for WebUINavigatedEventArgs {
    type Vtable = IWebUINavigatedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IWebUINavigatedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for WebUINavigatedEventArgs {
    const NAME: &'static str = "Windows.UI.WebUI.WebUINavigatedEventArgs";
}
impl ::core::convert::From<WebUINavigatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: WebUINavigatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WebUINavigatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &WebUINavigatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for WebUINavigatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a WebUINavigatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<WebUINavigatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: WebUINavigatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WebUINavigatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &WebUINavigatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for WebUINavigatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a WebUINavigatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<WebUINavigatedEventArgs> for IWebUINavigatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: WebUINavigatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&WebUINavigatedEventArgs> for IWebUINavigatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &WebUINavigatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IWebUINavigatedEventArgs> for WebUINavigatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IWebUINavigatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IWebUINavigatedEventArgs> for &WebUINavigatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IWebUINavigatedEventArgs> {
        ::core::convert::TryInto::<IWebUINavigatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[repr(transparent)]
pub struct WebUINavigatedOperation(::windows_core::IUnknown);
impl WebUINavigatedOperation {
    pub fn GetDeferral(&self) -> ::windows_core::Result<WebUINavigatedDeferral> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetDeferral)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<WebUINavigatedDeferral>(result__)
        }
    }
}
impl ::core::clone::Clone for WebUINavigatedOperation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for WebUINavigatedOperation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for WebUINavigatedOperation {}
impl ::core::fmt::Debug for WebUINavigatedOperation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WebUINavigatedOperation").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for WebUINavigatedOperation {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.WebUI.WebUINavigatedOperation;{7a965f08-8182-4a89-ab67-8492e8750d4b})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for WebUINavigatedOperation {
    type Vtable = IWebUINavigatedOperation_Vtbl;
    const IID: ::windows_core::GUID = <IWebUINavigatedOperation as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for WebUINavigatedOperation {
    const NAME: &'static str = "Windows.UI.WebUI.WebUINavigatedOperation";
}
impl ::core::convert::From<WebUINavigatedOperation> for ::windows_core::IUnknown {
    fn from(value: WebUINavigatedOperation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WebUINavigatedOperation> for ::windows_core::IUnknown {
    fn from(value: &WebUINavigatedOperation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for WebUINavigatedOperation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a WebUINavigatedOperation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<WebUINavigatedOperation> for ::windows_core::IInspectable {
    fn from(value: WebUINavigatedOperation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WebUINavigatedOperation> for ::windows_core::IInspectable {
    fn from(value: &WebUINavigatedOperation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for WebUINavigatedOperation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a WebUINavigatedOperation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
#[repr(transparent)]
pub struct WebUIPhoneCallActivatedEventArgs(::windows_core::IUnknown);
#[cfg(feature = "ApplicationModel_Activation")]
impl WebUIPhoneCallActivatedEventArgs {
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Kind(&self) -> ::windows_core::Result<::winrt_applicationmodel::Activation::ActivationKind> {
        let this = &::windows_core::Interface::cast::<::winrt_applicationmodel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_applicationmodel::Activation::ActivationKind>::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_applicationmodel::Activation::ActivationKind>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<::winrt_applicationmodel::Activation::ApplicationExecutionState> {
        let this = &::windows_core::Interface::cast::<::winrt_applicationmodel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_applicationmodel::Activation::ApplicationExecutionState>::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_applicationmodel::Activation::ApplicationExecutionState>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn SplashScreen(&self) -> ::windows_core::Result<::winrt_applicationmodel::Activation::SplashScreen> {
        let this = &::windows_core::Interface::cast::<::winrt_applicationmodel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_applicationmodel::Activation::SplashScreen>(result__)
        }
    }
    pub fn ActivatedOperation(&self) -> ::windows_core::Result<ActivatedOperation> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgsDeferral>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ActivatedOperation)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivatedOperation>(result__)
        }
    }
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "System"))]
    pub fn User(&self) -> ::windows_core::Result<::winrt_system::User> {
        let this = &::windows_core::Interface::cast::<::winrt_applicationmodel::Activation::IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).User)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_system::User>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn LineId(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::GUID>::zeroed();
            (::windows_core::Interface::vtable(this).LineId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::GUID>(result__)
        }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::clone::Clone for WebUIPhoneCallActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::cmp::PartialEq for WebUIPhoneCallActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::cmp::Eq for WebUIPhoneCallActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::fmt::Debug for WebUIPhoneCallActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WebUIPhoneCallActivatedEventArgs").field(&self.0).finish()
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows_core::RuntimeType for WebUIPhoneCallActivatedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.WebUI.WebUIPhoneCallActivatedEventArgs;{54615221-a3c1-4ced-b62f-8c60523619ad})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows_core::Interface for WebUIPhoneCallActivatedEventArgs {
    type Vtable = ::winrt_applicationmodel::Activation::IPhoneCallActivatedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <::winrt_applicationmodel::Activation::IPhoneCallActivatedEventArgs as ::windows_core::Interface>::IID;
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::RuntimeName for WebUIPhoneCallActivatedEventArgs {
    const NAME: &'static str = "Windows.UI.WebUI.WebUIPhoneCallActivatedEventArgs";
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<WebUIPhoneCallActivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: WebUIPhoneCallActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<&WebUIPhoneCallActivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &WebUIPhoneCallActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for WebUIPhoneCallActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a WebUIPhoneCallActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<WebUIPhoneCallActivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: WebUIPhoneCallActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<&WebUIPhoneCallActivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &WebUIPhoneCallActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for WebUIPhoneCallActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a WebUIPhoneCallActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIPhoneCallActivatedEventArgs> for ::winrt_applicationmodel::Activation::IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: WebUIPhoneCallActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIPhoneCallActivatedEventArgs> for ::winrt_applicationmodel::Activation::IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &WebUIPhoneCallActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgs> for WebUIPhoneCallActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgs> for &WebUIPhoneCallActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgs> {
        ::core::convert::TryInto::<::winrt_applicationmodel::Activation::IActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIPhoneCallActivatedEventArgs> for IActivatedEventArgsDeferral {
    type Error = ::windows_core::Error;
    fn try_from(value: WebUIPhoneCallActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIPhoneCallActivatedEventArgs> for IActivatedEventArgsDeferral {
    type Error = ::windows_core::Error;
    fn try_from(value: &WebUIPhoneCallActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgsDeferral> for WebUIPhoneCallActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgsDeferral> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgsDeferral> for &WebUIPhoneCallActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgsDeferral> {
        ::core::convert::TryInto::<IActivatedEventArgsDeferral>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIPhoneCallActivatedEventArgs> for ::winrt_applicationmodel::Activation::IActivatedEventArgsWithUser {
    type Error = ::windows_core::Error;
    fn try_from(value: WebUIPhoneCallActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIPhoneCallActivatedEventArgs> for ::winrt_applicationmodel::Activation::IActivatedEventArgsWithUser {
    type Error = ::windows_core::Error;
    fn try_from(value: &WebUIPhoneCallActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgsWithUser> for WebUIPhoneCallActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgsWithUser> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgsWithUser> for &WebUIPhoneCallActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgsWithUser> {
        ::core::convert::TryInto::<::winrt_applicationmodel::Activation::IActivatedEventArgsWithUser>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIPhoneCallActivatedEventArgs> for ::winrt_applicationmodel::Activation::IPhoneCallActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: WebUIPhoneCallActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIPhoneCallActivatedEventArgs> for ::winrt_applicationmodel::Activation::IPhoneCallActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &WebUIPhoneCallActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IPhoneCallActivatedEventArgs> for WebUIPhoneCallActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IPhoneCallActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IPhoneCallActivatedEventArgs> for &WebUIPhoneCallActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IPhoneCallActivatedEventArgs> {
        ::core::convert::TryInto::<::winrt_applicationmodel::Activation::IPhoneCallActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::core::marker::Send for WebUIPhoneCallActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::core::marker::Sync for WebUIPhoneCallActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
#[repr(transparent)]
pub struct WebUIPrint3DWorkflowActivatedEventArgs(::windows_core::IUnknown);
#[cfg(feature = "ApplicationModel_Activation")]
impl WebUIPrint3DWorkflowActivatedEventArgs {
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Kind(&self) -> ::windows_core::Result<::winrt_applicationmodel::Activation::ActivationKind> {
        let this = &::windows_core::Interface::cast::<::winrt_applicationmodel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_applicationmodel::Activation::ActivationKind>::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_applicationmodel::Activation::ActivationKind>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<::winrt_applicationmodel::Activation::ApplicationExecutionState> {
        let this = &::windows_core::Interface::cast::<::winrt_applicationmodel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_applicationmodel::Activation::ApplicationExecutionState>::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_applicationmodel::Activation::ApplicationExecutionState>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn SplashScreen(&self) -> ::windows_core::Result<::winrt_applicationmodel::Activation::SplashScreen> {
        let this = &::windows_core::Interface::cast::<::winrt_applicationmodel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_applicationmodel::Activation::SplashScreen>(result__)
        }
    }
    pub fn ActivatedOperation(&self) -> ::windows_core::Result<ActivatedOperation> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgsDeferral>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ActivatedOperation)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivatedOperation>(result__)
        }
    }
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "Devices_Printers_Extensions"))]
    pub fn Workflow(&self) -> ::windows_core::Result<::winrt_devices::Printers::Extensions::Print3DWorkflow> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Workflow)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_devices::Printers::Extensions::Print3DWorkflow>(result__)
        }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::clone::Clone for WebUIPrint3DWorkflowActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::cmp::PartialEq for WebUIPrint3DWorkflowActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::cmp::Eq for WebUIPrint3DWorkflowActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::fmt::Debug for WebUIPrint3DWorkflowActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WebUIPrint3DWorkflowActivatedEventArgs").field(&self.0).finish()
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows_core::RuntimeType for WebUIPrint3DWorkflowActivatedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.WebUI.WebUIPrint3DWorkflowActivatedEventArgs;{3f57e78b-f2ac-4619-8302-ef855e1c9b90})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows_core::Interface for WebUIPrint3DWorkflowActivatedEventArgs {
    type Vtable = ::winrt_applicationmodel::Activation::IPrint3DWorkflowActivatedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <::winrt_applicationmodel::Activation::IPrint3DWorkflowActivatedEventArgs as ::windows_core::Interface>::IID;
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::RuntimeName for WebUIPrint3DWorkflowActivatedEventArgs {
    const NAME: &'static str = "Windows.UI.WebUI.WebUIPrint3DWorkflowActivatedEventArgs";
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<WebUIPrint3DWorkflowActivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: WebUIPrint3DWorkflowActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<&WebUIPrint3DWorkflowActivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &WebUIPrint3DWorkflowActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for WebUIPrint3DWorkflowActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a WebUIPrint3DWorkflowActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<WebUIPrint3DWorkflowActivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: WebUIPrint3DWorkflowActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<&WebUIPrint3DWorkflowActivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &WebUIPrint3DWorkflowActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for WebUIPrint3DWorkflowActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a WebUIPrint3DWorkflowActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIPrint3DWorkflowActivatedEventArgs> for ::winrt_applicationmodel::Activation::IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: WebUIPrint3DWorkflowActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIPrint3DWorkflowActivatedEventArgs> for ::winrt_applicationmodel::Activation::IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &WebUIPrint3DWorkflowActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgs> for WebUIPrint3DWorkflowActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgs> for &WebUIPrint3DWorkflowActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgs> {
        ::core::convert::TryInto::<::winrt_applicationmodel::Activation::IActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIPrint3DWorkflowActivatedEventArgs> for IActivatedEventArgsDeferral {
    type Error = ::windows_core::Error;
    fn try_from(value: WebUIPrint3DWorkflowActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIPrint3DWorkflowActivatedEventArgs> for IActivatedEventArgsDeferral {
    type Error = ::windows_core::Error;
    fn try_from(value: &WebUIPrint3DWorkflowActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgsDeferral> for WebUIPrint3DWorkflowActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgsDeferral> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgsDeferral> for &WebUIPrint3DWorkflowActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgsDeferral> {
        ::core::convert::TryInto::<IActivatedEventArgsDeferral>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIPrint3DWorkflowActivatedEventArgs> for ::winrt_applicationmodel::Activation::IPrint3DWorkflowActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: WebUIPrint3DWorkflowActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIPrint3DWorkflowActivatedEventArgs> for ::winrt_applicationmodel::Activation::IPrint3DWorkflowActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &WebUIPrint3DWorkflowActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IPrint3DWorkflowActivatedEventArgs> for WebUIPrint3DWorkflowActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IPrint3DWorkflowActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IPrint3DWorkflowActivatedEventArgs> for &WebUIPrint3DWorkflowActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IPrint3DWorkflowActivatedEventArgs> {
        ::core::convert::TryInto::<::winrt_applicationmodel::Activation::IPrint3DWorkflowActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
#[repr(transparent)]
pub struct WebUIPrintTaskSettingsActivatedEventArgs(::windows_core::IUnknown);
#[cfg(feature = "ApplicationModel_Activation")]
impl WebUIPrintTaskSettingsActivatedEventArgs {
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Kind(&self) -> ::windows_core::Result<::winrt_applicationmodel::Activation::ActivationKind> {
        let this = &::windows_core::Interface::cast::<::winrt_applicationmodel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_applicationmodel::Activation::ActivationKind>::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_applicationmodel::Activation::ActivationKind>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<::winrt_applicationmodel::Activation::ApplicationExecutionState> {
        let this = &::windows_core::Interface::cast::<::winrt_applicationmodel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_applicationmodel::Activation::ApplicationExecutionState>::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_applicationmodel::Activation::ApplicationExecutionState>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn SplashScreen(&self) -> ::windows_core::Result<::winrt_applicationmodel::Activation::SplashScreen> {
        let this = &::windows_core::Interface::cast::<::winrt_applicationmodel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_applicationmodel::Activation::SplashScreen>(result__)
        }
    }
    pub fn ActivatedOperation(&self) -> ::windows_core::Result<ActivatedOperation> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgsDeferral>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ActivatedOperation)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivatedOperation>(result__)
        }
    }
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "Devices_Printers_Extensions"))]
    pub fn Configuration(&self) -> ::windows_core::Result<::winrt_devices::Printers::Extensions::PrintTaskConfiguration> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Configuration)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_devices::Printers::Extensions::PrintTaskConfiguration>(result__)
        }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::clone::Clone for WebUIPrintTaskSettingsActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::cmp::PartialEq for WebUIPrintTaskSettingsActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::cmp::Eq for WebUIPrintTaskSettingsActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::fmt::Debug for WebUIPrintTaskSettingsActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WebUIPrintTaskSettingsActivatedEventArgs").field(&self.0).finish()
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows_core::RuntimeType for WebUIPrintTaskSettingsActivatedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.WebUI.WebUIPrintTaskSettingsActivatedEventArgs;{ee30a0c9-ce56-4865-ba8e-8954ac271107})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows_core::Interface for WebUIPrintTaskSettingsActivatedEventArgs {
    type Vtable = ::winrt_applicationmodel::Activation::IPrintTaskSettingsActivatedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <::winrt_applicationmodel::Activation::IPrintTaskSettingsActivatedEventArgs as ::windows_core::Interface>::IID;
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::RuntimeName for WebUIPrintTaskSettingsActivatedEventArgs {
    const NAME: &'static str = "Windows.UI.WebUI.WebUIPrintTaskSettingsActivatedEventArgs";
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<WebUIPrintTaskSettingsActivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: WebUIPrintTaskSettingsActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<&WebUIPrintTaskSettingsActivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &WebUIPrintTaskSettingsActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for WebUIPrintTaskSettingsActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a WebUIPrintTaskSettingsActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<WebUIPrintTaskSettingsActivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: WebUIPrintTaskSettingsActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<&WebUIPrintTaskSettingsActivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &WebUIPrintTaskSettingsActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for WebUIPrintTaskSettingsActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a WebUIPrintTaskSettingsActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIPrintTaskSettingsActivatedEventArgs> for ::winrt_applicationmodel::Activation::IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: WebUIPrintTaskSettingsActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIPrintTaskSettingsActivatedEventArgs> for ::winrt_applicationmodel::Activation::IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &WebUIPrintTaskSettingsActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgs> for WebUIPrintTaskSettingsActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgs> for &WebUIPrintTaskSettingsActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgs> {
        ::core::convert::TryInto::<::winrt_applicationmodel::Activation::IActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIPrintTaskSettingsActivatedEventArgs> for IActivatedEventArgsDeferral {
    type Error = ::windows_core::Error;
    fn try_from(value: WebUIPrintTaskSettingsActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIPrintTaskSettingsActivatedEventArgs> for IActivatedEventArgsDeferral {
    type Error = ::windows_core::Error;
    fn try_from(value: &WebUIPrintTaskSettingsActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgsDeferral> for WebUIPrintTaskSettingsActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgsDeferral> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgsDeferral> for &WebUIPrintTaskSettingsActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgsDeferral> {
        ::core::convert::TryInto::<IActivatedEventArgsDeferral>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIPrintTaskSettingsActivatedEventArgs> for ::winrt_applicationmodel::Activation::IPrintTaskSettingsActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: WebUIPrintTaskSettingsActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIPrintTaskSettingsActivatedEventArgs> for ::winrt_applicationmodel::Activation::IPrintTaskSettingsActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &WebUIPrintTaskSettingsActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IPrintTaskSettingsActivatedEventArgs> for WebUIPrintTaskSettingsActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IPrintTaskSettingsActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IPrintTaskSettingsActivatedEventArgs> for &WebUIPrintTaskSettingsActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IPrintTaskSettingsActivatedEventArgs> {
        ::core::convert::TryInto::<::winrt_applicationmodel::Activation::IPrintTaskSettingsActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
#[repr(transparent)]
pub struct WebUIPrintWorkflowForegroundTaskActivatedEventArgs(::windows_core::IUnknown);
#[cfg(feature = "ApplicationModel_Activation")]
impl WebUIPrintWorkflowForegroundTaskActivatedEventArgs {
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Kind(&self) -> ::windows_core::Result<::winrt_applicationmodel::Activation::ActivationKind> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_applicationmodel::Activation::ActivationKind>::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_applicationmodel::Activation::ActivationKind>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<::winrt_applicationmodel::Activation::ApplicationExecutionState> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_applicationmodel::Activation::ApplicationExecutionState>::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_applicationmodel::Activation::ApplicationExecutionState>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn SplashScreen(&self) -> ::windows_core::Result<::winrt_applicationmodel::Activation::SplashScreen> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_applicationmodel::Activation::SplashScreen>(result__)
        }
    }
    pub fn ActivatedOperation(&self) -> ::windows_core::Result<ActivatedOperation> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgsDeferral>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ActivatedOperation)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivatedOperation>(result__)
        }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::clone::Clone for WebUIPrintWorkflowForegroundTaskActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::cmp::PartialEq for WebUIPrintWorkflowForegroundTaskActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::cmp::Eq for WebUIPrintWorkflowForegroundTaskActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::fmt::Debug for WebUIPrintWorkflowForegroundTaskActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WebUIPrintWorkflowForegroundTaskActivatedEventArgs").field(&self.0).finish()
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows_core::RuntimeType for WebUIPrintWorkflowForegroundTaskActivatedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.WebUI.WebUIPrintWorkflowForegroundTaskActivatedEventArgs;{cf651713-cd08-4fd8-b697-a281b6544e2e})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows_core::Interface for WebUIPrintWorkflowForegroundTaskActivatedEventArgs {
    type Vtable = ::winrt_applicationmodel::Activation::IActivatedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <::winrt_applicationmodel::Activation::IActivatedEventArgs as ::windows_core::Interface>::IID;
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::RuntimeName for WebUIPrintWorkflowForegroundTaskActivatedEventArgs {
    const NAME: &'static str = "Windows.UI.WebUI.WebUIPrintWorkflowForegroundTaskActivatedEventArgs";
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<WebUIPrintWorkflowForegroundTaskActivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: WebUIPrintWorkflowForegroundTaskActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<&WebUIPrintWorkflowForegroundTaskActivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &WebUIPrintWorkflowForegroundTaskActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for WebUIPrintWorkflowForegroundTaskActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a WebUIPrintWorkflowForegroundTaskActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<WebUIPrintWorkflowForegroundTaskActivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: WebUIPrintWorkflowForegroundTaskActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<&WebUIPrintWorkflowForegroundTaskActivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &WebUIPrintWorkflowForegroundTaskActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for WebUIPrintWorkflowForegroundTaskActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a WebUIPrintWorkflowForegroundTaskActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIPrintWorkflowForegroundTaskActivatedEventArgs> for ::winrt_applicationmodel::Activation::IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: WebUIPrintWorkflowForegroundTaskActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIPrintWorkflowForegroundTaskActivatedEventArgs> for ::winrt_applicationmodel::Activation::IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &WebUIPrintWorkflowForegroundTaskActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgs> for WebUIPrintWorkflowForegroundTaskActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgs> for &WebUIPrintWorkflowForegroundTaskActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgs> {
        ::core::convert::TryInto::<::winrt_applicationmodel::Activation::IActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIPrintWorkflowForegroundTaskActivatedEventArgs> for IActivatedEventArgsDeferral {
    type Error = ::windows_core::Error;
    fn try_from(value: WebUIPrintWorkflowForegroundTaskActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIPrintWorkflowForegroundTaskActivatedEventArgs> for IActivatedEventArgsDeferral {
    type Error = ::windows_core::Error;
    fn try_from(value: &WebUIPrintWorkflowForegroundTaskActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgsDeferral> for WebUIPrintWorkflowForegroundTaskActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgsDeferral> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgsDeferral> for &WebUIPrintWorkflowForegroundTaskActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgsDeferral> {
        ::core::convert::TryInto::<IActivatedEventArgsDeferral>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
#[repr(transparent)]
pub struct WebUIProtocolActivatedEventArgs(::windows_core::IUnknown);
#[cfg(feature = "ApplicationModel_Activation")]
impl WebUIProtocolActivatedEventArgs {
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Kind(&self) -> ::windows_core::Result<::winrt_applicationmodel::Activation::ActivationKind> {
        let this = &::windows_core::Interface::cast::<::winrt_applicationmodel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_applicationmodel::Activation::ActivationKind>::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_applicationmodel::Activation::ActivationKind>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<::winrt_applicationmodel::Activation::ApplicationExecutionState> {
        let this = &::windows_core::Interface::cast::<::winrt_applicationmodel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_applicationmodel::Activation::ApplicationExecutionState>::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_applicationmodel::Activation::ApplicationExecutionState>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn SplashScreen(&self) -> ::windows_core::Result<::winrt_applicationmodel::Activation::SplashScreen> {
        let this = &::windows_core::Interface::cast::<::winrt_applicationmodel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_applicationmodel::Activation::SplashScreen>(result__)
        }
    }
    pub fn ActivatedOperation(&self) -> ::windows_core::Result<ActivatedOperation> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgsDeferral>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ActivatedOperation)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivatedOperation>(result__)
        }
    }
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "System"))]
    pub fn User(&self) -> ::windows_core::Result<::winrt_system::User> {
        let this = &::windows_core::Interface::cast::<::winrt_applicationmodel::Activation::IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).User)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_system::User>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn CurrentlyShownApplicationViewId(&self) -> ::windows_core::Result<i32> {
        let this = &::windows_core::Interface::cast::<::winrt_applicationmodel::Activation::IApplicationViewActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows_core::Interface::vtable(this).CurrentlyShownApplicationViewId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Uri(&self) -> ::windows_core::Result<::winrt_foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Uri)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Uri>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn CallerPackageFamilyName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<::winrt_applicationmodel::Activation::IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).CallerPackageFamilyName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "Foundation_Collections"))]
    pub fn Data(&self) -> ::windows_core::Result<::winrt_foundation::Collections::ValueSet> {
        let this = &::windows_core::Interface::cast::<::winrt_applicationmodel::Activation::IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Data)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::ValueSet>(result__)
        }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::clone::Clone for WebUIProtocolActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::cmp::PartialEq for WebUIProtocolActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::cmp::Eq for WebUIProtocolActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::fmt::Debug for WebUIProtocolActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WebUIProtocolActivatedEventArgs").field(&self.0).finish()
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows_core::RuntimeType for WebUIProtocolActivatedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.WebUI.WebUIProtocolActivatedEventArgs;{6095f4dd-b7c0-46ab-81fe-d90f36d00d24})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows_core::Interface for WebUIProtocolActivatedEventArgs {
    type Vtable = ::winrt_applicationmodel::Activation::IProtocolActivatedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <::winrt_applicationmodel::Activation::IProtocolActivatedEventArgs as ::windows_core::Interface>::IID;
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::RuntimeName for WebUIProtocolActivatedEventArgs {
    const NAME: &'static str = "Windows.UI.WebUI.WebUIProtocolActivatedEventArgs";
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<WebUIProtocolActivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: WebUIProtocolActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<&WebUIProtocolActivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &WebUIProtocolActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for WebUIProtocolActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a WebUIProtocolActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<WebUIProtocolActivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: WebUIProtocolActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<&WebUIProtocolActivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &WebUIProtocolActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for WebUIProtocolActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a WebUIProtocolActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIProtocolActivatedEventArgs> for ::winrt_applicationmodel::Activation::IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: WebUIProtocolActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIProtocolActivatedEventArgs> for ::winrt_applicationmodel::Activation::IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &WebUIProtocolActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgs> for WebUIProtocolActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgs> for &WebUIProtocolActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgs> {
        ::core::convert::TryInto::<::winrt_applicationmodel::Activation::IActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIProtocolActivatedEventArgs> for IActivatedEventArgsDeferral {
    type Error = ::windows_core::Error;
    fn try_from(value: WebUIProtocolActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIProtocolActivatedEventArgs> for IActivatedEventArgsDeferral {
    type Error = ::windows_core::Error;
    fn try_from(value: &WebUIProtocolActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgsDeferral> for WebUIProtocolActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgsDeferral> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgsDeferral> for &WebUIProtocolActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgsDeferral> {
        ::core::convert::TryInto::<IActivatedEventArgsDeferral>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIProtocolActivatedEventArgs> for ::winrt_applicationmodel::Activation::IActivatedEventArgsWithUser {
    type Error = ::windows_core::Error;
    fn try_from(value: WebUIProtocolActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIProtocolActivatedEventArgs> for ::winrt_applicationmodel::Activation::IActivatedEventArgsWithUser {
    type Error = ::windows_core::Error;
    fn try_from(value: &WebUIProtocolActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgsWithUser> for WebUIProtocolActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgsWithUser> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgsWithUser> for &WebUIProtocolActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgsWithUser> {
        ::core::convert::TryInto::<::winrt_applicationmodel::Activation::IActivatedEventArgsWithUser>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIProtocolActivatedEventArgs> for ::winrt_applicationmodel::Activation::IApplicationViewActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: WebUIProtocolActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIProtocolActivatedEventArgs> for ::winrt_applicationmodel::Activation::IApplicationViewActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &WebUIProtocolActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IApplicationViewActivatedEventArgs> for WebUIProtocolActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IApplicationViewActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IApplicationViewActivatedEventArgs> for &WebUIProtocolActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IApplicationViewActivatedEventArgs> {
        ::core::convert::TryInto::<::winrt_applicationmodel::Activation::IApplicationViewActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIProtocolActivatedEventArgs> for ::winrt_applicationmodel::Activation::IProtocolActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: WebUIProtocolActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIProtocolActivatedEventArgs> for ::winrt_applicationmodel::Activation::IProtocolActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &WebUIProtocolActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IProtocolActivatedEventArgs> for WebUIProtocolActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IProtocolActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IProtocolActivatedEventArgs> for &WebUIProtocolActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IProtocolActivatedEventArgs> {
        ::core::convert::TryInto::<::winrt_applicationmodel::Activation::IProtocolActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIProtocolActivatedEventArgs> for ::winrt_applicationmodel::Activation::IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData {
    type Error = ::windows_core::Error;
    fn try_from(value: WebUIProtocolActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIProtocolActivatedEventArgs> for ::winrt_applicationmodel::Activation::IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData {
    type Error = ::windows_core::Error;
    fn try_from(value: &WebUIProtocolActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData> for WebUIProtocolActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData> for &WebUIProtocolActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData> {
        ::core::convert::TryInto::<::winrt_applicationmodel::Activation::IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
#[repr(transparent)]
pub struct WebUIProtocolForResultsActivatedEventArgs(::windows_core::IUnknown);
#[cfg(feature = "ApplicationModel_Activation")]
impl WebUIProtocolForResultsActivatedEventArgs {
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Kind(&self) -> ::windows_core::Result<::winrt_applicationmodel::Activation::ActivationKind> {
        let this = &::windows_core::Interface::cast::<::winrt_applicationmodel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_applicationmodel::Activation::ActivationKind>::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_applicationmodel::Activation::ActivationKind>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<::winrt_applicationmodel::Activation::ApplicationExecutionState> {
        let this = &::windows_core::Interface::cast::<::winrt_applicationmodel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_applicationmodel::Activation::ApplicationExecutionState>::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_applicationmodel::Activation::ApplicationExecutionState>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn SplashScreen(&self) -> ::windows_core::Result<::winrt_applicationmodel::Activation::SplashScreen> {
        let this = &::windows_core::Interface::cast::<::winrt_applicationmodel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_applicationmodel::Activation::SplashScreen>(result__)
        }
    }
    pub fn ActivatedOperation(&self) -> ::windows_core::Result<ActivatedOperation> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgsDeferral>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ActivatedOperation)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivatedOperation>(result__)
        }
    }
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "System"))]
    pub fn User(&self) -> ::windows_core::Result<::winrt_system::User> {
        let this = &::windows_core::Interface::cast::<::winrt_applicationmodel::Activation::IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).User)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_system::User>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn CurrentlyShownApplicationViewId(&self) -> ::windows_core::Result<i32> {
        let this = &::windows_core::Interface::cast::<::winrt_applicationmodel::Activation::IApplicationViewActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows_core::Interface::vtable(this).CurrentlyShownApplicationViewId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Uri(&self) -> ::windows_core::Result<::winrt_foundation::Uri> {
        let this = &::windows_core::Interface::cast::<::winrt_applicationmodel::Activation::IProtocolActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Uri)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Uri>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn CallerPackageFamilyName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<::winrt_applicationmodel::Activation::IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).CallerPackageFamilyName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "Foundation_Collections"))]
    pub fn Data(&self) -> ::windows_core::Result<::winrt_foundation::Collections::ValueSet> {
        let this = &::windows_core::Interface::cast::<::winrt_applicationmodel::Activation::IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Data)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::ValueSet>(result__)
        }
    }
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "System"))]
    pub fn ProtocolForResultsOperation(&self) -> ::windows_core::Result<::winrt_system::ProtocolForResultsOperation> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ProtocolForResultsOperation)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_system::ProtocolForResultsOperation>(result__)
        }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::clone::Clone for WebUIProtocolForResultsActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::cmp::PartialEq for WebUIProtocolForResultsActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::cmp::Eq for WebUIProtocolForResultsActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::fmt::Debug for WebUIProtocolForResultsActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WebUIProtocolForResultsActivatedEventArgs").field(&self.0).finish()
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows_core::RuntimeType for WebUIProtocolForResultsActivatedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.WebUI.WebUIProtocolForResultsActivatedEventArgs;{e75132c2-7ae7-4517-80ac-dbe8d7cc5b9c})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows_core::Interface for WebUIProtocolForResultsActivatedEventArgs {
    type Vtable = ::winrt_applicationmodel::Activation::IProtocolForResultsActivatedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <::winrt_applicationmodel::Activation::IProtocolForResultsActivatedEventArgs as ::windows_core::Interface>::IID;
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::RuntimeName for WebUIProtocolForResultsActivatedEventArgs {
    const NAME: &'static str = "Windows.UI.WebUI.WebUIProtocolForResultsActivatedEventArgs";
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<WebUIProtocolForResultsActivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: WebUIProtocolForResultsActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<&WebUIProtocolForResultsActivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &WebUIProtocolForResultsActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for WebUIProtocolForResultsActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a WebUIProtocolForResultsActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<WebUIProtocolForResultsActivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: WebUIProtocolForResultsActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<&WebUIProtocolForResultsActivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &WebUIProtocolForResultsActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for WebUIProtocolForResultsActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a WebUIProtocolForResultsActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIProtocolForResultsActivatedEventArgs> for ::winrt_applicationmodel::Activation::IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: WebUIProtocolForResultsActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIProtocolForResultsActivatedEventArgs> for ::winrt_applicationmodel::Activation::IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &WebUIProtocolForResultsActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgs> for WebUIProtocolForResultsActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgs> for &WebUIProtocolForResultsActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgs> {
        ::core::convert::TryInto::<::winrt_applicationmodel::Activation::IActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIProtocolForResultsActivatedEventArgs> for IActivatedEventArgsDeferral {
    type Error = ::windows_core::Error;
    fn try_from(value: WebUIProtocolForResultsActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIProtocolForResultsActivatedEventArgs> for IActivatedEventArgsDeferral {
    type Error = ::windows_core::Error;
    fn try_from(value: &WebUIProtocolForResultsActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgsDeferral> for WebUIProtocolForResultsActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgsDeferral> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgsDeferral> for &WebUIProtocolForResultsActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgsDeferral> {
        ::core::convert::TryInto::<IActivatedEventArgsDeferral>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIProtocolForResultsActivatedEventArgs> for ::winrt_applicationmodel::Activation::IActivatedEventArgsWithUser {
    type Error = ::windows_core::Error;
    fn try_from(value: WebUIProtocolForResultsActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIProtocolForResultsActivatedEventArgs> for ::winrt_applicationmodel::Activation::IActivatedEventArgsWithUser {
    type Error = ::windows_core::Error;
    fn try_from(value: &WebUIProtocolForResultsActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgsWithUser> for WebUIProtocolForResultsActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgsWithUser> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgsWithUser> for &WebUIProtocolForResultsActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgsWithUser> {
        ::core::convert::TryInto::<::winrt_applicationmodel::Activation::IActivatedEventArgsWithUser>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIProtocolForResultsActivatedEventArgs> for ::winrt_applicationmodel::Activation::IApplicationViewActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: WebUIProtocolForResultsActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIProtocolForResultsActivatedEventArgs> for ::winrt_applicationmodel::Activation::IApplicationViewActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &WebUIProtocolForResultsActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IApplicationViewActivatedEventArgs> for WebUIProtocolForResultsActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IApplicationViewActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IApplicationViewActivatedEventArgs> for &WebUIProtocolForResultsActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IApplicationViewActivatedEventArgs> {
        ::core::convert::TryInto::<::winrt_applicationmodel::Activation::IApplicationViewActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIProtocolForResultsActivatedEventArgs> for ::winrt_applicationmodel::Activation::IProtocolActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: WebUIProtocolForResultsActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIProtocolForResultsActivatedEventArgs> for ::winrt_applicationmodel::Activation::IProtocolActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &WebUIProtocolForResultsActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IProtocolActivatedEventArgs> for WebUIProtocolForResultsActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IProtocolActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IProtocolActivatedEventArgs> for &WebUIProtocolForResultsActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IProtocolActivatedEventArgs> {
        ::core::convert::TryInto::<::winrt_applicationmodel::Activation::IProtocolActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIProtocolForResultsActivatedEventArgs> for ::winrt_applicationmodel::Activation::IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData {
    type Error = ::windows_core::Error;
    fn try_from(value: WebUIProtocolForResultsActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIProtocolForResultsActivatedEventArgs> for ::winrt_applicationmodel::Activation::IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData {
    type Error = ::windows_core::Error;
    fn try_from(value: &WebUIProtocolForResultsActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData> for WebUIProtocolForResultsActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData> for &WebUIProtocolForResultsActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData> {
        ::core::convert::TryInto::<::winrt_applicationmodel::Activation::IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIProtocolForResultsActivatedEventArgs> for ::winrt_applicationmodel::Activation::IProtocolForResultsActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: WebUIProtocolForResultsActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIProtocolForResultsActivatedEventArgs> for ::winrt_applicationmodel::Activation::IProtocolForResultsActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &WebUIProtocolForResultsActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IProtocolForResultsActivatedEventArgs> for WebUIProtocolForResultsActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IProtocolForResultsActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IProtocolForResultsActivatedEventArgs> for &WebUIProtocolForResultsActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IProtocolForResultsActivatedEventArgs> {
        ::core::convert::TryInto::<::winrt_applicationmodel::Activation::IProtocolForResultsActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
#[repr(transparent)]
pub struct WebUIRestrictedLaunchActivatedEventArgs(::windows_core::IUnknown);
#[cfg(feature = "ApplicationModel_Activation")]
impl WebUIRestrictedLaunchActivatedEventArgs {
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Kind(&self) -> ::windows_core::Result<::winrt_applicationmodel::Activation::ActivationKind> {
        let this = &::windows_core::Interface::cast::<::winrt_applicationmodel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_applicationmodel::Activation::ActivationKind>::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_applicationmodel::Activation::ActivationKind>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<::winrt_applicationmodel::Activation::ApplicationExecutionState> {
        let this = &::windows_core::Interface::cast::<::winrt_applicationmodel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_applicationmodel::Activation::ApplicationExecutionState>::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_applicationmodel::Activation::ApplicationExecutionState>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn SplashScreen(&self) -> ::windows_core::Result<::winrt_applicationmodel::Activation::SplashScreen> {
        let this = &::windows_core::Interface::cast::<::winrt_applicationmodel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_applicationmodel::Activation::SplashScreen>(result__)
        }
    }
    pub fn ActivatedOperation(&self) -> ::windows_core::Result<ActivatedOperation> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgsDeferral>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ActivatedOperation)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivatedOperation>(result__)
        }
    }
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "System"))]
    pub fn User(&self) -> ::windows_core::Result<::winrt_system::User> {
        let this = &::windows_core::Interface::cast::<::winrt_applicationmodel::Activation::IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).User)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_system::User>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn SharedContext(&self) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows_core::Interface::vtable(this).SharedContext)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::IInspectable>(result__)
        }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::clone::Clone for WebUIRestrictedLaunchActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::cmp::PartialEq for WebUIRestrictedLaunchActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::cmp::Eq for WebUIRestrictedLaunchActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::fmt::Debug for WebUIRestrictedLaunchActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WebUIRestrictedLaunchActivatedEventArgs").field(&self.0).finish()
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows_core::RuntimeType for WebUIRestrictedLaunchActivatedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.WebUI.WebUIRestrictedLaunchActivatedEventArgs;{e0b7ac81-bfc3-4344-a5da-19fd5a27baae})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows_core::Interface for WebUIRestrictedLaunchActivatedEventArgs {
    type Vtable = ::winrt_applicationmodel::Activation::IRestrictedLaunchActivatedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <::winrt_applicationmodel::Activation::IRestrictedLaunchActivatedEventArgs as ::windows_core::Interface>::IID;
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::RuntimeName for WebUIRestrictedLaunchActivatedEventArgs {
    const NAME: &'static str = "Windows.UI.WebUI.WebUIRestrictedLaunchActivatedEventArgs";
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<WebUIRestrictedLaunchActivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: WebUIRestrictedLaunchActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<&WebUIRestrictedLaunchActivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &WebUIRestrictedLaunchActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for WebUIRestrictedLaunchActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a WebUIRestrictedLaunchActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<WebUIRestrictedLaunchActivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: WebUIRestrictedLaunchActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<&WebUIRestrictedLaunchActivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &WebUIRestrictedLaunchActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for WebUIRestrictedLaunchActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a WebUIRestrictedLaunchActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIRestrictedLaunchActivatedEventArgs> for ::winrt_applicationmodel::Activation::IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: WebUIRestrictedLaunchActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIRestrictedLaunchActivatedEventArgs> for ::winrt_applicationmodel::Activation::IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &WebUIRestrictedLaunchActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgs> for WebUIRestrictedLaunchActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgs> for &WebUIRestrictedLaunchActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgs> {
        ::core::convert::TryInto::<::winrt_applicationmodel::Activation::IActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIRestrictedLaunchActivatedEventArgs> for IActivatedEventArgsDeferral {
    type Error = ::windows_core::Error;
    fn try_from(value: WebUIRestrictedLaunchActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIRestrictedLaunchActivatedEventArgs> for IActivatedEventArgsDeferral {
    type Error = ::windows_core::Error;
    fn try_from(value: &WebUIRestrictedLaunchActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgsDeferral> for WebUIRestrictedLaunchActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgsDeferral> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgsDeferral> for &WebUIRestrictedLaunchActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgsDeferral> {
        ::core::convert::TryInto::<IActivatedEventArgsDeferral>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIRestrictedLaunchActivatedEventArgs> for ::winrt_applicationmodel::Activation::IActivatedEventArgsWithUser {
    type Error = ::windows_core::Error;
    fn try_from(value: WebUIRestrictedLaunchActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIRestrictedLaunchActivatedEventArgs> for ::winrt_applicationmodel::Activation::IActivatedEventArgsWithUser {
    type Error = ::windows_core::Error;
    fn try_from(value: &WebUIRestrictedLaunchActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgsWithUser> for WebUIRestrictedLaunchActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgsWithUser> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgsWithUser> for &WebUIRestrictedLaunchActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgsWithUser> {
        ::core::convert::TryInto::<::winrt_applicationmodel::Activation::IActivatedEventArgsWithUser>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIRestrictedLaunchActivatedEventArgs> for ::winrt_applicationmodel::Activation::IRestrictedLaunchActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: WebUIRestrictedLaunchActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIRestrictedLaunchActivatedEventArgs> for ::winrt_applicationmodel::Activation::IRestrictedLaunchActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &WebUIRestrictedLaunchActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IRestrictedLaunchActivatedEventArgs> for WebUIRestrictedLaunchActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IRestrictedLaunchActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IRestrictedLaunchActivatedEventArgs> for &WebUIRestrictedLaunchActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IRestrictedLaunchActivatedEventArgs> {
        ::core::convert::TryInto::<::winrt_applicationmodel::Activation::IRestrictedLaunchActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
#[repr(transparent)]
pub struct WebUISearchActivatedEventArgs(::windows_core::IUnknown);
#[cfg(feature = "ApplicationModel_Activation")]
impl WebUISearchActivatedEventArgs {
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Kind(&self) -> ::windows_core::Result<::winrt_applicationmodel::Activation::ActivationKind> {
        let this = &::windows_core::Interface::cast::<::winrt_applicationmodel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_applicationmodel::Activation::ActivationKind>::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_applicationmodel::Activation::ActivationKind>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<::winrt_applicationmodel::Activation::ApplicationExecutionState> {
        let this = &::windows_core::Interface::cast::<::winrt_applicationmodel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_applicationmodel::Activation::ApplicationExecutionState>::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_applicationmodel::Activation::ApplicationExecutionState>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn SplashScreen(&self) -> ::windows_core::Result<::winrt_applicationmodel::Activation::SplashScreen> {
        let this = &::windows_core::Interface::cast::<::winrt_applicationmodel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_applicationmodel::Activation::SplashScreen>(result__)
        }
    }
    pub fn ActivatedOperation(&self) -> ::windows_core::Result<ActivatedOperation> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgsDeferral>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ActivatedOperation)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivatedOperation>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn CurrentlyShownApplicationViewId(&self) -> ::windows_core::Result<i32> {
        let this = &::windows_core::Interface::cast::<::winrt_applicationmodel::Activation::IApplicationViewActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows_core::Interface::vtable(this).CurrentlyShownApplicationViewId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn QueryText(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).QueryText)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Language(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Language)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "ApplicationModel_Search"))]
    pub fn LinguisticDetails(&self) -> ::windows_core::Result<::winrt_applicationmodel::Search::SearchPaneQueryLinguisticDetails> {
        let this = &::windows_core::Interface::cast::<::winrt_applicationmodel::Activation::ISearchActivatedEventArgsWithLinguisticDetails>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).LinguisticDetails)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_applicationmodel::Search::SearchPaneQueryLinguisticDetails>(result__)
        }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::clone::Clone for WebUISearchActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::cmp::PartialEq for WebUISearchActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::cmp::Eq for WebUISearchActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::fmt::Debug for WebUISearchActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WebUISearchActivatedEventArgs").field(&self.0).finish()
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows_core::RuntimeType for WebUISearchActivatedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.WebUI.WebUISearchActivatedEventArgs;{8cb36951-58c8-43e3-94bc-41d33f8b630e})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows_core::Interface for WebUISearchActivatedEventArgs {
    type Vtable = ::winrt_applicationmodel::Activation::ISearchActivatedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <::winrt_applicationmodel::Activation::ISearchActivatedEventArgs as ::windows_core::Interface>::IID;
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::RuntimeName for WebUISearchActivatedEventArgs {
    const NAME: &'static str = "Windows.UI.WebUI.WebUISearchActivatedEventArgs";
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<WebUISearchActivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: WebUISearchActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<&WebUISearchActivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &WebUISearchActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for WebUISearchActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a WebUISearchActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<WebUISearchActivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: WebUISearchActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<&WebUISearchActivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &WebUISearchActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for WebUISearchActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a WebUISearchActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUISearchActivatedEventArgs> for ::winrt_applicationmodel::Activation::IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: WebUISearchActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUISearchActivatedEventArgs> for ::winrt_applicationmodel::Activation::IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &WebUISearchActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgs> for WebUISearchActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgs> for &WebUISearchActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgs> {
        ::core::convert::TryInto::<::winrt_applicationmodel::Activation::IActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUISearchActivatedEventArgs> for IActivatedEventArgsDeferral {
    type Error = ::windows_core::Error;
    fn try_from(value: WebUISearchActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUISearchActivatedEventArgs> for IActivatedEventArgsDeferral {
    type Error = ::windows_core::Error;
    fn try_from(value: &WebUISearchActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgsDeferral> for WebUISearchActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgsDeferral> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgsDeferral> for &WebUISearchActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgsDeferral> {
        ::core::convert::TryInto::<IActivatedEventArgsDeferral>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUISearchActivatedEventArgs> for ::winrt_applicationmodel::Activation::IApplicationViewActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: WebUISearchActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUISearchActivatedEventArgs> for ::winrt_applicationmodel::Activation::IApplicationViewActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &WebUISearchActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IApplicationViewActivatedEventArgs> for WebUISearchActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IApplicationViewActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IApplicationViewActivatedEventArgs> for &WebUISearchActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IApplicationViewActivatedEventArgs> {
        ::core::convert::TryInto::<::winrt_applicationmodel::Activation::IApplicationViewActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUISearchActivatedEventArgs> for ::winrt_applicationmodel::Activation::ISearchActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: WebUISearchActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUISearchActivatedEventArgs> for ::winrt_applicationmodel::Activation::ISearchActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &WebUISearchActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::ISearchActivatedEventArgs> for WebUISearchActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::ISearchActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::ISearchActivatedEventArgs> for &WebUISearchActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::ISearchActivatedEventArgs> {
        ::core::convert::TryInto::<::winrt_applicationmodel::Activation::ISearchActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUISearchActivatedEventArgs> for ::winrt_applicationmodel::Activation::ISearchActivatedEventArgsWithLinguisticDetails {
    type Error = ::windows_core::Error;
    fn try_from(value: WebUISearchActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUISearchActivatedEventArgs> for ::winrt_applicationmodel::Activation::ISearchActivatedEventArgsWithLinguisticDetails {
    type Error = ::windows_core::Error;
    fn try_from(value: &WebUISearchActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::ISearchActivatedEventArgsWithLinguisticDetails> for WebUISearchActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::ISearchActivatedEventArgsWithLinguisticDetails> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::ISearchActivatedEventArgsWithLinguisticDetails> for &WebUISearchActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::ISearchActivatedEventArgsWithLinguisticDetails> {
        ::core::convert::TryInto::<::winrt_applicationmodel::Activation::ISearchActivatedEventArgsWithLinguisticDetails>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
#[repr(transparent)]
pub struct WebUIShareTargetActivatedEventArgs(::windows_core::IUnknown);
#[cfg(feature = "ApplicationModel_Activation")]
impl WebUIShareTargetActivatedEventArgs {
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Kind(&self) -> ::windows_core::Result<::winrt_applicationmodel::Activation::ActivationKind> {
        let this = &::windows_core::Interface::cast::<::winrt_applicationmodel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_applicationmodel::Activation::ActivationKind>::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_applicationmodel::Activation::ActivationKind>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<::winrt_applicationmodel::Activation::ApplicationExecutionState> {
        let this = &::windows_core::Interface::cast::<::winrt_applicationmodel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_applicationmodel::Activation::ApplicationExecutionState>::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_applicationmodel::Activation::ApplicationExecutionState>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn SplashScreen(&self) -> ::windows_core::Result<::winrt_applicationmodel::Activation::SplashScreen> {
        let this = &::windows_core::Interface::cast::<::winrt_applicationmodel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_applicationmodel::Activation::SplashScreen>(result__)
        }
    }
    pub fn ActivatedOperation(&self) -> ::windows_core::Result<ActivatedOperation> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgsDeferral>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ActivatedOperation)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivatedOperation>(result__)
        }
    }
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "System"))]
    pub fn User(&self) -> ::windows_core::Result<::winrt_system::User> {
        let this = &::windows_core::Interface::cast::<::winrt_applicationmodel::Activation::IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).User)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_system::User>(result__)
        }
    }
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "ApplicationModel_DataTransfer_ShareTarget"))]
    pub fn ShareOperation(&self) -> ::windows_core::Result<::winrt_applicationmodel::DataTransfer::ShareTarget::ShareOperation> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ShareOperation)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_applicationmodel::DataTransfer::ShareTarget::ShareOperation>(result__)
        }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::clone::Clone for WebUIShareTargetActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::cmp::PartialEq for WebUIShareTargetActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::cmp::Eq for WebUIShareTargetActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::fmt::Debug for WebUIShareTargetActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WebUIShareTargetActivatedEventArgs").field(&self.0).finish()
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows_core::RuntimeType for WebUIShareTargetActivatedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.WebUI.WebUIShareTargetActivatedEventArgs;{4bdaf9c8-cdb2-4acb-bfc3-6648563378ec})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows_core::Interface for WebUIShareTargetActivatedEventArgs {
    type Vtable = ::winrt_applicationmodel::Activation::IShareTargetActivatedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <::winrt_applicationmodel::Activation::IShareTargetActivatedEventArgs as ::windows_core::Interface>::IID;
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::RuntimeName for WebUIShareTargetActivatedEventArgs {
    const NAME: &'static str = "Windows.UI.WebUI.WebUIShareTargetActivatedEventArgs";
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<WebUIShareTargetActivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: WebUIShareTargetActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<&WebUIShareTargetActivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &WebUIShareTargetActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for WebUIShareTargetActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a WebUIShareTargetActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<WebUIShareTargetActivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: WebUIShareTargetActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<&WebUIShareTargetActivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &WebUIShareTargetActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for WebUIShareTargetActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a WebUIShareTargetActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIShareTargetActivatedEventArgs> for ::winrt_applicationmodel::Activation::IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: WebUIShareTargetActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIShareTargetActivatedEventArgs> for ::winrt_applicationmodel::Activation::IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &WebUIShareTargetActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgs> for WebUIShareTargetActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgs> for &WebUIShareTargetActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgs> {
        ::core::convert::TryInto::<::winrt_applicationmodel::Activation::IActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIShareTargetActivatedEventArgs> for IActivatedEventArgsDeferral {
    type Error = ::windows_core::Error;
    fn try_from(value: WebUIShareTargetActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIShareTargetActivatedEventArgs> for IActivatedEventArgsDeferral {
    type Error = ::windows_core::Error;
    fn try_from(value: &WebUIShareTargetActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgsDeferral> for WebUIShareTargetActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgsDeferral> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgsDeferral> for &WebUIShareTargetActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgsDeferral> {
        ::core::convert::TryInto::<IActivatedEventArgsDeferral>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIShareTargetActivatedEventArgs> for ::winrt_applicationmodel::Activation::IActivatedEventArgsWithUser {
    type Error = ::windows_core::Error;
    fn try_from(value: WebUIShareTargetActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIShareTargetActivatedEventArgs> for ::winrt_applicationmodel::Activation::IActivatedEventArgsWithUser {
    type Error = ::windows_core::Error;
    fn try_from(value: &WebUIShareTargetActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgsWithUser> for WebUIShareTargetActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgsWithUser> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgsWithUser> for &WebUIShareTargetActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgsWithUser> {
        ::core::convert::TryInto::<::winrt_applicationmodel::Activation::IActivatedEventArgsWithUser>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIShareTargetActivatedEventArgs> for ::winrt_applicationmodel::Activation::IShareTargetActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: WebUIShareTargetActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIShareTargetActivatedEventArgs> for ::winrt_applicationmodel::Activation::IShareTargetActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &WebUIShareTargetActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IShareTargetActivatedEventArgs> for WebUIShareTargetActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IShareTargetActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IShareTargetActivatedEventArgs> for &WebUIShareTargetActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IShareTargetActivatedEventArgs> {
        ::core::convert::TryInto::<::winrt_applicationmodel::Activation::IShareTargetActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
#[repr(transparent)]
pub struct WebUIStartupTaskActivatedEventArgs(::windows_core::IUnknown);
#[cfg(feature = "ApplicationModel_Activation")]
impl WebUIStartupTaskActivatedEventArgs {
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Kind(&self) -> ::windows_core::Result<::winrt_applicationmodel::Activation::ActivationKind> {
        let this = &::windows_core::Interface::cast::<::winrt_applicationmodel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_applicationmodel::Activation::ActivationKind>::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_applicationmodel::Activation::ActivationKind>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<::winrt_applicationmodel::Activation::ApplicationExecutionState> {
        let this = &::windows_core::Interface::cast::<::winrt_applicationmodel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_applicationmodel::Activation::ApplicationExecutionState>::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_applicationmodel::Activation::ApplicationExecutionState>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn SplashScreen(&self) -> ::windows_core::Result<::winrt_applicationmodel::Activation::SplashScreen> {
        let this = &::windows_core::Interface::cast::<::winrt_applicationmodel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_applicationmodel::Activation::SplashScreen>(result__)
        }
    }
    pub fn ActivatedOperation(&self) -> ::windows_core::Result<ActivatedOperation> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgsDeferral>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ActivatedOperation)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivatedOperation>(result__)
        }
    }
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "System"))]
    pub fn User(&self) -> ::windows_core::Result<::winrt_system::User> {
        let this = &::windows_core::Interface::cast::<::winrt_applicationmodel::Activation::IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).User)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_system::User>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn TaskId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).TaskId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::clone::Clone for WebUIStartupTaskActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::cmp::PartialEq for WebUIStartupTaskActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::cmp::Eq for WebUIStartupTaskActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::fmt::Debug for WebUIStartupTaskActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WebUIStartupTaskActivatedEventArgs").field(&self.0).finish()
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows_core::RuntimeType for WebUIStartupTaskActivatedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.WebUI.WebUIStartupTaskActivatedEventArgs;{03b11a58-5276-4d91-8621-54611864d5fa})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows_core::Interface for WebUIStartupTaskActivatedEventArgs {
    type Vtable = ::winrt_applicationmodel::Activation::IStartupTaskActivatedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <::winrt_applicationmodel::Activation::IStartupTaskActivatedEventArgs as ::windows_core::Interface>::IID;
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::RuntimeName for WebUIStartupTaskActivatedEventArgs {
    const NAME: &'static str = "Windows.UI.WebUI.WebUIStartupTaskActivatedEventArgs";
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<WebUIStartupTaskActivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: WebUIStartupTaskActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<&WebUIStartupTaskActivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &WebUIStartupTaskActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for WebUIStartupTaskActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a WebUIStartupTaskActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<WebUIStartupTaskActivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: WebUIStartupTaskActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<&WebUIStartupTaskActivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &WebUIStartupTaskActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for WebUIStartupTaskActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a WebUIStartupTaskActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIStartupTaskActivatedEventArgs> for ::winrt_applicationmodel::Activation::IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: WebUIStartupTaskActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIStartupTaskActivatedEventArgs> for ::winrt_applicationmodel::Activation::IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &WebUIStartupTaskActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgs> for WebUIStartupTaskActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgs> for &WebUIStartupTaskActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgs> {
        ::core::convert::TryInto::<::winrt_applicationmodel::Activation::IActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIStartupTaskActivatedEventArgs> for IActivatedEventArgsDeferral {
    type Error = ::windows_core::Error;
    fn try_from(value: WebUIStartupTaskActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIStartupTaskActivatedEventArgs> for IActivatedEventArgsDeferral {
    type Error = ::windows_core::Error;
    fn try_from(value: &WebUIStartupTaskActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgsDeferral> for WebUIStartupTaskActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgsDeferral> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgsDeferral> for &WebUIStartupTaskActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgsDeferral> {
        ::core::convert::TryInto::<IActivatedEventArgsDeferral>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIStartupTaskActivatedEventArgs> for ::winrt_applicationmodel::Activation::IActivatedEventArgsWithUser {
    type Error = ::windows_core::Error;
    fn try_from(value: WebUIStartupTaskActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIStartupTaskActivatedEventArgs> for ::winrt_applicationmodel::Activation::IActivatedEventArgsWithUser {
    type Error = ::windows_core::Error;
    fn try_from(value: &WebUIStartupTaskActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgsWithUser> for WebUIStartupTaskActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgsWithUser> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgsWithUser> for &WebUIStartupTaskActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgsWithUser> {
        ::core::convert::TryInto::<::winrt_applicationmodel::Activation::IActivatedEventArgsWithUser>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIStartupTaskActivatedEventArgs> for ::winrt_applicationmodel::Activation::IStartupTaskActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: WebUIStartupTaskActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIStartupTaskActivatedEventArgs> for ::winrt_applicationmodel::Activation::IStartupTaskActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &WebUIStartupTaskActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IStartupTaskActivatedEventArgs> for WebUIStartupTaskActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IStartupTaskActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IStartupTaskActivatedEventArgs> for &WebUIStartupTaskActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IStartupTaskActivatedEventArgs> {
        ::core::convert::TryInto::<::winrt_applicationmodel::Activation::IStartupTaskActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::core::marker::Send for WebUIStartupTaskActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::core::marker::Sync for WebUIStartupTaskActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
#[repr(transparent)]
pub struct WebUIToastNotificationActivatedEventArgs(::windows_core::IUnknown);
#[cfg(feature = "ApplicationModel_Activation")]
impl WebUIToastNotificationActivatedEventArgs {
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Kind(&self) -> ::windows_core::Result<::winrt_applicationmodel::Activation::ActivationKind> {
        let this = &::windows_core::Interface::cast::<::winrt_applicationmodel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_applicationmodel::Activation::ActivationKind>::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_applicationmodel::Activation::ActivationKind>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<::winrt_applicationmodel::Activation::ApplicationExecutionState> {
        let this = &::windows_core::Interface::cast::<::winrt_applicationmodel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_applicationmodel::Activation::ApplicationExecutionState>::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_applicationmodel::Activation::ApplicationExecutionState>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn SplashScreen(&self) -> ::windows_core::Result<::winrt_applicationmodel::Activation::SplashScreen> {
        let this = &::windows_core::Interface::cast::<::winrt_applicationmodel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_applicationmodel::Activation::SplashScreen>(result__)
        }
    }
    pub fn ActivatedOperation(&self) -> ::windows_core::Result<ActivatedOperation> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgsDeferral>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ActivatedOperation)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivatedOperation>(result__)
        }
    }
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "System"))]
    pub fn User(&self) -> ::windows_core::Result<::winrt_system::User> {
        let this = &::windows_core::Interface::cast::<::winrt_applicationmodel::Activation::IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).User)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_system::User>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Argument(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Argument)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "Foundation_Collections"))]
    pub fn UserInput(&self) -> ::windows_core::Result<::winrt_foundation::Collections::ValueSet> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).UserInput)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::ValueSet>(result__)
        }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::clone::Clone for WebUIToastNotificationActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::cmp::PartialEq for WebUIToastNotificationActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::cmp::Eq for WebUIToastNotificationActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::fmt::Debug for WebUIToastNotificationActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WebUIToastNotificationActivatedEventArgs").field(&self.0).finish()
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows_core::RuntimeType for WebUIToastNotificationActivatedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.WebUI.WebUIToastNotificationActivatedEventArgs;{92a86f82-5290-431d-be85-c4aaeeb8685f})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows_core::Interface for WebUIToastNotificationActivatedEventArgs {
    type Vtable = ::winrt_applicationmodel::Activation::IToastNotificationActivatedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <::winrt_applicationmodel::Activation::IToastNotificationActivatedEventArgs as ::windows_core::Interface>::IID;
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::RuntimeName for WebUIToastNotificationActivatedEventArgs {
    const NAME: &'static str = "Windows.UI.WebUI.WebUIToastNotificationActivatedEventArgs";
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<WebUIToastNotificationActivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: WebUIToastNotificationActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<&WebUIToastNotificationActivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &WebUIToastNotificationActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for WebUIToastNotificationActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a WebUIToastNotificationActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<WebUIToastNotificationActivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: WebUIToastNotificationActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<&WebUIToastNotificationActivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &WebUIToastNotificationActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for WebUIToastNotificationActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a WebUIToastNotificationActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIToastNotificationActivatedEventArgs> for ::winrt_applicationmodel::Activation::IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: WebUIToastNotificationActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIToastNotificationActivatedEventArgs> for ::winrt_applicationmodel::Activation::IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &WebUIToastNotificationActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgs> for WebUIToastNotificationActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgs> for &WebUIToastNotificationActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgs> {
        ::core::convert::TryInto::<::winrt_applicationmodel::Activation::IActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIToastNotificationActivatedEventArgs> for IActivatedEventArgsDeferral {
    type Error = ::windows_core::Error;
    fn try_from(value: WebUIToastNotificationActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIToastNotificationActivatedEventArgs> for IActivatedEventArgsDeferral {
    type Error = ::windows_core::Error;
    fn try_from(value: &WebUIToastNotificationActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgsDeferral> for WebUIToastNotificationActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgsDeferral> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgsDeferral> for &WebUIToastNotificationActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgsDeferral> {
        ::core::convert::TryInto::<IActivatedEventArgsDeferral>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIToastNotificationActivatedEventArgs> for ::winrt_applicationmodel::Activation::IActivatedEventArgsWithUser {
    type Error = ::windows_core::Error;
    fn try_from(value: WebUIToastNotificationActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIToastNotificationActivatedEventArgs> for ::winrt_applicationmodel::Activation::IActivatedEventArgsWithUser {
    type Error = ::windows_core::Error;
    fn try_from(value: &WebUIToastNotificationActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgsWithUser> for WebUIToastNotificationActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgsWithUser> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgsWithUser> for &WebUIToastNotificationActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgsWithUser> {
        ::core::convert::TryInto::<::winrt_applicationmodel::Activation::IActivatedEventArgsWithUser>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIToastNotificationActivatedEventArgs> for ::winrt_applicationmodel::Activation::IToastNotificationActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: WebUIToastNotificationActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIToastNotificationActivatedEventArgs> for ::winrt_applicationmodel::Activation::IToastNotificationActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &WebUIToastNotificationActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IToastNotificationActivatedEventArgs> for WebUIToastNotificationActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IToastNotificationActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IToastNotificationActivatedEventArgs> for &WebUIToastNotificationActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IToastNotificationActivatedEventArgs> {
        ::core::convert::TryInto::<::winrt_applicationmodel::Activation::IToastNotificationActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
#[repr(transparent)]
pub struct WebUIUserDataAccountProviderActivatedEventArgs(::windows_core::IUnknown);
#[cfg(feature = "ApplicationModel_Activation")]
impl WebUIUserDataAccountProviderActivatedEventArgs {
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Kind(&self) -> ::windows_core::Result<::winrt_applicationmodel::Activation::ActivationKind> {
        let this = &::windows_core::Interface::cast::<::winrt_applicationmodel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_applicationmodel::Activation::ActivationKind>::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_applicationmodel::Activation::ActivationKind>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<::winrt_applicationmodel::Activation::ApplicationExecutionState> {
        let this = &::windows_core::Interface::cast::<::winrt_applicationmodel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_applicationmodel::Activation::ApplicationExecutionState>::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_applicationmodel::Activation::ApplicationExecutionState>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn SplashScreen(&self) -> ::windows_core::Result<::winrt_applicationmodel::Activation::SplashScreen> {
        let this = &::windows_core::Interface::cast::<::winrt_applicationmodel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_applicationmodel::Activation::SplashScreen>(result__)
        }
    }
    pub fn ActivatedOperation(&self) -> ::windows_core::Result<ActivatedOperation> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgsDeferral>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ActivatedOperation)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivatedOperation>(result__)
        }
    }
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "ApplicationModel_UserDataAccounts_Provider"))]
    pub fn Operation(&self) -> ::windows_core::Result<::winrt_applicationmodel::UserDataAccounts::Provider::IUserDataAccountProviderOperation> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Operation)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_applicationmodel::UserDataAccounts::Provider::IUserDataAccountProviderOperation>(result__)
        }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::clone::Clone for WebUIUserDataAccountProviderActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::cmp::PartialEq for WebUIUserDataAccountProviderActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::cmp::Eq for WebUIUserDataAccountProviderActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::fmt::Debug for WebUIUserDataAccountProviderActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WebUIUserDataAccountProviderActivatedEventArgs").field(&self.0).finish()
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows_core::RuntimeType for WebUIUserDataAccountProviderActivatedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.WebUI.WebUIUserDataAccountProviderActivatedEventArgs;{1bc9f723-8ef1-4a51-a63a-fe711eeab607})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows_core::Interface for WebUIUserDataAccountProviderActivatedEventArgs {
    type Vtable = ::winrt_applicationmodel::Activation::IUserDataAccountProviderActivatedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <::winrt_applicationmodel::Activation::IUserDataAccountProviderActivatedEventArgs as ::windows_core::Interface>::IID;
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::RuntimeName for WebUIUserDataAccountProviderActivatedEventArgs {
    const NAME: &'static str = "Windows.UI.WebUI.WebUIUserDataAccountProviderActivatedEventArgs";
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<WebUIUserDataAccountProviderActivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: WebUIUserDataAccountProviderActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<&WebUIUserDataAccountProviderActivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &WebUIUserDataAccountProviderActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for WebUIUserDataAccountProviderActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a WebUIUserDataAccountProviderActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<WebUIUserDataAccountProviderActivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: WebUIUserDataAccountProviderActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<&WebUIUserDataAccountProviderActivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &WebUIUserDataAccountProviderActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for WebUIUserDataAccountProviderActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a WebUIUserDataAccountProviderActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIUserDataAccountProviderActivatedEventArgs> for ::winrt_applicationmodel::Activation::IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: WebUIUserDataAccountProviderActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIUserDataAccountProviderActivatedEventArgs> for ::winrt_applicationmodel::Activation::IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &WebUIUserDataAccountProviderActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgs> for WebUIUserDataAccountProviderActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgs> for &WebUIUserDataAccountProviderActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgs> {
        ::core::convert::TryInto::<::winrt_applicationmodel::Activation::IActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIUserDataAccountProviderActivatedEventArgs> for IActivatedEventArgsDeferral {
    type Error = ::windows_core::Error;
    fn try_from(value: WebUIUserDataAccountProviderActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIUserDataAccountProviderActivatedEventArgs> for IActivatedEventArgsDeferral {
    type Error = ::windows_core::Error;
    fn try_from(value: &WebUIUserDataAccountProviderActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgsDeferral> for WebUIUserDataAccountProviderActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgsDeferral> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgsDeferral> for &WebUIUserDataAccountProviderActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgsDeferral> {
        ::core::convert::TryInto::<IActivatedEventArgsDeferral>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIUserDataAccountProviderActivatedEventArgs> for ::winrt_applicationmodel::Activation::IUserDataAccountProviderActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: WebUIUserDataAccountProviderActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIUserDataAccountProviderActivatedEventArgs> for ::winrt_applicationmodel::Activation::IUserDataAccountProviderActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &WebUIUserDataAccountProviderActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IUserDataAccountProviderActivatedEventArgs> for WebUIUserDataAccountProviderActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IUserDataAccountProviderActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IUserDataAccountProviderActivatedEventArgs> for &WebUIUserDataAccountProviderActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IUserDataAccountProviderActivatedEventArgs> {
        ::core::convert::TryInto::<::winrt_applicationmodel::Activation::IUserDataAccountProviderActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[repr(transparent)]
pub struct WebUIView(::windows_core::IUnknown);
impl WebUIView {
    pub fn ApplicationViewId(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows_core::Interface::vtable(this).ApplicationViewId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    pub fn Closed<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<WebUIView, ::windows_core::IInspectable>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
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
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Activated<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<WebUIView, ::winrt_applicationmodel::Activation::IActivatedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
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
    pub fn IgnoreApplicationContentUriRulesNavigationRestrictions(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IgnoreApplicationContentUriRulesNavigationRestrictions)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetIgnoreApplicationContentUriRulesNavigationRestrictions(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetIgnoreApplicationContentUriRulesNavigationRestrictions)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn CreateAsync() -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<WebUIView>> {
        Self::IWebUIViewStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<WebUIView>>(result__)
        })
    }
    pub fn CreateWithUriAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Uri>>(uri: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<WebUIView>> {
        Self::IWebUIViewStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateWithUriAsync)(::windows_core::Interface::as_raw(this), uri.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<WebUIView>>(result__)
        })
    }
    #[cfg(feature = "Web_UI")]
    pub fn Source(&self) -> ::windows_core::Result<::winrt_foundation::Uri> {
        let this = &::windows_core::Interface::cast::<::winrt_web::UI::IWebViewControl>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Source)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Uri>(result__)
        }
    }
    #[cfg(feature = "Web_UI")]
    pub fn SetSource<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Uri>>(&self, source: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_web::UI::IWebViewControl>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetSource)(::windows_core::Interface::as_raw(this), source.into_param().abi()).ok() }
    }
    #[cfg(feature = "Web_UI")]
    pub fn DocumentTitle(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<::winrt_web::UI::IWebViewControl>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).DocumentTitle)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Web_UI")]
    pub fn CanGoBack(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<::winrt_web::UI::IWebViewControl>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).CanGoBack)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Web_UI")]
    pub fn CanGoForward(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<::winrt_web::UI::IWebViewControl>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).CanGoForward)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Web_UI")]
    pub fn SetDefaultBackgroundColor<'a, Param0: ::windows_core::IntoParam<'a, super::Color>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_web::UI::IWebViewControl>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetDefaultBackgroundColor)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Web_UI")]
    pub fn DefaultBackgroundColor(&self) -> ::windows_core::Result<super::Color> {
        let this = &::windows_core::Interface::cast::<::winrt_web::UI::IWebViewControl>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::Color>::zeroed();
            (::windows_core::Interface::vtable(this).DefaultBackgroundColor)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::Color>(result__)
        }
    }
    #[cfg(feature = "Web_UI")]
    pub fn ContainsFullScreenElement(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<::winrt_web::UI::IWebViewControl>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).ContainsFullScreenElement)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Web_UI")]
    pub fn Settings(&self) -> ::windows_core::Result<::winrt_web::UI::WebViewControlSettings> {
        let this = &::windows_core::Interface::cast::<::winrt_web::UI::IWebViewControl>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Settings)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_web::UI::WebViewControlSettings>(result__)
        }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Web_UI"))]
    pub fn DeferredPermissionRequests(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<::winrt_web::UI::WebViewControlDeferredPermissionRequest>> {
        let this = &::windows_core::Interface::cast::<::winrt_web::UI::IWebViewControl>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).DeferredPermissionRequests)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<::winrt_web::UI::WebViewControlDeferredPermissionRequest>>(result__)
        }
    }
    #[cfg(feature = "Web_UI")]
    pub fn GoForward(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_web::UI::IWebViewControl>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).GoForward)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[cfg(feature = "Web_UI")]
    pub fn GoBack(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_web::UI::IWebViewControl>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).GoBack)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[cfg(feature = "Web_UI")]
    pub fn Refresh(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_web::UI::IWebViewControl>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Refresh)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[cfg(feature = "Web_UI")]
    pub fn Stop(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_web::UI::IWebViewControl>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Stop)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[cfg(feature = "Web_UI")]
    pub fn Navigate<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Uri>>(&self, source: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_web::UI::IWebViewControl>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Navigate)(::windows_core::Interface::as_raw(this), source.into_param().abi()).ok() }
    }
    #[cfg(feature = "Web_UI")]
    pub fn NavigateToString<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, text: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_web::UI::IWebViewControl>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).NavigateToString)(::windows_core::Interface::as_raw(this), text.into_param().abi()).ok() }
    }
    #[cfg(feature = "Web_UI")]
    pub fn NavigateToLocalStreamUri<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::Uri>, Param1: ::windows_core::IntoParam<'a, ::winrt_web::IUriToStreamResolver>>(&self, source: Param0, streamresolver: Param1) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_web::UI::IWebViewControl>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).NavigateToLocalStreamUri)(::windows_core::Interface::as_raw(this), source.into_param().abi(), streamresolver.into_param().abi()).ok() }
    }
    #[cfg(all(feature = "Web_Http", feature = "Web_UI"))]
    pub fn NavigateWithHttpRequestMessage<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_web::Http::HttpRequestMessage>>(&self, requestmessage: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_web::UI::IWebViewControl>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).NavigateWithHttpRequestMessage)(::windows_core::Interface::as_raw(this), requestmessage.into_param().abi()).ok() }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Web_UI"))]
    pub fn InvokeScriptAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<::windows_core::HSTRING>>>(&self, scriptname: Param0, arguments: Param1) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<::windows_core::HSTRING>> {
        let this = &::windows_core::Interface::cast::<::winrt_web::UI::IWebViewControl>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).InvokeScriptAsync)(::windows_core::Interface::as_raw(this), scriptname.into_param().abi(), arguments.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<::windows_core::HSTRING>>(result__)
        }
    }
    #[cfg(all(feature = "Storage_Streams", feature = "Web_UI"))]
    pub fn CapturePreviewToStreamAsync<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_storage::Streams::IRandomAccessStream>>(&self, stream: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncAction> {
        let this = &::windows_core::Interface::cast::<::winrt_web::UI::IWebViewControl>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CapturePreviewToStreamAsync)(::windows_core::Interface::as_raw(this), stream.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(all(feature = "ApplicationModel_DataTransfer", feature = "Web_UI"))]
    pub fn CaptureSelectedContentToDataPackageAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<::winrt_applicationmodel::DataTransfer::DataPackage>> {
        let this = &::windows_core::Interface::cast::<::winrt_web::UI::IWebViewControl>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CaptureSelectedContentToDataPackageAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<::winrt_applicationmodel::DataTransfer::DataPackage>>(result__)
        }
    }
    #[cfg(feature = "Web_UI")]
    pub fn BuildLocalStreamUri<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, contentidentifier: Param0, relativepath: Param1) -> ::windows_core::Result<::winrt_foundation::Uri> {
        let this = &::windows_core::Interface::cast::<::winrt_web::UI::IWebViewControl>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).BuildLocalStreamUri)(::windows_core::Interface::as_raw(this), contentidentifier.into_param().abi(), relativepath.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Uri>(result__)
        }
    }
    #[cfg(feature = "Web_UI")]
    pub fn GetDeferredPermissionRequestById(&self, id: u32, result: &mut ::core::option::Option<::winrt_web::UI::WebViewControlDeferredPermissionRequest>) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_web::UI::IWebViewControl>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).GetDeferredPermissionRequestById)(::windows_core::Interface::as_raw(this), id, result as *mut _ as _).ok() }
    }
    #[cfg(feature = "Web_UI")]
    pub fn NavigationStarting<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<::winrt_web::UI::IWebViewControl, ::winrt_web::UI::WebViewControlNavigationStartingEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = &::windows_core::Interface::cast::<::winrt_web::UI::IWebViewControl>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).NavigationStarting)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Web_UI")]
    pub fn RemoveNavigationStarting<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_web::UI::IWebViewControl>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).RemoveNavigationStarting)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Web_UI")]
    pub fn ContentLoading<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<::winrt_web::UI::IWebViewControl, ::winrt_web::UI::WebViewControlContentLoadingEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = &::windows_core::Interface::cast::<::winrt_web::UI::IWebViewControl>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).ContentLoading)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Web_UI")]
    pub fn RemoveContentLoading<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_web::UI::IWebViewControl>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).RemoveContentLoading)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Web_UI")]
    pub fn DOMContentLoaded<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<::winrt_web::UI::IWebViewControl, ::winrt_web::UI::WebViewControlDOMContentLoadedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = &::windows_core::Interface::cast::<::winrt_web::UI::IWebViewControl>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).DOMContentLoaded)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Web_UI")]
    pub fn RemoveDOMContentLoaded<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_web::UI::IWebViewControl>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).RemoveDOMContentLoaded)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Web_UI")]
    pub fn NavigationCompleted<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<::winrt_web::UI::IWebViewControl, ::winrt_web::UI::WebViewControlNavigationCompletedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = &::windows_core::Interface::cast::<::winrt_web::UI::IWebViewControl>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).NavigationCompleted)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Web_UI")]
    pub fn RemoveNavigationCompleted<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_web::UI::IWebViewControl>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).RemoveNavigationCompleted)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Web_UI")]
    pub fn FrameNavigationStarting<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<::winrt_web::UI::IWebViewControl, ::winrt_web::UI::WebViewControlNavigationStartingEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = &::windows_core::Interface::cast::<::winrt_web::UI::IWebViewControl>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).FrameNavigationStarting)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Web_UI")]
    pub fn RemoveFrameNavigationStarting<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_web::UI::IWebViewControl>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).RemoveFrameNavigationStarting)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Web_UI")]
    pub fn FrameContentLoading<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<::winrt_web::UI::IWebViewControl, ::winrt_web::UI::WebViewControlContentLoadingEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = &::windows_core::Interface::cast::<::winrt_web::UI::IWebViewControl>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).FrameContentLoading)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Web_UI")]
    pub fn RemoveFrameContentLoading<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_web::UI::IWebViewControl>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).RemoveFrameContentLoading)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Web_UI")]
    pub fn FrameDOMContentLoaded<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<::winrt_web::UI::IWebViewControl, ::winrt_web::UI::WebViewControlDOMContentLoadedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = &::windows_core::Interface::cast::<::winrt_web::UI::IWebViewControl>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).FrameDOMContentLoaded)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Web_UI")]
    pub fn RemoveFrameDOMContentLoaded<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_web::UI::IWebViewControl>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).RemoveFrameDOMContentLoaded)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Web_UI")]
    pub fn FrameNavigationCompleted<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<::winrt_web::UI::IWebViewControl, ::winrt_web::UI::WebViewControlNavigationCompletedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = &::windows_core::Interface::cast::<::winrt_web::UI::IWebViewControl>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).FrameNavigationCompleted)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Web_UI")]
    pub fn RemoveFrameNavigationCompleted<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_web::UI::IWebViewControl>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).RemoveFrameNavigationCompleted)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Web_UI")]
    pub fn ScriptNotify<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<::winrt_web::UI::IWebViewControl, ::winrt_web::UI::WebViewControlScriptNotifyEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = &::windows_core::Interface::cast::<::winrt_web::UI::IWebViewControl>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).ScriptNotify)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Web_UI")]
    pub fn RemoveScriptNotify<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_web::UI::IWebViewControl>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).RemoveScriptNotify)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Web_UI")]
    pub fn LongRunningScriptDetected<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<::winrt_web::UI::IWebViewControl, ::winrt_web::UI::WebViewControlLongRunningScriptDetectedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = &::windows_core::Interface::cast::<::winrt_web::UI::IWebViewControl>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).LongRunningScriptDetected)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Web_UI")]
    pub fn RemoveLongRunningScriptDetected<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_web::UI::IWebViewControl>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).RemoveLongRunningScriptDetected)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Web_UI")]
    pub fn UnsafeContentWarningDisplaying<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<::winrt_web::UI::IWebViewControl, ::windows_core::IInspectable>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = &::windows_core::Interface::cast::<::winrt_web::UI::IWebViewControl>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).UnsafeContentWarningDisplaying)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Web_UI")]
    pub fn RemoveUnsafeContentWarningDisplaying<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_web::UI::IWebViewControl>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).RemoveUnsafeContentWarningDisplaying)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Web_UI")]
    pub fn UnviewableContentIdentified<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<::winrt_web::UI::IWebViewControl, ::winrt_web::UI::WebViewControlUnviewableContentIdentifiedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = &::windows_core::Interface::cast::<::winrt_web::UI::IWebViewControl>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).UnviewableContentIdentified)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Web_UI")]
    pub fn RemoveUnviewableContentIdentified<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_web::UI::IWebViewControl>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).RemoveUnviewableContentIdentified)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Web_UI")]
    pub fn PermissionRequested<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<::winrt_web::UI::IWebViewControl, ::winrt_web::UI::WebViewControlPermissionRequestedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = &::windows_core::Interface::cast::<::winrt_web::UI::IWebViewControl>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).PermissionRequested)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Web_UI")]
    pub fn RemovePermissionRequested<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_web::UI::IWebViewControl>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).RemovePermissionRequested)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Web_UI")]
    pub fn UnsupportedUriSchemeIdentified<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<::winrt_web::UI::IWebViewControl, ::winrt_web::UI::WebViewControlUnsupportedUriSchemeIdentifiedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = &::windows_core::Interface::cast::<::winrt_web::UI::IWebViewControl>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).UnsupportedUriSchemeIdentified)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Web_UI")]
    pub fn RemoveUnsupportedUriSchemeIdentified<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_web::UI::IWebViewControl>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).RemoveUnsupportedUriSchemeIdentified)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Web_UI")]
    pub fn NewWindowRequested<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<::winrt_web::UI::IWebViewControl, ::winrt_web::UI::WebViewControlNewWindowRequestedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = &::windows_core::Interface::cast::<::winrt_web::UI::IWebViewControl>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).NewWindowRequested)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Web_UI")]
    pub fn RemoveNewWindowRequested<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_web::UI::IWebViewControl>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).RemoveNewWindowRequested)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Web_UI")]
    pub fn ContainsFullScreenElementChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<::winrt_web::UI::IWebViewControl, ::windows_core::IInspectable>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = &::windows_core::Interface::cast::<::winrt_web::UI::IWebViewControl>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).ContainsFullScreenElementChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Web_UI")]
    pub fn RemoveContainsFullScreenElementChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_web::UI::IWebViewControl>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).RemoveContainsFullScreenElementChanged)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Web_UI")]
    pub fn WebResourceRequested<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<::winrt_web::UI::IWebViewControl, ::winrt_web::UI::WebViewControlWebResourceRequestedEventArgs>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = &::windows_core::Interface::cast::<::winrt_web::UI::IWebViewControl>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).WebResourceRequested)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Web_UI")]
    pub fn RemoveWebResourceRequested<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_web::UI::IWebViewControl>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).RemoveWebResourceRequested)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Web_UI")]
    pub fn AddInitializeScript<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, script: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_web::UI::IWebViewControl2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).AddInitializeScript)(::windows_core::Interface::as_raw(this), script.into_param().abi()).ok() }
    }
    pub fn IWebUIViewStatics<R, F: FnOnce(&IWebUIViewStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<WebUIView, IWebUIViewStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for WebUIView {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for WebUIView {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for WebUIView {}
impl ::core::fmt::Debug for WebUIView {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WebUIView").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for WebUIView {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.WebUI.WebUIView;{6783f64f-52da-4fd7-be69-8ef6284b423c})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for WebUIView {
    type Vtable = IWebUIView_Vtbl;
    const IID: ::windows_core::GUID = <IWebUIView as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for WebUIView {
    const NAME: &'static str = "Windows.UI.WebUI.WebUIView";
}
impl ::core::convert::From<WebUIView> for ::windows_core::IUnknown {
    fn from(value: WebUIView) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WebUIView> for ::windows_core::IUnknown {
    fn from(value: &WebUIView) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for WebUIView {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a WebUIView {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<WebUIView> for ::windows_core::IInspectable {
    fn from(value: WebUIView) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WebUIView> for ::windows_core::IInspectable {
    fn from(value: &WebUIView) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for WebUIView {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a WebUIView {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Web_UI")]
impl ::core::convert::TryFrom<WebUIView> for ::winrt_web::UI::IWebViewControl {
    type Error = ::windows_core::Error;
    fn try_from(value: WebUIView) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Web_UI")]
impl ::core::convert::TryFrom<&WebUIView> for ::winrt_web::UI::IWebViewControl {
    type Error = ::windows_core::Error;
    fn try_from(value: &WebUIView) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "Web_UI")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_web::UI::IWebViewControl> for WebUIView {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_web::UI::IWebViewControl> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Web_UI")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_web::UI::IWebViewControl> for &WebUIView {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_web::UI::IWebViewControl> {
        ::core::convert::TryInto::<::winrt_web::UI::IWebViewControl>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "Web_UI")]
impl ::core::convert::TryFrom<WebUIView> for ::winrt_web::UI::IWebViewControl2 {
    type Error = ::windows_core::Error;
    fn try_from(value: WebUIView) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Web_UI")]
impl ::core::convert::TryFrom<&WebUIView> for ::winrt_web::UI::IWebViewControl2 {
    type Error = ::windows_core::Error;
    fn try_from(value: &WebUIView) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "Web_UI")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_web::UI::IWebViewControl2> for WebUIView {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_web::UI::IWebViewControl2> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Web_UI")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_web::UI::IWebViewControl2> for &WebUIView {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_web::UI::IWebViewControl2> {
        ::core::convert::TryInto::<::winrt_web::UI::IWebViewControl2>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
#[repr(transparent)]
pub struct WebUIVoiceCommandActivatedEventArgs(::windows_core::IUnknown);
#[cfg(feature = "ApplicationModel_Activation")]
impl WebUIVoiceCommandActivatedEventArgs {
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Kind(&self) -> ::windows_core::Result<::winrt_applicationmodel::Activation::ActivationKind> {
        let this = &::windows_core::Interface::cast::<::winrt_applicationmodel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_applicationmodel::Activation::ActivationKind>::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_applicationmodel::Activation::ActivationKind>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<::winrt_applicationmodel::Activation::ApplicationExecutionState> {
        let this = &::windows_core::Interface::cast::<::winrt_applicationmodel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_applicationmodel::Activation::ApplicationExecutionState>::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_applicationmodel::Activation::ApplicationExecutionState>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn SplashScreen(&self) -> ::windows_core::Result<::winrt_applicationmodel::Activation::SplashScreen> {
        let this = &::windows_core::Interface::cast::<::winrt_applicationmodel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_applicationmodel::Activation::SplashScreen>(result__)
        }
    }
    pub fn ActivatedOperation(&self) -> ::windows_core::Result<ActivatedOperation> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgsDeferral>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ActivatedOperation)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivatedOperation>(result__)
        }
    }
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "System"))]
    pub fn User(&self) -> ::windows_core::Result<::winrt_system::User> {
        let this = &::windows_core::Interface::cast::<::winrt_applicationmodel::Activation::IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).User)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_system::User>(result__)
        }
    }
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "Media_SpeechRecognition"))]
    pub fn Result(&self) -> ::windows_core::Result<::winrt_media::SpeechRecognition::SpeechRecognitionResult> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Result)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_media::SpeechRecognition::SpeechRecognitionResult>(result__)
        }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::clone::Clone for WebUIVoiceCommandActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::cmp::PartialEq for WebUIVoiceCommandActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::cmp::Eq for WebUIVoiceCommandActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::fmt::Debug for WebUIVoiceCommandActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WebUIVoiceCommandActivatedEventArgs").field(&self.0).finish()
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows_core::RuntimeType for WebUIVoiceCommandActivatedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.WebUI.WebUIVoiceCommandActivatedEventArgs;{ab92dcfd-8d43-4de6-9775-20704b581b00})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows_core::Interface for WebUIVoiceCommandActivatedEventArgs {
    type Vtable = ::winrt_applicationmodel::Activation::IVoiceCommandActivatedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <::winrt_applicationmodel::Activation::IVoiceCommandActivatedEventArgs as ::windows_core::Interface>::IID;
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::RuntimeName for WebUIVoiceCommandActivatedEventArgs {
    const NAME: &'static str = "Windows.UI.WebUI.WebUIVoiceCommandActivatedEventArgs";
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<WebUIVoiceCommandActivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: WebUIVoiceCommandActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<&WebUIVoiceCommandActivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &WebUIVoiceCommandActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for WebUIVoiceCommandActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a WebUIVoiceCommandActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<WebUIVoiceCommandActivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: WebUIVoiceCommandActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<&WebUIVoiceCommandActivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &WebUIVoiceCommandActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for WebUIVoiceCommandActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a WebUIVoiceCommandActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIVoiceCommandActivatedEventArgs> for ::winrt_applicationmodel::Activation::IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: WebUIVoiceCommandActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIVoiceCommandActivatedEventArgs> for ::winrt_applicationmodel::Activation::IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &WebUIVoiceCommandActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgs> for WebUIVoiceCommandActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgs> for &WebUIVoiceCommandActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgs> {
        ::core::convert::TryInto::<::winrt_applicationmodel::Activation::IActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIVoiceCommandActivatedEventArgs> for IActivatedEventArgsDeferral {
    type Error = ::windows_core::Error;
    fn try_from(value: WebUIVoiceCommandActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIVoiceCommandActivatedEventArgs> for IActivatedEventArgsDeferral {
    type Error = ::windows_core::Error;
    fn try_from(value: &WebUIVoiceCommandActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgsDeferral> for WebUIVoiceCommandActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgsDeferral> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgsDeferral> for &WebUIVoiceCommandActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgsDeferral> {
        ::core::convert::TryInto::<IActivatedEventArgsDeferral>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIVoiceCommandActivatedEventArgs> for ::winrt_applicationmodel::Activation::IActivatedEventArgsWithUser {
    type Error = ::windows_core::Error;
    fn try_from(value: WebUIVoiceCommandActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIVoiceCommandActivatedEventArgs> for ::winrt_applicationmodel::Activation::IActivatedEventArgsWithUser {
    type Error = ::windows_core::Error;
    fn try_from(value: &WebUIVoiceCommandActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgsWithUser> for WebUIVoiceCommandActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgsWithUser> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgsWithUser> for &WebUIVoiceCommandActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgsWithUser> {
        ::core::convert::TryInto::<::winrt_applicationmodel::Activation::IActivatedEventArgsWithUser>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIVoiceCommandActivatedEventArgs> for ::winrt_applicationmodel::Activation::IVoiceCommandActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: WebUIVoiceCommandActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIVoiceCommandActivatedEventArgs> for ::winrt_applicationmodel::Activation::IVoiceCommandActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &WebUIVoiceCommandActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IVoiceCommandActivatedEventArgs> for WebUIVoiceCommandActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IVoiceCommandActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IVoiceCommandActivatedEventArgs> for &WebUIVoiceCommandActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IVoiceCommandActivatedEventArgs> {
        ::core::convert::TryInto::<::winrt_applicationmodel::Activation::IVoiceCommandActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
#[repr(transparent)]
pub struct WebUIWalletActionActivatedEventArgs(::windows_core::IUnknown);
#[cfg(feature = "ApplicationModel_Activation")]
impl WebUIWalletActionActivatedEventArgs {
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Kind(&self) -> ::windows_core::Result<::winrt_applicationmodel::Activation::ActivationKind> {
        let this = &::windows_core::Interface::cast::<::winrt_applicationmodel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_applicationmodel::Activation::ActivationKind>::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_applicationmodel::Activation::ActivationKind>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<::winrt_applicationmodel::Activation::ApplicationExecutionState> {
        let this = &::windows_core::Interface::cast::<::winrt_applicationmodel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_applicationmodel::Activation::ApplicationExecutionState>::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_applicationmodel::Activation::ApplicationExecutionState>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn SplashScreen(&self) -> ::windows_core::Result<::winrt_applicationmodel::Activation::SplashScreen> {
        let this = &::windows_core::Interface::cast::<::winrt_applicationmodel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_applicationmodel::Activation::SplashScreen>(result__)
        }
    }
    pub fn ActivatedOperation(&self) -> ::windows_core::Result<ActivatedOperation> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgsDeferral>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ActivatedOperation)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivatedOperation>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn ItemId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).ItemId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "ApplicationModel_Wallet"))]
    pub fn ActionKind(&self) -> ::windows_core::Result<::winrt_applicationmodel::Wallet::WalletActionKind> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_applicationmodel::Wallet::WalletActionKind>::zeroed();
            (::windows_core::Interface::vtable(this).ActionKind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_applicationmodel::Wallet::WalletActionKind>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn ActionId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).ActionId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::clone::Clone for WebUIWalletActionActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::cmp::PartialEq for WebUIWalletActionActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::cmp::Eq for WebUIWalletActionActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::fmt::Debug for WebUIWalletActionActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WebUIWalletActionActivatedEventArgs").field(&self.0).finish()
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows_core::RuntimeType for WebUIWalletActionActivatedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.WebUI.WebUIWalletActionActivatedEventArgs;{fcfc027b-1a1a-4d22-923f-ae6f45fa52d9})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows_core::Interface for WebUIWalletActionActivatedEventArgs {
    type Vtable = ::winrt_applicationmodel::Activation::IWalletActionActivatedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <::winrt_applicationmodel::Activation::IWalletActionActivatedEventArgs as ::windows_core::Interface>::IID;
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::RuntimeName for WebUIWalletActionActivatedEventArgs {
    const NAME: &'static str = "Windows.UI.WebUI.WebUIWalletActionActivatedEventArgs";
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<WebUIWalletActionActivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: WebUIWalletActionActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<&WebUIWalletActionActivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &WebUIWalletActionActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for WebUIWalletActionActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a WebUIWalletActionActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<WebUIWalletActionActivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: WebUIWalletActionActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<&WebUIWalletActionActivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &WebUIWalletActionActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for WebUIWalletActionActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a WebUIWalletActionActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIWalletActionActivatedEventArgs> for ::winrt_applicationmodel::Activation::IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: WebUIWalletActionActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIWalletActionActivatedEventArgs> for ::winrt_applicationmodel::Activation::IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &WebUIWalletActionActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgs> for WebUIWalletActionActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgs> for &WebUIWalletActionActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgs> {
        ::core::convert::TryInto::<::winrt_applicationmodel::Activation::IActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIWalletActionActivatedEventArgs> for IActivatedEventArgsDeferral {
    type Error = ::windows_core::Error;
    fn try_from(value: WebUIWalletActionActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIWalletActionActivatedEventArgs> for IActivatedEventArgsDeferral {
    type Error = ::windows_core::Error;
    fn try_from(value: &WebUIWalletActionActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgsDeferral> for WebUIWalletActionActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgsDeferral> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgsDeferral> for &WebUIWalletActionActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgsDeferral> {
        ::core::convert::TryInto::<IActivatedEventArgsDeferral>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIWalletActionActivatedEventArgs> for ::winrt_applicationmodel::Activation::IWalletActionActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: WebUIWalletActionActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIWalletActionActivatedEventArgs> for ::winrt_applicationmodel::Activation::IWalletActionActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &WebUIWalletActionActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IWalletActionActivatedEventArgs> for WebUIWalletActionActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IWalletActionActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IWalletActionActivatedEventArgs> for &WebUIWalletActionActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IWalletActionActivatedEventArgs> {
        ::core::convert::TryInto::<::winrt_applicationmodel::Activation::IWalletActionActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
#[repr(transparent)]
pub struct WebUIWebAccountProviderActivatedEventArgs(::windows_core::IUnknown);
#[cfg(feature = "ApplicationModel_Activation")]
impl WebUIWebAccountProviderActivatedEventArgs {
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Kind(&self) -> ::windows_core::Result<::winrt_applicationmodel::Activation::ActivationKind> {
        let this = &::windows_core::Interface::cast::<::winrt_applicationmodel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_applicationmodel::Activation::ActivationKind>::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_applicationmodel::Activation::ActivationKind>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<::winrt_applicationmodel::Activation::ApplicationExecutionState> {
        let this = &::windows_core::Interface::cast::<::winrt_applicationmodel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_applicationmodel::Activation::ApplicationExecutionState>::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_applicationmodel::Activation::ApplicationExecutionState>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn SplashScreen(&self) -> ::windows_core::Result<::winrt_applicationmodel::Activation::SplashScreen> {
        let this = &::windows_core::Interface::cast::<::winrt_applicationmodel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_applicationmodel::Activation::SplashScreen>(result__)
        }
    }
    pub fn ActivatedOperation(&self) -> ::windows_core::Result<ActivatedOperation> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgsDeferral>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ActivatedOperation)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivatedOperation>(result__)
        }
    }
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "System"))]
    pub fn User(&self) -> ::windows_core::Result<::winrt_system::User> {
        let this = &::windows_core::Interface::cast::<::winrt_applicationmodel::Activation::IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).User)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_system::User>(result__)
        }
    }
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "Security_Authentication_Web_Provider"))]
    pub fn Operation(&self) -> ::windows_core::Result<::winrt_security::Authentication::Web::Provider::IWebAccountProviderOperation> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Operation)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_security::Authentication::Web::Provider::IWebAccountProviderOperation>(result__)
        }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::clone::Clone for WebUIWebAccountProviderActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::cmp::PartialEq for WebUIWebAccountProviderActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::cmp::Eq for WebUIWebAccountProviderActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::fmt::Debug for WebUIWebAccountProviderActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WebUIWebAccountProviderActivatedEventArgs").field(&self.0).finish()
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows_core::RuntimeType for WebUIWebAccountProviderActivatedEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.WebUI.WebUIWebAccountProviderActivatedEventArgs;{72b71774-98ea-4ccf-9752-46d9051004f1})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows_core::Interface for WebUIWebAccountProviderActivatedEventArgs {
    type Vtable = ::winrt_applicationmodel::Activation::IWebAccountProviderActivatedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <::winrt_applicationmodel::Activation::IWebAccountProviderActivatedEventArgs as ::windows_core::Interface>::IID;
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::RuntimeName for WebUIWebAccountProviderActivatedEventArgs {
    const NAME: &'static str = "Windows.UI.WebUI.WebUIWebAccountProviderActivatedEventArgs";
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<WebUIWebAccountProviderActivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: WebUIWebAccountProviderActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<&WebUIWebAccountProviderActivatedEventArgs> for ::windows_core::IUnknown {
    fn from(value: &WebUIWebAccountProviderActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for WebUIWebAccountProviderActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a WebUIWebAccountProviderActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<WebUIWebAccountProviderActivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: WebUIWebAccountProviderActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<&WebUIWebAccountProviderActivatedEventArgs> for ::windows_core::IInspectable {
    fn from(value: &WebUIWebAccountProviderActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for WebUIWebAccountProviderActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a WebUIWebAccountProviderActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIWebAccountProviderActivatedEventArgs> for ::winrt_applicationmodel::Activation::IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: WebUIWebAccountProviderActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIWebAccountProviderActivatedEventArgs> for ::winrt_applicationmodel::Activation::IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &WebUIWebAccountProviderActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgs> for WebUIWebAccountProviderActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgs> for &WebUIWebAccountProviderActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgs> {
        ::core::convert::TryInto::<::winrt_applicationmodel::Activation::IActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIWebAccountProviderActivatedEventArgs> for IActivatedEventArgsDeferral {
    type Error = ::windows_core::Error;
    fn try_from(value: WebUIWebAccountProviderActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIWebAccountProviderActivatedEventArgs> for IActivatedEventArgsDeferral {
    type Error = ::windows_core::Error;
    fn try_from(value: &WebUIWebAccountProviderActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgsDeferral> for WebUIWebAccountProviderActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgsDeferral> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgsDeferral> for &WebUIWebAccountProviderActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgsDeferral> {
        ::core::convert::TryInto::<IActivatedEventArgsDeferral>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIWebAccountProviderActivatedEventArgs> for ::winrt_applicationmodel::Activation::IActivatedEventArgsWithUser {
    type Error = ::windows_core::Error;
    fn try_from(value: WebUIWebAccountProviderActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIWebAccountProviderActivatedEventArgs> for ::winrt_applicationmodel::Activation::IActivatedEventArgsWithUser {
    type Error = ::windows_core::Error;
    fn try_from(value: &WebUIWebAccountProviderActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgsWithUser> for WebUIWebAccountProviderActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgsWithUser> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgsWithUser> for &WebUIWebAccountProviderActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgsWithUser> {
        ::core::convert::TryInto::<::winrt_applicationmodel::Activation::IActivatedEventArgsWithUser>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIWebAccountProviderActivatedEventArgs> for ::winrt_applicationmodel::Activation::IWebAccountProviderActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: WebUIWebAccountProviderActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIWebAccountProviderActivatedEventArgs> for ::winrt_applicationmodel::Activation::IWebAccountProviderActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &WebUIWebAccountProviderActivatedEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IWebAccountProviderActivatedEventArgs> for WebUIWebAccountProviderActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IWebAccountProviderActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IWebAccountProviderActivatedEventArgs> for &WebUIWebAccountProviderActivatedEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IWebAccountProviderActivatedEventArgs> {
        ::core::convert::TryInto::<::winrt_applicationmodel::Activation::IWebAccountProviderActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
#[repr(transparent)]
pub struct WebUIWebAuthenticationBrokerContinuationEventArgs(::windows_core::IUnknown);
#[cfg(feature = "ApplicationModel_Activation")]
impl WebUIWebAuthenticationBrokerContinuationEventArgs {
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Kind(&self) -> ::windows_core::Result<::winrt_applicationmodel::Activation::ActivationKind> {
        let this = &::windows_core::Interface::cast::<::winrt_applicationmodel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_applicationmodel::Activation::ActivationKind>::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_applicationmodel::Activation::ActivationKind>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<::winrt_applicationmodel::Activation::ApplicationExecutionState> {
        let this = &::windows_core::Interface::cast::<::winrt_applicationmodel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_applicationmodel::Activation::ApplicationExecutionState>::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_applicationmodel::Activation::ApplicationExecutionState>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn SplashScreen(&self) -> ::windows_core::Result<::winrt_applicationmodel::Activation::SplashScreen> {
        let this = &::windows_core::Interface::cast::<::winrt_applicationmodel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_applicationmodel::Activation::SplashScreen>(result__)
        }
    }
    pub fn ActivatedOperation(&self) -> ::windows_core::Result<ActivatedOperation> {
        let this = &::windows_core::Interface::cast::<IActivatedEventArgsDeferral>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ActivatedOperation)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivatedOperation>(result__)
        }
    }
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "Foundation_Collections"))]
    pub fn ContinuationData(&self) -> ::windows_core::Result<::winrt_foundation::Collections::ValueSet> {
        let this = &::windows_core::Interface::cast::<::winrt_applicationmodel::Activation::IContinuationActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ContinuationData)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::ValueSet>(result__)
        }
    }
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "Security_Authentication_Web"))]
    pub fn WebAuthenticationResult(&self) -> ::windows_core::Result<::winrt_security::Authentication::Web::WebAuthenticationResult> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).WebAuthenticationResult)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_security::Authentication::Web::WebAuthenticationResult>(result__)
        }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::clone::Clone for WebUIWebAuthenticationBrokerContinuationEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::cmp::PartialEq for WebUIWebAuthenticationBrokerContinuationEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::cmp::Eq for WebUIWebAuthenticationBrokerContinuationEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::fmt::Debug for WebUIWebAuthenticationBrokerContinuationEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WebUIWebAuthenticationBrokerContinuationEventArgs").field(&self.0).finish()
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows_core::RuntimeType for WebUIWebAuthenticationBrokerContinuationEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.WebUI.WebUIWebAuthenticationBrokerContinuationEventArgs;{75dda3d4-7714-453d-b7ff-b95e3a1709da})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows_core::Interface for WebUIWebAuthenticationBrokerContinuationEventArgs {
    type Vtable = ::winrt_applicationmodel::Activation::IWebAuthenticationBrokerContinuationEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <::winrt_applicationmodel::Activation::IWebAuthenticationBrokerContinuationEventArgs as ::windows_core::Interface>::IID;
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::RuntimeName for WebUIWebAuthenticationBrokerContinuationEventArgs {
    const NAME: &'static str = "Windows.UI.WebUI.WebUIWebAuthenticationBrokerContinuationEventArgs";
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<WebUIWebAuthenticationBrokerContinuationEventArgs> for ::windows_core::IUnknown {
    fn from(value: WebUIWebAuthenticationBrokerContinuationEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<&WebUIWebAuthenticationBrokerContinuationEventArgs> for ::windows_core::IUnknown {
    fn from(value: &WebUIWebAuthenticationBrokerContinuationEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for WebUIWebAuthenticationBrokerContinuationEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a WebUIWebAuthenticationBrokerContinuationEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<WebUIWebAuthenticationBrokerContinuationEventArgs> for ::windows_core::IInspectable {
    fn from(value: WebUIWebAuthenticationBrokerContinuationEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<&WebUIWebAuthenticationBrokerContinuationEventArgs> for ::windows_core::IInspectable {
    fn from(value: &WebUIWebAuthenticationBrokerContinuationEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for WebUIWebAuthenticationBrokerContinuationEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a WebUIWebAuthenticationBrokerContinuationEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIWebAuthenticationBrokerContinuationEventArgs> for ::winrt_applicationmodel::Activation::IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: WebUIWebAuthenticationBrokerContinuationEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIWebAuthenticationBrokerContinuationEventArgs> for ::winrt_applicationmodel::Activation::IActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &WebUIWebAuthenticationBrokerContinuationEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgs> for WebUIWebAuthenticationBrokerContinuationEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgs> for &WebUIWebAuthenticationBrokerContinuationEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IActivatedEventArgs> {
        ::core::convert::TryInto::<::winrt_applicationmodel::Activation::IActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIWebAuthenticationBrokerContinuationEventArgs> for IActivatedEventArgsDeferral {
    type Error = ::windows_core::Error;
    fn try_from(value: WebUIWebAuthenticationBrokerContinuationEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIWebAuthenticationBrokerContinuationEventArgs> for IActivatedEventArgsDeferral {
    type Error = ::windows_core::Error;
    fn try_from(value: &WebUIWebAuthenticationBrokerContinuationEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgsDeferral> for WebUIWebAuthenticationBrokerContinuationEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgsDeferral> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, IActivatedEventArgsDeferral> for &WebUIWebAuthenticationBrokerContinuationEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, IActivatedEventArgsDeferral> {
        ::core::convert::TryInto::<IActivatedEventArgsDeferral>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIWebAuthenticationBrokerContinuationEventArgs> for ::winrt_applicationmodel::Activation::IContinuationActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: WebUIWebAuthenticationBrokerContinuationEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIWebAuthenticationBrokerContinuationEventArgs> for ::winrt_applicationmodel::Activation::IContinuationActivatedEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &WebUIWebAuthenticationBrokerContinuationEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IContinuationActivatedEventArgs> for WebUIWebAuthenticationBrokerContinuationEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IContinuationActivatedEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IContinuationActivatedEventArgs> for &WebUIWebAuthenticationBrokerContinuationEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IContinuationActivatedEventArgs> {
        ::core::convert::TryInto::<::winrt_applicationmodel::Activation::IContinuationActivatedEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIWebAuthenticationBrokerContinuationEventArgs> for ::winrt_applicationmodel::Activation::IWebAuthenticationBrokerContinuationEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: WebUIWebAuthenticationBrokerContinuationEventArgs) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIWebAuthenticationBrokerContinuationEventArgs> for ::winrt_applicationmodel::Activation::IWebAuthenticationBrokerContinuationEventArgs {
    type Error = ::windows_core::Error;
    fn try_from(value: &WebUIWebAuthenticationBrokerContinuationEventArgs) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IWebAuthenticationBrokerContinuationEventArgs> for WebUIWebAuthenticationBrokerContinuationEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IWebAuthenticationBrokerContinuationEventArgs> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_applicationmodel::Activation::IWebAuthenticationBrokerContinuationEventArgs> for &WebUIWebAuthenticationBrokerContinuationEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_applicationmodel::Activation::IWebAuthenticationBrokerContinuationEventArgs> {
        ::core::convert::TryInto::<::winrt_applicationmodel::Activation::IWebAuthenticationBrokerContinuationEventArgs>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
