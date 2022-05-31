pub struct GameControllerFactoryManager;
impl GameControllerFactoryManager {
    pub fn RegisterCustomFactoryForGipInterface<'a, Param0: ::windows_core::IntoParam<'a, ICustomGameControllerFactory>, Param1: ::windows_core::IntoParam<'a, ::windows_core::GUID>>(factory: Param0, interfaceid: Param1) -> ::windows_core::Result<()> {
        Self::IGameControllerFactoryManagerStatics(|this| unsafe { (::windows_core::Interface::vtable(this).RegisterCustomFactoryForGipInterface)(::windows_core::Interface::as_raw(this), factory.into_param().abi(), interfaceid.into_param().abi()).ok() })
    }
    pub fn RegisterCustomFactoryForHardwareId<'a, Param0: ::windows_core::IntoParam<'a, ICustomGameControllerFactory>>(factory: Param0, hardwarevendorid: u16, hardwareproductid: u16) -> ::windows_core::Result<()> {
        Self::IGameControllerFactoryManagerStatics(|this| unsafe { (::windows_core::Interface::vtable(this).RegisterCustomFactoryForHardwareId)(::windows_core::Interface::as_raw(this), factory.into_param().abi(), hardwarevendorid, hardwareproductid).ok() })
    }
    pub fn RegisterCustomFactoryForXusbType<'a, Param0: ::windows_core::IntoParam<'a, ICustomGameControllerFactory>>(factory: Param0, xusbtype: XusbDeviceType, xusbsubtype: XusbDeviceSubtype) -> ::windows_core::Result<()> {
        Self::IGameControllerFactoryManagerStatics(|this| unsafe { (::windows_core::Interface::vtable(this).RegisterCustomFactoryForXusbType)(::windows_core::Interface::as_raw(this), factory.into_param().abi(), xusbtype, xusbsubtype).ok() })
    }
    pub fn TryGetFactoryControllerFromGameController<'a, Param0: ::windows_core::IntoParam<'a, ICustomGameControllerFactory>, Param1: ::windows_core::IntoParam<'a, super::IGameController>>(factory: Param0, gamecontroller: Param1) -> ::windows_core::Result<super::IGameController> {
        Self::IGameControllerFactoryManagerStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TryGetFactoryControllerFromGameController)(::windows_core::Interface::as_raw(this), factory.into_param().abi(), gamecontroller.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::IGameController>(result__)
        })
    }
    pub fn IGameControllerFactoryManagerStatics<R, F: FnOnce(&IGameControllerFactoryManagerStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<GameControllerFactoryManager, IGameControllerFactoryManagerStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IGameControllerFactoryManagerStatics2<R, F: FnOnce(&IGameControllerFactoryManagerStatics2) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<GameControllerFactoryManager, IGameControllerFactoryManagerStatics2> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows_core::RuntimeName for GameControllerFactoryManager {
    const NAME: &'static str = "Windows.Gaming.Input.Custom.GameControllerFactoryManager";
}
#[repr(C)]
pub struct GameControllerVersionInfo {
    pub Major: u16,
    pub Minor: u16,
    pub Build: u16,
    pub Revision: u16,
}
impl ::core::marker::Copy for GameControllerVersionInfo {}
impl ::core::clone::Clone for GameControllerVersionInfo {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for GameControllerVersionInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GameControllerVersionInfo").field("Major", &self.Major).field("Minor", &self.Minor).field("Build", &self.Build).field("Revision", &self.Revision).finish()
    }
}
unsafe impl ::windows_core::Abi for GameControllerVersionInfo {
    type Abi = Self;
}
unsafe impl ::windows_core::RuntimeType for GameControllerVersionInfo {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"struct(Windows.Gaming.Input.Custom.GameControllerVersionInfo;u2;u2;u2;u2)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
impl ::core::cmp::PartialEq for GameControllerVersionInfo {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<GameControllerVersionInfo>()) == 0 }
    }
}
impl ::core::cmp::Eq for GameControllerVersionInfo {}
impl ::core::default::Default for GameControllerVersionInfo {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct GipFirmwareUpdateProgress {
    pub PercentCompleted: f64,
    pub CurrentComponentId: u32,
}
impl ::core::marker::Copy for GipFirmwareUpdateProgress {}
impl ::core::clone::Clone for GipFirmwareUpdateProgress {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for GipFirmwareUpdateProgress {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GipFirmwareUpdateProgress").field("PercentCompleted", &self.PercentCompleted).field("CurrentComponentId", &self.CurrentComponentId).finish()
    }
}
unsafe impl ::windows_core::Abi for GipFirmwareUpdateProgress {
    type Abi = Self;
}
unsafe impl ::windows_core::RuntimeType for GipFirmwareUpdateProgress {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"struct(Windows.Gaming.Input.Custom.GipFirmwareUpdateProgress;f8;u4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
impl ::core::cmp::PartialEq for GipFirmwareUpdateProgress {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<GipFirmwareUpdateProgress>()) == 0 }
    }
}
impl ::core::cmp::Eq for GipFirmwareUpdateProgress {}
impl ::core::default::Default for GipFirmwareUpdateProgress {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
pub struct GipFirmwareUpdateResult(::windows_core::IUnknown);
impl GipFirmwareUpdateResult {
    pub fn ExtendedErrorCode(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).ExtendedErrorCode)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn FinalComponentId(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).FinalComponentId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn Status(&self) -> ::windows_core::Result<GipFirmwareUpdateStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<GipFirmwareUpdateStatus>::zeroed();
            (::windows_core::Interface::vtable(this).Status)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<GipFirmwareUpdateStatus>(result__)
        }
    }
}
impl ::core::clone::Clone for GipFirmwareUpdateResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for GipFirmwareUpdateResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GipFirmwareUpdateResult {}
impl ::core::fmt::Debug for GipFirmwareUpdateResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GipFirmwareUpdateResult").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for GipFirmwareUpdateResult {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Gaming.Input.Custom.GipFirmwareUpdateResult;{6b794d32-8553-4292-8e03-e16651a2f8bc})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for GipFirmwareUpdateResult {
    type Vtable = IGipFirmwareUpdateResult_Vtbl;
    const IID: ::windows_core::GUID = <IGipFirmwareUpdateResult as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for GipFirmwareUpdateResult {
    const NAME: &'static str = "Windows.Gaming.Input.Custom.GipFirmwareUpdateResult";
}
impl ::core::convert::From<GipFirmwareUpdateResult> for ::windows_core::IUnknown {
    fn from(value: GipFirmwareUpdateResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&GipFirmwareUpdateResult> for ::windows_core::IUnknown {
    fn from(value: &GipFirmwareUpdateResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for GipFirmwareUpdateResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a GipFirmwareUpdateResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<GipFirmwareUpdateResult> for ::windows_core::IInspectable {
    fn from(value: GipFirmwareUpdateResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&GipFirmwareUpdateResult> for ::windows_core::IInspectable {
    fn from(value: &GipFirmwareUpdateResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for GipFirmwareUpdateResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a GipFirmwareUpdateResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for GipFirmwareUpdateResult {}
unsafe impl ::core::marker::Sync for GipFirmwareUpdateResult {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct GipFirmwareUpdateStatus(pub i32);
impl GipFirmwareUpdateStatus {
    pub const Completed: Self = Self(0i32);
    pub const UpToDate: Self = Self(1i32);
    pub const Failed: Self = Self(2i32);
}
impl ::core::marker::Copy for GipFirmwareUpdateStatus {}
impl ::core::clone::Clone for GipFirmwareUpdateStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for GipFirmwareUpdateStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for GipFirmwareUpdateStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for GipFirmwareUpdateStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GipFirmwareUpdateStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for GipFirmwareUpdateStatus {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Gaming.Input.Custom.GipFirmwareUpdateStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct GipGameControllerProvider(::windows_core::IUnknown);
impl GipGameControllerProvider {
    pub fn FirmwareVersionInfo(&self) -> ::windows_core::Result<GameControllerVersionInfo> {
        let this = &::windows_core::Interface::cast::<IGameControllerProvider>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<GameControllerVersionInfo>::zeroed();
            (::windows_core::Interface::vtable(this).FirmwareVersionInfo)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<GameControllerVersionInfo>(result__)
        }
    }
    pub fn HardwareProductId(&self) -> ::windows_core::Result<u16> {
        let this = &::windows_core::Interface::cast::<IGameControllerProvider>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u16>::zeroed();
            (::windows_core::Interface::vtable(this).HardwareProductId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u16>(result__)
        }
    }
    pub fn HardwareVendorId(&self) -> ::windows_core::Result<u16> {
        let this = &::windows_core::Interface::cast::<IGameControllerProvider>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u16>::zeroed();
            (::windows_core::Interface::vtable(this).HardwareVendorId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u16>(result__)
        }
    }
    pub fn HardwareVersionInfo(&self) -> ::windows_core::Result<GameControllerVersionInfo> {
        let this = &::windows_core::Interface::cast::<IGameControllerProvider>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<GameControllerVersionInfo>::zeroed();
            (::windows_core::Interface::vtable(this).HardwareVersionInfo)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<GameControllerVersionInfo>(result__)
        }
    }
    pub fn IsConnected(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IGameControllerProvider>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsConnected)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SendMessage(&self, messageclass: GipMessageClass, messageid: u8, messagebuffer: &[u8]) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SendMessage)(::windows_core::Interface::as_raw(this), messageclass, messageid, messagebuffer.len() as u32, ::core::mem::transmute(messagebuffer.as_ptr())).ok() }
    }
    pub fn SendReceiveMessage(&self, messageclass: GipMessageClass, messageid: u8, requestmessagebuffer: &[u8], responsemessagebuffer: &mut [u8]) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SendReceiveMessage)(::windows_core::Interface::as_raw(this), messageclass, messageid, requestmessagebuffer.len() as u32, ::core::mem::transmute(requestmessagebuffer.as_ptr()), responsemessagebuffer.len() as u32, ::core::mem::transmute_copy(&responsemessagebuffer)).ok() }
    }
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn UpdateFirmwareAsync<'a, Param0: ::windows_core::IntoParam<'a, super::super::super::Storage::Streams::IInputStream>>(&self, firmwareimage: Param0) -> ::windows_core::Result<super::super::super::Foundation::IAsyncOperationWithProgress<GipFirmwareUpdateResult, GipFirmwareUpdateProgress>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).UpdateFirmwareAsync)(::windows_core::Interface::as_raw(this), firmwareimage.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::IAsyncOperationWithProgress<GipFirmwareUpdateResult, GipFirmwareUpdateProgress>>(result__)
        }
    }
}
impl ::core::clone::Clone for GipGameControllerProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for GipGameControllerProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GipGameControllerProvider {}
impl ::core::fmt::Debug for GipGameControllerProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GipGameControllerProvider").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for GipGameControllerProvider {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Gaming.Input.Custom.GipGameControllerProvider;{dbcf1e19-1af5-45a8-bf02-a0ee50c823fc})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for GipGameControllerProvider {
    type Vtable = IGipGameControllerProvider_Vtbl;
    const IID: ::windows_core::GUID = <IGipGameControllerProvider as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for GipGameControllerProvider {
    const NAME: &'static str = "Windows.Gaming.Input.Custom.GipGameControllerProvider";
}
impl ::core::convert::From<GipGameControllerProvider> for ::windows_core::IUnknown {
    fn from(value: GipGameControllerProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&GipGameControllerProvider> for ::windows_core::IUnknown {
    fn from(value: &GipGameControllerProvider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for GipGameControllerProvider {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a GipGameControllerProvider {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<GipGameControllerProvider> for ::windows_core::IInspectable {
    fn from(value: GipGameControllerProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&GipGameControllerProvider> for ::windows_core::IInspectable {
    fn from(value: &GipGameControllerProvider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for GipGameControllerProvider {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a GipGameControllerProvider {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<GipGameControllerProvider> for IGameControllerProvider {
    type Error = ::windows_core::Error;
    fn try_from(value: GipGameControllerProvider) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&GipGameControllerProvider> for IGameControllerProvider {
    type Error = ::windows_core::Error;
    fn try_from(value: &GipGameControllerProvider) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IGameControllerProvider> for GipGameControllerProvider {
    fn into_param(self) -> ::windows_core::Param<'a, IGameControllerProvider> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IGameControllerProvider> for &GipGameControllerProvider {
    fn into_param(self) -> ::windows_core::Param<'a, IGameControllerProvider> {
        ::core::convert::TryInto::<IGameControllerProvider>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for GipGameControllerProvider {}
unsafe impl ::core::marker::Sync for GipGameControllerProvider {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct GipMessageClass(pub i32);
impl GipMessageClass {
    pub const Command: Self = Self(0i32);
    pub const LowLatency: Self = Self(1i32);
    pub const StandardLatency: Self = Self(2i32);
}
impl ::core::marker::Copy for GipMessageClass {}
impl ::core::clone::Clone for GipMessageClass {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for GipMessageClass {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for GipMessageClass {
    type Abi = Self;
}
impl ::core::fmt::Debug for GipMessageClass {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GipMessageClass").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for GipMessageClass {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Gaming.Input.Custom.GipMessageClass;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct HidGameControllerProvider(::windows_core::IUnknown);
impl HidGameControllerProvider {
    pub fn FirmwareVersionInfo(&self) -> ::windows_core::Result<GameControllerVersionInfo> {
        let this = &::windows_core::Interface::cast::<IGameControllerProvider>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<GameControllerVersionInfo>::zeroed();
            (::windows_core::Interface::vtable(this).FirmwareVersionInfo)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<GameControllerVersionInfo>(result__)
        }
    }
    pub fn HardwareProductId(&self) -> ::windows_core::Result<u16> {
        let this = &::windows_core::Interface::cast::<IGameControllerProvider>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u16>::zeroed();
            (::windows_core::Interface::vtable(this).HardwareProductId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u16>(result__)
        }
    }
    pub fn HardwareVendorId(&self) -> ::windows_core::Result<u16> {
        let this = &::windows_core::Interface::cast::<IGameControllerProvider>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u16>::zeroed();
            (::windows_core::Interface::vtable(this).HardwareVendorId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u16>(result__)
        }
    }
    pub fn HardwareVersionInfo(&self) -> ::windows_core::Result<GameControllerVersionInfo> {
        let this = &::windows_core::Interface::cast::<IGameControllerProvider>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<GameControllerVersionInfo>::zeroed();
            (::windows_core::Interface::vtable(this).HardwareVersionInfo)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<GameControllerVersionInfo>(result__)
        }
    }
    pub fn IsConnected(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IGameControllerProvider>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsConnected)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn UsageId(&self) -> ::windows_core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u16>::zeroed();
            (::windows_core::Interface::vtable(this).UsageId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u16>(result__)
        }
    }
    pub fn UsagePage(&self) -> ::windows_core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u16>::zeroed();
            (::windows_core::Interface::vtable(this).UsagePage)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u16>(result__)
        }
    }
    pub fn GetFeatureReport(&self, reportid: u8, reportbuffer: &mut [u8]) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).GetFeatureReport)(::windows_core::Interface::as_raw(this), reportid, reportbuffer.len() as u32, ::core::mem::transmute_copy(&reportbuffer)).ok() }
    }
    pub fn SendFeatureReport(&self, reportid: u8, reportbuffer: &[u8]) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SendFeatureReport)(::windows_core::Interface::as_raw(this), reportid, reportbuffer.len() as u32, ::core::mem::transmute(reportbuffer.as_ptr())).ok() }
    }
    pub fn SendOutputReport(&self, reportid: u8, reportbuffer: &[u8]) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SendOutputReport)(::windows_core::Interface::as_raw(this), reportid, reportbuffer.len() as u32, ::core::mem::transmute(reportbuffer.as_ptr())).ok() }
    }
}
impl ::core::clone::Clone for HidGameControllerProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for HidGameControllerProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HidGameControllerProvider {}
impl ::core::fmt::Debug for HidGameControllerProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HidGameControllerProvider").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for HidGameControllerProvider {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Gaming.Input.Custom.HidGameControllerProvider;{95ce3af4-abf0-4b68-a081-3b7de73ff0e7})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for HidGameControllerProvider {
    type Vtable = IHidGameControllerProvider_Vtbl;
    const IID: ::windows_core::GUID = <IHidGameControllerProvider as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for HidGameControllerProvider {
    const NAME: &'static str = "Windows.Gaming.Input.Custom.HidGameControllerProvider";
}
impl ::core::convert::From<HidGameControllerProvider> for ::windows_core::IUnknown {
    fn from(value: HidGameControllerProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HidGameControllerProvider> for ::windows_core::IUnknown {
    fn from(value: &HidGameControllerProvider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for HidGameControllerProvider {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a HidGameControllerProvider {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<HidGameControllerProvider> for ::windows_core::IInspectable {
    fn from(value: HidGameControllerProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HidGameControllerProvider> for ::windows_core::IInspectable {
    fn from(value: &HidGameControllerProvider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for HidGameControllerProvider {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a HidGameControllerProvider {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<HidGameControllerProvider> for IGameControllerProvider {
    type Error = ::windows_core::Error;
    fn try_from(value: HidGameControllerProvider) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&HidGameControllerProvider> for IGameControllerProvider {
    type Error = ::windows_core::Error;
    fn try_from(value: &HidGameControllerProvider) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IGameControllerProvider> for HidGameControllerProvider {
    fn into_param(self) -> ::windows_core::Param<'a, IGameControllerProvider> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IGameControllerProvider> for &HidGameControllerProvider {
    fn into_param(self) -> ::windows_core::Param<'a, IGameControllerProvider> {
        ::core::convert::TryInto::<IGameControllerProvider>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for HidGameControllerProvider {}
unsafe impl ::core::marker::Sync for HidGameControllerProvider {}
#[repr(transparent)]
pub struct ICustomGameControllerFactory(::windows_core::IUnknown);
impl ICustomGameControllerFactory {
    pub fn CreateGameController<'a, Param0: ::windows_core::IntoParam<'a, IGameControllerProvider>>(&self, provider: Param0) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows_core::Interface::vtable(this).CreateGameController)(::windows_core::Interface::as_raw(this), provider.into_param().abi(), result__.as_mut_ptr()).from_abi::<::windows_core::IInspectable>(result__)
        }
    }
    pub fn OnGameControllerAdded<'a, Param0: ::windows_core::IntoParam<'a, super::IGameController>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).OnGameControllerAdded)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn OnGameControllerRemoved<'a, Param0: ::windows_core::IntoParam<'a, super::IGameController>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).OnGameControllerRemoved)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
}
impl ::core::convert::From<ICustomGameControllerFactory> for ::windows_core::IUnknown {
    fn from(value: ICustomGameControllerFactory) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ICustomGameControllerFactory> for ::windows_core::IUnknown {
    fn from(value: &ICustomGameControllerFactory) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ICustomGameControllerFactory {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ICustomGameControllerFactory {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ICustomGameControllerFactory> for ::windows_core::IInspectable {
    fn from(value: ICustomGameControllerFactory) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ICustomGameControllerFactory> for ::windows_core::IInspectable {
    fn from(value: &ICustomGameControllerFactory) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for ICustomGameControllerFactory {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a ICustomGameControllerFactory {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ICustomGameControllerFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ICustomGameControllerFactory {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICustomGameControllerFactory {}
impl ::core::fmt::Debug for ICustomGameControllerFactory {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICustomGameControllerFactory").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for ICustomGameControllerFactory {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"{69a0ae5e-758e-4cbe-ace6-62155fe9126f}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for ICustomGameControllerFactory {
    type Vtable = ICustomGameControllerFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x69a0ae5e_758e_4cbe_ace6_62155fe9126f);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICustomGameControllerFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CreateGameController: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, provider: ::windows_core::RawPtr, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub OnGameControllerAdded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub OnGameControllerRemoved: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGameControllerFactoryManagerStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGameControllerFactoryManagerStatics {
    type Vtable = IGameControllerFactoryManagerStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x36cb66e3_d0a1_4986_a24c_40b137deba9e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGameControllerFactoryManagerStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub RegisterCustomFactoryForGipInterface: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, factory: ::windows_core::RawPtr, interfaceid: ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub RegisterCustomFactoryForHardwareId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, factory: ::windows_core::RawPtr, hardwarevendorid: u16, hardwareproductid: u16) -> ::windows_core::HRESULT,
    pub RegisterCustomFactoryForXusbType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, factory: ::windows_core::RawPtr, xusbtype: XusbDeviceType, xusbsubtype: XusbDeviceSubtype) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGameControllerFactoryManagerStatics2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGameControllerFactoryManagerStatics2 {
    type Vtable = IGameControllerFactoryManagerStatics2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xeace5644_19df_4115_b32a_2793e2aea3bb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGameControllerFactoryManagerStatics2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub TryGetFactoryControllerFromGameController: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, factory: ::windows_core::RawPtr, gamecontroller: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IGameControllerInputSink(::windows_core::IUnknown);
impl IGameControllerInputSink {
    pub fn OnInputResumed(&self, timestamp: u64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).OnInputResumed)(::windows_core::Interface::as_raw(this), timestamp).ok() }
    }
    pub fn OnInputSuspended(&self, timestamp: u64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).OnInputSuspended)(::windows_core::Interface::as_raw(this), timestamp).ok() }
    }
}
impl ::core::convert::From<IGameControllerInputSink> for ::windows_core::IUnknown {
    fn from(value: IGameControllerInputSink) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IGameControllerInputSink> for ::windows_core::IUnknown {
    fn from(value: &IGameControllerInputSink) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IGameControllerInputSink {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IGameControllerInputSink {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IGameControllerInputSink> for ::windows_core::IInspectable {
    fn from(value: IGameControllerInputSink) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IGameControllerInputSink> for ::windows_core::IInspectable {
    fn from(value: &IGameControllerInputSink) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for IGameControllerInputSink {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a IGameControllerInputSink {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IGameControllerInputSink {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IGameControllerInputSink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IGameControllerInputSink {}
impl ::core::fmt::Debug for IGameControllerInputSink {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IGameControllerInputSink").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for IGameControllerInputSink {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"{1ff6f922-c640-4c78-a820-9a715c558bcb}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for IGameControllerInputSink {
    type Vtable = IGameControllerInputSink_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1ff6f922_c640_4c78_a820_9a715c558bcb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGameControllerInputSink_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub OnInputResumed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, timestamp: u64) -> ::windows_core::HRESULT,
    pub OnInputSuspended: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, timestamp: u64) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IGameControllerProvider(::windows_core::IUnknown);
impl IGameControllerProvider {
    pub fn FirmwareVersionInfo(&self) -> ::windows_core::Result<GameControllerVersionInfo> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<GameControllerVersionInfo>::zeroed();
            (::windows_core::Interface::vtable(this).FirmwareVersionInfo)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<GameControllerVersionInfo>(result__)
        }
    }
    pub fn HardwareProductId(&self) -> ::windows_core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u16>::zeroed();
            (::windows_core::Interface::vtable(this).HardwareProductId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u16>(result__)
        }
    }
    pub fn HardwareVendorId(&self) -> ::windows_core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u16>::zeroed();
            (::windows_core::Interface::vtable(this).HardwareVendorId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u16>(result__)
        }
    }
    pub fn HardwareVersionInfo(&self) -> ::windows_core::Result<GameControllerVersionInfo> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<GameControllerVersionInfo>::zeroed();
            (::windows_core::Interface::vtable(this).HardwareVersionInfo)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<GameControllerVersionInfo>(result__)
        }
    }
    pub fn IsConnected(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsConnected)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
}
impl ::core::convert::From<IGameControllerProvider> for ::windows_core::IUnknown {
    fn from(value: IGameControllerProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IGameControllerProvider> for ::windows_core::IUnknown {
    fn from(value: &IGameControllerProvider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IGameControllerProvider {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IGameControllerProvider {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IGameControllerProvider> for ::windows_core::IInspectable {
    fn from(value: IGameControllerProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IGameControllerProvider> for ::windows_core::IInspectable {
    fn from(value: &IGameControllerProvider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for IGameControllerProvider {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a IGameControllerProvider {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IGameControllerProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IGameControllerProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IGameControllerProvider {}
impl ::core::fmt::Debug for IGameControllerProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IGameControllerProvider").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for IGameControllerProvider {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"{e6d73982-2996-4559-b16c-3e57d46e58d6}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for IGameControllerProvider {
    type Vtable = IGameControllerProvider_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe6d73982_2996_4559_b16c_3e57d46e58d6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGameControllerProvider_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub FirmwareVersionInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut GameControllerVersionInfo) -> ::windows_core::HRESULT,
    pub HardwareProductId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows_core::HRESULT,
    pub HardwareVendorId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows_core::HRESULT,
    pub HardwareVersionInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut GameControllerVersionInfo) -> ::windows_core::HRESULT,
    pub IsConnected: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGipFirmwareUpdateResult(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGipFirmwareUpdateResult {
    type Vtable = IGipFirmwareUpdateResult_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6b794d32_8553_4292_8e03_e16651a2f8bc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGipFirmwareUpdateResult_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub ExtendedErrorCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub FinalComponentId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut GipFirmwareUpdateStatus) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IGipGameControllerInputSink(::windows_core::IUnknown);
impl IGipGameControllerInputSink {
    pub fn OnKeyReceived(&self, timestamp: u64, keycode: u8, ispressed: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).OnKeyReceived)(::windows_core::Interface::as_raw(this), timestamp, keycode, ispressed).ok() }
    }
    pub fn OnMessageReceived(&self, timestamp: u64, messageclass: GipMessageClass, messageid: u8, sequenceid: u8, messagebuffer: &[u8]) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).OnMessageReceived)(::windows_core::Interface::as_raw(this), timestamp, messageclass, messageid, sequenceid, messagebuffer.len() as u32, ::core::mem::transmute(messagebuffer.as_ptr())).ok() }
    }
    pub fn OnInputResumed(&self, timestamp: u64) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IGameControllerInputSink>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).OnInputResumed)(::windows_core::Interface::as_raw(this), timestamp).ok() }
    }
    pub fn OnInputSuspended(&self, timestamp: u64) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IGameControllerInputSink>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).OnInputSuspended)(::windows_core::Interface::as_raw(this), timestamp).ok() }
    }
}
impl ::core::convert::From<IGipGameControllerInputSink> for ::windows_core::IUnknown {
    fn from(value: IGipGameControllerInputSink) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IGipGameControllerInputSink> for ::windows_core::IUnknown {
    fn from(value: &IGipGameControllerInputSink) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IGipGameControllerInputSink {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IGipGameControllerInputSink {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IGipGameControllerInputSink> for ::windows_core::IInspectable {
    fn from(value: IGipGameControllerInputSink) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IGipGameControllerInputSink> for ::windows_core::IInspectable {
    fn from(value: &IGipGameControllerInputSink) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for IGipGameControllerInputSink {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a IGipGameControllerInputSink {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<IGipGameControllerInputSink> for IGameControllerInputSink {
    type Error = ::windows_core::Error;
    fn try_from(value: IGipGameControllerInputSink) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IGipGameControllerInputSink> for IGameControllerInputSink {
    type Error = ::windows_core::Error;
    fn try_from(value: &IGipGameControllerInputSink) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IGameControllerInputSink> for IGipGameControllerInputSink {
    fn into_param(self) -> ::windows_core::Param<'a, IGameControllerInputSink> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IGameControllerInputSink> for &IGipGameControllerInputSink {
    fn into_param(self) -> ::windows_core::Param<'a, IGameControllerInputSink> {
        ::core::convert::TryInto::<IGameControllerInputSink>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::clone::Clone for IGipGameControllerInputSink {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IGipGameControllerInputSink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IGipGameControllerInputSink {}
impl ::core::fmt::Debug for IGipGameControllerInputSink {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IGipGameControllerInputSink").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for IGipGameControllerInputSink {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"{a2108abf-09f1-43bc-a140-80f899ec36fb}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for IGipGameControllerInputSink {
    type Vtable = IGipGameControllerInputSink_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa2108abf_09f1_43bc_a140_80f899ec36fb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGipGameControllerInputSink_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub OnKeyReceived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, timestamp: u64, keycode: u8, ispressed: bool) -> ::windows_core::HRESULT,
    pub OnMessageReceived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, timestamp: u64, messageclass: GipMessageClass, messageid: u8, sequenceid: u8, messageBuffer_array_size: u32, messagebuffer: *const u8) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGipGameControllerProvider(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGipGameControllerProvider {
    type Vtable = IGipGameControllerProvider_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xdbcf1e19_1af5_45a8_bf02_a0ee50c823fc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGipGameControllerProvider_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub SendMessage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, messageclass: GipMessageClass, messageid: u8, messageBuffer_array_size: u32, messagebuffer: *const u8) -> ::windows_core::HRESULT,
    pub SendReceiveMessage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, messageclass: GipMessageClass, messageid: u8, requestMessageBuffer_array_size: u32, requestmessagebuffer: *const u8, responseMessageBuffer_array_size: u32, responsemessagebuffer: *mut u8) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub UpdateFirmwareAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, firmwareimage: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    UpdateFirmwareAsync: usize,
}
#[repr(transparent)]
pub struct IHidGameControllerInputSink(::windows_core::IUnknown);
impl IHidGameControllerInputSink {
    pub fn OnInputReportReceived(&self, timestamp: u64, reportid: u8, reportbuffer: &[u8]) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).OnInputReportReceived)(::windows_core::Interface::as_raw(this), timestamp, reportid, reportbuffer.len() as u32, ::core::mem::transmute(reportbuffer.as_ptr())).ok() }
    }
    pub fn OnInputResumed(&self, timestamp: u64) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IGameControllerInputSink>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).OnInputResumed)(::windows_core::Interface::as_raw(this), timestamp).ok() }
    }
    pub fn OnInputSuspended(&self, timestamp: u64) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IGameControllerInputSink>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).OnInputSuspended)(::windows_core::Interface::as_raw(this), timestamp).ok() }
    }
}
impl ::core::convert::From<IHidGameControllerInputSink> for ::windows_core::IUnknown {
    fn from(value: IHidGameControllerInputSink) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IHidGameControllerInputSink> for ::windows_core::IUnknown {
    fn from(value: &IHidGameControllerInputSink) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IHidGameControllerInputSink {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IHidGameControllerInputSink {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IHidGameControllerInputSink> for ::windows_core::IInspectable {
    fn from(value: IHidGameControllerInputSink) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IHidGameControllerInputSink> for ::windows_core::IInspectable {
    fn from(value: &IHidGameControllerInputSink) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for IHidGameControllerInputSink {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a IHidGameControllerInputSink {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<IHidGameControllerInputSink> for IGameControllerInputSink {
    type Error = ::windows_core::Error;
    fn try_from(value: IHidGameControllerInputSink) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IHidGameControllerInputSink> for IGameControllerInputSink {
    type Error = ::windows_core::Error;
    fn try_from(value: &IHidGameControllerInputSink) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IGameControllerInputSink> for IHidGameControllerInputSink {
    fn into_param(self) -> ::windows_core::Param<'a, IGameControllerInputSink> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IGameControllerInputSink> for &IHidGameControllerInputSink {
    fn into_param(self) -> ::windows_core::Param<'a, IGameControllerInputSink> {
        ::core::convert::TryInto::<IGameControllerInputSink>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::clone::Clone for IHidGameControllerInputSink {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IHidGameControllerInputSink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IHidGameControllerInputSink {}
impl ::core::fmt::Debug for IHidGameControllerInputSink {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IHidGameControllerInputSink").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for IHidGameControllerInputSink {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"{f754c322-182d-40e4-a126-fcee4ffa1e31}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for IHidGameControllerInputSink {
    type Vtable = IHidGameControllerInputSink_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf754c322_182d_40e4_a126_fcee4ffa1e31);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHidGameControllerInputSink_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub OnInputReportReceived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, timestamp: u64, reportid: u8, reportBuffer_array_size: u32, reportbuffer: *const u8) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHidGameControllerProvider(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IHidGameControllerProvider {
    type Vtable = IHidGameControllerProvider_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x95ce3af4_abf0_4b68_a081_3b7de73ff0e7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHidGameControllerProvider_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub UsageId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows_core::HRESULT,
    pub UsagePage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows_core::HRESULT,
    pub GetFeatureReport: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reportid: u8, reportBuffer_array_size: u32, reportbuffer: *mut u8) -> ::windows_core::HRESULT,
    pub SendFeatureReport: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reportid: u8, reportBuffer_array_size: u32, reportbuffer: *const u8) -> ::windows_core::HRESULT,
    pub SendOutputReport: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reportid: u8, reportBuffer_array_size: u32, reportbuffer: *const u8) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IXusbGameControllerInputSink(::windows_core::IUnknown);
impl IXusbGameControllerInputSink {
    pub fn OnInputReceived(&self, timestamp: u64, reportid: u8, inputbuffer: &[u8]) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).OnInputReceived)(::windows_core::Interface::as_raw(this), timestamp, reportid, inputbuffer.len() as u32, ::core::mem::transmute(inputbuffer.as_ptr())).ok() }
    }
    pub fn OnInputResumed(&self, timestamp: u64) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IGameControllerInputSink>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).OnInputResumed)(::windows_core::Interface::as_raw(this), timestamp).ok() }
    }
    pub fn OnInputSuspended(&self, timestamp: u64) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IGameControllerInputSink>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).OnInputSuspended)(::windows_core::Interface::as_raw(this), timestamp).ok() }
    }
}
impl ::core::convert::From<IXusbGameControllerInputSink> for ::windows_core::IUnknown {
    fn from(value: IXusbGameControllerInputSink) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IXusbGameControllerInputSink> for ::windows_core::IUnknown {
    fn from(value: &IXusbGameControllerInputSink) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IXusbGameControllerInputSink {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IXusbGameControllerInputSink {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IXusbGameControllerInputSink> for ::windows_core::IInspectable {
    fn from(value: IXusbGameControllerInputSink) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IXusbGameControllerInputSink> for ::windows_core::IInspectable {
    fn from(value: &IXusbGameControllerInputSink) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for IXusbGameControllerInputSink {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a IXusbGameControllerInputSink {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<IXusbGameControllerInputSink> for IGameControllerInputSink {
    type Error = ::windows_core::Error;
    fn try_from(value: IXusbGameControllerInputSink) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IXusbGameControllerInputSink> for IGameControllerInputSink {
    type Error = ::windows_core::Error;
    fn try_from(value: &IXusbGameControllerInputSink) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IGameControllerInputSink> for IXusbGameControllerInputSink {
    fn into_param(self) -> ::windows_core::Param<'a, IGameControllerInputSink> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IGameControllerInputSink> for &IXusbGameControllerInputSink {
    fn into_param(self) -> ::windows_core::Param<'a, IGameControllerInputSink> {
        ::core::convert::TryInto::<IGameControllerInputSink>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::clone::Clone for IXusbGameControllerInputSink {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IXusbGameControllerInputSink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IXusbGameControllerInputSink {}
impl ::core::fmt::Debug for IXusbGameControllerInputSink {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXusbGameControllerInputSink").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for IXusbGameControllerInputSink {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"{b2ac1d95-6ecb-42b3-8aab-025401ca4712}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for IXusbGameControllerInputSink {
    type Vtable = IXusbGameControllerInputSink_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb2ac1d95_6ecb_42b3_8aab_025401ca4712);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXusbGameControllerInputSink_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub OnInputReceived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, timestamp: u64, reportid: u8, inputBuffer_array_size: u32, inputbuffer: *const u8) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IXusbGameControllerProvider(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IXusbGameControllerProvider {
    type Vtable = IXusbGameControllerProvider_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6e2971eb_0efb_48b4_808b_837643b2f216);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXusbGameControllerProvider_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub SetVibration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lowfrequencymotorspeed: f64, highfrequencymotorspeed: f64) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct XusbDeviceSubtype(pub i32);
impl XusbDeviceSubtype {
    pub const Unknown: Self = Self(0i32);
    pub const Gamepad: Self = Self(1i32);
    pub const ArcadePad: Self = Self(2i32);
    pub const ArcadeStick: Self = Self(3i32);
    pub const FlightStick: Self = Self(4i32);
    pub const Wheel: Self = Self(5i32);
    pub const Guitar: Self = Self(6i32);
    pub const GuitarAlternate: Self = Self(7i32);
    pub const GuitarBass: Self = Self(8i32);
    pub const DrumKit: Self = Self(9i32);
    pub const DancePad: Self = Self(10i32);
}
impl ::core::marker::Copy for XusbDeviceSubtype {}
impl ::core::clone::Clone for XusbDeviceSubtype {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for XusbDeviceSubtype {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for XusbDeviceSubtype {
    type Abi = Self;
}
impl ::core::fmt::Debug for XusbDeviceSubtype {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("XusbDeviceSubtype").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for XusbDeviceSubtype {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Gaming.Input.Custom.XusbDeviceSubtype;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct XusbDeviceType(pub i32);
impl XusbDeviceType {
    pub const Unknown: Self = Self(0i32);
    pub const Gamepad: Self = Self(1i32);
}
impl ::core::marker::Copy for XusbDeviceType {}
impl ::core::clone::Clone for XusbDeviceType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for XusbDeviceType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for XusbDeviceType {
    type Abi = Self;
}
impl ::core::fmt::Debug for XusbDeviceType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("XusbDeviceType").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for XusbDeviceType {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Gaming.Input.Custom.XusbDeviceType;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct XusbGameControllerProvider(::windows_core::IUnknown);
impl XusbGameControllerProvider {
    pub fn FirmwareVersionInfo(&self) -> ::windows_core::Result<GameControllerVersionInfo> {
        let this = &::windows_core::Interface::cast::<IGameControllerProvider>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<GameControllerVersionInfo>::zeroed();
            (::windows_core::Interface::vtable(this).FirmwareVersionInfo)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<GameControllerVersionInfo>(result__)
        }
    }
    pub fn HardwareProductId(&self) -> ::windows_core::Result<u16> {
        let this = &::windows_core::Interface::cast::<IGameControllerProvider>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u16>::zeroed();
            (::windows_core::Interface::vtable(this).HardwareProductId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u16>(result__)
        }
    }
    pub fn HardwareVendorId(&self) -> ::windows_core::Result<u16> {
        let this = &::windows_core::Interface::cast::<IGameControllerProvider>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u16>::zeroed();
            (::windows_core::Interface::vtable(this).HardwareVendorId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u16>(result__)
        }
    }
    pub fn HardwareVersionInfo(&self) -> ::windows_core::Result<GameControllerVersionInfo> {
        let this = &::windows_core::Interface::cast::<IGameControllerProvider>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<GameControllerVersionInfo>::zeroed();
            (::windows_core::Interface::vtable(this).HardwareVersionInfo)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<GameControllerVersionInfo>(result__)
        }
    }
    pub fn IsConnected(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IGameControllerProvider>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsConnected)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetVibration(&self, lowfrequencymotorspeed: f64, highfrequencymotorspeed: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetVibration)(::windows_core::Interface::as_raw(this), lowfrequencymotorspeed, highfrequencymotorspeed).ok() }
    }
}
impl ::core::clone::Clone for XusbGameControllerProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for XusbGameControllerProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for XusbGameControllerProvider {}
impl ::core::fmt::Debug for XusbGameControllerProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("XusbGameControllerProvider").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for XusbGameControllerProvider {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Gaming.Input.Custom.XusbGameControllerProvider;{6e2971eb-0efb-48b4-808b-837643b2f216})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for XusbGameControllerProvider {
    type Vtable = IXusbGameControllerProvider_Vtbl;
    const IID: ::windows_core::GUID = <IXusbGameControllerProvider as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for XusbGameControllerProvider {
    const NAME: &'static str = "Windows.Gaming.Input.Custom.XusbGameControllerProvider";
}
impl ::core::convert::From<XusbGameControllerProvider> for ::windows_core::IUnknown {
    fn from(value: XusbGameControllerProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&XusbGameControllerProvider> for ::windows_core::IUnknown {
    fn from(value: &XusbGameControllerProvider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for XusbGameControllerProvider {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a XusbGameControllerProvider {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<XusbGameControllerProvider> for ::windows_core::IInspectable {
    fn from(value: XusbGameControllerProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&XusbGameControllerProvider> for ::windows_core::IInspectable {
    fn from(value: &XusbGameControllerProvider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for XusbGameControllerProvider {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a XusbGameControllerProvider {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<XusbGameControllerProvider> for IGameControllerProvider {
    type Error = ::windows_core::Error;
    fn try_from(value: XusbGameControllerProvider) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&XusbGameControllerProvider> for IGameControllerProvider {
    type Error = ::windows_core::Error;
    fn try_from(value: &XusbGameControllerProvider) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IGameControllerProvider> for XusbGameControllerProvider {
    fn into_param(self) -> ::windows_core::Param<'a, IGameControllerProvider> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IGameControllerProvider> for &XusbGameControllerProvider {
    fn into_param(self) -> ::windows_core::Param<'a, IGameControllerProvider> {
        ::core::convert::TryInto::<IGameControllerProvider>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for XusbGameControllerProvider {}
unsafe impl ::core::marker::Sync for XusbGameControllerProvider {}
