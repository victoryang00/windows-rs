#[doc(hidden)]
#[repr(transparent)]
pub struct ISystemUpdateItem(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISystemUpdateItem {
    type Vtable = ISystemUpdateItem_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x779740eb_5624_519e_a8e2_09e9173b3fb7);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISystemUpdateItem_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub State: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SystemUpdateItemState) -> ::windows_core::HRESULT,
    pub Title: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Description: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Revision: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub DownloadProgress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub InstallProgress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub ExtendedError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::HRESULT) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISystemUpdateLastErrorInfo(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISystemUpdateLastErrorInfo {
    type Vtable = ISystemUpdateLastErrorInfo_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7ee887f7_8a44_5b6e_bd07_7aece4116ea9);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISystemUpdateLastErrorInfo_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub State: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SystemUpdateManagerState) -> ::windows_core::HRESULT,
    pub ExtendedError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::HRESULT) -> ::windows_core::HRESULT,
    pub IsInteractive: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISystemUpdateManagerStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISystemUpdateManagerStatics {
    type Vtable = ISystemUpdateManagerStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb2d3fcef_2971_51be_b41a_8bd703bb701a);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISystemUpdateManagerStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub IsSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub State: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SystemUpdateManagerState) -> ::windows_core::HRESULT,
    pub StateChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows_core::RawPtr, result__: *mut ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveStateChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::winrt_foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub DownloadProgress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub InstallProgress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub UserActiveHoursStart: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::TimeSpan) -> ::windows_core::HRESULT,
    pub UserActiveHoursEnd: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::TimeSpan) -> ::windows_core::HRESULT,
    pub UserActiveHoursMax: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub TrySetUserActiveHours: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, start: ::winrt_foundation::TimeSpan, end: ::winrt_foundation::TimeSpan, result__: *mut bool) -> ::windows_core::HRESULT,
    pub LastUpdateCheckTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::DateTime) -> ::windows_core::HRESULT,
    pub LastUpdateInstallTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::winrt_foundation::DateTime) -> ::windows_core::HRESULT,
    pub LastErrorInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub GetAutomaticRebootBlockIds: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetAutomaticRebootBlockIds: usize,
    pub BlockAutomaticRebootAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lockid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub UnblockAutomaticRebootAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lockid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ExtendedError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::HRESULT) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub GetUpdateItems: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetUpdateItems: usize,
    pub AttentionRequiredReason: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SystemUpdateAttentionRequiredReason) -> ::windows_core::HRESULT,
    pub SetFlightRing: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, flightring: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut bool) -> ::windows_core::HRESULT,
    pub GetFlightRing: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub StartInstall: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, action: SystemUpdateStartInstallAction) -> ::windows_core::HRESULT,
    pub RebootToCompleteInstall: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub StartCancelUpdates: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct SystemUpdateAttentionRequiredReason(pub i32);
impl SystemUpdateAttentionRequiredReason {
    pub const None: Self = Self(0i32);
    pub const NetworkRequired: Self = Self(1i32);
    pub const InsufficientDiskSpace: Self = Self(2i32);
    pub const InsufficientBattery: Self = Self(3i32);
    pub const UpdateBlocked: Self = Self(4i32);
}
impl ::core::marker::Copy for SystemUpdateAttentionRequiredReason {}
impl ::core::clone::Clone for SystemUpdateAttentionRequiredReason {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SystemUpdateAttentionRequiredReason {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for SystemUpdateAttentionRequiredReason {
    type Abi = Self;
}
impl ::core::fmt::Debug for SystemUpdateAttentionRequiredReason {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SystemUpdateAttentionRequiredReason").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SystemUpdateAttentionRequiredReason {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.System.Update.SystemUpdateAttentionRequiredReason;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct SystemUpdateItem(::windows_core::IUnknown);
impl SystemUpdateItem {
    pub fn State(&self) -> ::windows_core::Result<SystemUpdateItemState> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<SystemUpdateItemState>::zeroed();
            (::windows_core::Interface::vtable(this).State)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SystemUpdateItemState>(result__)
        }
    }
    pub fn Title(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Title)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn Description(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Description)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn Id(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Id)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn Revision(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).Revision)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn DownloadProgress(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).DownloadProgress)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn InstallProgress(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).InstallProgress)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn ExtendedError(&self) -> ::windows_core::Result<::windows_core::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::HRESULT>::zeroed();
            (::windows_core::Interface::vtable(this).ExtendedError)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HRESULT>(result__)
        }
    }
}
impl ::core::clone::Clone for SystemUpdateItem {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SystemUpdateItem {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SystemUpdateItem {}
impl ::core::fmt::Debug for SystemUpdateItem {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SystemUpdateItem").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SystemUpdateItem {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.System.Update.SystemUpdateItem;{779740eb-5624-519e-a8e2-09e9173b3fb7})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for SystemUpdateItem {
    type Vtable = ISystemUpdateItem_Vtbl;
    const IID: ::windows_core::GUID = <ISystemUpdateItem as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for SystemUpdateItem {
    const NAME: &'static str = "Windows.System.Update.SystemUpdateItem";
}
impl ::core::convert::From<SystemUpdateItem> for ::windows_core::IUnknown {
    fn from(value: SystemUpdateItem) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SystemUpdateItem> for ::windows_core::IUnknown {
    fn from(value: &SystemUpdateItem) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for SystemUpdateItem {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a SystemUpdateItem {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SystemUpdateItem> for ::windows_core::IInspectable {
    fn from(value: SystemUpdateItem) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SystemUpdateItem> for ::windows_core::IInspectable {
    fn from(value: &SystemUpdateItem) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for SystemUpdateItem {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a SystemUpdateItem {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for SystemUpdateItem {}
unsafe impl ::core::marker::Sync for SystemUpdateItem {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct SystemUpdateItemState(pub i32);
impl SystemUpdateItemState {
    pub const NotStarted: Self = Self(0i32);
    pub const Initializing: Self = Self(1i32);
    pub const Preparing: Self = Self(2i32);
    pub const Calculating: Self = Self(3i32);
    pub const Downloading: Self = Self(4i32);
    pub const Installing: Self = Self(5i32);
    pub const Completed: Self = Self(6i32);
    pub const RebootRequired: Self = Self(7i32);
    pub const Error: Self = Self(8i32);
}
impl ::core::marker::Copy for SystemUpdateItemState {}
impl ::core::clone::Clone for SystemUpdateItemState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SystemUpdateItemState {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for SystemUpdateItemState {
    type Abi = Self;
}
impl ::core::fmt::Debug for SystemUpdateItemState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SystemUpdateItemState").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SystemUpdateItemState {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.System.Update.SystemUpdateItemState;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct SystemUpdateLastErrorInfo(::windows_core::IUnknown);
impl SystemUpdateLastErrorInfo {
    pub fn State(&self) -> ::windows_core::Result<SystemUpdateManagerState> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<SystemUpdateManagerState>::zeroed();
            (::windows_core::Interface::vtable(this).State)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SystemUpdateManagerState>(result__)
        }
    }
    pub fn ExtendedError(&self) -> ::windows_core::Result<::windows_core::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::HRESULT>::zeroed();
            (::windows_core::Interface::vtable(this).ExtendedError)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HRESULT>(result__)
        }
    }
    pub fn IsInteractive(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsInteractive)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
}
impl ::core::clone::Clone for SystemUpdateLastErrorInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SystemUpdateLastErrorInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SystemUpdateLastErrorInfo {}
impl ::core::fmt::Debug for SystemUpdateLastErrorInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SystemUpdateLastErrorInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SystemUpdateLastErrorInfo {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.System.Update.SystemUpdateLastErrorInfo;{7ee887f7-8a44-5b6e-bd07-7aece4116ea9})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for SystemUpdateLastErrorInfo {
    type Vtable = ISystemUpdateLastErrorInfo_Vtbl;
    const IID: ::windows_core::GUID = <ISystemUpdateLastErrorInfo as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for SystemUpdateLastErrorInfo {
    const NAME: &'static str = "Windows.System.Update.SystemUpdateLastErrorInfo";
}
impl ::core::convert::From<SystemUpdateLastErrorInfo> for ::windows_core::IUnknown {
    fn from(value: SystemUpdateLastErrorInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SystemUpdateLastErrorInfo> for ::windows_core::IUnknown {
    fn from(value: &SystemUpdateLastErrorInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for SystemUpdateLastErrorInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a SystemUpdateLastErrorInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SystemUpdateLastErrorInfo> for ::windows_core::IInspectable {
    fn from(value: SystemUpdateLastErrorInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SystemUpdateLastErrorInfo> for ::windows_core::IInspectable {
    fn from(value: &SystemUpdateLastErrorInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for SystemUpdateLastErrorInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a SystemUpdateLastErrorInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for SystemUpdateLastErrorInfo {}
unsafe impl ::core::marker::Sync for SystemUpdateLastErrorInfo {}
pub struct SystemUpdateManager;
impl SystemUpdateManager {
    pub fn IsSupported() -> ::windows_core::Result<bool> {
        Self::ISystemUpdateManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsSupported)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        })
    }
    pub fn State() -> ::windows_core::Result<SystemUpdateManagerState> {
        Self::ISystemUpdateManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<SystemUpdateManagerState>::zeroed();
            (::windows_core::Interface::vtable(this).State)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SystemUpdateManagerState>(result__)
        })
    }
    pub fn StateChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventHandler<::windows_core::IInspectable>>>(handler: Param0) -> ::windows_core::Result<::winrt_foundation::EventRegistrationToken> {
        Self::ISystemUpdateManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::EventRegistrationToken>::zeroed();
            (::windows_core::Interface::vtable(this).StateChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::EventRegistrationToken>(result__)
        })
    }
    pub fn RemoveStateChanged<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::EventRegistrationToken>>(token: Param0) -> ::windows_core::Result<()> {
        Self::ISystemUpdateManagerStatics(|this| unsafe { (::windows_core::Interface::vtable(this).RemoveStateChanged)(::windows_core::Interface::as_raw(this), token.into_param().abi()).ok() })
    }
    pub fn DownloadProgress() -> ::windows_core::Result<f64> {
        Self::ISystemUpdateManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).DownloadProgress)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        })
    }
    pub fn InstallProgress() -> ::windows_core::Result<f64> {
        Self::ISystemUpdateManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).InstallProgress)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        })
    }
    pub fn UserActiveHoursStart() -> ::windows_core::Result<::winrt_foundation::TimeSpan> {
        Self::ISystemUpdateManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::TimeSpan>::zeroed();
            (::windows_core::Interface::vtable(this).UserActiveHoursStart)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::TimeSpan>(result__)
        })
    }
    pub fn UserActiveHoursEnd() -> ::windows_core::Result<::winrt_foundation::TimeSpan> {
        Self::ISystemUpdateManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::TimeSpan>::zeroed();
            (::windows_core::Interface::vtable(this).UserActiveHoursEnd)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::TimeSpan>(result__)
        })
    }
    pub fn UserActiveHoursMax() -> ::windows_core::Result<i32> {
        Self::ISystemUpdateManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows_core::Interface::vtable(this).UserActiveHoursMax)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        })
    }
    pub fn TrySetUserActiveHours<'a, Param0: ::windows_core::IntoParam<'a, ::winrt_foundation::TimeSpan>, Param1: ::windows_core::IntoParam<'a, ::winrt_foundation::TimeSpan>>(start: Param0, end: Param1) -> ::windows_core::Result<bool> {
        Self::ISystemUpdateManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).TrySetUserActiveHours)(::windows_core::Interface::as_raw(this), start.into_param().abi(), end.into_param().abi(), result__.as_mut_ptr()).from_abi::<bool>(result__)
        })
    }
    pub fn LastUpdateCheckTime() -> ::windows_core::Result<::winrt_foundation::DateTime> {
        Self::ISystemUpdateManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::DateTime>::zeroed();
            (::windows_core::Interface::vtable(this).LastUpdateCheckTime)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::DateTime>(result__)
        })
    }
    pub fn LastUpdateInstallTime() -> ::windows_core::Result<::winrt_foundation::DateTime> {
        Self::ISystemUpdateManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::winrt_foundation::DateTime>::zeroed();
            (::windows_core::Interface::vtable(this).LastUpdateInstallTime)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::DateTime>(result__)
        })
    }
    pub fn LastErrorInfo() -> ::windows_core::Result<SystemUpdateLastErrorInfo> {
        Self::ISystemUpdateManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).LastErrorInfo)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SystemUpdateLastErrorInfo>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetAutomaticRebootBlockIds() -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<::windows_core::HSTRING>> {
        Self::ISystemUpdateManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetAutomaticRebootBlockIds)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<::windows_core::HSTRING>>(result__)
        })
    }
    pub fn BlockAutomaticRebootAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(lockid: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<bool>> {
        Self::ISystemUpdateManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).BlockAutomaticRebootAsync)(::windows_core::Interface::as_raw(this), lockid.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<bool>>(result__)
        })
    }
    pub fn UnblockAutomaticRebootAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(lockid: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<bool>> {
        Self::ISystemUpdateManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).UnblockAutomaticRebootAsync)(::windows_core::Interface::as_raw(this), lockid.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<bool>>(result__)
        })
    }
    pub fn ExtendedError() -> ::windows_core::Result<::windows_core::HRESULT> {
        Self::ISystemUpdateManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::HRESULT>::zeroed();
            (::windows_core::Interface::vtable(this).ExtendedError)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HRESULT>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetUpdateItems() -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<SystemUpdateItem>> {
        Self::ISystemUpdateManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetUpdateItems)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<SystemUpdateItem>>(result__)
        })
    }
    pub fn AttentionRequiredReason() -> ::windows_core::Result<SystemUpdateAttentionRequiredReason> {
        Self::ISystemUpdateManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<SystemUpdateAttentionRequiredReason>::zeroed();
            (::windows_core::Interface::vtable(this).AttentionRequiredReason)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SystemUpdateAttentionRequiredReason>(result__)
        })
    }
    pub fn SetFlightRing<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(flightring: Param0) -> ::windows_core::Result<bool> {
        Self::ISystemUpdateManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).SetFlightRing)(::windows_core::Interface::as_raw(this), flightring.into_param().abi(), result__.as_mut_ptr()).from_abi::<bool>(result__)
        })
    }
    pub fn GetFlightRing() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ISystemUpdateManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).GetFlightRing)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        })
    }
    pub fn StartInstall(action: SystemUpdateStartInstallAction) -> ::windows_core::Result<()> {
        Self::ISystemUpdateManagerStatics(|this| unsafe { (::windows_core::Interface::vtable(this).StartInstall)(::windows_core::Interface::as_raw(this), action).ok() })
    }
    pub fn RebootToCompleteInstall() -> ::windows_core::Result<()> {
        Self::ISystemUpdateManagerStatics(|this| unsafe { (::windows_core::Interface::vtable(this).RebootToCompleteInstall)(::windows_core::Interface::as_raw(this)).ok() })
    }
    pub fn StartCancelUpdates() -> ::windows_core::Result<()> {
        Self::ISystemUpdateManagerStatics(|this| unsafe { (::windows_core::Interface::vtable(this).StartCancelUpdates)(::windows_core::Interface::as_raw(this)).ok() })
    }
    pub fn ISystemUpdateManagerStatics<R, F: FnOnce(&ISystemUpdateManagerStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<SystemUpdateManager, ISystemUpdateManagerStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows_core::RuntimeName for SystemUpdateManager {
    const NAME: &'static str = "Windows.System.Update.SystemUpdateManager";
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct SystemUpdateManagerState(pub i32);
impl SystemUpdateManagerState {
    pub const Idle: Self = Self(0i32);
    pub const Detecting: Self = Self(1i32);
    pub const ReadyToDownload: Self = Self(2i32);
    pub const Downloading: Self = Self(3i32);
    pub const ReadyToInstall: Self = Self(4i32);
    pub const Installing: Self = Self(5i32);
    pub const RebootRequired: Self = Self(6i32);
    pub const ReadyToFinalize: Self = Self(7i32);
    pub const Finalizing: Self = Self(8i32);
    pub const Completed: Self = Self(9i32);
    pub const AttentionRequired: Self = Self(10i32);
    pub const Error: Self = Self(11i32);
}
impl ::core::marker::Copy for SystemUpdateManagerState {}
impl ::core::clone::Clone for SystemUpdateManagerState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SystemUpdateManagerState {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for SystemUpdateManagerState {
    type Abi = Self;
}
impl ::core::fmt::Debug for SystemUpdateManagerState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SystemUpdateManagerState").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SystemUpdateManagerState {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.System.Update.SystemUpdateManagerState;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct SystemUpdateStartInstallAction(pub i32);
impl SystemUpdateStartInstallAction {
    pub const UpToReboot: Self = Self(0i32);
    pub const AllowReboot: Self = Self(1i32);
}
impl ::core::marker::Copy for SystemUpdateStartInstallAction {}
impl ::core::clone::Clone for SystemUpdateStartInstallAction {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SystemUpdateStartInstallAction {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for SystemUpdateStartInstallAction {
    type Abi = Self;
}
impl ::core::fmt::Debug for SystemUpdateStartInstallAction {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SystemUpdateStartInstallAction").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for SystemUpdateStartInstallAction {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.System.Update.SystemUpdateStartInstallAction;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
