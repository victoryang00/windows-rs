#[repr(transparent)]
pub struct AddPagesEventArgs(::windows_core::IUnknown);
impl AddPagesEventArgs {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<AddPagesEventArgs, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "winrt-graphics")]
    pub fn PrintTaskOptions(&self) -> ::windows_core::Result<::winrt_graphics::Printing::PrintTaskOptions> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).PrintTaskOptions)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_graphics::Printing::PrintTaskOptions>(result__)
        }
    }
}
impl ::core::clone::Clone for AddPagesEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AddPagesEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AddPagesEventArgs {}
impl ::core::fmt::Debug for AddPagesEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AddPagesEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for AddPagesEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Printing.AddPagesEventArgs;{e2e52be5-056c-4420-9795-cb3526ce0c20})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for AddPagesEventArgs {
    type Vtable = IAddPagesEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IAddPagesEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for AddPagesEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Printing.AddPagesEventArgs";
}
impl ::core::convert::From<AddPagesEventArgs> for ::windows_core::IUnknown {
    fn from(value: AddPagesEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AddPagesEventArgs> for ::windows_core::IUnknown {
    fn from(value: &AddPagesEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for AddPagesEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a AddPagesEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AddPagesEventArgs> for ::windows_core::IInspectable {
    fn from(value: AddPagesEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AddPagesEventArgs> for ::windows_core::IInspectable {
    fn from(value: &AddPagesEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for AddPagesEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a AddPagesEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AddPagesEventArgs {}
unsafe impl ::core::marker::Sync for AddPagesEventArgs {}
#[repr(transparent)]
pub struct AddPagesEventHandler(pub ::windows_core::IUnknown);
impl AddPagesEventHandler {
    pub fn new<F: FnMut(&::core::option::Option<::windows_core::IInspectable>, &::core::option::Option<AddPagesEventArgs>) -> ::windows_core::Result<()> + ::core::marker::Send + 'static>(invoke: F) -> Self {
        let com = AddPagesEventHandlerBox::<F> { vtable: &AddPagesEventHandlerBox::<F>::VTABLE, count: ::windows_core::RefCount::new(1), invoke };
        unsafe { ::core::mem::transmute(::windows_core::alloc::boxed::Box::new(com)) }
    }
    pub fn Invoke<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::IInspectable>, Param1: ::windows_core::IntoParam<'a, AddPagesEventArgs>>(&self, sender: Param0, e: Param1) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Invoke)(::windows_core::Interface::as_raw(this), sender.into_param().abi(), e.into_param().abi()).ok() }
    }
}
#[repr(C)]
struct AddPagesEventHandlerBox<F: FnMut(&::core::option::Option<::windows_core::IInspectable>, &::core::option::Option<AddPagesEventArgs>) -> ::windows_core::Result<()> + ::core::marker::Send + 'static> {
    vtable: *const AddPagesEventHandler_Vtbl,
    invoke: F,
    count: ::windows_core::RefCount,
}
impl<F: FnMut(&::core::option::Option<::windows_core::IInspectable>, &::core::option::Option<AddPagesEventArgs>) -> ::windows_core::Result<()> + ::core::marker::Send + 'static> AddPagesEventHandlerBox<F> {
    const VTABLE: AddPagesEventHandler_Vtbl = AddPagesEventHandler_Vtbl {
        base__: ::windows_core::IUnknownVtbl { QueryInterface: Self::QueryInterface, AddRef: Self::AddRef, Release: Self::Release },
        Invoke: Self::Invoke,
    };
    unsafe extern "system" fn QueryInterface(this: ::windows_core::RawPtr, iid: &::windows_core::GUID, interface: *mut *const ::core::ffi::c_void) -> ::windows_core::HRESULT {
        let this = this as *mut ::windows_core::RawPtr as *mut Self;
        *interface = if iid == &<AddPagesEventHandler as ::windows_core::Interface>::IID || iid == &<::windows_core::IUnknown as ::windows_core::Interface>::IID || iid == &<::windows_core::IAgileObject as ::windows_core::Interface>::IID { &mut (*this).vtable as *mut _ as _ } else { ::core::ptr::null_mut() };
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
impl ::core::clone::Clone for AddPagesEventHandler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AddPagesEventHandler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AddPagesEventHandler {}
impl ::core::fmt::Debug for AddPagesEventHandler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AddPagesEventHandler").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for AddPagesEventHandler {
    type Vtable = AddPagesEventHandler_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd4b57970_57a0_4209_847c_c093b54bc729);
}
unsafe impl ::windows_core::RuntimeType for AddPagesEventHandler {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"{d4b57970-57a0-4209-847c-c093b54bc729}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct AddPagesEventHandler_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub Invoke: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sender: *mut ::core::ffi::c_void, e: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct GetPreviewPageEventArgs(::windows_core::IUnknown);
impl GetPreviewPageEventArgs {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<GetPreviewPageEventArgs, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn PageNumber(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows_core::Interface::vtable(this).PageNumber)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
}
impl ::core::clone::Clone for GetPreviewPageEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for GetPreviewPageEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GetPreviewPageEventArgs {}
impl ::core::fmt::Debug for GetPreviewPageEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GetPreviewPageEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for GetPreviewPageEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Printing.GetPreviewPageEventArgs;{a43d703d-dea9-4df6-a7ed-35049cd485c7})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for GetPreviewPageEventArgs {
    type Vtable = IGetPreviewPageEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IGetPreviewPageEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for GetPreviewPageEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Printing.GetPreviewPageEventArgs";
}
impl ::core::convert::From<GetPreviewPageEventArgs> for ::windows_core::IUnknown {
    fn from(value: GetPreviewPageEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&GetPreviewPageEventArgs> for ::windows_core::IUnknown {
    fn from(value: &GetPreviewPageEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for GetPreviewPageEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a GetPreviewPageEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<GetPreviewPageEventArgs> for ::windows_core::IInspectable {
    fn from(value: GetPreviewPageEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&GetPreviewPageEventArgs> for ::windows_core::IInspectable {
    fn from(value: &GetPreviewPageEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for GetPreviewPageEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a GetPreviewPageEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for GetPreviewPageEventArgs {}
unsafe impl ::core::marker::Sync for GetPreviewPageEventArgs {}
#[repr(transparent)]
pub struct GetPreviewPageEventHandler(pub ::windows_core::IUnknown);
impl GetPreviewPageEventHandler {
    pub fn new<F: FnMut(&::core::option::Option<::windows_core::IInspectable>, &::core::option::Option<GetPreviewPageEventArgs>) -> ::windows_core::Result<()> + ::core::marker::Send + 'static>(invoke: F) -> Self {
        let com = GetPreviewPageEventHandlerBox::<F> { vtable: &GetPreviewPageEventHandlerBox::<F>::VTABLE, count: ::windows_core::RefCount::new(1), invoke };
        unsafe { ::core::mem::transmute(::windows_core::alloc::boxed::Box::new(com)) }
    }
    pub fn Invoke<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::IInspectable>, Param1: ::windows_core::IntoParam<'a, GetPreviewPageEventArgs>>(&self, sender: Param0, e: Param1) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Invoke)(::windows_core::Interface::as_raw(this), sender.into_param().abi(), e.into_param().abi()).ok() }
    }
}
#[repr(C)]
struct GetPreviewPageEventHandlerBox<F: FnMut(&::core::option::Option<::windows_core::IInspectable>, &::core::option::Option<GetPreviewPageEventArgs>) -> ::windows_core::Result<()> + ::core::marker::Send + 'static> {
    vtable: *const GetPreviewPageEventHandler_Vtbl,
    invoke: F,
    count: ::windows_core::RefCount,
}
impl<F: FnMut(&::core::option::Option<::windows_core::IInspectable>, &::core::option::Option<GetPreviewPageEventArgs>) -> ::windows_core::Result<()> + ::core::marker::Send + 'static> GetPreviewPageEventHandlerBox<F> {
    const VTABLE: GetPreviewPageEventHandler_Vtbl = GetPreviewPageEventHandler_Vtbl {
        base__: ::windows_core::IUnknownVtbl { QueryInterface: Self::QueryInterface, AddRef: Self::AddRef, Release: Self::Release },
        Invoke: Self::Invoke,
    };
    unsafe extern "system" fn QueryInterface(this: ::windows_core::RawPtr, iid: &::windows_core::GUID, interface: *mut *const ::core::ffi::c_void) -> ::windows_core::HRESULT {
        let this = this as *mut ::windows_core::RawPtr as *mut Self;
        *interface = if iid == &<GetPreviewPageEventHandler as ::windows_core::Interface>::IID || iid == &<::windows_core::IUnknown as ::windows_core::Interface>::IID || iid == &<::windows_core::IAgileObject as ::windows_core::Interface>::IID { &mut (*this).vtable as *mut _ as _ } else { ::core::ptr::null_mut() };
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
impl ::core::clone::Clone for GetPreviewPageEventHandler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for GetPreviewPageEventHandler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GetPreviewPageEventHandler {}
impl ::core::fmt::Debug for GetPreviewPageEventHandler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GetPreviewPageEventHandler").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for GetPreviewPageEventHandler {
    type Vtable = GetPreviewPageEventHandler_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xccb3e9ed_9c11_4e50_ab49_e98086bbfdef);
}
unsafe impl ::windows_core::RuntimeType for GetPreviewPageEventHandler {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"{ccb3e9ed-9c11-4e50-ab49-e98086bbfdef}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct GetPreviewPageEventHandler_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub Invoke: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sender: *mut ::core::ffi::c_void, e: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAddPagesEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAddPagesEventArgs {
    type Vtable = IAddPagesEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe2e52be5_056c_4420_9795_cb3526ce0c20);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAddPagesEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-graphics")]
    pub PrintTaskOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-graphics"))]
    PrintTaskOptions: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGetPreviewPageEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGetPreviewPageEventArgs {
    type Vtable = IGetPreviewPageEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa43d703d_dea9_4df6_a7ed_35049cd485c7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGetPreviewPageEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub PageNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPaginateEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPaginateEventArgs {
    type Vtable = IPaginateEventArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xed945fd6_79ab_42b7_930a_3d6e09011d21);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPaginateEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-graphics")]
    pub PrintTaskOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-graphics"))]
    PrintTaskOptions: usize,
    pub CurrentPreviewPageNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPrintDocument(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPrintDocument {
    type Vtable = IPrintDocument_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe44327c3_a999_485b_b1d8_72dc517821e6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintDocument_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-graphics")]
    pub DocumentSource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-graphics"))]
    DocumentSource: usize,
    pub Paginate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemovePaginate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub GetPreviewPage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveGetPreviewPage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub AddPages: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveAddPages: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub AddPage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pagevisual: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub AddPagesComplete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetPreviewPageCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: i32, r#type: PreviewPageCountType) -> ::windows_core::HRESULT,
    pub SetPreviewPage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pagenumber: i32, pagevisual: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub InvalidatePreview: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPrintDocumentFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPrintDocumentFactory {
    type Vtable = IPrintDocumentFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xfb87b18f_2606_4a2f_99d4_a7cdbc35d7c7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintDocumentFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CreateInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPrintDocumentStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPrintDocumentStatics {
    type Vtable = IPrintDocumentStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xfd970a3c_b152_49e0_a6bd_6aa6477e43c7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintDocumentStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub DocumentSourceProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct PaginateEventArgs(::windows_core::IUnknown);
impl PaginateEventArgs {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<PaginateEventArgs, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "winrt-graphics")]
    pub fn PrintTaskOptions(&self) -> ::windows_core::Result<::winrt_graphics::Printing::PrintTaskOptions> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).PrintTaskOptions)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_graphics::Printing::PrintTaskOptions>(result__)
        }
    }
    pub fn CurrentPreviewPageNumber(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows_core::Interface::vtable(this).CurrentPreviewPageNumber)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
}
impl ::core::clone::Clone for PaginateEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PaginateEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PaginateEventArgs {}
impl ::core::fmt::Debug for PaginateEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PaginateEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PaginateEventArgs {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Printing.PaginateEventArgs;{ed945fd6-79ab-42b7-930a-3d6e09011d21})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for PaginateEventArgs {
    type Vtable = IPaginateEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IPaginateEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for PaginateEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Printing.PaginateEventArgs";
}
impl ::core::convert::From<PaginateEventArgs> for ::windows_core::IUnknown {
    fn from(value: PaginateEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PaginateEventArgs> for ::windows_core::IUnknown {
    fn from(value: &PaginateEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for PaginateEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a PaginateEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PaginateEventArgs> for ::windows_core::IInspectable {
    fn from(value: PaginateEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PaginateEventArgs> for ::windows_core::IInspectable {
    fn from(value: &PaginateEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for PaginateEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a PaginateEventArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for PaginateEventArgs {}
unsafe impl ::core::marker::Sync for PaginateEventArgs {}
#[repr(transparent)]
pub struct PaginateEventHandler(pub ::windows_core::IUnknown);
impl PaginateEventHandler {
    pub fn new<F: FnMut(&::core::option::Option<::windows_core::IInspectable>, &::core::option::Option<PaginateEventArgs>) -> ::windows_core::Result<()> + ::core::marker::Send + 'static>(invoke: F) -> Self {
        let com = PaginateEventHandlerBox::<F> { vtable: &PaginateEventHandlerBox::<F>::VTABLE, count: ::windows_core::RefCount::new(1), invoke };
        unsafe { ::core::mem::transmute(::windows_core::alloc::boxed::Box::new(com)) }
    }
    pub fn Invoke<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::IInspectable>, Param1: ::windows_core::IntoParam<'a, PaginateEventArgs>>(&self, sender: Param0, e: Param1) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Invoke)(::windows_core::Interface::as_raw(this), sender.into_param().abi(), e.into_param().abi()).ok() }
    }
}
#[repr(C)]
struct PaginateEventHandlerBox<F: FnMut(&::core::option::Option<::windows_core::IInspectable>, &::core::option::Option<PaginateEventArgs>) -> ::windows_core::Result<()> + ::core::marker::Send + 'static> {
    vtable: *const PaginateEventHandler_Vtbl,
    invoke: F,
    count: ::windows_core::RefCount,
}
impl<F: FnMut(&::core::option::Option<::windows_core::IInspectable>, &::core::option::Option<PaginateEventArgs>) -> ::windows_core::Result<()> + ::core::marker::Send + 'static> PaginateEventHandlerBox<F> {
    const VTABLE: PaginateEventHandler_Vtbl = PaginateEventHandler_Vtbl {
        base__: ::windows_core::IUnknownVtbl { QueryInterface: Self::QueryInterface, AddRef: Self::AddRef, Release: Self::Release },
        Invoke: Self::Invoke,
    };
    unsafe extern "system" fn QueryInterface(this: ::windows_core::RawPtr, iid: &::windows_core::GUID, interface: *mut *const ::core::ffi::c_void) -> ::windows_core::HRESULT {
        let this = this as *mut ::windows_core::RawPtr as *mut Self;
        *interface = if iid == &<PaginateEventHandler as ::windows_core::Interface>::IID || iid == &<::windows_core::IUnknown as ::windows_core::Interface>::IID || iid == &<::windows_core::IAgileObject as ::windows_core::Interface>::IID { &mut (*this).vtable as *mut _ as _ } else { ::core::ptr::null_mut() };
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
impl ::core::clone::Clone for PaginateEventHandler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PaginateEventHandler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PaginateEventHandler {}
impl ::core::fmt::Debug for PaginateEventHandler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PaginateEventHandler").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for PaginateEventHandler {
    type Vtable = PaginateEventHandler_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0cc05b61_811b_4a32_9965_13eb78dbb01b);
}
unsafe impl ::windows_core::RuntimeType for PaginateEventHandler {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"{0cc05b61-811b-4a32-9965-13eb78dbb01b}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct PaginateEventHandler_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub Invoke: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sender: *mut ::core::ffi::c_void, e: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PreviewPageCountType(pub i32);
impl PreviewPageCountType {
    pub const Final: Self = Self(0i32);
    pub const Intermediate: Self = Self(1i32);
}
impl ::core::marker::Copy for PreviewPageCountType {}
impl ::core::clone::Clone for PreviewPageCountType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PreviewPageCountType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for PreviewPageCountType {
    type Abi = Self;
}
impl ::core::fmt::Debug for PreviewPageCountType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PreviewPageCountType").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PreviewPageCountType {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Printing.PreviewPageCountType;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct PrintDocument(::windows_core::IUnknown);
impl PrintDocument {
    #[cfg(feature = "winrt-graphics")]
    pub fn DocumentSource(&self) -> ::windows_core::Result<::winrt_graphics::Printing::IPrintDocumentSource> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).DocumentSource)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_graphics::Printing::IPrintDocumentSource>(result__)
        }
    }
    pub fn Paginate<'a, Param0: ::windows_core::IntoParam<'a, PaginateEventHandler>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).Paginate)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemovePaginate<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemovePaginate)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn GetPreviewPage<'a, Param0: ::windows_core::IntoParam<'a, GetPreviewPageEventHandler>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).GetPreviewPage)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveGetPreviewPage<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveGetPreviewPage)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn AddPages<'a, Param0: ::windows_core::IntoParam<'a, AddPagesEventHandler>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).AddPages)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveAddPages<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveAddPages)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    pub fn AddPage<'a, Param0: ::windows_core::IntoParam<'a, super::UIElement>>(&self, pagevisual: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).AddPage)(::windows_core::Interface::as_raw(this), pagevisual.into_param().abi()).ok() }
    }
    pub fn AddPagesComplete(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).AddPagesComplete)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn SetPreviewPageCount(&self, count: i32, r#type: PreviewPageCountType) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetPreviewPageCount)(::windows_core::Interface::as_raw(this), count, r#type).ok() }
    }
    pub fn SetPreviewPage<'a, Param1: ::windows_core::IntoParam<'a, super::UIElement>>(&self, pagenumber: i32, pagevisual: Param1) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetPreviewPage)(::windows_core::Interface::as_raw(this), pagenumber, pagevisual.into_param().abi()).ok() }
    }
    pub fn InvalidatePreview(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).InvalidatePreview)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn new() -> ::windows_core::Result<PrintDocument> {
        Self::IPrintDocumentFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateInstance)(::windows_core::Interface::as_raw(this), ::core::ptr::null_mut(), &mut ::core::option::Option::<::windows_core::IInspectable>::None as *mut _ as _, result__.as_mut_ptr()).from_abi::<PrintDocument>(result__)
        })
    }
    pub fn compose<T: ::windows_core::Compose>(compose: T) -> ::windows_core::Result<PrintDocument> {
        Self::IPrintDocumentFactory(|this| unsafe {
            let (derived__, base__) = ::windows_core::Compose::compose(compose);
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateInstance)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(&derived__), base__ as *mut _ as _, result__.as_mut_ptr()).from_abi::<PrintDocument>(result__)
        })
    }
    pub fn DocumentSourceProperty() -> ::windows_core::Result<super::DependencyProperty> {
        Self::IPrintDocumentStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).DocumentSourceProperty)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::DependencyProperty>(result__)
        })
    }
    pub fn IPrintDocumentFactory<R, F: FnOnce(&IPrintDocumentFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<PrintDocument, IPrintDocumentFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IPrintDocumentStatics<R, F: FnOnce(&IPrintDocumentStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<PrintDocument, IPrintDocumentStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for PrintDocument {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PrintDocument {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PrintDocument {}
impl ::core::fmt::Debug for PrintDocument {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PrintDocument").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for PrintDocument {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Printing.PrintDocument;{e44327c3-a999-485b-b1d8-72dc517821e6})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for PrintDocument {
    type Vtable = IPrintDocument_Vtbl;
    const IID: ::windows_core::GUID = <IPrintDocument as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for PrintDocument {
    const NAME: &'static str = "Windows.UI.Xaml.Printing.PrintDocument";
}
impl ::core::convert::From<PrintDocument> for ::windows_core::IUnknown {
    fn from(value: PrintDocument) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PrintDocument> for ::windows_core::IUnknown {
    fn from(value: &PrintDocument) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for PrintDocument {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a PrintDocument {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PrintDocument> for ::windows_core::IInspectable {
    fn from(value: PrintDocument) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PrintDocument> for ::windows_core::IInspectable {
    fn from(value: &PrintDocument) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for PrintDocument {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a PrintDocument {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PrintDocument> for super::DependencyObject {
    fn from(value: PrintDocument) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&PrintDocument> for super::DependencyObject {
    fn from(value: &PrintDocument) -> Self {
        ::windows_core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::DependencyObject> for PrintDocument {
    fn into_param(self) -> ::windows_core::Param<'a, super::DependencyObject> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, super::DependencyObject> for &PrintDocument {
    fn into_param(self) -> ::windows_core::Param<'a, super::DependencyObject> {
        ::windows_core::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for PrintDocument {}
unsafe impl ::core::marker::Sync for PrintDocument {}
