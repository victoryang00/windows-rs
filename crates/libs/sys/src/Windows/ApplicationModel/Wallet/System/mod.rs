#[repr(C)]
pub struct IWalletItemSystemStore {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub GetItemsAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetItemsAsync: usize,
    #[cfg(feature = "Foundation")]
    pub DeleteAsync: unsafe extern "system" fn(this: *mut *mut Self, item: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DeleteAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub ImportItemAsync: unsafe extern "system" fn(this: *mut *mut Self, stream: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    ImportItemAsync: usize,
    pub GetAppStatusForItem: unsafe extern "system" fn(this: *mut *mut Self, item: *mut ::core::ffi::c_void, result__: *mut WalletItemAppAssociation) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub LaunchAppForItemAsync: unsafe extern "system" fn(this: *mut *mut Self, item: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LaunchAppForItemAsync: usize,
}
impl ::windows_sys::core::Interface for IWalletItemSystemStore {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1378757631, data2: 38562, data3: 18967, data4: [141, 25, 254, 29, 159, 131, 117, 97] };
}
#[repr(C)]
pub struct IWalletItemSystemStore2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub ItemsChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ItemsChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveItemsChanged: unsafe extern "system" fn(this: *mut *mut Self, cookie: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveItemsChanged: usize,
}
impl ::windows_sys::core::Interface for IWalletItemSystemStore2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4186782286, data2: 48640, data3: 20445, data4: [151, 52, 108, 17, 60, 26, 193, 203] };
}
#[repr(C)]
pub struct IWalletManagerSystemStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub RequestStoreAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestStoreAsync: usize,
}
impl ::windows_sys::core::Interface for IWalletManagerSystemStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3202935689, data2: 9780, data3: 19354, data4: [139, 35, 238, 137, 3, 201, 31, 224] };
}
#[doc = "*Required features: `\"ApplicationModel_Wallet_System\"`*"]
#[repr(transparent)]
pub struct WalletItemAppAssociation(pub i32);
impl WalletItemAppAssociation {
    pub const None: Self = Self(0i32);
    pub const AppInstalled: Self = Self(1i32);
    pub const AppNotInstalled: Self = Self(2i32);
}
impl ::core::marker::Copy for WalletItemAppAssociation {}
impl ::core::clone::Clone for WalletItemAppAssociation {
    fn clone(&self) -> Self {
        *self
    }
}
pub type WalletItemSystemStore = *mut ::core::ffi::c_void;
