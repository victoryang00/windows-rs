pub type BindableVectorChangedEventHandler = *mut ::core::ffi::c_void;
#[repr(C)]
pub struct IBindableIterable {
    pub base__: ::windows_sys::core::IInspectable,
    pub First: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IBindableIterator {
    pub base__: ::windows_sys::core::IInspectable,
    pub Current: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub HasCurrent: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub MoveNext: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IBindableObservableVector {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub VectorChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    VectorChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveVectorChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveVectorChanged: usize,
}
#[repr(C)]
pub struct IBindableVector {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetAt: unsafe extern "system" fn(this: *mut *mut Self, index: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Size: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetView: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub IndexOf: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void, index: *mut u32, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetAt: unsafe extern "system" fn(this: *mut *mut Self, index: u32, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub InsertAt: unsafe extern "system" fn(this: *mut *mut Self, index: u32, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub RemoveAt: unsafe extern "system" fn(this: *mut *mut Self, index: u32) -> ::windows_sys::core::HRESULT,
    pub Append: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub RemoveAtEnd: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Clear: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IBindableVectorView {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetAt: unsafe extern "system" fn(this: *mut *mut Self, index: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Size: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub IndexOf: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void, index: *mut u32, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct INotifyCollectionChanged {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub CollectionChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CollectionChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveCollectionChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveCollectionChanged: usize,
}
#[repr(C)]
pub struct INotifyCollectionChangedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Action: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut NotifyCollectionChangedAction) -> ::windows_sys::core::HRESULT,
    pub NewItems: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub OldItems: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub NewStartingIndex: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub OldStartingIndex: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct INotifyCollectionChangedEventArgsFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstanceWithAllParameters: unsafe extern "system" fn(this: *mut *mut Self, action: NotifyCollectionChangedAction, newitems: *mut ::core::ffi::c_void, olditems: *mut ::core::ffi::c_void, newindex: i32, oldindex: i32, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[doc = "*Required features: `\"UI_Xaml_Interop\"`*"]
#[repr(transparent)]
pub struct NotifyCollectionChangedAction(pub i32);
impl NotifyCollectionChangedAction {
    pub const Add: Self = Self(0i32);
    pub const Remove: Self = Self(1i32);
    pub const Replace: Self = Self(2i32);
    pub const Move: Self = Self(3i32);
    pub const Reset: Self = Self(4i32);
}
impl ::core::marker::Copy for NotifyCollectionChangedAction {}
impl ::core::clone::Clone for NotifyCollectionChangedAction {
    fn clone(&self) -> Self {
        *self
    }
}
pub type NotifyCollectionChangedEventArgs = *mut ::core::ffi::c_void;
pub type NotifyCollectionChangedEventHandler = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"UI_Xaml_Interop\"`*"]
#[repr(transparent)]
pub struct TypeKind(pub i32);
impl TypeKind {
    pub const Primitive: Self = Self(0i32);
    pub const Metadata: Self = Self(1i32);
    pub const Custom: Self = Self(2i32);
}
impl ::core::marker::Copy for TypeKind {}
impl ::core::clone::Clone for TypeKind {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"UI_Xaml_Interop\"`*"]
pub struct TypeName {
    pub Name: ::windows_sys::core::HSTRING,
    pub Kind: TypeKind,
}
impl ::core::marker::Copy for TypeName {}
impl ::core::clone::Clone for TypeName {
    fn clone(&self) -> Self {
        *self
    }
}
