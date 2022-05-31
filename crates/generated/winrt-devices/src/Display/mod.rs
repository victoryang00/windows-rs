#[cfg(feature = "Core")]
pub mod Core;
#[repr(transparent)]
pub struct DisplayMonitor(::windows_core::IUnknown);
impl DisplayMonitor {
    pub fn DeviceId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).DeviceId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn DisplayName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).DisplayName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn ConnectionKind(&self) -> ::windows_core::Result<DisplayMonitorConnectionKind> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<DisplayMonitorConnectionKind>::zeroed();
            (::windows_core::Interface::vtable(this).ConnectionKind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<DisplayMonitorConnectionKind>(result__)
        }
    }
    pub fn PhysicalConnector(&self) -> ::windows_core::Result<DisplayMonitorPhysicalConnectorKind> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<DisplayMonitorPhysicalConnectorKind>::zeroed();
            (::windows_core::Interface::vtable(this).PhysicalConnector)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<DisplayMonitorPhysicalConnectorKind>(result__)
        }
    }
    pub fn DisplayAdapterDeviceId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).DisplayAdapterDeviceId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "winrt-graphics")]
    pub fn DisplayAdapterId(&self) -> ::windows_core::Result<::winrt_graphics::DisplayAdapterId> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_graphics::DisplayAdapterId>::zeroed();
            (::windows_core::Interface::vtable(this).DisplayAdapterId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_graphics::DisplayAdapterId>(result__)
        }
    }
    pub fn DisplayAdapterTargetId(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).DisplayAdapterTargetId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn UsageKind(&self) -> ::windows_core::Result<DisplayMonitorUsageKind> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<DisplayMonitorUsageKind>::zeroed();
            (::windows_core::Interface::vtable(this).UsageKind)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<DisplayMonitorUsageKind>(result__)
        }
    }
    #[cfg(feature = "winrt-graphics")]
    pub fn NativeResolutionInRawPixels(&self) -> ::windows_core::Result<::winrt_graphics::SizeInt32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_graphics::SizeInt32>::zeroed();
            (::windows_core::Interface::vtable(this).NativeResolutionInRawPixels)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_graphics::SizeInt32>(result__)
        }
    }
    pub fn PhysicalSizeInInches(&self) -> ::windows_core::Result<::winrt_foundation::IReference<::winrt_foundation::Size>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).PhysicalSizeInInches)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IReference<::winrt_foundation::Size>>(result__)
        }
    }
    pub fn RawDpiX(&self) -> ::windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f32>::zeroed();
            (::windows_core::Interface::vtable(this).RawDpiX)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f32>(result__)
        }
    }
    pub fn RawDpiY(&self) -> ::windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f32>::zeroed();
            (::windows_core::Interface::vtable(this).RawDpiY)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f32>(result__)
        }
    }
    pub fn RedPrimary(&self) -> ::windows_core::Result<::winrt_foundation::Point> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::Point>::zeroed();
            (::windows_core::Interface::vtable(this).RedPrimary)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Point>(result__)
        }
    }
    pub fn GreenPrimary(&self) -> ::windows_core::Result<::winrt_foundation::Point> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::Point>::zeroed();
            (::windows_core::Interface::vtable(this).GreenPrimary)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Point>(result__)
        }
    }
    pub fn BluePrimary(&self) -> ::windows_core::Result<::winrt_foundation::Point> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::Point>::zeroed();
            (::windows_core::Interface::vtable(this).BluePrimary)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Point>(result__)
        }
    }
    pub fn WhitePoint(&self) -> ::windows_core::Result<::winrt_foundation::Point> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::Point>::zeroed();
            (::windows_core::Interface::vtable(this).WhitePoint)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Point>(result__)
        }
    }
    pub fn MaxLuminanceInNits(&self) -> ::windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f32>::zeroed();
            (::windows_core::Interface::vtable(this).MaxLuminanceInNits)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f32>(result__)
        }
    }
    pub fn MinLuminanceInNits(&self) -> ::windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f32>::zeroed();
            (::windows_core::Interface::vtable(this).MinLuminanceInNits)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f32>(result__)
        }
    }
    pub fn MaxAverageFullFrameLuminanceInNits(&self) -> ::windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f32>::zeroed();
            (::windows_core::Interface::vtable(this).MaxAverageFullFrameLuminanceInNits)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f32>(result__)
        }
    }
    pub fn GetDescriptor(&self, descriptorkind: DisplayMonitorDescriptorKind) -> ::windows_core::Result<::windows_core::Array<u8>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::Array<u8>>::zeroed();
            (::windows_core::Interface::vtable(this).GetDescriptor)(::windows_core::Interface::as_raw(this), descriptorkind, ::windows_core::Array::<u8>::set_abi_len(result__.assume_init_mut()), result__.as_mut_ptr() as *mut _ as _).and_then(|| result__.assume_init())
        }
    }
    pub fn IsDolbyVisionSupportedInHdrMode(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IDisplayMonitor2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsDolbyVisionSupportedInHdrMode)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn GetDeviceSelector() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IDisplayMonitorStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).GetDeviceSelector)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn FromIdAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(deviceid: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<DisplayMonitor>> {
        Self::IDisplayMonitorStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FromIdAsync)(::windows_core::Interface::as_raw(this), deviceid.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<DisplayMonitor>>(result__)
        })
    }
    pub fn FromInterfaceIdAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(deviceinterfaceid: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<DisplayMonitor>> {
        Self::IDisplayMonitorStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FromInterfaceIdAsync)(::windows_core::Interface::as_raw(this), deviceinterfaceid.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<DisplayMonitor>>(result__)
        })
    }
    pub fn IDisplayMonitorStatics<R, F: FnOnce(&IDisplayMonitorStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<DisplayMonitor, IDisplayMonitorStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for DisplayMonitor {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for DisplayMonitor {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DisplayMonitor {}
impl ::core::fmt::Debug for DisplayMonitor {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DisplayMonitor").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for DisplayMonitor {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Devices.Display.DisplayMonitor;{1f6b15d4-1d01-4c51-87e2-6f954a772b59})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for DisplayMonitor {
    type Vtable = IDisplayMonitor_Vtbl;
    const IID: ::windows_core::GUID = <IDisplayMonitor as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for DisplayMonitor {
    const NAME: &'static str = "Windows.Devices.Display.DisplayMonitor";
}
impl ::core::convert::From<DisplayMonitor> for ::windows_core::IUnknown {
    fn from(value: DisplayMonitor) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DisplayMonitor> for ::windows_core::IUnknown {
    fn from(value: &DisplayMonitor) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for DisplayMonitor {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a DisplayMonitor {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<DisplayMonitor> for ::windows_core::IInspectable {
    fn from(value: DisplayMonitor) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DisplayMonitor> for ::windows_core::IInspectable {
    fn from(value: &DisplayMonitor) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for DisplayMonitor {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a DisplayMonitor {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for DisplayMonitor {}
unsafe impl ::core::marker::Sync for DisplayMonitor {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DisplayMonitorConnectionKind(pub i32);
impl DisplayMonitorConnectionKind {
    pub const Internal: Self = Self(0i32);
    pub const Wired: Self = Self(1i32);
    pub const Wireless: Self = Self(2i32);
    pub const Virtual: Self = Self(3i32);
}
impl ::core::marker::Copy for DisplayMonitorConnectionKind {}
impl ::core::clone::Clone for DisplayMonitorConnectionKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DisplayMonitorConnectionKind {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for DisplayMonitorConnectionKind {
    type Abi = Self;
}
impl ::core::fmt::Debug for DisplayMonitorConnectionKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DisplayMonitorConnectionKind").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for DisplayMonitorConnectionKind {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Devices.Display.DisplayMonitorConnectionKind;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DisplayMonitorDescriptorKind(pub i32);
impl DisplayMonitorDescriptorKind {
    pub const Edid: Self = Self(0i32);
    pub const DisplayId: Self = Self(1i32);
}
impl ::core::marker::Copy for DisplayMonitorDescriptorKind {}
impl ::core::clone::Clone for DisplayMonitorDescriptorKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DisplayMonitorDescriptorKind {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for DisplayMonitorDescriptorKind {
    type Abi = Self;
}
impl ::core::fmt::Debug for DisplayMonitorDescriptorKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DisplayMonitorDescriptorKind").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for DisplayMonitorDescriptorKind {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Devices.Display.DisplayMonitorDescriptorKind;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DisplayMonitorPhysicalConnectorKind(pub i32);
impl DisplayMonitorPhysicalConnectorKind {
    pub const Unknown: Self = Self(0i32);
    pub const HD15: Self = Self(1i32);
    pub const AnalogTV: Self = Self(2i32);
    pub const Dvi: Self = Self(3i32);
    pub const Hdmi: Self = Self(4i32);
    pub const Lvds: Self = Self(5i32);
    pub const Sdi: Self = Self(6i32);
    pub const DisplayPort: Self = Self(7i32);
}
impl ::core::marker::Copy for DisplayMonitorPhysicalConnectorKind {}
impl ::core::clone::Clone for DisplayMonitorPhysicalConnectorKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DisplayMonitorPhysicalConnectorKind {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for DisplayMonitorPhysicalConnectorKind {
    type Abi = Self;
}
impl ::core::fmt::Debug for DisplayMonitorPhysicalConnectorKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DisplayMonitorPhysicalConnectorKind").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for DisplayMonitorPhysicalConnectorKind {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Devices.Display.DisplayMonitorPhysicalConnectorKind;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DisplayMonitorUsageKind(pub i32);
impl DisplayMonitorUsageKind {
    pub const Standard: Self = Self(0i32);
    pub const HeadMounted: Self = Self(1i32);
    pub const SpecialPurpose: Self = Self(2i32);
}
impl ::core::marker::Copy for DisplayMonitorUsageKind {}
impl ::core::clone::Clone for DisplayMonitorUsageKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DisplayMonitorUsageKind {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for DisplayMonitorUsageKind {
    type Abi = Self;
}
impl ::core::fmt::Debug for DisplayMonitorUsageKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DisplayMonitorUsageKind").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for DisplayMonitorUsageKind {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Devices.Display.DisplayMonitorUsageKind;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDisplayMonitor(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDisplayMonitor {
    type Vtable = IDisplayMonitor_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1f6b15d4_1d01_4c51_87e2_6f954a772b59);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDisplayMonitor_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub DeviceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub DisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub ConnectionKind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut DisplayMonitorConnectionKind) -> ::windows_core::HRESULT,
    pub PhysicalConnector: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut DisplayMonitorPhysicalConnectorKind) -> ::windows_core::HRESULT,
    pub DisplayAdapterDeviceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-graphics")]
    pub DisplayAdapterId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_graphics::DisplayAdapterId) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-graphics"))]
    DisplayAdapterId: usize,
    pub DisplayAdapterTargetId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub UsageKind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut DisplayMonitorUsageKind) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-graphics")]
    pub NativeResolutionInRawPixels: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_graphics::SizeInt32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-graphics"))]
    NativeResolutionInRawPixels: usize,
    pub PhysicalSizeInInches: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub RawDpiX: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows_core::HRESULT,
    pub RawDpiY: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows_core::HRESULT,
    pub RedPrimary: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::Point) -> ::windows_core::HRESULT,
    pub GreenPrimary: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::Point) -> ::windows_core::HRESULT,
    pub BluePrimary: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::Point) -> ::windows_core::HRESULT,
    pub WhitePoint: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::Point) -> ::windows_core::HRESULT,
    pub MaxLuminanceInNits: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows_core::HRESULT,
    pub MinLuminanceInNits: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows_core::HRESULT,
    pub MaxAverageFullFrameLuminanceInNits: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows_core::HRESULT,
    pub GetDescriptor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, descriptorkind: DisplayMonitorDescriptorKind, result_size__: *mut u32, result__: *mut *mut u8) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDisplayMonitor2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDisplayMonitor2 {
    type Vtable = IDisplayMonitor2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x023018e6_cb23_5830_96df_a7bf6e602577);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDisplayMonitor2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub IsDolbyVisionSupportedInHdrMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDisplayMonitorStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDisplayMonitorStatics {
    type Vtable = IDisplayMonitorStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6eae698f_a228_4c05_821d_b695d667de8e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDisplayMonitorStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub GetDeviceSelector: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub FromIdAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub FromInterfaceIdAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceinterfaceid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
