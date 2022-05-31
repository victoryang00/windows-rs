#[doc(hidden)]
#[repr(transparent)]
pub struct IRadio(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IRadio {
    type Vtable = IRadio_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x252118df_b33e_416a_875f_1cf38ae2d83e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRadio_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub SetStateAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: RadioState, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub StateChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveStateChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, eventcookie: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub State: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut RadioState) -> ::windows_core::HRESULT,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Kind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut RadioKind) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRadioStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IRadioStatics {
    type Vtable = IRadioStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5fb6a12e_67cb_46ae_aae9_65919f86eff4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRadioStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-foundation")]
    pub GetRadiosAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    GetRadiosAsync: usize,
    pub GetDeviceSelector: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub FromIdAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub RequestAccessAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct Radio(::windows_core::IUnknown);
impl Radio {
    pub fn SetStateAsync(&self, value: RadioState) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<RadioAccessStatus>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).SetStateAsync)(::windows_core::Interface::as_raw(this), value, result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<RadioAccessStatus>>(result__)
        }
    }
    pub fn StateChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TypedEventHandler<Radio, ::windows_core::IInspectable>>>(&self, handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).StateChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        }
    }
    pub fn RemoveStateChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(&self, eventcookie: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveStateChanged)(::windows_core::Interface::as_raw(this), eventcookie.into_param().abi()).ok() }
    }
    pub fn State(&self) -> ::windows_core::Result<RadioState> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<RadioState>::zeroed();
            (::windows_core::Interface::vtable(this).State)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<RadioState>(result__)
        }
    }
    pub fn Name(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Name)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn Kind(&self) -> ::windows_core::Result<RadioKind> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<RadioKind>::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<RadioKind>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn GetRadiosAsync() -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<::winrt_foundation::Collections::IVectorView<Radio>>> {
        Self::IRadioStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetRadiosAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<::winrt_foundation::Collections::IVectorView<Radio>>>(result__)
        })
    }
    pub fn GetDeviceSelector() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IRadioStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).GetDeviceSelector)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn FromIdAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(deviceid: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<Radio>> {
        Self::IRadioStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FromIdAsync)(::windows_core::Interface::as_raw(this), deviceid.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<Radio>>(result__)
        })
    }
    pub fn RequestAccessAsync() -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<RadioAccessStatus>> {
        Self::IRadioStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).RequestAccessAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<RadioAccessStatus>>(result__)
        })
    }
    pub fn IRadioStatics<R, F: FnOnce(&IRadioStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<Radio, IRadioStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for Radio {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for Radio {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for Radio {}
impl ::core::fmt::Debug for Radio {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Radio").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for Radio {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.Radios.Radio;{252118df-b33e-416a-875f-1cf38ae2d83e})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for Radio {
    type Vtable = IRadio_Vtbl;
    const IID: ::windows_core::GUID = <IRadio as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for Radio {
    const NAME: &'static str = "Windows.Devices.Radios.Radio";
}
impl ::core::convert::From<Radio> for ::windows_core::IUnknown {
    fn from(value: Radio) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&Radio> for ::windows_core::IUnknown {
    fn from(value: &Radio) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for Radio {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a Radio {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<Radio> for ::windows_core::IInspectable {
    fn from(value: Radio) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&Radio> for ::windows_core::IInspectable {
    fn from(value: &Radio) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for Radio {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a Radio {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for Radio {}
unsafe impl ::core::marker::Sync for Radio {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct RadioAccessStatus(pub i32);
impl RadioAccessStatus {
    pub const Unspecified: Self = Self(0i32);
    pub const Allowed: Self = Self(1i32);
    pub const DeniedByUser: Self = Self(2i32);
    pub const DeniedBySystem: Self = Self(3i32);
}
impl ::core::marker::Copy for RadioAccessStatus {}
impl ::core::clone::Clone for RadioAccessStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RadioAccessStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for RadioAccessStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for RadioAccessStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RadioAccessStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for RadioAccessStatus {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Devices.Radios.RadioAccessStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct RadioKind(pub i32);
impl RadioKind {
    pub const Other: Self = Self(0i32);
    pub const WiFi: Self = Self(1i32);
    pub const MobileBroadband: Self = Self(2i32);
    pub const Bluetooth: Self = Self(3i32);
    pub const FM: Self = Self(4i32);
}
impl ::core::marker::Copy for RadioKind {}
impl ::core::clone::Clone for RadioKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RadioKind {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for RadioKind {
    type Abi = Self;
}
impl ::core::fmt::Debug for RadioKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RadioKind").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for RadioKind {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Devices.Radios.RadioKind;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct RadioState(pub i32);
impl RadioState {
    pub const Unknown: Self = Self(0i32);
    pub const On: Self = Self(1i32);
    pub const Off: Self = Self(2i32);
    pub const Disabled: Self = Self(3i32);
}
impl ::core::marker::Copy for RadioState {}
impl ::core::clone::Clone for RadioState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RadioState {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for RadioState {
    type Abi = Self;
}
impl ::core::fmt::Debug for RadioState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RadioState").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for RadioState {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Devices.Radios.RadioState;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
