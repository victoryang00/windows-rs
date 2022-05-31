#[doc(hidden)]
#[repr(transparent)]
pub struct IWiFiAdapter(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWiFiAdapter {
    type Vtable = IWiFiAdapter_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa6c4e423_3d75_43a4_b9de_11e26b72d9b0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiFiAdapter_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-networking")]
    pub NetworkAdapter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-networking"))]
    NetworkAdapter: usize,
    pub ScanAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub NetworkReport: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub AvailableNetworksChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, args: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveAvailableNetworksChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, eventcookie: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub ConnectAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, availablenetwork: ::windows_core::RawPtr, reconnectionkind: WiFiReconnectionKind, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-security")]
    pub ConnectWithPasswordCredentialAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, availablenetwork: ::windows_core::RawPtr, reconnectionkind: WiFiReconnectionKind, passwordcredential: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-security"))]
    ConnectWithPasswordCredentialAsync: usize,
    #[cfg(feature = "winrt-security")]
    pub ConnectWithPasswordCredentialAndSsidAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, availablenetwork: ::windows_core::RawPtr, reconnectionkind: WiFiReconnectionKind, passwordcredential: ::windows_core::RawPtr, ssid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-security"))]
    ConnectWithPasswordCredentialAndSsidAsync: usize,
    pub Disconnect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWiFiAdapter2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWiFiAdapter2 {
    type Vtable = IWiFiAdapter2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5bc4501d_81e4_453d_9430_1fcafbadd6b6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiFiAdapter2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub GetWpsConfigurationAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, availablenetwork: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-security")]
    pub ConnectWithPasswordCredentialAndSsidAndConnectionMethodAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, availablenetwork: ::windows_core::RawPtr, reconnectionkind: WiFiReconnectionKind, passwordcredential: ::windows_core::RawPtr, ssid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, connectionmethod: WiFiConnectionMethod, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-security"))]
    ConnectWithPasswordCredentialAndSsidAndConnectionMethodAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWiFiAdapterStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWiFiAdapterStatics {
    type Vtable = IWiFiAdapterStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xda25fddd_d24c_43e3_aabd_c4659f730f99);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiFiAdapterStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-foundation")]
    pub FindAllAdaptersAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    FindAllAdaptersAsync: usize,
    pub GetDeviceSelector: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub FromIdAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub RequestAccessAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWiFiAvailableNetwork(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWiFiAvailableNetwork {
    type Vtable = IWiFiAvailableNetwork_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x26e96246_183e_4704_9826_71b4a2f0f668);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiFiAvailableNetwork_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Uptime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::TimeSpan) -> ::windows_core::HRESULT,
    pub Ssid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Bssid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub ChannelCenterFrequencyInKilohertz: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub NetworkRssiInDecibelMilliwatts: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub SignalBars: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows_core::HRESULT,
    pub NetworkKind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut WiFiNetworkKind) -> ::windows_core::HRESULT,
    pub PhyKind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut WiFiPhyKind) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-networking")]
    pub SecuritySettings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-networking"))]
    SecuritySettings: usize,
    pub BeaconInterval: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::TimeSpan) -> ::windows_core::HRESULT,
    pub IsWiFiDirect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWiFiConnectionResult(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWiFiConnectionResult {
    type Vtable = IWiFiConnectionResult_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x143bdfd9_c37d_40be_a5c8_857bce85a931);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiFiConnectionResult_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub ConnectionStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut WiFiConnectionStatus) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWiFiNetworkReport(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWiFiNetworkReport {
    type Vtable = IWiFiNetworkReport_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9524ded2_5911_445e_8194_be4f1a704895);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiFiNetworkReport_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Timestamp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::DateTime) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-foundation")]
    pub AvailableNetworks: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    AvailableNetworks: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWiFiWpsConfigurationResult(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWiFiWpsConfigurationResult {
    type Vtable = IWiFiWpsConfigurationResult_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x67b49871_17ee_42d1_b14f_5a11f1226fb5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWiFiWpsConfigurationResult_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut WiFiWpsConfigurationStatus) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-foundation")]
    pub SupportedWpsKinds: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    SupportedWpsKinds: usize,
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WiFiAccessStatus(pub i32);
impl WiFiAccessStatus {
    pub const Unspecified: Self = Self(0i32);
    pub const Allowed: Self = Self(1i32);
    pub const DeniedByUser: Self = Self(2i32);
    pub const DeniedBySystem: Self = Self(3i32);
}
impl ::core::marker::Copy for WiFiAccessStatus {}
impl ::core::clone::Clone for WiFiAccessStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WiFiAccessStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for WiFiAccessStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for WiFiAccessStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WiFiAccessStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for WiFiAccessStatus {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Devices.WiFi.WiFiAccessStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct WiFiAdapter(::windows_core::IUnknown);
impl WiFiAdapter {
    #[cfg(feature = "winrt-networking")]
    pub fn NetworkAdapter(&self) -> ::windows_core::Result<::winrt_networking::Connectivity::NetworkAdapter> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).NetworkAdapter)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_networking::Connectivity::NetworkAdapter>(result__)
        }
    }
    pub fn ScanAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ScanAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncAction>(result__)
        }
    }
    pub fn NetworkReport(&self) -> ::windows_core::Result<WiFiNetworkReport> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).NetworkReport)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<WiFiNetworkReport>(result__)
        }
    }
    pub fn AvailableNetworksChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<WiFiAdapter, ::windows_core::IInspectable>>>(&self, args: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).AvailableNetworksChanged)(::windows_core::Interface::as_raw(this), args.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveAvailableNetworksChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, eventcookie: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveAvailableNetworksChanged)(::windows_core::Interface::as_raw(this), eventcookie.into_param().abi()).ok() }
    }
    pub fn ConnectAsync<'a, Param0: ::windows_core::IntoParam<'a, WiFiAvailableNetwork>>(&self, availablenetwork: Param0, reconnectionkind: WiFiReconnectionKind) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<WiFiConnectionResult>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ConnectAsync)(::windows_core::Interface::as_raw(this), availablenetwork.into_param().abi(), reconnectionkind, result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<WiFiConnectionResult>>(result__)
        }
    }
    #[cfg(feature = "winrt-security")]
    pub fn ConnectWithPasswordCredentialAsync<'a, Param0: ::windows_core::IntoParam<'a, WiFiAvailableNetwork>, Param2: ::windows_core::IntoParam<'a, ::winrt_security::Credentials::PasswordCredential>>(&self, availablenetwork: Param0, reconnectionkind: WiFiReconnectionKind, passwordcredential: Param2) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<WiFiConnectionResult>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ConnectWithPasswordCredentialAsync)(::windows_core::Interface::as_raw(this), availablenetwork.into_param().abi(), reconnectionkind, passwordcredential.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<WiFiConnectionResult>>(result__)
        }
    }
    #[cfg(feature = "winrt-security")]
    pub fn ConnectWithPasswordCredentialAndSsidAsync<'a, Param0: ::windows_core::IntoParam<'a, WiFiAvailableNetwork>, Param2: ::windows_core::IntoParam<'a, ::winrt_security::Credentials::PasswordCredential>, Param3: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, availablenetwork: Param0, reconnectionkind: WiFiReconnectionKind, passwordcredential: Param2, ssid: Param3) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<WiFiConnectionResult>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ConnectWithPasswordCredentialAndSsidAsync)(::windows_core::Interface::as_raw(this), availablenetwork.into_param().abi(), reconnectionkind, passwordcredential.into_param().abi(), ssid.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<WiFiConnectionResult>>(result__)
        }
    }
    pub fn Disconnect(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Disconnect)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn GetWpsConfigurationAsync<'a, Param0: ::windows_core::IntoParam<'a, WiFiAvailableNetwork>>(&self, availablenetwork: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<WiFiWpsConfigurationResult>> {
        let this = &::windows_core::Interface::cast::<IWiFiAdapter2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetWpsConfigurationAsync)(::windows_core::Interface::as_raw(this), availablenetwork.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<WiFiWpsConfigurationResult>>(result__)
        }
    }
    #[cfg(feature = "winrt-security")]
    pub fn ConnectWithPasswordCredentialAndSsidAndConnectionMethodAsync<'a, Param0: ::windows_core::IntoParam<'a, WiFiAvailableNetwork>, Param2: ::windows_core::IntoParam<'a, ::winrt_security::Credentials::PasswordCredential>, Param3: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, availablenetwork: Param0, reconnectionkind: WiFiReconnectionKind, passwordcredential: Param2, ssid: Param3, connectionmethod: WiFiConnectionMethod) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<WiFiConnectionResult>> {
        let this = &::windows_core::Interface::cast::<IWiFiAdapter2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ConnectWithPasswordCredentialAndSsidAndConnectionMethodAsync)(::windows_core::Interface::as_raw(this), availablenetwork.into_param().abi(), reconnectionkind, passwordcredential.into_param().abi(), ssid.into_param().abi(), connectionmethod, result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<WiFiConnectionResult>>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn FindAllAdaptersAsync() -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<::winrt_foundation::Collections::IVectorView<WiFiAdapter>>> {
        Self::IWiFiAdapterStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FindAllAdaptersAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<::winrt_foundation::Collections::IVectorView<WiFiAdapter>>>(result__)
        })
    }
    pub fn GetDeviceSelector() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IWiFiAdapterStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).GetDeviceSelector)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn FromIdAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(deviceid: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<WiFiAdapter>> {
        Self::IWiFiAdapterStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FromIdAsync)(::windows_core::Interface::as_raw(this), deviceid.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<WiFiAdapter>>(result__)
        })
    }
    pub fn RequestAccessAsync() -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<WiFiAccessStatus>> {
        Self::IWiFiAdapterStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RequestAccessAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<WiFiAccessStatus>>(result__)
        })
    }
    pub fn IWiFiAdapterStatics<R, F: FnOnce(&IWiFiAdapterStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<WiFiAdapter, IWiFiAdapterStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for WiFiAdapter {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for WiFiAdapter {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for WiFiAdapter {}
impl ::core::fmt::Debug for WiFiAdapter {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WiFiAdapter").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for WiFiAdapter {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.WiFi.WiFiAdapter;{a6c4e423-3d75-43a4-b9de-11e26b72d9b0})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for WiFiAdapter {
    type Vtable = IWiFiAdapter_Vtbl;
    const IID: ::windows_core::GUID = <IWiFiAdapter as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for WiFiAdapter {
    const NAME: &'static str = "Windows.Devices.WiFi.WiFiAdapter";
}
impl ::core::convert::From<WiFiAdapter> for ::windows_core::IUnknown {
    fn from(value: WiFiAdapter) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WiFiAdapter> for ::windows_core::IUnknown {
    fn from(value: &WiFiAdapter) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for WiFiAdapter {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a WiFiAdapter {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<WiFiAdapter> for ::windows_core::IInspectable {
    fn from(value: WiFiAdapter) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WiFiAdapter> for ::windows_core::IInspectable {
    fn from(value: &WiFiAdapter) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for WiFiAdapter {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a WiFiAdapter {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for WiFiAdapter {}
unsafe impl ::core::marker::Sync for WiFiAdapter {}
#[repr(transparent)]
pub struct WiFiAvailableNetwork(::windows_core::IUnknown);
impl WiFiAvailableNetwork {
    pub fn Uptime(&self) -> ::windows_core::Result<::winrt_foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::TimeSpan>::zeroed();
            (::windows_core::Interface::vtable(this).Uptime)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::TimeSpan>(result__)
        }
    }
    pub fn Ssid(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Ssid)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn Bssid(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Bssid)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn ChannelCenterFrequencyInKilohertz(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows_core::Interface::vtable(this).ChannelCenterFrequencyInKilohertz)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    pub fn NetworkRssiInDecibelMilliwatts(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).NetworkRssiInDecibelMilliwatts)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn SignalBars(&self) -> ::windows_core::Result<u8> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u8>::zeroed();
            (::windows_core::Interface::vtable(this).SignalBars)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u8>(result__)
        }
    }
    pub fn NetworkKind(&self) -> ::windows_core::Result<WiFiNetworkKind> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<WiFiNetworkKind>::zeroed();
            (::windows_core::Interface::vtable(this).NetworkKind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<WiFiNetworkKind>(result__)
        }
    }
    pub fn PhyKind(&self) -> ::windows_core::Result<WiFiPhyKind> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<WiFiPhyKind>::zeroed();
            (::windows_core::Interface::vtable(this).PhyKind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<WiFiPhyKind>(result__)
        }
    }
    #[cfg(feature = "winrt-networking")]
    pub fn SecuritySettings(&self) -> ::windows_core::Result<::winrt_networking::Connectivity::NetworkSecuritySettings> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SecuritySettings)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_networking::Connectivity::NetworkSecuritySettings>(result__)
        }
    }
    pub fn BeaconInterval(&self) -> ::windows_core::Result<::winrt_foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::TimeSpan>::zeroed();
            (::windows_core::Interface::vtable(this).BeaconInterval)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::TimeSpan>(result__)
        }
    }
    pub fn IsWiFiDirect(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsWiFiDirect)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
}
impl ::core::clone::Clone for WiFiAvailableNetwork {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for WiFiAvailableNetwork {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for WiFiAvailableNetwork {}
impl ::core::fmt::Debug for WiFiAvailableNetwork {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WiFiAvailableNetwork").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for WiFiAvailableNetwork {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.WiFi.WiFiAvailableNetwork;{26e96246-183e-4704-9826-71b4a2f0f668})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for WiFiAvailableNetwork {
    type Vtable = IWiFiAvailableNetwork_Vtbl;
    const IID: ::windows_core::GUID = <IWiFiAvailableNetwork as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for WiFiAvailableNetwork {
    const NAME: &'static str = "Windows.Devices.WiFi.WiFiAvailableNetwork";
}
impl ::core::convert::From<WiFiAvailableNetwork> for ::windows_core::IUnknown {
    fn from(value: WiFiAvailableNetwork) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WiFiAvailableNetwork> for ::windows_core::IUnknown {
    fn from(value: &WiFiAvailableNetwork) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for WiFiAvailableNetwork {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a WiFiAvailableNetwork {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<WiFiAvailableNetwork> for ::windows_core::IInspectable {
    fn from(value: WiFiAvailableNetwork) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WiFiAvailableNetwork> for ::windows_core::IInspectable {
    fn from(value: &WiFiAvailableNetwork) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for WiFiAvailableNetwork {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a WiFiAvailableNetwork {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for WiFiAvailableNetwork {}
unsafe impl ::core::marker::Sync for WiFiAvailableNetwork {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WiFiConnectionMethod(pub i32);
impl WiFiConnectionMethod {
    pub const Default: Self = Self(0i32);
    pub const WpsPin: Self = Self(1i32);
    pub const WpsPushButton: Self = Self(2i32);
}
impl ::core::marker::Copy for WiFiConnectionMethod {}
impl ::core::clone::Clone for WiFiConnectionMethod {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WiFiConnectionMethod {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for WiFiConnectionMethod {
    type Abi = Self;
}
impl ::core::fmt::Debug for WiFiConnectionMethod {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WiFiConnectionMethod").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for WiFiConnectionMethod {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Devices.WiFi.WiFiConnectionMethod;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct WiFiConnectionResult(::windows_core::IUnknown);
impl WiFiConnectionResult {
    pub fn ConnectionStatus(&self) -> ::windows_core::Result<WiFiConnectionStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<WiFiConnectionStatus>::zeroed();
            (::windows_core::Interface::vtable(this).ConnectionStatus)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<WiFiConnectionStatus>(result__)
        }
    }
}
impl ::core::clone::Clone for WiFiConnectionResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for WiFiConnectionResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for WiFiConnectionResult {}
impl ::core::fmt::Debug for WiFiConnectionResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WiFiConnectionResult").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for WiFiConnectionResult {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.WiFi.WiFiConnectionResult;{143bdfd9-c37d-40be-a5c8-857bce85a931})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for WiFiConnectionResult {
    type Vtable = IWiFiConnectionResult_Vtbl;
    const IID: ::windows_core::GUID = <IWiFiConnectionResult as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for WiFiConnectionResult {
    const NAME: &'static str = "Windows.Devices.WiFi.WiFiConnectionResult";
}
impl ::core::convert::From<WiFiConnectionResult> for ::windows_core::IUnknown {
    fn from(value: WiFiConnectionResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WiFiConnectionResult> for ::windows_core::IUnknown {
    fn from(value: &WiFiConnectionResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for WiFiConnectionResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a WiFiConnectionResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<WiFiConnectionResult> for ::windows_core::IInspectable {
    fn from(value: WiFiConnectionResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WiFiConnectionResult> for ::windows_core::IInspectable {
    fn from(value: &WiFiConnectionResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for WiFiConnectionResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a WiFiConnectionResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for WiFiConnectionResult {}
unsafe impl ::core::marker::Sync for WiFiConnectionResult {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WiFiConnectionStatus(pub i32);
impl WiFiConnectionStatus {
    pub const UnspecifiedFailure: Self = Self(0i32);
    pub const Success: Self = Self(1i32);
    pub const AccessRevoked: Self = Self(2i32);
    pub const InvalidCredential: Self = Self(3i32);
    pub const NetworkNotAvailable: Self = Self(4i32);
    pub const Timeout: Self = Self(5i32);
    pub const UnsupportedAuthenticationProtocol: Self = Self(6i32);
}
impl ::core::marker::Copy for WiFiConnectionStatus {}
impl ::core::clone::Clone for WiFiConnectionStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WiFiConnectionStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for WiFiConnectionStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for WiFiConnectionStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WiFiConnectionStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for WiFiConnectionStatus {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Devices.WiFi.WiFiConnectionStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WiFiNetworkKind(pub i32);
impl WiFiNetworkKind {
    pub const Any: Self = Self(0i32);
    pub const Infrastructure: Self = Self(1i32);
    pub const Adhoc: Self = Self(2i32);
}
impl ::core::marker::Copy for WiFiNetworkKind {}
impl ::core::clone::Clone for WiFiNetworkKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WiFiNetworkKind {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for WiFiNetworkKind {
    type Abi = Self;
}
impl ::core::fmt::Debug for WiFiNetworkKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WiFiNetworkKind").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for WiFiNetworkKind {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Devices.WiFi.WiFiNetworkKind;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct WiFiNetworkReport(::windows_core::IUnknown);
impl WiFiNetworkReport {
    pub fn Timestamp(&self) -> ::windows_core::Result<::winrt_foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::DateTime>::zeroed();
            (::windows_core::Interface::vtable(this).Timestamp)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::DateTime>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn AvailableNetworks(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<WiFiAvailableNetwork>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).AvailableNetworks)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<WiFiAvailableNetwork>>(result__)
        }
    }
}
impl ::core::clone::Clone for WiFiNetworkReport {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for WiFiNetworkReport {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for WiFiNetworkReport {}
impl ::core::fmt::Debug for WiFiNetworkReport {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WiFiNetworkReport").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for WiFiNetworkReport {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.WiFi.WiFiNetworkReport;{9524ded2-5911-445e-8194-be4f1a704895})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for WiFiNetworkReport {
    type Vtable = IWiFiNetworkReport_Vtbl;
    const IID: ::windows_core::GUID = <IWiFiNetworkReport as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for WiFiNetworkReport {
    const NAME: &'static str = "Windows.Devices.WiFi.WiFiNetworkReport";
}
impl ::core::convert::From<WiFiNetworkReport> for ::windows_core::IUnknown {
    fn from(value: WiFiNetworkReport) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WiFiNetworkReport> for ::windows_core::IUnknown {
    fn from(value: &WiFiNetworkReport) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for WiFiNetworkReport {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a WiFiNetworkReport {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<WiFiNetworkReport> for ::windows_core::IInspectable {
    fn from(value: WiFiNetworkReport) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WiFiNetworkReport> for ::windows_core::IInspectable {
    fn from(value: &WiFiNetworkReport) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for WiFiNetworkReport {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a WiFiNetworkReport {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for WiFiNetworkReport {}
unsafe impl ::core::marker::Sync for WiFiNetworkReport {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WiFiPhyKind(pub i32);
impl WiFiPhyKind {
    pub const Unknown: Self = Self(0i32);
    pub const Fhss: Self = Self(1i32);
    pub const Dsss: Self = Self(2i32);
    pub const IRBaseband: Self = Self(3i32);
    pub const Ofdm: Self = Self(4i32);
    pub const Hrdsss: Self = Self(5i32);
    pub const Erp: Self = Self(6i32);
    pub const HT: Self = Self(7i32);
    pub const Vht: Self = Self(8i32);
    pub const Dmg: Self = Self(9i32);
    pub const HE: Self = Self(10i32);
}
impl ::core::marker::Copy for WiFiPhyKind {}
impl ::core::clone::Clone for WiFiPhyKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WiFiPhyKind {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for WiFiPhyKind {
    type Abi = Self;
}
impl ::core::fmt::Debug for WiFiPhyKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WiFiPhyKind").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for WiFiPhyKind {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Devices.WiFi.WiFiPhyKind;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WiFiReconnectionKind(pub i32);
impl WiFiReconnectionKind {
    pub const Automatic: Self = Self(0i32);
    pub const Manual: Self = Self(1i32);
}
impl ::core::marker::Copy for WiFiReconnectionKind {}
impl ::core::clone::Clone for WiFiReconnectionKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WiFiReconnectionKind {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for WiFiReconnectionKind {
    type Abi = Self;
}
impl ::core::fmt::Debug for WiFiReconnectionKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WiFiReconnectionKind").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for WiFiReconnectionKind {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Devices.WiFi.WiFiReconnectionKind;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct WiFiWpsConfigurationResult(::windows_core::IUnknown);
impl WiFiWpsConfigurationResult {
    pub fn Status(&self) -> ::windows_core::Result<WiFiWpsConfigurationStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<WiFiWpsConfigurationStatus>::zeroed();
            (::windows_core::Interface::vtable(this).Status)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<WiFiWpsConfigurationStatus>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn SupportedWpsKinds(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<WiFiWpsKind>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SupportedWpsKinds)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<WiFiWpsKind>>(result__)
        }
    }
}
impl ::core::clone::Clone for WiFiWpsConfigurationResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for WiFiWpsConfigurationResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for WiFiWpsConfigurationResult {}
impl ::core::fmt::Debug for WiFiWpsConfigurationResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WiFiWpsConfigurationResult").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for WiFiWpsConfigurationResult {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.WiFi.WiFiWpsConfigurationResult;{67b49871-17ee-42d1-b14f-5a11f1226fb5})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for WiFiWpsConfigurationResult {
    type Vtable = IWiFiWpsConfigurationResult_Vtbl;
    const IID: ::windows_core::GUID = <IWiFiWpsConfigurationResult as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for WiFiWpsConfigurationResult {
    const NAME: &'static str = "Windows.Devices.WiFi.WiFiWpsConfigurationResult";
}
impl ::core::convert::From<WiFiWpsConfigurationResult> for ::windows_core::IUnknown {
    fn from(value: WiFiWpsConfigurationResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WiFiWpsConfigurationResult> for ::windows_core::IUnknown {
    fn from(value: &WiFiWpsConfigurationResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for WiFiWpsConfigurationResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a WiFiWpsConfigurationResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<WiFiWpsConfigurationResult> for ::windows_core::IInspectable {
    fn from(value: WiFiWpsConfigurationResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WiFiWpsConfigurationResult> for ::windows_core::IInspectable {
    fn from(value: &WiFiWpsConfigurationResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for WiFiWpsConfigurationResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a WiFiWpsConfigurationResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for WiFiWpsConfigurationResult {}
unsafe impl ::core::marker::Sync for WiFiWpsConfigurationResult {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WiFiWpsConfigurationStatus(pub i32);
impl WiFiWpsConfigurationStatus {
    pub const UnspecifiedFailure: Self = Self(0i32);
    pub const Success: Self = Self(1i32);
    pub const Timeout: Self = Self(2i32);
}
impl ::core::marker::Copy for WiFiWpsConfigurationStatus {}
impl ::core::clone::Clone for WiFiWpsConfigurationStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WiFiWpsConfigurationStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for WiFiWpsConfigurationStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for WiFiWpsConfigurationStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WiFiWpsConfigurationStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for WiFiWpsConfigurationStatus {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Devices.WiFi.WiFiWpsConfigurationStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WiFiWpsKind(pub i32);
impl WiFiWpsKind {
    pub const Unknown: Self = Self(0i32);
    pub const Pin: Self = Self(1i32);
    pub const PushButton: Self = Self(2i32);
    pub const Nfc: Self = Self(3i32);
    pub const Ethernet: Self = Self(4i32);
    pub const Usb: Self = Self(5i32);
}
impl ::core::marker::Copy for WiFiWpsKind {}
impl ::core::clone::Clone for WiFiWpsKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WiFiWpsKind {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for WiFiWpsKind {
    type Abi = Self;
}
impl ::core::fmt::Debug for WiFiWpsKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WiFiWpsKind").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for WiFiWpsKind {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Devices.WiFi.WiFiWpsKind;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
