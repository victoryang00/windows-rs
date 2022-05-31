pub const FHCFG_E_CONFIGURATION_PREVIOUSLY_LOADED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147220731i32);
pub const FHCFG_E_CONFIG_ALREADY_EXISTS: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147220734i32);
pub const FHCFG_E_CONFIG_FILE_NOT_FOUND: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147220735i32);
pub const FHCFG_E_CORRUPT_CONFIG_FILE: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147220736i32);
pub const FHCFG_E_INVALID_REHYDRATION_STATE: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147220726i32);
pub const FHCFG_E_LEGACY_BACKUP_NOT_FOUND: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147220715i32);
pub const FHCFG_E_LEGACY_BACKUP_USER_EXCLUDED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147220716i32);
pub const FHCFG_E_LEGACY_TARGET_UNSUPPORTED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147220718i32);
pub const FHCFG_E_LEGACY_TARGET_VALIDATION_UNSUPPORTED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147220717i32);
pub const FHCFG_E_NO_VALID_CONFIGURATION_LOADED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147220733i32);
pub const FHCFG_E_RECOMMENDATION_CHANGE_NOT_ALLOWED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147220720i32);
pub const FHCFG_E_TARGET_CANNOT_BE_USED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147220727i32);
pub const FHCFG_E_TARGET_NOT_CONFIGURED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147220729i32);
pub const FHCFG_E_TARGET_NOT_CONNECTED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147220732i32);
pub const FHCFG_E_TARGET_NOT_ENOUGH_FREE_SPACE: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147220728i32);
pub const FHCFG_E_TARGET_REHYDRATED_ELSEWHERE: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147220719i32);
pub const FHCFG_E_TARGET_VERIFICATION_FAILED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147220730i32);
pub const FHSVC_E_BACKUP_BLOCKED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147219968i32);
pub const FHSVC_E_CONFIG_DISABLED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147219966i32);
pub const FHSVC_E_CONFIG_DISABLED_GP: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147219965i32);
pub const FHSVC_E_CONFIG_REHYDRATING: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147219963i32);
pub const FHSVC_E_FATAL_CONFIG_ERROR: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147219964i32);
pub const FHSVC_E_NOT_CONFIGURED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147219967i32);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct FH_BACKUP_STATUS(pub i32);
pub const FH_STATUS_DISABLED: FH_BACKUP_STATUS = FH_BACKUP_STATUS(0i32);
pub const FH_STATUS_DISABLED_BY_GP: FH_BACKUP_STATUS = FH_BACKUP_STATUS(1i32);
pub const FH_STATUS_ENABLED: FH_BACKUP_STATUS = FH_BACKUP_STATUS(2i32);
pub const FH_STATUS_REHYDRATING: FH_BACKUP_STATUS = FH_BACKUP_STATUS(3i32);
pub const MAX_BACKUP_STATUS: FH_BACKUP_STATUS = FH_BACKUP_STATUS(4i32);
impl ::core::marker::Copy for FH_BACKUP_STATUS {}
impl ::core::clone::Clone for FH_BACKUP_STATUS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FH_BACKUP_STATUS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for FH_BACKUP_STATUS {
    type Abi = Self;
}
impl ::core::fmt::Debug for FH_BACKUP_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FH_BACKUP_STATUS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct FH_DEVICE_VALIDATION_RESULT(pub i32);
pub const FH_ACCESS_DENIED: FH_DEVICE_VALIDATION_RESULT = FH_DEVICE_VALIDATION_RESULT(0i32);
pub const FH_INVALID_DRIVE_TYPE: FH_DEVICE_VALIDATION_RESULT = FH_DEVICE_VALIDATION_RESULT(1i32);
pub const FH_READ_ONLY_PERMISSION: FH_DEVICE_VALIDATION_RESULT = FH_DEVICE_VALIDATION_RESULT(2i32);
pub const FH_CURRENT_DEFAULT: FH_DEVICE_VALIDATION_RESULT = FH_DEVICE_VALIDATION_RESULT(3i32);
pub const FH_NAMESPACE_EXISTS: FH_DEVICE_VALIDATION_RESULT = FH_DEVICE_VALIDATION_RESULT(4i32);
pub const FH_TARGET_PART_OF_LIBRARY: FH_DEVICE_VALIDATION_RESULT = FH_DEVICE_VALIDATION_RESULT(5i32);
pub const FH_VALID_TARGET: FH_DEVICE_VALIDATION_RESULT = FH_DEVICE_VALIDATION_RESULT(6i32);
pub const MAX_VALIDATION_RESULT: FH_DEVICE_VALIDATION_RESULT = FH_DEVICE_VALIDATION_RESULT(7i32);
impl ::core::marker::Copy for FH_DEVICE_VALIDATION_RESULT {}
impl ::core::clone::Clone for FH_DEVICE_VALIDATION_RESULT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FH_DEVICE_VALIDATION_RESULT {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for FH_DEVICE_VALIDATION_RESULT {
    type Abi = Self;
}
impl ::core::fmt::Debug for FH_DEVICE_VALIDATION_RESULT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FH_DEVICE_VALIDATION_RESULT").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct FH_LOCAL_POLICY_TYPE(pub i32);
pub const FH_FREQUENCY: FH_LOCAL_POLICY_TYPE = FH_LOCAL_POLICY_TYPE(0i32);
pub const FH_RETENTION_TYPE: FH_LOCAL_POLICY_TYPE = FH_LOCAL_POLICY_TYPE(1i32);
pub const FH_RETENTION_AGE: FH_LOCAL_POLICY_TYPE = FH_LOCAL_POLICY_TYPE(2i32);
pub const MAX_LOCAL_POLICY: FH_LOCAL_POLICY_TYPE = FH_LOCAL_POLICY_TYPE(3i32);
impl ::core::marker::Copy for FH_LOCAL_POLICY_TYPE {}
impl ::core::clone::Clone for FH_LOCAL_POLICY_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FH_LOCAL_POLICY_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for FH_LOCAL_POLICY_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for FH_LOCAL_POLICY_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FH_LOCAL_POLICY_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct FH_PROTECTED_ITEM_CATEGORY(pub i32);
pub const FH_FOLDER: FH_PROTECTED_ITEM_CATEGORY = FH_PROTECTED_ITEM_CATEGORY(0i32);
pub const FH_LIBRARY: FH_PROTECTED_ITEM_CATEGORY = FH_PROTECTED_ITEM_CATEGORY(1i32);
pub const MAX_PROTECTED_ITEM_CATEGORY: FH_PROTECTED_ITEM_CATEGORY = FH_PROTECTED_ITEM_CATEGORY(2i32);
impl ::core::marker::Copy for FH_PROTECTED_ITEM_CATEGORY {}
impl ::core::clone::Clone for FH_PROTECTED_ITEM_CATEGORY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FH_PROTECTED_ITEM_CATEGORY {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for FH_PROTECTED_ITEM_CATEGORY {
    type Abi = Self;
}
impl ::core::fmt::Debug for FH_PROTECTED_ITEM_CATEGORY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FH_PROTECTED_ITEM_CATEGORY").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct FH_RETENTION_TYPES(pub i32);
pub const FH_RETENTION_DISABLED: FH_RETENTION_TYPES = FH_RETENTION_TYPES(0i32);
pub const FH_RETENTION_UNLIMITED: FH_RETENTION_TYPES = FH_RETENTION_TYPES(1i32);
pub const FH_RETENTION_AGE_BASED: FH_RETENTION_TYPES = FH_RETENTION_TYPES(2i32);
pub const MAX_RETENTION_TYPE: FH_RETENTION_TYPES = FH_RETENTION_TYPES(3i32);
impl ::core::marker::Copy for FH_RETENTION_TYPES {}
impl ::core::clone::Clone for FH_RETENTION_TYPES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FH_RETENTION_TYPES {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for FH_RETENTION_TYPES {
    type Abi = Self;
}
impl ::core::fmt::Debug for FH_RETENTION_TYPES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FH_RETENTION_TYPES").field(&self.0).finish()
    }
}
pub const FH_STATE_BACKUP_NOT_SUPPORTED: u32 = 2064u32;
pub const FH_STATE_DISABLED_BY_GP: u32 = 2u32;
pub const FH_STATE_FATAL_CONFIG_ERROR: u32 = 3u32;
pub const FH_STATE_MIGRATING: u32 = 4u32;
pub const FH_STATE_NOT_TRACKED: u32 = 0u32;
pub const FH_STATE_NO_ERROR: u32 = 255u32;
pub const FH_STATE_OFF: u32 = 1u32;
pub const FH_STATE_REHYDRATING: u32 = 5u32;
pub const FH_STATE_RUNNING: u32 = 256u32;
pub const FH_STATE_STAGING_FULL: u32 = 18u32;
pub const FH_STATE_TARGET_ABSENT: u32 = 21u32;
pub const FH_STATE_TARGET_ACCESS_DENIED: u32 = 14u32;
pub const FH_STATE_TARGET_FS_LIMITATION: u32 = 13u32;
pub const FH_STATE_TARGET_FULL: u32 = 17u32;
pub const FH_STATE_TARGET_FULL_RETENTION_MAX: u32 = 16u32;
pub const FH_STATE_TARGET_LOW_SPACE: u32 = 20u32;
pub const FH_STATE_TARGET_LOW_SPACE_RETENTION_MAX: u32 = 19u32;
pub const FH_STATE_TARGET_VOLUME_DIRTY: u32 = 15u32;
pub const FH_STATE_TOO_MUCH_BEHIND: u32 = 240u32;
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct FH_TARGET_DRIVE_TYPES(pub i32);
pub const FH_DRIVE_UNKNOWN: FH_TARGET_DRIVE_TYPES = FH_TARGET_DRIVE_TYPES(0i32);
pub const FH_DRIVE_REMOVABLE: FH_TARGET_DRIVE_TYPES = FH_TARGET_DRIVE_TYPES(2i32);
pub const FH_DRIVE_FIXED: FH_TARGET_DRIVE_TYPES = FH_TARGET_DRIVE_TYPES(3i32);
pub const FH_DRIVE_REMOTE: FH_TARGET_DRIVE_TYPES = FH_TARGET_DRIVE_TYPES(4i32);
impl ::core::marker::Copy for FH_TARGET_DRIVE_TYPES {}
impl ::core::clone::Clone for FH_TARGET_DRIVE_TYPES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FH_TARGET_DRIVE_TYPES {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for FH_TARGET_DRIVE_TYPES {
    type Abi = Self;
}
impl ::core::fmt::Debug for FH_TARGET_DRIVE_TYPES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FH_TARGET_DRIVE_TYPES").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct FH_TARGET_PROPERTY_TYPE(pub i32);
pub const FH_TARGET_NAME: FH_TARGET_PROPERTY_TYPE = FH_TARGET_PROPERTY_TYPE(0i32);
pub const FH_TARGET_URL: FH_TARGET_PROPERTY_TYPE = FH_TARGET_PROPERTY_TYPE(1i32);
pub const FH_TARGET_DRIVE_TYPE: FH_TARGET_PROPERTY_TYPE = FH_TARGET_PROPERTY_TYPE(2i32);
pub const MAX_TARGET_PROPERTY: FH_TARGET_PROPERTY_TYPE = FH_TARGET_PROPERTY_TYPE(3i32);
impl ::core::marker::Copy for FH_TARGET_PROPERTY_TYPE {}
impl ::core::clone::Clone for FH_TARGET_PROPERTY_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FH_TARGET_PROPERTY_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for FH_TARGET_PROPERTY_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for FH_TARGET_PROPERTY_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FH_TARGET_PROPERTY_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct FhBackupStopReason(pub i32);
pub const BackupInvalidStopReason: FhBackupStopReason = FhBackupStopReason(0i32);
pub const BackupLimitUserBusyMachineOnAC: FhBackupStopReason = FhBackupStopReason(1i32);
pub const BackupLimitUserIdleMachineOnDC: FhBackupStopReason = FhBackupStopReason(2i32);
pub const BackupLimitUserBusyMachineOnDC: FhBackupStopReason = FhBackupStopReason(3i32);
pub const BackupCancelled: FhBackupStopReason = FhBackupStopReason(4i32);
impl ::core::marker::Copy for FhBackupStopReason {}
impl ::core::clone::Clone for FhBackupStopReason {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FhBackupStopReason {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for FhBackupStopReason {
    type Abi = Self;
}
impl ::core::fmt::Debug for FhBackupStopReason {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FhBackupStopReason").field(&self.0).finish()
    }
}
pub const FhConfigMgr: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xed43bb3c_09e9_498a_9df6_2177244c6db4);
pub const FhReassociation: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4d728e35_16fa_4320_9e8b_bfd7100a8846);
#[cfg(feature = "Win32_System_WindowsProgramming")]
#[inline]
pub unsafe fn FhServiceBlockBackup<'a, Param0: ::windows_core::IntoParam<'a, ::win32_system::WindowsProgramming::FH_SERVICE_PIPE_HANDLE>>(pipe: Param0) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FhServiceBlockBackup(pipe: ::win32_system::WindowsProgramming::FH_SERVICE_PIPE_HANDLE) -> ::windows_core::HRESULT;
        }
        FhServiceBlockBackup(pipe.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_WindowsProgramming")]
#[inline]
pub unsafe fn FhServiceClosePipe<'a, Param0: ::windows_core::IntoParam<'a, ::win32_system::WindowsProgramming::FH_SERVICE_PIPE_HANDLE>>(pipe: Param0) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FhServiceClosePipe(pipe: ::win32_system::WindowsProgramming::FH_SERVICE_PIPE_HANDLE) -> ::windows_core::HRESULT;
        }
        FhServiceClosePipe(pipe.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_WindowsProgramming")]
#[inline]
pub unsafe fn FhServiceOpenPipe<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(startserviceifstopped: Param0) -> ::windows_core::Result<::win32_system::WindowsProgramming::FH_SERVICE_PIPE_HANDLE> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FhServiceOpenPipe(startserviceifstopped: ::win32_foundation::BOOL, pipe: *mut ::win32_system::WindowsProgramming::FH_SERVICE_PIPE_HANDLE) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<::win32_system::WindowsProgramming::FH_SERVICE_PIPE_HANDLE>::zeroed();
        FhServiceOpenPipe(startserviceifstopped.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_system::WindowsProgramming::FH_SERVICE_PIPE_HANDLE>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_WindowsProgramming")]
#[inline]
pub unsafe fn FhServiceReloadConfiguration<'a, Param0: ::windows_core::IntoParam<'a, ::win32_system::WindowsProgramming::FH_SERVICE_PIPE_HANDLE>>(pipe: Param0) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FhServiceReloadConfiguration(pipe: ::win32_system::WindowsProgramming::FH_SERVICE_PIPE_HANDLE) -> ::windows_core::HRESULT;
        }
        FhServiceReloadConfiguration(pipe.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_WindowsProgramming")]
#[inline]
pub unsafe fn FhServiceStartBackup<'a, Param0: ::windows_core::IntoParam<'a, ::win32_system::WindowsProgramming::FH_SERVICE_PIPE_HANDLE>, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(pipe: Param0, lowpriorityio: Param1) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FhServiceStartBackup(pipe: ::win32_system::WindowsProgramming::FH_SERVICE_PIPE_HANDLE, lowpriorityio: ::win32_foundation::BOOL) -> ::windows_core::HRESULT;
        }
        FhServiceStartBackup(pipe.into_param().abi(), lowpriorityio.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_WindowsProgramming")]
#[inline]
pub unsafe fn FhServiceStopBackup<'a, Param0: ::windows_core::IntoParam<'a, ::win32_system::WindowsProgramming::FH_SERVICE_PIPE_HANDLE>, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(pipe: Param0, stoptracking: Param1) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FhServiceStopBackup(pipe: ::win32_system::WindowsProgramming::FH_SERVICE_PIPE_HANDLE, stoptracking: ::win32_foundation::BOOL) -> ::windows_core::HRESULT;
        }
        FhServiceStopBackup(pipe.into_param().abi(), stoptracking.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_WindowsProgramming")]
#[inline]
pub unsafe fn FhServiceUnblockBackup<'a, Param0: ::windows_core::IntoParam<'a, ::win32_system::WindowsProgramming::FH_SERVICE_PIPE_HANDLE>>(pipe: Param0) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FhServiceUnblockBackup(pipe: ::win32_system::WindowsProgramming::FH_SERVICE_PIPE_HANDLE) -> ::windows_core::HRESULT;
        }
        FhServiceUnblockBackup(pipe.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[repr(transparent)]
pub struct IFhConfigMgr(::windows_core::IUnknown);
impl IFhConfigMgr {
    pub unsafe fn LoadConfiguration(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).LoadConfiguration)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn CreateDefaultConfiguration<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(&self, overwriteifexists: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).CreateDefaultConfiguration)(::windows_core::Interface::as_raw(self), overwriteifexists.into_param().abi()).ok()
    }
    pub unsafe fn SaveConfiguration(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SaveConfiguration)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn AddRemoveExcludeRule<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>, Param2: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, add: Param0, category: FH_PROTECTED_ITEM_CATEGORY, item: Param2) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).AddRemoveExcludeRule)(::windows_core::Interface::as_raw(self), add.into_param().abi(), ::core::mem::transmute(category), item.into_param().abi()).ok()
    }
    pub unsafe fn GetIncludeExcludeRules<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(&self, include: Param0, category: FH_PROTECTED_ITEM_CATEGORY) -> ::windows_core::Result<IFhScopeIterator> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetIncludeExcludeRules)(::windows_core::Interface::as_raw(self), include.into_param().abi(), ::core::mem::transmute(category), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IFhScopeIterator>(result__)
    }
    pub unsafe fn GetLocalPolicy(&self, localpolicytype: FH_LOCAL_POLICY_TYPE) -> ::windows_core::Result<u64> {
        let mut result__ = ::core::mem::MaybeUninit::<u64>::zeroed();
        (::windows_core::Interface::vtable(self).GetLocalPolicy)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(localpolicytype), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u64>(result__)
    }
    pub unsafe fn SetLocalPolicy(&self, localpolicytype: FH_LOCAL_POLICY_TYPE, policyvalue: u64) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetLocalPolicy)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(localpolicytype), ::core::mem::transmute(policyvalue)).ok()
    }
    pub unsafe fn GetBackupStatus(&self) -> ::windows_core::Result<FH_BACKUP_STATUS> {
        let mut result__ = ::core::mem::MaybeUninit::<FH_BACKUP_STATUS>::zeroed();
        (::windows_core::Interface::vtable(self).GetBackupStatus)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<FH_BACKUP_STATUS>(result__)
    }
    pub unsafe fn SetBackupStatus(&self, backupstatus: FH_BACKUP_STATUS) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetBackupStatus)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(backupstatus)).ok()
    }
    pub unsafe fn GetDefaultTarget(&self) -> ::windows_core::Result<IFhTarget> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetDefaultTarget)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IFhTarget>(result__)
    }
    pub unsafe fn ValidateTarget<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, targeturl: Param0) -> ::windows_core::Result<FH_DEVICE_VALIDATION_RESULT> {
        let mut result__ = ::core::mem::MaybeUninit::<FH_DEVICE_VALIDATION_RESULT>::zeroed();
        (::windows_core::Interface::vtable(self).ValidateTarget)(::windows_core::Interface::as_raw(self), targeturl.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<FH_DEVICE_VALIDATION_RESULT>(result__)
    }
    pub unsafe fn ProvisionAndSetNewTarget<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, targeturl: Param0, targetname: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ProvisionAndSetNewTarget)(::windows_core::Interface::as_raw(self), targeturl.into_param().abi(), targetname.into_param().abi()).ok()
    }
    pub unsafe fn ChangeDefaultTargetRecommendation<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(&self, recommend: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ChangeDefaultTargetRecommendation)(::windows_core::Interface::as_raw(self), recommend.into_param().abi()).ok()
    }
    pub unsafe fn QueryProtectionStatus(&self, protectionstate: *mut u32, protecteduntiltime: *mut ::win32_foundation::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).QueryProtectionStatus)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(protectionstate), ::core::mem::transmute(protecteduntiltime)).ok()
    }
}
impl ::core::convert::From<IFhConfigMgr> for ::windows_core::IUnknown {
    fn from(value: IFhConfigMgr) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IFhConfigMgr> for ::windows_core::IUnknown {
    fn from(value: &IFhConfigMgr) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IFhConfigMgr {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IFhConfigMgr {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IFhConfigMgr {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IFhConfigMgr {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFhConfigMgr {}
impl ::core::fmt::Debug for IFhConfigMgr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFhConfigMgr").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IFhConfigMgr {
    type Vtable = IFhConfigMgr_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6a5fea5b_bf8f_4ee5_b8c3_44d8a0d7331c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFhConfigMgr_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub LoadConfiguration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub CreateDefaultConfiguration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, overwriteifexists: ::win32_foundation::BOOL) -> ::windows_core::HRESULT,
    pub SaveConfiguration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub AddRemoveExcludeRule: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, add: ::win32_foundation::BOOL, category: FH_PROTECTED_ITEM_CATEGORY, item: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub GetIncludeExcludeRules: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, include: ::win32_foundation::BOOL, category: FH_PROTECTED_ITEM_CATEGORY, iterator: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetLocalPolicy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, localpolicytype: FH_LOCAL_POLICY_TYPE, policyvalue: *mut u64) -> ::windows_core::HRESULT,
    pub SetLocalPolicy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, localpolicytype: FH_LOCAL_POLICY_TYPE, policyvalue: u64) -> ::windows_core::HRESULT,
    pub GetBackupStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, backupstatus: *mut FH_BACKUP_STATUS) -> ::windows_core::HRESULT,
    pub SetBackupStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, backupstatus: FH_BACKUP_STATUS) -> ::windows_core::HRESULT,
    pub GetDefaultTarget: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, defaulttarget: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ValidateTarget: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, targeturl: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, validationresult: *mut FH_DEVICE_VALIDATION_RESULT) -> ::windows_core::HRESULT,
    pub ProvisionAndSetNewTarget: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, targeturl: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, targetname: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub ChangeDefaultTargetRecommendation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, recommend: ::win32_foundation::BOOL) -> ::windows_core::HRESULT,
    pub QueryProtectionStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, protectionstate: *mut u32, protecteduntiltime: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IFhReassociation(::windows_core::IUnknown);
impl IFhReassociation {
    pub unsafe fn ValidateTarget<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, targeturl: Param0) -> ::windows_core::Result<FH_DEVICE_VALIDATION_RESULT> {
        let mut result__ = ::core::mem::MaybeUninit::<FH_DEVICE_VALIDATION_RESULT>::zeroed();
        (::windows_core::Interface::vtable(self).ValidateTarget)(::windows_core::Interface::as_raw(self), targeturl.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<FH_DEVICE_VALIDATION_RESULT>(result__)
    }
    pub unsafe fn ScanTargetForConfigurations<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, targeturl: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ScanTargetForConfigurations)(::windows_core::Interface::as_raw(self), targeturl.into_param().abi()).ok()
    }
    pub unsafe fn GetConfigurationDetails(&self, index: u32, username: *mut ::win32_foundation::BSTR, pcname: *mut ::win32_foundation::BSTR, backuptime: *mut ::win32_foundation::FILETIME) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetConfigurationDetails)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(index), ::core::mem::transmute(username), ::core::mem::transmute(pcname), ::core::mem::transmute(backuptime)).ok()
    }
    pub unsafe fn SelectConfiguration(&self, index: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SelectConfiguration)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(index)).ok()
    }
    pub unsafe fn PerformReassociation<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(&self, overwriteifexists: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).PerformReassociation)(::windows_core::Interface::as_raw(self), overwriteifexists.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IFhReassociation> for ::windows_core::IUnknown {
    fn from(value: IFhReassociation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IFhReassociation> for ::windows_core::IUnknown {
    fn from(value: &IFhReassociation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IFhReassociation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IFhReassociation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IFhReassociation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IFhReassociation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFhReassociation {}
impl ::core::fmt::Debug for IFhReassociation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFhReassociation").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IFhReassociation {
    type Vtable = IFhReassociation_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6544a28a_f68d_47ac_91ef_16b2b36aa3ee);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFhReassociation_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub ValidateTarget: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, targeturl: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, validationresult: *mut FH_DEVICE_VALIDATION_RESULT) -> ::windows_core::HRESULT,
    pub ScanTargetForConfigurations: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, targeturl: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub GetConfigurationDetails: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, username: *mut ::win32_foundation::BSTR, pcname: *mut ::win32_foundation::BSTR, backuptime: *mut ::win32_foundation::FILETIME) -> ::windows_core::HRESULT,
    pub SelectConfiguration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32) -> ::windows_core::HRESULT,
    pub PerformReassociation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, overwriteifexists: ::win32_foundation::BOOL) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IFhScopeIterator(::windows_core::IUnknown);
impl IFhScopeIterator {
    pub unsafe fn MoveToNextItem(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).MoveToNextItem)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn GetItem(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).GetItem)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
}
impl ::core::convert::From<IFhScopeIterator> for ::windows_core::IUnknown {
    fn from(value: IFhScopeIterator) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IFhScopeIterator> for ::windows_core::IUnknown {
    fn from(value: &IFhScopeIterator) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IFhScopeIterator {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IFhScopeIterator {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IFhScopeIterator {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IFhScopeIterator {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFhScopeIterator {}
impl ::core::fmt::Debug for IFhScopeIterator {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFhScopeIterator").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IFhScopeIterator {
    type Vtable = IFhScopeIterator_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3197abce_532a_44c6_8615_f3666566a720);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFhScopeIterator_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub MoveToNextItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, item: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IFhTarget(::windows_core::IUnknown);
impl IFhTarget {
    pub unsafe fn GetStringProperty(&self, propertytype: FH_TARGET_PROPERTY_TYPE) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).GetStringProperty)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(propertytype), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn GetNumericalProperty(&self, propertytype: FH_TARGET_PROPERTY_TYPE) -> ::windows_core::Result<u64> {
        let mut result__ = ::core::mem::MaybeUninit::<u64>::zeroed();
        (::windows_core::Interface::vtable(self).GetNumericalProperty)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(propertytype), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u64>(result__)
    }
}
impl ::core::convert::From<IFhTarget> for ::windows_core::IUnknown {
    fn from(value: IFhTarget) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IFhTarget> for ::windows_core::IUnknown {
    fn from(value: &IFhTarget) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IFhTarget {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IFhTarget {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IFhTarget {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IFhTarget {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFhTarget {}
impl ::core::fmt::Debug for IFhTarget {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFhTarget").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IFhTarget {
    type Vtable = IFhTarget_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd87965fd_2bad_4657_bd3b_9567eb300ced);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFhTarget_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub GetStringProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, propertytype: FH_TARGET_PROPERTY_TYPE, propertyvalue: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub GetNumericalProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, propertytype: FH_TARGET_PROPERTY_TYPE, propertyvalue: *mut u64) -> ::windows_core::HRESULT,
}
