#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct AddServiceFlag(pub i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const asfAllowPendingRegistration: AddServiceFlag = AddServiceFlag(1i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const asfAllowOnlineRegistration: AddServiceFlag = AddServiceFlag(2i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const asfRegisterServiceWithAU: AddServiceFlag = AddServiceFlag(4i32);
impl ::core::marker::Copy for AddServiceFlag {}
impl ::core::clone::Clone for AddServiceFlag {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AddServiceFlag {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for AddServiceFlag {
    type Abi = Self;
}
impl ::core::fmt::Debug for AddServiceFlag {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AddServiceFlag").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct AutoDownloadMode(pub i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const adLetWindowsUpdateDecide: AutoDownloadMode = AutoDownloadMode(0i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const adNeverAutoDownload: AutoDownloadMode = AutoDownloadMode(1i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const adAlwaysAutoDownload: AutoDownloadMode = AutoDownloadMode(2i32);
impl ::core::marker::Copy for AutoDownloadMode {}
impl ::core::clone::Clone for AutoDownloadMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AutoDownloadMode {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for AutoDownloadMode {
    type Abi = Self;
}
impl ::core::fmt::Debug for AutoDownloadMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AutoDownloadMode").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct AutoSelectionMode(pub i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const asLetWindowsUpdateDecide: AutoSelectionMode = AutoSelectionMode(0i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const asAutoSelectIfDownloaded: AutoSelectionMode = AutoSelectionMode(1i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const asNeverAutoSelect: AutoSelectionMode = AutoSelectionMode(2i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const asAlwaysAutoSelect: AutoSelectionMode = AutoSelectionMode(3i32);
impl ::core::marker::Copy for AutoSelectionMode {}
impl ::core::clone::Clone for AutoSelectionMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AutoSelectionMode {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for AutoSelectionMode {
    type Abi = Self;
}
impl ::core::fmt::Debug for AutoSelectionMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AutoSelectionMode").field(&self.0).finish()
    }
}
pub const AutomaticUpdates: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xbfe18e9c_6d87_4450_b37c_e02f0b373803);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct AutomaticUpdatesNotificationLevel(pub i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const aunlNotConfigured: AutomaticUpdatesNotificationLevel = AutomaticUpdatesNotificationLevel(0i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const aunlDisabled: AutomaticUpdatesNotificationLevel = AutomaticUpdatesNotificationLevel(1i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const aunlNotifyBeforeDownload: AutomaticUpdatesNotificationLevel = AutomaticUpdatesNotificationLevel(2i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const aunlNotifyBeforeInstallation: AutomaticUpdatesNotificationLevel = AutomaticUpdatesNotificationLevel(3i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const aunlScheduledInstallation: AutomaticUpdatesNotificationLevel = AutomaticUpdatesNotificationLevel(4i32);
impl ::core::marker::Copy for AutomaticUpdatesNotificationLevel {}
impl ::core::clone::Clone for AutomaticUpdatesNotificationLevel {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AutomaticUpdatesNotificationLevel {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for AutomaticUpdatesNotificationLevel {
    type Abi = Self;
}
impl ::core::fmt::Debug for AutomaticUpdatesNotificationLevel {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AutomaticUpdatesNotificationLevel").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct AutomaticUpdatesPermissionType(pub i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const auptSetNotificationLevel: AutomaticUpdatesPermissionType = AutomaticUpdatesPermissionType(1i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const auptDisableAutomaticUpdates: AutomaticUpdatesPermissionType = AutomaticUpdatesPermissionType(2i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const auptSetIncludeRecommendedUpdates: AutomaticUpdatesPermissionType = AutomaticUpdatesPermissionType(3i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const auptSetFeaturedUpdatesEnabled: AutomaticUpdatesPermissionType = AutomaticUpdatesPermissionType(4i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const auptSetNonAdministratorsElevated: AutomaticUpdatesPermissionType = AutomaticUpdatesPermissionType(5i32);
impl ::core::marker::Copy for AutomaticUpdatesPermissionType {}
impl ::core::clone::Clone for AutomaticUpdatesPermissionType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AutomaticUpdatesPermissionType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for AutomaticUpdatesPermissionType {
    type Abi = Self;
}
impl ::core::fmt::Debug for AutomaticUpdatesPermissionType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AutomaticUpdatesPermissionType").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct AutomaticUpdatesScheduledInstallationDay(pub i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const ausidEveryDay: AutomaticUpdatesScheduledInstallationDay = AutomaticUpdatesScheduledInstallationDay(0i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const ausidEverySunday: AutomaticUpdatesScheduledInstallationDay = AutomaticUpdatesScheduledInstallationDay(1i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const ausidEveryMonday: AutomaticUpdatesScheduledInstallationDay = AutomaticUpdatesScheduledInstallationDay(2i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const ausidEveryTuesday: AutomaticUpdatesScheduledInstallationDay = AutomaticUpdatesScheduledInstallationDay(3i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const ausidEveryWednesday: AutomaticUpdatesScheduledInstallationDay = AutomaticUpdatesScheduledInstallationDay(4i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const ausidEveryThursday: AutomaticUpdatesScheduledInstallationDay = AutomaticUpdatesScheduledInstallationDay(5i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const ausidEveryFriday: AutomaticUpdatesScheduledInstallationDay = AutomaticUpdatesScheduledInstallationDay(6i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const ausidEverySaturday: AutomaticUpdatesScheduledInstallationDay = AutomaticUpdatesScheduledInstallationDay(7i32);
impl ::core::marker::Copy for AutomaticUpdatesScheduledInstallationDay {}
impl ::core::clone::Clone for AutomaticUpdatesScheduledInstallationDay {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AutomaticUpdatesScheduledInstallationDay {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for AutomaticUpdatesScheduledInstallationDay {
    type Abi = Self;
}
impl ::core::fmt::Debug for AutomaticUpdatesScheduledInstallationDay {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AutomaticUpdatesScheduledInstallationDay").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct AutomaticUpdatesUserType(pub i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const auutCurrentUser: AutomaticUpdatesUserType = AutomaticUpdatesUserType(1i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const auutLocalAdministrator: AutomaticUpdatesUserType = AutomaticUpdatesUserType(2i32);
impl ::core::marker::Copy for AutomaticUpdatesUserType {}
impl ::core::clone::Clone for AutomaticUpdatesUserType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AutomaticUpdatesUserType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for AutomaticUpdatesUserType {
    type Abi = Self;
}
impl ::core::fmt::Debug for AutomaticUpdatesUserType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AutomaticUpdatesUserType").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DeploymentAction(pub i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const daNone: DeploymentAction = DeploymentAction(0i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const daInstallation: DeploymentAction = DeploymentAction(1i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const daUninstallation: DeploymentAction = DeploymentAction(2i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const daDetection: DeploymentAction = DeploymentAction(3i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const daOptionalInstallation: DeploymentAction = DeploymentAction(4i32);
impl ::core::marker::Copy for DeploymentAction {}
impl ::core::clone::Clone for DeploymentAction {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DeploymentAction {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for DeploymentAction {
    type Abi = Self;
}
impl ::core::fmt::Debug for DeploymentAction {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DeploymentAction").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DownloadPhase(pub i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const dphInitializing: DownloadPhase = DownloadPhase(1i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const dphDownloading: DownloadPhase = DownloadPhase(2i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const dphVerifying: DownloadPhase = DownloadPhase(3i32);
impl ::core::marker::Copy for DownloadPhase {}
impl ::core::clone::Clone for DownloadPhase {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DownloadPhase {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for DownloadPhase {
    type Abi = Self;
}
impl ::core::fmt::Debug for DownloadPhase {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DownloadPhase").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DownloadPriority(pub i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const dpLow: DownloadPriority = DownloadPriority(1i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const dpNormal: DownloadPriority = DownloadPriority(2i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const dpHigh: DownloadPriority = DownloadPriority(3i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const dpExtraHigh: DownloadPriority = DownloadPriority(4i32);
impl ::core::marker::Copy for DownloadPriority {}
impl ::core::clone::Clone for DownloadPriority {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DownloadPriority {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for DownloadPriority {
    type Abi = Self;
}
impl ::core::fmt::Debug for DownloadPriority {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DownloadPriority").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IAutomaticUpdates(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IAutomaticUpdates {
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn DetectNow(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).DetectNow)(::windows_core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn Pause(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Pause)(::windows_core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn Resume(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Resume)(::windows_core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn ShowSettingsDialog(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ShowSettingsDialog)(::windows_core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Settings(&self) -> ::windows_core::Result<IAutomaticUpdatesSettings> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Settings)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IAutomaticUpdatesSettings>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn ServiceEnabled(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).ServiceEnabled)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn EnableService(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).EnableService)(::windows_core::Interface::as_raw(self)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IAutomaticUpdates> for ::windows_core::IUnknown {
    fn from(value: IAutomaticUpdates) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IAutomaticUpdates> for ::windows_core::IUnknown {
    fn from(value: &IAutomaticUpdates) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IAutomaticUpdates {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IAutomaticUpdates {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IAutomaticUpdates> for super::Com::IDispatch {
    fn from(value: IAutomaticUpdates) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IAutomaticUpdates> for super::Com::IDispatch {
    fn from(value: &IAutomaticUpdates) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for IAutomaticUpdates {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a IAutomaticUpdates {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IAutomaticUpdates {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IAutomaticUpdates {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IAutomaticUpdates {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IAutomaticUpdates {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAutomaticUpdates").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IAutomaticUpdates {
    type Vtable = IAutomaticUpdates_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x673425bf_c082_4c7c_bdfd_569464b8e0ce);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IAutomaticUpdates_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub DetectNow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Pause: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Resume: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub ShowSettingsDialog: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Settings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Settings: usize,
    pub ServiceEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows_core::HRESULT,
    pub EnableService: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IAutomaticUpdates2(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IAutomaticUpdates2 {
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn DetectNow(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.DetectNow)(::windows_core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn Pause(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Pause)(::windows_core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn Resume(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Resume)(::windows_core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn ShowSettingsDialog(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.ShowSettingsDialog)(::windows_core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Settings(&self) -> ::windows_core::Result<IAutomaticUpdatesSettings> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.Settings)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IAutomaticUpdatesSettings>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn ServiceEnabled(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).base__.ServiceEnabled)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn EnableService(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.EnableService)(::windows_core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Results(&self) -> ::windows_core::Result<IAutomaticUpdatesResults> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Results)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IAutomaticUpdatesResults>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IAutomaticUpdates2> for ::windows_core::IUnknown {
    fn from(value: IAutomaticUpdates2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IAutomaticUpdates2> for ::windows_core::IUnknown {
    fn from(value: &IAutomaticUpdates2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IAutomaticUpdates2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IAutomaticUpdates2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IAutomaticUpdates2> for super::Com::IDispatch {
    fn from(value: IAutomaticUpdates2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IAutomaticUpdates2> for super::Com::IDispatch {
    fn from(value: &IAutomaticUpdates2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for IAutomaticUpdates2 {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a IAutomaticUpdates2 {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IAutomaticUpdates2> for IAutomaticUpdates {
    fn from(value: IAutomaticUpdates2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IAutomaticUpdates2> for IAutomaticUpdates {
    fn from(value: &IAutomaticUpdates2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, IAutomaticUpdates> for IAutomaticUpdates2 {
    fn into_param(self) -> ::windows_core::Param<'a, IAutomaticUpdates> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, IAutomaticUpdates> for &'a IAutomaticUpdates2 {
    fn into_param(self) -> ::windows_core::Param<'a, IAutomaticUpdates> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IAutomaticUpdates2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IAutomaticUpdates2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IAutomaticUpdates2 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IAutomaticUpdates2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAutomaticUpdates2").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IAutomaticUpdates2 {
    type Vtable = IAutomaticUpdates2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4a2f5c31_cfd9_410e_b7fb_29a653973a0f);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IAutomaticUpdates2_Vtbl {
    pub base__: IAutomaticUpdates_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Results: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Results: usize,
}
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IAutomaticUpdatesResults(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IAutomaticUpdatesResults {
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn LastSearchSuccessDate(&self) -> ::windows_core::Result<super::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).LastSearchSuccessDate)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::VARIANT>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn LastInstallationSuccessDate(&self) -> ::windows_core::Result<super::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).LastInstallationSuccessDate)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::VARIANT>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IAutomaticUpdatesResults> for ::windows_core::IUnknown {
    fn from(value: IAutomaticUpdatesResults) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IAutomaticUpdatesResults> for ::windows_core::IUnknown {
    fn from(value: &IAutomaticUpdatesResults) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IAutomaticUpdatesResults {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IAutomaticUpdatesResults {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IAutomaticUpdatesResults> for super::Com::IDispatch {
    fn from(value: IAutomaticUpdatesResults) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IAutomaticUpdatesResults> for super::Com::IDispatch {
    fn from(value: &IAutomaticUpdatesResults) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for IAutomaticUpdatesResults {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a IAutomaticUpdatesResults {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IAutomaticUpdatesResults {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IAutomaticUpdatesResults {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IAutomaticUpdatesResults {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IAutomaticUpdatesResults {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAutomaticUpdatesResults").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IAutomaticUpdatesResults {
    type Vtable = IAutomaticUpdatesResults_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe7a4d634_7942_4dd9_a111_82228ba33901);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IAutomaticUpdatesResults_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub LastSearchSuccessDate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut super::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    LastSearchSuccessDate: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub LastInstallationSuccessDate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut super::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    LastInstallationSuccessDate: usize,
}
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IAutomaticUpdatesSettings(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IAutomaticUpdatesSettings {
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn NotificationLevel(&self) -> ::windows_core::Result<AutomaticUpdatesNotificationLevel> {
        let mut result__ = ::core::mem::MaybeUninit::<AutomaticUpdatesNotificationLevel>::zeroed();
        (::windows_core::Interface::vtable(self).NotificationLevel)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<AutomaticUpdatesNotificationLevel>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn SetNotificationLevel(&self, value: AutomaticUpdatesNotificationLevel) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetNotificationLevel)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn ReadOnly(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).ReadOnly)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn Required(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).Required)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn ScheduledInstallationDay(&self) -> ::windows_core::Result<AutomaticUpdatesScheduledInstallationDay> {
        let mut result__ = ::core::mem::MaybeUninit::<AutomaticUpdatesScheduledInstallationDay>::zeroed();
        (::windows_core::Interface::vtable(self).ScheduledInstallationDay)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<AutomaticUpdatesScheduledInstallationDay>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn SetScheduledInstallationDay(&self, value: AutomaticUpdatesScheduledInstallationDay) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetScheduledInstallationDay)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn ScheduledInstallationTime(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).ScheduledInstallationTime)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn SetScheduledInstallationTime(&self, value: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetScheduledInstallationTime)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn Refresh(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Refresh)(::windows_core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn Save(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Save)(::windows_core::Interface::as_raw(self)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IAutomaticUpdatesSettings> for ::windows_core::IUnknown {
    fn from(value: IAutomaticUpdatesSettings) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IAutomaticUpdatesSettings> for ::windows_core::IUnknown {
    fn from(value: &IAutomaticUpdatesSettings) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IAutomaticUpdatesSettings {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IAutomaticUpdatesSettings {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IAutomaticUpdatesSettings> for super::Com::IDispatch {
    fn from(value: IAutomaticUpdatesSettings) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IAutomaticUpdatesSettings> for super::Com::IDispatch {
    fn from(value: &IAutomaticUpdatesSettings) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for IAutomaticUpdatesSettings {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a IAutomaticUpdatesSettings {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IAutomaticUpdatesSettings {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IAutomaticUpdatesSettings {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IAutomaticUpdatesSettings {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IAutomaticUpdatesSettings {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAutomaticUpdatesSettings").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IAutomaticUpdatesSettings {
    type Vtable = IAutomaticUpdatesSettings_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2ee48f22_af3c_405f_8970_f71be12ee9a2);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IAutomaticUpdatesSettings_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub NotificationLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut AutomaticUpdatesNotificationLevel) -> ::windows_core::HRESULT,
    pub SetNotificationLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: AutomaticUpdatesNotificationLevel) -> ::windows_core::HRESULT,
    pub ReadOnly: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows_core::HRESULT,
    pub Required: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows_core::HRESULT,
    pub ScheduledInstallationDay: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut AutomaticUpdatesScheduledInstallationDay) -> ::windows_core::HRESULT,
    pub SetScheduledInstallationDay: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: AutomaticUpdatesScheduledInstallationDay) -> ::windows_core::HRESULT,
    pub ScheduledInstallationTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT,
    pub SetScheduledInstallationTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows_core::HRESULT,
    pub Refresh: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Save: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IAutomaticUpdatesSettings2(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IAutomaticUpdatesSettings2 {
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn NotificationLevel(&self) -> ::windows_core::Result<AutomaticUpdatesNotificationLevel> {
        let mut result__ = ::core::mem::MaybeUninit::<AutomaticUpdatesNotificationLevel>::zeroed();
        (::windows_core::Interface::vtable(self).base__.NotificationLevel)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<AutomaticUpdatesNotificationLevel>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn SetNotificationLevel(&self, value: AutomaticUpdatesNotificationLevel) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetNotificationLevel)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn ReadOnly(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).base__.ReadOnly)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn Required(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).base__.Required)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn ScheduledInstallationDay(&self) -> ::windows_core::Result<AutomaticUpdatesScheduledInstallationDay> {
        let mut result__ = ::core::mem::MaybeUninit::<AutomaticUpdatesScheduledInstallationDay>::zeroed();
        (::windows_core::Interface::vtable(self).base__.ScheduledInstallationDay)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<AutomaticUpdatesScheduledInstallationDay>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn SetScheduledInstallationDay(&self, value: AutomaticUpdatesScheduledInstallationDay) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetScheduledInstallationDay)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn ScheduledInstallationTime(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).base__.ScheduledInstallationTime)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn SetScheduledInstallationTime(&self, value: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetScheduledInstallationTime)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn Refresh(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Refresh)(::windows_core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn Save(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Save)(::windows_core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn IncludeRecommendedUpdates(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).IncludeRecommendedUpdates)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn SetIncludeRecommendedUpdates(&self, value: i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetIncludeRecommendedUpdates)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn CheckPermission(&self, usertype: AutomaticUpdatesUserType, permissiontype: AutomaticUpdatesPermissionType) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).CheckPermission)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(usertype), ::core::mem::transmute(permissiontype), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IAutomaticUpdatesSettings2> for ::windows_core::IUnknown {
    fn from(value: IAutomaticUpdatesSettings2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IAutomaticUpdatesSettings2> for ::windows_core::IUnknown {
    fn from(value: &IAutomaticUpdatesSettings2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IAutomaticUpdatesSettings2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IAutomaticUpdatesSettings2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IAutomaticUpdatesSettings2> for super::Com::IDispatch {
    fn from(value: IAutomaticUpdatesSettings2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IAutomaticUpdatesSettings2> for super::Com::IDispatch {
    fn from(value: &IAutomaticUpdatesSettings2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for IAutomaticUpdatesSettings2 {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a IAutomaticUpdatesSettings2 {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IAutomaticUpdatesSettings2> for IAutomaticUpdatesSettings {
    fn from(value: IAutomaticUpdatesSettings2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IAutomaticUpdatesSettings2> for IAutomaticUpdatesSettings {
    fn from(value: &IAutomaticUpdatesSettings2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, IAutomaticUpdatesSettings> for IAutomaticUpdatesSettings2 {
    fn into_param(self) -> ::windows_core::Param<'a, IAutomaticUpdatesSettings> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, IAutomaticUpdatesSettings> for &'a IAutomaticUpdatesSettings2 {
    fn into_param(self) -> ::windows_core::Param<'a, IAutomaticUpdatesSettings> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IAutomaticUpdatesSettings2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IAutomaticUpdatesSettings2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IAutomaticUpdatesSettings2 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IAutomaticUpdatesSettings2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAutomaticUpdatesSettings2").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IAutomaticUpdatesSettings2 {
    type Vtable = IAutomaticUpdatesSettings2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6abc136a_c3ca_4384_8171_cb2b1e59b8dc);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IAutomaticUpdatesSettings2_Vtbl {
    pub base__: IAutomaticUpdatesSettings_Vtbl,
    pub IncludeRecommendedUpdates: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows_core::HRESULT,
    pub SetIncludeRecommendedUpdates: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i16) -> ::windows_core::HRESULT,
    pub CheckPermission: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, usertype: AutomaticUpdatesUserType, permissiontype: AutomaticUpdatesPermissionType, userhaspermission: *mut i16) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IAutomaticUpdatesSettings3(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IAutomaticUpdatesSettings3 {
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn NotificationLevel(&self) -> ::windows_core::Result<AutomaticUpdatesNotificationLevel> {
        let mut result__ = ::core::mem::MaybeUninit::<AutomaticUpdatesNotificationLevel>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.NotificationLevel)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<AutomaticUpdatesNotificationLevel>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn SetNotificationLevel(&self, value: AutomaticUpdatesNotificationLevel) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.SetNotificationLevel)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn ReadOnly(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.ReadOnly)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn Required(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.Required)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn ScheduledInstallationDay(&self) -> ::windows_core::Result<AutomaticUpdatesScheduledInstallationDay> {
        let mut result__ = ::core::mem::MaybeUninit::<AutomaticUpdatesScheduledInstallationDay>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.ScheduledInstallationDay)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<AutomaticUpdatesScheduledInstallationDay>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn SetScheduledInstallationDay(&self, value: AutomaticUpdatesScheduledInstallationDay) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.SetScheduledInstallationDay)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn ScheduledInstallationTime(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.ScheduledInstallationTime)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn SetScheduledInstallationTime(&self, value: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.SetScheduledInstallationTime)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn Refresh(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.Refresh)(::windows_core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn Save(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.Save)(::windows_core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn IncludeRecommendedUpdates(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).base__.IncludeRecommendedUpdates)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn SetIncludeRecommendedUpdates(&self, value: i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetIncludeRecommendedUpdates)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn CheckPermission(&self, usertype: AutomaticUpdatesUserType, permissiontype: AutomaticUpdatesPermissionType) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).base__.CheckPermission)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(usertype), ::core::mem::transmute(permissiontype), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn NonAdministratorsElevated(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).NonAdministratorsElevated)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn SetNonAdministratorsElevated(&self, value: i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetNonAdministratorsElevated)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn FeaturedUpdatesEnabled(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).FeaturedUpdatesEnabled)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn SetFeaturedUpdatesEnabled(&self, value: i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetFeaturedUpdatesEnabled)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(value)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IAutomaticUpdatesSettings3> for ::windows_core::IUnknown {
    fn from(value: IAutomaticUpdatesSettings3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IAutomaticUpdatesSettings3> for ::windows_core::IUnknown {
    fn from(value: &IAutomaticUpdatesSettings3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IAutomaticUpdatesSettings3 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IAutomaticUpdatesSettings3 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IAutomaticUpdatesSettings3> for super::Com::IDispatch {
    fn from(value: IAutomaticUpdatesSettings3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IAutomaticUpdatesSettings3> for super::Com::IDispatch {
    fn from(value: &IAutomaticUpdatesSettings3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for IAutomaticUpdatesSettings3 {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a IAutomaticUpdatesSettings3 {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IAutomaticUpdatesSettings3> for IAutomaticUpdatesSettings {
    fn from(value: IAutomaticUpdatesSettings3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IAutomaticUpdatesSettings3> for IAutomaticUpdatesSettings {
    fn from(value: &IAutomaticUpdatesSettings3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, IAutomaticUpdatesSettings> for IAutomaticUpdatesSettings3 {
    fn into_param(self) -> ::windows_core::Param<'a, IAutomaticUpdatesSettings> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, IAutomaticUpdatesSettings> for &'a IAutomaticUpdatesSettings3 {
    fn into_param(self) -> ::windows_core::Param<'a, IAutomaticUpdatesSettings> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IAutomaticUpdatesSettings3> for IAutomaticUpdatesSettings2 {
    fn from(value: IAutomaticUpdatesSettings3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IAutomaticUpdatesSettings3> for IAutomaticUpdatesSettings2 {
    fn from(value: &IAutomaticUpdatesSettings3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, IAutomaticUpdatesSettings2> for IAutomaticUpdatesSettings3 {
    fn into_param(self) -> ::windows_core::Param<'a, IAutomaticUpdatesSettings2> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, IAutomaticUpdatesSettings2> for &'a IAutomaticUpdatesSettings3 {
    fn into_param(self) -> ::windows_core::Param<'a, IAutomaticUpdatesSettings2> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IAutomaticUpdatesSettings3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IAutomaticUpdatesSettings3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IAutomaticUpdatesSettings3 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IAutomaticUpdatesSettings3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAutomaticUpdatesSettings3").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IAutomaticUpdatesSettings3 {
    type Vtable = IAutomaticUpdatesSettings3_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb587f5c3_f57e_485f_bbf5_0d181c5cd0dc);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IAutomaticUpdatesSettings3_Vtbl {
    pub base__: IAutomaticUpdatesSettings2_Vtbl,
    pub NonAdministratorsElevated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows_core::HRESULT,
    pub SetNonAdministratorsElevated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i16) -> ::windows_core::HRESULT,
    pub FeaturedUpdatesEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows_core::HRESULT,
    pub SetFeaturedUpdatesEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i16) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ICategory(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ICategory {
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Name(&self) -> ::windows_core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::super::Foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).Name)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CategoryID(&self) -> ::windows_core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::super::Foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).CategoryID)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Children(&self) -> ::windows_core::Result<ICategoryCollection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Children)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ICategoryCollection>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Description(&self) -> ::windows_core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::super::Foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).Description)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Image(&self) -> ::windows_core::Result<IImageInformation> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Image)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IImageInformation>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn Order(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).Order)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Parent(&self) -> ::windows_core::Result<ICategory> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Parent)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ICategory>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Type(&self) -> ::windows_core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::super::Foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).Type)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Updates(&self) -> ::windows_core::Result<IUpdateCollection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Updates)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IUpdateCollection>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<ICategory> for ::windows_core::IUnknown {
    fn from(value: ICategory) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&ICategory> for ::windows_core::IUnknown {
    fn from(value: &ICategory) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ICategory {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ICategory {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<ICategory> for super::Com::IDispatch {
    fn from(value: ICategory) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&ICategory> for super::Com::IDispatch {
    fn from(value: &ICategory) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for ICategory {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a ICategory {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ICategory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ICategory {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ICategory {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ICategory {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICategory").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for ICategory {
    type Vtable = ICategory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x81ddc1b8_9d35_47a6_b471_5b80f519223b);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ICategory_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Name: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub CategoryID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CategoryID: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Children: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Children: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Description: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Description: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Image: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Image: usize,
    pub Order: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Parent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Parent: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Type: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Type: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Updates: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Updates: usize,
}
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ICategoryCollection(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ICategoryCollection {
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn get_Item(&self, index: i32) -> ::windows_core::Result<ICategory> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).get_Item)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(index), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ICategory>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn _NewEnum(&self) -> ::windows_core::Result<::windows_core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
        (::windows_core::Interface::vtable(self)._NewEnum)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::IUnknown>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn Count(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).Count)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<ICategoryCollection> for ::windows_core::IUnknown {
    fn from(value: ICategoryCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&ICategoryCollection> for ::windows_core::IUnknown {
    fn from(value: &ICategoryCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ICategoryCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ICategoryCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<ICategoryCollection> for super::Com::IDispatch {
    fn from(value: ICategoryCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&ICategoryCollection> for super::Com::IDispatch {
    fn from(value: &ICategoryCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for ICategoryCollection {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a ICategoryCollection {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ICategoryCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ICategoryCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ICategoryCollection {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ICategoryCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICategoryCollection").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for ICategoryCollection {
    type Vtable = ICategoryCollection_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3a56bfb8_576c_43f7_9335_fe4838fd7e37);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ICategoryCollection_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub get_Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: i32, retval: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    get_Item: usize,
    pub _NewEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
#[repr(transparent)]
pub struct IDownloadCompletedCallback(::windows_core::IUnknown);
impl IDownloadCompletedCallback {
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Invoke<'a, Param0: ::windows_core::IntoParam<'a, IDownloadJob>, Param1: ::windows_core::IntoParam<'a, IDownloadCompletedCallbackArgs>>(&self, downloadjob: Param0, callbackargs: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Invoke)(::windows_core::Interface::as_raw(self), downloadjob.into_param().abi(), callbackargs.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IDownloadCompletedCallback> for ::windows_core::IUnknown {
    fn from(value: IDownloadCompletedCallback) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDownloadCompletedCallback> for ::windows_core::IUnknown {
    fn from(value: &IDownloadCompletedCallback) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IDownloadCompletedCallback {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IDownloadCompletedCallback {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDownloadCompletedCallback {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDownloadCompletedCallback {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDownloadCompletedCallback {}
impl ::core::fmt::Debug for IDownloadCompletedCallback {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDownloadCompletedCallback").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IDownloadCompletedCallback {
    type Vtable = IDownloadCompletedCallback_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x77254866_9f5b_4c8e_b9e2_c77a8530d64b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDownloadCompletedCallback_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Invoke: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, downloadjob: ::windows_core::RawPtr, callbackargs: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Invoke: usize,
}
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IDownloadCompletedCallbackArgs(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IDownloadCompletedCallbackArgs {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IDownloadCompletedCallbackArgs> for ::windows_core::IUnknown {
    fn from(value: IDownloadCompletedCallbackArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IDownloadCompletedCallbackArgs> for ::windows_core::IUnknown {
    fn from(value: &IDownloadCompletedCallbackArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IDownloadCompletedCallbackArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IDownloadCompletedCallbackArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IDownloadCompletedCallbackArgs> for super::Com::IDispatch {
    fn from(value: IDownloadCompletedCallbackArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IDownloadCompletedCallbackArgs> for super::Com::IDispatch {
    fn from(value: &IDownloadCompletedCallbackArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for IDownloadCompletedCallbackArgs {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a IDownloadCompletedCallbackArgs {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IDownloadCompletedCallbackArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IDownloadCompletedCallbackArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IDownloadCompletedCallbackArgs {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IDownloadCompletedCallbackArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDownloadCompletedCallbackArgs").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IDownloadCompletedCallbackArgs {
    type Vtable = IDownloadCompletedCallbackArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xfa565b23_498c_47a0_979d_e7d5b1813360);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IDownloadCompletedCallbackArgs_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
}
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IDownloadJob(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IDownloadJob {
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AsyncState(&self) -> ::windows_core::Result<super::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).AsyncState)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::VARIANT>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn IsCompleted(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).IsCompleted)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Updates(&self) -> ::windows_core::Result<IUpdateCollection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Updates)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IUpdateCollection>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn CleanUp(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).CleanUp)(::windows_core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetProgress(&self) -> ::windows_core::Result<IDownloadProgress> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetProgress)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IDownloadProgress>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn RequestAbort(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).RequestAbort)(::windows_core::Interface::as_raw(self)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IDownloadJob> for ::windows_core::IUnknown {
    fn from(value: IDownloadJob) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IDownloadJob> for ::windows_core::IUnknown {
    fn from(value: &IDownloadJob) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IDownloadJob {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IDownloadJob {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IDownloadJob> for super::Com::IDispatch {
    fn from(value: IDownloadJob) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IDownloadJob> for super::Com::IDispatch {
    fn from(value: &IDownloadJob) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for IDownloadJob {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a IDownloadJob {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IDownloadJob {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IDownloadJob {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IDownloadJob {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IDownloadJob {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDownloadJob").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IDownloadJob {
    type Vtable = IDownloadJob_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc574de85_7358_43f6_aae8_8697e62d8ba7);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IDownloadJob_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub AsyncState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut super::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    AsyncState: usize,
    pub IsCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Updates: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Updates: usize,
    pub CleanUp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetProgress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetProgress: usize,
    pub RequestAbort: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IDownloadProgress(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IDownloadProgress {
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CurrentUpdateBytesDownloaded(&self) -> ::windows_core::Result<super::super::Foundation::DECIMAL> {
        let mut result__ = ::core::mem::MaybeUninit::<super::super::Foundation::DECIMAL>::zeroed();
        (::windows_core::Interface::vtable(self).CurrentUpdateBytesDownloaded)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::DECIMAL>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CurrentUpdateBytesToDownload(&self) -> ::windows_core::Result<super::super::Foundation::DECIMAL> {
        let mut result__ = ::core::mem::MaybeUninit::<super::super::Foundation::DECIMAL>::zeroed();
        (::windows_core::Interface::vtable(self).CurrentUpdateBytesToDownload)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::DECIMAL>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn CurrentUpdateIndex(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).CurrentUpdateIndex)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn PercentComplete(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).PercentComplete)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn TotalBytesDownloaded(&self) -> ::windows_core::Result<super::super::Foundation::DECIMAL> {
        let mut result__ = ::core::mem::MaybeUninit::<super::super::Foundation::DECIMAL>::zeroed();
        (::windows_core::Interface::vtable(self).TotalBytesDownloaded)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::DECIMAL>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn TotalBytesToDownload(&self) -> ::windows_core::Result<super::super::Foundation::DECIMAL> {
        let mut result__ = ::core::mem::MaybeUninit::<super::super::Foundation::DECIMAL>::zeroed();
        (::windows_core::Interface::vtable(self).TotalBytesToDownload)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::DECIMAL>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetUpdateResult(&self, updateindex: i32) -> ::windows_core::Result<IUpdateDownloadResult> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetUpdateResult)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(updateindex), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IUpdateDownloadResult>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn CurrentUpdateDownloadPhase(&self) -> ::windows_core::Result<DownloadPhase> {
        let mut result__ = ::core::mem::MaybeUninit::<DownloadPhase>::zeroed();
        (::windows_core::Interface::vtable(self).CurrentUpdateDownloadPhase)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<DownloadPhase>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn CurrentUpdatePercentComplete(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).CurrentUpdatePercentComplete)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IDownloadProgress> for ::windows_core::IUnknown {
    fn from(value: IDownloadProgress) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IDownloadProgress> for ::windows_core::IUnknown {
    fn from(value: &IDownloadProgress) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IDownloadProgress {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IDownloadProgress {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IDownloadProgress> for super::Com::IDispatch {
    fn from(value: IDownloadProgress) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IDownloadProgress> for super::Com::IDispatch {
    fn from(value: &IDownloadProgress) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for IDownloadProgress {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a IDownloadProgress {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IDownloadProgress {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IDownloadProgress {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IDownloadProgress {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IDownloadProgress {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDownloadProgress").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IDownloadProgress {
    type Vtable = IDownloadProgress_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd31a5bac_f719_4178_9dbb_5e2cb47fd18a);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IDownloadProgress_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub CurrentUpdateBytesDownloaded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::DECIMAL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CurrentUpdateBytesDownloaded: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub CurrentUpdateBytesToDownload: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::DECIMAL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CurrentUpdateBytesToDownload: usize,
    pub CurrentUpdateIndex: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT,
    pub PercentComplete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub TotalBytesDownloaded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::DECIMAL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    TotalBytesDownloaded: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub TotalBytesToDownload: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::DECIMAL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    TotalBytesToDownload: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetUpdateResult: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, updateindex: i32, retval: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetUpdateResult: usize,
    pub CurrentUpdateDownloadPhase: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut DownloadPhase) -> ::windows_core::HRESULT,
    pub CurrentUpdatePercentComplete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
#[repr(transparent)]
pub struct IDownloadProgressChangedCallback(::windows_core::IUnknown);
impl IDownloadProgressChangedCallback {
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Invoke<'a, Param0: ::windows_core::IntoParam<'a, IDownloadJob>, Param1: ::windows_core::IntoParam<'a, IDownloadProgressChangedCallbackArgs>>(&self, downloadjob: Param0, callbackargs: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Invoke)(::windows_core::Interface::as_raw(self), downloadjob.into_param().abi(), callbackargs.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IDownloadProgressChangedCallback> for ::windows_core::IUnknown {
    fn from(value: IDownloadProgressChangedCallback) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDownloadProgressChangedCallback> for ::windows_core::IUnknown {
    fn from(value: &IDownloadProgressChangedCallback) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IDownloadProgressChangedCallback {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IDownloadProgressChangedCallback {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDownloadProgressChangedCallback {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDownloadProgressChangedCallback {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDownloadProgressChangedCallback {}
impl ::core::fmt::Debug for IDownloadProgressChangedCallback {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDownloadProgressChangedCallback").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IDownloadProgressChangedCallback {
    type Vtable = IDownloadProgressChangedCallback_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8c3f1cdd_6173_4591_aebd_a56a53ca77c1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDownloadProgressChangedCallback_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Invoke: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, downloadjob: ::windows_core::RawPtr, callbackargs: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Invoke: usize,
}
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IDownloadProgressChangedCallbackArgs(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IDownloadProgressChangedCallbackArgs {
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Progress(&self) -> ::windows_core::Result<IDownloadProgress> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Progress)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IDownloadProgress>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IDownloadProgressChangedCallbackArgs> for ::windows_core::IUnknown {
    fn from(value: IDownloadProgressChangedCallbackArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IDownloadProgressChangedCallbackArgs> for ::windows_core::IUnknown {
    fn from(value: &IDownloadProgressChangedCallbackArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IDownloadProgressChangedCallbackArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IDownloadProgressChangedCallbackArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IDownloadProgressChangedCallbackArgs> for super::Com::IDispatch {
    fn from(value: IDownloadProgressChangedCallbackArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IDownloadProgressChangedCallbackArgs> for super::Com::IDispatch {
    fn from(value: &IDownloadProgressChangedCallbackArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for IDownloadProgressChangedCallbackArgs {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a IDownloadProgressChangedCallbackArgs {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IDownloadProgressChangedCallbackArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IDownloadProgressChangedCallbackArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IDownloadProgressChangedCallbackArgs {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IDownloadProgressChangedCallbackArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDownloadProgressChangedCallbackArgs").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IDownloadProgressChangedCallbackArgs {
    type Vtable = IDownloadProgressChangedCallbackArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x324ff2c6_4981_4b04_9412_57481745ab24);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IDownloadProgressChangedCallbackArgs_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Progress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Progress: usize,
}
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IDownloadResult(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IDownloadResult {
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn HResult(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).HResult)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn ResultCode(&self) -> ::windows_core::Result<OperationResultCode> {
        let mut result__ = ::core::mem::MaybeUninit::<OperationResultCode>::zeroed();
        (::windows_core::Interface::vtable(self).ResultCode)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<OperationResultCode>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetUpdateResult(&self, updateindex: i32) -> ::windows_core::Result<IUpdateDownloadResult> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetUpdateResult)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(updateindex), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IUpdateDownloadResult>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IDownloadResult> for ::windows_core::IUnknown {
    fn from(value: IDownloadResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IDownloadResult> for ::windows_core::IUnknown {
    fn from(value: &IDownloadResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IDownloadResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IDownloadResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IDownloadResult> for super::Com::IDispatch {
    fn from(value: IDownloadResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IDownloadResult> for super::Com::IDispatch {
    fn from(value: &IDownloadResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for IDownloadResult {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a IDownloadResult {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IDownloadResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IDownloadResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IDownloadResult {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IDownloadResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDownloadResult").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IDownloadResult {
    type Vtable = IDownloadResult_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xdaa4fdd0_4727_4dbe_a1e7_745dca317144);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IDownloadResult_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub HResult: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT,
    pub ResultCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut OperationResultCode) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetUpdateResult: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, updateindex: i32, retval: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetUpdateResult: usize,
}
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IImageInformation(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IImageInformation {
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AltText(&self) -> ::windows_core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::super::Foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).AltText)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn Height(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).Height)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Source(&self) -> ::windows_core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::super::Foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).Source)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn Width(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).Width)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IImageInformation> for ::windows_core::IUnknown {
    fn from(value: IImageInformation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IImageInformation> for ::windows_core::IUnknown {
    fn from(value: &IImageInformation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IImageInformation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IImageInformation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IImageInformation> for super::Com::IDispatch {
    fn from(value: IImageInformation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IImageInformation> for super::Com::IDispatch {
    fn from(value: &IImageInformation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for IImageInformation {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a IImageInformation {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IImageInformation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IImageInformation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IImageInformation {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IImageInformation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IImageInformation").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IImageInformation {
    type Vtable = IImageInformation_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7c907864_346c_4aeb_8f3f_57da289f969f);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IImageInformation_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub AltText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    AltText: usize,
    pub Height: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Source: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Source: usize,
    pub Width: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IInstallationAgent(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IInstallationAgent {
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn RecordInstallationResult<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::BSTR>, Param2: ::windows_core::IntoParam<'a, IStringCollection>>(&self, installationresultcookie: Param0, hresult: i32, extendedreportingdata: Param2) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).RecordInstallationResult)(::windows_core::Interface::as_raw(self), installationresultcookie.into_param().abi(), ::core::mem::transmute(hresult), extendedreportingdata.into_param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IInstallationAgent> for ::windows_core::IUnknown {
    fn from(value: IInstallationAgent) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IInstallationAgent> for ::windows_core::IUnknown {
    fn from(value: &IInstallationAgent) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IInstallationAgent {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IInstallationAgent {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IInstallationAgent> for super::Com::IDispatch {
    fn from(value: IInstallationAgent) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IInstallationAgent> for super::Com::IDispatch {
    fn from(value: &IInstallationAgent) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for IInstallationAgent {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a IInstallationAgent {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IInstallationAgent {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IInstallationAgent {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IInstallationAgent {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IInstallationAgent {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IInstallationAgent").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IInstallationAgent {
    type Vtable = IInstallationAgent_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x925cbc18_a2ea_4648_bf1c_ec8badcfe20a);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IInstallationAgent_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub RecordInstallationResult: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, installationresultcookie: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, hresult: i32, extendedreportingdata: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    RecordInstallationResult: usize,
}
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IInstallationBehavior(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IInstallationBehavior {
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn CanRequestUserInput(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).CanRequestUserInput)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn Impact(&self) -> ::windows_core::Result<InstallationImpact> {
        let mut result__ = ::core::mem::MaybeUninit::<InstallationImpact>::zeroed();
        (::windows_core::Interface::vtable(self).Impact)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<InstallationImpact>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn RebootBehavior(&self) -> ::windows_core::Result<InstallationRebootBehavior> {
        let mut result__ = ::core::mem::MaybeUninit::<InstallationRebootBehavior>::zeroed();
        (::windows_core::Interface::vtable(self).RebootBehavior)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<InstallationRebootBehavior>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn RequiresNetworkConnectivity(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).RequiresNetworkConnectivity)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IInstallationBehavior> for ::windows_core::IUnknown {
    fn from(value: IInstallationBehavior) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IInstallationBehavior> for ::windows_core::IUnknown {
    fn from(value: &IInstallationBehavior) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IInstallationBehavior {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IInstallationBehavior {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IInstallationBehavior> for super::Com::IDispatch {
    fn from(value: IInstallationBehavior) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IInstallationBehavior> for super::Com::IDispatch {
    fn from(value: &IInstallationBehavior) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for IInstallationBehavior {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a IInstallationBehavior {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IInstallationBehavior {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IInstallationBehavior {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IInstallationBehavior {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IInstallationBehavior {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IInstallationBehavior").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IInstallationBehavior {
    type Vtable = IInstallationBehavior_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd9a59339_e245_4dbd_9686_4d5763e39624);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IInstallationBehavior_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub CanRequestUserInput: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows_core::HRESULT,
    pub Impact: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut InstallationImpact) -> ::windows_core::HRESULT,
    pub RebootBehavior: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut InstallationRebootBehavior) -> ::windows_core::HRESULT,
    pub RequiresNetworkConnectivity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
#[repr(transparent)]
pub struct IInstallationCompletedCallback(::windows_core::IUnknown);
impl IInstallationCompletedCallback {
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Invoke<'a, Param0: ::windows_core::IntoParam<'a, IInstallationJob>, Param1: ::windows_core::IntoParam<'a, IInstallationCompletedCallbackArgs>>(&self, installationjob: Param0, callbackargs: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Invoke)(::windows_core::Interface::as_raw(self), installationjob.into_param().abi(), callbackargs.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IInstallationCompletedCallback> for ::windows_core::IUnknown {
    fn from(value: IInstallationCompletedCallback) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IInstallationCompletedCallback> for ::windows_core::IUnknown {
    fn from(value: &IInstallationCompletedCallback) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IInstallationCompletedCallback {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IInstallationCompletedCallback {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IInstallationCompletedCallback {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IInstallationCompletedCallback {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IInstallationCompletedCallback {}
impl ::core::fmt::Debug for IInstallationCompletedCallback {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IInstallationCompletedCallback").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IInstallationCompletedCallback {
    type Vtable = IInstallationCompletedCallback_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x45f4f6f3_d602_4f98_9a8a_3efa152ad2d3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInstallationCompletedCallback_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Invoke: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, installationjob: ::windows_core::RawPtr, callbackargs: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Invoke: usize,
}
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IInstallationCompletedCallbackArgs(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IInstallationCompletedCallbackArgs {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IInstallationCompletedCallbackArgs> for ::windows_core::IUnknown {
    fn from(value: IInstallationCompletedCallbackArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IInstallationCompletedCallbackArgs> for ::windows_core::IUnknown {
    fn from(value: &IInstallationCompletedCallbackArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IInstallationCompletedCallbackArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IInstallationCompletedCallbackArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IInstallationCompletedCallbackArgs> for super::Com::IDispatch {
    fn from(value: IInstallationCompletedCallbackArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IInstallationCompletedCallbackArgs> for super::Com::IDispatch {
    fn from(value: &IInstallationCompletedCallbackArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for IInstallationCompletedCallbackArgs {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a IInstallationCompletedCallbackArgs {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IInstallationCompletedCallbackArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IInstallationCompletedCallbackArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IInstallationCompletedCallbackArgs {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IInstallationCompletedCallbackArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IInstallationCompletedCallbackArgs").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IInstallationCompletedCallbackArgs {
    type Vtable = IInstallationCompletedCallbackArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x250e2106_8efb_4705_9653_ef13c581b6a1);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IInstallationCompletedCallbackArgs_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
}
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IInstallationJob(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IInstallationJob {
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AsyncState(&self) -> ::windows_core::Result<super::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).AsyncState)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::VARIANT>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn IsCompleted(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).IsCompleted)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Updates(&self) -> ::windows_core::Result<IUpdateCollection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Updates)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IUpdateCollection>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn CleanUp(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).CleanUp)(::windows_core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetProgress(&self) -> ::windows_core::Result<IInstallationProgress> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetProgress)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IInstallationProgress>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn RequestAbort(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).RequestAbort)(::windows_core::Interface::as_raw(self)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IInstallationJob> for ::windows_core::IUnknown {
    fn from(value: IInstallationJob) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IInstallationJob> for ::windows_core::IUnknown {
    fn from(value: &IInstallationJob) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IInstallationJob {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IInstallationJob {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IInstallationJob> for super::Com::IDispatch {
    fn from(value: IInstallationJob) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IInstallationJob> for super::Com::IDispatch {
    fn from(value: &IInstallationJob) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for IInstallationJob {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a IInstallationJob {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IInstallationJob {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IInstallationJob {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IInstallationJob {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IInstallationJob {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IInstallationJob").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IInstallationJob {
    type Vtable = IInstallationJob_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5c209f0b_bad5_432a_9556_4699bed2638a);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IInstallationJob_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub AsyncState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut super::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    AsyncState: usize,
    pub IsCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Updates: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Updates: usize,
    pub CleanUp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetProgress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetProgress: usize,
    pub RequestAbort: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IInstallationProgress(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IInstallationProgress {
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn CurrentUpdateIndex(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).CurrentUpdateIndex)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn CurrentUpdatePercentComplete(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).CurrentUpdatePercentComplete)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn PercentComplete(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).PercentComplete)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetUpdateResult(&self, updateindex: i32) -> ::windows_core::Result<IUpdateInstallationResult> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetUpdateResult)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(updateindex), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IUpdateInstallationResult>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IInstallationProgress> for ::windows_core::IUnknown {
    fn from(value: IInstallationProgress) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IInstallationProgress> for ::windows_core::IUnknown {
    fn from(value: &IInstallationProgress) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IInstallationProgress {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IInstallationProgress {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IInstallationProgress> for super::Com::IDispatch {
    fn from(value: IInstallationProgress) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IInstallationProgress> for super::Com::IDispatch {
    fn from(value: &IInstallationProgress) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for IInstallationProgress {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a IInstallationProgress {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IInstallationProgress {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IInstallationProgress {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IInstallationProgress {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IInstallationProgress {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IInstallationProgress").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IInstallationProgress {
    type Vtable = IInstallationProgress_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x345c8244_43a3_4e32_a368_65f073b76f36);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IInstallationProgress_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub CurrentUpdateIndex: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT,
    pub CurrentUpdatePercentComplete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT,
    pub PercentComplete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetUpdateResult: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, updateindex: i32, retval: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetUpdateResult: usize,
}
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
#[repr(transparent)]
pub struct IInstallationProgressChangedCallback(::windows_core::IUnknown);
impl IInstallationProgressChangedCallback {
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Invoke<'a, Param0: ::windows_core::IntoParam<'a, IInstallationJob>, Param1: ::windows_core::IntoParam<'a, IInstallationProgressChangedCallbackArgs>>(&self, installationjob: Param0, callbackargs: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Invoke)(::windows_core::Interface::as_raw(self), installationjob.into_param().abi(), callbackargs.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IInstallationProgressChangedCallback> for ::windows_core::IUnknown {
    fn from(value: IInstallationProgressChangedCallback) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IInstallationProgressChangedCallback> for ::windows_core::IUnknown {
    fn from(value: &IInstallationProgressChangedCallback) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IInstallationProgressChangedCallback {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IInstallationProgressChangedCallback {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IInstallationProgressChangedCallback {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IInstallationProgressChangedCallback {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IInstallationProgressChangedCallback {}
impl ::core::fmt::Debug for IInstallationProgressChangedCallback {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IInstallationProgressChangedCallback").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IInstallationProgressChangedCallback {
    type Vtable = IInstallationProgressChangedCallback_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe01402d5_f8da_43ba_a012_38894bd048f1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInstallationProgressChangedCallback_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Invoke: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, installationjob: ::windows_core::RawPtr, callbackargs: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Invoke: usize,
}
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IInstallationProgressChangedCallbackArgs(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IInstallationProgressChangedCallbackArgs {
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Progress(&self) -> ::windows_core::Result<IInstallationProgress> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Progress)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IInstallationProgress>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IInstallationProgressChangedCallbackArgs> for ::windows_core::IUnknown {
    fn from(value: IInstallationProgressChangedCallbackArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IInstallationProgressChangedCallbackArgs> for ::windows_core::IUnknown {
    fn from(value: &IInstallationProgressChangedCallbackArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IInstallationProgressChangedCallbackArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IInstallationProgressChangedCallbackArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IInstallationProgressChangedCallbackArgs> for super::Com::IDispatch {
    fn from(value: IInstallationProgressChangedCallbackArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IInstallationProgressChangedCallbackArgs> for super::Com::IDispatch {
    fn from(value: &IInstallationProgressChangedCallbackArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for IInstallationProgressChangedCallbackArgs {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a IInstallationProgressChangedCallbackArgs {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IInstallationProgressChangedCallbackArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IInstallationProgressChangedCallbackArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IInstallationProgressChangedCallbackArgs {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IInstallationProgressChangedCallbackArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IInstallationProgressChangedCallbackArgs").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IInstallationProgressChangedCallbackArgs {
    type Vtable = IInstallationProgressChangedCallbackArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe4f14e1e_689d_4218_a0b9_bc189c484a01);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IInstallationProgressChangedCallbackArgs_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Progress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Progress: usize,
}
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IInstallationResult(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IInstallationResult {
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn HResult(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).HResult)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn RebootRequired(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).RebootRequired)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn ResultCode(&self) -> ::windows_core::Result<OperationResultCode> {
        let mut result__ = ::core::mem::MaybeUninit::<OperationResultCode>::zeroed();
        (::windows_core::Interface::vtable(self).ResultCode)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<OperationResultCode>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetUpdateResult(&self, updateindex: i32) -> ::windows_core::Result<IUpdateInstallationResult> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetUpdateResult)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(updateindex), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IUpdateInstallationResult>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IInstallationResult> for ::windows_core::IUnknown {
    fn from(value: IInstallationResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IInstallationResult> for ::windows_core::IUnknown {
    fn from(value: &IInstallationResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IInstallationResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IInstallationResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IInstallationResult> for super::Com::IDispatch {
    fn from(value: IInstallationResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IInstallationResult> for super::Com::IDispatch {
    fn from(value: &IInstallationResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for IInstallationResult {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a IInstallationResult {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IInstallationResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IInstallationResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IInstallationResult {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IInstallationResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IInstallationResult").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IInstallationResult {
    type Vtable = IInstallationResult_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa43c56d6_7451_48d4_af96_b6cd2d0d9b7a);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IInstallationResult_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub HResult: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT,
    pub RebootRequired: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows_core::HRESULT,
    pub ResultCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut OperationResultCode) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetUpdateResult: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, updateindex: i32, retval: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetUpdateResult: usize,
}
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IInvalidProductLicenseException(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IInvalidProductLicenseException {
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Message(&self) -> ::windows_core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::super::Foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.Message)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn HResult(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).base__.HResult)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn Context(&self) -> ::windows_core::Result<UpdateExceptionContext> {
        let mut result__ = ::core::mem::MaybeUninit::<UpdateExceptionContext>::zeroed();
        (::windows_core::Interface::vtable(self).base__.Context)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<UpdateExceptionContext>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Product(&self) -> ::windows_core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::super::Foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).Product)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IInvalidProductLicenseException> for ::windows_core::IUnknown {
    fn from(value: IInvalidProductLicenseException) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IInvalidProductLicenseException> for ::windows_core::IUnknown {
    fn from(value: &IInvalidProductLicenseException) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IInvalidProductLicenseException {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IInvalidProductLicenseException {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IInvalidProductLicenseException> for super::Com::IDispatch {
    fn from(value: IInvalidProductLicenseException) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IInvalidProductLicenseException> for super::Com::IDispatch {
    fn from(value: &IInvalidProductLicenseException) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for IInvalidProductLicenseException {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a IInvalidProductLicenseException {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IInvalidProductLicenseException> for IUpdateException {
    fn from(value: IInvalidProductLicenseException) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IInvalidProductLicenseException> for IUpdateException {
    fn from(value: &IInvalidProductLicenseException) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, IUpdateException> for IInvalidProductLicenseException {
    fn into_param(self) -> ::windows_core::Param<'a, IUpdateException> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, IUpdateException> for &'a IInvalidProductLicenseException {
    fn into_param(self) -> ::windows_core::Param<'a, IUpdateException> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IInvalidProductLicenseException {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IInvalidProductLicenseException {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IInvalidProductLicenseException {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IInvalidProductLicenseException {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IInvalidProductLicenseException").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IInvalidProductLicenseException {
    type Vtable = IInvalidProductLicenseException_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa37d00f5_7bb0_4953_b414_f9e98326f2e8);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IInvalidProductLicenseException_Vtbl {
    pub base__: IUpdateException_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub Product: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Product: usize,
}
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
#[repr(transparent)]
pub struct ISearchCompletedCallback(::windows_core::IUnknown);
impl ISearchCompletedCallback {
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Invoke<'a, Param0: ::windows_core::IntoParam<'a, ISearchJob>, Param1: ::windows_core::IntoParam<'a, ISearchCompletedCallbackArgs>>(&self, searchjob: Param0, callbackargs: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Invoke)(::windows_core::Interface::as_raw(self), searchjob.into_param().abi(), callbackargs.into_param().abi()).ok()
    }
}
impl ::core::convert::From<ISearchCompletedCallback> for ::windows_core::IUnknown {
    fn from(value: ISearchCompletedCallback) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISearchCompletedCallback> for ::windows_core::IUnknown {
    fn from(value: &ISearchCompletedCallback) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ISearchCompletedCallback {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ISearchCompletedCallback {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ISearchCompletedCallback {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISearchCompletedCallback {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISearchCompletedCallback {}
impl ::core::fmt::Debug for ISearchCompletedCallback {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISearchCompletedCallback").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for ISearchCompletedCallback {
    type Vtable = ISearchCompletedCallback_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x88aee058_d4b0_4725_a2f1_814a67ae964c);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISearchCompletedCallback_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Invoke: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, searchjob: ::windows_core::RawPtr, callbackargs: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Invoke: usize,
}
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ISearchCompletedCallbackArgs(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ISearchCompletedCallbackArgs {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<ISearchCompletedCallbackArgs> for ::windows_core::IUnknown {
    fn from(value: ISearchCompletedCallbackArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&ISearchCompletedCallbackArgs> for ::windows_core::IUnknown {
    fn from(value: &ISearchCompletedCallbackArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ISearchCompletedCallbackArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ISearchCompletedCallbackArgs {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<ISearchCompletedCallbackArgs> for super::Com::IDispatch {
    fn from(value: ISearchCompletedCallbackArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&ISearchCompletedCallbackArgs> for super::Com::IDispatch {
    fn from(value: &ISearchCompletedCallbackArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for ISearchCompletedCallbackArgs {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a ISearchCompletedCallbackArgs {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ISearchCompletedCallbackArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ISearchCompletedCallbackArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ISearchCompletedCallbackArgs {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ISearchCompletedCallbackArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISearchCompletedCallbackArgs").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for ISearchCompletedCallbackArgs {
    type Vtable = ISearchCompletedCallbackArgs_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa700a634_2850_4c47_938a_9e4b6e5af9a6);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ISearchCompletedCallbackArgs_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
}
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ISearchJob(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ISearchJob {
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AsyncState(&self) -> ::windows_core::Result<super::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).AsyncState)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::VARIANT>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn IsCompleted(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).IsCompleted)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn CleanUp(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).CleanUp)(::windows_core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn RequestAbort(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).RequestAbort)(::windows_core::Interface::as_raw(self)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<ISearchJob> for ::windows_core::IUnknown {
    fn from(value: ISearchJob) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&ISearchJob> for ::windows_core::IUnknown {
    fn from(value: &ISearchJob) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ISearchJob {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ISearchJob {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<ISearchJob> for super::Com::IDispatch {
    fn from(value: ISearchJob) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&ISearchJob> for super::Com::IDispatch {
    fn from(value: &ISearchJob) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for ISearchJob {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a ISearchJob {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ISearchJob {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ISearchJob {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ISearchJob {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ISearchJob {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISearchJob").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for ISearchJob {
    type Vtable = ISearchJob_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7366ea16_7a1a_4ea2_b042_973d3e9cd99b);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ISearchJob_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub AsyncState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut super::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    AsyncState: usize,
    pub IsCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows_core::HRESULT,
    pub CleanUp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub RequestAbort: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ISearchResult(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ISearchResult {
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn ResultCode(&self) -> ::windows_core::Result<OperationResultCode> {
        let mut result__ = ::core::mem::MaybeUninit::<OperationResultCode>::zeroed();
        (::windows_core::Interface::vtable(self).ResultCode)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<OperationResultCode>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn RootCategories(&self) -> ::windows_core::Result<ICategoryCollection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).RootCategories)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ICategoryCollection>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Updates(&self) -> ::windows_core::Result<IUpdateCollection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Updates)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IUpdateCollection>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Warnings(&self) -> ::windows_core::Result<IUpdateExceptionCollection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Warnings)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IUpdateExceptionCollection>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<ISearchResult> for ::windows_core::IUnknown {
    fn from(value: ISearchResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&ISearchResult> for ::windows_core::IUnknown {
    fn from(value: &ISearchResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ISearchResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ISearchResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<ISearchResult> for super::Com::IDispatch {
    fn from(value: ISearchResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&ISearchResult> for super::Com::IDispatch {
    fn from(value: &ISearchResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for ISearchResult {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a ISearchResult {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ISearchResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ISearchResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ISearchResult {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ISearchResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISearchResult").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for ISearchResult {
    type Vtable = ISearchResult_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd40cff62_e08c_4498_941a_01e25f0fd33c);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ISearchResult_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub ResultCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut OperationResultCode) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub RootCategories: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    RootCategories: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Updates: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Updates: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Warnings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Warnings: usize,
}
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IStringCollection(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IStringCollection {
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn get_Item(&self, index: i32) -> ::windows_core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::super::Foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).get_Item)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(index), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn put_Item<'a, Param1: ::windows_core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, index: i32, value: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).put_Item)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(index), value.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn _NewEnum(&self) -> ::windows_core::Result<::windows_core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
        (::windows_core::Interface::vtable(self)._NewEnum)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::IUnknown>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn Count(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).Count)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn ReadOnly(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).ReadOnly)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Add<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, value: Param0) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).Add)(::windows_core::Interface::as_raw(self), value.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn Clear(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Clear)(::windows_core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Copy(&self) -> ::windows_core::Result<IStringCollection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Copy)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IStringCollection>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Insert<'a, Param1: ::windows_core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, index: i32, value: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Insert)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(index), value.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn RemoveAt(&self, index: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).RemoveAt)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(index)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IStringCollection> for ::windows_core::IUnknown {
    fn from(value: IStringCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IStringCollection> for ::windows_core::IUnknown {
    fn from(value: &IStringCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IStringCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IStringCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IStringCollection> for super::Com::IDispatch {
    fn from(value: IStringCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IStringCollection> for super::Com::IDispatch {
    fn from(value: &IStringCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for IStringCollection {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a IStringCollection {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IStringCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IStringCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IStringCollection {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IStringCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IStringCollection").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IStringCollection {
    type Vtable = IStringCollection_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xeff90582_2ddc_480f_a06d_60f3fbc362c3);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IStringCollection_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub get_Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: i32, retval: *mut super::super::Foundation::BSTR) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    get_Item: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub put_Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: i32, value: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    put_Item: usize,
    pub _NewEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT,
    pub ReadOnly: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Add: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, retval: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Add: usize,
    pub Clear: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Copy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Copy: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Insert: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: i32, value: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Insert: usize,
    pub RemoveAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: i32) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ISystemInformation(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ISystemInformation {
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OemHardwareSupportLink(&self) -> ::windows_core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::super::Foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).OemHardwareSupportLink)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn RebootRequired(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).RebootRequired)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<ISystemInformation> for ::windows_core::IUnknown {
    fn from(value: ISystemInformation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&ISystemInformation> for ::windows_core::IUnknown {
    fn from(value: &ISystemInformation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ISystemInformation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ISystemInformation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<ISystemInformation> for super::Com::IDispatch {
    fn from(value: ISystemInformation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&ISystemInformation> for super::Com::IDispatch {
    fn from(value: &ISystemInformation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for ISystemInformation {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a ISystemInformation {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ISystemInformation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ISystemInformation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ISystemInformation {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ISystemInformation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISystemInformation").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for ISystemInformation {
    type Vtable = ISystemInformation_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xade87bf7_7b56_4275_8fab_b9b0e591844b);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ISystemInformation_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub OemHardwareSupportLink: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    OemHardwareSupportLink: usize,
    pub RebootRequired: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IUpdate(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IUpdate {
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Title(&self) -> ::windows_core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::super::Foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).Title)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn AutoSelectOnWebSites(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).AutoSelectOnWebSites)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn BundledUpdates(&self) -> ::windows_core::Result<IUpdateCollection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).BundledUpdates)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IUpdateCollection>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn CanRequireSource(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).CanRequireSource)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Categories(&self) -> ::windows_core::Result<ICategoryCollection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Categories)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ICategoryCollection>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Deadline(&self) -> ::windows_core::Result<super::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).Deadline)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::VARIANT>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn DeltaCompressedContentAvailable(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).DeltaCompressedContentAvailable)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn DeltaCompressedContentPreferred(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).DeltaCompressedContentPreferred)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Description(&self) -> ::windows_core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::super::Foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).Description)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn EulaAccepted(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).EulaAccepted)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EulaText(&self) -> ::windows_core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::super::Foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).EulaText)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn HandlerID(&self) -> ::windows_core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::super::Foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).HandlerID)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Identity(&self) -> ::windows_core::Result<IUpdateIdentity> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Identity)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IUpdateIdentity>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Image(&self) -> ::windows_core::Result<IImageInformation> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Image)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IImageInformation>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn InstallationBehavior(&self) -> ::windows_core::Result<IInstallationBehavior> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).InstallationBehavior)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IInstallationBehavior>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn IsBeta(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).IsBeta)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn IsDownloaded(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).IsDownloaded)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn IsHidden(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).IsHidden)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn SetIsHidden(&self, value: i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetIsHidden)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn IsInstalled(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).IsInstalled)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn IsMandatory(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).IsMandatory)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn IsUninstallable(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).IsUninstallable)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Languages(&self) -> ::windows_core::Result<IStringCollection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Languages)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IStringCollection>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn LastDeploymentChangeTime(&self) -> ::windows_core::Result<f64> {
        let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
        (::windows_core::Interface::vtable(self).LastDeploymentChangeTime)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<f64>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MaxDownloadSize(&self) -> ::windows_core::Result<super::super::Foundation::DECIMAL> {
        let mut result__ = ::core::mem::MaybeUninit::<super::super::Foundation::DECIMAL>::zeroed();
        (::windows_core::Interface::vtable(self).MaxDownloadSize)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::DECIMAL>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MinDownloadSize(&self) -> ::windows_core::Result<super::super::Foundation::DECIMAL> {
        let mut result__ = ::core::mem::MaybeUninit::<super::super::Foundation::DECIMAL>::zeroed();
        (::windows_core::Interface::vtable(self).MinDownloadSize)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::DECIMAL>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn MoreInfoUrls(&self) -> ::windows_core::Result<IStringCollection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).MoreInfoUrls)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IStringCollection>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MsrcSeverity(&self) -> ::windows_core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::super::Foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).MsrcSeverity)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn RecommendedCpuSpeed(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).RecommendedCpuSpeed)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn RecommendedHardDiskSpace(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).RecommendedHardDiskSpace)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn RecommendedMemory(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).RecommendedMemory)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ReleaseNotes(&self) -> ::windows_core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::super::Foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).ReleaseNotes)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SecurityBulletinIDs(&self) -> ::windows_core::Result<IStringCollection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).SecurityBulletinIDs)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IStringCollection>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SupersededUpdateIDs(&self) -> ::windows_core::Result<IStringCollection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).SupersededUpdateIDs)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IStringCollection>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SupportUrl(&self) -> ::windows_core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::super::Foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).SupportUrl)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn Type(&self) -> ::windows_core::Result<UpdateType> {
        let mut result__ = ::core::mem::MaybeUninit::<UpdateType>::zeroed();
        (::windows_core::Interface::vtable(self).Type)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<UpdateType>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn UninstallationNotes(&self) -> ::windows_core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::super::Foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).UninstallationNotes)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn UninstallationBehavior(&self) -> ::windows_core::Result<IInstallationBehavior> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).UninstallationBehavior)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IInstallationBehavior>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn UninstallationSteps(&self) -> ::windows_core::Result<IStringCollection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).UninstallationSteps)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IStringCollection>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn KBArticleIDs(&self) -> ::windows_core::Result<IStringCollection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).KBArticleIDs)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IStringCollection>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn AcceptEula(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).AcceptEula)(::windows_core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn DeploymentAction(&self) -> ::windows_core::Result<DeploymentAction> {
        let mut result__ = ::core::mem::MaybeUninit::<DeploymentAction>::zeroed();
        (::windows_core::Interface::vtable(self).DeploymentAction)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<DeploymentAction>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CopyFromCache<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, path: Param0, toextractcabfiles: i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).CopyFromCache)(::windows_core::Interface::as_raw(self), path.into_param().abi(), ::core::mem::transmute(toextractcabfiles)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn DownloadPriority(&self) -> ::windows_core::Result<DownloadPriority> {
        let mut result__ = ::core::mem::MaybeUninit::<DownloadPriority>::zeroed();
        (::windows_core::Interface::vtable(self).DownloadPriority)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<DownloadPriority>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn DownloadContents(&self) -> ::windows_core::Result<IUpdateDownloadContentCollection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).DownloadContents)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IUpdateDownloadContentCollection>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IUpdate> for ::windows_core::IUnknown {
    fn from(value: IUpdate) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IUpdate> for ::windows_core::IUnknown {
    fn from(value: &IUpdate) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IUpdate {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IUpdate {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IUpdate> for super::Com::IDispatch {
    fn from(value: IUpdate) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IUpdate> for super::Com::IDispatch {
    fn from(value: &IUpdate) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for IUpdate {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a IUpdate {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IUpdate {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IUpdate {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IUpdate {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IUpdate {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUpdate").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IUpdate {
    type Vtable = IUpdate_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6a92b07a_d821_4682_b423_5c805022cc4d);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IUpdate_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub Title: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Title: usize,
    pub AutoSelectOnWebSites: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub BundledUpdates: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    BundledUpdates: usize,
    pub CanRequireSource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Categories: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Categories: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Deadline: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut super::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Deadline: usize,
    pub DeltaCompressedContentAvailable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows_core::HRESULT,
    pub DeltaCompressedContentPreferred: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Description: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Description: usize,
    pub EulaAccepted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub EulaText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    EulaText: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub HandlerID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    HandlerID: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Identity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Identity: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Image: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Image: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub InstallationBehavior: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    InstallationBehavior: usize,
    pub IsBeta: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows_core::HRESULT,
    pub IsDownloaded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows_core::HRESULT,
    pub IsHidden: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows_core::HRESULT,
    pub SetIsHidden: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i16) -> ::windows_core::HRESULT,
    pub IsInstalled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows_core::HRESULT,
    pub IsMandatory: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows_core::HRESULT,
    pub IsUninstallable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Languages: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Languages: usize,
    pub LastDeploymentChangeTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub MaxDownloadSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::DECIMAL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    MaxDownloadSize: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub MinDownloadSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::DECIMAL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    MinDownloadSize: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub MoreInfoUrls: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    MoreInfoUrls: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub MsrcSeverity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    MsrcSeverity: usize,
    pub RecommendedCpuSpeed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT,
    pub RecommendedHardDiskSpace: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT,
    pub RecommendedMemory: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub ReleaseNotes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ReleaseNotes: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SecurityBulletinIDs: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SecurityBulletinIDs: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SupersededUpdateIDs: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SupersededUpdateIDs: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SupportUrl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SupportUrl: usize,
    pub Type: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut UpdateType) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub UninstallationNotes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    UninstallationNotes: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub UninstallationBehavior: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    UninstallationBehavior: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub UninstallationSteps: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    UninstallationSteps: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub KBArticleIDs: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    KBArticleIDs: usize,
    pub AcceptEula: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub DeploymentAction: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut DeploymentAction) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub CopyFromCache: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, toextractcabfiles: i16) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CopyFromCache: usize,
    pub DownloadPriority: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut DownloadPriority) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub DownloadContents: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    DownloadContents: usize,
}
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IUpdate2(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IUpdate2 {
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Title(&self) -> ::windows_core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::super::Foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.Title)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn AutoSelectOnWebSites(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).base__.AutoSelectOnWebSites)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn BundledUpdates(&self) -> ::windows_core::Result<IUpdateCollection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.BundledUpdates)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IUpdateCollection>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn CanRequireSource(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).base__.CanRequireSource)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Categories(&self) -> ::windows_core::Result<ICategoryCollection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.Categories)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ICategoryCollection>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Deadline(&self) -> ::windows_core::Result<super::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.Deadline)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::VARIANT>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn DeltaCompressedContentAvailable(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).base__.DeltaCompressedContentAvailable)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn DeltaCompressedContentPreferred(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).base__.DeltaCompressedContentPreferred)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Description(&self) -> ::windows_core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::super::Foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.Description)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn EulaAccepted(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).base__.EulaAccepted)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EulaText(&self) -> ::windows_core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::super::Foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.EulaText)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn HandlerID(&self) -> ::windows_core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::super::Foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.HandlerID)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Identity(&self) -> ::windows_core::Result<IUpdateIdentity> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.Identity)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IUpdateIdentity>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Image(&self) -> ::windows_core::Result<IImageInformation> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.Image)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IImageInformation>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn InstallationBehavior(&self) -> ::windows_core::Result<IInstallationBehavior> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.InstallationBehavior)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IInstallationBehavior>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn IsBeta(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).base__.IsBeta)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn IsDownloaded(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).base__.IsDownloaded)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn IsHidden(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).base__.IsHidden)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn SetIsHidden(&self, value: i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetIsHidden)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn IsInstalled(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).base__.IsInstalled)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn IsMandatory(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).base__.IsMandatory)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn IsUninstallable(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).base__.IsUninstallable)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Languages(&self) -> ::windows_core::Result<IStringCollection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.Languages)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IStringCollection>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn LastDeploymentChangeTime(&self) -> ::windows_core::Result<f64> {
        let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
        (::windows_core::Interface::vtable(self).base__.LastDeploymentChangeTime)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<f64>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MaxDownloadSize(&self) -> ::windows_core::Result<super::super::Foundation::DECIMAL> {
        let mut result__ = ::core::mem::MaybeUninit::<super::super::Foundation::DECIMAL>::zeroed();
        (::windows_core::Interface::vtable(self).base__.MaxDownloadSize)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::DECIMAL>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MinDownloadSize(&self) -> ::windows_core::Result<super::super::Foundation::DECIMAL> {
        let mut result__ = ::core::mem::MaybeUninit::<super::super::Foundation::DECIMAL>::zeroed();
        (::windows_core::Interface::vtable(self).base__.MinDownloadSize)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::DECIMAL>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn MoreInfoUrls(&self) -> ::windows_core::Result<IStringCollection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.MoreInfoUrls)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IStringCollection>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MsrcSeverity(&self) -> ::windows_core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::super::Foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.MsrcSeverity)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn RecommendedCpuSpeed(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).base__.RecommendedCpuSpeed)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn RecommendedHardDiskSpace(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).base__.RecommendedHardDiskSpace)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn RecommendedMemory(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).base__.RecommendedMemory)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ReleaseNotes(&self) -> ::windows_core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::super::Foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.ReleaseNotes)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SecurityBulletinIDs(&self) -> ::windows_core::Result<IStringCollection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.SecurityBulletinIDs)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IStringCollection>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SupersededUpdateIDs(&self) -> ::windows_core::Result<IStringCollection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.SupersededUpdateIDs)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IStringCollection>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SupportUrl(&self) -> ::windows_core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::super::Foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.SupportUrl)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn Type(&self) -> ::windows_core::Result<UpdateType> {
        let mut result__ = ::core::mem::MaybeUninit::<UpdateType>::zeroed();
        (::windows_core::Interface::vtable(self).base__.Type)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<UpdateType>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn UninstallationNotes(&self) -> ::windows_core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::super::Foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.UninstallationNotes)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn UninstallationBehavior(&self) -> ::windows_core::Result<IInstallationBehavior> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.UninstallationBehavior)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IInstallationBehavior>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn UninstallationSteps(&self) -> ::windows_core::Result<IStringCollection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.UninstallationSteps)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IStringCollection>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn KBArticleIDs(&self) -> ::windows_core::Result<IStringCollection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.KBArticleIDs)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IStringCollection>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn AcceptEula(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.AcceptEula)(::windows_core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn DeploymentAction(&self) -> ::windows_core::Result<DeploymentAction> {
        let mut result__ = ::core::mem::MaybeUninit::<DeploymentAction>::zeroed();
        (::windows_core::Interface::vtable(self).base__.DeploymentAction)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<DeploymentAction>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CopyFromCache<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, path: Param0, toextractcabfiles: i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.CopyFromCache)(::windows_core::Interface::as_raw(self), path.into_param().abi(), ::core::mem::transmute(toextractcabfiles)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn DownloadPriority(&self) -> ::windows_core::Result<DownloadPriority> {
        let mut result__ = ::core::mem::MaybeUninit::<DownloadPriority>::zeroed();
        (::windows_core::Interface::vtable(self).base__.DownloadPriority)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<DownloadPriority>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn DownloadContents(&self) -> ::windows_core::Result<IUpdateDownloadContentCollection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.DownloadContents)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IUpdateDownloadContentCollection>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn RebootRequired(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).RebootRequired)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn IsPresent(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).IsPresent)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CveIDs(&self) -> ::windows_core::Result<IStringCollection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).CveIDs)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IStringCollection>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CopyToCache<'a, Param0: ::windows_core::IntoParam<'a, IStringCollection>>(&self, pfiles: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).CopyToCache)(::windows_core::Interface::as_raw(self), pfiles.into_param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IUpdate2> for ::windows_core::IUnknown {
    fn from(value: IUpdate2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IUpdate2> for ::windows_core::IUnknown {
    fn from(value: &IUpdate2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IUpdate2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IUpdate2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IUpdate2> for super::Com::IDispatch {
    fn from(value: IUpdate2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IUpdate2> for super::Com::IDispatch {
    fn from(value: &IUpdate2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for IUpdate2 {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a IUpdate2 {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IUpdate2> for IUpdate {
    fn from(value: IUpdate2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IUpdate2> for IUpdate {
    fn from(value: &IUpdate2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, IUpdate> for IUpdate2 {
    fn into_param(self) -> ::windows_core::Param<'a, IUpdate> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, IUpdate> for &'a IUpdate2 {
    fn into_param(self) -> ::windows_core::Param<'a, IUpdate> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IUpdate2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IUpdate2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IUpdate2 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IUpdate2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUpdate2").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IUpdate2 {
    type Vtable = IUpdate2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x144fe9b0_d23d_4a8b_8634_fb4457533b7a);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IUpdate2_Vtbl {
    pub base__: IUpdate_Vtbl,
    pub RebootRequired: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows_core::HRESULT,
    pub IsPresent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub CveIDs: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CveIDs: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CopyToCache: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfiles: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CopyToCache: usize,
}
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IUpdate3(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IUpdate3 {
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Title(&self) -> ::windows_core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::super::Foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.Title)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn AutoSelectOnWebSites(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.AutoSelectOnWebSites)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn BundledUpdates(&self) -> ::windows_core::Result<IUpdateCollection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.BundledUpdates)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IUpdateCollection>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn CanRequireSource(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.CanRequireSource)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Categories(&self) -> ::windows_core::Result<ICategoryCollection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.Categories)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ICategoryCollection>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Deadline(&self) -> ::windows_core::Result<super::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.Deadline)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::VARIANT>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn DeltaCompressedContentAvailable(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.DeltaCompressedContentAvailable)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn DeltaCompressedContentPreferred(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.DeltaCompressedContentPreferred)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Description(&self) -> ::windows_core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::super::Foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.Description)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn EulaAccepted(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.EulaAccepted)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EulaText(&self) -> ::windows_core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::super::Foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.EulaText)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn HandlerID(&self) -> ::windows_core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::super::Foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.HandlerID)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Identity(&self) -> ::windows_core::Result<IUpdateIdentity> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.Identity)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IUpdateIdentity>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Image(&self) -> ::windows_core::Result<IImageInformation> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.Image)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IImageInformation>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn InstallationBehavior(&self) -> ::windows_core::Result<IInstallationBehavior> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.InstallationBehavior)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IInstallationBehavior>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn IsBeta(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.IsBeta)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn IsDownloaded(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.IsDownloaded)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn IsHidden(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.IsHidden)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn SetIsHidden(&self, value: i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.SetIsHidden)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn IsInstalled(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.IsInstalled)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn IsMandatory(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.IsMandatory)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn IsUninstallable(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.IsUninstallable)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Languages(&self) -> ::windows_core::Result<IStringCollection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.Languages)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IStringCollection>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn LastDeploymentChangeTime(&self) -> ::windows_core::Result<f64> {
        let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.LastDeploymentChangeTime)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<f64>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MaxDownloadSize(&self) -> ::windows_core::Result<super::super::Foundation::DECIMAL> {
        let mut result__ = ::core::mem::MaybeUninit::<super::super::Foundation::DECIMAL>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.MaxDownloadSize)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::DECIMAL>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MinDownloadSize(&self) -> ::windows_core::Result<super::super::Foundation::DECIMAL> {
        let mut result__ = ::core::mem::MaybeUninit::<super::super::Foundation::DECIMAL>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.MinDownloadSize)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::DECIMAL>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn MoreInfoUrls(&self) -> ::windows_core::Result<IStringCollection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.MoreInfoUrls)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IStringCollection>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MsrcSeverity(&self) -> ::windows_core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::super::Foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.MsrcSeverity)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn RecommendedCpuSpeed(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.RecommendedCpuSpeed)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn RecommendedHardDiskSpace(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.RecommendedHardDiskSpace)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn RecommendedMemory(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.RecommendedMemory)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ReleaseNotes(&self) -> ::windows_core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::super::Foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.ReleaseNotes)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SecurityBulletinIDs(&self) -> ::windows_core::Result<IStringCollection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.SecurityBulletinIDs)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IStringCollection>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SupersededUpdateIDs(&self) -> ::windows_core::Result<IStringCollection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.SupersededUpdateIDs)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IStringCollection>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SupportUrl(&self) -> ::windows_core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::super::Foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.SupportUrl)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn Type(&self) -> ::windows_core::Result<UpdateType> {
        let mut result__ = ::core::mem::MaybeUninit::<UpdateType>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.Type)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<UpdateType>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn UninstallationNotes(&self) -> ::windows_core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::super::Foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.UninstallationNotes)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn UninstallationBehavior(&self) -> ::windows_core::Result<IInstallationBehavior> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.UninstallationBehavior)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IInstallationBehavior>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn UninstallationSteps(&self) -> ::windows_core::Result<IStringCollection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.UninstallationSteps)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IStringCollection>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn KBArticleIDs(&self) -> ::windows_core::Result<IStringCollection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.KBArticleIDs)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IStringCollection>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn AcceptEula(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.AcceptEula)(::windows_core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn DeploymentAction(&self) -> ::windows_core::Result<DeploymentAction> {
        let mut result__ = ::core::mem::MaybeUninit::<DeploymentAction>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.DeploymentAction)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<DeploymentAction>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CopyFromCache<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, path: Param0, toextractcabfiles: i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.CopyFromCache)(::windows_core::Interface::as_raw(self), path.into_param().abi(), ::core::mem::transmute(toextractcabfiles)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn DownloadPriority(&self) -> ::windows_core::Result<DownloadPriority> {
        let mut result__ = ::core::mem::MaybeUninit::<DownloadPriority>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.DownloadPriority)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<DownloadPriority>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn DownloadContents(&self) -> ::windows_core::Result<IUpdateDownloadContentCollection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.DownloadContents)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IUpdateDownloadContentCollection>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn RebootRequired(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).base__.RebootRequired)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn IsPresent(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).base__.IsPresent)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CveIDs(&self) -> ::windows_core::Result<IStringCollection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.CveIDs)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IStringCollection>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CopyToCache<'a, Param0: ::windows_core::IntoParam<'a, IStringCollection>>(&self, pfiles: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.CopyToCache)(::windows_core::Interface::as_raw(self), pfiles.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn BrowseOnly(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).BrowseOnly)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IUpdate3> for ::windows_core::IUnknown {
    fn from(value: IUpdate3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IUpdate3> for ::windows_core::IUnknown {
    fn from(value: &IUpdate3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IUpdate3 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IUpdate3 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IUpdate3> for super::Com::IDispatch {
    fn from(value: IUpdate3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IUpdate3> for super::Com::IDispatch {
    fn from(value: &IUpdate3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for IUpdate3 {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a IUpdate3 {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IUpdate3> for IUpdate {
    fn from(value: IUpdate3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IUpdate3> for IUpdate {
    fn from(value: &IUpdate3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, IUpdate> for IUpdate3 {
    fn into_param(self) -> ::windows_core::Param<'a, IUpdate> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, IUpdate> for &'a IUpdate3 {
    fn into_param(self) -> ::windows_core::Param<'a, IUpdate> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IUpdate3> for IUpdate2 {
    fn from(value: IUpdate3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IUpdate3> for IUpdate2 {
    fn from(value: &IUpdate3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, IUpdate2> for IUpdate3 {
    fn into_param(self) -> ::windows_core::Param<'a, IUpdate2> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, IUpdate2> for &'a IUpdate3 {
    fn into_param(self) -> ::windows_core::Param<'a, IUpdate2> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IUpdate3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IUpdate3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IUpdate3 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IUpdate3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUpdate3").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IUpdate3 {
    type Vtable = IUpdate3_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x112eda6b_95b3_476f_9d90_aee82c6b8181);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IUpdate3_Vtbl {
    pub base__: IUpdate2_Vtbl,
    pub BrowseOnly: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IUpdate4(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IUpdate4 {
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Title(&self) -> ::windows_core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::super::Foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.Title)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn AutoSelectOnWebSites(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.AutoSelectOnWebSites)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn BundledUpdates(&self) -> ::windows_core::Result<IUpdateCollection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.BundledUpdates)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IUpdateCollection>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn CanRequireSource(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.CanRequireSource)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Categories(&self) -> ::windows_core::Result<ICategoryCollection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.Categories)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ICategoryCollection>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Deadline(&self) -> ::windows_core::Result<super::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.Deadline)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::VARIANT>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn DeltaCompressedContentAvailable(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.DeltaCompressedContentAvailable)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn DeltaCompressedContentPreferred(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.DeltaCompressedContentPreferred)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Description(&self) -> ::windows_core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::super::Foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.Description)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn EulaAccepted(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.EulaAccepted)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EulaText(&self) -> ::windows_core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::super::Foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.EulaText)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn HandlerID(&self) -> ::windows_core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::super::Foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.HandlerID)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Identity(&self) -> ::windows_core::Result<IUpdateIdentity> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.Identity)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IUpdateIdentity>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Image(&self) -> ::windows_core::Result<IImageInformation> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.Image)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IImageInformation>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn InstallationBehavior(&self) -> ::windows_core::Result<IInstallationBehavior> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.InstallationBehavior)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IInstallationBehavior>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn IsBeta(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.IsBeta)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn IsDownloaded(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.IsDownloaded)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn IsHidden(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.IsHidden)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn SetIsHidden(&self, value: i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.base__.SetIsHidden)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn IsInstalled(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.IsInstalled)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn IsMandatory(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.IsMandatory)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn IsUninstallable(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.IsUninstallable)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Languages(&self) -> ::windows_core::Result<IStringCollection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.Languages)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IStringCollection>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn LastDeploymentChangeTime(&self) -> ::windows_core::Result<f64> {
        let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.LastDeploymentChangeTime)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<f64>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MaxDownloadSize(&self) -> ::windows_core::Result<super::super::Foundation::DECIMAL> {
        let mut result__ = ::core::mem::MaybeUninit::<super::super::Foundation::DECIMAL>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.MaxDownloadSize)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::DECIMAL>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MinDownloadSize(&self) -> ::windows_core::Result<super::super::Foundation::DECIMAL> {
        let mut result__ = ::core::mem::MaybeUninit::<super::super::Foundation::DECIMAL>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.MinDownloadSize)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::DECIMAL>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn MoreInfoUrls(&self) -> ::windows_core::Result<IStringCollection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.MoreInfoUrls)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IStringCollection>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MsrcSeverity(&self) -> ::windows_core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::super::Foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.MsrcSeverity)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn RecommendedCpuSpeed(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.RecommendedCpuSpeed)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn RecommendedHardDiskSpace(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.RecommendedHardDiskSpace)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn RecommendedMemory(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.RecommendedMemory)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ReleaseNotes(&self) -> ::windows_core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::super::Foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.ReleaseNotes)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SecurityBulletinIDs(&self) -> ::windows_core::Result<IStringCollection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.SecurityBulletinIDs)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IStringCollection>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SupersededUpdateIDs(&self) -> ::windows_core::Result<IStringCollection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.SupersededUpdateIDs)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IStringCollection>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SupportUrl(&self) -> ::windows_core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::super::Foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.SupportUrl)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn Type(&self) -> ::windows_core::Result<UpdateType> {
        let mut result__ = ::core::mem::MaybeUninit::<UpdateType>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.Type)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<UpdateType>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn UninstallationNotes(&self) -> ::windows_core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::super::Foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.UninstallationNotes)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn UninstallationBehavior(&self) -> ::windows_core::Result<IInstallationBehavior> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.UninstallationBehavior)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IInstallationBehavior>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn UninstallationSteps(&self) -> ::windows_core::Result<IStringCollection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.UninstallationSteps)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IStringCollection>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn KBArticleIDs(&self) -> ::windows_core::Result<IStringCollection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.KBArticleIDs)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IStringCollection>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn AcceptEula(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.base__.AcceptEula)(::windows_core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn DeploymentAction(&self) -> ::windows_core::Result<DeploymentAction> {
        let mut result__ = ::core::mem::MaybeUninit::<DeploymentAction>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.DeploymentAction)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<DeploymentAction>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CopyFromCache<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, path: Param0, toextractcabfiles: i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.base__.CopyFromCache)(::windows_core::Interface::as_raw(self), path.into_param().abi(), ::core::mem::transmute(toextractcabfiles)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn DownloadPriority(&self) -> ::windows_core::Result<DownloadPriority> {
        let mut result__ = ::core::mem::MaybeUninit::<DownloadPriority>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.DownloadPriority)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<DownloadPriority>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn DownloadContents(&self) -> ::windows_core::Result<IUpdateDownloadContentCollection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.DownloadContents)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IUpdateDownloadContentCollection>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn RebootRequired(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.RebootRequired)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn IsPresent(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.IsPresent)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CveIDs(&self) -> ::windows_core::Result<IStringCollection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.CveIDs)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IStringCollection>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CopyToCache<'a, Param0: ::windows_core::IntoParam<'a, IStringCollection>>(&self, pfiles: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.CopyToCache)(::windows_core::Interface::as_raw(self), pfiles.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn BrowseOnly(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).base__.BrowseOnly)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn PerUser(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).PerUser)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IUpdate4> for ::windows_core::IUnknown {
    fn from(value: IUpdate4) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IUpdate4> for ::windows_core::IUnknown {
    fn from(value: &IUpdate4) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IUpdate4 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IUpdate4 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IUpdate4> for super::Com::IDispatch {
    fn from(value: IUpdate4) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IUpdate4> for super::Com::IDispatch {
    fn from(value: &IUpdate4) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for IUpdate4 {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a IUpdate4 {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IUpdate4> for IUpdate {
    fn from(value: IUpdate4) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IUpdate4> for IUpdate {
    fn from(value: &IUpdate4) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, IUpdate> for IUpdate4 {
    fn into_param(self) -> ::windows_core::Param<'a, IUpdate> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, IUpdate> for &'a IUpdate4 {
    fn into_param(self) -> ::windows_core::Param<'a, IUpdate> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IUpdate4> for IUpdate2 {
    fn from(value: IUpdate4) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IUpdate4> for IUpdate2 {
    fn from(value: &IUpdate4) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, IUpdate2> for IUpdate4 {
    fn into_param(self) -> ::windows_core::Param<'a, IUpdate2> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, IUpdate2> for &'a IUpdate4 {
    fn into_param(self) -> ::windows_core::Param<'a, IUpdate2> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IUpdate4> for IUpdate3 {
    fn from(value: IUpdate4) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IUpdate4> for IUpdate3 {
    fn from(value: &IUpdate4) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, IUpdate3> for IUpdate4 {
    fn into_param(self) -> ::windows_core::Param<'a, IUpdate3> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, IUpdate3> for &'a IUpdate4 {
    fn into_param(self) -> ::windows_core::Param<'a, IUpdate3> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IUpdate4 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IUpdate4 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IUpdate4 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IUpdate4 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUpdate4").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IUpdate4 {
    type Vtable = IUpdate4_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x27e94b0d_5139_49a2_9a61_93522dc54652);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IUpdate4_Vtbl {
    pub base__: IUpdate3_Vtbl,
    pub PerUser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IUpdate5(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IUpdate5 {
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Title(&self) -> ::windows_core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::super::Foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.Title)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn AutoSelectOnWebSites(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.AutoSelectOnWebSites)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn BundledUpdates(&self) -> ::windows_core::Result<IUpdateCollection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.BundledUpdates)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IUpdateCollection>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn CanRequireSource(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.CanRequireSource)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Categories(&self) -> ::windows_core::Result<ICategoryCollection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.Categories)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ICategoryCollection>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Deadline(&self) -> ::windows_core::Result<super::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.Deadline)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::VARIANT>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn DeltaCompressedContentAvailable(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.DeltaCompressedContentAvailable)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn DeltaCompressedContentPreferred(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.DeltaCompressedContentPreferred)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Description(&self) -> ::windows_core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::super::Foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.Description)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn EulaAccepted(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.EulaAccepted)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EulaText(&self) -> ::windows_core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::super::Foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.EulaText)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn HandlerID(&self) -> ::windows_core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::super::Foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.HandlerID)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Identity(&self) -> ::windows_core::Result<IUpdateIdentity> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.Identity)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IUpdateIdentity>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Image(&self) -> ::windows_core::Result<IImageInformation> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.Image)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IImageInformation>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn InstallationBehavior(&self) -> ::windows_core::Result<IInstallationBehavior> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.InstallationBehavior)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IInstallationBehavior>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn IsBeta(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.IsBeta)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn IsDownloaded(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.IsDownloaded)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn IsHidden(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.IsHidden)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn SetIsHidden(&self, value: i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.SetIsHidden)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn IsInstalled(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.IsInstalled)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn IsMandatory(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.IsMandatory)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn IsUninstallable(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.IsUninstallable)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Languages(&self) -> ::windows_core::Result<IStringCollection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.Languages)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IStringCollection>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn LastDeploymentChangeTime(&self) -> ::windows_core::Result<f64> {
        let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.LastDeploymentChangeTime)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<f64>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MaxDownloadSize(&self) -> ::windows_core::Result<super::super::Foundation::DECIMAL> {
        let mut result__ = ::core::mem::MaybeUninit::<super::super::Foundation::DECIMAL>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.MaxDownloadSize)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::DECIMAL>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MinDownloadSize(&self) -> ::windows_core::Result<super::super::Foundation::DECIMAL> {
        let mut result__ = ::core::mem::MaybeUninit::<super::super::Foundation::DECIMAL>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.MinDownloadSize)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::DECIMAL>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn MoreInfoUrls(&self) -> ::windows_core::Result<IStringCollection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.MoreInfoUrls)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IStringCollection>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MsrcSeverity(&self) -> ::windows_core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::super::Foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.MsrcSeverity)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn RecommendedCpuSpeed(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.RecommendedCpuSpeed)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn RecommendedHardDiskSpace(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.RecommendedHardDiskSpace)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn RecommendedMemory(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.RecommendedMemory)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ReleaseNotes(&self) -> ::windows_core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::super::Foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.ReleaseNotes)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SecurityBulletinIDs(&self) -> ::windows_core::Result<IStringCollection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.SecurityBulletinIDs)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IStringCollection>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SupersededUpdateIDs(&self) -> ::windows_core::Result<IStringCollection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.SupersededUpdateIDs)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IStringCollection>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SupportUrl(&self) -> ::windows_core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::super::Foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.SupportUrl)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn Type(&self) -> ::windows_core::Result<UpdateType> {
        let mut result__ = ::core::mem::MaybeUninit::<UpdateType>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.Type)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<UpdateType>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn UninstallationNotes(&self) -> ::windows_core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::super::Foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.UninstallationNotes)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn UninstallationBehavior(&self) -> ::windows_core::Result<IInstallationBehavior> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.UninstallationBehavior)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IInstallationBehavior>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn UninstallationSteps(&self) -> ::windows_core::Result<IStringCollection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.UninstallationSteps)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IStringCollection>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn KBArticleIDs(&self) -> ::windows_core::Result<IStringCollection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.KBArticleIDs)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IStringCollection>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn AcceptEula(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.AcceptEula)(::windows_core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn DeploymentAction(&self) -> ::windows_core::Result<DeploymentAction> {
        let mut result__ = ::core::mem::MaybeUninit::<DeploymentAction>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.DeploymentAction)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<DeploymentAction>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CopyFromCache<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, path: Param0, toextractcabfiles: i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.CopyFromCache)(::windows_core::Interface::as_raw(self), path.into_param().abi(), ::core::mem::transmute(toextractcabfiles)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn DownloadPriority(&self) -> ::windows_core::Result<DownloadPriority> {
        let mut result__ = ::core::mem::MaybeUninit::<DownloadPriority>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.DownloadPriority)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<DownloadPriority>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn DownloadContents(&self) -> ::windows_core::Result<IUpdateDownloadContentCollection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.DownloadContents)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IUpdateDownloadContentCollection>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn RebootRequired(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.RebootRequired)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn IsPresent(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.IsPresent)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CveIDs(&self) -> ::windows_core::Result<IStringCollection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.CveIDs)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IStringCollection>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CopyToCache<'a, Param0: ::windows_core::IntoParam<'a, IStringCollection>>(&self, pfiles: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.base__.CopyToCache)(::windows_core::Interface::as_raw(self), pfiles.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn BrowseOnly(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.BrowseOnly)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn PerUser(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).base__.PerUser)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn AutoSelection(&self) -> ::windows_core::Result<AutoSelectionMode> {
        let mut result__ = ::core::mem::MaybeUninit::<AutoSelectionMode>::zeroed();
        (::windows_core::Interface::vtable(self).AutoSelection)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<AutoSelectionMode>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn AutoDownload(&self) -> ::windows_core::Result<AutoDownloadMode> {
        let mut result__ = ::core::mem::MaybeUninit::<AutoDownloadMode>::zeroed();
        (::windows_core::Interface::vtable(self).AutoDownload)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<AutoDownloadMode>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IUpdate5> for ::windows_core::IUnknown {
    fn from(value: IUpdate5) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IUpdate5> for ::windows_core::IUnknown {
    fn from(value: &IUpdate5) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IUpdate5 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IUpdate5 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IUpdate5> for super::Com::IDispatch {
    fn from(value: IUpdate5) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IUpdate5> for super::Com::IDispatch {
    fn from(value: &IUpdate5) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for IUpdate5 {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a IUpdate5 {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IUpdate5> for IUpdate {
    fn from(value: IUpdate5) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IUpdate5> for IUpdate {
    fn from(value: &IUpdate5) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, IUpdate> for IUpdate5 {
    fn into_param(self) -> ::windows_core::Param<'a, IUpdate> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, IUpdate> for &'a IUpdate5 {
    fn into_param(self) -> ::windows_core::Param<'a, IUpdate> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IUpdate5> for IUpdate2 {
    fn from(value: IUpdate5) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IUpdate5> for IUpdate2 {
    fn from(value: &IUpdate5) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, IUpdate2> for IUpdate5 {
    fn into_param(self) -> ::windows_core::Param<'a, IUpdate2> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, IUpdate2> for &'a IUpdate5 {
    fn into_param(self) -> ::windows_core::Param<'a, IUpdate2> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IUpdate5> for IUpdate3 {
    fn from(value: IUpdate5) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IUpdate5> for IUpdate3 {
    fn from(value: &IUpdate5) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, IUpdate3> for IUpdate5 {
    fn into_param(self) -> ::windows_core::Param<'a, IUpdate3> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, IUpdate3> for &'a IUpdate5 {
    fn into_param(self) -> ::windows_core::Param<'a, IUpdate3> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IUpdate5> for IUpdate4 {
    fn from(value: IUpdate5) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IUpdate5> for IUpdate4 {
    fn from(value: &IUpdate5) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, IUpdate4> for IUpdate5 {
    fn into_param(self) -> ::windows_core::Param<'a, IUpdate4> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, IUpdate4> for &'a IUpdate5 {
    fn into_param(self) -> ::windows_core::Param<'a, IUpdate4> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IUpdate5 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IUpdate5 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IUpdate5 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IUpdate5 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUpdate5").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IUpdate5 {
    type Vtable = IUpdate5_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc1c2f21a_d2f4_4902_b5c6_8a081c19a890);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IUpdate5_Vtbl {
    pub base__: IUpdate4_Vtbl,
    pub AutoSelection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut AutoSelectionMode) -> ::windows_core::HRESULT,
    pub AutoDownload: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut AutoDownloadMode) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IUpdateCollection(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IUpdateCollection {
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn get_Item(&self, index: i32) -> ::windows_core::Result<IUpdate> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).get_Item)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(index), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IUpdate>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn put_Item<'a, Param1: ::windows_core::IntoParam<'a, IUpdate>>(&self, index: i32, value: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).put_Item)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(index), value.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn _NewEnum(&self) -> ::windows_core::Result<::windows_core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
        (::windows_core::Interface::vtable(self)._NewEnum)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::IUnknown>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn Count(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).Count)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn ReadOnly(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).ReadOnly)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Add<'a, Param0: ::windows_core::IntoParam<'a, IUpdate>>(&self, value: Param0) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).Add)(::windows_core::Interface::as_raw(self), value.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn Clear(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Clear)(::windows_core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Copy(&self) -> ::windows_core::Result<IUpdateCollection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Copy)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IUpdateCollection>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Insert<'a, Param1: ::windows_core::IntoParam<'a, IUpdate>>(&self, index: i32, value: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Insert)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(index), value.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn RemoveAt(&self, index: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).RemoveAt)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(index)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IUpdateCollection> for ::windows_core::IUnknown {
    fn from(value: IUpdateCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IUpdateCollection> for ::windows_core::IUnknown {
    fn from(value: &IUpdateCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IUpdateCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IUpdateCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IUpdateCollection> for super::Com::IDispatch {
    fn from(value: IUpdateCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IUpdateCollection> for super::Com::IDispatch {
    fn from(value: &IUpdateCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for IUpdateCollection {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a IUpdateCollection {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IUpdateCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IUpdateCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IUpdateCollection {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IUpdateCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUpdateCollection").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IUpdateCollection {
    type Vtable = IUpdateCollection_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x07f7438c_7709_4ca5_b518_91279288134e);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IUpdateCollection_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub get_Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: i32, retval: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    get_Item: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub put_Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: i32, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    put_Item: usize,
    pub _NewEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT,
    pub ReadOnly: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Add: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr, retval: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Add: usize,
    pub Clear: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Copy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Copy: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Insert: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: i32, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Insert: usize,
    pub RemoveAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: i32) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IUpdateDownloadContent(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IUpdateDownloadContent {
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DownloadUrl(&self) -> ::windows_core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::super::Foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).DownloadUrl)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IUpdateDownloadContent> for ::windows_core::IUnknown {
    fn from(value: IUpdateDownloadContent) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IUpdateDownloadContent> for ::windows_core::IUnknown {
    fn from(value: &IUpdateDownloadContent) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IUpdateDownloadContent {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IUpdateDownloadContent {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IUpdateDownloadContent> for super::Com::IDispatch {
    fn from(value: IUpdateDownloadContent) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IUpdateDownloadContent> for super::Com::IDispatch {
    fn from(value: &IUpdateDownloadContent) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for IUpdateDownloadContent {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a IUpdateDownloadContent {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IUpdateDownloadContent {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IUpdateDownloadContent {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IUpdateDownloadContent {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IUpdateDownloadContent {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUpdateDownloadContent").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IUpdateDownloadContent {
    type Vtable = IUpdateDownloadContent_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x54a2cb2d_9a0c_48b6_8a50_9abb69ee2d02);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IUpdateDownloadContent_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub DownloadUrl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DownloadUrl: usize,
}
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IUpdateDownloadContent2(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IUpdateDownloadContent2 {
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DownloadUrl(&self) -> ::windows_core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::super::Foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.DownloadUrl)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn IsDeltaCompressedContent(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).IsDeltaCompressedContent)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IUpdateDownloadContent2> for ::windows_core::IUnknown {
    fn from(value: IUpdateDownloadContent2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IUpdateDownloadContent2> for ::windows_core::IUnknown {
    fn from(value: &IUpdateDownloadContent2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IUpdateDownloadContent2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IUpdateDownloadContent2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IUpdateDownloadContent2> for super::Com::IDispatch {
    fn from(value: IUpdateDownloadContent2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IUpdateDownloadContent2> for super::Com::IDispatch {
    fn from(value: &IUpdateDownloadContent2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for IUpdateDownloadContent2 {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a IUpdateDownloadContent2 {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IUpdateDownloadContent2> for IUpdateDownloadContent {
    fn from(value: IUpdateDownloadContent2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IUpdateDownloadContent2> for IUpdateDownloadContent {
    fn from(value: &IUpdateDownloadContent2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, IUpdateDownloadContent> for IUpdateDownloadContent2 {
    fn into_param(self) -> ::windows_core::Param<'a, IUpdateDownloadContent> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, IUpdateDownloadContent> for &'a IUpdateDownloadContent2 {
    fn into_param(self) -> ::windows_core::Param<'a, IUpdateDownloadContent> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IUpdateDownloadContent2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IUpdateDownloadContent2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IUpdateDownloadContent2 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IUpdateDownloadContent2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUpdateDownloadContent2").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IUpdateDownloadContent2 {
    type Vtable = IUpdateDownloadContent2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc97ad11b_f257_420b_9d9f_377f733f6f68);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IUpdateDownloadContent2_Vtbl {
    pub base__: IUpdateDownloadContent_Vtbl,
    pub IsDeltaCompressedContent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IUpdateDownloadContentCollection(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IUpdateDownloadContentCollection {
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn get_Item(&self, index: i32) -> ::windows_core::Result<IUpdateDownloadContent> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).get_Item)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(index), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IUpdateDownloadContent>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn _NewEnum(&self) -> ::windows_core::Result<::windows_core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
        (::windows_core::Interface::vtable(self)._NewEnum)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::IUnknown>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn Count(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).Count)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IUpdateDownloadContentCollection> for ::windows_core::IUnknown {
    fn from(value: IUpdateDownloadContentCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IUpdateDownloadContentCollection> for ::windows_core::IUnknown {
    fn from(value: &IUpdateDownloadContentCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IUpdateDownloadContentCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IUpdateDownloadContentCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IUpdateDownloadContentCollection> for super::Com::IDispatch {
    fn from(value: IUpdateDownloadContentCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IUpdateDownloadContentCollection> for super::Com::IDispatch {
    fn from(value: &IUpdateDownloadContentCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for IUpdateDownloadContentCollection {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a IUpdateDownloadContentCollection {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IUpdateDownloadContentCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IUpdateDownloadContentCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IUpdateDownloadContentCollection {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IUpdateDownloadContentCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUpdateDownloadContentCollection").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IUpdateDownloadContentCollection {
    type Vtable = IUpdateDownloadContentCollection_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xbc5513c8_b3b8_4bf7_a4d4_361c0d8c88ba);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IUpdateDownloadContentCollection_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub get_Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: i32, retval: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    get_Item: usize,
    pub _NewEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IUpdateDownloadResult(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IUpdateDownloadResult {
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn HResult(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).HResult)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn ResultCode(&self) -> ::windows_core::Result<OperationResultCode> {
        let mut result__ = ::core::mem::MaybeUninit::<OperationResultCode>::zeroed();
        (::windows_core::Interface::vtable(self).ResultCode)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<OperationResultCode>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IUpdateDownloadResult> for ::windows_core::IUnknown {
    fn from(value: IUpdateDownloadResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IUpdateDownloadResult> for ::windows_core::IUnknown {
    fn from(value: &IUpdateDownloadResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IUpdateDownloadResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IUpdateDownloadResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IUpdateDownloadResult> for super::Com::IDispatch {
    fn from(value: IUpdateDownloadResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IUpdateDownloadResult> for super::Com::IDispatch {
    fn from(value: &IUpdateDownloadResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for IUpdateDownloadResult {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a IUpdateDownloadResult {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IUpdateDownloadResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IUpdateDownloadResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IUpdateDownloadResult {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IUpdateDownloadResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUpdateDownloadResult").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IUpdateDownloadResult {
    type Vtable = IUpdateDownloadResult_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xbf99af76_b575_42ad_8aa4_33cbb5477af1);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IUpdateDownloadResult_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub HResult: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT,
    pub ResultCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut OperationResultCode) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IUpdateDownloader(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IUpdateDownloader {
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ClientApplicationID(&self) -> ::windows_core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::super::Foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).ClientApplicationID)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetClientApplicationID<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, value: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetClientApplicationID)(::windows_core::Interface::as_raw(self), value.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn IsForced(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).IsForced)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn SetIsForced(&self, value: i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetIsForced)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn Priority(&self) -> ::windows_core::Result<DownloadPriority> {
        let mut result__ = ::core::mem::MaybeUninit::<DownloadPriority>::zeroed();
        (::windows_core::Interface::vtable(self).Priority)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<DownloadPriority>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn SetPriority(&self, value: DownloadPriority) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetPriority)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Updates(&self) -> ::windows_core::Result<IUpdateCollection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Updates)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IUpdateCollection>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetUpdates<'a, Param0: ::windows_core::IntoParam<'a, IUpdateCollection>>(&self, value: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetUpdates)(::windows_core::Interface::as_raw(self), value.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn BeginDownload<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::IUnknown>, Param1: ::windows_core::IntoParam<'a, ::windows_core::IUnknown>, Param2: ::windows_core::IntoParam<'a, super::Com::VARIANT>>(&self, onprogresschanged: Param0, oncompleted: Param1, state: Param2) -> ::windows_core::Result<IDownloadJob> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).BeginDownload)(::windows_core::Interface::as_raw(self), onprogresschanged.into_param().abi(), oncompleted.into_param().abi(), state.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IDownloadJob>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Download(&self) -> ::windows_core::Result<IDownloadResult> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Download)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IDownloadResult>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn EndDownload<'a, Param0: ::windows_core::IntoParam<'a, IDownloadJob>>(&self, value: Param0) -> ::windows_core::Result<IDownloadResult> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).EndDownload)(::windows_core::Interface::as_raw(self), value.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IDownloadResult>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IUpdateDownloader> for ::windows_core::IUnknown {
    fn from(value: IUpdateDownloader) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IUpdateDownloader> for ::windows_core::IUnknown {
    fn from(value: &IUpdateDownloader) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IUpdateDownloader {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IUpdateDownloader {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IUpdateDownloader> for super::Com::IDispatch {
    fn from(value: IUpdateDownloader) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IUpdateDownloader> for super::Com::IDispatch {
    fn from(value: &IUpdateDownloader) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for IUpdateDownloader {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a IUpdateDownloader {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IUpdateDownloader {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IUpdateDownloader {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IUpdateDownloader {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IUpdateDownloader {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUpdateDownloader").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IUpdateDownloader {
    type Vtable = IUpdateDownloader_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x68f1c6f9_7ecc_4666_a464_247fe12496c3);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IUpdateDownloader_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub ClientApplicationID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ClientApplicationID: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetClientApplicationID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetClientApplicationID: usize,
    pub IsForced: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows_core::HRESULT,
    pub SetIsForced: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i16) -> ::windows_core::HRESULT,
    pub Priority: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut DownloadPriority) -> ::windows_core::HRESULT,
    pub SetPriority: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: DownloadPriority) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Updates: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Updates: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetUpdates: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetUpdates: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub BeginDownload: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, onprogresschanged: *mut ::core::ffi::c_void, oncompleted: *mut ::core::ffi::c_void, state: ::core::mem::ManuallyDrop<super::Com::VARIANT>, retval: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    BeginDownload: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Download: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Download: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub EndDownload: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr, retval: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    EndDownload: usize,
}
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IUpdateException(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IUpdateException {
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Message(&self) -> ::windows_core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::super::Foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).Message)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn HResult(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).HResult)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn Context(&self) -> ::windows_core::Result<UpdateExceptionContext> {
        let mut result__ = ::core::mem::MaybeUninit::<UpdateExceptionContext>::zeroed();
        (::windows_core::Interface::vtable(self).Context)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<UpdateExceptionContext>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IUpdateException> for ::windows_core::IUnknown {
    fn from(value: IUpdateException) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IUpdateException> for ::windows_core::IUnknown {
    fn from(value: &IUpdateException) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IUpdateException {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IUpdateException {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IUpdateException> for super::Com::IDispatch {
    fn from(value: IUpdateException) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IUpdateException> for super::Com::IDispatch {
    fn from(value: &IUpdateException) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for IUpdateException {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a IUpdateException {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IUpdateException {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IUpdateException {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IUpdateException {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IUpdateException {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUpdateException").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IUpdateException {
    type Vtable = IUpdateException_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa376dd5e_09d4_427f_af7c_fed5b6e1c1d6);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IUpdateException_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub Message: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Message: usize,
    pub HResult: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT,
    pub Context: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut UpdateExceptionContext) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IUpdateExceptionCollection(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IUpdateExceptionCollection {
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn get_Item(&self, index: i32) -> ::windows_core::Result<IUpdateException> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).get_Item)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(index), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IUpdateException>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn _NewEnum(&self) -> ::windows_core::Result<::windows_core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
        (::windows_core::Interface::vtable(self)._NewEnum)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::IUnknown>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn Count(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).Count)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IUpdateExceptionCollection> for ::windows_core::IUnknown {
    fn from(value: IUpdateExceptionCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IUpdateExceptionCollection> for ::windows_core::IUnknown {
    fn from(value: &IUpdateExceptionCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IUpdateExceptionCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IUpdateExceptionCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IUpdateExceptionCollection> for super::Com::IDispatch {
    fn from(value: IUpdateExceptionCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IUpdateExceptionCollection> for super::Com::IDispatch {
    fn from(value: &IUpdateExceptionCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for IUpdateExceptionCollection {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a IUpdateExceptionCollection {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IUpdateExceptionCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IUpdateExceptionCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IUpdateExceptionCollection {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IUpdateExceptionCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUpdateExceptionCollection").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IUpdateExceptionCollection {
    type Vtable = IUpdateExceptionCollection_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x503626a3_8e14_4729_9355_0fe664bd2321);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IUpdateExceptionCollection_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub get_Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: i32, retval: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    get_Item: usize,
    pub _NewEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IUpdateHistoryEntry(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IUpdateHistoryEntry {
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn Operation(&self) -> ::windows_core::Result<UpdateOperation> {
        let mut result__ = ::core::mem::MaybeUninit::<UpdateOperation>::zeroed();
        (::windows_core::Interface::vtable(self).Operation)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<UpdateOperation>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn ResultCode(&self) -> ::windows_core::Result<OperationResultCode> {
        let mut result__ = ::core::mem::MaybeUninit::<OperationResultCode>::zeroed();
        (::windows_core::Interface::vtable(self).ResultCode)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<OperationResultCode>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn HResult(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).HResult)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn Date(&self) -> ::windows_core::Result<f64> {
        let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
        (::windows_core::Interface::vtable(self).Date)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<f64>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn UpdateIdentity(&self) -> ::windows_core::Result<IUpdateIdentity> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).UpdateIdentity)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IUpdateIdentity>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Title(&self) -> ::windows_core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::super::Foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).Title)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Description(&self) -> ::windows_core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::super::Foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).Description)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn UnmappedResultCode(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).UnmappedResultCode)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ClientApplicationID(&self) -> ::windows_core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::super::Foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).ClientApplicationID)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn ServerSelection(&self) -> ::windows_core::Result<ServerSelection> {
        let mut result__ = ::core::mem::MaybeUninit::<ServerSelection>::zeroed();
        (::windows_core::Interface::vtable(self).ServerSelection)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ServerSelection>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ServiceID(&self) -> ::windows_core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::super::Foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).ServiceID)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn UninstallationSteps(&self) -> ::windows_core::Result<IStringCollection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).UninstallationSteps)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IStringCollection>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn UninstallationNotes(&self) -> ::windows_core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::super::Foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).UninstallationNotes)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SupportUrl(&self) -> ::windows_core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::super::Foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).SupportUrl)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IUpdateHistoryEntry> for ::windows_core::IUnknown {
    fn from(value: IUpdateHistoryEntry) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IUpdateHistoryEntry> for ::windows_core::IUnknown {
    fn from(value: &IUpdateHistoryEntry) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IUpdateHistoryEntry {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IUpdateHistoryEntry {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IUpdateHistoryEntry> for super::Com::IDispatch {
    fn from(value: IUpdateHistoryEntry) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IUpdateHistoryEntry> for super::Com::IDispatch {
    fn from(value: &IUpdateHistoryEntry) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for IUpdateHistoryEntry {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a IUpdateHistoryEntry {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IUpdateHistoryEntry {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IUpdateHistoryEntry {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IUpdateHistoryEntry {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IUpdateHistoryEntry {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUpdateHistoryEntry").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IUpdateHistoryEntry {
    type Vtable = IUpdateHistoryEntry_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xbe56a644_af0e_4e0e_a311_c1d8e695cbff);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IUpdateHistoryEntry_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Operation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut UpdateOperation) -> ::windows_core::HRESULT,
    pub ResultCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut OperationResultCode) -> ::windows_core::HRESULT,
    pub HResult: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT,
    pub Date: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub UpdateIdentity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    UpdateIdentity: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Title: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Title: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Description: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Description: usize,
    pub UnmappedResultCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub ClientApplicationID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ClientApplicationID: usize,
    pub ServerSelection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut ServerSelection) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub ServiceID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ServiceID: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub UninstallationSteps: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    UninstallationSteps: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub UninstallationNotes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    UninstallationNotes: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SupportUrl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SupportUrl: usize,
}
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IUpdateHistoryEntry2(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IUpdateHistoryEntry2 {
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn Operation(&self) -> ::windows_core::Result<UpdateOperation> {
        let mut result__ = ::core::mem::MaybeUninit::<UpdateOperation>::zeroed();
        (::windows_core::Interface::vtable(self).base__.Operation)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<UpdateOperation>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn ResultCode(&self) -> ::windows_core::Result<OperationResultCode> {
        let mut result__ = ::core::mem::MaybeUninit::<OperationResultCode>::zeroed();
        (::windows_core::Interface::vtable(self).base__.ResultCode)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<OperationResultCode>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn HResult(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).base__.HResult)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn Date(&self) -> ::windows_core::Result<f64> {
        let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
        (::windows_core::Interface::vtable(self).base__.Date)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<f64>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn UpdateIdentity(&self) -> ::windows_core::Result<IUpdateIdentity> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.UpdateIdentity)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IUpdateIdentity>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Title(&self) -> ::windows_core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::super::Foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.Title)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Description(&self) -> ::windows_core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::super::Foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.Description)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn UnmappedResultCode(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).base__.UnmappedResultCode)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ClientApplicationID(&self) -> ::windows_core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::super::Foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.ClientApplicationID)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn ServerSelection(&self) -> ::windows_core::Result<ServerSelection> {
        let mut result__ = ::core::mem::MaybeUninit::<ServerSelection>::zeroed();
        (::windows_core::Interface::vtable(self).base__.ServerSelection)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ServerSelection>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ServiceID(&self) -> ::windows_core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::super::Foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.ServiceID)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn UninstallationSteps(&self) -> ::windows_core::Result<IStringCollection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.UninstallationSteps)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IStringCollection>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn UninstallationNotes(&self) -> ::windows_core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::super::Foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.UninstallationNotes)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SupportUrl(&self) -> ::windows_core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::super::Foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.SupportUrl)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Categories(&self) -> ::windows_core::Result<ICategoryCollection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Categories)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ICategoryCollection>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IUpdateHistoryEntry2> for ::windows_core::IUnknown {
    fn from(value: IUpdateHistoryEntry2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IUpdateHistoryEntry2> for ::windows_core::IUnknown {
    fn from(value: &IUpdateHistoryEntry2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IUpdateHistoryEntry2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IUpdateHistoryEntry2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IUpdateHistoryEntry2> for super::Com::IDispatch {
    fn from(value: IUpdateHistoryEntry2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IUpdateHistoryEntry2> for super::Com::IDispatch {
    fn from(value: &IUpdateHistoryEntry2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for IUpdateHistoryEntry2 {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a IUpdateHistoryEntry2 {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IUpdateHistoryEntry2> for IUpdateHistoryEntry {
    fn from(value: IUpdateHistoryEntry2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IUpdateHistoryEntry2> for IUpdateHistoryEntry {
    fn from(value: &IUpdateHistoryEntry2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, IUpdateHistoryEntry> for IUpdateHistoryEntry2 {
    fn into_param(self) -> ::windows_core::Param<'a, IUpdateHistoryEntry> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, IUpdateHistoryEntry> for &'a IUpdateHistoryEntry2 {
    fn into_param(self) -> ::windows_core::Param<'a, IUpdateHistoryEntry> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IUpdateHistoryEntry2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IUpdateHistoryEntry2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IUpdateHistoryEntry2 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IUpdateHistoryEntry2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUpdateHistoryEntry2").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IUpdateHistoryEntry2 {
    type Vtable = IUpdateHistoryEntry2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc2bfb780_4539_4132_ab8c_0a8772013ab6);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IUpdateHistoryEntry2_Vtbl {
    pub base__: IUpdateHistoryEntry_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Categories: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Categories: usize,
}
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IUpdateHistoryEntryCollection(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IUpdateHistoryEntryCollection {
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn get_Item(&self, index: i32) -> ::windows_core::Result<IUpdateHistoryEntry> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).get_Item)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(index), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IUpdateHistoryEntry>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn _NewEnum(&self) -> ::windows_core::Result<::windows_core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
        (::windows_core::Interface::vtable(self)._NewEnum)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::IUnknown>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn Count(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).Count)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IUpdateHistoryEntryCollection> for ::windows_core::IUnknown {
    fn from(value: IUpdateHistoryEntryCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IUpdateHistoryEntryCollection> for ::windows_core::IUnknown {
    fn from(value: &IUpdateHistoryEntryCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IUpdateHistoryEntryCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IUpdateHistoryEntryCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IUpdateHistoryEntryCollection> for super::Com::IDispatch {
    fn from(value: IUpdateHistoryEntryCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IUpdateHistoryEntryCollection> for super::Com::IDispatch {
    fn from(value: &IUpdateHistoryEntryCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for IUpdateHistoryEntryCollection {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a IUpdateHistoryEntryCollection {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IUpdateHistoryEntryCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IUpdateHistoryEntryCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IUpdateHistoryEntryCollection {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IUpdateHistoryEntryCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUpdateHistoryEntryCollection").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IUpdateHistoryEntryCollection {
    type Vtable = IUpdateHistoryEntryCollection_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa7f04f3c_a290_435b_aadf_a116c3357a5c);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IUpdateHistoryEntryCollection_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub get_Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: i32, retval: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    get_Item: usize,
    pub _NewEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IUpdateIdentity(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IUpdateIdentity {
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn RevisionNumber(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).RevisionNumber)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn UpdateID(&self) -> ::windows_core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::super::Foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).UpdateID)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IUpdateIdentity> for ::windows_core::IUnknown {
    fn from(value: IUpdateIdentity) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IUpdateIdentity> for ::windows_core::IUnknown {
    fn from(value: &IUpdateIdentity) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IUpdateIdentity {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IUpdateIdentity {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IUpdateIdentity> for super::Com::IDispatch {
    fn from(value: IUpdateIdentity) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IUpdateIdentity> for super::Com::IDispatch {
    fn from(value: &IUpdateIdentity) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for IUpdateIdentity {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a IUpdateIdentity {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IUpdateIdentity {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IUpdateIdentity {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IUpdateIdentity {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IUpdateIdentity {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUpdateIdentity").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IUpdateIdentity {
    type Vtable = IUpdateIdentity_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x46297823_9940_4c09_aed9_cd3ea6d05968);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IUpdateIdentity_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub RevisionNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub UpdateID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    UpdateID: usize,
}
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IUpdateInstallationResult(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IUpdateInstallationResult {
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn HResult(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).HResult)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn RebootRequired(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).RebootRequired)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn ResultCode(&self) -> ::windows_core::Result<OperationResultCode> {
        let mut result__ = ::core::mem::MaybeUninit::<OperationResultCode>::zeroed();
        (::windows_core::Interface::vtable(self).ResultCode)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<OperationResultCode>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IUpdateInstallationResult> for ::windows_core::IUnknown {
    fn from(value: IUpdateInstallationResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IUpdateInstallationResult> for ::windows_core::IUnknown {
    fn from(value: &IUpdateInstallationResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IUpdateInstallationResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IUpdateInstallationResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IUpdateInstallationResult> for super::Com::IDispatch {
    fn from(value: IUpdateInstallationResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IUpdateInstallationResult> for super::Com::IDispatch {
    fn from(value: &IUpdateInstallationResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for IUpdateInstallationResult {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a IUpdateInstallationResult {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IUpdateInstallationResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IUpdateInstallationResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IUpdateInstallationResult {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IUpdateInstallationResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUpdateInstallationResult").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IUpdateInstallationResult {
    type Vtable = IUpdateInstallationResult_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd940f0f8_3cbb_4fd0_993f_471e7f2328ad);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IUpdateInstallationResult_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub HResult: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT,
    pub RebootRequired: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows_core::HRESULT,
    pub ResultCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut OperationResultCode) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IUpdateInstaller(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IUpdateInstaller {
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ClientApplicationID(&self) -> ::windows_core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::super::Foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).ClientApplicationID)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetClientApplicationID<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, value: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetClientApplicationID)(::windows_core::Interface::as_raw(self), value.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn IsForced(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).IsForced)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn SetIsForced(&self, value: i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetIsForced)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ParentHwnd(&self) -> ::windows_core::Result<super::super::Foundation::HWND> {
        let mut result__ = ::core::mem::MaybeUninit::<super::super::Foundation::HWND>::zeroed();
        (::windows_core::Interface::vtable(self).ParentHwnd)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::HWND>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetParentHwnd<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::HWND>>(&self, value: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetParentHwnd)(::windows_core::Interface::as_raw(self), value.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn SetParentWindow<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::IUnknown>>(&self, value: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetParentWindow)(::windows_core::Interface::as_raw(self), value.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn ParentWindow(&self) -> ::windows_core::Result<::windows_core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
        (::windows_core::Interface::vtable(self).ParentWindow)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::IUnknown>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Updates(&self) -> ::windows_core::Result<IUpdateCollection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Updates)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IUpdateCollection>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetUpdates<'a, Param0: ::windows_core::IntoParam<'a, IUpdateCollection>>(&self, value: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetUpdates)(::windows_core::Interface::as_raw(self), value.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn BeginInstall<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::IUnknown>, Param1: ::windows_core::IntoParam<'a, ::windows_core::IUnknown>, Param2: ::windows_core::IntoParam<'a, super::Com::VARIANT>>(&self, onprogresschanged: Param0, oncompleted: Param1, state: Param2) -> ::windows_core::Result<IInstallationJob> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).BeginInstall)(::windows_core::Interface::as_raw(self), onprogresschanged.into_param().abi(), oncompleted.into_param().abi(), state.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IInstallationJob>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn BeginUninstall<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::IUnknown>, Param1: ::windows_core::IntoParam<'a, ::windows_core::IUnknown>, Param2: ::windows_core::IntoParam<'a, super::Com::VARIANT>>(&self, onprogresschanged: Param0, oncompleted: Param1, state: Param2) -> ::windows_core::Result<IInstallationJob> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).BeginUninstall)(::windows_core::Interface::as_raw(self), onprogresschanged.into_param().abi(), oncompleted.into_param().abi(), state.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IInstallationJob>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn EndInstall<'a, Param0: ::windows_core::IntoParam<'a, IInstallationJob>>(&self, value: Param0) -> ::windows_core::Result<IInstallationResult> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).EndInstall)(::windows_core::Interface::as_raw(self), value.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IInstallationResult>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn EndUninstall<'a, Param0: ::windows_core::IntoParam<'a, IInstallationJob>>(&self, value: Param0) -> ::windows_core::Result<IInstallationResult> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).EndUninstall)(::windows_core::Interface::as_raw(self), value.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IInstallationResult>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Install(&self) -> ::windows_core::Result<IInstallationResult> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Install)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IInstallationResult>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn RunWizard<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, dialogtitle: Param0) -> ::windows_core::Result<IInstallationResult> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).RunWizard)(::windows_core::Interface::as_raw(self), dialogtitle.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IInstallationResult>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn IsBusy(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).IsBusy)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Uninstall(&self) -> ::windows_core::Result<IInstallationResult> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Uninstall)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IInstallationResult>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn AllowSourcePrompts(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).AllowSourcePrompts)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn SetAllowSourcePrompts(&self, value: i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetAllowSourcePrompts)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn RebootRequiredBeforeInstallation(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).RebootRequiredBeforeInstallation)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IUpdateInstaller> for ::windows_core::IUnknown {
    fn from(value: IUpdateInstaller) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IUpdateInstaller> for ::windows_core::IUnknown {
    fn from(value: &IUpdateInstaller) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IUpdateInstaller {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IUpdateInstaller {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IUpdateInstaller> for super::Com::IDispatch {
    fn from(value: IUpdateInstaller) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IUpdateInstaller> for super::Com::IDispatch {
    fn from(value: &IUpdateInstaller) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for IUpdateInstaller {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a IUpdateInstaller {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IUpdateInstaller {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IUpdateInstaller {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IUpdateInstaller {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IUpdateInstaller {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUpdateInstaller").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IUpdateInstaller {
    type Vtable = IUpdateInstaller_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7b929c68_ccdc_4226_96b1_8724600b54c2);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IUpdateInstaller_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub ClientApplicationID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ClientApplicationID: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetClientApplicationID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetClientApplicationID: usize,
    pub IsForced: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows_core::HRESULT,
    pub SetIsForced: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i16) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub ParentHwnd: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::HWND) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ParentHwnd: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetParentHwnd: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Foundation::HWND) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetParentHwnd: usize,
    pub SetParentWindow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub ParentWindow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Updates: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Updates: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetUpdates: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetUpdates: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub BeginInstall: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, onprogresschanged: *mut ::core::ffi::c_void, oncompleted: *mut ::core::ffi::c_void, state: ::core::mem::ManuallyDrop<super::Com::VARIANT>, retval: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    BeginInstall: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub BeginUninstall: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, onprogresschanged: *mut ::core::ffi::c_void, oncompleted: *mut ::core::ffi::c_void, state: ::core::mem::ManuallyDrop<super::Com::VARIANT>, retval: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    BeginUninstall: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub EndInstall: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr, retval: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    EndInstall: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub EndUninstall: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr, retval: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    EndUninstall: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Install: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Install: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub RunWizard: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dialogtitle: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, retval: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    RunWizard: usize,
    pub IsBusy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Uninstall: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Uninstall: usize,
    pub AllowSourcePrompts: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows_core::HRESULT,
    pub SetAllowSourcePrompts: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i16) -> ::windows_core::HRESULT,
    pub RebootRequiredBeforeInstallation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IUpdateInstaller2(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IUpdateInstaller2 {
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ClientApplicationID(&self) -> ::windows_core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::super::Foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.ClientApplicationID)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetClientApplicationID<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, value: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetClientApplicationID)(::windows_core::Interface::as_raw(self), value.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn IsForced(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).base__.IsForced)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn SetIsForced(&self, value: i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetIsForced)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ParentHwnd(&self) -> ::windows_core::Result<super::super::Foundation::HWND> {
        let mut result__ = ::core::mem::MaybeUninit::<super::super::Foundation::HWND>::zeroed();
        (::windows_core::Interface::vtable(self).base__.ParentHwnd)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::HWND>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetParentHwnd<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::HWND>>(&self, value: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetParentHwnd)(::windows_core::Interface::as_raw(self), value.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn SetParentWindow<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::IUnknown>>(&self, value: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetParentWindow)(::windows_core::Interface::as_raw(self), value.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn ParentWindow(&self) -> ::windows_core::Result<::windows_core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
        (::windows_core::Interface::vtable(self).base__.ParentWindow)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::IUnknown>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Updates(&self) -> ::windows_core::Result<IUpdateCollection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.Updates)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IUpdateCollection>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetUpdates<'a, Param0: ::windows_core::IntoParam<'a, IUpdateCollection>>(&self, value: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetUpdates)(::windows_core::Interface::as_raw(self), value.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn BeginInstall<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::IUnknown>, Param1: ::windows_core::IntoParam<'a, ::windows_core::IUnknown>, Param2: ::windows_core::IntoParam<'a, super::Com::VARIANT>>(&self, onprogresschanged: Param0, oncompleted: Param1, state: Param2) -> ::windows_core::Result<IInstallationJob> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.BeginInstall)(::windows_core::Interface::as_raw(self), onprogresschanged.into_param().abi(), oncompleted.into_param().abi(), state.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IInstallationJob>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn BeginUninstall<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::IUnknown>, Param1: ::windows_core::IntoParam<'a, ::windows_core::IUnknown>, Param2: ::windows_core::IntoParam<'a, super::Com::VARIANT>>(&self, onprogresschanged: Param0, oncompleted: Param1, state: Param2) -> ::windows_core::Result<IInstallationJob> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.BeginUninstall)(::windows_core::Interface::as_raw(self), onprogresschanged.into_param().abi(), oncompleted.into_param().abi(), state.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IInstallationJob>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn EndInstall<'a, Param0: ::windows_core::IntoParam<'a, IInstallationJob>>(&self, value: Param0) -> ::windows_core::Result<IInstallationResult> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.EndInstall)(::windows_core::Interface::as_raw(self), value.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IInstallationResult>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn EndUninstall<'a, Param0: ::windows_core::IntoParam<'a, IInstallationJob>>(&self, value: Param0) -> ::windows_core::Result<IInstallationResult> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.EndUninstall)(::windows_core::Interface::as_raw(self), value.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IInstallationResult>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Install(&self) -> ::windows_core::Result<IInstallationResult> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.Install)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IInstallationResult>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn RunWizard<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, dialogtitle: Param0) -> ::windows_core::Result<IInstallationResult> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.RunWizard)(::windows_core::Interface::as_raw(self), dialogtitle.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IInstallationResult>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn IsBusy(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).base__.IsBusy)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Uninstall(&self) -> ::windows_core::Result<IInstallationResult> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.Uninstall)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IInstallationResult>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn AllowSourcePrompts(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).base__.AllowSourcePrompts)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn SetAllowSourcePrompts(&self, value: i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetAllowSourcePrompts)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn RebootRequiredBeforeInstallation(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).base__.RebootRequiredBeforeInstallation)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn ForceQuiet(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).ForceQuiet)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn SetForceQuiet(&self, value: i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetForceQuiet)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(value)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IUpdateInstaller2> for ::windows_core::IUnknown {
    fn from(value: IUpdateInstaller2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IUpdateInstaller2> for ::windows_core::IUnknown {
    fn from(value: &IUpdateInstaller2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IUpdateInstaller2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IUpdateInstaller2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IUpdateInstaller2> for super::Com::IDispatch {
    fn from(value: IUpdateInstaller2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IUpdateInstaller2> for super::Com::IDispatch {
    fn from(value: &IUpdateInstaller2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for IUpdateInstaller2 {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a IUpdateInstaller2 {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IUpdateInstaller2> for IUpdateInstaller {
    fn from(value: IUpdateInstaller2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IUpdateInstaller2> for IUpdateInstaller {
    fn from(value: &IUpdateInstaller2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, IUpdateInstaller> for IUpdateInstaller2 {
    fn into_param(self) -> ::windows_core::Param<'a, IUpdateInstaller> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, IUpdateInstaller> for &'a IUpdateInstaller2 {
    fn into_param(self) -> ::windows_core::Param<'a, IUpdateInstaller> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IUpdateInstaller2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IUpdateInstaller2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IUpdateInstaller2 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IUpdateInstaller2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUpdateInstaller2").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IUpdateInstaller2 {
    type Vtable = IUpdateInstaller2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3442d4fe_224d_4cee_98cf_30e0c4d229e6);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IUpdateInstaller2_Vtbl {
    pub base__: IUpdateInstaller_Vtbl,
    pub ForceQuiet: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows_core::HRESULT,
    pub SetForceQuiet: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i16) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IUpdateInstaller3(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IUpdateInstaller3 {
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ClientApplicationID(&self) -> ::windows_core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::super::Foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.ClientApplicationID)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetClientApplicationID<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, value: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.SetClientApplicationID)(::windows_core::Interface::as_raw(self), value.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn IsForced(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.IsForced)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn SetIsForced(&self, value: i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.SetIsForced)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ParentHwnd(&self) -> ::windows_core::Result<super::super::Foundation::HWND> {
        let mut result__ = ::core::mem::MaybeUninit::<super::super::Foundation::HWND>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.ParentHwnd)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::HWND>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetParentHwnd<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::HWND>>(&self, value: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.SetParentHwnd)(::windows_core::Interface::as_raw(self), value.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn SetParentWindow<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::IUnknown>>(&self, value: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.SetParentWindow)(::windows_core::Interface::as_raw(self), value.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn ParentWindow(&self) -> ::windows_core::Result<::windows_core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.ParentWindow)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::IUnknown>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Updates(&self) -> ::windows_core::Result<IUpdateCollection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.Updates)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IUpdateCollection>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetUpdates<'a, Param0: ::windows_core::IntoParam<'a, IUpdateCollection>>(&self, value: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.SetUpdates)(::windows_core::Interface::as_raw(self), value.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn BeginInstall<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::IUnknown>, Param1: ::windows_core::IntoParam<'a, ::windows_core::IUnknown>, Param2: ::windows_core::IntoParam<'a, super::Com::VARIANT>>(&self, onprogresschanged: Param0, oncompleted: Param1, state: Param2) -> ::windows_core::Result<IInstallationJob> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.BeginInstall)(::windows_core::Interface::as_raw(self), onprogresschanged.into_param().abi(), oncompleted.into_param().abi(), state.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IInstallationJob>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn BeginUninstall<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::IUnknown>, Param1: ::windows_core::IntoParam<'a, ::windows_core::IUnknown>, Param2: ::windows_core::IntoParam<'a, super::Com::VARIANT>>(&self, onprogresschanged: Param0, oncompleted: Param1, state: Param2) -> ::windows_core::Result<IInstallationJob> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.BeginUninstall)(::windows_core::Interface::as_raw(self), onprogresschanged.into_param().abi(), oncompleted.into_param().abi(), state.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IInstallationJob>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn EndInstall<'a, Param0: ::windows_core::IntoParam<'a, IInstallationJob>>(&self, value: Param0) -> ::windows_core::Result<IInstallationResult> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.EndInstall)(::windows_core::Interface::as_raw(self), value.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IInstallationResult>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn EndUninstall<'a, Param0: ::windows_core::IntoParam<'a, IInstallationJob>>(&self, value: Param0) -> ::windows_core::Result<IInstallationResult> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.EndUninstall)(::windows_core::Interface::as_raw(self), value.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IInstallationResult>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Install(&self) -> ::windows_core::Result<IInstallationResult> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.Install)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IInstallationResult>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn RunWizard<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, dialogtitle: Param0) -> ::windows_core::Result<IInstallationResult> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.RunWizard)(::windows_core::Interface::as_raw(self), dialogtitle.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IInstallationResult>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn IsBusy(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.IsBusy)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Uninstall(&self) -> ::windows_core::Result<IInstallationResult> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.Uninstall)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IInstallationResult>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn AllowSourcePrompts(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.AllowSourcePrompts)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn SetAllowSourcePrompts(&self, value: i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.SetAllowSourcePrompts)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn RebootRequiredBeforeInstallation(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.RebootRequiredBeforeInstallation)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn ForceQuiet(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).base__.ForceQuiet)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn SetForceQuiet(&self, value: i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetForceQuiet)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn AttemptCloseAppsIfNecessary(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).AttemptCloseAppsIfNecessary)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn SetAttemptCloseAppsIfNecessary(&self, value: i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetAttemptCloseAppsIfNecessary)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(value)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IUpdateInstaller3> for ::windows_core::IUnknown {
    fn from(value: IUpdateInstaller3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IUpdateInstaller3> for ::windows_core::IUnknown {
    fn from(value: &IUpdateInstaller3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IUpdateInstaller3 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IUpdateInstaller3 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IUpdateInstaller3> for super::Com::IDispatch {
    fn from(value: IUpdateInstaller3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IUpdateInstaller3> for super::Com::IDispatch {
    fn from(value: &IUpdateInstaller3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for IUpdateInstaller3 {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a IUpdateInstaller3 {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IUpdateInstaller3> for IUpdateInstaller {
    fn from(value: IUpdateInstaller3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IUpdateInstaller3> for IUpdateInstaller {
    fn from(value: &IUpdateInstaller3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, IUpdateInstaller> for IUpdateInstaller3 {
    fn into_param(self) -> ::windows_core::Param<'a, IUpdateInstaller> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, IUpdateInstaller> for &'a IUpdateInstaller3 {
    fn into_param(self) -> ::windows_core::Param<'a, IUpdateInstaller> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IUpdateInstaller3> for IUpdateInstaller2 {
    fn from(value: IUpdateInstaller3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IUpdateInstaller3> for IUpdateInstaller2 {
    fn from(value: &IUpdateInstaller3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, IUpdateInstaller2> for IUpdateInstaller3 {
    fn into_param(self) -> ::windows_core::Param<'a, IUpdateInstaller2> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, IUpdateInstaller2> for &'a IUpdateInstaller3 {
    fn into_param(self) -> ::windows_core::Param<'a, IUpdateInstaller2> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IUpdateInstaller3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IUpdateInstaller3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IUpdateInstaller3 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IUpdateInstaller3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUpdateInstaller3").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IUpdateInstaller3 {
    type Vtable = IUpdateInstaller3_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x16d11c35_099a_48d0_8338_5fae64047f8e);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IUpdateInstaller3_Vtbl {
    pub base__: IUpdateInstaller2_Vtbl,
    pub AttemptCloseAppsIfNecessary: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows_core::HRESULT,
    pub SetAttemptCloseAppsIfNecessary: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i16) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IUpdateInstaller4(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IUpdateInstaller4 {
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ClientApplicationID(&self) -> ::windows_core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::super::Foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.ClientApplicationID)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetClientApplicationID<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, value: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.base__.SetClientApplicationID)(::windows_core::Interface::as_raw(self), value.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn IsForced(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.IsForced)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn SetIsForced(&self, value: i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.base__.SetIsForced)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ParentHwnd(&self) -> ::windows_core::Result<super::super::Foundation::HWND> {
        let mut result__ = ::core::mem::MaybeUninit::<super::super::Foundation::HWND>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.ParentHwnd)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::HWND>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetParentHwnd<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::HWND>>(&self, value: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.base__.SetParentHwnd)(::windows_core::Interface::as_raw(self), value.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn SetParentWindow<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::IUnknown>>(&self, value: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.base__.SetParentWindow)(::windows_core::Interface::as_raw(self), value.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn ParentWindow(&self) -> ::windows_core::Result<::windows_core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.ParentWindow)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::IUnknown>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Updates(&self) -> ::windows_core::Result<IUpdateCollection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.Updates)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IUpdateCollection>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetUpdates<'a, Param0: ::windows_core::IntoParam<'a, IUpdateCollection>>(&self, value: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.base__.SetUpdates)(::windows_core::Interface::as_raw(self), value.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn BeginInstall<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::IUnknown>, Param1: ::windows_core::IntoParam<'a, ::windows_core::IUnknown>, Param2: ::windows_core::IntoParam<'a, super::Com::VARIANT>>(&self, onprogresschanged: Param0, oncompleted: Param1, state: Param2) -> ::windows_core::Result<IInstallationJob> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.BeginInstall)(::windows_core::Interface::as_raw(self), onprogresschanged.into_param().abi(), oncompleted.into_param().abi(), state.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IInstallationJob>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn BeginUninstall<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::IUnknown>, Param1: ::windows_core::IntoParam<'a, ::windows_core::IUnknown>, Param2: ::windows_core::IntoParam<'a, super::Com::VARIANT>>(&self, onprogresschanged: Param0, oncompleted: Param1, state: Param2) -> ::windows_core::Result<IInstallationJob> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.BeginUninstall)(::windows_core::Interface::as_raw(self), onprogresschanged.into_param().abi(), oncompleted.into_param().abi(), state.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IInstallationJob>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn EndInstall<'a, Param0: ::windows_core::IntoParam<'a, IInstallationJob>>(&self, value: Param0) -> ::windows_core::Result<IInstallationResult> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.EndInstall)(::windows_core::Interface::as_raw(self), value.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IInstallationResult>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn EndUninstall<'a, Param0: ::windows_core::IntoParam<'a, IInstallationJob>>(&self, value: Param0) -> ::windows_core::Result<IInstallationResult> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.EndUninstall)(::windows_core::Interface::as_raw(self), value.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IInstallationResult>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Install(&self) -> ::windows_core::Result<IInstallationResult> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.Install)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IInstallationResult>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn RunWizard<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, dialogtitle: Param0) -> ::windows_core::Result<IInstallationResult> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.RunWizard)(::windows_core::Interface::as_raw(self), dialogtitle.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IInstallationResult>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn IsBusy(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.IsBusy)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Uninstall(&self) -> ::windows_core::Result<IInstallationResult> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.Uninstall)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IInstallationResult>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn AllowSourcePrompts(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.AllowSourcePrompts)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn SetAllowSourcePrompts(&self, value: i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.base__.SetAllowSourcePrompts)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn RebootRequiredBeforeInstallation(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.RebootRequiredBeforeInstallation)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn ForceQuiet(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.ForceQuiet)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn SetForceQuiet(&self, value: i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.SetForceQuiet)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn AttemptCloseAppsIfNecessary(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).base__.AttemptCloseAppsIfNecessary)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn SetAttemptCloseAppsIfNecessary(&self, value: i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetAttemptCloseAppsIfNecessary)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn Commit(&self, dwflags: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Commit)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwflags)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IUpdateInstaller4> for ::windows_core::IUnknown {
    fn from(value: IUpdateInstaller4) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IUpdateInstaller4> for ::windows_core::IUnknown {
    fn from(value: &IUpdateInstaller4) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IUpdateInstaller4 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IUpdateInstaller4 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IUpdateInstaller4> for super::Com::IDispatch {
    fn from(value: IUpdateInstaller4) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IUpdateInstaller4> for super::Com::IDispatch {
    fn from(value: &IUpdateInstaller4) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for IUpdateInstaller4 {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a IUpdateInstaller4 {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IUpdateInstaller4> for IUpdateInstaller {
    fn from(value: IUpdateInstaller4) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IUpdateInstaller4> for IUpdateInstaller {
    fn from(value: &IUpdateInstaller4) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, IUpdateInstaller> for IUpdateInstaller4 {
    fn into_param(self) -> ::windows_core::Param<'a, IUpdateInstaller> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, IUpdateInstaller> for &'a IUpdateInstaller4 {
    fn into_param(self) -> ::windows_core::Param<'a, IUpdateInstaller> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IUpdateInstaller4> for IUpdateInstaller2 {
    fn from(value: IUpdateInstaller4) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IUpdateInstaller4> for IUpdateInstaller2 {
    fn from(value: &IUpdateInstaller4) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, IUpdateInstaller2> for IUpdateInstaller4 {
    fn into_param(self) -> ::windows_core::Param<'a, IUpdateInstaller2> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, IUpdateInstaller2> for &'a IUpdateInstaller4 {
    fn into_param(self) -> ::windows_core::Param<'a, IUpdateInstaller2> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IUpdateInstaller4> for IUpdateInstaller3 {
    fn from(value: IUpdateInstaller4) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IUpdateInstaller4> for IUpdateInstaller3 {
    fn from(value: &IUpdateInstaller4) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, IUpdateInstaller3> for IUpdateInstaller4 {
    fn into_param(self) -> ::windows_core::Param<'a, IUpdateInstaller3> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, IUpdateInstaller3> for &'a IUpdateInstaller4 {
    fn into_param(self) -> ::windows_core::Param<'a, IUpdateInstaller3> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IUpdateInstaller4 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IUpdateInstaller4 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IUpdateInstaller4 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IUpdateInstaller4 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUpdateInstaller4").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IUpdateInstaller4 {
    type Vtable = IUpdateInstaller4_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xef8208ea_2304_492d_9109_23813b0958e1);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IUpdateInstaller4_Vtbl {
    pub base__: IUpdateInstaller3_Vtbl,
    pub Commit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwflags: u32) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
#[repr(transparent)]
pub struct IUpdateLockdown(::windows_core::IUnknown);
impl IUpdateLockdown {
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn LockDown(&self, flags: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).LockDown)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(flags)).ok()
    }
}
impl ::core::convert::From<IUpdateLockdown> for ::windows_core::IUnknown {
    fn from(value: IUpdateLockdown) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IUpdateLockdown> for ::windows_core::IUnknown {
    fn from(value: &IUpdateLockdown) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IUpdateLockdown {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IUpdateLockdown {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IUpdateLockdown {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IUpdateLockdown {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUpdateLockdown {}
impl ::core::fmt::Debug for IUpdateLockdown {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUpdateLockdown").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IUpdateLockdown {
    type Vtable = IUpdateLockdown_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa976c28d_75a1_42aa_94ae_8af8b872089a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUpdateLockdown_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub LockDown: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, flags: i32) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IUpdateSearcher(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IUpdateSearcher {
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn CanAutomaticallyUpgradeService(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).CanAutomaticallyUpgradeService)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn SetCanAutomaticallyUpgradeService(&self, value: i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetCanAutomaticallyUpgradeService)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ClientApplicationID(&self) -> ::windows_core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::super::Foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).ClientApplicationID)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetClientApplicationID<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, value: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetClientApplicationID)(::windows_core::Interface::as_raw(self), value.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn IncludePotentiallySupersededUpdates(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).IncludePotentiallySupersededUpdates)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn SetIncludePotentiallySupersededUpdates(&self, value: i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetIncludePotentiallySupersededUpdates)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn ServerSelection(&self) -> ::windows_core::Result<ServerSelection> {
        let mut result__ = ::core::mem::MaybeUninit::<ServerSelection>::zeroed();
        (::windows_core::Interface::vtable(self).ServerSelection)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ServerSelection>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn SetServerSelection(&self, value: ServerSelection) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetServerSelection)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn BeginSearch<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::windows_core::IUnknown>, Param2: ::windows_core::IntoParam<'a, super::Com::VARIANT>>(&self, criteria: Param0, oncompleted: Param1, state: Param2) -> ::windows_core::Result<ISearchJob> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).BeginSearch)(::windows_core::Interface::as_raw(self), criteria.into_param().abi(), oncompleted.into_param().abi(), state.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISearchJob>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn EndSearch<'a, Param0: ::windows_core::IntoParam<'a, ISearchJob>>(&self, searchjob: Param0) -> ::windows_core::Result<ISearchResult> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).EndSearch)(::windows_core::Interface::as_raw(self), searchjob.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISearchResult>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EscapeString<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, unescaped: Param0) -> ::windows_core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::super::Foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).EscapeString)(::windows_core::Interface::as_raw(self), unescaped.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn QueryHistory(&self, startindex: i32, count: i32) -> ::windows_core::Result<IUpdateHistoryEntryCollection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).QueryHistory)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(startindex), ::core::mem::transmute(count), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IUpdateHistoryEntryCollection>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn Search<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, criteria: Param0) -> ::windows_core::Result<ISearchResult> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Search)(::windows_core::Interface::as_raw(self), criteria.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISearchResult>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn Online(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).Online)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn SetOnline(&self, value: i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetOnline)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn GetTotalHistoryCount(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).GetTotalHistoryCount)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ServiceID(&self) -> ::windows_core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::super::Foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).ServiceID)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetServiceID<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, value: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetServiceID)(::windows_core::Interface::as_raw(self), value.into_param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IUpdateSearcher> for ::windows_core::IUnknown {
    fn from(value: IUpdateSearcher) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IUpdateSearcher> for ::windows_core::IUnknown {
    fn from(value: &IUpdateSearcher) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IUpdateSearcher {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IUpdateSearcher {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IUpdateSearcher> for super::Com::IDispatch {
    fn from(value: IUpdateSearcher) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IUpdateSearcher> for super::Com::IDispatch {
    fn from(value: &IUpdateSearcher) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for IUpdateSearcher {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a IUpdateSearcher {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IUpdateSearcher {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IUpdateSearcher {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IUpdateSearcher {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IUpdateSearcher {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUpdateSearcher").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IUpdateSearcher {
    type Vtable = IUpdateSearcher_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8f45abf1_f9ae_4b95_a933_f0f66e5056ea);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IUpdateSearcher_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub CanAutomaticallyUpgradeService: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows_core::HRESULT,
    pub SetCanAutomaticallyUpgradeService: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i16) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub ClientApplicationID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ClientApplicationID: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetClientApplicationID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetClientApplicationID: usize,
    pub IncludePotentiallySupersededUpdates: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows_core::HRESULT,
    pub SetIncludePotentiallySupersededUpdates: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i16) -> ::windows_core::HRESULT,
    pub ServerSelection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut ServerSelection) -> ::windows_core::HRESULT,
    pub SetServerSelection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ServerSelection) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub BeginSearch: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, criteria: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, oncompleted: *mut ::core::ffi::c_void, state: ::core::mem::ManuallyDrop<super::Com::VARIANT>, retval: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    BeginSearch: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub EndSearch: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, searchjob: ::windows_core::RawPtr, retval: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    EndSearch: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub EscapeString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, unescaped: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, retval: *mut super::super::Foundation::BSTR) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    EscapeString: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub QueryHistory: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, startindex: i32, count: i32, retval: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    QueryHistory: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub Search: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, criteria: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, retval: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    Search: usize,
    pub Online: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows_core::HRESULT,
    pub SetOnline: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i16) -> ::windows_core::HRESULT,
    pub GetTotalHistoryCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub ServiceID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ServiceID: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetServiceID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetServiceID: usize,
}
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IUpdateSearcher2(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IUpdateSearcher2 {
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn CanAutomaticallyUpgradeService(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).base__.CanAutomaticallyUpgradeService)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn SetCanAutomaticallyUpgradeService(&self, value: i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetCanAutomaticallyUpgradeService)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ClientApplicationID(&self) -> ::windows_core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::super::Foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.ClientApplicationID)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetClientApplicationID<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, value: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetClientApplicationID)(::windows_core::Interface::as_raw(self), value.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn IncludePotentiallySupersededUpdates(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).base__.IncludePotentiallySupersededUpdates)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn SetIncludePotentiallySupersededUpdates(&self, value: i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetIncludePotentiallySupersededUpdates)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn ServerSelection(&self) -> ::windows_core::Result<ServerSelection> {
        let mut result__ = ::core::mem::MaybeUninit::<ServerSelection>::zeroed();
        (::windows_core::Interface::vtable(self).base__.ServerSelection)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ServerSelection>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn SetServerSelection(&self, value: ServerSelection) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetServerSelection)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn BeginSearch<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::windows_core::IUnknown>, Param2: ::windows_core::IntoParam<'a, super::Com::VARIANT>>(&self, criteria: Param0, oncompleted: Param1, state: Param2) -> ::windows_core::Result<ISearchJob> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.BeginSearch)(::windows_core::Interface::as_raw(self), criteria.into_param().abi(), oncompleted.into_param().abi(), state.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISearchJob>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn EndSearch<'a, Param0: ::windows_core::IntoParam<'a, ISearchJob>>(&self, searchjob: Param0) -> ::windows_core::Result<ISearchResult> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.EndSearch)(::windows_core::Interface::as_raw(self), searchjob.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISearchResult>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EscapeString<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, unescaped: Param0) -> ::windows_core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::super::Foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.EscapeString)(::windows_core::Interface::as_raw(self), unescaped.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn QueryHistory(&self, startindex: i32, count: i32) -> ::windows_core::Result<IUpdateHistoryEntryCollection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.QueryHistory)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(startindex), ::core::mem::transmute(count), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IUpdateHistoryEntryCollection>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn Search<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, criteria: Param0) -> ::windows_core::Result<ISearchResult> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.Search)(::windows_core::Interface::as_raw(self), criteria.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISearchResult>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn Online(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).base__.Online)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn SetOnline(&self, value: i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetOnline)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn GetTotalHistoryCount(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetTotalHistoryCount)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ServiceID(&self) -> ::windows_core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::super::Foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.ServiceID)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetServiceID<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, value: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetServiceID)(::windows_core::Interface::as_raw(self), value.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn IgnoreDownloadPriority(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).IgnoreDownloadPriority)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn SetIgnoreDownloadPriority(&self, value: i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetIgnoreDownloadPriority)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(value)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IUpdateSearcher2> for ::windows_core::IUnknown {
    fn from(value: IUpdateSearcher2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IUpdateSearcher2> for ::windows_core::IUnknown {
    fn from(value: &IUpdateSearcher2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IUpdateSearcher2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IUpdateSearcher2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IUpdateSearcher2> for super::Com::IDispatch {
    fn from(value: IUpdateSearcher2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IUpdateSearcher2> for super::Com::IDispatch {
    fn from(value: &IUpdateSearcher2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for IUpdateSearcher2 {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a IUpdateSearcher2 {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IUpdateSearcher2> for IUpdateSearcher {
    fn from(value: IUpdateSearcher2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IUpdateSearcher2> for IUpdateSearcher {
    fn from(value: &IUpdateSearcher2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, IUpdateSearcher> for IUpdateSearcher2 {
    fn into_param(self) -> ::windows_core::Param<'a, IUpdateSearcher> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, IUpdateSearcher> for &'a IUpdateSearcher2 {
    fn into_param(self) -> ::windows_core::Param<'a, IUpdateSearcher> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IUpdateSearcher2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IUpdateSearcher2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IUpdateSearcher2 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IUpdateSearcher2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUpdateSearcher2").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IUpdateSearcher2 {
    type Vtable = IUpdateSearcher2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4cbdcb2d_1589_4beb_bd1c_3e582ff0add0);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IUpdateSearcher2_Vtbl {
    pub base__: IUpdateSearcher_Vtbl,
    pub IgnoreDownloadPriority: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows_core::HRESULT,
    pub SetIgnoreDownloadPriority: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i16) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IUpdateSearcher3(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IUpdateSearcher3 {
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn CanAutomaticallyUpgradeService(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.CanAutomaticallyUpgradeService)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn SetCanAutomaticallyUpgradeService(&self, value: i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.SetCanAutomaticallyUpgradeService)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ClientApplicationID(&self) -> ::windows_core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::super::Foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.ClientApplicationID)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetClientApplicationID<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, value: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.SetClientApplicationID)(::windows_core::Interface::as_raw(self), value.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn IncludePotentiallySupersededUpdates(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.IncludePotentiallySupersededUpdates)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn SetIncludePotentiallySupersededUpdates(&self, value: i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.SetIncludePotentiallySupersededUpdates)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn ServerSelection(&self) -> ::windows_core::Result<ServerSelection> {
        let mut result__ = ::core::mem::MaybeUninit::<ServerSelection>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.ServerSelection)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ServerSelection>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn SetServerSelection(&self, value: ServerSelection) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.SetServerSelection)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn BeginSearch<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::windows_core::IUnknown>, Param2: ::windows_core::IntoParam<'a, super::Com::VARIANT>>(&self, criteria: Param0, oncompleted: Param1, state: Param2) -> ::windows_core::Result<ISearchJob> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.BeginSearch)(::windows_core::Interface::as_raw(self), criteria.into_param().abi(), oncompleted.into_param().abi(), state.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISearchJob>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn EndSearch<'a, Param0: ::windows_core::IntoParam<'a, ISearchJob>>(&self, searchjob: Param0) -> ::windows_core::Result<ISearchResult> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.EndSearch)(::windows_core::Interface::as_raw(self), searchjob.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISearchResult>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EscapeString<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, unescaped: Param0) -> ::windows_core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::super::Foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.EscapeString)(::windows_core::Interface::as_raw(self), unescaped.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn QueryHistory(&self, startindex: i32, count: i32) -> ::windows_core::Result<IUpdateHistoryEntryCollection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.QueryHistory)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(startindex), ::core::mem::transmute(count), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IUpdateHistoryEntryCollection>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn Search<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, criteria: Param0) -> ::windows_core::Result<ISearchResult> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.Search)(::windows_core::Interface::as_raw(self), criteria.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISearchResult>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn Online(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.Online)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn SetOnline(&self, value: i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.SetOnline)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn GetTotalHistoryCount(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.GetTotalHistoryCount)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ServiceID(&self) -> ::windows_core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::super::Foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.ServiceID)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetServiceID<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, value: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.SetServiceID)(::windows_core::Interface::as_raw(self), value.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn IgnoreDownloadPriority(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).base__.IgnoreDownloadPriority)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn SetIgnoreDownloadPriority(&self, value: i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetIgnoreDownloadPriority)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn SearchScope(&self) -> ::windows_core::Result<SearchScope> {
        let mut result__ = ::core::mem::MaybeUninit::<SearchScope>::zeroed();
        (::windows_core::Interface::vtable(self).SearchScope)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<SearchScope>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn SetSearchScope(&self, value: SearchScope) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetSearchScope)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(value)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IUpdateSearcher3> for ::windows_core::IUnknown {
    fn from(value: IUpdateSearcher3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IUpdateSearcher3> for ::windows_core::IUnknown {
    fn from(value: &IUpdateSearcher3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IUpdateSearcher3 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IUpdateSearcher3 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IUpdateSearcher3> for super::Com::IDispatch {
    fn from(value: IUpdateSearcher3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IUpdateSearcher3> for super::Com::IDispatch {
    fn from(value: &IUpdateSearcher3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for IUpdateSearcher3 {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a IUpdateSearcher3 {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IUpdateSearcher3> for IUpdateSearcher {
    fn from(value: IUpdateSearcher3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IUpdateSearcher3> for IUpdateSearcher {
    fn from(value: &IUpdateSearcher3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, IUpdateSearcher> for IUpdateSearcher3 {
    fn into_param(self) -> ::windows_core::Param<'a, IUpdateSearcher> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, IUpdateSearcher> for &'a IUpdateSearcher3 {
    fn into_param(self) -> ::windows_core::Param<'a, IUpdateSearcher> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IUpdateSearcher3> for IUpdateSearcher2 {
    fn from(value: IUpdateSearcher3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IUpdateSearcher3> for IUpdateSearcher2 {
    fn from(value: &IUpdateSearcher3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, IUpdateSearcher2> for IUpdateSearcher3 {
    fn into_param(self) -> ::windows_core::Param<'a, IUpdateSearcher2> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, IUpdateSearcher2> for &'a IUpdateSearcher3 {
    fn into_param(self) -> ::windows_core::Param<'a, IUpdateSearcher2> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IUpdateSearcher3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IUpdateSearcher3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IUpdateSearcher3 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IUpdateSearcher3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUpdateSearcher3").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IUpdateSearcher3 {
    type Vtable = IUpdateSearcher3_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x04c6895d_eaf2_4034_97f3_311de9be413a);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IUpdateSearcher3_Vtbl {
    pub base__: IUpdateSearcher2_Vtbl,
    pub SearchScope: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut SearchScope) -> ::windows_core::HRESULT,
    pub SetSearchScope: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: SearchScope) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IUpdateService(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IUpdateService {
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Name(&self) -> ::windows_core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::super::Foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).Name)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn ContentValidationCert(&self) -> ::windows_core::Result<super::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).ContentValidationCert)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::VARIANT>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn ExpirationDate(&self) -> ::windows_core::Result<f64> {
        let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
        (::windows_core::Interface::vtable(self).ExpirationDate)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<f64>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn IsManaged(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).IsManaged)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn IsRegisteredWithAU(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).IsRegisteredWithAU)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn IssueDate(&self) -> ::windows_core::Result<f64> {
        let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
        (::windows_core::Interface::vtable(self).IssueDate)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<f64>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn OffersWindowsUpdates(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).OffersWindowsUpdates)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn RedirectUrls(&self) -> ::windows_core::Result<IStringCollection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).RedirectUrls)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IStringCollection>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ServiceID(&self) -> ::windows_core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::super::Foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).ServiceID)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn IsScanPackageService(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).IsScanPackageService)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn CanRegisterWithAU(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).CanRegisterWithAU)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ServiceUrl(&self) -> ::windows_core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::super::Foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).ServiceUrl)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetupPrefix(&self) -> ::windows_core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::super::Foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).SetupPrefix)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IUpdateService> for ::windows_core::IUnknown {
    fn from(value: IUpdateService) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IUpdateService> for ::windows_core::IUnknown {
    fn from(value: &IUpdateService) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IUpdateService {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IUpdateService {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IUpdateService> for super::Com::IDispatch {
    fn from(value: IUpdateService) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IUpdateService> for super::Com::IDispatch {
    fn from(value: &IUpdateService) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for IUpdateService {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a IUpdateService {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IUpdateService {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IUpdateService {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IUpdateService {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IUpdateService {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUpdateService").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IUpdateService {
    type Vtable = IUpdateService_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x76b3b17e_aed6_4da5_85f0_83587f81abe3);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IUpdateService_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Name: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub ContentValidationCert: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut super::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    ContentValidationCert: usize,
    pub ExpirationDate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows_core::HRESULT,
    pub IsManaged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows_core::HRESULT,
    pub IsRegisteredWithAU: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows_core::HRESULT,
    pub IssueDate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows_core::HRESULT,
    pub OffersWindowsUpdates: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub RedirectUrls: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    RedirectUrls: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ServiceID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ServiceID: usize,
    pub IsScanPackageService: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows_core::HRESULT,
    pub CanRegisterWithAU: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub ServiceUrl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ServiceUrl: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetupPrefix: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetupPrefix: usize,
}
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IUpdateService2(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IUpdateService2 {
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Name(&self) -> ::windows_core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::super::Foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.Name)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn ContentValidationCert(&self) -> ::windows_core::Result<super::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.ContentValidationCert)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::VARIANT>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn ExpirationDate(&self) -> ::windows_core::Result<f64> {
        let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
        (::windows_core::Interface::vtable(self).base__.ExpirationDate)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<f64>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn IsManaged(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).base__.IsManaged)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn IsRegisteredWithAU(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).base__.IsRegisteredWithAU)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn IssueDate(&self) -> ::windows_core::Result<f64> {
        let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
        (::windows_core::Interface::vtable(self).base__.IssueDate)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<f64>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn OffersWindowsUpdates(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).base__.OffersWindowsUpdates)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn RedirectUrls(&self) -> ::windows_core::Result<IStringCollection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.RedirectUrls)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IStringCollection>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ServiceID(&self) -> ::windows_core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::super::Foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.ServiceID)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn IsScanPackageService(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).base__.IsScanPackageService)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn CanRegisterWithAU(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).base__.CanRegisterWithAU)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ServiceUrl(&self) -> ::windows_core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::super::Foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.ServiceUrl)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetupPrefix(&self) -> ::windows_core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::super::Foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.SetupPrefix)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn IsDefaultAUService(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).IsDefaultAUService)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IUpdateService2> for ::windows_core::IUnknown {
    fn from(value: IUpdateService2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IUpdateService2> for ::windows_core::IUnknown {
    fn from(value: &IUpdateService2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IUpdateService2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IUpdateService2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IUpdateService2> for super::Com::IDispatch {
    fn from(value: IUpdateService2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IUpdateService2> for super::Com::IDispatch {
    fn from(value: &IUpdateService2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for IUpdateService2 {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a IUpdateService2 {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IUpdateService2> for IUpdateService {
    fn from(value: IUpdateService2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IUpdateService2> for IUpdateService {
    fn from(value: &IUpdateService2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, IUpdateService> for IUpdateService2 {
    fn into_param(self) -> ::windows_core::Param<'a, IUpdateService> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, IUpdateService> for &'a IUpdateService2 {
    fn into_param(self) -> ::windows_core::Param<'a, IUpdateService> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IUpdateService2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IUpdateService2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IUpdateService2 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IUpdateService2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUpdateService2").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IUpdateService2 {
    type Vtable = IUpdateService2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1518b460_6518_4172_940f_c75883b24ceb);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IUpdateService2_Vtbl {
    pub base__: IUpdateService_Vtbl,
    pub IsDefaultAUService: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IUpdateServiceCollection(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IUpdateServiceCollection {
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn get_Item(&self, index: i32) -> ::windows_core::Result<IUpdateService> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).get_Item)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(index), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IUpdateService>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn _NewEnum(&self) -> ::windows_core::Result<::windows_core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
        (::windows_core::Interface::vtable(self)._NewEnum)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::IUnknown>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn Count(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).Count)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IUpdateServiceCollection> for ::windows_core::IUnknown {
    fn from(value: IUpdateServiceCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IUpdateServiceCollection> for ::windows_core::IUnknown {
    fn from(value: &IUpdateServiceCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IUpdateServiceCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IUpdateServiceCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IUpdateServiceCollection> for super::Com::IDispatch {
    fn from(value: IUpdateServiceCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IUpdateServiceCollection> for super::Com::IDispatch {
    fn from(value: &IUpdateServiceCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for IUpdateServiceCollection {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a IUpdateServiceCollection {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IUpdateServiceCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IUpdateServiceCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IUpdateServiceCollection {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IUpdateServiceCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUpdateServiceCollection").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IUpdateServiceCollection {
    type Vtable = IUpdateServiceCollection_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9b0353aa_0e52_44ff_b8b0_1f7fa0437f88);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IUpdateServiceCollection_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub get_Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: i32, retval: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    get_Item: usize,
    pub _NewEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IUpdateServiceManager(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IUpdateServiceManager {
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Services(&self) -> ::windows_core::Result<IUpdateServiceCollection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Services)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IUpdateServiceCollection>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn AddService<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, serviceid: Param0, authorizationcabpath: Param1) -> ::windows_core::Result<IUpdateService> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).AddService)(::windows_core::Interface::as_raw(self), serviceid.into_param().abi(), authorizationcabpath.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IUpdateService>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RegisterServiceWithAU<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, serviceid: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).RegisterServiceWithAU)(::windows_core::Interface::as_raw(self), serviceid.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RemoveService<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, serviceid: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).RemoveService)(::windows_core::Interface::as_raw(self), serviceid.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn UnregisterServiceWithAU<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, serviceid: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).UnregisterServiceWithAU)(::windows_core::Interface::as_raw(self), serviceid.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn AddScanPackageService<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, servicename: Param0, scanfilelocation: Param1, flags: i32) -> ::windows_core::Result<IUpdateService> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).AddScanPackageService)(::windows_core::Interface::as_raw(self), servicename.into_param().abi(), scanfilelocation.into_param().abi(), ::core::mem::transmute(flags), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IUpdateService>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetOption<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, super::Com::VARIANT>>(&self, optionname: Param0, optionvalue: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetOption)(::windows_core::Interface::as_raw(self), optionname.into_param().abi(), optionvalue.into_param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IUpdateServiceManager> for ::windows_core::IUnknown {
    fn from(value: IUpdateServiceManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IUpdateServiceManager> for ::windows_core::IUnknown {
    fn from(value: &IUpdateServiceManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IUpdateServiceManager {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IUpdateServiceManager {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IUpdateServiceManager> for super::Com::IDispatch {
    fn from(value: IUpdateServiceManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IUpdateServiceManager> for super::Com::IDispatch {
    fn from(value: &IUpdateServiceManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for IUpdateServiceManager {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a IUpdateServiceManager {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IUpdateServiceManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IUpdateServiceManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IUpdateServiceManager {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IUpdateServiceManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUpdateServiceManager").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IUpdateServiceManager {
    type Vtable = IUpdateServiceManager_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x23857e3c_02ba_44a3_9423_b1c900805f37);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IUpdateServiceManager_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Services: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Services: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub AddService: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, serviceid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, authorizationcabpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, retval: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    AddService: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub RegisterServiceWithAU: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, serviceid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    RegisterServiceWithAU: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub RemoveService: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, serviceid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    RemoveService: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub UnregisterServiceWithAU: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, serviceid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    UnregisterServiceWithAU: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub AddScanPackageService: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, servicename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, scanfilelocation: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, flags: i32, ppservice: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    AddScanPackageService: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SetOption: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, optionname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, optionvalue: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SetOption: usize,
}
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IUpdateServiceManager2(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IUpdateServiceManager2 {
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Services(&self) -> ::windows_core::Result<IUpdateServiceCollection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.Services)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IUpdateServiceCollection>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn AddService<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, serviceid: Param0, authorizationcabpath: Param1) -> ::windows_core::Result<IUpdateService> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.AddService)(::windows_core::Interface::as_raw(self), serviceid.into_param().abi(), authorizationcabpath.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IUpdateService>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RegisterServiceWithAU<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, serviceid: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.RegisterServiceWithAU)(::windows_core::Interface::as_raw(self), serviceid.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RemoveService<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, serviceid: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.RemoveService)(::windows_core::Interface::as_raw(self), serviceid.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn UnregisterServiceWithAU<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, serviceid: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.UnregisterServiceWithAU)(::windows_core::Interface::as_raw(self), serviceid.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn AddScanPackageService<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, servicename: Param0, scanfilelocation: Param1, flags: i32) -> ::windows_core::Result<IUpdateService> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.AddScanPackageService)(::windows_core::Interface::as_raw(self), servicename.into_param().abi(), scanfilelocation.into_param().abi(), ::core::mem::transmute(flags), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IUpdateService>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetOption<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, super::Com::VARIANT>>(&self, optionname: Param0, optionvalue: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetOption)(::windows_core::Interface::as_raw(self), optionname.into_param().abi(), optionvalue.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ClientApplicationID(&self) -> ::windows_core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::super::Foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).ClientApplicationID)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetClientApplicationID<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, value: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetClientApplicationID)(::windows_core::Interface::as_raw(self), value.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn QueryServiceRegistration<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, serviceid: Param0) -> ::windows_core::Result<IUpdateServiceRegistration> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).QueryServiceRegistration)(::windows_core::Interface::as_raw(self), serviceid.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IUpdateServiceRegistration>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn AddService2<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::BSTR>, Param2: ::windows_core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, serviceid: Param0, flags: i32, authorizationcabpath: Param2) -> ::windows_core::Result<IUpdateServiceRegistration> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).AddService2)(::windows_core::Interface::as_raw(self), serviceid.into_param().abi(), ::core::mem::transmute(flags), authorizationcabpath.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IUpdateServiceRegistration>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IUpdateServiceManager2> for ::windows_core::IUnknown {
    fn from(value: IUpdateServiceManager2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IUpdateServiceManager2> for ::windows_core::IUnknown {
    fn from(value: &IUpdateServiceManager2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IUpdateServiceManager2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IUpdateServiceManager2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IUpdateServiceManager2> for super::Com::IDispatch {
    fn from(value: IUpdateServiceManager2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IUpdateServiceManager2> for super::Com::IDispatch {
    fn from(value: &IUpdateServiceManager2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for IUpdateServiceManager2 {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a IUpdateServiceManager2 {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IUpdateServiceManager2> for IUpdateServiceManager {
    fn from(value: IUpdateServiceManager2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IUpdateServiceManager2> for IUpdateServiceManager {
    fn from(value: &IUpdateServiceManager2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, IUpdateServiceManager> for IUpdateServiceManager2 {
    fn into_param(self) -> ::windows_core::Param<'a, IUpdateServiceManager> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, IUpdateServiceManager> for &'a IUpdateServiceManager2 {
    fn into_param(self) -> ::windows_core::Param<'a, IUpdateServiceManager> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IUpdateServiceManager2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IUpdateServiceManager2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IUpdateServiceManager2 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IUpdateServiceManager2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUpdateServiceManager2").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IUpdateServiceManager2 {
    type Vtable = IUpdateServiceManager2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0bb8531d_7e8d_424f_986c_a0b8f60a3e7b);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IUpdateServiceManager2_Vtbl {
    pub base__: IUpdateServiceManager_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub ClientApplicationID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ClientApplicationID: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetClientApplicationID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetClientApplicationID: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub QueryServiceRegistration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, serviceid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, retval: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    QueryServiceRegistration: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub AddService2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, serviceid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, flags: i32, authorizationcabpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, retval: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    AddService2: usize,
}
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IUpdateServiceRegistration(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IUpdateServiceRegistration {
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn RegistrationState(&self) -> ::windows_core::Result<UpdateServiceRegistrationState> {
        let mut result__ = ::core::mem::MaybeUninit::<UpdateServiceRegistrationState>::zeroed();
        (::windows_core::Interface::vtable(self).RegistrationState)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<UpdateServiceRegistrationState>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ServiceID(&self) -> ::windows_core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::super::Foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).ServiceID)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn IsPendingRegistrationWithAU(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).IsPendingRegistrationWithAU)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Service(&self) -> ::windows_core::Result<IUpdateService2> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Service)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IUpdateService2>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IUpdateServiceRegistration> for ::windows_core::IUnknown {
    fn from(value: IUpdateServiceRegistration) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IUpdateServiceRegistration> for ::windows_core::IUnknown {
    fn from(value: &IUpdateServiceRegistration) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IUpdateServiceRegistration {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IUpdateServiceRegistration {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IUpdateServiceRegistration> for super::Com::IDispatch {
    fn from(value: IUpdateServiceRegistration) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IUpdateServiceRegistration> for super::Com::IDispatch {
    fn from(value: &IUpdateServiceRegistration) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for IUpdateServiceRegistration {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a IUpdateServiceRegistration {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IUpdateServiceRegistration {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IUpdateServiceRegistration {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IUpdateServiceRegistration {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IUpdateServiceRegistration {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUpdateServiceRegistration").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IUpdateServiceRegistration {
    type Vtable = IUpdateServiceRegistration_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xdde02280_12b3_4e0b_937b_6747f6acb286);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IUpdateServiceRegistration_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub RegistrationState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut UpdateServiceRegistrationState) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub ServiceID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ServiceID: usize,
    pub IsPendingRegistrationWithAU: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Service: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Service: usize,
}
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IUpdateSession(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IUpdateSession {
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ClientApplicationID(&self) -> ::windows_core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::super::Foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).ClientApplicationID)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetClientApplicationID<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, value: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetClientApplicationID)(::windows_core::Interface::as_raw(self), value.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn ReadOnly(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).ReadOnly)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn WebProxy(&self) -> ::windows_core::Result<IWebProxy> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).WebProxy)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWebProxy>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetWebProxy<'a, Param0: ::windows_core::IntoParam<'a, IWebProxy>>(&self, value: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetWebProxy)(::windows_core::Interface::as_raw(self), value.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateUpdateSearcher(&self) -> ::windows_core::Result<IUpdateSearcher> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).CreateUpdateSearcher)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IUpdateSearcher>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateUpdateDownloader(&self) -> ::windows_core::Result<IUpdateDownloader> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).CreateUpdateDownloader)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IUpdateDownloader>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateUpdateInstaller(&self) -> ::windows_core::Result<IUpdateInstaller> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).CreateUpdateInstaller)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IUpdateInstaller>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IUpdateSession> for ::windows_core::IUnknown {
    fn from(value: IUpdateSession) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IUpdateSession> for ::windows_core::IUnknown {
    fn from(value: &IUpdateSession) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IUpdateSession {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IUpdateSession {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IUpdateSession> for super::Com::IDispatch {
    fn from(value: IUpdateSession) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IUpdateSession> for super::Com::IDispatch {
    fn from(value: &IUpdateSession) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for IUpdateSession {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a IUpdateSession {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IUpdateSession {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IUpdateSession {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IUpdateSession {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IUpdateSession {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUpdateSession").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IUpdateSession {
    type Vtable = IUpdateSession_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x816858a4_260d_4260_933a_2585f1abc76b);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IUpdateSession_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub ClientApplicationID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ClientApplicationID: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetClientApplicationID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetClientApplicationID: usize,
    pub ReadOnly: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub WebProxy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    WebProxy: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetWebProxy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetWebProxy: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateUpdateSearcher: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateUpdateSearcher: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateUpdateDownloader: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateUpdateDownloader: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateUpdateInstaller: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateUpdateInstaller: usize,
}
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IUpdateSession2(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IUpdateSession2 {
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ClientApplicationID(&self) -> ::windows_core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::super::Foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.ClientApplicationID)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetClientApplicationID<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, value: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetClientApplicationID)(::windows_core::Interface::as_raw(self), value.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn ReadOnly(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).base__.ReadOnly)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn WebProxy(&self) -> ::windows_core::Result<IWebProxy> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.WebProxy)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWebProxy>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetWebProxy<'a, Param0: ::windows_core::IntoParam<'a, IWebProxy>>(&self, value: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetWebProxy)(::windows_core::Interface::as_raw(self), value.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateUpdateSearcher(&self) -> ::windows_core::Result<IUpdateSearcher> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.CreateUpdateSearcher)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IUpdateSearcher>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateUpdateDownloader(&self) -> ::windows_core::Result<IUpdateDownloader> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.CreateUpdateDownloader)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IUpdateDownloader>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateUpdateInstaller(&self) -> ::windows_core::Result<IUpdateInstaller> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.CreateUpdateInstaller)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IUpdateInstaller>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn UserLocale(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).UserLocale)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn SetUserLocale(&self, lcid: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetUserLocale)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lcid)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IUpdateSession2> for ::windows_core::IUnknown {
    fn from(value: IUpdateSession2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IUpdateSession2> for ::windows_core::IUnknown {
    fn from(value: &IUpdateSession2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IUpdateSession2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IUpdateSession2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IUpdateSession2> for super::Com::IDispatch {
    fn from(value: IUpdateSession2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IUpdateSession2> for super::Com::IDispatch {
    fn from(value: &IUpdateSession2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for IUpdateSession2 {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a IUpdateSession2 {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IUpdateSession2> for IUpdateSession {
    fn from(value: IUpdateSession2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IUpdateSession2> for IUpdateSession {
    fn from(value: &IUpdateSession2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, IUpdateSession> for IUpdateSession2 {
    fn into_param(self) -> ::windows_core::Param<'a, IUpdateSession> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, IUpdateSession> for &'a IUpdateSession2 {
    fn into_param(self) -> ::windows_core::Param<'a, IUpdateSession> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IUpdateSession2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IUpdateSession2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IUpdateSession2 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IUpdateSession2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUpdateSession2").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IUpdateSession2 {
    type Vtable = IUpdateSession2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x91caf7b0_eb23_49ed_9937_c52d817f46f7);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IUpdateSession2_Vtbl {
    pub base__: IUpdateSession_Vtbl,
    pub UserLocale: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut u32) -> ::windows_core::HRESULT,
    pub SetUserLocale: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lcid: u32) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IUpdateSession3(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IUpdateSession3 {
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ClientApplicationID(&self) -> ::windows_core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::super::Foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.ClientApplicationID)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetClientApplicationID<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, value: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.SetClientApplicationID)(::windows_core::Interface::as_raw(self), value.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn ReadOnly(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.ReadOnly)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn WebProxy(&self) -> ::windows_core::Result<IWebProxy> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.WebProxy)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWebProxy>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetWebProxy<'a, Param0: ::windows_core::IntoParam<'a, IWebProxy>>(&self, value: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.SetWebProxy)(::windows_core::Interface::as_raw(self), value.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateUpdateSearcher(&self) -> ::windows_core::Result<IUpdateSearcher> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.CreateUpdateSearcher)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IUpdateSearcher>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateUpdateDownloader(&self) -> ::windows_core::Result<IUpdateDownloader> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.CreateUpdateDownloader)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IUpdateDownloader>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateUpdateInstaller(&self) -> ::windows_core::Result<IUpdateInstaller> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.CreateUpdateInstaller)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IUpdateInstaller>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn UserLocale(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).base__.UserLocale)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn SetUserLocale(&self, lcid: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetUserLocale)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lcid)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateUpdateServiceManager(&self) -> ::windows_core::Result<IUpdateServiceManager2> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).CreateUpdateServiceManager)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IUpdateServiceManager2>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn QueryHistory<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, criteria: Param0, startindex: i32, count: i32) -> ::windows_core::Result<IUpdateHistoryEntryCollection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).QueryHistory)(::windows_core::Interface::as_raw(self), criteria.into_param().abi(), ::core::mem::transmute(startindex), ::core::mem::transmute(count), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IUpdateHistoryEntryCollection>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IUpdateSession3> for ::windows_core::IUnknown {
    fn from(value: IUpdateSession3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IUpdateSession3> for ::windows_core::IUnknown {
    fn from(value: &IUpdateSession3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IUpdateSession3 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IUpdateSession3 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IUpdateSession3> for super::Com::IDispatch {
    fn from(value: IUpdateSession3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IUpdateSession3> for super::Com::IDispatch {
    fn from(value: &IUpdateSession3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for IUpdateSession3 {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a IUpdateSession3 {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IUpdateSession3> for IUpdateSession {
    fn from(value: IUpdateSession3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IUpdateSession3> for IUpdateSession {
    fn from(value: &IUpdateSession3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, IUpdateSession> for IUpdateSession3 {
    fn into_param(self) -> ::windows_core::Param<'a, IUpdateSession> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, IUpdateSession> for &'a IUpdateSession3 {
    fn into_param(self) -> ::windows_core::Param<'a, IUpdateSession> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IUpdateSession3> for IUpdateSession2 {
    fn from(value: IUpdateSession3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IUpdateSession3> for IUpdateSession2 {
    fn from(value: &IUpdateSession3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, IUpdateSession2> for IUpdateSession3 {
    fn into_param(self) -> ::windows_core::Param<'a, IUpdateSession2> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, IUpdateSession2> for &'a IUpdateSession3 {
    fn into_param(self) -> ::windows_core::Param<'a, IUpdateSession2> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IUpdateSession3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IUpdateSession3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IUpdateSession3 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IUpdateSession3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUpdateSession3").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IUpdateSession3 {
    type Vtable = IUpdateSession3_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x918efd1e_b5d8_4c90_8540_aeb9bdc56f9d);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IUpdateSession3_Vtbl {
    pub base__: IUpdateSession2_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateUpdateServiceManager: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateUpdateServiceManager: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub QueryHistory: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, criteria: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, startindex: i32, count: i32, retval: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    QueryHistory: usize,
}
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IWebProxy(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IWebProxy {
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Address(&self) -> ::windows_core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::super::Foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).Address)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetAddress<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, value: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetAddress)(::windows_core::Interface::as_raw(self), value.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn BypassList(&self) -> ::windows_core::Result<IStringCollection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).BypassList)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IStringCollection>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetBypassList<'a, Param0: ::windows_core::IntoParam<'a, IStringCollection>>(&self, value: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetBypassList)(::windows_core::Interface::as_raw(self), value.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn BypassProxyOnLocal(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).BypassProxyOnLocal)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn SetBypassProxyOnLocal(&self, value: i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetBypassProxyOnLocal)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn ReadOnly(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).ReadOnly)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn UserName(&self) -> ::windows_core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::super::Foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).UserName)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetUserName<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, value: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetUserName)(::windows_core::Interface::as_raw(self), value.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetPassword<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, value: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetPassword)(::windows_core::Interface::as_raw(self), value.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn PromptForCredentials<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::IUnknown>, Param1: ::windows_core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, parentwindow: Param0, title: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).PromptForCredentials)(::windows_core::Interface::as_raw(self), parentwindow.into_param().abi(), title.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn PromptForCredentialsFromHwnd<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::HWND>, Param1: ::windows_core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, parentwindow: Param0, title: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).PromptForCredentialsFromHwnd)(::windows_core::Interface::as_raw(self), parentwindow.into_param().abi(), title.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn AutoDetect(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).AutoDetect)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn SetAutoDetect(&self, value: i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetAutoDetect)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(value)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IWebProxy> for ::windows_core::IUnknown {
    fn from(value: IWebProxy) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IWebProxy> for ::windows_core::IUnknown {
    fn from(value: &IWebProxy) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IWebProxy {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IWebProxy {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IWebProxy> for super::Com::IDispatch {
    fn from(value: IWebProxy) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IWebProxy> for super::Com::IDispatch {
    fn from(value: &IWebProxy) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for IWebProxy {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a IWebProxy {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IWebProxy {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IWebProxy {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IWebProxy {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IWebProxy {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWebProxy").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IWebProxy {
    type Vtable = IWebProxy_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x174c81fe_aecd_4dae_b8a0_2c6318dd86a8);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IWebProxy_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub Address: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Address: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetAddress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetAddress: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub BypassList: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    BypassList: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetBypassList: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetBypassList: usize,
    pub BypassProxyOnLocal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows_core::HRESULT,
    pub SetBypassProxyOnLocal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i16) -> ::windows_core::HRESULT,
    pub ReadOnly: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub UserName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    UserName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetUserName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetUserName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetPassword: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetPassword: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub PromptForCredentials: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, parentwindow: *mut ::core::ffi::c_void, title: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    PromptForCredentials: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub PromptForCredentialsFromHwnd: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, parentwindow: super::super::Foundation::HWND, title: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    PromptForCredentialsFromHwnd: usize,
    pub AutoDetect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows_core::HRESULT,
    pub SetAutoDetect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i16) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IWindowsDriverUpdate(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IWindowsDriverUpdate {
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Title(&self) -> ::windows_core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::super::Foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.Title)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn AutoSelectOnWebSites(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).base__.AutoSelectOnWebSites)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn BundledUpdates(&self) -> ::windows_core::Result<IUpdateCollection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.BundledUpdates)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IUpdateCollection>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn CanRequireSource(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).base__.CanRequireSource)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Categories(&self) -> ::windows_core::Result<ICategoryCollection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.Categories)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ICategoryCollection>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Deadline(&self) -> ::windows_core::Result<super::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.Deadline)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::VARIANT>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn DeltaCompressedContentAvailable(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).base__.DeltaCompressedContentAvailable)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn DeltaCompressedContentPreferred(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).base__.DeltaCompressedContentPreferred)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Description(&self) -> ::windows_core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::super::Foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.Description)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn EulaAccepted(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).base__.EulaAccepted)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EulaText(&self) -> ::windows_core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::super::Foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.EulaText)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn HandlerID(&self) -> ::windows_core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::super::Foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.HandlerID)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Identity(&self) -> ::windows_core::Result<IUpdateIdentity> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.Identity)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IUpdateIdentity>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Image(&self) -> ::windows_core::Result<IImageInformation> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.Image)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IImageInformation>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn InstallationBehavior(&self) -> ::windows_core::Result<IInstallationBehavior> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.InstallationBehavior)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IInstallationBehavior>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn IsBeta(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).base__.IsBeta)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn IsDownloaded(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).base__.IsDownloaded)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn IsHidden(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).base__.IsHidden)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn SetIsHidden(&self, value: i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetIsHidden)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn IsInstalled(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).base__.IsInstalled)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn IsMandatory(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).base__.IsMandatory)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn IsUninstallable(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).base__.IsUninstallable)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Languages(&self) -> ::windows_core::Result<IStringCollection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.Languages)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IStringCollection>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn LastDeploymentChangeTime(&self) -> ::windows_core::Result<f64> {
        let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
        (::windows_core::Interface::vtable(self).base__.LastDeploymentChangeTime)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<f64>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MaxDownloadSize(&self) -> ::windows_core::Result<super::super::Foundation::DECIMAL> {
        let mut result__ = ::core::mem::MaybeUninit::<super::super::Foundation::DECIMAL>::zeroed();
        (::windows_core::Interface::vtable(self).base__.MaxDownloadSize)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::DECIMAL>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MinDownloadSize(&self) -> ::windows_core::Result<super::super::Foundation::DECIMAL> {
        let mut result__ = ::core::mem::MaybeUninit::<super::super::Foundation::DECIMAL>::zeroed();
        (::windows_core::Interface::vtable(self).base__.MinDownloadSize)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::DECIMAL>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn MoreInfoUrls(&self) -> ::windows_core::Result<IStringCollection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.MoreInfoUrls)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IStringCollection>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MsrcSeverity(&self) -> ::windows_core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::super::Foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.MsrcSeverity)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn RecommendedCpuSpeed(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).base__.RecommendedCpuSpeed)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn RecommendedHardDiskSpace(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).base__.RecommendedHardDiskSpace)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn RecommendedMemory(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).base__.RecommendedMemory)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ReleaseNotes(&self) -> ::windows_core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::super::Foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.ReleaseNotes)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SecurityBulletinIDs(&self) -> ::windows_core::Result<IStringCollection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.SecurityBulletinIDs)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IStringCollection>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SupersededUpdateIDs(&self) -> ::windows_core::Result<IStringCollection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.SupersededUpdateIDs)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IStringCollection>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SupportUrl(&self) -> ::windows_core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::super::Foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.SupportUrl)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn Type(&self) -> ::windows_core::Result<UpdateType> {
        let mut result__ = ::core::mem::MaybeUninit::<UpdateType>::zeroed();
        (::windows_core::Interface::vtable(self).base__.Type)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<UpdateType>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn UninstallationNotes(&self) -> ::windows_core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::super::Foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.UninstallationNotes)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn UninstallationBehavior(&self) -> ::windows_core::Result<IInstallationBehavior> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.UninstallationBehavior)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IInstallationBehavior>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn UninstallationSteps(&self) -> ::windows_core::Result<IStringCollection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.UninstallationSteps)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IStringCollection>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn KBArticleIDs(&self) -> ::windows_core::Result<IStringCollection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.KBArticleIDs)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IStringCollection>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn AcceptEula(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.AcceptEula)(::windows_core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn DeploymentAction(&self) -> ::windows_core::Result<DeploymentAction> {
        let mut result__ = ::core::mem::MaybeUninit::<DeploymentAction>::zeroed();
        (::windows_core::Interface::vtable(self).base__.DeploymentAction)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<DeploymentAction>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CopyFromCache<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, path: Param0, toextractcabfiles: i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.CopyFromCache)(::windows_core::Interface::as_raw(self), path.into_param().abi(), ::core::mem::transmute(toextractcabfiles)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn DownloadPriority(&self) -> ::windows_core::Result<DownloadPriority> {
        let mut result__ = ::core::mem::MaybeUninit::<DownloadPriority>::zeroed();
        (::windows_core::Interface::vtable(self).base__.DownloadPriority)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<DownloadPriority>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn DownloadContents(&self) -> ::windows_core::Result<IUpdateDownloadContentCollection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.DownloadContents)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IUpdateDownloadContentCollection>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DriverClass(&self) -> ::windows_core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::super::Foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).DriverClass)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DriverHardwareID(&self) -> ::windows_core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::super::Foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).DriverHardwareID)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DriverManufacturer(&self) -> ::windows_core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::super::Foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).DriverManufacturer)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DriverModel(&self) -> ::windows_core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::super::Foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).DriverModel)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DriverProvider(&self) -> ::windows_core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::super::Foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).DriverProvider)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn DriverVerDate(&self) -> ::windows_core::Result<f64> {
        let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
        (::windows_core::Interface::vtable(self).DriverVerDate)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<f64>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn DeviceProblemNumber(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).DeviceProblemNumber)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn DeviceStatus(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).DeviceStatus)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IWindowsDriverUpdate> for ::windows_core::IUnknown {
    fn from(value: IWindowsDriverUpdate) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IWindowsDriverUpdate> for ::windows_core::IUnknown {
    fn from(value: &IWindowsDriverUpdate) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IWindowsDriverUpdate {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IWindowsDriverUpdate {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IWindowsDriverUpdate> for super::Com::IDispatch {
    fn from(value: IWindowsDriverUpdate) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IWindowsDriverUpdate> for super::Com::IDispatch {
    fn from(value: &IWindowsDriverUpdate) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for IWindowsDriverUpdate {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a IWindowsDriverUpdate {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IWindowsDriverUpdate> for IUpdate {
    fn from(value: IWindowsDriverUpdate) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IWindowsDriverUpdate> for IUpdate {
    fn from(value: &IWindowsDriverUpdate) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, IUpdate> for IWindowsDriverUpdate {
    fn into_param(self) -> ::windows_core::Param<'a, IUpdate> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, IUpdate> for &'a IWindowsDriverUpdate {
    fn into_param(self) -> ::windows_core::Param<'a, IUpdate> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IWindowsDriverUpdate {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IWindowsDriverUpdate {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IWindowsDriverUpdate {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IWindowsDriverUpdate {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWindowsDriverUpdate").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IWindowsDriverUpdate {
    type Vtable = IWindowsDriverUpdate_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb383cd1a_5ce9_4504_9f63_764b1236f191);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IWindowsDriverUpdate_Vtbl {
    pub base__: IUpdate_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub DriverClass: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DriverClass: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub DriverHardwareID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DriverHardwareID: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub DriverManufacturer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DriverManufacturer: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub DriverModel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DriverModel: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub DriverProvider: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DriverProvider: usize,
    pub DriverVerDate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows_core::HRESULT,
    pub DeviceProblemNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT,
    pub DeviceStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IWindowsDriverUpdate2(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IWindowsDriverUpdate2 {
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Title(&self) -> ::windows_core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::super::Foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.Title)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn AutoSelectOnWebSites(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.AutoSelectOnWebSites)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn BundledUpdates(&self) -> ::windows_core::Result<IUpdateCollection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.BundledUpdates)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IUpdateCollection>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn CanRequireSource(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.CanRequireSource)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Categories(&self) -> ::windows_core::Result<ICategoryCollection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.Categories)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ICategoryCollection>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Deadline(&self) -> ::windows_core::Result<super::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.Deadline)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::VARIANT>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn DeltaCompressedContentAvailable(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.DeltaCompressedContentAvailable)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn DeltaCompressedContentPreferred(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.DeltaCompressedContentPreferred)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Description(&self) -> ::windows_core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::super::Foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.Description)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn EulaAccepted(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.EulaAccepted)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EulaText(&self) -> ::windows_core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::super::Foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.EulaText)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn HandlerID(&self) -> ::windows_core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::super::Foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.HandlerID)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Identity(&self) -> ::windows_core::Result<IUpdateIdentity> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.Identity)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IUpdateIdentity>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Image(&self) -> ::windows_core::Result<IImageInformation> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.Image)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IImageInformation>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn InstallationBehavior(&self) -> ::windows_core::Result<IInstallationBehavior> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.InstallationBehavior)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IInstallationBehavior>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn IsBeta(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.IsBeta)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn IsDownloaded(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.IsDownloaded)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn IsHidden(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.IsHidden)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn SetIsHidden(&self, value: i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.SetIsHidden)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn IsInstalled(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.IsInstalled)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn IsMandatory(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.IsMandatory)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn IsUninstallable(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.IsUninstallable)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Languages(&self) -> ::windows_core::Result<IStringCollection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.Languages)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IStringCollection>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn LastDeploymentChangeTime(&self) -> ::windows_core::Result<f64> {
        let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.LastDeploymentChangeTime)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<f64>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MaxDownloadSize(&self) -> ::windows_core::Result<super::super::Foundation::DECIMAL> {
        let mut result__ = ::core::mem::MaybeUninit::<super::super::Foundation::DECIMAL>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.MaxDownloadSize)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::DECIMAL>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MinDownloadSize(&self) -> ::windows_core::Result<super::super::Foundation::DECIMAL> {
        let mut result__ = ::core::mem::MaybeUninit::<super::super::Foundation::DECIMAL>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.MinDownloadSize)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::DECIMAL>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn MoreInfoUrls(&self) -> ::windows_core::Result<IStringCollection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.MoreInfoUrls)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IStringCollection>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MsrcSeverity(&self) -> ::windows_core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::super::Foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.MsrcSeverity)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn RecommendedCpuSpeed(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.RecommendedCpuSpeed)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn RecommendedHardDiskSpace(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.RecommendedHardDiskSpace)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn RecommendedMemory(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.RecommendedMemory)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ReleaseNotes(&self) -> ::windows_core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::super::Foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.ReleaseNotes)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SecurityBulletinIDs(&self) -> ::windows_core::Result<IStringCollection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.SecurityBulletinIDs)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IStringCollection>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SupersededUpdateIDs(&self) -> ::windows_core::Result<IStringCollection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.SupersededUpdateIDs)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IStringCollection>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SupportUrl(&self) -> ::windows_core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::super::Foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.SupportUrl)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn Type(&self) -> ::windows_core::Result<UpdateType> {
        let mut result__ = ::core::mem::MaybeUninit::<UpdateType>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.Type)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<UpdateType>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn UninstallationNotes(&self) -> ::windows_core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::super::Foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.UninstallationNotes)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn UninstallationBehavior(&self) -> ::windows_core::Result<IInstallationBehavior> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.UninstallationBehavior)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IInstallationBehavior>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn UninstallationSteps(&self) -> ::windows_core::Result<IStringCollection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.UninstallationSteps)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IStringCollection>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn KBArticleIDs(&self) -> ::windows_core::Result<IStringCollection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.KBArticleIDs)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IStringCollection>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn AcceptEula(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.AcceptEula)(::windows_core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn DeploymentAction(&self) -> ::windows_core::Result<DeploymentAction> {
        let mut result__ = ::core::mem::MaybeUninit::<DeploymentAction>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.DeploymentAction)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<DeploymentAction>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CopyFromCache<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, path: Param0, toextractcabfiles: i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.CopyFromCache)(::windows_core::Interface::as_raw(self), path.into_param().abi(), ::core::mem::transmute(toextractcabfiles)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn DownloadPriority(&self) -> ::windows_core::Result<DownloadPriority> {
        let mut result__ = ::core::mem::MaybeUninit::<DownloadPriority>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.DownloadPriority)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<DownloadPriority>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn DownloadContents(&self) -> ::windows_core::Result<IUpdateDownloadContentCollection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.DownloadContents)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IUpdateDownloadContentCollection>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DriverClass(&self) -> ::windows_core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::super::Foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.DriverClass)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DriverHardwareID(&self) -> ::windows_core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::super::Foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.DriverHardwareID)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DriverManufacturer(&self) -> ::windows_core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::super::Foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.DriverManufacturer)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DriverModel(&self) -> ::windows_core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::super::Foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.DriverModel)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DriverProvider(&self) -> ::windows_core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::super::Foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.DriverProvider)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn DriverVerDate(&self) -> ::windows_core::Result<f64> {
        let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
        (::windows_core::Interface::vtable(self).base__.DriverVerDate)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<f64>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn DeviceProblemNumber(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).base__.DeviceProblemNumber)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn DeviceStatus(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).base__.DeviceStatus)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn RebootRequired(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).RebootRequired)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn IsPresent(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).IsPresent)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CveIDs(&self) -> ::windows_core::Result<IStringCollection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).CveIDs)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IStringCollection>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CopyToCache<'a, Param0: ::windows_core::IntoParam<'a, IStringCollection>>(&self, pfiles: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).CopyToCache)(::windows_core::Interface::as_raw(self), pfiles.into_param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IWindowsDriverUpdate2> for ::windows_core::IUnknown {
    fn from(value: IWindowsDriverUpdate2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IWindowsDriverUpdate2> for ::windows_core::IUnknown {
    fn from(value: &IWindowsDriverUpdate2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IWindowsDriverUpdate2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IWindowsDriverUpdate2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IWindowsDriverUpdate2> for super::Com::IDispatch {
    fn from(value: IWindowsDriverUpdate2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IWindowsDriverUpdate2> for super::Com::IDispatch {
    fn from(value: &IWindowsDriverUpdate2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for IWindowsDriverUpdate2 {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a IWindowsDriverUpdate2 {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IWindowsDriverUpdate2> for IUpdate {
    fn from(value: IWindowsDriverUpdate2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IWindowsDriverUpdate2> for IUpdate {
    fn from(value: &IWindowsDriverUpdate2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, IUpdate> for IWindowsDriverUpdate2 {
    fn into_param(self) -> ::windows_core::Param<'a, IUpdate> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, IUpdate> for &'a IWindowsDriverUpdate2 {
    fn into_param(self) -> ::windows_core::Param<'a, IUpdate> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IWindowsDriverUpdate2> for IWindowsDriverUpdate {
    fn from(value: IWindowsDriverUpdate2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IWindowsDriverUpdate2> for IWindowsDriverUpdate {
    fn from(value: &IWindowsDriverUpdate2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, IWindowsDriverUpdate> for IWindowsDriverUpdate2 {
    fn into_param(self) -> ::windows_core::Param<'a, IWindowsDriverUpdate> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, IWindowsDriverUpdate> for &'a IWindowsDriverUpdate2 {
    fn into_param(self) -> ::windows_core::Param<'a, IWindowsDriverUpdate> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IWindowsDriverUpdate2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IWindowsDriverUpdate2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IWindowsDriverUpdate2 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IWindowsDriverUpdate2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWindowsDriverUpdate2").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IWindowsDriverUpdate2 {
    type Vtable = IWindowsDriverUpdate2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x615c4269_7a48_43bd_96b7_bf6ca27d6c3e);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IWindowsDriverUpdate2_Vtbl {
    pub base__: IWindowsDriverUpdate_Vtbl,
    pub RebootRequired: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows_core::HRESULT,
    pub IsPresent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub CveIDs: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CveIDs: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CopyToCache: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfiles: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CopyToCache: usize,
}
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IWindowsDriverUpdate3(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IWindowsDriverUpdate3 {
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Title(&self) -> ::windows_core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::super::Foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.Title)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn AutoSelectOnWebSites(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.AutoSelectOnWebSites)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn BundledUpdates(&self) -> ::windows_core::Result<IUpdateCollection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.BundledUpdates)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IUpdateCollection>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn CanRequireSource(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.CanRequireSource)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Categories(&self) -> ::windows_core::Result<ICategoryCollection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.Categories)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ICategoryCollection>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Deadline(&self) -> ::windows_core::Result<super::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.Deadline)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::VARIANT>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn DeltaCompressedContentAvailable(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.DeltaCompressedContentAvailable)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn DeltaCompressedContentPreferred(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.DeltaCompressedContentPreferred)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Description(&self) -> ::windows_core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::super::Foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.Description)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn EulaAccepted(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.EulaAccepted)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EulaText(&self) -> ::windows_core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::super::Foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.EulaText)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn HandlerID(&self) -> ::windows_core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::super::Foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.HandlerID)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Identity(&self) -> ::windows_core::Result<IUpdateIdentity> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.Identity)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IUpdateIdentity>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Image(&self) -> ::windows_core::Result<IImageInformation> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.Image)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IImageInformation>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn InstallationBehavior(&self) -> ::windows_core::Result<IInstallationBehavior> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.InstallationBehavior)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IInstallationBehavior>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn IsBeta(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.IsBeta)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn IsDownloaded(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.IsDownloaded)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn IsHidden(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.IsHidden)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn SetIsHidden(&self, value: i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.base__.SetIsHidden)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn IsInstalled(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.IsInstalled)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn IsMandatory(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.IsMandatory)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn IsUninstallable(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.IsUninstallable)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Languages(&self) -> ::windows_core::Result<IStringCollection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.Languages)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IStringCollection>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn LastDeploymentChangeTime(&self) -> ::windows_core::Result<f64> {
        let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.LastDeploymentChangeTime)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<f64>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MaxDownloadSize(&self) -> ::windows_core::Result<super::super::Foundation::DECIMAL> {
        let mut result__ = ::core::mem::MaybeUninit::<super::super::Foundation::DECIMAL>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.MaxDownloadSize)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::DECIMAL>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MinDownloadSize(&self) -> ::windows_core::Result<super::super::Foundation::DECIMAL> {
        let mut result__ = ::core::mem::MaybeUninit::<super::super::Foundation::DECIMAL>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.MinDownloadSize)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::DECIMAL>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn MoreInfoUrls(&self) -> ::windows_core::Result<IStringCollection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.MoreInfoUrls)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IStringCollection>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MsrcSeverity(&self) -> ::windows_core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::super::Foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.MsrcSeverity)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn RecommendedCpuSpeed(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.RecommendedCpuSpeed)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn RecommendedHardDiskSpace(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.RecommendedHardDiskSpace)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn RecommendedMemory(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.RecommendedMemory)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ReleaseNotes(&self) -> ::windows_core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::super::Foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.ReleaseNotes)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SecurityBulletinIDs(&self) -> ::windows_core::Result<IStringCollection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.SecurityBulletinIDs)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IStringCollection>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SupersededUpdateIDs(&self) -> ::windows_core::Result<IStringCollection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.SupersededUpdateIDs)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IStringCollection>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SupportUrl(&self) -> ::windows_core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::super::Foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.SupportUrl)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn Type(&self) -> ::windows_core::Result<UpdateType> {
        let mut result__ = ::core::mem::MaybeUninit::<UpdateType>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.Type)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<UpdateType>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn UninstallationNotes(&self) -> ::windows_core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::super::Foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.UninstallationNotes)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn UninstallationBehavior(&self) -> ::windows_core::Result<IInstallationBehavior> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.UninstallationBehavior)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IInstallationBehavior>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn UninstallationSteps(&self) -> ::windows_core::Result<IStringCollection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.UninstallationSteps)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IStringCollection>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn KBArticleIDs(&self) -> ::windows_core::Result<IStringCollection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.KBArticleIDs)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IStringCollection>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn AcceptEula(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.base__.AcceptEula)(::windows_core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn DeploymentAction(&self) -> ::windows_core::Result<DeploymentAction> {
        let mut result__ = ::core::mem::MaybeUninit::<DeploymentAction>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.DeploymentAction)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<DeploymentAction>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CopyFromCache<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, path: Param0, toextractcabfiles: i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.base__.CopyFromCache)(::windows_core::Interface::as_raw(self), path.into_param().abi(), ::core::mem::transmute(toextractcabfiles)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn DownloadPriority(&self) -> ::windows_core::Result<DownloadPriority> {
        let mut result__ = ::core::mem::MaybeUninit::<DownloadPriority>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.DownloadPriority)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<DownloadPriority>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn DownloadContents(&self) -> ::windows_core::Result<IUpdateDownloadContentCollection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.DownloadContents)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IUpdateDownloadContentCollection>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DriverClass(&self) -> ::windows_core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::super::Foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.DriverClass)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DriverHardwareID(&self) -> ::windows_core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::super::Foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.DriverHardwareID)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DriverManufacturer(&self) -> ::windows_core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::super::Foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.DriverManufacturer)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DriverModel(&self) -> ::windows_core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::super::Foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.DriverModel)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DriverProvider(&self) -> ::windows_core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::super::Foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.DriverProvider)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn DriverVerDate(&self) -> ::windows_core::Result<f64> {
        let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.DriverVerDate)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<f64>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn DeviceProblemNumber(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.DeviceProblemNumber)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn DeviceStatus(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.DeviceStatus)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn RebootRequired(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).base__.RebootRequired)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn IsPresent(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).base__.IsPresent)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CveIDs(&self) -> ::windows_core::Result<IStringCollection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.CveIDs)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IStringCollection>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CopyToCache<'a, Param0: ::windows_core::IntoParam<'a, IStringCollection>>(&self, pfiles: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.CopyToCache)(::windows_core::Interface::as_raw(self), pfiles.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn BrowseOnly(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).BrowseOnly)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IWindowsDriverUpdate3> for ::windows_core::IUnknown {
    fn from(value: IWindowsDriverUpdate3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IWindowsDriverUpdate3> for ::windows_core::IUnknown {
    fn from(value: &IWindowsDriverUpdate3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IWindowsDriverUpdate3 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IWindowsDriverUpdate3 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IWindowsDriverUpdate3> for super::Com::IDispatch {
    fn from(value: IWindowsDriverUpdate3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IWindowsDriverUpdate3> for super::Com::IDispatch {
    fn from(value: &IWindowsDriverUpdate3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for IWindowsDriverUpdate3 {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a IWindowsDriverUpdate3 {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IWindowsDriverUpdate3> for IUpdate {
    fn from(value: IWindowsDriverUpdate3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IWindowsDriverUpdate3> for IUpdate {
    fn from(value: &IWindowsDriverUpdate3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, IUpdate> for IWindowsDriverUpdate3 {
    fn into_param(self) -> ::windows_core::Param<'a, IUpdate> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, IUpdate> for &'a IWindowsDriverUpdate3 {
    fn into_param(self) -> ::windows_core::Param<'a, IUpdate> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IWindowsDriverUpdate3> for IWindowsDriverUpdate {
    fn from(value: IWindowsDriverUpdate3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IWindowsDriverUpdate3> for IWindowsDriverUpdate {
    fn from(value: &IWindowsDriverUpdate3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, IWindowsDriverUpdate> for IWindowsDriverUpdate3 {
    fn into_param(self) -> ::windows_core::Param<'a, IWindowsDriverUpdate> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, IWindowsDriverUpdate> for &'a IWindowsDriverUpdate3 {
    fn into_param(self) -> ::windows_core::Param<'a, IWindowsDriverUpdate> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IWindowsDriverUpdate3> for IWindowsDriverUpdate2 {
    fn from(value: IWindowsDriverUpdate3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IWindowsDriverUpdate3> for IWindowsDriverUpdate2 {
    fn from(value: &IWindowsDriverUpdate3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, IWindowsDriverUpdate2> for IWindowsDriverUpdate3 {
    fn into_param(self) -> ::windows_core::Param<'a, IWindowsDriverUpdate2> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, IWindowsDriverUpdate2> for &'a IWindowsDriverUpdate3 {
    fn into_param(self) -> ::windows_core::Param<'a, IWindowsDriverUpdate2> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IWindowsDriverUpdate3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IWindowsDriverUpdate3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IWindowsDriverUpdate3 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IWindowsDriverUpdate3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWindowsDriverUpdate3").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IWindowsDriverUpdate3 {
    type Vtable = IWindowsDriverUpdate3_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x49ebd502_4a96_41bd_9e3e_4c5057f4250c);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IWindowsDriverUpdate3_Vtbl {
    pub base__: IWindowsDriverUpdate2_Vtbl,
    pub BrowseOnly: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IWindowsDriverUpdate4(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IWindowsDriverUpdate4 {
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Title(&self) -> ::windows_core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::super::Foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.Title)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn AutoSelectOnWebSites(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.AutoSelectOnWebSites)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn BundledUpdates(&self) -> ::windows_core::Result<IUpdateCollection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.BundledUpdates)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IUpdateCollection>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn CanRequireSource(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.CanRequireSource)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Categories(&self) -> ::windows_core::Result<ICategoryCollection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.Categories)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ICategoryCollection>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Deadline(&self) -> ::windows_core::Result<super::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.Deadline)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::VARIANT>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn DeltaCompressedContentAvailable(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.DeltaCompressedContentAvailable)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn DeltaCompressedContentPreferred(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.DeltaCompressedContentPreferred)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Description(&self) -> ::windows_core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::super::Foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.Description)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn EulaAccepted(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.EulaAccepted)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EulaText(&self) -> ::windows_core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::super::Foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.EulaText)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn HandlerID(&self) -> ::windows_core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::super::Foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.HandlerID)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Identity(&self) -> ::windows_core::Result<IUpdateIdentity> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.Identity)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IUpdateIdentity>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Image(&self) -> ::windows_core::Result<IImageInformation> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.Image)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IImageInformation>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn InstallationBehavior(&self) -> ::windows_core::Result<IInstallationBehavior> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.InstallationBehavior)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IInstallationBehavior>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn IsBeta(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.IsBeta)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn IsDownloaded(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.IsDownloaded)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn IsHidden(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.IsHidden)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn SetIsHidden(&self, value: i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.SetIsHidden)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn IsInstalled(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.IsInstalled)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn IsMandatory(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.IsMandatory)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn IsUninstallable(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.IsUninstallable)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Languages(&self) -> ::windows_core::Result<IStringCollection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.Languages)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IStringCollection>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn LastDeploymentChangeTime(&self) -> ::windows_core::Result<f64> {
        let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.LastDeploymentChangeTime)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<f64>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MaxDownloadSize(&self) -> ::windows_core::Result<super::super::Foundation::DECIMAL> {
        let mut result__ = ::core::mem::MaybeUninit::<super::super::Foundation::DECIMAL>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.MaxDownloadSize)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::DECIMAL>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MinDownloadSize(&self) -> ::windows_core::Result<super::super::Foundation::DECIMAL> {
        let mut result__ = ::core::mem::MaybeUninit::<super::super::Foundation::DECIMAL>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.MinDownloadSize)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::DECIMAL>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn MoreInfoUrls(&self) -> ::windows_core::Result<IStringCollection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.MoreInfoUrls)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IStringCollection>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MsrcSeverity(&self) -> ::windows_core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::super::Foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.MsrcSeverity)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn RecommendedCpuSpeed(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.RecommendedCpuSpeed)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn RecommendedHardDiskSpace(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.RecommendedHardDiskSpace)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn RecommendedMemory(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.RecommendedMemory)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ReleaseNotes(&self) -> ::windows_core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::super::Foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.ReleaseNotes)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SecurityBulletinIDs(&self) -> ::windows_core::Result<IStringCollection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.SecurityBulletinIDs)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IStringCollection>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SupersededUpdateIDs(&self) -> ::windows_core::Result<IStringCollection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.SupersededUpdateIDs)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IStringCollection>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SupportUrl(&self) -> ::windows_core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::super::Foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.SupportUrl)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn Type(&self) -> ::windows_core::Result<UpdateType> {
        let mut result__ = ::core::mem::MaybeUninit::<UpdateType>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.Type)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<UpdateType>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn UninstallationNotes(&self) -> ::windows_core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::super::Foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.UninstallationNotes)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn UninstallationBehavior(&self) -> ::windows_core::Result<IInstallationBehavior> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.UninstallationBehavior)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IInstallationBehavior>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn UninstallationSteps(&self) -> ::windows_core::Result<IStringCollection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.UninstallationSteps)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IStringCollection>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn KBArticleIDs(&self) -> ::windows_core::Result<IStringCollection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.KBArticleIDs)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IStringCollection>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn AcceptEula(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.AcceptEula)(::windows_core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn DeploymentAction(&self) -> ::windows_core::Result<DeploymentAction> {
        let mut result__ = ::core::mem::MaybeUninit::<DeploymentAction>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.DeploymentAction)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<DeploymentAction>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CopyFromCache<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, path: Param0, toextractcabfiles: i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.CopyFromCache)(::windows_core::Interface::as_raw(self), path.into_param().abi(), ::core::mem::transmute(toextractcabfiles)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn DownloadPriority(&self) -> ::windows_core::Result<DownloadPriority> {
        let mut result__ = ::core::mem::MaybeUninit::<DownloadPriority>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.DownloadPriority)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<DownloadPriority>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn DownloadContents(&self) -> ::windows_core::Result<IUpdateDownloadContentCollection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.DownloadContents)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IUpdateDownloadContentCollection>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DriverClass(&self) -> ::windows_core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::super::Foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.DriverClass)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DriverHardwareID(&self) -> ::windows_core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::super::Foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.DriverHardwareID)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DriverManufacturer(&self) -> ::windows_core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::super::Foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.DriverManufacturer)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DriverModel(&self) -> ::windows_core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::super::Foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.DriverModel)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DriverProvider(&self) -> ::windows_core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::super::Foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.DriverProvider)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn DriverVerDate(&self) -> ::windows_core::Result<f64> {
        let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.DriverVerDate)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<f64>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn DeviceProblemNumber(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.DeviceProblemNumber)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn DeviceStatus(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.DeviceStatus)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn RebootRequired(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.RebootRequired)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn IsPresent(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.IsPresent)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CveIDs(&self) -> ::windows_core::Result<IStringCollection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.CveIDs)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IStringCollection>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CopyToCache<'a, Param0: ::windows_core::IntoParam<'a, IStringCollection>>(&self, pfiles: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.CopyToCache)(::windows_core::Interface::as_raw(self), pfiles.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn BrowseOnly(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).base__.BrowseOnly)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn WindowsDriverUpdateEntries(&self) -> ::windows_core::Result<IWindowsDriverUpdateEntryCollection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).WindowsDriverUpdateEntries)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWindowsDriverUpdateEntryCollection>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn PerUser(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).PerUser)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IWindowsDriverUpdate4> for ::windows_core::IUnknown {
    fn from(value: IWindowsDriverUpdate4) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IWindowsDriverUpdate4> for ::windows_core::IUnknown {
    fn from(value: &IWindowsDriverUpdate4) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IWindowsDriverUpdate4 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IWindowsDriverUpdate4 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IWindowsDriverUpdate4> for super::Com::IDispatch {
    fn from(value: IWindowsDriverUpdate4) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IWindowsDriverUpdate4> for super::Com::IDispatch {
    fn from(value: &IWindowsDriverUpdate4) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for IWindowsDriverUpdate4 {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a IWindowsDriverUpdate4 {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IWindowsDriverUpdate4> for IUpdate {
    fn from(value: IWindowsDriverUpdate4) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IWindowsDriverUpdate4> for IUpdate {
    fn from(value: &IWindowsDriverUpdate4) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, IUpdate> for IWindowsDriverUpdate4 {
    fn into_param(self) -> ::windows_core::Param<'a, IUpdate> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, IUpdate> for &'a IWindowsDriverUpdate4 {
    fn into_param(self) -> ::windows_core::Param<'a, IUpdate> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IWindowsDriverUpdate4> for IWindowsDriverUpdate {
    fn from(value: IWindowsDriverUpdate4) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IWindowsDriverUpdate4> for IWindowsDriverUpdate {
    fn from(value: &IWindowsDriverUpdate4) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, IWindowsDriverUpdate> for IWindowsDriverUpdate4 {
    fn into_param(self) -> ::windows_core::Param<'a, IWindowsDriverUpdate> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, IWindowsDriverUpdate> for &'a IWindowsDriverUpdate4 {
    fn into_param(self) -> ::windows_core::Param<'a, IWindowsDriverUpdate> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IWindowsDriverUpdate4> for IWindowsDriverUpdate2 {
    fn from(value: IWindowsDriverUpdate4) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IWindowsDriverUpdate4> for IWindowsDriverUpdate2 {
    fn from(value: &IWindowsDriverUpdate4) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, IWindowsDriverUpdate2> for IWindowsDriverUpdate4 {
    fn into_param(self) -> ::windows_core::Param<'a, IWindowsDriverUpdate2> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, IWindowsDriverUpdate2> for &'a IWindowsDriverUpdate4 {
    fn into_param(self) -> ::windows_core::Param<'a, IWindowsDriverUpdate2> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IWindowsDriverUpdate4> for IWindowsDriverUpdate3 {
    fn from(value: IWindowsDriverUpdate4) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IWindowsDriverUpdate4> for IWindowsDriverUpdate3 {
    fn from(value: &IWindowsDriverUpdate4) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, IWindowsDriverUpdate3> for IWindowsDriverUpdate4 {
    fn into_param(self) -> ::windows_core::Param<'a, IWindowsDriverUpdate3> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, IWindowsDriverUpdate3> for &'a IWindowsDriverUpdate4 {
    fn into_param(self) -> ::windows_core::Param<'a, IWindowsDriverUpdate3> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IWindowsDriverUpdate4 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IWindowsDriverUpdate4 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IWindowsDriverUpdate4 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IWindowsDriverUpdate4 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWindowsDriverUpdate4").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IWindowsDriverUpdate4 {
    type Vtable = IWindowsDriverUpdate4_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x004c6a2b_0c19_4c69_9f5c_a269b2560db9);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IWindowsDriverUpdate4_Vtbl {
    pub base__: IWindowsDriverUpdate3_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub WindowsDriverUpdateEntries: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    WindowsDriverUpdateEntries: usize,
    pub PerUser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i16) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IWindowsDriverUpdate5(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IWindowsDriverUpdate5 {
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Title(&self) -> ::windows_core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::super::Foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.base__.Title)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn AutoSelectOnWebSites(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.base__.AutoSelectOnWebSites)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn BundledUpdates(&self) -> ::windows_core::Result<IUpdateCollection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.base__.BundledUpdates)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IUpdateCollection>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn CanRequireSource(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.base__.CanRequireSource)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Categories(&self) -> ::windows_core::Result<ICategoryCollection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.base__.Categories)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ICategoryCollection>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Deadline(&self) -> ::windows_core::Result<super::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.base__.Deadline)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::VARIANT>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn DeltaCompressedContentAvailable(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.base__.DeltaCompressedContentAvailable)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn DeltaCompressedContentPreferred(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.base__.DeltaCompressedContentPreferred)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Description(&self) -> ::windows_core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::super::Foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.base__.Description)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn EulaAccepted(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.base__.EulaAccepted)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EulaText(&self) -> ::windows_core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::super::Foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.base__.EulaText)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn HandlerID(&self) -> ::windows_core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::super::Foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.base__.HandlerID)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Identity(&self) -> ::windows_core::Result<IUpdateIdentity> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.base__.Identity)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IUpdateIdentity>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Image(&self) -> ::windows_core::Result<IImageInformation> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.base__.Image)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IImageInformation>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn InstallationBehavior(&self) -> ::windows_core::Result<IInstallationBehavior> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.base__.InstallationBehavior)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IInstallationBehavior>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn IsBeta(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.base__.IsBeta)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn IsDownloaded(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.base__.IsDownloaded)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn IsHidden(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.base__.IsHidden)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn SetIsHidden(&self, value: i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.base__.SetIsHidden)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn IsInstalled(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.base__.IsInstalled)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn IsMandatory(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.base__.IsMandatory)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn IsUninstallable(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.base__.IsUninstallable)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Languages(&self) -> ::windows_core::Result<IStringCollection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.base__.Languages)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IStringCollection>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn LastDeploymentChangeTime(&self) -> ::windows_core::Result<f64> {
        let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.base__.LastDeploymentChangeTime)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<f64>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MaxDownloadSize(&self) -> ::windows_core::Result<super::super::Foundation::DECIMAL> {
        let mut result__ = ::core::mem::MaybeUninit::<super::super::Foundation::DECIMAL>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.base__.MaxDownloadSize)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::DECIMAL>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MinDownloadSize(&self) -> ::windows_core::Result<super::super::Foundation::DECIMAL> {
        let mut result__ = ::core::mem::MaybeUninit::<super::super::Foundation::DECIMAL>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.base__.MinDownloadSize)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::DECIMAL>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn MoreInfoUrls(&self) -> ::windows_core::Result<IStringCollection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.base__.MoreInfoUrls)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IStringCollection>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MsrcSeverity(&self) -> ::windows_core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::super::Foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.base__.MsrcSeverity)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn RecommendedCpuSpeed(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.base__.RecommendedCpuSpeed)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn RecommendedHardDiskSpace(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.base__.RecommendedHardDiskSpace)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn RecommendedMemory(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.base__.RecommendedMemory)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ReleaseNotes(&self) -> ::windows_core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::super::Foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.base__.ReleaseNotes)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SecurityBulletinIDs(&self) -> ::windows_core::Result<IStringCollection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.base__.SecurityBulletinIDs)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IStringCollection>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SupersededUpdateIDs(&self) -> ::windows_core::Result<IStringCollection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.base__.SupersededUpdateIDs)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IStringCollection>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SupportUrl(&self) -> ::windows_core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::super::Foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.base__.SupportUrl)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn Type(&self) -> ::windows_core::Result<UpdateType> {
        let mut result__ = ::core::mem::MaybeUninit::<UpdateType>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.base__.Type)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<UpdateType>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn UninstallationNotes(&self) -> ::windows_core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::super::Foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.base__.UninstallationNotes)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn UninstallationBehavior(&self) -> ::windows_core::Result<IInstallationBehavior> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.base__.UninstallationBehavior)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IInstallationBehavior>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn UninstallationSteps(&self) -> ::windows_core::Result<IStringCollection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.base__.UninstallationSteps)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IStringCollection>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn KBArticleIDs(&self) -> ::windows_core::Result<IStringCollection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.base__.KBArticleIDs)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IStringCollection>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn AcceptEula(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.base__.AcceptEula)(::windows_core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn DeploymentAction(&self) -> ::windows_core::Result<DeploymentAction> {
        let mut result__ = ::core::mem::MaybeUninit::<DeploymentAction>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.base__.DeploymentAction)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<DeploymentAction>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CopyFromCache<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, path: Param0, toextractcabfiles: i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.base__.CopyFromCache)(::windows_core::Interface::as_raw(self), path.into_param().abi(), ::core::mem::transmute(toextractcabfiles)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn DownloadPriority(&self) -> ::windows_core::Result<DownloadPriority> {
        let mut result__ = ::core::mem::MaybeUninit::<DownloadPriority>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.base__.DownloadPriority)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<DownloadPriority>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn DownloadContents(&self) -> ::windows_core::Result<IUpdateDownloadContentCollection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.base__.DownloadContents)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IUpdateDownloadContentCollection>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DriverClass(&self) -> ::windows_core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::super::Foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.DriverClass)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DriverHardwareID(&self) -> ::windows_core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::super::Foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.DriverHardwareID)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DriverManufacturer(&self) -> ::windows_core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::super::Foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.DriverManufacturer)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DriverModel(&self) -> ::windows_core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::super::Foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.DriverModel)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DriverProvider(&self) -> ::windows_core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::super::Foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.DriverProvider)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn DriverVerDate(&self) -> ::windows_core::Result<f64> {
        let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.DriverVerDate)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<f64>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn DeviceProblemNumber(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.DeviceProblemNumber)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn DeviceStatus(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.base__.DeviceStatus)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn RebootRequired(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.RebootRequired)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn IsPresent(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.IsPresent)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CveIDs(&self) -> ::windows_core::Result<IStringCollection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.CveIDs)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IStringCollection>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CopyToCache<'a, Param0: ::windows_core::IntoParam<'a, IStringCollection>>(&self, pfiles: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.base__.CopyToCache)(::windows_core::Interface::as_raw(self), pfiles.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn BrowseOnly(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.BrowseOnly)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn WindowsDriverUpdateEntries(&self) -> ::windows_core::Result<IWindowsDriverUpdateEntryCollection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.WindowsDriverUpdateEntries)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWindowsDriverUpdateEntryCollection>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn PerUser(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).base__.PerUser)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn AutoSelection(&self) -> ::windows_core::Result<AutoSelectionMode> {
        let mut result__ = ::core::mem::MaybeUninit::<AutoSelectionMode>::zeroed();
        (::windows_core::Interface::vtable(self).AutoSelection)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<AutoSelectionMode>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn AutoDownload(&self) -> ::windows_core::Result<AutoDownloadMode> {
        let mut result__ = ::core::mem::MaybeUninit::<AutoDownloadMode>::zeroed();
        (::windows_core::Interface::vtable(self).AutoDownload)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<AutoDownloadMode>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IWindowsDriverUpdate5> for ::windows_core::IUnknown {
    fn from(value: IWindowsDriverUpdate5) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IWindowsDriverUpdate5> for ::windows_core::IUnknown {
    fn from(value: &IWindowsDriverUpdate5) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IWindowsDriverUpdate5 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IWindowsDriverUpdate5 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IWindowsDriverUpdate5> for super::Com::IDispatch {
    fn from(value: IWindowsDriverUpdate5) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IWindowsDriverUpdate5> for super::Com::IDispatch {
    fn from(value: &IWindowsDriverUpdate5) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for IWindowsDriverUpdate5 {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a IWindowsDriverUpdate5 {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IWindowsDriverUpdate5> for IUpdate {
    fn from(value: IWindowsDriverUpdate5) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IWindowsDriverUpdate5> for IUpdate {
    fn from(value: &IWindowsDriverUpdate5) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, IUpdate> for IWindowsDriverUpdate5 {
    fn into_param(self) -> ::windows_core::Param<'a, IUpdate> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, IUpdate> for &'a IWindowsDriverUpdate5 {
    fn into_param(self) -> ::windows_core::Param<'a, IUpdate> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IWindowsDriverUpdate5> for IWindowsDriverUpdate {
    fn from(value: IWindowsDriverUpdate5) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IWindowsDriverUpdate5> for IWindowsDriverUpdate {
    fn from(value: &IWindowsDriverUpdate5) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, IWindowsDriverUpdate> for IWindowsDriverUpdate5 {
    fn into_param(self) -> ::windows_core::Param<'a, IWindowsDriverUpdate> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, IWindowsDriverUpdate> for &'a IWindowsDriverUpdate5 {
    fn into_param(self) -> ::windows_core::Param<'a, IWindowsDriverUpdate> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IWindowsDriverUpdate5> for IWindowsDriverUpdate2 {
    fn from(value: IWindowsDriverUpdate5) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IWindowsDriverUpdate5> for IWindowsDriverUpdate2 {
    fn from(value: &IWindowsDriverUpdate5) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, IWindowsDriverUpdate2> for IWindowsDriverUpdate5 {
    fn into_param(self) -> ::windows_core::Param<'a, IWindowsDriverUpdate2> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, IWindowsDriverUpdate2> for &'a IWindowsDriverUpdate5 {
    fn into_param(self) -> ::windows_core::Param<'a, IWindowsDriverUpdate2> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IWindowsDriverUpdate5> for IWindowsDriverUpdate3 {
    fn from(value: IWindowsDriverUpdate5) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IWindowsDriverUpdate5> for IWindowsDriverUpdate3 {
    fn from(value: &IWindowsDriverUpdate5) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, IWindowsDriverUpdate3> for IWindowsDriverUpdate5 {
    fn into_param(self) -> ::windows_core::Param<'a, IWindowsDriverUpdate3> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, IWindowsDriverUpdate3> for &'a IWindowsDriverUpdate5 {
    fn into_param(self) -> ::windows_core::Param<'a, IWindowsDriverUpdate3> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IWindowsDriverUpdate5> for IWindowsDriverUpdate4 {
    fn from(value: IWindowsDriverUpdate5) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IWindowsDriverUpdate5> for IWindowsDriverUpdate4 {
    fn from(value: &IWindowsDriverUpdate5) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, IWindowsDriverUpdate4> for IWindowsDriverUpdate5 {
    fn into_param(self) -> ::windows_core::Param<'a, IWindowsDriverUpdate4> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, IWindowsDriverUpdate4> for &'a IWindowsDriverUpdate5 {
    fn into_param(self) -> ::windows_core::Param<'a, IWindowsDriverUpdate4> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IWindowsDriverUpdate5 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IWindowsDriverUpdate5 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IWindowsDriverUpdate5 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IWindowsDriverUpdate5 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWindowsDriverUpdate5").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IWindowsDriverUpdate5 {
    type Vtable = IWindowsDriverUpdate5_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x70cf5c82_8642_42bb_9dbc_0cfd263c6c4f);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IWindowsDriverUpdate5_Vtbl {
    pub base__: IWindowsDriverUpdate4_Vtbl,
    pub AutoSelection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut AutoSelectionMode) -> ::windows_core::HRESULT,
    pub AutoDownload: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut AutoDownloadMode) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IWindowsDriverUpdateEntry(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IWindowsDriverUpdateEntry {
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DriverClass(&self) -> ::windows_core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::super::Foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).DriverClass)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DriverHardwareID(&self) -> ::windows_core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::super::Foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).DriverHardwareID)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DriverManufacturer(&self) -> ::windows_core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::super::Foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).DriverManufacturer)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DriverModel(&self) -> ::windows_core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::super::Foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).DriverModel)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DriverProvider(&self) -> ::windows_core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::super::Foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).DriverProvider)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn DriverVerDate(&self) -> ::windows_core::Result<f64> {
        let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
        (::windows_core::Interface::vtable(self).DriverVerDate)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<f64>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn DeviceProblemNumber(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).DeviceProblemNumber)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn DeviceStatus(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).DeviceStatus)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IWindowsDriverUpdateEntry> for ::windows_core::IUnknown {
    fn from(value: IWindowsDriverUpdateEntry) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IWindowsDriverUpdateEntry> for ::windows_core::IUnknown {
    fn from(value: &IWindowsDriverUpdateEntry) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IWindowsDriverUpdateEntry {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IWindowsDriverUpdateEntry {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IWindowsDriverUpdateEntry> for super::Com::IDispatch {
    fn from(value: IWindowsDriverUpdateEntry) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IWindowsDriverUpdateEntry> for super::Com::IDispatch {
    fn from(value: &IWindowsDriverUpdateEntry) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for IWindowsDriverUpdateEntry {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a IWindowsDriverUpdateEntry {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IWindowsDriverUpdateEntry {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IWindowsDriverUpdateEntry {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IWindowsDriverUpdateEntry {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IWindowsDriverUpdateEntry {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWindowsDriverUpdateEntry").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IWindowsDriverUpdateEntry {
    type Vtable = IWindowsDriverUpdateEntry_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xed8bfe40_a60b_42ea_9652_817dfcfa23ec);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IWindowsDriverUpdateEntry_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub DriverClass: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DriverClass: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub DriverHardwareID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DriverHardwareID: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub DriverManufacturer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DriverManufacturer: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub DriverModel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DriverModel: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub DriverProvider: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DriverProvider: usize,
    pub DriverVerDate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows_core::HRESULT,
    pub DeviceProblemNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT,
    pub DeviceStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IWindowsDriverUpdateEntryCollection(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IWindowsDriverUpdateEntryCollection {
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn get_Item(&self, index: i32) -> ::windows_core::Result<IWindowsDriverUpdateEntry> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).get_Item)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(index), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWindowsDriverUpdateEntry>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn _NewEnum(&self) -> ::windows_core::Result<::windows_core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
        (::windows_core::Interface::vtable(self)._NewEnum)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::IUnknown>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
    pub unsafe fn Count(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).Count)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IWindowsDriverUpdateEntryCollection> for ::windows_core::IUnknown {
    fn from(value: IWindowsDriverUpdateEntryCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IWindowsDriverUpdateEntryCollection> for ::windows_core::IUnknown {
    fn from(value: &IWindowsDriverUpdateEntryCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IWindowsDriverUpdateEntryCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IWindowsDriverUpdateEntryCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IWindowsDriverUpdateEntryCollection> for super::Com::IDispatch {
    fn from(value: IWindowsDriverUpdateEntryCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IWindowsDriverUpdateEntryCollection> for super::Com::IDispatch {
    fn from(value: &IWindowsDriverUpdateEntryCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for IWindowsDriverUpdateEntryCollection {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a IWindowsDriverUpdateEntryCollection {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IWindowsDriverUpdateEntryCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IWindowsDriverUpdateEntryCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IWindowsDriverUpdateEntryCollection {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IWindowsDriverUpdateEntryCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWindowsDriverUpdateEntryCollection").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IWindowsDriverUpdateEntryCollection {
    type Vtable = IWindowsDriverUpdateEntryCollection_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0d521700_a372_4bef_828b_3d00c10adebd);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IWindowsDriverUpdateEntryCollection_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub get_Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: i32, retval: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    get_Item: usize,
    pub _NewEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IWindowsUpdateAgentInfo(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IWindowsUpdateAgentInfo {
    #[doc = "*Required features: `\"Win32_System_UpdateAgent\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetInfo<'a, Param0: ::windows_core::IntoParam<'a, super::Com::VARIANT>>(&self, varinfoidentifier: Param0) -> ::windows_core::Result<super::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).GetInfo)(::windows_core::Interface::as_raw(self), varinfoidentifier.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::VARIANT>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IWindowsUpdateAgentInfo> for ::windows_core::IUnknown {
    fn from(value: IWindowsUpdateAgentInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IWindowsUpdateAgentInfo> for ::windows_core::IUnknown {
    fn from(value: &IWindowsUpdateAgentInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IWindowsUpdateAgentInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IWindowsUpdateAgentInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IWindowsUpdateAgentInfo> for super::Com::IDispatch {
    fn from(value: IWindowsUpdateAgentInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IWindowsUpdateAgentInfo> for super::Com::IDispatch {
    fn from(value: &IWindowsUpdateAgentInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for IWindowsUpdateAgentInfo {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a IWindowsUpdateAgentInfo {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IWindowsUpdateAgentInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IWindowsUpdateAgentInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IWindowsUpdateAgentInfo {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IWindowsUpdateAgentInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWindowsUpdateAgentInfo").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IWindowsUpdateAgentInfo {
    type Vtable = IWindowsUpdateAgentInfo_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x85713fa1_7796_4fa2_be3b_e2d6124dd373);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IWindowsUpdateAgentInfo_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub GetInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, varinfoidentifier: ::core::mem::ManuallyDrop<super::Com::VARIANT>, retval: *mut super::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    GetInfo: usize,
}
pub const InstallationAgent: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x317e92fc_1679_46fd_a0b5_f08914dd8623);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct InstallationImpact(pub i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const iiNormal: InstallationImpact = InstallationImpact(0i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const iiMinor: InstallationImpact = InstallationImpact(1i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const iiRequiresExclusiveHandling: InstallationImpact = InstallationImpact(2i32);
impl ::core::marker::Copy for InstallationImpact {}
impl ::core::clone::Clone for InstallationImpact {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for InstallationImpact {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for InstallationImpact {
    type Abi = Self;
}
impl ::core::fmt::Debug for InstallationImpact {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InstallationImpact").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct InstallationRebootBehavior(pub i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const irbNeverReboots: InstallationRebootBehavior = InstallationRebootBehavior(0i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const irbAlwaysRequiresReboot: InstallationRebootBehavior = InstallationRebootBehavior(1i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const irbCanRequestReboot: InstallationRebootBehavior = InstallationRebootBehavior(2i32);
impl ::core::marker::Copy for InstallationRebootBehavior {}
impl ::core::clone::Clone for InstallationRebootBehavior {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for InstallationRebootBehavior {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for InstallationRebootBehavior {
    type Abi = Self;
}
impl ::core::fmt::Debug for InstallationRebootBehavior {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InstallationRebootBehavior").field(&self.0).finish()
    }
}
pub const LIBID_WUApiLib: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb596cc9f_56e5_419e_a622_e01bb457431e);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct OperationResultCode(pub i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const orcNotStarted: OperationResultCode = OperationResultCode(0i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const orcInProgress: OperationResultCode = OperationResultCode(1i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const orcSucceeded: OperationResultCode = OperationResultCode(2i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const orcSucceededWithErrors: OperationResultCode = OperationResultCode(3i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const orcFailed: OperationResultCode = OperationResultCode(4i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const orcAborted: OperationResultCode = OperationResultCode(5i32);
impl ::core::marker::Copy for OperationResultCode {}
impl ::core::clone::Clone for OperationResultCode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for OperationResultCode {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for OperationResultCode {
    type Abi = Self;
}
impl ::core::fmt::Debug for OperationResultCode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("OperationResultCode").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct SearchScope(pub i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const searchScopeDefault: SearchScope = SearchScope(0i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const searchScopeMachineOnly: SearchScope = SearchScope(1i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const searchScopeCurrentUserOnly: SearchScope = SearchScope(2i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const searchScopeMachineAndCurrentUser: SearchScope = SearchScope(3i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const searchScopeMachineAndAllUsers: SearchScope = SearchScope(4i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const searchScopeAllUsers: SearchScope = SearchScope(5i32);
impl ::core::marker::Copy for SearchScope {}
impl ::core::clone::Clone for SearchScope {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SearchScope {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for SearchScope {
    type Abi = Self;
}
impl ::core::fmt::Debug for SearchScope {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SearchScope").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct ServerSelection(pub i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const ssDefault: ServerSelection = ServerSelection(0i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const ssManagedServer: ServerSelection = ServerSelection(1i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const ssWindowsUpdate: ServerSelection = ServerSelection(2i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const ssOthers: ServerSelection = ServerSelection(3i32);
impl ::core::marker::Copy for ServerSelection {}
impl ::core::clone::Clone for ServerSelection {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ServerSelection {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for ServerSelection {
    type Abi = Self;
}
impl ::core::fmt::Debug for ServerSelection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ServerSelection").field(&self.0).finish()
    }
}
pub const StringCollection: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x72c97d74_7c3b_40ae_b77d_abdb22eba6fb);
pub const SystemInformation: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc01b9ba0_bea7_41ba_b604_d0a36f469133);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const UPDATE_LOCKDOWN_WEBSITE_ACCESS: u32 = 1u32;
pub const UpdateCollection: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x13639463_00db_4646_803d_528026140d88);
pub const UpdateDownloader: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5baf654a_5a07_4264_a255_9ff54c7151e7);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct UpdateExceptionContext(pub i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const uecGeneral: UpdateExceptionContext = UpdateExceptionContext(1i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const uecWindowsDriver: UpdateExceptionContext = UpdateExceptionContext(2i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const uecWindowsInstaller: UpdateExceptionContext = UpdateExceptionContext(3i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const uecSearchIncomplete: UpdateExceptionContext = UpdateExceptionContext(4i32);
impl ::core::marker::Copy for UpdateExceptionContext {}
impl ::core::clone::Clone for UpdateExceptionContext {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UpdateExceptionContext {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for UpdateExceptionContext {
    type Abi = Self;
}
impl ::core::fmt::Debug for UpdateExceptionContext {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UpdateExceptionContext").field(&self.0).finish()
    }
}
pub const UpdateInstaller: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd2e0fe7f_d23e_48e1_93c0_6fa8cc346474);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct UpdateLockdownOption(pub i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const uloForWebsiteAccess: UpdateLockdownOption = UpdateLockdownOption(1i32);
impl ::core::marker::Copy for UpdateLockdownOption {}
impl ::core::clone::Clone for UpdateLockdownOption {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UpdateLockdownOption {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for UpdateLockdownOption {
    type Abi = Self;
}
impl ::core::fmt::Debug for UpdateLockdownOption {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UpdateLockdownOption").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct UpdateOperation(pub i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const uoInstallation: UpdateOperation = UpdateOperation(1i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const uoUninstallation: UpdateOperation = UpdateOperation(2i32);
impl ::core::marker::Copy for UpdateOperation {}
impl ::core::clone::Clone for UpdateOperation {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UpdateOperation {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for UpdateOperation {
    type Abi = Self;
}
impl ::core::fmt::Debug for UpdateOperation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UpdateOperation").field(&self.0).finish()
    }
}
pub const UpdateSearcher: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb699e5e8_67ff_4177_88b0_3684a3388bfb);
pub const UpdateServiceManager: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf8d253d9_89a4_4daa_87b6_1168369f0b21);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct UpdateServiceOption(pub i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const usoNonVolatileService: UpdateServiceOption = UpdateServiceOption(1i32);
impl ::core::marker::Copy for UpdateServiceOption {}
impl ::core::clone::Clone for UpdateServiceOption {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UpdateServiceOption {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for UpdateServiceOption {
    type Abi = Self;
}
impl ::core::fmt::Debug for UpdateServiceOption {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UpdateServiceOption").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct UpdateServiceRegistrationState(pub i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const usrsNotRegistered: UpdateServiceRegistrationState = UpdateServiceRegistrationState(1i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const usrsRegistrationPending: UpdateServiceRegistrationState = UpdateServiceRegistrationState(2i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const usrsRegistered: UpdateServiceRegistrationState = UpdateServiceRegistrationState(3i32);
impl ::core::marker::Copy for UpdateServiceRegistrationState {}
impl ::core::clone::Clone for UpdateServiceRegistrationState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UpdateServiceRegistrationState {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for UpdateServiceRegistrationState {
    type Abi = Self;
}
impl ::core::fmt::Debug for UpdateServiceRegistrationState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UpdateServiceRegistrationState").field(&self.0).finish()
    }
}
pub const UpdateSession: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4cb43d7f_7eee_4906_8698_60da1c38f2fe);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct UpdateType(pub i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const utSoftware: UpdateType = UpdateType(1i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const utDriver: UpdateType = UpdateType(2i32);
impl ::core::marker::Copy for UpdateType {}
impl ::core::clone::Clone for UpdateType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UpdateType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for UpdateType {
    type Abi = Self;
}
impl ::core::fmt::Debug for UpdateType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UpdateType").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_ALL_UPDATES_FAILED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145124318i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_AUCLIENT_UNEXPECTED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145107969i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_AU_CALL_CANCELLED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145124267i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_AU_DETECT_SVCID_MISMATCH: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145083386i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_AU_LEGACYCLIENTDISABLED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145083389i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_AU_NONLEGACYSERVER: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145083390i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_AU_NOSERVICE: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145083392i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_AU_NO_REGISTERED_SERVICE: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145083387i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_AU_OOBE_IN_PROGRESS: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145083384i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_AU_PAUSED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145083388i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_AU_UNEXPECTED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145079297i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_BAD_FILE_URL: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145124282i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_BAD_XML_HARDWARECAPABILITY: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145079038i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_BIN_SOURCE_ABSENT: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145124308i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_CALLBACK_COOKIE_NOT_FOUND: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145062907i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_CALL_CANCELLED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145124341i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_CALL_CANCELLED_BY_HIDE: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145124262i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_CALL_CANCELLED_BY_INTERACTIVE_SEARCH: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145124253i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_CALL_CANCELLED_BY_INVALID: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145124261i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_CALL_CANCELLED_BY_POLICY: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145124305i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_COULDNOTCANCEL: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145124342i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_CYCLE_DETECTED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145124337i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_DM_BG_ERROR_TOKEN_REQUIRED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145099761i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_DM_BITSTRANSFERERROR: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145099767i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_DM_CONTENTCHANGED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145099765i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_DM_DOSVC_REQUIRED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145099746i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_DM_DOWNLOADFILEMISSING: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145099758i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_DM_DOWNLOADFILEPATHUNKNOWN: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145099759i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_DM_DOWNLOADLIMITEDBYUPDATESIZE: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145099764i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_DM_DOWNLOADLOCATIONCHANGED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145099766i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_DM_DOWNLOADSANDBOXNOTFOUND: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145099760i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_DM_DOWNLOAD_VOLUME_CONFLICT: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145099749i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_DM_FAILTOCONNECTTOBITS: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145099768i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_DM_FALLINGBACKTOBITS: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145099750i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_DM_HARDRESERVEID_CONFLICT: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145099747i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_DM_INCORRECTFILEHASH: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145099774i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_DM_NEEDDOWNLOADREQUEST: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145099772i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_DM_NONETWORK: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145099771i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_DM_NOTDOWNLOADED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145099769i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_DM_READRANGEFAILED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145099756i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_DM_SANDBOX_HASH_MISMATCH: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145099748i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_DM_UNAUTHORIZED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145099762i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_DM_UNAUTHORIZED_DOMAIN_USER: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145099752i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_DM_UNAUTHORIZED_LOCAL_USER: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145099753i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_DM_UNAUTHORIZED_MSA_USER: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145099751i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_DM_UNAUTHORIZED_NO_USER: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145099754i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_DM_UNEXPECTED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145095681i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_DM_UNKNOWNALGORITHM: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145099773i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_DM_UPDATEREMOVED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145099757i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_DM_URLNOTAVAILABLE: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145099775i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_DM_WRONGBITSVERSION: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145099770i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_DOWNLOAD_FAILED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145124300i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_DRV_DEVICE_PROBLEM: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145075192i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_DRV_MISSING_ATTRIBUTE: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145075195i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_DRV_NOPROP_OR_LEGACY: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145075198i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_DRV_NO_METADATA: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145075196i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_DRV_NO_PRINTER_CONTENT: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145075193i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_DRV_PRUNED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145075199i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_DRV_REG_MISMATCH: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145075197i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_DRV_SYNC_FAILED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145075194i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_DRV_UNEXPECTED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145071105i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_DS_BADVERSION: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145091578i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_DS_CANNOTREGISTER: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145091568i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_DS_CANTDELETE: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145091573i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_DS_DATANOTAVAILABLE: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145091554i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_DS_DATANOTLOADED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145091553i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_DS_DECLINENOTALLOWED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145091562i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_DS_DUPLICATEUPDATEID: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145091565i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_DS_IMPERSONATED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145091555i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_DS_INUSE: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145091583i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_DS_INVALID: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145091582i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_DS_INVALIDOPERATION: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145091558i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_DS_INVALIDTABLENAME: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145091579i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_DS_LOCKTIMEOUTEXPIRED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145091572i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_DS_MISSINGDATA: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145091576i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_DS_MISSINGREF: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145091575i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_DS_NEEDWINDOWSSERVICE: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145091559i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_DS_NOCATEGORIES: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145091571i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_DS_NODATA: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145091577i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_DS_NODATA_CCR: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145091546i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_DS_NODATA_COOKIE: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145091548i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_DS_NODATA_DOWNLOADJOB: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145091544i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_DS_NODATA_EULA: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145091550i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_DS_NODATA_FILE: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145091545i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_DS_NODATA_NOSUCHREVISION: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145091552i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_DS_NODATA_NOSUCHUPDATE: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145091551i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_DS_NODATA_SERVICE: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145091549i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_DS_NODATA_TIMER: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145091547i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_DS_NODATA_TMI: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145091543i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_DS_RESETREQUIRED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145091556i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_DS_ROWEXISTS: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145091570i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_DS_SCHEMAMISMATCH: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145091557i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_DS_SERVICEEXPIRED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145091563i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_DS_SESSIONLOCKMISMATCH: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145091560i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_DS_SHUTDOWN: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145091584i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_DS_STOREFILELOCKED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145091569i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_DS_TABLEINCORRECT: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145091580i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_DS_TABLEMISSING: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145091581i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_DS_TABLESESSIONMISMATCH: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145091561i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_DS_UNABLETOSTART: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145091567i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_DS_UNEXPECTED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145087489i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_DS_UNKNOWNHANDLER: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145091574i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_DS_UNKNOWNSERVICE: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145091564i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_DUPLICATE_ITEM: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145124333i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_EE_CLUSTER_ERROR: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145067001i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_EE_INVALID_ATTRIBUTEDATA: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145067002i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_EE_INVALID_EXPRESSION: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145067006i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_EE_INVALID_VERSION: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145067004i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_EE_MISSING_METADATA: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145067005i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_EE_NOT_INITIALIZED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145067003i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_EE_UNEXPECTED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145062913i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_EE_UNKNOWN_EXPRESSION: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145067007i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_EULAS_DECLINED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145124317i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_EULA_UNAVAILABLE: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145124301i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_EXCLUSIVE_INSTALL_CONFLICT: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145124327i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_EXTENDEDERROR_FAILED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145124257i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_EXTENDEDERROR_NOTSET: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145124258i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_FILETRUST_DUALSIGNATURE_ECC: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145078526i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_FILETRUST_DUALSIGNATURE_RSA: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145078527i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_FILETRUST_SHA2SIGNATURE_MISSING: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145124255i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_IDLESHUTDOWN_OPCOUNT_DISCOVERY: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145124273i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_IDLESHUTDOWN_OPCOUNT_DOWNLOAD: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145124271i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_IDLESHUTDOWN_OPCOUNT_INSTALL: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145124270i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_IDLESHUTDOWN_OPCOUNT_OTHER: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145124269i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_IDLESHUTDOWN_OPCOUNT_SEARCH: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145124272i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_IDLESHUTDOWN_OPCOUNT_SERVICEREGISTRATION: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145124256i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_INFRASTRUCTUREFILE_INVALID_FORMAT: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145124275i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_INFRASTRUCTUREFILE_REQUIRES_SSL: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145124274i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_INSTALLATION_RESULTS_INVALID_DATA: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145112062i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_INSTALLATION_RESULTS_NOT_FOUND: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145112061i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_INSTALLATION_RESULTS_UNKNOWN_VERSION: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145112063i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_INSTALL_JOB_NOT_SUSPENDED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145124251i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_INSTALL_JOB_RESUME_NOT_ALLOWED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145124252i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_INSTALL_NOT_ALLOWED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145124330i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_INSTALL_USERCONTEXT_ACCESSDENIED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145124250i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_INTERACTIVE_CALL_CANCELLED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145124268i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_INVALIDINDEX: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145124345i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_INVALID_CRITERIA: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145124302i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_INVALID_EVENT: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145062909i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_INVALID_EVENT_PAYLOAD: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145095677i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_INVALID_EVENT_PAYLOADSIZE: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145095676i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_INVALID_FILE: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145124303i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_INVALID_INSTALL_REQUESTED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145124332i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_INVALID_NOTIFICATION_INFO: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145124280i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_INVALID_OPERATION: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145124298i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_INVALID_PRODUCT_LICENSE: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145124311i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_INVALID_PROXY_SERVER: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145124304i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_INVALID_RELATIONSHIP: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145124335i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_INVALID_SERIALIZATION_VERSION: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145124264i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_INVALID_UPDATE: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145124323i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_INVALID_UPDATE_TYPE: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145124314i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_INVALID_VOLUMEID: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145124260i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_INVENTORY_GET_INVENTORY_TYPE_FAILED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145087486i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_INVENTORY_PARSEFAILED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145087487i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_INVENTORY_RESULT_UPLOAD_FAILED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145087485i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_INVENTORY_UNEXPECTED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145087484i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_INVENTORY_WMI_ERROR: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145087483i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_ITEMNOTFOUND: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145124344i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_LEGACYSERVER: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145124309i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_LOW_BATTERY: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145124276i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_MAX_CAPACITY_REACHED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145124350i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_METADATATRUST_CERTIFICATECHAIN_VERIFICATION: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145095344i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_METADATATRUST_UNTRUSTED_CERTIFICATECHAIN: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145095343i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_METADATA_BAD_FRAGMENTSIGNING_CONFIG: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145095417i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_METADATA_BAD_SIGNATURE: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145095360i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_METADATA_CERT_MISSING: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145095296i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_METADATA_CERT_UNTRUSTED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145095293i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_METADATA_CONFIG_INVALID_BINARY_ENCODING: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145095423i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_METADATA_FAILURE_PROCESSING_FRAGMENTSIGNING_CONFIG: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145095416i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_METADATA_FETCH_CONFIG: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145095422i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_METADATA_INTCERT_BAD_TRANSPORT_ENCODING: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145095294i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_METADATA_INVALID_PARAMETER: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145095420i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_METADATA_LEAFCERT_BAD_TRANSPORT_ENCODING: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145095295i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_METADATA_NOOP: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145095424i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_METADATA_NO_VERIFICATION_DATA: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145095418i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_METADATA_SIGNATURE_VERIFY_FAILED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145095358i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_METADATA_TIMESTAMP_TOKEN_ALL_BAD: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145095321i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_METADATA_TIMESTAMP_TOKEN_CACHELOOKUP: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145095319i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_METADATA_TIMESTAMP_TOKEN_CERTCHAIN: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145095323i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_METADATA_TIMESTAMP_TOKEN_MISSING: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145095328i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_METADATA_TIMESTAMP_TOKEN_NODATA: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145095320i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_METADATA_TIMESTAMP_TOKEN_REFRESHONLINE: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145095322i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_METADATA_TIMESTAMP_TOKEN_SIGNATURE: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145095324i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_METADATA_TIMESTAMP_TOKEN_UNEXPECTED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145095297i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_METADATA_TIMESTAMP_TOKEN_UNTRUSTED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145095326i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_METADATA_TIMESTAMP_TOKEN_VALIDITYWINDOW_UNEXPECTED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145095298i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_METADATA_TIMESTAMP_TOKEN_VALIDITY_WINDOW: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145095325i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_METADATA_TIMESTAMP_TOKEN_VERIFICATION_FAILED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145095327i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_METADATA_UNEXPECTED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145095419i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_METADATA_UNSUPPORTED_HASH_ALG: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145095359i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_METADATA_XML_BASE64CERDATA_MISSING: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145095384i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_METADATA_XML_FRAGMENTSIGNING_MISSING: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145095391i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_METADATA_XML_INTERMEDIATECERT_MISSING: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145095386i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_METADATA_XML_LEAFCERT_ID_MISSING: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145095385i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_METADATA_XML_LEAFCERT_MISSING: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145095387i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_METADATA_XML_MISSING: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145095392i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_METADATA_XML_MODE_INVALID: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145095389i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_METADATA_XML_MODE_MISSING: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145095390i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_METADATA_XML_VALIDITY_INVALID: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145095388i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_MISSING_HANDLER: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145124310i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_MSI_NOT_CONFIGURED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145120254i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_MSI_NOT_PRESENT: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145120251i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_MSI_WRONG_APP_CONTEXT: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145120252i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_MSI_WRONG_VERSION: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145120255i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_MSP_DISABLED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145120253i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_MSP_UNEXPECTED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145116161i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_NETWORK_COST_EXCEEDS_POLICY: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145124263i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_NON_UI_MODE: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145107971i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_NOOP: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145124340i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_NOT_APPLICABLE: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145124329i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_NOT_INITIALIZED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145124348i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_NOT_SUPPORTED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145124297i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_NO_CONNECTION: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145124321i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_NO_INTERACTIVE_USER: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145124320i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_NO_SERVER_CORE_SUPPORT: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145124288i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_NO_SERVICE: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145124351i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_NO_SUCH_HANDLER_PLUGIN: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145124265i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_NO_UI_SUPPORT: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145124285i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_NO_UPDATE: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145124316i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_NO_USERTOKEN: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145124328i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_OL_INVALID_SCANFILE: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145095679i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_OL_NEWCLIENT_REQUIRED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145095678i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_OL_UNEXPECTED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145091585i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_OPERATIONINPROGRESS: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145124343i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_ORPHANED_DOWNLOAD_JOB: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145124277i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_OUTOFRANGE: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145124279i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_PER_MACHINE_UPDATE_ACCESS_DENIED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145124284i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_POLICY_NOT_SET: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145124326i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_PT_ADDRESS_IN_USE: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145123256i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_PT_ADDRESS_NOT_AVAILABLE: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145123255i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_PT_CATALOG_SYNC_REQUIRED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145123274i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_PT_CONFIG_PROP_MISSING: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145107926i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_PT_DOUBLE_INITIALIZATION: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145107950i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_PT_ECP_FAILURE_TO_DECOMPRESS_CAB_FILE: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145107916i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_PT_ECP_FAILURE_TO_EXTRACT_DIGEST: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145107917i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_PT_ECP_FILE_LOCATION_ERROR: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145107915i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_PT_ECP_INIT_FAILED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145107920i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_PT_ECP_INVALID_FILE_FORMAT: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145107919i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_PT_ECP_INVALID_METADATA: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145107918i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_PT_ECP_SUCCEEDED_WITH_ERRORS: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145107921i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_PT_ENDPOINTURL_NOTAVAIL: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145123265i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_PT_ENDPOINT_DISCONNECTED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145123264i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_PT_ENDPOINT_REFRESH_REQUIRED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145123266i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_PT_ENDPOINT_UNREACHABLE: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145123272i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_PT_EXCEEDED_MAX_SERVER_TRIPS: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145107952i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_PT_FILE_LOCATIONS_CHANGED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145107931i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_PT_HTTP_STATUS_BAD_GATEWAY: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145107935i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_PT_HTTP_STATUS_BAD_METHOD: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145107942i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_PT_HTTP_STATUS_BAD_REQUEST: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145107946i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_PT_HTTP_STATUS_CONFLICT: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145107939i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_PT_HTTP_STATUS_DENIED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145107945i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_PT_HTTP_STATUS_FORBIDDEN: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145107944i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_PT_HTTP_STATUS_GATEWAY_TIMEOUT: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145107933i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_PT_HTTP_STATUS_GONE: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145107938i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_PT_HTTP_STATUS_NOT_FOUND: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145107943i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_PT_HTTP_STATUS_NOT_MAPPED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145107925i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_PT_HTTP_STATUS_NOT_SUPPORTED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145107936i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_PT_HTTP_STATUS_PROXY_AUTH_REQ: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145107941i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_PT_HTTP_STATUS_REQUEST_TIMEOUT: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145107940i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_PT_HTTP_STATUS_SERVER_ERROR: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145107937i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_PT_HTTP_STATUS_SERVICE_UNAVAIL: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145107934i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_PT_HTTP_STATUS_VERSION_NOT_SUP: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145107932i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_PT_INVALID_COMPUTER_NAME: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145107949i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_PT_INVALID_CONFIG_PROP: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145107927i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_PT_INVALID_FORMAT: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145123271i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_PT_INVALID_OPERATION: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145123263i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_PT_INVALID_URL: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145123270i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_PT_LOAD_SHEDDING: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145107923i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_PT_NO_AUTH_COOKIES_CREATED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145107928i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_PT_NO_AUTH_PLUGINS_REQUESTED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145107929i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_PT_NO_MANAGED_RECOVER: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145103826i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_PT_NO_TRANSLATION_AVAILABLE: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145123257i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_PT_NUMERIC_OVERFLOW: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145123261i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_PT_NWS_NOT_LOADED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145123269i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_PT_OBJECT_FAULTED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145123262i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_PT_OPERATION_ABANDONED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145123259i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_PT_OPERATION_ABORTED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145123260i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_PT_OTHER: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145123254i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_PT_PROXY_AUTH_SCHEME_NOT_SUPPORTED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145123268i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_PT_QUOTA_EXCEEDED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145123258i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_PT_REFRESH_CACHE_REQUIRED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145107947i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_PT_REGISTRATION_NOT_SUPPORTED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145107930i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_PT_SAME_REDIR_ID: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145103827i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_PT_SECURITY_SYSTEM_FAILURE: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145123253i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_PT_SECURITY_VERIFICATION_FAILURE: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145123273i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_PT_SOAPCLIENT_BASE: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145107968i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_PT_SOAPCLIENT_CONNECT: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145107964i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_PT_SOAPCLIENT_GENERATE: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145107965i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_PT_SOAPCLIENT_INITIALIZE: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145107967i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_PT_SOAPCLIENT_OUTOFMEMORY: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145107966i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_PT_SOAPCLIENT_PARSE: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145107958i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_PT_SOAPCLIENT_PARSEFAULT: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145107960i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_PT_SOAPCLIENT_READ: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145107959i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_PT_SOAPCLIENT_SEND: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145107963i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_PT_SOAPCLIENT_SERVER: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145107962i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_PT_SOAPCLIENT_SOAPFAULT: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145107961i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_PT_SOAP_CLIENT: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145107955i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_PT_SOAP_MUST_UNDERSTAND: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145107956i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_PT_SOAP_SERVER: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145107954i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_PT_SOAP_VERSION: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145107957i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_PT_SUS_SERVER_NOT_SET: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145107951i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_PT_UNEXPECTED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145103873i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_PT_WINHTTP_NAME_NOT_RESOLVED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145107924i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_PT_WMI_ERROR: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145107953i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_RANGEOVERLAP: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145124347i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_REBOOT_IN_PROGRESS: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145083385i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_REDIRECTOR_ATTRPROVIDER_EXCEEDED_MAX_NAMEVALUE: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145103864i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_REDIRECTOR_ATTRPROVIDER_INVALID_NAME: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145103863i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_REDIRECTOR_ATTRPROVIDER_INVALID_VALUE: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145103862i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_REDIRECTOR_CONNECT_POLICY: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145103860i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_REDIRECTOR_ID_SMALLER: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145103869i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_REDIRECTOR_INVALID_RESPONSE: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145103866i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_REDIRECTOR_LOAD_XML: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145103871i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_REDIRECTOR_ONLINE_DISALLOWED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145103859i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_REDIRECTOR_SLS_GENERIC_ERROR: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145103861i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_REDIRECTOR_S_FALSE: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145103870i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_REDIRECTOR_UNEXPECTED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145103617i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_REDIRECTOR_UNKNOWN_SERVICE: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145103868i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_REDIRECTOR_UNSUPPORTED_CONTENTTYPE: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145103867i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_REG_VALUE_INVALID: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145124334i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_REPORTER_EVENTCACHECORRUPT: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145062911i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_REPORTER_EVENTNAMESPACEPARSEFAILED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145062910i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_REPORTER_UNEXPECTED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145058817i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_REVERT_NOT_ALLOWED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145124281i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_SELFUPDATE_IN_PROGRESS: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145124325i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_SELFUPDATE_REQUIRED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145071087i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_SELFUPDATE_REQUIRED_ADMIN: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145071086i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_SELFUPDATE_SKIP_ON_FAILURE: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145071096i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_SERVER_BUSY: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145062908i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_SERVICEPROP_NOTAVAIL: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145123267i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_SERVICE_NOT_REGISTERED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145095675i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_SERVICE_STOP: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145124322i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_SETUP_ALREADYRUNNING: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145071091i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_SETUP_ALREADY_INITIALIZED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145071101i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_SETUP_BLOCKED_CONFIGURATION: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145071093i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_SETUP_DEFERRABLE_REBOOT_PENDING: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145071084i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_SETUP_FAIL: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145071082i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_SETUP_HANDLER_EXEC_FAILURE: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145071089i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_SETUP_INVALID_IDENTDATA: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145071102i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_SETUP_INVALID_INFDATA: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145071103i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_SETUP_INVALID_REGISTRY_DATA: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145071088i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_SETUP_IN_PROGRESS: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145124278i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_SETUP_NON_DEFERRABLE_REBOOT_PENDING: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145071083i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_SETUP_NOT_INITIALIZED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145071100i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_SETUP_REBOOTREQUIRED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145071090i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_SETUP_REBOOT_TO_FIX: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145071092i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_SETUP_REGISTRATION_FAILED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145071097i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_SETUP_SKIP_UPDATE: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145071095i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_SETUP_SOURCE_VERSION_MISMATCH: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145071099i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_SETUP_TARGET_VERSION_GREATER: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145071098i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_SETUP_UNEXPECTED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145067009i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_SETUP_UNSUPPORTED_CONFIGURATION: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145071094i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_SETUP_WRONG_SERVER_VERSION: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145071085i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_SIH_ACTION_NOT_FOUND: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145103611i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_SIH_ANOTHER_INSTANCE_RUNNING: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145103597i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_SIH_BLOCKED_FOR_PLATFORM: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145103598i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_SIH_DNSRESILIENCY_OFF: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145103596i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_SIH_ENGINE_EXCEPTION: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145103599i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_SIH_INVALIDHASH: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145103609i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_SIH_NONSTDEXCEPTION: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145103600i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_SIH_NO_ENGINE: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145103608i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_SIH_PARSE: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145103605i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_SIH_POLICY: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145103602i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_SIH_POST_REBOOT_INSTALL_FAILED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145103607i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_SIH_POST_REBOOT_NO_CACHED_SLS_RESPONSE: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145103606i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_SIH_PPL: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145103603i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_SIH_SECURITY: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145103604i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_SIH_SLS_PARSE: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145103610i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_SIH_STDEXCEPTION: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145103601i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_SIH_UNEXPECTED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145103361i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_SIH_VERIFY_DOWNLOAD_ENGINE: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145103615i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_SIH_VERIFY_DOWNLOAD_PAYLOAD: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145103614i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_SIH_VERIFY_STAGE_ENGINE: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145103613i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_SIH_VERIFY_STAGE_PAYLOAD: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145103612i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_SKIPPED_UPDATE_INSTALLATION: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145079035i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_SLS_INVALID_REVISION: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145078783i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_SOURCE_ABSENT: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145124307i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_SYSPREP_IN_PROGRESS: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145124287i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_SYSTEM_UNSUPPORTED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145124266i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_TIME_OUT: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145124319i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_TOOMANYRANGES: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145124346i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_TOO_DEEP_RELATION: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145124336i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_TOO_MANY_RESYNC: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145124295i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_TRAYICON_FAILURE: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145112060i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_TRUST_PROVIDER_UNKNOWN: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145078524i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_TRUST_SUBJECT_NOT_TRUSTED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145078525i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_UH_APPX_DEFAULT_PACKAGE_VOLUME_UNAVAILABLE: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145116127i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_UH_APPX_INSTALLED_PACKAGE_VOLUME_UNAVAILABLE: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145116126i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_UH_APPX_INVALID_PACKAGE_VOLUME: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145116128i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_UH_APPX_NOT_PRESENT: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145116130i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_UH_APPX_PACKAGE_FAMILY_NOT_FOUND: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145116125i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_UH_APPX_SYSTEM_VOLUME_NOT_FOUND: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145116124i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_UH_BADCBSPACKAGEID: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145116141i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_UH_BADHANDLERXML: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145116151i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_UH_CALLED_BACK_FAILURE: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145116136i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_UH_CANREQUIREINPUT: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145116150i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_UH_CUSTOMINSTALLER_INVALID_SIGNATURE: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145116135i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_UH_DECRYPTFAILURE: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145116132i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_UH_DOESNOTSUPPORTACTION: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145116156i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_UH_FALLBACKERROR: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145116144i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_UH_FALLBACKTOSELFCONTAINED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145116148i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_UH_HANDLER_DISABLEDUNTILREBOOT: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145116131i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_UH_INCONSISTENT_FILE_NAMES: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145116145i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_UH_INSTALLERFAILURE: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145116149i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_UH_INSTALLERHUNG: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145116153i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_UH_INVALIDMETADATA: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145116154i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_UH_INVALID_TARGETSESSION: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145116133i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_UH_LOCALONLY: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145116159i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_UH_NEEDANOTHERDOWNLOAD: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145116147i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_UH_NEW_SERVICING_STACK_REQUIRED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145116137i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_UH_NOTIFYFAILURE: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145116146i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_UH_NOTREADYTOCOMMIT: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145116129i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_UH_OPERATIONCANCELLED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145116152i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_UH_POSTREBOOTRESULTUNKNOWN: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145116139i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_UH_POSTREBOOTSTILLPENDING: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145116140i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_UH_POSTREBOOTUNEXPECTEDSTATE: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145116138i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_UH_REMOTEALREADYACTIVE: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145116157i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_UH_REMOTEUNAVAILABLE: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145116160i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_UH_TOOMANYDOWNLOADREQUESTS: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145116143i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_UH_UNEXPECTED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145112065i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_UH_UNEXPECTEDCBSRESPONSE: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145116142i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_UH_UNKNOWNHANDLER: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145116158i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_UH_UNSUPPORTED_INSTALLCONTEXT: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145116134i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_UH_WRONGHANDLER: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145116155i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_UNEXPECTED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145120257i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_UNINSTALL_NOT_ALLOWED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145124312i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_UNKNOWN_HARDWARECAPABILITY: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145079039i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_UNKNOWN_ID: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145124349i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_UNKNOWN_SERVICE: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145124286i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_UNRECOGNIZED_VOLUMEID: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145124259i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_UNSUPPORTED_SEARCHSCOPE: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145124283i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_UPDATE_MERGE_NOT_ALLOWED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145079036i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_UPDATE_NOT_APPROVED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145124254i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_UPDATE_NOT_PROCESSED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145124299i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_URL_TOO_LONG: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145124313i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_USER_ACCESS_DISABLED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145124315i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_WINHTTP_INVALID_FILE: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145124296i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_WMI_NOT_SUPPORTED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145079037i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_WUCLTUI_UNSUPPORTED_VERSION: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145107970i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_WUTASK_CANCELINSTALL_DISALLOWED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145079291i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_WUTASK_INPROGRESS: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145079295i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_WUTASK_NOT_STARTED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145079293i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_WUTASK_RETRY: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145079292i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_WUTASK_STATUS_DISABLED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145079294i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_WU_DISABLED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145124306i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_XML_INVALID: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145124338i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_E_XML_MISSINGDATA: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145124339i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_S_ALREADY_DOWNLOADED: ::windows_core::HRESULT = ::windows_core::HRESULT(2359304i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_S_ALREADY_INSTALLED: ::windows_core::HRESULT = ::windows_core::HRESULT(2359302i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_S_ALREADY_REVERTED: ::windows_core::HRESULT = ::windows_core::HRESULT(2359306i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_S_ALREADY_UNINSTALLED: ::windows_core::HRESULT = ::windows_core::HRESULT(2359303i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_S_DM_ALREADYDOWNLOADING: ::windows_core::HRESULT = ::windows_core::HRESULT(2383873i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_S_MARKED_FOR_DISCONNECT: ::windows_core::HRESULT = ::windows_core::HRESULT(2359300i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_S_METADATA_IGNORED_SIGNATURE_VERIFICATION: ::windows_core::HRESULT = ::windows_core::HRESULT(2388226i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_S_METADATA_SKIPPED_BY_ENFORCEMENTMODE: ::windows_core::HRESULT = ::windows_core::HRESULT(2388225i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_S_REBOOT_REQUIRED: ::windows_core::HRESULT = ::windows_core::HRESULT(2359301i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_S_SEARCH_CRITERIA_NOT_SUPPORTED: ::windows_core::HRESULT = ::windows_core::HRESULT(2359312i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_S_SEARCH_LOAD_SHEDDING: ::windows_core::HRESULT = ::windows_core::HRESULT(2392065i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_S_SELFUPDATE: ::windows_core::HRESULT = ::windows_core::HRESULT(2359298i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_S_SERVICE_STOP: ::windows_core::HRESULT = ::windows_core::HRESULT(2359297i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_S_SIH_NOOP: ::windows_core::HRESULT = ::windows_core::HRESULT(2379777i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_S_SOME_UPDATES_SKIPPED_ON_BATTERY: ::windows_core::HRESULT = ::windows_core::HRESULT(2359305i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_S_UH_DOWNLOAD_SIZE_CALCULATED: ::windows_core::HRESULT = ::windows_core::HRESULT(2367510i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_S_UH_INSTALLSTILLPENDING: ::windows_core::HRESULT = ::windows_core::HRESULT(2367509i32);
#[doc = "*Required features: `\"Win32_System_UpdateAgent\"`*"]
pub const WU_S_UPDATE_ERROR: ::windows_core::HRESULT = ::windows_core::HRESULT(2359299i32);
pub const WebProxy: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x650503cf_9108_4ddc_a2ce_6c2341e1c582);
pub const WindowsUpdateAgentInfo: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc2e88c2f_6f5b_4aaa_894b_55c847ad3a2d);
#[cfg(feature = "implement")]
::core::include!("impl.rs");
