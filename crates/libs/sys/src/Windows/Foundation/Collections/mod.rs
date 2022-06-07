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
impl ::windows_sys::core::Interface for IIterable<T> {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4205151722, data2: 25108, data3: 16919, data4: [175, 218, 127, 70, 222, 88, 105, 179] };
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
impl ::windows_sys::core::Interface for IIterator<T> {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1786374243, data2: 17152, data3: 17818, data4: [153, 102, 203, 182, 96, 150, 62, 225] };
}
#[repr(C)]
pub struct IKeyValuePair<K, V> {
    pub base__: ::windows_sys::core::IInspectable,
    pub Key: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut K) -> ::windows_sys::core::HRESULT,
    pub Value: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut V) -> ::windows_sys::core::HRESULT,
    pub K: ::core::marker::PhantomData<K>,
    pub V: ::core::marker::PhantomData<V>,
}
impl ::windows_sys::core::Interface for IKeyValuePair<K, V> {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 45422889, data2: 49604, data3: 19070, data4: [137, 64, 3, 18, 181, 193, 133, 0] };
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
impl ::windows_sys::core::Interface for IMap<K, V> {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1009329662, data2: 34073, data3: 17857, data4: [170, 121, 25, 123, 103, 24, 193, 193] };
}
#[repr(C)]
pub struct IMapChangedEventArgs<K> {
    pub base__: ::windows_sys::core::IInspectable,
    pub CollectionChange: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut CollectionChange) -> ::windows_sys::core::HRESULT,
    pub Key: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut K) -> ::windows_sys::core::HRESULT,
    pub K: ::core::marker::PhantomData<K>,
}
impl ::windows_sys::core::Interface for IMapChangedEventArgs<K> {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2570712287, data2: 1290, data3: 19471, data4: [170, 96, 119, 7, 95, 156, 71, 119] };
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
impl ::windows_sys::core::Interface for IMapView<K, V> {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3833646656, data2: 41784, data3: 19162, data4: [173, 207, 39, 34, 114, 228, 140, 185] };
}
#[repr(C)]
pub struct IObservableMap<K, V> {
    pub base__: ::windows_sys::core::IInspectable,
    pub MapChanged: unsafe extern "system" fn(this: *mut *mut Self, vhnd: *mut ::core::ffi::c_void, result__: *mut super::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    pub RemoveMapChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    pub K: ::core::marker::PhantomData<K>,
    pub V: ::core::marker::PhantomData<V>,
}
impl ::windows_sys::core::Interface for IObservableMap<K, V> {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1709124597, data2: 48953, data3: 16821, data4: [174, 188, 90, 157, 134, 94, 71, 43] };
}
#[repr(C)]
pub struct IObservableVector<T> {
    pub base__: ::windows_sys::core::IInspectable,
    pub VectorChanged: unsafe extern "system" fn(this: *mut *mut Self, vhnd: *mut ::core::ffi::c_void, result__: *mut super::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    pub RemoveVectorChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    pub T: ::core::marker::PhantomData<T>,
}
impl ::windows_sys::core::Interface for IObservableVector<T> {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1494739795, data2: 20660, data3: 18957, data4: [179, 9, 101, 134, 43, 63, 29, 188] };
}
#[repr(C)]
pub struct IPropertySet {
    pub base__: ::windows_sys::core::IInspectable,
}
impl ::windows_sys::core::Interface for IPropertySet {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2319707551, data2: 62694, data3: 17441, data4: [172, 249, 29, 171, 41, 134, 130, 12] };
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
impl ::windows_sys::core::Interface for IVector<T> {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2436052969, data2: 4513, data3: 17221, data4: [163, 162, 78, 127, 149, 110, 34, 45] };
}
#[repr(C)]
pub struct IVectorChangedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub CollectionChange: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut CollectionChange) -> ::windows_sys::core::HRESULT,
    pub Index: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IVectorChangedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1465463775, data2: 13566, data3: 17536, data4: [175, 21, 7, 105, 31, 61, 93, 155] };
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
impl ::windows_sys::core::Interface for IVectorView<T> {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3152149068, data2: 45283, data3: 17795, data4: [186, 239, 31, 27, 46, 72, 62, 86] };
}
pub type MapChangedEventHandler = *mut ::core::ffi::c_void;
pub type PropertySet = *mut ::core::ffi::c_void;
pub type StringMap = *mut ::core::ffi::c_void;
pub type ValueSet = *mut ::core::ffi::c_void;
pub type VectorChangedEventHandler = *mut ::core::ffi::c_void;
