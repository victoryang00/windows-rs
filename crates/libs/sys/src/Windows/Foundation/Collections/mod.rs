#[doc = "*Required features: `\"Foundation_Collections\"`*"]
#[repr(transparent)]
pub struct CollectionChange(pub i32);
impl CollectionChange {
    pub const Reset: Self = Self(0i32);
    pub const ItemInserted: Self = Self(1i32);
    pub const ItemRemoved: Self = Self(2i32);
    pub const ItemChanged: Self = Self(3i32);
}
impl ::core::marker::Copy for CollectionChange {}
impl ::core::clone::Clone for CollectionChange {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct IIterable<T> {
    pub base__: ::windows_sys::core::IInspectable,
    pub First: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub T: ::core::marker::PhantomData<T>,
}
#[repr(C)]
pub struct IIterator<T> {
    pub base__: ::windows_sys::core::IInspectable,
    pub Current: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut T) -> ::windows_sys::core::HRESULT,
    pub HasCurrent: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub MoveNext: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub GetMany: unsafe extern "system" fn(this: *mut *mut Self, items_array_size: u32, items: *mut T, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub T: ::core::marker::PhantomData<T>,
}
#[repr(C)]
pub struct IKeyValuePair<K, V> {
    pub base__: ::windows_sys::core::IInspectable,
    pub Key: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut K) -> ::windows_sys::core::HRESULT,
    pub Value: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut V) -> ::windows_sys::core::HRESULT,
    pub K: ::core::marker::PhantomData<K>,
    pub V: ::core::marker::PhantomData<V>,
}
#[repr(C)]
pub struct IMap<K, V> {
    pub base__: ::windows_sys::core::IInspectable,
    pub Lookup: unsafe extern "system" fn(this: *mut *mut Self, key: K, result__: *mut V) -> ::windows_sys::core::HRESULT,
    pub Size: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub HasKey: unsafe extern "system" fn(this: *mut *mut Self, key: K, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub GetView: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Insert: unsafe extern "system" fn(this: *mut *mut Self, key: K, value: V, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub Remove: unsafe extern "system" fn(this: *mut *mut Self, key: K) -> ::windows_sys::core::HRESULT,
    pub Clear: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub K: ::core::marker::PhantomData<K>,
    pub V: ::core::marker::PhantomData<V>,
}
#[repr(C)]
pub struct IMapChangedEventArgs<K> {
    pub base__: ::windows_sys::core::IInspectable,
    pub CollectionChange: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut CollectionChange) -> ::windows_sys::core::HRESULT,
    pub Key: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut K) -> ::windows_sys::core::HRESULT,
    pub K: ::core::marker::PhantomData<K>,
}
#[repr(C)]
pub struct IMapView<K, V> {
    pub base__: ::windows_sys::core::IInspectable,
    pub Lookup: unsafe extern "system" fn(this: *mut *mut Self, key: K, result__: *mut V) -> ::windows_sys::core::HRESULT,
    pub Size: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub HasKey: unsafe extern "system" fn(this: *mut *mut Self, key: K, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub Split: unsafe extern "system" fn(this: *mut *mut Self, first: *mut *mut ::core::ffi::c_void, second: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub K: ::core::marker::PhantomData<K>,
    pub V: ::core::marker::PhantomData<V>,
}
#[repr(C)]
pub struct IObservableMap<K, V> {
    pub base__: ::windows_sys::core::IInspectable,
    pub MapChanged: unsafe extern "system" fn(this: *mut *mut Self, vhnd: *mut ::core::ffi::c_void, result__: *mut super::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    pub RemoveMapChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    pub K: ::core::marker::PhantomData<K>,
    pub V: ::core::marker::PhantomData<V>,
}
#[repr(C)]
pub struct IObservableVector<T> {
    pub base__: ::windows_sys::core::IInspectable,
    pub VectorChanged: unsafe extern "system" fn(this: *mut *mut Self, vhnd: *mut ::core::ffi::c_void, result__: *mut super::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    pub RemoveVectorChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    pub T: ::core::marker::PhantomData<T>,
}
#[repr(C)]
pub struct IPropertySet {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IVector<T> {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetAt: unsafe extern "system" fn(this: *mut *mut Self, index: u32, result__: *mut T) -> ::windows_sys::core::HRESULT,
    pub Size: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetView: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub IndexOf: unsafe extern "system" fn(this: *mut *mut Self, value: T, index: *mut u32, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetAt: unsafe extern "system" fn(this: *mut *mut Self, index: u32, value: T) -> ::windows_sys::core::HRESULT,
    pub InsertAt: unsafe extern "system" fn(this: *mut *mut Self, index: u32, value: T) -> ::windows_sys::core::HRESULT,
    pub RemoveAt: unsafe extern "system" fn(this: *mut *mut Self, index: u32) -> ::windows_sys::core::HRESULT,
    pub Append: unsafe extern "system" fn(this: *mut *mut Self, value: T) -> ::windows_sys::core::HRESULT,
    pub RemoveAtEnd: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Clear: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub GetMany: unsafe extern "system" fn(this: *mut *mut Self, startindex: u32, items_array_size: u32, items: *mut T, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub ReplaceAll: unsafe extern "system" fn(this: *mut *mut Self, items_array_size: u32, items: *const T) -> ::windows_sys::core::HRESULT,
    pub T: ::core::marker::PhantomData<T>,
}
#[repr(C)]
pub struct IVectorChangedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub CollectionChange: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut CollectionChange) -> ::windows_sys::core::HRESULT,
    pub Index: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IVectorView<T> {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetAt: unsafe extern "system" fn(this: *mut *mut Self, index: u32, result__: *mut T) -> ::windows_sys::core::HRESULT,
    pub Size: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub IndexOf: unsafe extern "system" fn(this: *mut *mut Self, value: T, index: *mut u32, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub GetMany: unsafe extern "system" fn(this: *mut *mut Self, startindex: u32, items_array_size: u32, items: *mut T, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub T: ::core::marker::PhantomData<T>,
}
pub type MapChangedEventHandler = *mut ::core::ffi::c_void;
pub type PropertySet = *mut ::core::ffi::c_void;
pub type StringMap = *mut ::core::ffi::c_void;
pub type ValueSet = *mut ::core::ffi::c_void;
pub type VectorChangedEventHandler = *mut ::core::ffi::c_void;
