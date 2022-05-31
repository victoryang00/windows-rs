#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct APPSTATE(pub i32);
pub const ABSENT: APPSTATE = APPSTATE(0i32);
pub const ASSIGNED: APPSTATE = APPSTATE(1i32);
pub const PUBLISHED: APPSTATE = APPSTATE(2i32);
impl ::core::marker::Copy for APPSTATE {}
impl ::core::clone::Clone for APPSTATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for APPSTATE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for APPSTATE {
    type Abi = Self;
}
impl ::core::fmt::Debug for APPSTATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("APPSTATE").field(&self.0).finish()
    }
}
#[inline]
pub unsafe fn BrowseForGPO(lpbrowseinfo: *mut GPOBROWSEINFO) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn BrowseForGPO(lpbrowseinfo: *mut GPOBROWSEINFO) -> ::windows_core::HRESULT;
        }
        BrowseForGPO(::core::mem::transmute(lpbrowseinfo)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub const CLSID_GPESnapIn: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8fc0b734_a0e1_11d1_a7d3_0000f87571e3);
pub const CLSID_GroupPolicyObject: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xea502722_a23d_11d1_a7d3_0000f87571e3);
pub const CLSID_RSOPSnapIn: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6dc3804b_7212_458d_adb0_9a07e2ae1fa2);
#[inline]
pub unsafe fn CommandLineFromMsiDescriptor<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(descriptor: Param0, commandline: ::windows_core::PWSTR, commandlinelength: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CommandLineFromMsiDescriptor(descriptor: ::windows_core::PCWSTR, commandline: ::windows_core::PWSTR, commandlinelength: *mut u32) -> u32;
        }
        ::core::mem::transmute(CommandLineFromMsiDescriptor(descriptor.into_param().abi(), ::core::mem::transmute(commandline), ::core::mem::transmute(commandlinelength)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CreateGPOLink<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param2: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(lpgpo: Param0, lpcontainer: Param1, fhighpriority: Param2) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateGPOLink(lpgpo: ::windows_core::PCWSTR, lpcontainer: ::windows_core::PCWSTR, fhighpriority: ::win32_foundation::BOOL) -> ::windows_core::HRESULT;
        }
        CreateGPOLink(lpgpo.into_param().abi(), lpcontainer.into_param().abi(), fhighpriority.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct CriticalPolicySectionHandle(pub isize);
impl CriticalPolicySectionHandle {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 || self.0 == 0
    }
}
impl ::core::default::Default for CriticalPolicySectionHandle {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for CriticalPolicySectionHandle {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for CriticalPolicySectionHandle {}
impl ::core::fmt::Debug for CriticalPolicySectionHandle {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CriticalPolicySectionHandle").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Abi for CriticalPolicySectionHandle {
    type Abi = Self;
}
#[inline]
pub unsafe fn DeleteAllGPOLinks<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(lpcontainer: Param0) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DeleteAllGPOLinks(lpcontainer: ::windows_core::PCWSTR) -> ::windows_core::HRESULT;
        }
        DeleteAllGPOLinks(lpcontainer.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn DeleteGPOLink<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(lpgpo: Param0, lpcontainer: Param1) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DeleteGPOLink(lpgpo: ::windows_core::PCWSTR, lpcontainer: ::windows_core::PCWSTR) -> ::windows_core::HRESULT;
        }
        DeleteGPOLink(lpgpo.into_param().abi(), lpcontainer.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn EnterCriticalPolicySection<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(bmachine: Param0) -> ::windows_core::Result<::win32_foundation::HANDLE> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EnterCriticalPolicySection(bmachine: ::win32_foundation::BOOL) -> ::win32_foundation::HANDLE;
        }
        let result__ = EnterCriticalPolicySection(bmachine.into_param().abi());
        (!result__.is_invalid()).then(|| result__).ok_or_else(::windows_core::Error::from_win32)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn ExportRSoPData<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(lpnamespace: Param0, lpfilename: Param1) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ExportRSoPData(lpnamespace: ::windows_core::PCWSTR, lpfilename: ::windows_core::PCWSTR) -> ::windows_core::HRESULT;
        }
        ExportRSoPData(lpnamespace.into_param().abi(), lpfilename.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub const FLAG_ASSUME_COMP_WQLFILTER_TRUE: u32 = 33554432u32;
pub const FLAG_ASSUME_SLOW_LINK: u32 = 536870912u32;
pub const FLAG_ASSUME_USER_WQLFILTER_TRUE: u32 = 67108864u32;
pub const FLAG_FORCE_CREATENAMESPACE: u32 = 4u32;
pub const FLAG_LOOPBACK_MERGE: u32 = 268435456u32;
pub const FLAG_LOOPBACK_REPLACE: u32 = 134217728u32;
pub const FLAG_NO_COMPUTER: u32 = 2u32;
pub const FLAG_NO_CSE_INVOKE: u32 = 1073741824u32;
pub const FLAG_NO_GPO_FILTER: u32 = 2147483648u32;
pub const FLAG_NO_USER: u32 = 1u32;
pub const FLAG_PLANNING_MODE: u32 = 16777216u32;
#[inline]
pub unsafe fn FreeGPOListA(pgpolist: *const GROUP_POLICY_OBJECTA) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FreeGPOListA(pgpolist: *const GROUP_POLICY_OBJECTA) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(FreeGPOListA(::core::mem::transmute(pgpolist)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn FreeGPOListW(pgpolist: *const GROUP_POLICY_OBJECTW) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FreeGPOListW(pgpolist: *const GROUP_POLICY_OBJECTW) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(FreeGPOListW(::core::mem::transmute(pgpolist)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub const GPC_BLOCK_POLICY: u32 = 1u32;
pub const GPM: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf5694708_88fe_4b35_babf_e56162d5fbc8);
pub const GPMAsyncCancel: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x372796a9_76ec_479d_ad6c_556318ed5f9d);
pub const GPMBackup: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xed1a54b8_5efa_482a_93c0_8ad86f0d68c3);
pub const GPMBackupCollection: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xeb8f035b_70db_4a9f_9676_37c25994e9dc);
pub const GPMBackupDir: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xfce4a59d_0f21_4afa_b859_e6d0c62cd10c);
pub const GPMBackupDirEx: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe8c0988a_cf03_4c5b_8be2_2aa9ad32aada);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct GPMBackupType(pub i32);
pub const typeGPO: GPMBackupType = GPMBackupType(0i32);
pub const typeStarterGPO: GPMBackupType = GPMBackupType(1i32);
impl ::core::marker::Copy for GPMBackupType {}
impl ::core::clone::Clone for GPMBackupType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for GPMBackupType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for GPMBackupType {
    type Abi = Self;
}
impl ::core::fmt::Debug for GPMBackupType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GPMBackupType").field(&self.0).finish()
    }
}
pub const GPMCSECollection: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xcf92b828_2d44_4b61_b10a_b327afd42da8);
pub const GPMClientSideExtension: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc1a2e70e_659c_4b1a_940b_f88b0af9c8a4);
pub const GPMConstants: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3855e880_cd9e_4d0c_9eaf_1579283a1888);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct GPMDestinationOption(pub i32);
pub const opDestinationSameAsSource: GPMDestinationOption = GPMDestinationOption(0i32);
pub const opDestinationNone: GPMDestinationOption = GPMDestinationOption(1i32);
pub const opDestinationByRelativeName: GPMDestinationOption = GPMDestinationOption(2i32);
pub const opDestinationSet: GPMDestinationOption = GPMDestinationOption(3i32);
impl ::core::marker::Copy for GPMDestinationOption {}
impl ::core::clone::Clone for GPMDestinationOption {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for GPMDestinationOption {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for GPMDestinationOption {
    type Abi = Self;
}
impl ::core::fmt::Debug for GPMDestinationOption {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GPMDestinationOption").field(&self.0).finish()
    }
}
pub const GPMDomain: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x710901be_1050_4cb1_838a_c5cff259e183);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct GPMEntryType(pub i32);
pub const typeUser: GPMEntryType = GPMEntryType(0i32);
pub const typeComputer: GPMEntryType = GPMEntryType(1i32);
pub const typeLocalGroup: GPMEntryType = GPMEntryType(2i32);
pub const typeGlobalGroup: GPMEntryType = GPMEntryType(3i32);
pub const typeUniversalGroup: GPMEntryType = GPMEntryType(4i32);
pub const typeUNCPath: GPMEntryType = GPMEntryType(5i32);
pub const typeUnknown: GPMEntryType = GPMEntryType(6i32);
impl ::core::marker::Copy for GPMEntryType {}
impl ::core::clone::Clone for GPMEntryType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for GPMEntryType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for GPMEntryType {
    type Abi = Self;
}
impl ::core::fmt::Debug for GPMEntryType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GPMEntryType").field(&self.0).finish()
    }
}
pub const GPMGPO: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd2ce2994_59b5_4064_b581_4d68486a16c4);
pub const GPMGPOCollection: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7a057325_832d_4de3_a41f_c780436a4e09);
pub const GPMGPOLink: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc1df9880_5303_42c6_8a3c_0488e1bf7364);
pub const GPMGPOLinksCollection: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf6ed581a_49a5_47e2_b771_fd8dc02b6259);
pub const GPMMapEntry: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8c975253_5431_4471_b35d_0626c928258a);
pub const GPMMapEntryCollection: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0cf75d5b_a3a1_4c55_b4fe_9e149c41f66d);
pub const GPMMigrationTable: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x55af4043_2a06_4f72_abef_631b44079c76);
pub const GPMPermission: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5871a40a_e9c0_46ec_913e_944ef9225a94);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct GPMPermissionType(pub i32);
pub const permGPOApply: GPMPermissionType = GPMPermissionType(65536i32);
pub const permGPORead: GPMPermissionType = GPMPermissionType(65792i32);
pub const permGPOEdit: GPMPermissionType = GPMPermissionType(65793i32);
pub const permGPOEditSecurityAndDelete: GPMPermissionType = GPMPermissionType(65794i32);
pub const permGPOCustom: GPMPermissionType = GPMPermissionType(65795i32);
pub const permWMIFilterEdit: GPMPermissionType = GPMPermissionType(131072i32);
pub const permWMIFilterFullControl: GPMPermissionType = GPMPermissionType(131073i32);
pub const permWMIFilterCustom: GPMPermissionType = GPMPermissionType(131074i32);
pub const permSOMLink: GPMPermissionType = GPMPermissionType(1835008i32);
pub const permSOMLogging: GPMPermissionType = GPMPermissionType(1573120i32);
pub const permSOMPlanning: GPMPermissionType = GPMPermissionType(1573376i32);
pub const permSOMWMICreate: GPMPermissionType = GPMPermissionType(1049344i32);
pub const permSOMWMIFullControl: GPMPermissionType = GPMPermissionType(1049345i32);
pub const permSOMGPOCreate: GPMPermissionType = GPMPermissionType(1049600i32);
pub const permStarterGPORead: GPMPermissionType = GPMPermissionType(197888i32);
pub const permStarterGPOEdit: GPMPermissionType = GPMPermissionType(197889i32);
pub const permStarterGPOFullControl: GPMPermissionType = GPMPermissionType(197890i32);
pub const permStarterGPOCustom: GPMPermissionType = GPMPermissionType(197891i32);
pub const permSOMStarterGPOCreate: GPMPermissionType = GPMPermissionType(1049856i32);
impl ::core::marker::Copy for GPMPermissionType {}
impl ::core::clone::Clone for GPMPermissionType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for GPMPermissionType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for GPMPermissionType {
    type Abi = Self;
}
impl ::core::fmt::Debug for GPMPermissionType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GPMPermissionType").field(&self.0).finish()
    }
}
pub const GPMRSOP: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x489b0caf_9ec2_4eb7_91f5_b6f71d43da8c);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct GPMRSOPMode(pub i32);
pub const rsopUnknown: GPMRSOPMode = GPMRSOPMode(0i32);
pub const rsopPlanning: GPMRSOPMode = GPMRSOPMode(1i32);
pub const rsopLogging: GPMRSOPMode = GPMRSOPMode(2i32);
impl ::core::marker::Copy for GPMRSOPMode {}
impl ::core::clone::Clone for GPMRSOPMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for GPMRSOPMode {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for GPMRSOPMode {
    type Abi = Self;
}
impl ::core::fmt::Debug for GPMRSOPMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GPMRSOPMode").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct GPMReportType(pub i32);
pub const repXML: GPMReportType = GPMReportType(0i32);
pub const repHTML: GPMReportType = GPMReportType(1i32);
pub const repInfraXML: GPMReportType = GPMReportType(2i32);
pub const repInfraRefreshXML: GPMReportType = GPMReportType(3i32);
pub const repClientHealthXML: GPMReportType = GPMReportType(4i32);
pub const repClientHealthRefreshXML: GPMReportType = GPMReportType(5i32);
impl ::core::marker::Copy for GPMReportType {}
impl ::core::clone::Clone for GPMReportType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for GPMReportType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for GPMReportType {
    type Abi = Self;
}
impl ::core::fmt::Debug for GPMReportType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GPMReportType").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct GPMReportingOptions(pub i32);
pub const opReportLegacy: GPMReportingOptions = GPMReportingOptions(0i32);
pub const opReportComments: GPMReportingOptions = GPMReportingOptions(1i32);
impl ::core::marker::Copy for GPMReportingOptions {}
impl ::core::clone::Clone for GPMReportingOptions {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for GPMReportingOptions {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for GPMReportingOptions {
    type Abi = Self;
}
impl ::core::fmt::Debug for GPMReportingOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GPMReportingOptions").field(&self.0).finish()
    }
}
pub const GPMResult: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x92101ac0_9287_4206_a3b2_4bdb73d225f6);
pub const GPMSOM: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x32d93fac_450e_44cf_829c_8b22ff6bdae1);
pub const GPMSOMCollection: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x24c1f147_3720_4f5b_a9c3_06b4e4f931d2);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct GPMSOMType(pub i32);
pub const somSite: GPMSOMType = GPMSOMType(0i32);
pub const somDomain: GPMSOMType = GPMSOMType(1i32);
pub const somOU: GPMSOMType = GPMSOMType(2i32);
impl ::core::marker::Copy for GPMSOMType {}
impl ::core::clone::Clone for GPMSOMType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for GPMSOMType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for GPMSOMType {
    type Abi = Self;
}
impl ::core::fmt::Debug for GPMSOMType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GPMSOMType").field(&self.0).finish()
    }
}
pub const GPMSearchCriteria: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x17aaca26_5ce0_44fa_8cc0_5259e6483566);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct GPMSearchOperation(pub i32);
pub const opEquals: GPMSearchOperation = GPMSearchOperation(0i32);
pub const opContains: GPMSearchOperation = GPMSearchOperation(1i32);
pub const opNotContains: GPMSearchOperation = GPMSearchOperation(2i32);
pub const opNotEquals: GPMSearchOperation = GPMSearchOperation(3i32);
impl ::core::marker::Copy for GPMSearchOperation {}
impl ::core::clone::Clone for GPMSearchOperation {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for GPMSearchOperation {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for GPMSearchOperation {
    type Abi = Self;
}
impl ::core::fmt::Debug for GPMSearchOperation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GPMSearchOperation").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct GPMSearchProperty(pub i32);
pub const gpoPermissions: GPMSearchProperty = GPMSearchProperty(0i32);
pub const gpoEffectivePermissions: GPMSearchProperty = GPMSearchProperty(1i32);
pub const gpoDisplayName: GPMSearchProperty = GPMSearchProperty(2i32);
pub const gpoWMIFilter: GPMSearchProperty = GPMSearchProperty(3i32);
pub const gpoID: GPMSearchProperty = GPMSearchProperty(4i32);
pub const gpoComputerExtensions: GPMSearchProperty = GPMSearchProperty(5i32);
pub const gpoUserExtensions: GPMSearchProperty = GPMSearchProperty(6i32);
pub const somLinks: GPMSearchProperty = GPMSearchProperty(7i32);
pub const gpoDomain: GPMSearchProperty = GPMSearchProperty(8i32);
pub const backupMostRecent: GPMSearchProperty = GPMSearchProperty(9i32);
pub const starterGPOPermissions: GPMSearchProperty = GPMSearchProperty(10i32);
pub const starterGPOEffectivePermissions: GPMSearchProperty = GPMSearchProperty(11i32);
pub const starterGPODisplayName: GPMSearchProperty = GPMSearchProperty(12i32);
pub const starterGPOID: GPMSearchProperty = GPMSearchProperty(13i32);
pub const starterGPODomain: GPMSearchProperty = GPMSearchProperty(14i32);
impl ::core::marker::Copy for GPMSearchProperty {}
impl ::core::clone::Clone for GPMSearchProperty {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for GPMSearchProperty {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for GPMSearchProperty {
    type Abi = Self;
}
impl ::core::fmt::Debug for GPMSearchProperty {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GPMSearchProperty").field(&self.0).finish()
    }
}
pub const GPMSecurityInfo: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x547a5e8f_9162_4516_a4df_9ddb9686d846);
pub const GPMSitesContainer: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x229f5c42_852c_4b30_945f_c522be9bd386);
pub const GPMStarterGPOBackup: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x389e400a_d8ef_455b_a861_5f9ca34a6a02);
pub const GPMStarterGPOBackupCollection: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe75ea59d_1aeb_4cb5_a78a_281daa582406);
pub const GPMStarterGPOCollection: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x82f8aa8b_49ba_43b2_956e_3397f9b94c3a);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct GPMStarterGPOType(pub i32);
pub const typeSystem: GPMStarterGPOType = GPMStarterGPOType(0i32);
pub const typeCustom: GPMStarterGPOType = GPMStarterGPOType(1i32);
impl ::core::marker::Copy for GPMStarterGPOType {}
impl ::core::clone::Clone for GPMStarterGPOType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for GPMStarterGPOType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for GPMStarterGPOType {
    type Abi = Self;
}
impl ::core::fmt::Debug for GPMStarterGPOType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GPMStarterGPOType").field(&self.0).finish()
    }
}
pub const GPMStatusMessage: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4b77cc94_d255_409b_bc62_370881715a19);
pub const GPMStatusMsgCollection: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2824e4be_4bcc_4cac_9e60_0e3ed7f12496);
pub const GPMTemplate: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xecf1d454_71da_4e2f_a8c0_8185465911d9);
pub const GPMTrustee: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc54a700d_19b6_4211_bcb0_e8e2475e471e);
pub const GPMWMIFilter: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x626745d8_0dea_4062_bf60_cfc5b1ca1286);
pub const GPMWMIFilterCollection: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x74dc6d28_e820_47d6_a0b8_f08d93d7fa33);
pub const GPM_DONOTUSE_W2KDC: u32 = 2u32;
pub const GPM_DONOT_VALIDATEDC: u32 = 1u32;
pub const GPM_MIGRATIONTABLE_ONLY: u32 = 1u32;
pub const GPM_PROCESS_SECURITY: u32 = 2u32;
pub const GPM_USE_ANYDC: u32 = 1u32;
pub const GPM_USE_PDC: u32 = 0u32;
#[repr(C)]
pub struct GPOBROWSEINFO {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub hwndOwner: ::win32_foundation::HWND,
    pub lpTitle: ::windows_core::PWSTR,
    pub lpInitialOU: ::windows_core::PWSTR,
    pub lpDSPath: ::windows_core::PWSTR,
    pub dwDSPathSize: u32,
    pub lpName: ::windows_core::PWSTR,
    pub dwNameSize: u32,
    pub gpoType: GROUP_POLICY_OBJECT_TYPE,
    pub gpoHint: GROUP_POLICY_HINT_TYPE,
}
impl ::core::marker::Copy for GPOBROWSEINFO {}
impl ::core::clone::Clone for GPOBROWSEINFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for GPOBROWSEINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GPOBROWSEINFO").field("dwSize", &self.dwSize).field("dwFlags", &self.dwFlags).field("hwndOwner", &self.hwndOwner).field("lpTitle", &self.lpTitle).field("lpInitialOU", &self.lpInitialOU).field("lpDSPath", &self.lpDSPath).field("dwDSPathSize", &self.dwDSPathSize).field("lpName", &self.lpName).field("dwNameSize", &self.dwNameSize).field("gpoType", &self.gpoType).field("gpoHint", &self.gpoHint).finish()
    }
}
unsafe impl ::windows_core::Abi for GPOBROWSEINFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for GPOBROWSEINFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<GPOBROWSEINFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for GPOBROWSEINFO {}
impl ::core::default::Default for GPOBROWSEINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const GPO_BROWSE_DISABLENEW: u32 = 1u32;
pub const GPO_BROWSE_INITTOALL: u32 = 16u32;
pub const GPO_BROWSE_NOCOMPUTERS: u32 = 2u32;
pub const GPO_BROWSE_NODSGPOS: u32 = 4u32;
pub const GPO_BROWSE_NOUSERGPOS: u32 = 32u32;
pub const GPO_BROWSE_OPENBUTTON: u32 = 8u32;
pub const GPO_BROWSE_SENDAPPLYONEDIT: u32 = 64u32;
pub const GPO_FLAG_DISABLE: u32 = 1u32;
pub const GPO_FLAG_FORCE: u32 = 2u32;
pub const GPO_INFO_FLAG_ASYNC_FOREGROUND: u32 = 4096u32;
pub const GPO_INFO_FLAG_BACKGROUND: u32 = 16u32;
pub const GPO_INFO_FLAG_FORCED_REFRESH: u32 = 1024u32;
pub const GPO_INFO_FLAG_LINKTRANSITION: u32 = 256u32;
pub const GPO_INFO_FLAG_LOGRSOP_TRANSITION: u32 = 512u32;
pub const GPO_INFO_FLAG_MACHINE: u32 = 1u32;
pub const GPO_INFO_FLAG_NOCHANGES: u32 = 128u32;
pub const GPO_INFO_FLAG_SAFEMODE_BOOT: u32 = 2048u32;
pub const GPO_INFO_FLAG_SLOWLINK: u32 = 32u32;
pub const GPO_INFO_FLAG_VERBOSE: u32 = 64u32;
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct GPO_LINK(pub i32);
pub const GPLinkUnknown: GPO_LINK = GPO_LINK(0i32);
pub const GPLinkMachine: GPO_LINK = GPO_LINK(1i32);
pub const GPLinkSite: GPO_LINK = GPO_LINK(2i32);
pub const GPLinkDomain: GPO_LINK = GPO_LINK(3i32);
pub const GPLinkOrganizationalUnit: GPO_LINK = GPO_LINK(4i32);
impl ::core::marker::Copy for GPO_LINK {}
impl ::core::clone::Clone for GPO_LINK {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for GPO_LINK {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for GPO_LINK {
    type Abi = Self;
}
impl ::core::fmt::Debug for GPO_LINK {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GPO_LINK").field(&self.0).finish()
    }
}
pub const GPO_LIST_FLAG_MACHINE: u32 = 1u32;
pub const GPO_LIST_FLAG_NO_SECURITYFILTERS: u32 = 8u32;
pub const GPO_LIST_FLAG_NO_WMIFILTERS: u32 = 4u32;
pub const GPO_LIST_FLAG_SITEONLY: u32 = 2u32;
pub const GPO_OPEN_LOAD_REGISTRY: u32 = 1u32;
pub const GPO_OPEN_READ_ONLY: u32 = 2u32;
pub const GPO_OPTION_DISABLE_MACHINE: u32 = 2u32;
pub const GPO_OPTION_DISABLE_USER: u32 = 1u32;
pub const GPO_SECTION_MACHINE: u32 = 2u32;
pub const GPO_SECTION_ROOT: u32 = 0u32;
pub const GPO_SECTION_USER: u32 = 1u32;
pub const GP_DLLNAME: &str = "DllName";
pub const GP_ENABLEASYNCHRONOUSPROCESSING: &str = "EnableAsynchronousProcessing";
pub const GP_MAXNOGPOLISTCHANGESINTERVAL: &str = "MaxNoGPOListChangesInterval";
pub const GP_NOBACKGROUNDPOLICY: &str = "NoBackgroundPolicy";
pub const GP_NOGPOLISTCHANGES: &str = "NoGPOListChanges";
pub const GP_NOMACHINEPOLICY: &str = "NoMachinePolicy";
pub const GP_NOSLOWLINK: &str = "NoSlowLink";
pub const GP_NOTIFYLINKTRANSITION: &str = "NotifyLinkTransition";
pub const GP_NOUSERPOLICY: &str = "NoUserPolicy";
pub const GP_PERUSERLOCALSETTINGS: &str = "PerUserLocalSettings";
pub const GP_PROCESSGROUPPOLICY: &str = "ProcessGroupPolicy";
pub const GP_REQUIRESSUCCESSFULREGISTRY: &str = "RequiresSuccessfulRegistry";
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct GROUP_POLICY_HINT_TYPE(pub i32);
pub const GPHintUnknown: GROUP_POLICY_HINT_TYPE = GROUP_POLICY_HINT_TYPE(0i32);
pub const GPHintMachine: GROUP_POLICY_HINT_TYPE = GROUP_POLICY_HINT_TYPE(1i32);
pub const GPHintSite: GROUP_POLICY_HINT_TYPE = GROUP_POLICY_HINT_TYPE(2i32);
pub const GPHintDomain: GROUP_POLICY_HINT_TYPE = GROUP_POLICY_HINT_TYPE(3i32);
pub const GPHintOrganizationalUnit: GROUP_POLICY_HINT_TYPE = GROUP_POLICY_HINT_TYPE(4i32);
impl ::core::marker::Copy for GROUP_POLICY_HINT_TYPE {}
impl ::core::clone::Clone for GROUP_POLICY_HINT_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for GROUP_POLICY_HINT_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for GROUP_POLICY_HINT_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for GROUP_POLICY_HINT_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GROUP_POLICY_HINT_TYPE").field(&self.0).finish()
    }
}
#[repr(C)]
pub struct GROUP_POLICY_OBJECTA {
    pub dwOptions: u32,
    pub dwVersion: u32,
    pub lpDSPath: ::windows_core::PSTR,
    pub lpFileSysPath: ::windows_core::PSTR,
    pub lpDisplayName: ::windows_core::PSTR,
    pub szGPOName: [::win32_foundation::CHAR; 50],
    pub GPOLink: GPO_LINK,
    pub lParam: ::win32_foundation::LPARAM,
    pub pNext: *mut GROUP_POLICY_OBJECTA,
    pub pPrev: *mut GROUP_POLICY_OBJECTA,
    pub lpExtensions: ::windows_core::PSTR,
    pub lParam2: ::win32_foundation::LPARAM,
    pub lpLink: ::windows_core::PSTR,
}
impl ::core::marker::Copy for GROUP_POLICY_OBJECTA {}
impl ::core::clone::Clone for GROUP_POLICY_OBJECTA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for GROUP_POLICY_OBJECTA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GROUP_POLICY_OBJECTA")
            .field("dwOptions", &self.dwOptions)
            .field("dwVersion", &self.dwVersion)
            .field("lpDSPath", &self.lpDSPath)
            .field("lpFileSysPath", &self.lpFileSysPath)
            .field("lpDisplayName", &self.lpDisplayName)
            .field("szGPOName", &self.szGPOName)
            .field("GPOLink", &self.GPOLink)
            .field("lParam", &self.lParam)
            .field("pNext", &self.pNext)
            .field("pPrev", &self.pPrev)
            .field("lpExtensions", &self.lpExtensions)
            .field("lParam2", &self.lParam2)
            .field("lpLink", &self.lpLink)
            .finish()
    }
}
unsafe impl ::windows_core::Abi for GROUP_POLICY_OBJECTA {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for GROUP_POLICY_OBJECTA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<GROUP_POLICY_OBJECTA>()) == 0 }
    }
}
impl ::core::cmp::Eq for GROUP_POLICY_OBJECTA {}
impl ::core::default::Default for GROUP_POLICY_OBJECTA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct GROUP_POLICY_OBJECTW {
    pub dwOptions: u32,
    pub dwVersion: u32,
    pub lpDSPath: ::windows_core::PWSTR,
    pub lpFileSysPath: ::windows_core::PWSTR,
    pub lpDisplayName: ::windows_core::PWSTR,
    pub szGPOName: [u16; 50],
    pub GPOLink: GPO_LINK,
    pub lParam: ::win32_foundation::LPARAM,
    pub pNext: *mut GROUP_POLICY_OBJECTW,
    pub pPrev: *mut GROUP_POLICY_OBJECTW,
    pub lpExtensions: ::windows_core::PWSTR,
    pub lParam2: ::win32_foundation::LPARAM,
    pub lpLink: ::windows_core::PWSTR,
}
impl ::core::marker::Copy for GROUP_POLICY_OBJECTW {}
impl ::core::clone::Clone for GROUP_POLICY_OBJECTW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for GROUP_POLICY_OBJECTW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GROUP_POLICY_OBJECTW")
            .field("dwOptions", &self.dwOptions)
            .field("dwVersion", &self.dwVersion)
            .field("lpDSPath", &self.lpDSPath)
            .field("lpFileSysPath", &self.lpFileSysPath)
            .field("lpDisplayName", &self.lpDisplayName)
            .field("szGPOName", &self.szGPOName)
            .field("GPOLink", &self.GPOLink)
            .field("lParam", &self.lParam)
            .field("pNext", &self.pNext)
            .field("pPrev", &self.pPrev)
            .field("lpExtensions", &self.lpExtensions)
            .field("lParam2", &self.lParam2)
            .field("lpLink", &self.lpLink)
            .finish()
    }
}
unsafe impl ::windows_core::Abi for GROUP_POLICY_OBJECTW {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for GROUP_POLICY_OBJECTW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<GROUP_POLICY_OBJECTW>()) == 0 }
    }
}
impl ::core::cmp::Eq for GROUP_POLICY_OBJECTW {}
impl ::core::default::Default for GROUP_POLICY_OBJECTW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct GROUP_POLICY_OBJECT_TYPE(pub i32);
pub const GPOTypeLocal: GROUP_POLICY_OBJECT_TYPE = GROUP_POLICY_OBJECT_TYPE(0i32);
pub const GPOTypeRemote: GROUP_POLICY_OBJECT_TYPE = GROUP_POLICY_OBJECT_TYPE(1i32);
pub const GPOTypeDS: GROUP_POLICY_OBJECT_TYPE = GROUP_POLICY_OBJECT_TYPE(2i32);
pub const GPOTypeLocalUser: GROUP_POLICY_OBJECT_TYPE = GROUP_POLICY_OBJECT_TYPE(3i32);
pub const GPOTypeLocalGroup: GROUP_POLICY_OBJECT_TYPE = GROUP_POLICY_OBJECT_TYPE(4i32);
impl ::core::marker::Copy for GROUP_POLICY_OBJECT_TYPE {}
impl ::core::clone::Clone for GROUP_POLICY_OBJECT_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for GROUP_POLICY_OBJECT_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for GROUP_POLICY_OBJECT_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for GROUP_POLICY_OBJECT_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GROUP_POLICY_OBJECT_TYPE").field(&self.0).finish()
    }
}
#[inline]
pub unsafe fn GenerateGPNotification<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(bmachine: Param0, lpwszmgmtproduct: Param1, dwmgmtproductoptions: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GenerateGPNotification(bmachine: ::win32_foundation::BOOL, lpwszmgmtproduct: ::windows_core::PCWSTR, dwmgmtproductoptions: u32) -> u32;
        }
        ::core::mem::transmute(GenerateGPNotification(bmachine.into_param().abi(), lpwszmgmtproduct.into_param().abi(), ::core::mem::transmute(dwmgmtproductoptions)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn GetAppliedGPOListA<'a, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCSTR>, Param2: ::windows_core::IntoParam<'a, ::win32_foundation::PSID>>(dwflags: u32, pmachinename: Param1, psiduser: Param2, pguidextension: *const ::windows_core::GUID, ppgpolist: *mut *mut GROUP_POLICY_OBJECTA) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetAppliedGPOListA(dwflags: u32, pmachinename: ::windows_core::PCSTR, psiduser: ::win32_foundation::PSID, pguidextension: *const ::windows_core::GUID, ppgpolist: *mut *mut GROUP_POLICY_OBJECTA) -> u32;
        }
        ::core::mem::transmute(GetAppliedGPOListA(::core::mem::transmute(dwflags), pmachinename.into_param().abi(), psiduser.into_param().abi(), ::core::mem::transmute(pguidextension), ::core::mem::transmute(ppgpolist)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn GetAppliedGPOListW<'a, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param2: ::windows_core::IntoParam<'a, ::win32_foundation::PSID>>(dwflags: u32, pmachinename: Param1, psiduser: Param2, pguidextension: *const ::windows_core::GUID, ppgpolist: *mut *mut GROUP_POLICY_OBJECTW) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetAppliedGPOListW(dwflags: u32, pmachinename: ::windows_core::PCWSTR, psiduser: ::win32_foundation::PSID, pguidextension: *const ::windows_core::GUID, ppgpolist: *mut *mut GROUP_POLICY_OBJECTW) -> u32;
        }
        ::core::mem::transmute(GetAppliedGPOListW(::core::mem::transmute(dwflags), pmachinename.into_param().abi(), psiduser.into_param().abi(), ::core::mem::transmute(pguidextension), ::core::mem::transmute(ppgpolist)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn GetGPOListA<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HANDLE>, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCSTR>, Param2: ::windows_core::IntoParam<'a, ::windows_core::PCSTR>, Param3: ::windows_core::IntoParam<'a, ::windows_core::PCSTR>>(htoken: Param0, lpname: Param1, lphostname: Param2, lpcomputername: Param3, dwflags: u32, pgpolist: *mut *mut GROUP_POLICY_OBJECTA) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetGPOListA(htoken: ::win32_foundation::HANDLE, lpname: ::windows_core::PCSTR, lphostname: ::windows_core::PCSTR, lpcomputername: ::windows_core::PCSTR, dwflags: u32, pgpolist: *mut *mut GROUP_POLICY_OBJECTA) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(GetGPOListA(htoken.into_param().abi(), lpname.into_param().abi(), lphostname.into_param().abi(), lpcomputername.into_param().abi(), ::core::mem::transmute(dwflags), ::core::mem::transmute(pgpolist)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn GetGPOListW<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HANDLE>, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param2: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param3: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(htoken: Param0, lpname: Param1, lphostname: Param2, lpcomputername: Param3, dwflags: u32, pgpolist: *mut *mut GROUP_POLICY_OBJECTW) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetGPOListW(htoken: ::win32_foundation::HANDLE, lpname: ::windows_core::PCWSTR, lphostname: ::windows_core::PCWSTR, lpcomputername: ::windows_core::PCWSTR, dwflags: u32, pgpolist: *mut *mut GROUP_POLICY_OBJECTW) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(GetGPOListW(htoken.into_param().abi(), lpname.into_param().abi(), lphostname.into_param().abi(), lpcomputername.into_param().abi(), ::core::mem::transmute(dwflags), ::core::mem::transmute(pgpolist)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn GetLocalManagedApplicationData<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(productcode: Param0, displayname: *mut ::windows_core::PWSTR, supporturl: *mut ::windows_core::PWSTR) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetLocalManagedApplicationData(productcode: ::windows_core::PCWSTR, displayname: *mut ::windows_core::PWSTR, supporturl: *mut ::windows_core::PWSTR);
        }
        GetLocalManagedApplicationData(productcode.into_param().abi(), ::core::mem::transmute(displayname), ::core::mem::transmute(supporturl))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn GetLocalManagedApplications<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(buserapps: Param0, pdwapps: *mut u32, prglocalapps: *mut *mut LOCALMANAGEDAPPLICATION) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetLocalManagedApplications(buserapps: ::win32_foundation::BOOL, pdwapps: *mut u32, prglocalapps: *mut *mut LOCALMANAGEDAPPLICATION) -> u32;
        }
        ::core::mem::transmute(GetLocalManagedApplications(buserapps.into_param().abi(), ::core::mem::transmute(pdwapps), ::core::mem::transmute(prglocalapps)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "win32-ui")]
#[inline]
pub unsafe fn GetManagedApplicationCategories(dwreserved: u32, pappcategory: *mut ::win32_ui::Shell::APPCATEGORYINFOLIST) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetManagedApplicationCategories(dwreserved: u32, pappcategory: *mut ::win32_ui::Shell::APPCATEGORYINFOLIST) -> u32;
        }
        ::core::mem::transmute(GetManagedApplicationCategories(::core::mem::transmute(dwreserved), ::core::mem::transmute(pappcategory)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn GetManagedApplications(pcategory: *const ::windows_core::GUID, dwqueryflags: u32, dwinfolevel: u32, pdwapps: *mut u32, prgmanagedapps: *mut *mut MANAGEDAPPLICATION) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetManagedApplications(pcategory: *const ::windows_core::GUID, dwqueryflags: u32, dwinfolevel: u32, pdwapps: *mut u32, prgmanagedapps: *mut *mut MANAGEDAPPLICATION) -> u32;
        }
        ::core::mem::transmute(GetManagedApplications(::core::mem::transmute(pcategory), ::core::mem::transmute(dwqueryflags), ::core::mem::transmute(dwinfolevel), ::core::mem::transmute(pdwapps), ::core::mem::transmute(prgmanagedapps)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[repr(transparent)]
pub struct IGPEInformation(::windows_core::IUnknown);
impl IGPEInformation {
    pub unsafe fn GetName(&self, pszname: &mut [u16]) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetName)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(::windows_core::as_mut_ptr_or_null(pszname)), pszname.len() as _).ok()
    }
    pub unsafe fn GetDisplayName(&self, pszname: &mut [u16]) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetDisplayName)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(::windows_core::as_mut_ptr_or_null(pszname)), pszname.len() as _).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn GetRegistryKey(&self, dwsection: u32, hkey: *mut super::Registry::HKEY) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetRegistryKey)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwsection), ::core::mem::transmute(hkey)).ok()
    }
    pub unsafe fn GetDSPath(&self, dwsection: u32, pszpath: &mut [u16]) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetDSPath)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwsection), ::core::mem::transmute(::windows_core::as_mut_ptr_or_null(pszpath)), pszpath.len() as _).ok()
    }
    pub unsafe fn GetFileSysPath(&self, dwsection: u32, pszpath: &mut [u16]) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetFileSysPath)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwsection), ::core::mem::transmute(::windows_core::as_mut_ptr_or_null(pszpath)), pszpath.len() as _).ok()
    }
    pub unsafe fn GetOptions(&self, dwoptions: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetOptions)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwoptions)).ok()
    }
    pub unsafe fn GetType(&self, gpotype: *mut GROUP_POLICY_OBJECT_TYPE) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetType)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(gpotype)).ok()
    }
    pub unsafe fn GetHint(&self, gphint: *mut GROUP_POLICY_HINT_TYPE) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetHint)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(gphint)).ok()
    }
    pub unsafe fn PolicyChanged<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(&self, bmachine: Param0, badd: Param1, pguidextension: *mut ::windows_core::GUID, pguidsnapin: *mut ::windows_core::GUID) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).PolicyChanged)(::windows_core::Interface::as_raw(self), bmachine.into_param().abi(), badd.into_param().abi(), ::core::mem::transmute(pguidextension), ::core::mem::transmute(pguidsnapin)).ok()
    }
}
impl ::core::convert::From<IGPEInformation> for ::windows_core::IUnknown {
    fn from(value: IGPEInformation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IGPEInformation> for ::windows_core::IUnknown {
    fn from(value: &IGPEInformation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IGPEInformation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IGPEInformation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IGPEInformation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IGPEInformation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IGPEInformation {}
impl ::core::fmt::Debug for IGPEInformation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IGPEInformation").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IGPEInformation {
    type Vtable = IGPEInformation_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8fc0b735_a0e1_11d1_a7d3_0000f87571e3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGPEInformation_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub GetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszname: ::windows_core::PWSTR, cchmaxlength: i32) -> ::windows_core::HRESULT,
    pub GetDisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszname: ::windows_core::PWSTR, cchmaxlength: i32) -> ::windows_core::HRESULT,
    #[cfg(feature = "win32-system")]
    pub GetRegistryKey: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwsection: u32, hkey: *mut super::Registry::HKEY) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    GetRegistryKey: usize,
    pub GetDSPath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwsection: u32, pszpath: ::windows_core::PWSTR, cchmaxpath: i32) -> ::windows_core::HRESULT,
    pub GetFileSysPath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwsection: u32, pszpath: ::windows_core::PWSTR, cchmaxpath: i32) -> ::windows_core::HRESULT,
    pub GetOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwoptions: *mut u32) -> ::windows_core::HRESULT,
    pub GetType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, gpotype: *mut GROUP_POLICY_OBJECT_TYPE) -> ::windows_core::HRESULT,
    pub GetHint: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, gphint: *mut GROUP_POLICY_HINT_TYPE) -> ::windows_core::HRESULT,
    pub PolicyChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bmachine: ::win32_foundation::BOOL, badd: ::win32_foundation::BOOL, pguidextension: *mut ::windows_core::GUID, pguidsnapin: *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct IGPM(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl IGPM {
    #[cfg(feature = "win32-system")]
    pub unsafe fn GetDomain<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrdomain: Param0, bstrdomaincontroller: Param1, ldcflags: i32) -> ::windows_core::Result<IGPMDomain> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetDomain)(::windows_core::Interface::as_raw(self), bstrdomain.into_param().abi(), bstrdomaincontroller.into_param().abi(), ::core::mem::transmute(ldcflags), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IGPMDomain>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn GetBackupDir<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrbackupdir: Param0) -> ::windows_core::Result<IGPMBackupDir> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetBackupDir)(::windows_core::Interface::as_raw(self), bstrbackupdir.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IGPMBackupDir>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn GetSitesContainer<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param2: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrforest: Param0, bstrdomain: Param1, bstrdomaincontroller: Param2, ldcflags: i32) -> ::windows_core::Result<IGPMSitesContainer> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetSitesContainer)(::windows_core::Interface::as_raw(self), bstrforest.into_param().abi(), bstrdomain.into_param().abi(), bstrdomaincontroller.into_param().abi(), ::core::mem::transmute(ldcflags), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IGPMSitesContainer>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn GetRSOP<'a, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, gpmrsopmode: GPMRSOPMode, bstrnamespace: Param1, lflags: i32) -> ::windows_core::Result<IGPMRSOP> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetRSOP)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(gpmrsopmode), bstrnamespace.into_param().abi(), ::core::mem::transmute(lflags), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IGPMRSOP>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn CreatePermission<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrtrustee: Param0, perm: GPMPermissionType, binheritable: i16) -> ::windows_core::Result<IGPMPermission> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).CreatePermission)(::windows_core::Interface::as_raw(self), bstrtrustee.into_param().abi(), ::core::mem::transmute(perm), ::core::mem::transmute(binheritable), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IGPMPermission>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn CreateSearchCriteria(&self) -> ::windows_core::Result<IGPMSearchCriteria> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).CreateSearchCriteria)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IGPMSearchCriteria>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn CreateTrustee<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrtrustee: Param0) -> ::windows_core::Result<IGPMTrustee> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).CreateTrustee)(::windows_core::Interface::as_raw(self), bstrtrustee.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IGPMTrustee>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn GetClientSideExtensions(&self) -> ::windows_core::Result<IGPMCSECollection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetClientSideExtensions)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IGPMCSECollection>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn GetConstants(&self) -> ::windows_core::Result<IGPMConstants> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetConstants)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IGPMConstants>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn GetMigrationTable<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrmigrationtablepath: Param0) -> ::windows_core::Result<IGPMMigrationTable> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetMigrationTable)(::windows_core::Interface::as_raw(self), bstrmigrationtablepath.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IGPMMigrationTable>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn CreateMigrationTable(&self) -> ::windows_core::Result<IGPMMigrationTable> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).CreateMigrationTable)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IGPMMigrationTable>(result__)
    }
    pub unsafe fn InitializeReporting<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstradmpath: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).InitializeReporting)(::windows_core::Interface::as_raw(self), bstradmpath.into_param().abi()).ok()
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IGPM> for ::windows_core::IUnknown {
    fn from(value: IGPM) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IGPM> for ::windows_core::IUnknown {
    fn from(value: &IGPM) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IGPM {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IGPM {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IGPM> for super::Com::IDispatch {
    fn from(value: IGPM) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IGPM> for super::Com::IDispatch {
    fn from(value: &IGPM) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for IGPM {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a IGPM {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for IGPM {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for IGPM {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for IGPM {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for IGPM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IGPM").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for IGPM {
    type Vtable = IGPM_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf5fae809_3bd6_4da9_a65e_17665b41d763);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct IGPM_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    #[cfg(feature = "win32-system")]
    pub GetDomain: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrdomain: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, bstrdomaincontroller: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, ldcflags: i32, pigpmdomain: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    GetDomain: usize,
    #[cfg(feature = "win32-system")]
    pub GetBackupDir: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrbackupdir: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, pigpmbackupdir: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    GetBackupDir: usize,
    #[cfg(feature = "win32-system")]
    pub GetSitesContainer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrforest: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, bstrdomain: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, bstrdomaincontroller: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, ldcflags: i32, ppigpmsitescontainer: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    GetSitesContainer: usize,
    #[cfg(feature = "win32-system")]
    pub GetRSOP: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, gpmrsopmode: GPMRSOPMode, bstrnamespace: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, lflags: i32, ppigpmrsop: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    GetRSOP: usize,
    #[cfg(feature = "win32-system")]
    pub CreatePermission: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrtrustee: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, perm: GPMPermissionType, binheritable: i16, ppperm: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    CreatePermission: usize,
    #[cfg(feature = "win32-system")]
    pub CreateSearchCriteria: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppigpmsearchcriteria: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    CreateSearchCriteria: usize,
    #[cfg(feature = "win32-system")]
    pub CreateTrustee: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrtrustee: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, ppigpmtrustee: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    CreateTrustee: usize,
    #[cfg(feature = "win32-system")]
    pub GetClientSideExtensions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppigpmcsecollection: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    GetClientSideExtensions: usize,
    #[cfg(feature = "win32-system")]
    pub GetConstants: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppigpmconstants: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    GetConstants: usize,
    #[cfg(feature = "win32-system")]
    pub GetMigrationTable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrmigrationtablepath: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, ppmigrationtable: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    GetMigrationTable: usize,
    #[cfg(feature = "win32-system")]
    pub CreateMigrationTable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppmigrationtable: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    CreateMigrationTable: usize,
    pub InitializeReporting: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstradmpath: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct IGPM2(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl IGPM2 {
    #[cfg(feature = "win32-system")]
    pub unsafe fn GetDomain<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrdomain: Param0, bstrdomaincontroller: Param1, ldcflags: i32) -> ::windows_core::Result<IGPMDomain> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetDomain)(::windows_core::Interface::as_raw(self), bstrdomain.into_param().abi(), bstrdomaincontroller.into_param().abi(), ::core::mem::transmute(ldcflags), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IGPMDomain>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn GetBackupDir<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrbackupdir: Param0) -> ::windows_core::Result<IGPMBackupDir> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetBackupDir)(::windows_core::Interface::as_raw(self), bstrbackupdir.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IGPMBackupDir>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn GetSitesContainer<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param2: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrforest: Param0, bstrdomain: Param1, bstrdomaincontroller: Param2, ldcflags: i32) -> ::windows_core::Result<IGPMSitesContainer> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetSitesContainer)(::windows_core::Interface::as_raw(self), bstrforest.into_param().abi(), bstrdomain.into_param().abi(), bstrdomaincontroller.into_param().abi(), ::core::mem::transmute(ldcflags), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IGPMSitesContainer>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn GetRSOP<'a, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, gpmrsopmode: GPMRSOPMode, bstrnamespace: Param1, lflags: i32) -> ::windows_core::Result<IGPMRSOP> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetRSOP)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(gpmrsopmode), bstrnamespace.into_param().abi(), ::core::mem::transmute(lflags), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IGPMRSOP>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn CreatePermission<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrtrustee: Param0, perm: GPMPermissionType, binheritable: i16) -> ::windows_core::Result<IGPMPermission> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.CreatePermission)(::windows_core::Interface::as_raw(self), bstrtrustee.into_param().abi(), ::core::mem::transmute(perm), ::core::mem::transmute(binheritable), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IGPMPermission>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn CreateSearchCriteria(&self) -> ::windows_core::Result<IGPMSearchCriteria> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.CreateSearchCriteria)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IGPMSearchCriteria>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn CreateTrustee<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrtrustee: Param0) -> ::windows_core::Result<IGPMTrustee> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.CreateTrustee)(::windows_core::Interface::as_raw(self), bstrtrustee.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IGPMTrustee>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn GetClientSideExtensions(&self) -> ::windows_core::Result<IGPMCSECollection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetClientSideExtensions)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IGPMCSECollection>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn GetConstants(&self) -> ::windows_core::Result<IGPMConstants> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetConstants)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IGPMConstants>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn GetMigrationTable<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrmigrationtablepath: Param0) -> ::windows_core::Result<IGPMMigrationTable> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetMigrationTable)(::windows_core::Interface::as_raw(self), bstrmigrationtablepath.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IGPMMigrationTable>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn CreateMigrationTable(&self) -> ::windows_core::Result<IGPMMigrationTable> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.CreateMigrationTable)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IGPMMigrationTable>(result__)
    }
    pub unsafe fn InitializeReporting<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstradmpath: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.InitializeReporting)(::windows_core::Interface::as_raw(self), bstradmpath.into_param().abi()).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn GetBackupDirEx<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrbackupdir: Param0, backupdirtype: GPMBackupType) -> ::windows_core::Result<IGPMBackupDirEx> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetBackupDirEx)(::windows_core::Interface::as_raw(self), bstrbackupdir.into_param().abi(), ::core::mem::transmute(backupdirtype), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IGPMBackupDirEx>(result__)
    }
    pub unsafe fn InitializeReportingEx<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstradmpath: Param0, reportingoptions: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).InitializeReportingEx)(::windows_core::Interface::as_raw(self), bstradmpath.into_param().abi(), ::core::mem::transmute(reportingoptions)).ok()
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IGPM2> for ::windows_core::IUnknown {
    fn from(value: IGPM2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IGPM2> for ::windows_core::IUnknown {
    fn from(value: &IGPM2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IGPM2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IGPM2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IGPM2> for super::Com::IDispatch {
    fn from(value: IGPM2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IGPM2> for super::Com::IDispatch {
    fn from(value: &IGPM2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for IGPM2 {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a IGPM2 {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IGPM2> for IGPM {
    fn from(value: IGPM2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IGPM2> for IGPM {
    fn from(value: &IGPM2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, IGPM> for IGPM2 {
    fn into_param(self) -> ::windows_core::Param<'a, IGPM> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, IGPM> for &'a IGPM2 {
    fn into_param(self) -> ::windows_core::Param<'a, IGPM> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for IGPM2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for IGPM2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for IGPM2 {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for IGPM2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IGPM2").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for IGPM2 {
    type Vtable = IGPM2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x00238f8a_3d86_41ac_8f5e_06a6638a634a);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct IGPM2_Vtbl {
    pub base__: IGPM_Vtbl,
    #[cfg(feature = "win32-system")]
    pub GetBackupDirEx: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrbackupdir: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, backupdirtype: GPMBackupType, ppigpmbackupdirex: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    GetBackupDirEx: usize,
    pub InitializeReportingEx: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstradmpath: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, reportingoptions: i32) -> ::windows_core::HRESULT,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct IGPMAsyncCancel(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl IGPMAsyncCancel {
    pub unsafe fn Cancel(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Cancel)(::windows_core::Interface::as_raw(self)).ok()
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IGPMAsyncCancel> for ::windows_core::IUnknown {
    fn from(value: IGPMAsyncCancel) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IGPMAsyncCancel> for ::windows_core::IUnknown {
    fn from(value: &IGPMAsyncCancel) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IGPMAsyncCancel {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IGPMAsyncCancel {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IGPMAsyncCancel> for super::Com::IDispatch {
    fn from(value: IGPMAsyncCancel) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IGPMAsyncCancel> for super::Com::IDispatch {
    fn from(value: &IGPMAsyncCancel) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for IGPMAsyncCancel {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a IGPMAsyncCancel {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for IGPMAsyncCancel {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for IGPMAsyncCancel {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for IGPMAsyncCancel {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for IGPMAsyncCancel {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IGPMAsyncCancel").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for IGPMAsyncCancel {
    type Vtable = IGPMAsyncCancel_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xddc67754_be67_4541_8166_f48166868c9c);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct IGPMAsyncCancel_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Cancel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct IGPMAsyncProgress(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl IGPMAsyncProgress {
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn Status<'a, Param4: ::windows_core::IntoParam<'a, IGPMStatusMsgCollection>>(&self, lprogressnumerator: i32, lprogressdenominator: i32, hrstatus: ::windows_core::HRESULT, presult: *const super::Com::VARIANT, ppigpmstatusmsgcollection: Param4) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Status)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lprogressnumerator), ::core::mem::transmute(lprogressdenominator), ::core::mem::transmute(hrstatus), ::core::mem::transmute(presult), ppigpmstatusmsgcollection.into_param().abi()).ok()
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IGPMAsyncProgress> for ::windows_core::IUnknown {
    fn from(value: IGPMAsyncProgress) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IGPMAsyncProgress> for ::windows_core::IUnknown {
    fn from(value: &IGPMAsyncProgress) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IGPMAsyncProgress {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IGPMAsyncProgress {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IGPMAsyncProgress> for super::Com::IDispatch {
    fn from(value: IGPMAsyncProgress) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IGPMAsyncProgress> for super::Com::IDispatch {
    fn from(value: &IGPMAsyncProgress) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for IGPMAsyncProgress {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a IGPMAsyncProgress {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for IGPMAsyncProgress {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for IGPMAsyncProgress {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for IGPMAsyncProgress {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for IGPMAsyncProgress {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IGPMAsyncProgress").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for IGPMAsyncProgress {
    type Vtable = IGPMAsyncProgress_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6aac29f8_5948_4324_bf70_423818942dbc);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct IGPMAsyncProgress_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lprogressnumerator: i32, lprogressdenominator: i32, hrstatus: ::windows_core::HRESULT, presult: *const super::Com::VARIANT, ppigpmstatusmsgcollection: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    Status: usize,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct IGPMBackup(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl IGPMBackup {
    pub unsafe fn ID(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).ID)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn GPOID(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).GPOID)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn GPODomain(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).GPODomain)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn GPODisplayName(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).GPODisplayName)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn Timestamp(&self) -> ::windows_core::Result<f64> {
        let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
        (::windows_core::Interface::vtable(self).Timestamp)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<f64>(result__)
    }
    pub unsafe fn Comment(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).Comment)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn BackupDir(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).BackupDir)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn Delete(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Delete)(::windows_core::Interface::as_raw(self)).ok()
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn GenerateReport(&self, gpmreporttype: GPMReportType, pvargpmprogress: *const super::Com::VARIANT, pvargpmcancel: *mut super::Com::VARIANT, ppigpmresult: *mut ::core::option::Option<IGPMResult>) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GenerateReport)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(gpmreporttype), ::core::mem::transmute(pvargpmprogress), ::core::mem::transmute(pvargpmcancel), ::core::mem::transmute(ppigpmresult)).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn GenerateReportToFile<'a, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, gpmreporttype: GPMReportType, bstrtargetfilepath: Param1) -> ::windows_core::Result<IGPMResult> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GenerateReportToFile)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(gpmreporttype), bstrtargetfilepath.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IGPMResult>(result__)
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IGPMBackup> for ::windows_core::IUnknown {
    fn from(value: IGPMBackup) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IGPMBackup> for ::windows_core::IUnknown {
    fn from(value: &IGPMBackup) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IGPMBackup {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IGPMBackup {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IGPMBackup> for super::Com::IDispatch {
    fn from(value: IGPMBackup) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IGPMBackup> for super::Com::IDispatch {
    fn from(value: &IGPMBackup) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for IGPMBackup {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a IGPMBackup {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for IGPMBackup {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for IGPMBackup {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for IGPMBackup {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for IGPMBackup {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IGPMBackup").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for IGPMBackup {
    type Vtable = IGPMBackup_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd8a16a35_3b0d_416b_8d02_4df6f95a7119);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct IGPMBackup_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub ID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub GPOID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub GPODomain: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub GPODisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub Timestamp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut f64) -> ::windows_core::HRESULT,
    pub Comment: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub BackupDir: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub Delete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub GenerateReport: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, gpmreporttype: GPMReportType, pvargpmprogress: *const super::Com::VARIANT, pvargpmcancel: *mut super::Com::VARIANT, ppigpmresult: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    GenerateReport: usize,
    #[cfg(feature = "win32-system")]
    pub GenerateReportToFile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, gpmreporttype: GPMReportType, bstrtargetfilepath: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, ppigpmresult: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    GenerateReportToFile: usize,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct IGPMBackupCollection(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl IGPMBackupCollection {
    pub unsafe fn Count(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).Count)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn get_Item(&self, lindex: i32) -> ::windows_core::Result<super::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).get_Item)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lindex), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::VARIANT>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn _NewEnum(&self) -> ::windows_core::Result<super::Ole::IEnumVARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self)._NewEnum)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Ole::IEnumVARIANT>(result__)
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IGPMBackupCollection> for ::windows_core::IUnknown {
    fn from(value: IGPMBackupCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IGPMBackupCollection> for ::windows_core::IUnknown {
    fn from(value: &IGPMBackupCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IGPMBackupCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IGPMBackupCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IGPMBackupCollection> for super::Com::IDispatch {
    fn from(value: IGPMBackupCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IGPMBackupCollection> for super::Com::IDispatch {
    fn from(value: &IGPMBackupCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for IGPMBackupCollection {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a IGPMBackupCollection {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for IGPMBackupCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for IGPMBackupCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for IGPMBackupCollection {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for IGPMBackupCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IGPMBackupCollection").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for IGPMBackupCollection {
    type Vtable = IGPMBackupCollection_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc786fc0f_26d8_4bab_a745_39ca7e800cac);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct IGPMBackupCollection_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub get_Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lindex: i32, pval: *mut super::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    get_Item: usize,
    #[cfg(feature = "win32-system")]
    pub _NewEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppigpmbackup: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    _NewEnum: usize,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct IGPMBackupDir(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl IGPMBackupDir {
    pub unsafe fn BackupDirectory(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).BackupDirectory)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn GetBackup<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrid: Param0) -> ::windows_core::Result<IGPMBackup> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetBackup)(::windows_core::Interface::as_raw(self), bstrid.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IGPMBackup>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn SearchBackups<'a, Param0: ::windows_core::IntoParam<'a, IGPMSearchCriteria>>(&self, pigpmsearchcriteria: Param0) -> ::windows_core::Result<IGPMBackupCollection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).SearchBackups)(::windows_core::Interface::as_raw(self), pigpmsearchcriteria.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IGPMBackupCollection>(result__)
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IGPMBackupDir> for ::windows_core::IUnknown {
    fn from(value: IGPMBackupDir) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IGPMBackupDir> for ::windows_core::IUnknown {
    fn from(value: &IGPMBackupDir) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IGPMBackupDir {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IGPMBackupDir {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IGPMBackupDir> for super::Com::IDispatch {
    fn from(value: IGPMBackupDir) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IGPMBackupDir> for super::Com::IDispatch {
    fn from(value: &IGPMBackupDir) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for IGPMBackupDir {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a IGPMBackupDir {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for IGPMBackupDir {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for IGPMBackupDir {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for IGPMBackupDir {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for IGPMBackupDir {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IGPMBackupDir").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for IGPMBackupDir {
    type Vtable = IGPMBackupDir_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb1568bed_0a93_4acc_810f_afe7081019b9);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct IGPMBackupDir_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub BackupDirectory: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    #[cfg(feature = "win32-system")]
    pub GetBackup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrid: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, ppbackup: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    GetBackup: usize,
    #[cfg(feature = "win32-system")]
    pub SearchBackups: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pigpmsearchcriteria: ::windows_core::RawPtr, ppigpmbackupcollection: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    SearchBackups: usize,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct IGPMBackupDirEx(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl IGPMBackupDirEx {
    pub unsafe fn BackupDir(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).BackupDir)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn BackupType(&self) -> ::windows_core::Result<GPMBackupType> {
        let mut result__ = ::core::mem::MaybeUninit::<GPMBackupType>::zeroed();
        (::windows_core::Interface::vtable(self).BackupType)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<GPMBackupType>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn GetBackup<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrid: Param0) -> ::windows_core::Result<super::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).GetBackup)(::windows_core::Interface::as_raw(self), bstrid.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn SearchBackups<'a, Param0: ::windows_core::IntoParam<'a, IGPMSearchCriteria>>(&self, pigpmsearchcriteria: Param0) -> ::windows_core::Result<super::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).SearchBackups)(::windows_core::Interface::as_raw(self), pigpmsearchcriteria.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::VARIANT>(result__)
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IGPMBackupDirEx> for ::windows_core::IUnknown {
    fn from(value: IGPMBackupDirEx) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IGPMBackupDirEx> for ::windows_core::IUnknown {
    fn from(value: &IGPMBackupDirEx) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IGPMBackupDirEx {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IGPMBackupDirEx {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IGPMBackupDirEx> for super::Com::IDispatch {
    fn from(value: IGPMBackupDirEx) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IGPMBackupDirEx> for super::Com::IDispatch {
    fn from(value: &IGPMBackupDirEx) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for IGPMBackupDirEx {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a IGPMBackupDirEx {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for IGPMBackupDirEx {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for IGPMBackupDirEx {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for IGPMBackupDirEx {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for IGPMBackupDirEx {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IGPMBackupDirEx").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for IGPMBackupDirEx {
    type Vtable = IGPMBackupDirEx_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf8dc55ed_3ba0_4864_aad4_d365189ee1d5);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct IGPMBackupDirEx_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub BackupDir: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrbackupdir: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub BackupType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pgpmbackuptype: *mut GPMBackupType) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub GetBackup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrid: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, pvarbackup: *mut super::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    GetBackup: usize,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub SearchBackups: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pigpmsearchcriteria: ::windows_core::RawPtr, pvarbackupcollection: *mut super::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    SearchBackups: usize,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct IGPMCSECollection(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl IGPMCSECollection {
    pub unsafe fn Count(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).Count)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn get_Item(&self, lindex: i32) -> ::windows_core::Result<super::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).get_Item)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lindex), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::VARIANT>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn _NewEnum(&self) -> ::windows_core::Result<super::Ole::IEnumVARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self)._NewEnum)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Ole::IEnumVARIANT>(result__)
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IGPMCSECollection> for ::windows_core::IUnknown {
    fn from(value: IGPMCSECollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IGPMCSECollection> for ::windows_core::IUnknown {
    fn from(value: &IGPMCSECollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IGPMCSECollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IGPMCSECollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IGPMCSECollection> for super::Com::IDispatch {
    fn from(value: IGPMCSECollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IGPMCSECollection> for super::Com::IDispatch {
    fn from(value: &IGPMCSECollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for IGPMCSECollection {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a IGPMCSECollection {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for IGPMCSECollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for IGPMCSECollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for IGPMCSECollection {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for IGPMCSECollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IGPMCSECollection").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for IGPMCSECollection {
    type Vtable = IGPMCSECollection_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2e52a97d_0a4a_4a6f_85db_201622455da0);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct IGPMCSECollection_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub get_Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lindex: i32, pval: *mut super::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    get_Item: usize,
    #[cfg(feature = "win32-system")]
    pub _NewEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppigpmcses: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    _NewEnum: usize,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct IGPMClientSideExtension(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl IGPMClientSideExtension {
    pub unsafe fn ID(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).ID)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn DisplayName(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).DisplayName)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn IsUserEnabled(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).IsUserEnabled)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn IsComputerEnabled(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).IsComputerEnabled)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IGPMClientSideExtension> for ::windows_core::IUnknown {
    fn from(value: IGPMClientSideExtension) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IGPMClientSideExtension> for ::windows_core::IUnknown {
    fn from(value: &IGPMClientSideExtension) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IGPMClientSideExtension {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IGPMClientSideExtension {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IGPMClientSideExtension> for super::Com::IDispatch {
    fn from(value: IGPMClientSideExtension) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IGPMClientSideExtension> for super::Com::IDispatch {
    fn from(value: &IGPMClientSideExtension) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for IGPMClientSideExtension {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a IGPMClientSideExtension {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for IGPMClientSideExtension {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for IGPMClientSideExtension {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for IGPMClientSideExtension {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for IGPMClientSideExtension {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IGPMClientSideExtension").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for IGPMClientSideExtension {
    type Vtable = IGPMClientSideExtension_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x69da7488_b8db_415e_9266_901be4d49928);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct IGPMClientSideExtension_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub ID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub DisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub IsUserEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvbenabled: *mut i16) -> ::windows_core::HRESULT,
    pub IsComputerEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvbenabled: *mut i16) -> ::windows_core::HRESULT,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct IGPMConstants(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl IGPMConstants {
    pub unsafe fn PermGPOApply(&self) -> ::windows_core::Result<GPMPermissionType> {
        let mut result__ = ::core::mem::MaybeUninit::<GPMPermissionType>::zeroed();
        (::windows_core::Interface::vtable(self).PermGPOApply)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<GPMPermissionType>(result__)
    }
    pub unsafe fn PermGPORead(&self) -> ::windows_core::Result<GPMPermissionType> {
        let mut result__ = ::core::mem::MaybeUninit::<GPMPermissionType>::zeroed();
        (::windows_core::Interface::vtable(self).PermGPORead)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<GPMPermissionType>(result__)
    }
    pub unsafe fn PermGPOEdit(&self) -> ::windows_core::Result<GPMPermissionType> {
        let mut result__ = ::core::mem::MaybeUninit::<GPMPermissionType>::zeroed();
        (::windows_core::Interface::vtable(self).PermGPOEdit)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<GPMPermissionType>(result__)
    }
    pub unsafe fn PermGPOEditSecurityAndDelete(&self) -> ::windows_core::Result<GPMPermissionType> {
        let mut result__ = ::core::mem::MaybeUninit::<GPMPermissionType>::zeroed();
        (::windows_core::Interface::vtable(self).PermGPOEditSecurityAndDelete)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<GPMPermissionType>(result__)
    }
    pub unsafe fn PermGPOCustom(&self) -> ::windows_core::Result<GPMPermissionType> {
        let mut result__ = ::core::mem::MaybeUninit::<GPMPermissionType>::zeroed();
        (::windows_core::Interface::vtable(self).PermGPOCustom)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<GPMPermissionType>(result__)
    }
    pub unsafe fn PermWMIFilterEdit(&self) -> ::windows_core::Result<GPMPermissionType> {
        let mut result__ = ::core::mem::MaybeUninit::<GPMPermissionType>::zeroed();
        (::windows_core::Interface::vtable(self).PermWMIFilterEdit)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<GPMPermissionType>(result__)
    }
    pub unsafe fn PermWMIFilterFullControl(&self) -> ::windows_core::Result<GPMPermissionType> {
        let mut result__ = ::core::mem::MaybeUninit::<GPMPermissionType>::zeroed();
        (::windows_core::Interface::vtable(self).PermWMIFilterFullControl)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<GPMPermissionType>(result__)
    }
    pub unsafe fn PermWMIFilterCustom(&self) -> ::windows_core::Result<GPMPermissionType> {
        let mut result__ = ::core::mem::MaybeUninit::<GPMPermissionType>::zeroed();
        (::windows_core::Interface::vtable(self).PermWMIFilterCustom)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<GPMPermissionType>(result__)
    }
    pub unsafe fn PermSOMLink(&self) -> ::windows_core::Result<GPMPermissionType> {
        let mut result__ = ::core::mem::MaybeUninit::<GPMPermissionType>::zeroed();
        (::windows_core::Interface::vtable(self).PermSOMLink)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<GPMPermissionType>(result__)
    }
    pub unsafe fn PermSOMLogging(&self) -> ::windows_core::Result<GPMPermissionType> {
        let mut result__ = ::core::mem::MaybeUninit::<GPMPermissionType>::zeroed();
        (::windows_core::Interface::vtable(self).PermSOMLogging)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<GPMPermissionType>(result__)
    }
    pub unsafe fn PermSOMPlanning(&self) -> ::windows_core::Result<GPMPermissionType> {
        let mut result__ = ::core::mem::MaybeUninit::<GPMPermissionType>::zeroed();
        (::windows_core::Interface::vtable(self).PermSOMPlanning)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<GPMPermissionType>(result__)
    }
    pub unsafe fn PermSOMGPOCreate(&self) -> ::windows_core::Result<GPMPermissionType> {
        let mut result__ = ::core::mem::MaybeUninit::<GPMPermissionType>::zeroed();
        (::windows_core::Interface::vtable(self).PermSOMGPOCreate)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<GPMPermissionType>(result__)
    }
    pub unsafe fn PermSOMWMICreate(&self) -> ::windows_core::Result<GPMPermissionType> {
        let mut result__ = ::core::mem::MaybeUninit::<GPMPermissionType>::zeroed();
        (::windows_core::Interface::vtable(self).PermSOMWMICreate)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<GPMPermissionType>(result__)
    }
    pub unsafe fn PermSOMWMIFullControl(&self) -> ::windows_core::Result<GPMPermissionType> {
        let mut result__ = ::core::mem::MaybeUninit::<GPMPermissionType>::zeroed();
        (::windows_core::Interface::vtable(self).PermSOMWMIFullControl)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<GPMPermissionType>(result__)
    }
    pub unsafe fn SearchPropertyGPOPermissions(&self) -> ::windows_core::Result<GPMSearchProperty> {
        let mut result__ = ::core::mem::MaybeUninit::<GPMSearchProperty>::zeroed();
        (::windows_core::Interface::vtable(self).SearchPropertyGPOPermissions)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<GPMSearchProperty>(result__)
    }
    pub unsafe fn SearchPropertyGPOEffectivePermissions(&self) -> ::windows_core::Result<GPMSearchProperty> {
        let mut result__ = ::core::mem::MaybeUninit::<GPMSearchProperty>::zeroed();
        (::windows_core::Interface::vtable(self).SearchPropertyGPOEffectivePermissions)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<GPMSearchProperty>(result__)
    }
    pub unsafe fn SearchPropertyGPODisplayName(&self) -> ::windows_core::Result<GPMSearchProperty> {
        let mut result__ = ::core::mem::MaybeUninit::<GPMSearchProperty>::zeroed();
        (::windows_core::Interface::vtable(self).SearchPropertyGPODisplayName)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<GPMSearchProperty>(result__)
    }
    pub unsafe fn SearchPropertyGPOWMIFilter(&self) -> ::windows_core::Result<GPMSearchProperty> {
        let mut result__ = ::core::mem::MaybeUninit::<GPMSearchProperty>::zeroed();
        (::windows_core::Interface::vtable(self).SearchPropertyGPOWMIFilter)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<GPMSearchProperty>(result__)
    }
    pub unsafe fn SearchPropertyGPOID(&self) -> ::windows_core::Result<GPMSearchProperty> {
        let mut result__ = ::core::mem::MaybeUninit::<GPMSearchProperty>::zeroed();
        (::windows_core::Interface::vtable(self).SearchPropertyGPOID)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<GPMSearchProperty>(result__)
    }
    pub unsafe fn SearchPropertyGPOComputerExtensions(&self) -> ::windows_core::Result<GPMSearchProperty> {
        let mut result__ = ::core::mem::MaybeUninit::<GPMSearchProperty>::zeroed();
        (::windows_core::Interface::vtable(self).SearchPropertyGPOComputerExtensions)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<GPMSearchProperty>(result__)
    }
    pub unsafe fn SearchPropertyGPOUserExtensions(&self) -> ::windows_core::Result<GPMSearchProperty> {
        let mut result__ = ::core::mem::MaybeUninit::<GPMSearchProperty>::zeroed();
        (::windows_core::Interface::vtable(self).SearchPropertyGPOUserExtensions)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<GPMSearchProperty>(result__)
    }
    pub unsafe fn SearchPropertySOMLinks(&self) -> ::windows_core::Result<GPMSearchProperty> {
        let mut result__ = ::core::mem::MaybeUninit::<GPMSearchProperty>::zeroed();
        (::windows_core::Interface::vtable(self).SearchPropertySOMLinks)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<GPMSearchProperty>(result__)
    }
    pub unsafe fn SearchPropertyGPODomain(&self) -> ::windows_core::Result<GPMSearchProperty> {
        let mut result__ = ::core::mem::MaybeUninit::<GPMSearchProperty>::zeroed();
        (::windows_core::Interface::vtable(self).SearchPropertyGPODomain)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<GPMSearchProperty>(result__)
    }
    pub unsafe fn SearchPropertyBackupMostRecent(&self) -> ::windows_core::Result<GPMSearchProperty> {
        let mut result__ = ::core::mem::MaybeUninit::<GPMSearchProperty>::zeroed();
        (::windows_core::Interface::vtable(self).SearchPropertyBackupMostRecent)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<GPMSearchProperty>(result__)
    }
    pub unsafe fn SearchOpEquals(&self) -> ::windows_core::Result<GPMSearchOperation> {
        let mut result__ = ::core::mem::MaybeUninit::<GPMSearchOperation>::zeroed();
        (::windows_core::Interface::vtable(self).SearchOpEquals)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<GPMSearchOperation>(result__)
    }
    pub unsafe fn SearchOpContains(&self) -> ::windows_core::Result<GPMSearchOperation> {
        let mut result__ = ::core::mem::MaybeUninit::<GPMSearchOperation>::zeroed();
        (::windows_core::Interface::vtable(self).SearchOpContains)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<GPMSearchOperation>(result__)
    }
    pub unsafe fn SearchOpNotContains(&self) -> ::windows_core::Result<GPMSearchOperation> {
        let mut result__ = ::core::mem::MaybeUninit::<GPMSearchOperation>::zeroed();
        (::windows_core::Interface::vtable(self).SearchOpNotContains)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<GPMSearchOperation>(result__)
    }
    pub unsafe fn SearchOpNotEquals(&self) -> ::windows_core::Result<GPMSearchOperation> {
        let mut result__ = ::core::mem::MaybeUninit::<GPMSearchOperation>::zeroed();
        (::windows_core::Interface::vtable(self).SearchOpNotEquals)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<GPMSearchOperation>(result__)
    }
    pub unsafe fn UsePDC(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).UsePDC)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn UseAnyDC(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).UseAnyDC)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn DoNotUseW2KDC(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).DoNotUseW2KDC)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SOMSite(&self) -> ::windows_core::Result<GPMSOMType> {
        let mut result__ = ::core::mem::MaybeUninit::<GPMSOMType>::zeroed();
        (::windows_core::Interface::vtable(self).SOMSite)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<GPMSOMType>(result__)
    }
    pub unsafe fn SOMDomain(&self) -> ::windows_core::Result<GPMSOMType> {
        let mut result__ = ::core::mem::MaybeUninit::<GPMSOMType>::zeroed();
        (::windows_core::Interface::vtable(self).SOMDomain)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<GPMSOMType>(result__)
    }
    pub unsafe fn SOMOU(&self) -> ::windows_core::Result<GPMSOMType> {
        let mut result__ = ::core::mem::MaybeUninit::<GPMSOMType>::zeroed();
        (::windows_core::Interface::vtable(self).SOMOU)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<GPMSOMType>(result__)
    }
    pub unsafe fn get_SecurityFlags(&self, vbowner: i16, vbgroup: i16, vbdacl: i16, vbsacl: i16) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).get_SecurityFlags)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(vbowner), ::core::mem::transmute(vbgroup), ::core::mem::transmute(vbdacl), ::core::mem::transmute(vbsacl), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn DoNotValidateDC(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).DoNotValidateDC)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn ReportHTML(&self) -> ::windows_core::Result<GPMReportType> {
        let mut result__ = ::core::mem::MaybeUninit::<GPMReportType>::zeroed();
        (::windows_core::Interface::vtable(self).ReportHTML)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<GPMReportType>(result__)
    }
    pub unsafe fn ReportXML(&self) -> ::windows_core::Result<GPMReportType> {
        let mut result__ = ::core::mem::MaybeUninit::<GPMReportType>::zeroed();
        (::windows_core::Interface::vtable(self).ReportXML)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<GPMReportType>(result__)
    }
    pub unsafe fn RSOPModeUnknown(&self) -> ::windows_core::Result<GPMRSOPMode> {
        let mut result__ = ::core::mem::MaybeUninit::<GPMRSOPMode>::zeroed();
        (::windows_core::Interface::vtable(self).RSOPModeUnknown)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<GPMRSOPMode>(result__)
    }
    pub unsafe fn RSOPModePlanning(&self) -> ::windows_core::Result<GPMRSOPMode> {
        let mut result__ = ::core::mem::MaybeUninit::<GPMRSOPMode>::zeroed();
        (::windows_core::Interface::vtable(self).RSOPModePlanning)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<GPMRSOPMode>(result__)
    }
    pub unsafe fn RSOPModeLogging(&self) -> ::windows_core::Result<GPMRSOPMode> {
        let mut result__ = ::core::mem::MaybeUninit::<GPMRSOPMode>::zeroed();
        (::windows_core::Interface::vtable(self).RSOPModeLogging)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<GPMRSOPMode>(result__)
    }
    pub unsafe fn EntryTypeUser(&self) -> ::windows_core::Result<GPMEntryType> {
        let mut result__ = ::core::mem::MaybeUninit::<GPMEntryType>::zeroed();
        (::windows_core::Interface::vtable(self).EntryTypeUser)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<GPMEntryType>(result__)
    }
    pub unsafe fn EntryTypeComputer(&self) -> ::windows_core::Result<GPMEntryType> {
        let mut result__ = ::core::mem::MaybeUninit::<GPMEntryType>::zeroed();
        (::windows_core::Interface::vtable(self).EntryTypeComputer)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<GPMEntryType>(result__)
    }
    pub unsafe fn EntryTypeLocalGroup(&self) -> ::windows_core::Result<GPMEntryType> {
        let mut result__ = ::core::mem::MaybeUninit::<GPMEntryType>::zeroed();
        (::windows_core::Interface::vtable(self).EntryTypeLocalGroup)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<GPMEntryType>(result__)
    }
    pub unsafe fn EntryTypeGlobalGroup(&self) -> ::windows_core::Result<GPMEntryType> {
        let mut result__ = ::core::mem::MaybeUninit::<GPMEntryType>::zeroed();
        (::windows_core::Interface::vtable(self).EntryTypeGlobalGroup)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<GPMEntryType>(result__)
    }
    pub unsafe fn EntryTypeUniversalGroup(&self) -> ::windows_core::Result<GPMEntryType> {
        let mut result__ = ::core::mem::MaybeUninit::<GPMEntryType>::zeroed();
        (::windows_core::Interface::vtable(self).EntryTypeUniversalGroup)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<GPMEntryType>(result__)
    }
    pub unsafe fn EntryTypeUNCPath(&self) -> ::windows_core::Result<GPMEntryType> {
        let mut result__ = ::core::mem::MaybeUninit::<GPMEntryType>::zeroed();
        (::windows_core::Interface::vtable(self).EntryTypeUNCPath)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<GPMEntryType>(result__)
    }
    pub unsafe fn EntryTypeUnknown(&self) -> ::windows_core::Result<GPMEntryType> {
        let mut result__ = ::core::mem::MaybeUninit::<GPMEntryType>::zeroed();
        (::windows_core::Interface::vtable(self).EntryTypeUnknown)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<GPMEntryType>(result__)
    }
    pub unsafe fn DestinationOptionSameAsSource(&self) -> ::windows_core::Result<GPMDestinationOption> {
        let mut result__ = ::core::mem::MaybeUninit::<GPMDestinationOption>::zeroed();
        (::windows_core::Interface::vtable(self).DestinationOptionSameAsSource)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<GPMDestinationOption>(result__)
    }
    pub unsafe fn DestinationOptionNone(&self) -> ::windows_core::Result<GPMDestinationOption> {
        let mut result__ = ::core::mem::MaybeUninit::<GPMDestinationOption>::zeroed();
        (::windows_core::Interface::vtable(self).DestinationOptionNone)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<GPMDestinationOption>(result__)
    }
    pub unsafe fn DestinationOptionByRelativeName(&self) -> ::windows_core::Result<GPMDestinationOption> {
        let mut result__ = ::core::mem::MaybeUninit::<GPMDestinationOption>::zeroed();
        (::windows_core::Interface::vtable(self).DestinationOptionByRelativeName)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<GPMDestinationOption>(result__)
    }
    pub unsafe fn DestinationOptionSet(&self) -> ::windows_core::Result<GPMDestinationOption> {
        let mut result__ = ::core::mem::MaybeUninit::<GPMDestinationOption>::zeroed();
        (::windows_core::Interface::vtable(self).DestinationOptionSet)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<GPMDestinationOption>(result__)
    }
    pub unsafe fn MigrationTableOnly(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).MigrationTableOnly)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn ProcessSecurity(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).ProcessSecurity)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn RsopLoggingNoComputer(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).RsopLoggingNoComputer)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn RsopLoggingNoUser(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).RsopLoggingNoUser)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn RsopPlanningAssumeSlowLink(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).RsopPlanningAssumeSlowLink)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn get_RsopPlanningLoopbackOption(&self, vbmerge: i16) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).get_RsopPlanningLoopbackOption)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(vbmerge), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn RsopPlanningAssumeUserWQLFilterTrue(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).RsopPlanningAssumeUserWQLFilterTrue)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn RsopPlanningAssumeCompWQLFilterTrue(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).RsopPlanningAssumeCompWQLFilterTrue)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IGPMConstants> for ::windows_core::IUnknown {
    fn from(value: IGPMConstants) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IGPMConstants> for ::windows_core::IUnknown {
    fn from(value: &IGPMConstants) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IGPMConstants {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IGPMConstants {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IGPMConstants> for super::Com::IDispatch {
    fn from(value: IGPMConstants) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IGPMConstants> for super::Com::IDispatch {
    fn from(value: &IGPMConstants) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for IGPMConstants {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a IGPMConstants {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for IGPMConstants {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for IGPMConstants {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for IGPMConstants {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for IGPMConstants {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IGPMConstants").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for IGPMConstants {
    type Vtable = IGPMConstants_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x50ef73e6_d35c_4c8d_be63_7ea5d2aac5c4);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct IGPMConstants_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub PermGPOApply: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut GPMPermissionType) -> ::windows_core::HRESULT,
    pub PermGPORead: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut GPMPermissionType) -> ::windows_core::HRESULT,
    pub PermGPOEdit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut GPMPermissionType) -> ::windows_core::HRESULT,
    pub PermGPOEditSecurityAndDelete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut GPMPermissionType) -> ::windows_core::HRESULT,
    pub PermGPOCustom: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut GPMPermissionType) -> ::windows_core::HRESULT,
    pub PermWMIFilterEdit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut GPMPermissionType) -> ::windows_core::HRESULT,
    pub PermWMIFilterFullControl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut GPMPermissionType) -> ::windows_core::HRESULT,
    pub PermWMIFilterCustom: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut GPMPermissionType) -> ::windows_core::HRESULT,
    pub PermSOMLink: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut GPMPermissionType) -> ::windows_core::HRESULT,
    pub PermSOMLogging: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut GPMPermissionType) -> ::windows_core::HRESULT,
    pub PermSOMPlanning: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut GPMPermissionType) -> ::windows_core::HRESULT,
    pub PermSOMGPOCreate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut GPMPermissionType) -> ::windows_core::HRESULT,
    pub PermSOMWMICreate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut GPMPermissionType) -> ::windows_core::HRESULT,
    pub PermSOMWMIFullControl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut GPMPermissionType) -> ::windows_core::HRESULT,
    pub SearchPropertyGPOPermissions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut GPMSearchProperty) -> ::windows_core::HRESULT,
    pub SearchPropertyGPOEffectivePermissions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut GPMSearchProperty) -> ::windows_core::HRESULT,
    pub SearchPropertyGPODisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut GPMSearchProperty) -> ::windows_core::HRESULT,
    pub SearchPropertyGPOWMIFilter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut GPMSearchProperty) -> ::windows_core::HRESULT,
    pub SearchPropertyGPOID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut GPMSearchProperty) -> ::windows_core::HRESULT,
    pub SearchPropertyGPOComputerExtensions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut GPMSearchProperty) -> ::windows_core::HRESULT,
    pub SearchPropertyGPOUserExtensions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut GPMSearchProperty) -> ::windows_core::HRESULT,
    pub SearchPropertySOMLinks: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut GPMSearchProperty) -> ::windows_core::HRESULT,
    pub SearchPropertyGPODomain: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut GPMSearchProperty) -> ::windows_core::HRESULT,
    pub SearchPropertyBackupMostRecent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut GPMSearchProperty) -> ::windows_core::HRESULT,
    pub SearchOpEquals: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut GPMSearchOperation) -> ::windows_core::HRESULT,
    pub SearchOpContains: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut GPMSearchOperation) -> ::windows_core::HRESULT,
    pub SearchOpNotContains: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut GPMSearchOperation) -> ::windows_core::HRESULT,
    pub SearchOpNotEquals: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut GPMSearchOperation) -> ::windows_core::HRESULT,
    pub UsePDC: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows_core::HRESULT,
    pub UseAnyDC: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows_core::HRESULT,
    pub DoNotUseW2KDC: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows_core::HRESULT,
    pub SOMSite: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut GPMSOMType) -> ::windows_core::HRESULT,
    pub SOMDomain: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut GPMSOMType) -> ::windows_core::HRESULT,
    pub SOMOU: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut GPMSOMType) -> ::windows_core::HRESULT,
    pub get_SecurityFlags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, vbowner: i16, vbgroup: i16, vbdacl: i16, vbsacl: i16, pval: *mut i32) -> ::windows_core::HRESULT,
    pub DoNotValidateDC: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows_core::HRESULT,
    pub ReportHTML: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut GPMReportType) -> ::windows_core::HRESULT,
    pub ReportXML: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut GPMReportType) -> ::windows_core::HRESULT,
    pub RSOPModeUnknown: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut GPMRSOPMode) -> ::windows_core::HRESULT,
    pub RSOPModePlanning: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut GPMRSOPMode) -> ::windows_core::HRESULT,
    pub RSOPModeLogging: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut GPMRSOPMode) -> ::windows_core::HRESULT,
    pub EntryTypeUser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut GPMEntryType) -> ::windows_core::HRESULT,
    pub EntryTypeComputer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut GPMEntryType) -> ::windows_core::HRESULT,
    pub EntryTypeLocalGroup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut GPMEntryType) -> ::windows_core::HRESULT,
    pub EntryTypeGlobalGroup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut GPMEntryType) -> ::windows_core::HRESULT,
    pub EntryTypeUniversalGroup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut GPMEntryType) -> ::windows_core::HRESULT,
    pub EntryTypeUNCPath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut GPMEntryType) -> ::windows_core::HRESULT,
    pub EntryTypeUnknown: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut GPMEntryType) -> ::windows_core::HRESULT,
    pub DestinationOptionSameAsSource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut GPMDestinationOption) -> ::windows_core::HRESULT,
    pub DestinationOptionNone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut GPMDestinationOption) -> ::windows_core::HRESULT,
    pub DestinationOptionByRelativeName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut GPMDestinationOption) -> ::windows_core::HRESULT,
    pub DestinationOptionSet: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut GPMDestinationOption) -> ::windows_core::HRESULT,
    pub MigrationTableOnly: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows_core::HRESULT,
    pub ProcessSecurity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows_core::HRESULT,
    pub RsopLoggingNoComputer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows_core::HRESULT,
    pub RsopLoggingNoUser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows_core::HRESULT,
    pub RsopPlanningAssumeSlowLink: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows_core::HRESULT,
    pub get_RsopPlanningLoopbackOption: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, vbmerge: i16, pval: *mut i32) -> ::windows_core::HRESULT,
    pub RsopPlanningAssumeUserWQLFilterTrue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows_core::HRESULT,
    pub RsopPlanningAssumeCompWQLFilterTrue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows_core::HRESULT,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct IGPMConstants2(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl IGPMConstants2 {
    pub unsafe fn PermGPOApply(&self) -> ::windows_core::Result<GPMPermissionType> {
        let mut result__ = ::core::mem::MaybeUninit::<GPMPermissionType>::zeroed();
        (::windows_core::Interface::vtable(self).base__.PermGPOApply)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<GPMPermissionType>(result__)
    }
    pub unsafe fn PermGPORead(&self) -> ::windows_core::Result<GPMPermissionType> {
        let mut result__ = ::core::mem::MaybeUninit::<GPMPermissionType>::zeroed();
        (::windows_core::Interface::vtable(self).base__.PermGPORead)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<GPMPermissionType>(result__)
    }
    pub unsafe fn PermGPOEdit(&self) -> ::windows_core::Result<GPMPermissionType> {
        let mut result__ = ::core::mem::MaybeUninit::<GPMPermissionType>::zeroed();
        (::windows_core::Interface::vtable(self).base__.PermGPOEdit)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<GPMPermissionType>(result__)
    }
    pub unsafe fn PermGPOEditSecurityAndDelete(&self) -> ::windows_core::Result<GPMPermissionType> {
        let mut result__ = ::core::mem::MaybeUninit::<GPMPermissionType>::zeroed();
        (::windows_core::Interface::vtable(self).base__.PermGPOEditSecurityAndDelete)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<GPMPermissionType>(result__)
    }
    pub unsafe fn PermGPOCustom(&self) -> ::windows_core::Result<GPMPermissionType> {
        let mut result__ = ::core::mem::MaybeUninit::<GPMPermissionType>::zeroed();
        (::windows_core::Interface::vtable(self).base__.PermGPOCustom)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<GPMPermissionType>(result__)
    }
    pub unsafe fn PermWMIFilterEdit(&self) -> ::windows_core::Result<GPMPermissionType> {
        let mut result__ = ::core::mem::MaybeUninit::<GPMPermissionType>::zeroed();
        (::windows_core::Interface::vtable(self).base__.PermWMIFilterEdit)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<GPMPermissionType>(result__)
    }
    pub unsafe fn PermWMIFilterFullControl(&self) -> ::windows_core::Result<GPMPermissionType> {
        let mut result__ = ::core::mem::MaybeUninit::<GPMPermissionType>::zeroed();
        (::windows_core::Interface::vtable(self).base__.PermWMIFilterFullControl)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<GPMPermissionType>(result__)
    }
    pub unsafe fn PermWMIFilterCustom(&self) -> ::windows_core::Result<GPMPermissionType> {
        let mut result__ = ::core::mem::MaybeUninit::<GPMPermissionType>::zeroed();
        (::windows_core::Interface::vtable(self).base__.PermWMIFilterCustom)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<GPMPermissionType>(result__)
    }
    pub unsafe fn PermSOMLink(&self) -> ::windows_core::Result<GPMPermissionType> {
        let mut result__ = ::core::mem::MaybeUninit::<GPMPermissionType>::zeroed();
        (::windows_core::Interface::vtable(self).base__.PermSOMLink)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<GPMPermissionType>(result__)
    }
    pub unsafe fn PermSOMLogging(&self) -> ::windows_core::Result<GPMPermissionType> {
        let mut result__ = ::core::mem::MaybeUninit::<GPMPermissionType>::zeroed();
        (::windows_core::Interface::vtable(self).base__.PermSOMLogging)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<GPMPermissionType>(result__)
    }
    pub unsafe fn PermSOMPlanning(&self) -> ::windows_core::Result<GPMPermissionType> {
        let mut result__ = ::core::mem::MaybeUninit::<GPMPermissionType>::zeroed();
        (::windows_core::Interface::vtable(self).base__.PermSOMPlanning)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<GPMPermissionType>(result__)
    }
    pub unsafe fn PermSOMGPOCreate(&self) -> ::windows_core::Result<GPMPermissionType> {
        let mut result__ = ::core::mem::MaybeUninit::<GPMPermissionType>::zeroed();
        (::windows_core::Interface::vtable(self).base__.PermSOMGPOCreate)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<GPMPermissionType>(result__)
    }
    pub unsafe fn PermSOMWMICreate(&self) -> ::windows_core::Result<GPMPermissionType> {
        let mut result__ = ::core::mem::MaybeUninit::<GPMPermissionType>::zeroed();
        (::windows_core::Interface::vtable(self).base__.PermSOMWMICreate)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<GPMPermissionType>(result__)
    }
    pub unsafe fn PermSOMWMIFullControl(&self) -> ::windows_core::Result<GPMPermissionType> {
        let mut result__ = ::core::mem::MaybeUninit::<GPMPermissionType>::zeroed();
        (::windows_core::Interface::vtable(self).base__.PermSOMWMIFullControl)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<GPMPermissionType>(result__)
    }
    pub unsafe fn SearchPropertyGPOPermissions(&self) -> ::windows_core::Result<GPMSearchProperty> {
        let mut result__ = ::core::mem::MaybeUninit::<GPMSearchProperty>::zeroed();
        (::windows_core::Interface::vtable(self).base__.SearchPropertyGPOPermissions)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<GPMSearchProperty>(result__)
    }
    pub unsafe fn SearchPropertyGPOEffectivePermissions(&self) -> ::windows_core::Result<GPMSearchProperty> {
        let mut result__ = ::core::mem::MaybeUninit::<GPMSearchProperty>::zeroed();
        (::windows_core::Interface::vtable(self).base__.SearchPropertyGPOEffectivePermissions)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<GPMSearchProperty>(result__)
    }
    pub unsafe fn SearchPropertyGPODisplayName(&self) -> ::windows_core::Result<GPMSearchProperty> {
        let mut result__ = ::core::mem::MaybeUninit::<GPMSearchProperty>::zeroed();
        (::windows_core::Interface::vtable(self).base__.SearchPropertyGPODisplayName)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<GPMSearchProperty>(result__)
    }
    pub unsafe fn SearchPropertyGPOWMIFilter(&self) -> ::windows_core::Result<GPMSearchProperty> {
        let mut result__ = ::core::mem::MaybeUninit::<GPMSearchProperty>::zeroed();
        (::windows_core::Interface::vtable(self).base__.SearchPropertyGPOWMIFilter)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<GPMSearchProperty>(result__)
    }
    pub unsafe fn SearchPropertyGPOID(&self) -> ::windows_core::Result<GPMSearchProperty> {
        let mut result__ = ::core::mem::MaybeUninit::<GPMSearchProperty>::zeroed();
        (::windows_core::Interface::vtable(self).base__.SearchPropertyGPOID)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<GPMSearchProperty>(result__)
    }
    pub unsafe fn SearchPropertyGPOComputerExtensions(&self) -> ::windows_core::Result<GPMSearchProperty> {
        let mut result__ = ::core::mem::MaybeUninit::<GPMSearchProperty>::zeroed();
        (::windows_core::Interface::vtable(self).base__.SearchPropertyGPOComputerExtensions)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<GPMSearchProperty>(result__)
    }
    pub unsafe fn SearchPropertyGPOUserExtensions(&self) -> ::windows_core::Result<GPMSearchProperty> {
        let mut result__ = ::core::mem::MaybeUninit::<GPMSearchProperty>::zeroed();
        (::windows_core::Interface::vtable(self).base__.SearchPropertyGPOUserExtensions)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<GPMSearchProperty>(result__)
    }
    pub unsafe fn SearchPropertySOMLinks(&self) -> ::windows_core::Result<GPMSearchProperty> {
        let mut result__ = ::core::mem::MaybeUninit::<GPMSearchProperty>::zeroed();
        (::windows_core::Interface::vtable(self).base__.SearchPropertySOMLinks)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<GPMSearchProperty>(result__)
    }
    pub unsafe fn SearchPropertyGPODomain(&self) -> ::windows_core::Result<GPMSearchProperty> {
        let mut result__ = ::core::mem::MaybeUninit::<GPMSearchProperty>::zeroed();
        (::windows_core::Interface::vtable(self).base__.SearchPropertyGPODomain)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<GPMSearchProperty>(result__)
    }
    pub unsafe fn SearchPropertyBackupMostRecent(&self) -> ::windows_core::Result<GPMSearchProperty> {
        let mut result__ = ::core::mem::MaybeUninit::<GPMSearchProperty>::zeroed();
        (::windows_core::Interface::vtable(self).base__.SearchPropertyBackupMostRecent)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<GPMSearchProperty>(result__)
    }
    pub unsafe fn SearchOpEquals(&self) -> ::windows_core::Result<GPMSearchOperation> {
        let mut result__ = ::core::mem::MaybeUninit::<GPMSearchOperation>::zeroed();
        (::windows_core::Interface::vtable(self).base__.SearchOpEquals)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<GPMSearchOperation>(result__)
    }
    pub unsafe fn SearchOpContains(&self) -> ::windows_core::Result<GPMSearchOperation> {
        let mut result__ = ::core::mem::MaybeUninit::<GPMSearchOperation>::zeroed();
        (::windows_core::Interface::vtable(self).base__.SearchOpContains)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<GPMSearchOperation>(result__)
    }
    pub unsafe fn SearchOpNotContains(&self) -> ::windows_core::Result<GPMSearchOperation> {
        let mut result__ = ::core::mem::MaybeUninit::<GPMSearchOperation>::zeroed();
        (::windows_core::Interface::vtable(self).base__.SearchOpNotContains)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<GPMSearchOperation>(result__)
    }
    pub unsafe fn SearchOpNotEquals(&self) -> ::windows_core::Result<GPMSearchOperation> {
        let mut result__ = ::core::mem::MaybeUninit::<GPMSearchOperation>::zeroed();
        (::windows_core::Interface::vtable(self).base__.SearchOpNotEquals)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<GPMSearchOperation>(result__)
    }
    pub unsafe fn UsePDC(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).base__.UsePDC)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn UseAnyDC(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).base__.UseAnyDC)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn DoNotUseW2KDC(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).base__.DoNotUseW2KDC)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SOMSite(&self) -> ::windows_core::Result<GPMSOMType> {
        let mut result__ = ::core::mem::MaybeUninit::<GPMSOMType>::zeroed();
        (::windows_core::Interface::vtable(self).base__.SOMSite)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<GPMSOMType>(result__)
    }
    pub unsafe fn SOMDomain(&self) -> ::windows_core::Result<GPMSOMType> {
        let mut result__ = ::core::mem::MaybeUninit::<GPMSOMType>::zeroed();
        (::windows_core::Interface::vtable(self).base__.SOMDomain)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<GPMSOMType>(result__)
    }
    pub unsafe fn SOMOU(&self) -> ::windows_core::Result<GPMSOMType> {
        let mut result__ = ::core::mem::MaybeUninit::<GPMSOMType>::zeroed();
        (::windows_core::Interface::vtable(self).base__.SOMOU)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<GPMSOMType>(result__)
    }
    pub unsafe fn get_SecurityFlags(&self, vbowner: i16, vbgroup: i16, vbdacl: i16, vbsacl: i16) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).base__.get_SecurityFlags)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(vbowner), ::core::mem::transmute(vbgroup), ::core::mem::transmute(vbdacl), ::core::mem::transmute(vbsacl), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn DoNotValidateDC(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).base__.DoNotValidateDC)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn ReportHTML(&self) -> ::windows_core::Result<GPMReportType> {
        let mut result__ = ::core::mem::MaybeUninit::<GPMReportType>::zeroed();
        (::windows_core::Interface::vtable(self).base__.ReportHTML)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<GPMReportType>(result__)
    }
    pub unsafe fn ReportXML(&self) -> ::windows_core::Result<GPMReportType> {
        let mut result__ = ::core::mem::MaybeUninit::<GPMReportType>::zeroed();
        (::windows_core::Interface::vtable(self).base__.ReportXML)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<GPMReportType>(result__)
    }
    pub unsafe fn RSOPModeUnknown(&self) -> ::windows_core::Result<GPMRSOPMode> {
        let mut result__ = ::core::mem::MaybeUninit::<GPMRSOPMode>::zeroed();
        (::windows_core::Interface::vtable(self).base__.RSOPModeUnknown)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<GPMRSOPMode>(result__)
    }
    pub unsafe fn RSOPModePlanning(&self) -> ::windows_core::Result<GPMRSOPMode> {
        let mut result__ = ::core::mem::MaybeUninit::<GPMRSOPMode>::zeroed();
        (::windows_core::Interface::vtable(self).base__.RSOPModePlanning)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<GPMRSOPMode>(result__)
    }
    pub unsafe fn RSOPModeLogging(&self) -> ::windows_core::Result<GPMRSOPMode> {
        let mut result__ = ::core::mem::MaybeUninit::<GPMRSOPMode>::zeroed();
        (::windows_core::Interface::vtable(self).base__.RSOPModeLogging)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<GPMRSOPMode>(result__)
    }
    pub unsafe fn EntryTypeUser(&self) -> ::windows_core::Result<GPMEntryType> {
        let mut result__ = ::core::mem::MaybeUninit::<GPMEntryType>::zeroed();
        (::windows_core::Interface::vtable(self).base__.EntryTypeUser)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<GPMEntryType>(result__)
    }
    pub unsafe fn EntryTypeComputer(&self) -> ::windows_core::Result<GPMEntryType> {
        let mut result__ = ::core::mem::MaybeUninit::<GPMEntryType>::zeroed();
        (::windows_core::Interface::vtable(self).base__.EntryTypeComputer)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<GPMEntryType>(result__)
    }
    pub unsafe fn EntryTypeLocalGroup(&self) -> ::windows_core::Result<GPMEntryType> {
        let mut result__ = ::core::mem::MaybeUninit::<GPMEntryType>::zeroed();
        (::windows_core::Interface::vtable(self).base__.EntryTypeLocalGroup)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<GPMEntryType>(result__)
    }
    pub unsafe fn EntryTypeGlobalGroup(&self) -> ::windows_core::Result<GPMEntryType> {
        let mut result__ = ::core::mem::MaybeUninit::<GPMEntryType>::zeroed();
        (::windows_core::Interface::vtable(self).base__.EntryTypeGlobalGroup)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<GPMEntryType>(result__)
    }
    pub unsafe fn EntryTypeUniversalGroup(&self) -> ::windows_core::Result<GPMEntryType> {
        let mut result__ = ::core::mem::MaybeUninit::<GPMEntryType>::zeroed();
        (::windows_core::Interface::vtable(self).base__.EntryTypeUniversalGroup)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<GPMEntryType>(result__)
    }
    pub unsafe fn EntryTypeUNCPath(&self) -> ::windows_core::Result<GPMEntryType> {
        let mut result__ = ::core::mem::MaybeUninit::<GPMEntryType>::zeroed();
        (::windows_core::Interface::vtable(self).base__.EntryTypeUNCPath)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<GPMEntryType>(result__)
    }
    pub unsafe fn EntryTypeUnknown(&self) -> ::windows_core::Result<GPMEntryType> {
        let mut result__ = ::core::mem::MaybeUninit::<GPMEntryType>::zeroed();
        (::windows_core::Interface::vtable(self).base__.EntryTypeUnknown)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<GPMEntryType>(result__)
    }
    pub unsafe fn DestinationOptionSameAsSource(&self) -> ::windows_core::Result<GPMDestinationOption> {
        let mut result__ = ::core::mem::MaybeUninit::<GPMDestinationOption>::zeroed();
        (::windows_core::Interface::vtable(self).base__.DestinationOptionSameAsSource)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<GPMDestinationOption>(result__)
    }
    pub unsafe fn DestinationOptionNone(&self) -> ::windows_core::Result<GPMDestinationOption> {
        let mut result__ = ::core::mem::MaybeUninit::<GPMDestinationOption>::zeroed();
        (::windows_core::Interface::vtable(self).base__.DestinationOptionNone)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<GPMDestinationOption>(result__)
    }
    pub unsafe fn DestinationOptionByRelativeName(&self) -> ::windows_core::Result<GPMDestinationOption> {
        let mut result__ = ::core::mem::MaybeUninit::<GPMDestinationOption>::zeroed();
        (::windows_core::Interface::vtable(self).base__.DestinationOptionByRelativeName)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<GPMDestinationOption>(result__)
    }
    pub unsafe fn DestinationOptionSet(&self) -> ::windows_core::Result<GPMDestinationOption> {
        let mut result__ = ::core::mem::MaybeUninit::<GPMDestinationOption>::zeroed();
        (::windows_core::Interface::vtable(self).base__.DestinationOptionSet)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<GPMDestinationOption>(result__)
    }
    pub unsafe fn MigrationTableOnly(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).base__.MigrationTableOnly)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn ProcessSecurity(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).base__.ProcessSecurity)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn RsopLoggingNoComputer(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).base__.RsopLoggingNoComputer)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn RsopLoggingNoUser(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).base__.RsopLoggingNoUser)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn RsopPlanningAssumeSlowLink(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).base__.RsopPlanningAssumeSlowLink)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn get_RsopPlanningLoopbackOption(&self, vbmerge: i16) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).base__.get_RsopPlanningLoopbackOption)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(vbmerge), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn RsopPlanningAssumeUserWQLFilterTrue(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).base__.RsopPlanningAssumeUserWQLFilterTrue)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn RsopPlanningAssumeCompWQLFilterTrue(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).base__.RsopPlanningAssumeCompWQLFilterTrue)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn BackupTypeGPO(&self) -> ::windows_core::Result<GPMBackupType> {
        let mut result__ = ::core::mem::MaybeUninit::<GPMBackupType>::zeroed();
        (::windows_core::Interface::vtable(self).BackupTypeGPO)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<GPMBackupType>(result__)
    }
    pub unsafe fn BackupTypeStarterGPO(&self) -> ::windows_core::Result<GPMBackupType> {
        let mut result__ = ::core::mem::MaybeUninit::<GPMBackupType>::zeroed();
        (::windows_core::Interface::vtable(self).BackupTypeStarterGPO)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<GPMBackupType>(result__)
    }
    pub unsafe fn StarterGPOTypeSystem(&self) -> ::windows_core::Result<GPMStarterGPOType> {
        let mut result__ = ::core::mem::MaybeUninit::<GPMStarterGPOType>::zeroed();
        (::windows_core::Interface::vtable(self).StarterGPOTypeSystem)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<GPMStarterGPOType>(result__)
    }
    pub unsafe fn StarterGPOTypeCustom(&self) -> ::windows_core::Result<GPMStarterGPOType> {
        let mut result__ = ::core::mem::MaybeUninit::<GPMStarterGPOType>::zeroed();
        (::windows_core::Interface::vtable(self).StarterGPOTypeCustom)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<GPMStarterGPOType>(result__)
    }
    pub unsafe fn SearchPropertyStarterGPOPermissions(&self) -> ::windows_core::Result<GPMSearchProperty> {
        let mut result__ = ::core::mem::MaybeUninit::<GPMSearchProperty>::zeroed();
        (::windows_core::Interface::vtable(self).SearchPropertyStarterGPOPermissions)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<GPMSearchProperty>(result__)
    }
    pub unsafe fn SearchPropertyStarterGPOEffectivePermissions(&self) -> ::windows_core::Result<GPMSearchProperty> {
        let mut result__ = ::core::mem::MaybeUninit::<GPMSearchProperty>::zeroed();
        (::windows_core::Interface::vtable(self).SearchPropertyStarterGPOEffectivePermissions)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<GPMSearchProperty>(result__)
    }
    pub unsafe fn SearchPropertyStarterGPODisplayName(&self) -> ::windows_core::Result<GPMSearchProperty> {
        let mut result__ = ::core::mem::MaybeUninit::<GPMSearchProperty>::zeroed();
        (::windows_core::Interface::vtable(self).SearchPropertyStarterGPODisplayName)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<GPMSearchProperty>(result__)
    }
    pub unsafe fn SearchPropertyStarterGPOID(&self) -> ::windows_core::Result<GPMSearchProperty> {
        let mut result__ = ::core::mem::MaybeUninit::<GPMSearchProperty>::zeroed();
        (::windows_core::Interface::vtable(self).SearchPropertyStarterGPOID)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<GPMSearchProperty>(result__)
    }
    pub unsafe fn SearchPropertyStarterGPODomain(&self) -> ::windows_core::Result<GPMSearchProperty> {
        let mut result__ = ::core::mem::MaybeUninit::<GPMSearchProperty>::zeroed();
        (::windows_core::Interface::vtable(self).SearchPropertyStarterGPODomain)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<GPMSearchProperty>(result__)
    }
    pub unsafe fn PermStarterGPORead(&self) -> ::windows_core::Result<GPMPermissionType> {
        let mut result__ = ::core::mem::MaybeUninit::<GPMPermissionType>::zeroed();
        (::windows_core::Interface::vtable(self).PermStarterGPORead)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<GPMPermissionType>(result__)
    }
    pub unsafe fn PermStarterGPOEdit(&self) -> ::windows_core::Result<GPMPermissionType> {
        let mut result__ = ::core::mem::MaybeUninit::<GPMPermissionType>::zeroed();
        (::windows_core::Interface::vtable(self).PermStarterGPOEdit)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<GPMPermissionType>(result__)
    }
    pub unsafe fn PermStarterGPOFullControl(&self) -> ::windows_core::Result<GPMPermissionType> {
        let mut result__ = ::core::mem::MaybeUninit::<GPMPermissionType>::zeroed();
        (::windows_core::Interface::vtable(self).PermStarterGPOFullControl)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<GPMPermissionType>(result__)
    }
    pub unsafe fn PermStarterGPOCustom(&self) -> ::windows_core::Result<GPMPermissionType> {
        let mut result__ = ::core::mem::MaybeUninit::<GPMPermissionType>::zeroed();
        (::windows_core::Interface::vtable(self).PermStarterGPOCustom)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<GPMPermissionType>(result__)
    }
    pub unsafe fn ReportLegacy(&self) -> ::windows_core::Result<GPMReportingOptions> {
        let mut result__ = ::core::mem::MaybeUninit::<GPMReportingOptions>::zeroed();
        (::windows_core::Interface::vtable(self).ReportLegacy)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<GPMReportingOptions>(result__)
    }
    pub unsafe fn ReportComments(&self) -> ::windows_core::Result<GPMReportingOptions> {
        let mut result__ = ::core::mem::MaybeUninit::<GPMReportingOptions>::zeroed();
        (::windows_core::Interface::vtable(self).ReportComments)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<GPMReportingOptions>(result__)
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IGPMConstants2> for ::windows_core::IUnknown {
    fn from(value: IGPMConstants2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IGPMConstants2> for ::windows_core::IUnknown {
    fn from(value: &IGPMConstants2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IGPMConstants2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IGPMConstants2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IGPMConstants2> for super::Com::IDispatch {
    fn from(value: IGPMConstants2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IGPMConstants2> for super::Com::IDispatch {
    fn from(value: &IGPMConstants2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for IGPMConstants2 {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a IGPMConstants2 {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IGPMConstants2> for IGPMConstants {
    fn from(value: IGPMConstants2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IGPMConstants2> for IGPMConstants {
    fn from(value: &IGPMConstants2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, IGPMConstants> for IGPMConstants2 {
    fn into_param(self) -> ::windows_core::Param<'a, IGPMConstants> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, IGPMConstants> for &'a IGPMConstants2 {
    fn into_param(self) -> ::windows_core::Param<'a, IGPMConstants> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for IGPMConstants2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for IGPMConstants2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for IGPMConstants2 {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for IGPMConstants2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IGPMConstants2").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for IGPMConstants2 {
    type Vtable = IGPMConstants2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x05ae21b0_ac09_4032_a26f_9e7da786dc19);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct IGPMConstants2_Vtbl {
    pub base__: IGPMConstants_Vtbl,
    pub BackupTypeGPO: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut GPMBackupType) -> ::windows_core::HRESULT,
    pub BackupTypeStarterGPO: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut GPMBackupType) -> ::windows_core::HRESULT,
    pub StarterGPOTypeSystem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut GPMStarterGPOType) -> ::windows_core::HRESULT,
    pub StarterGPOTypeCustom: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut GPMStarterGPOType) -> ::windows_core::HRESULT,
    pub SearchPropertyStarterGPOPermissions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut GPMSearchProperty) -> ::windows_core::HRESULT,
    pub SearchPropertyStarterGPOEffectivePermissions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut GPMSearchProperty) -> ::windows_core::HRESULT,
    pub SearchPropertyStarterGPODisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut GPMSearchProperty) -> ::windows_core::HRESULT,
    pub SearchPropertyStarterGPOID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut GPMSearchProperty) -> ::windows_core::HRESULT,
    pub SearchPropertyStarterGPODomain: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut GPMSearchProperty) -> ::windows_core::HRESULT,
    pub PermStarterGPORead: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut GPMPermissionType) -> ::windows_core::HRESULT,
    pub PermStarterGPOEdit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut GPMPermissionType) -> ::windows_core::HRESULT,
    pub PermStarterGPOFullControl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut GPMPermissionType) -> ::windows_core::HRESULT,
    pub PermStarterGPOCustom: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut GPMPermissionType) -> ::windows_core::HRESULT,
    pub ReportLegacy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut GPMReportingOptions) -> ::windows_core::HRESULT,
    pub ReportComments: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut GPMReportingOptions) -> ::windows_core::HRESULT,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct IGPMDomain(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl IGPMDomain {
    pub unsafe fn DomainController(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).DomainController)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn Domain(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).Domain)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn CreateGPO(&self) -> ::windows_core::Result<IGPMGPO> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).CreateGPO)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IGPMGPO>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn GetGPO<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrguid: Param0) -> ::windows_core::Result<IGPMGPO> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetGPO)(::windows_core::Interface::as_raw(self), bstrguid.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IGPMGPO>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn SearchGPOs<'a, Param0: ::windows_core::IntoParam<'a, IGPMSearchCriteria>>(&self, pigpmsearchcriteria: Param0) -> ::windows_core::Result<IGPMGPOCollection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).SearchGPOs)(::windows_core::Interface::as_raw(self), pigpmsearchcriteria.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IGPMGPOCollection>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn RestoreGPO<'a, Param0: ::windows_core::IntoParam<'a, IGPMBackup>>(&self, pigpmbackup: Param0, ldcflags: i32, pvargpmprogress: *const super::Com::VARIANT, pvargpmcancel: *mut super::Com::VARIANT, ppigpmresult: *mut ::core::option::Option<IGPMResult>) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).RestoreGPO)(::windows_core::Interface::as_raw(self), pigpmbackup.into_param().abi(), ::core::mem::transmute(ldcflags), ::core::mem::transmute(pvargpmprogress), ::core::mem::transmute(pvargpmcancel), ::core::mem::transmute(ppigpmresult)).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn GetSOM<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrpath: Param0) -> ::windows_core::Result<IGPMSOM> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetSOM)(::windows_core::Interface::as_raw(self), bstrpath.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IGPMSOM>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn SearchSOMs<'a, Param0: ::windows_core::IntoParam<'a, IGPMSearchCriteria>>(&self, pigpmsearchcriteria: Param0) -> ::windows_core::Result<IGPMSOMCollection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).SearchSOMs)(::windows_core::Interface::as_raw(self), pigpmsearchcriteria.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IGPMSOMCollection>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn GetWMIFilter<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrpath: Param0) -> ::windows_core::Result<IGPMWMIFilter> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetWMIFilter)(::windows_core::Interface::as_raw(self), bstrpath.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IGPMWMIFilter>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn SearchWMIFilters<'a, Param0: ::windows_core::IntoParam<'a, IGPMSearchCriteria>>(&self, pigpmsearchcriteria: Param0) -> ::windows_core::Result<IGPMWMIFilterCollection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).SearchWMIFilters)(::windows_core::Interface::as_raw(self), pigpmsearchcriteria.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IGPMWMIFilterCollection>(result__)
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IGPMDomain> for ::windows_core::IUnknown {
    fn from(value: IGPMDomain) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IGPMDomain> for ::windows_core::IUnknown {
    fn from(value: &IGPMDomain) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IGPMDomain {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IGPMDomain {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IGPMDomain> for super::Com::IDispatch {
    fn from(value: IGPMDomain) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IGPMDomain> for super::Com::IDispatch {
    fn from(value: &IGPMDomain) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for IGPMDomain {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a IGPMDomain {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for IGPMDomain {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for IGPMDomain {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for IGPMDomain {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for IGPMDomain {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IGPMDomain").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for IGPMDomain {
    type Vtable = IGPMDomain_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6b21cc14_5a00_4f44_a738_feec8a94c7e3);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct IGPMDomain_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub DomainController: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub Domain: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    #[cfg(feature = "win32-system")]
    pub CreateGPO: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppnewgpo: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    CreateGPO: usize,
    #[cfg(feature = "win32-system")]
    pub GetGPO: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrguid: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, ppgpo: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    GetGPO: usize,
    #[cfg(feature = "win32-system")]
    pub SearchGPOs: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pigpmsearchcriteria: ::windows_core::RawPtr, ppigpmgpocollection: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    SearchGPOs: usize,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub RestoreGPO: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pigpmbackup: ::windows_core::RawPtr, ldcflags: i32, pvargpmprogress: *const super::Com::VARIANT, pvargpmcancel: *mut super::Com::VARIANT, ppigpmresult: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    RestoreGPO: usize,
    #[cfg(feature = "win32-system")]
    pub GetSOM: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrpath: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, ppsom: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    GetSOM: usize,
    #[cfg(feature = "win32-system")]
    pub SearchSOMs: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pigpmsearchcriteria: ::windows_core::RawPtr, ppigpmsomcollection: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    SearchSOMs: usize,
    #[cfg(feature = "win32-system")]
    pub GetWMIFilter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrpath: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, ppwmifilter: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    GetWMIFilter: usize,
    #[cfg(feature = "win32-system")]
    pub SearchWMIFilters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pigpmsearchcriteria: ::windows_core::RawPtr, ppigpmwmifiltercollection: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    SearchWMIFilters: usize,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct IGPMDomain2(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl IGPMDomain2 {
    pub unsafe fn DomainController(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.DomainController)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn Domain(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.Domain)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn CreateGPO(&self) -> ::windows_core::Result<IGPMGPO> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.CreateGPO)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IGPMGPO>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn GetGPO<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrguid: Param0) -> ::windows_core::Result<IGPMGPO> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetGPO)(::windows_core::Interface::as_raw(self), bstrguid.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IGPMGPO>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn SearchGPOs<'a, Param0: ::windows_core::IntoParam<'a, IGPMSearchCriteria>>(&self, pigpmsearchcriteria: Param0) -> ::windows_core::Result<IGPMGPOCollection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.SearchGPOs)(::windows_core::Interface::as_raw(self), pigpmsearchcriteria.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IGPMGPOCollection>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn RestoreGPO<'a, Param0: ::windows_core::IntoParam<'a, IGPMBackup>>(&self, pigpmbackup: Param0, ldcflags: i32, pvargpmprogress: *const super::Com::VARIANT, pvargpmcancel: *mut super::Com::VARIANT, ppigpmresult: *mut ::core::option::Option<IGPMResult>) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.RestoreGPO)(::windows_core::Interface::as_raw(self), pigpmbackup.into_param().abi(), ::core::mem::transmute(ldcflags), ::core::mem::transmute(pvargpmprogress), ::core::mem::transmute(pvargpmcancel), ::core::mem::transmute(ppigpmresult)).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn GetSOM<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrpath: Param0) -> ::windows_core::Result<IGPMSOM> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetSOM)(::windows_core::Interface::as_raw(self), bstrpath.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IGPMSOM>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn SearchSOMs<'a, Param0: ::windows_core::IntoParam<'a, IGPMSearchCriteria>>(&self, pigpmsearchcriteria: Param0) -> ::windows_core::Result<IGPMSOMCollection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.SearchSOMs)(::windows_core::Interface::as_raw(self), pigpmsearchcriteria.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IGPMSOMCollection>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn GetWMIFilter<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrpath: Param0) -> ::windows_core::Result<IGPMWMIFilter> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetWMIFilter)(::windows_core::Interface::as_raw(self), bstrpath.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IGPMWMIFilter>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn SearchWMIFilters<'a, Param0: ::windows_core::IntoParam<'a, IGPMSearchCriteria>>(&self, pigpmsearchcriteria: Param0) -> ::windows_core::Result<IGPMWMIFilterCollection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.SearchWMIFilters)(::windows_core::Interface::as_raw(self), pigpmsearchcriteria.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IGPMWMIFilterCollection>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn CreateStarterGPO(&self) -> ::windows_core::Result<IGPMStarterGPO> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).CreateStarterGPO)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IGPMStarterGPO>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn CreateGPOFromStarterGPO<'a, Param0: ::windows_core::IntoParam<'a, IGPMStarterGPO>>(&self, pgpotemplate: Param0) -> ::windows_core::Result<IGPMGPO> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).CreateGPOFromStarterGPO)(::windows_core::Interface::as_raw(self), pgpotemplate.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IGPMGPO>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn GetStarterGPO<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrguid: Param0) -> ::windows_core::Result<IGPMStarterGPO> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetStarterGPO)(::windows_core::Interface::as_raw(self), bstrguid.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IGPMStarterGPO>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn SearchStarterGPOs<'a, Param0: ::windows_core::IntoParam<'a, IGPMSearchCriteria>>(&self, pigpmsearchcriteria: Param0) -> ::windows_core::Result<IGPMStarterGPOCollection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).SearchStarterGPOs)(::windows_core::Interface::as_raw(self), pigpmsearchcriteria.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IGPMStarterGPOCollection>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn LoadStarterGPO<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrloadfile: Param0, boverwrite: i16, pvargpmprogress: *const super::Com::VARIANT, pvargpmcancel: *mut super::Com::VARIANT, ppigpmresult: *mut ::core::option::Option<IGPMResult>) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).LoadStarterGPO)(::windows_core::Interface::as_raw(self), bstrloadfile.into_param().abi(), ::core::mem::transmute(boverwrite), ::core::mem::transmute(pvargpmprogress), ::core::mem::transmute(pvargpmcancel), ::core::mem::transmute(ppigpmresult)).ok()
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn RestoreStarterGPO<'a, Param0: ::windows_core::IntoParam<'a, IGPMStarterGPOBackup>>(&self, pigpmtmplbackup: Param0, pvargpmprogress: *const super::Com::VARIANT, pvargpmcancel: *mut super::Com::VARIANT, ppigpmresult: *mut ::core::option::Option<IGPMResult>) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).RestoreStarterGPO)(::windows_core::Interface::as_raw(self), pigpmtmplbackup.into_param().abi(), ::core::mem::transmute(pvargpmprogress), ::core::mem::transmute(pvargpmcancel), ::core::mem::transmute(ppigpmresult)).ok()
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IGPMDomain2> for ::windows_core::IUnknown {
    fn from(value: IGPMDomain2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IGPMDomain2> for ::windows_core::IUnknown {
    fn from(value: &IGPMDomain2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IGPMDomain2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IGPMDomain2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IGPMDomain2> for super::Com::IDispatch {
    fn from(value: IGPMDomain2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IGPMDomain2> for super::Com::IDispatch {
    fn from(value: &IGPMDomain2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for IGPMDomain2 {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a IGPMDomain2 {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IGPMDomain2> for IGPMDomain {
    fn from(value: IGPMDomain2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IGPMDomain2> for IGPMDomain {
    fn from(value: &IGPMDomain2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, IGPMDomain> for IGPMDomain2 {
    fn into_param(self) -> ::windows_core::Param<'a, IGPMDomain> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, IGPMDomain> for &'a IGPMDomain2 {
    fn into_param(self) -> ::windows_core::Param<'a, IGPMDomain> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for IGPMDomain2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for IGPMDomain2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for IGPMDomain2 {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for IGPMDomain2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IGPMDomain2").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for IGPMDomain2 {
    type Vtable = IGPMDomain2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7ca6bb8b_f1eb_490a_938d_3c4e51c768e6);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct IGPMDomain2_Vtbl {
    pub base__: IGPMDomain_Vtbl,
    #[cfg(feature = "win32-system")]
    pub CreateStarterGPO: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppnewtemplate: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    CreateStarterGPO: usize,
    #[cfg(feature = "win32-system")]
    pub CreateGPOFromStarterGPO: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pgpotemplate: ::windows_core::RawPtr, ppnewgpo: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    CreateGPOFromStarterGPO: usize,
    #[cfg(feature = "win32-system")]
    pub GetStarterGPO: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrguid: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, pptemplate: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    GetStarterGPO: usize,
    #[cfg(feature = "win32-system")]
    pub SearchStarterGPOs: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pigpmsearchcriteria: ::windows_core::RawPtr, ppigpmtemplatecollection: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    SearchStarterGPOs: usize,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub LoadStarterGPO: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrloadfile: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, boverwrite: i16, pvargpmprogress: *const super::Com::VARIANT, pvargpmcancel: *mut super::Com::VARIANT, ppigpmresult: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    LoadStarterGPO: usize,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub RestoreStarterGPO: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pigpmtmplbackup: ::windows_core::RawPtr, pvargpmprogress: *const super::Com::VARIANT, pvargpmcancel: *mut super::Com::VARIANT, ppigpmresult: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    RestoreStarterGPO: usize,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct IGPMDomain3(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl IGPMDomain3 {
    pub unsafe fn DomainController(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.DomainController)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn Domain(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.Domain)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn CreateGPO(&self) -> ::windows_core::Result<IGPMGPO> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.CreateGPO)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IGPMGPO>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn GetGPO<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrguid: Param0) -> ::windows_core::Result<IGPMGPO> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.GetGPO)(::windows_core::Interface::as_raw(self), bstrguid.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IGPMGPO>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn SearchGPOs<'a, Param0: ::windows_core::IntoParam<'a, IGPMSearchCriteria>>(&self, pigpmsearchcriteria: Param0) -> ::windows_core::Result<IGPMGPOCollection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.SearchGPOs)(::windows_core::Interface::as_raw(self), pigpmsearchcriteria.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IGPMGPOCollection>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn RestoreGPO<'a, Param0: ::windows_core::IntoParam<'a, IGPMBackup>>(&self, pigpmbackup: Param0, ldcflags: i32, pvargpmprogress: *const super::Com::VARIANT, pvargpmcancel: *mut super::Com::VARIANT, ppigpmresult: *mut ::core::option::Option<IGPMResult>) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.RestoreGPO)(::windows_core::Interface::as_raw(self), pigpmbackup.into_param().abi(), ::core::mem::transmute(ldcflags), ::core::mem::transmute(pvargpmprogress), ::core::mem::transmute(pvargpmcancel), ::core::mem::transmute(ppigpmresult)).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn GetSOM<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrpath: Param0) -> ::windows_core::Result<IGPMSOM> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.GetSOM)(::windows_core::Interface::as_raw(self), bstrpath.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IGPMSOM>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn SearchSOMs<'a, Param0: ::windows_core::IntoParam<'a, IGPMSearchCriteria>>(&self, pigpmsearchcriteria: Param0) -> ::windows_core::Result<IGPMSOMCollection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.SearchSOMs)(::windows_core::Interface::as_raw(self), pigpmsearchcriteria.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IGPMSOMCollection>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn GetWMIFilter<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrpath: Param0) -> ::windows_core::Result<IGPMWMIFilter> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.GetWMIFilter)(::windows_core::Interface::as_raw(self), bstrpath.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IGPMWMIFilter>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn SearchWMIFilters<'a, Param0: ::windows_core::IntoParam<'a, IGPMSearchCriteria>>(&self, pigpmsearchcriteria: Param0) -> ::windows_core::Result<IGPMWMIFilterCollection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.SearchWMIFilters)(::windows_core::Interface::as_raw(self), pigpmsearchcriteria.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IGPMWMIFilterCollection>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn CreateStarterGPO(&self) -> ::windows_core::Result<IGPMStarterGPO> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.CreateStarterGPO)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IGPMStarterGPO>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn CreateGPOFromStarterGPO<'a, Param0: ::windows_core::IntoParam<'a, IGPMStarterGPO>>(&self, pgpotemplate: Param0) -> ::windows_core::Result<IGPMGPO> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.CreateGPOFromStarterGPO)(::windows_core::Interface::as_raw(self), pgpotemplate.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IGPMGPO>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn GetStarterGPO<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrguid: Param0) -> ::windows_core::Result<IGPMStarterGPO> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetStarterGPO)(::windows_core::Interface::as_raw(self), bstrguid.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IGPMStarterGPO>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn SearchStarterGPOs<'a, Param0: ::windows_core::IntoParam<'a, IGPMSearchCriteria>>(&self, pigpmsearchcriteria: Param0) -> ::windows_core::Result<IGPMStarterGPOCollection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.SearchStarterGPOs)(::windows_core::Interface::as_raw(self), pigpmsearchcriteria.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IGPMStarterGPOCollection>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn LoadStarterGPO<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrloadfile: Param0, boverwrite: i16, pvargpmprogress: *const super::Com::VARIANT, pvargpmcancel: *mut super::Com::VARIANT, ppigpmresult: *mut ::core::option::Option<IGPMResult>) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.LoadStarterGPO)(::windows_core::Interface::as_raw(self), bstrloadfile.into_param().abi(), ::core::mem::transmute(boverwrite), ::core::mem::transmute(pvargpmprogress), ::core::mem::transmute(pvargpmcancel), ::core::mem::transmute(ppigpmresult)).ok()
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn RestoreStarterGPO<'a, Param0: ::windows_core::IntoParam<'a, IGPMStarterGPOBackup>>(&self, pigpmtmplbackup: Param0, pvargpmprogress: *const super::Com::VARIANT, pvargpmcancel: *mut super::Com::VARIANT, ppigpmresult: *mut ::core::option::Option<IGPMResult>) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.RestoreStarterGPO)(::windows_core::Interface::as_raw(self), pigpmtmplbackup.into_param().abi(), ::core::mem::transmute(pvargpmprogress), ::core::mem::transmute(pvargpmcancel), ::core::mem::transmute(ppigpmresult)).ok()
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn GenerateReport(&self, gpmreporttype: GPMReportType, pvargpmprogress: *const super::Com::VARIANT, pvargpmcancel: *mut super::Com::VARIANT, ppigpmresult: *mut ::core::option::Option<IGPMResult>) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GenerateReport)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(gpmreporttype), ::core::mem::transmute(pvargpmprogress), ::core::mem::transmute(pvargpmcancel), ::core::mem::transmute(ppigpmresult)).ok()
    }
    pub unsafe fn InfrastructureDC(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).InfrastructureDC)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetInfrastructureDC<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, newval: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetInfrastructureDC)(::windows_core::Interface::as_raw(self), newval.into_param().abi()).ok()
    }
    pub unsafe fn SetInfrastructureFlags(&self, dwflags: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetInfrastructureFlags)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwflags)).ok()
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IGPMDomain3> for ::windows_core::IUnknown {
    fn from(value: IGPMDomain3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IGPMDomain3> for ::windows_core::IUnknown {
    fn from(value: &IGPMDomain3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IGPMDomain3 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IGPMDomain3 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IGPMDomain3> for super::Com::IDispatch {
    fn from(value: IGPMDomain3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IGPMDomain3> for super::Com::IDispatch {
    fn from(value: &IGPMDomain3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for IGPMDomain3 {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a IGPMDomain3 {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IGPMDomain3> for IGPMDomain {
    fn from(value: IGPMDomain3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IGPMDomain3> for IGPMDomain {
    fn from(value: &IGPMDomain3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, IGPMDomain> for IGPMDomain3 {
    fn into_param(self) -> ::windows_core::Param<'a, IGPMDomain> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, IGPMDomain> for &'a IGPMDomain3 {
    fn into_param(self) -> ::windows_core::Param<'a, IGPMDomain> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IGPMDomain3> for IGPMDomain2 {
    fn from(value: IGPMDomain3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IGPMDomain3> for IGPMDomain2 {
    fn from(value: &IGPMDomain3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, IGPMDomain2> for IGPMDomain3 {
    fn into_param(self) -> ::windows_core::Param<'a, IGPMDomain2> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, IGPMDomain2> for &'a IGPMDomain3 {
    fn into_param(self) -> ::windows_core::Param<'a, IGPMDomain2> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for IGPMDomain3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for IGPMDomain3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for IGPMDomain3 {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for IGPMDomain3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IGPMDomain3").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for IGPMDomain3 {
    type Vtable = IGPMDomain3_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0077fdfe_88c7_4acf_a11d_d10a7c310a03);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct IGPMDomain3_Vtbl {
    pub base__: IGPMDomain2_Vtbl,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub GenerateReport: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, gpmreporttype: GPMReportType, pvargpmprogress: *const super::Com::VARIANT, pvargpmcancel: *mut super::Com::VARIANT, ppigpmresult: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    GenerateReport: usize,
    pub InfrastructureDC: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SetInfrastructureDC: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newval: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub SetInfrastructureFlags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwflags: u32) -> ::windows_core::HRESULT,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct IGPMGPO(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl IGPMGPO {
    pub unsafe fn DisplayName(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).DisplayName)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetDisplayName<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, newval: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetDisplayName)(::windows_core::Interface::as_raw(self), newval.into_param().abi()).ok()
    }
    pub unsafe fn Path(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).Path)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn ID(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).ID)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn DomainName(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).DomainName)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn CreationTime(&self) -> ::windows_core::Result<f64> {
        let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
        (::windows_core::Interface::vtable(self).CreationTime)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<f64>(result__)
    }
    pub unsafe fn ModificationTime(&self) -> ::windows_core::Result<f64> {
        let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
        (::windows_core::Interface::vtable(self).ModificationTime)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<f64>(result__)
    }
    pub unsafe fn UserDSVersionNumber(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).UserDSVersionNumber)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn ComputerDSVersionNumber(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).ComputerDSVersionNumber)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn UserSysvolVersionNumber(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).UserSysvolVersionNumber)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn ComputerSysvolVersionNumber(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).ComputerSysvolVersionNumber)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn GetWMIFilter(&self) -> ::windows_core::Result<IGPMWMIFilter> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetWMIFilter)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IGPMWMIFilter>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn SetWMIFilter<'a, Param0: ::windows_core::IntoParam<'a, IGPMWMIFilter>>(&self, pigpmwmifilter: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetWMIFilter)(::windows_core::Interface::as_raw(self), pigpmwmifilter.into_param().abi()).ok()
    }
    pub unsafe fn SetUserEnabled(&self, vbenabled: i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetUserEnabled)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(vbenabled)).ok()
    }
    pub unsafe fn SetComputerEnabled(&self, vbenabled: i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetComputerEnabled)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(vbenabled)).ok()
    }
    pub unsafe fn IsUserEnabled(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).IsUserEnabled)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn IsComputerEnabled(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).IsComputerEnabled)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn GetSecurityInfo(&self) -> ::windows_core::Result<IGPMSecurityInfo> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetSecurityInfo)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IGPMSecurityInfo>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn SetSecurityInfo<'a, Param0: ::windows_core::IntoParam<'a, IGPMSecurityInfo>>(&self, psecurityinfo: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetSecurityInfo)(::windows_core::Interface::as_raw(self), psecurityinfo.into_param().abi()).ok()
    }
    pub unsafe fn Delete(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Delete)(::windows_core::Interface::as_raw(self)).ok()
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn Backup<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrbackupdir: Param0, bstrcomment: Param1, pvargpmprogress: *const super::Com::VARIANT, pvargpmcancel: *mut super::Com::VARIANT, ppigpmresult: *mut ::core::option::Option<IGPMResult>) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Backup)(::windows_core::Interface::as_raw(self), bstrbackupdir.into_param().abi(), bstrcomment.into_param().abi(), ::core::mem::transmute(pvargpmprogress), ::core::mem::transmute(pvargpmcancel), ::core::mem::transmute(ppigpmresult)).ok()
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn Import<'a, Param1: ::windows_core::IntoParam<'a, IGPMBackup>>(&self, lflags: i32, pigpmbackup: Param1, pvarmigrationtable: *const super::Com::VARIANT, pvargpmprogress: *const super::Com::VARIANT, pvargpmcancel: *mut super::Com::VARIANT, ppigpmresult: *mut ::core::option::Option<IGPMResult>) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Import)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lflags), pigpmbackup.into_param().abi(), ::core::mem::transmute(pvarmigrationtable), ::core::mem::transmute(pvargpmprogress), ::core::mem::transmute(pvargpmcancel), ::core::mem::transmute(ppigpmresult)).ok()
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn GenerateReport(&self, gpmreporttype: GPMReportType, pvargpmprogress: *const super::Com::VARIANT, pvargpmcancel: *mut super::Com::VARIANT, ppigpmresult: *mut ::core::option::Option<IGPMResult>) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GenerateReport)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(gpmreporttype), ::core::mem::transmute(pvargpmprogress), ::core::mem::transmute(pvargpmcancel), ::core::mem::transmute(ppigpmresult)).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn GenerateReportToFile<'a, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, gpmreporttype: GPMReportType, bstrtargetfilepath: Param1) -> ::windows_core::Result<IGPMResult> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GenerateReportToFile)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(gpmreporttype), bstrtargetfilepath.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IGPMResult>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn CopyTo<'a, Param1: ::windows_core::IntoParam<'a, IGPMDomain>>(&self, lflags: i32, pigpmdomain: Param1, pvarnewdisplayname: *const super::Com::VARIANT, pvarmigrationtable: *const super::Com::VARIANT, pvargpmprogress: *const super::Com::VARIANT, pvargpmcancel: *mut super::Com::VARIANT, ppigpmresult: *mut ::core::option::Option<IGPMResult>) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).CopyTo)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lflags), pigpmdomain.into_param().abi(), ::core::mem::transmute(pvarnewdisplayname), ::core::mem::transmute(pvarmigrationtable), ::core::mem::transmute(pvargpmprogress), ::core::mem::transmute(pvargpmcancel), ::core::mem::transmute(ppigpmresult)).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn SetSecurityDescriptor<'a, Param1: ::windows_core::IntoParam<'a, super::Com::IDispatch>>(&self, lflags: i32, psd: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetSecurityDescriptor)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lflags), psd.into_param().abi()).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn GetSecurityDescriptor(&self, lflags: i32) -> ::windows_core::Result<super::Com::IDispatch> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetSecurityDescriptor)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lflags), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::IDispatch>(result__)
    }
    pub unsafe fn IsACLConsistent(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).IsACLConsistent)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn MakeACLConsistent(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).MakeACLConsistent)(::windows_core::Interface::as_raw(self)).ok()
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IGPMGPO> for ::windows_core::IUnknown {
    fn from(value: IGPMGPO) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IGPMGPO> for ::windows_core::IUnknown {
    fn from(value: &IGPMGPO) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IGPMGPO {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IGPMGPO {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IGPMGPO> for super::Com::IDispatch {
    fn from(value: IGPMGPO) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IGPMGPO> for super::Com::IDispatch {
    fn from(value: &IGPMGPO) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for IGPMGPO {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a IGPMGPO {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for IGPMGPO {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for IGPMGPO {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for IGPMGPO {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for IGPMGPO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IGPMGPO").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for IGPMGPO {
    type Vtable = IGPMGPO_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x58cc4352_1ca3_48e5_9864_1da4d6e0d60f);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct IGPMGPO_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub DisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SetDisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newval: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub Path: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub ID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub DomainName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub CreationTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdate: *mut f64) -> ::windows_core::HRESULT,
    pub ModificationTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdate: *mut f64) -> ::windows_core::HRESULT,
    pub UserDSVersionNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows_core::HRESULT,
    pub ComputerDSVersionNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows_core::HRESULT,
    pub UserSysvolVersionNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows_core::HRESULT,
    pub ComputerSysvolVersionNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(feature = "win32-system")]
    pub GetWMIFilter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppigpmwmifilter: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    GetWMIFilter: usize,
    #[cfg(feature = "win32-system")]
    pub SetWMIFilter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pigpmwmifilter: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    SetWMIFilter: usize,
    pub SetUserEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, vbenabled: i16) -> ::windows_core::HRESULT,
    pub SetComputerEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, vbenabled: i16) -> ::windows_core::HRESULT,
    pub IsUserEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvbenabled: *mut i16) -> ::windows_core::HRESULT,
    pub IsComputerEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvbenabled: *mut i16) -> ::windows_core::HRESULT,
    #[cfg(feature = "win32-system")]
    pub GetSecurityInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppsecurityinfo: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    GetSecurityInfo: usize,
    #[cfg(feature = "win32-system")]
    pub SetSecurityInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psecurityinfo: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    SetSecurityInfo: usize,
    pub Delete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub Backup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrbackupdir: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, bstrcomment: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, pvargpmprogress: *const super::Com::VARIANT, pvargpmcancel: *mut super::Com::VARIANT, ppigpmresult: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    Backup: usize,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub Import: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lflags: i32, pigpmbackup: ::windows_core::RawPtr, pvarmigrationtable: *const super::Com::VARIANT, pvargpmprogress: *const super::Com::VARIANT, pvargpmcancel: *mut super::Com::VARIANT, ppigpmresult: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    Import: usize,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub GenerateReport: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, gpmreporttype: GPMReportType, pvargpmprogress: *const super::Com::VARIANT, pvargpmcancel: *mut super::Com::VARIANT, ppigpmresult: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    GenerateReport: usize,
    #[cfg(feature = "win32-system")]
    pub GenerateReportToFile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, gpmreporttype: GPMReportType, bstrtargetfilepath: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, ppigpmresult: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    GenerateReportToFile: usize,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub CopyTo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lflags: i32, pigpmdomain: ::windows_core::RawPtr, pvarnewdisplayname: *const super::Com::VARIANT, pvarmigrationtable: *const super::Com::VARIANT, pvargpmprogress: *const super::Com::VARIANT, pvargpmcancel: *mut super::Com::VARIANT, ppigpmresult: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    CopyTo: usize,
    #[cfg(feature = "win32-system")]
    pub SetSecurityDescriptor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lflags: i32, psd: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    SetSecurityDescriptor: usize,
    #[cfg(feature = "win32-system")]
    pub GetSecurityDescriptor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lflags: i32, ppsd: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    GetSecurityDescriptor: usize,
    pub IsACLConsistent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvbconsistent: *mut i16) -> ::windows_core::HRESULT,
    pub MakeACLConsistent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct IGPMGPO2(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl IGPMGPO2 {
    pub unsafe fn DisplayName(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.DisplayName)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetDisplayName<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, newval: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetDisplayName)(::windows_core::Interface::as_raw(self), newval.into_param().abi()).ok()
    }
    pub unsafe fn Path(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.Path)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn ID(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.ID)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn DomainName(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.DomainName)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn CreationTime(&self) -> ::windows_core::Result<f64> {
        let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
        (::windows_core::Interface::vtable(self).base__.CreationTime)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<f64>(result__)
    }
    pub unsafe fn ModificationTime(&self) -> ::windows_core::Result<f64> {
        let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
        (::windows_core::Interface::vtable(self).base__.ModificationTime)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<f64>(result__)
    }
    pub unsafe fn UserDSVersionNumber(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).base__.UserDSVersionNumber)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn ComputerDSVersionNumber(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).base__.ComputerDSVersionNumber)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn UserSysvolVersionNumber(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).base__.UserSysvolVersionNumber)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn ComputerSysvolVersionNumber(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).base__.ComputerSysvolVersionNumber)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn GetWMIFilter(&self) -> ::windows_core::Result<IGPMWMIFilter> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetWMIFilter)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IGPMWMIFilter>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn SetWMIFilter<'a, Param0: ::windows_core::IntoParam<'a, IGPMWMIFilter>>(&self, pigpmwmifilter: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetWMIFilter)(::windows_core::Interface::as_raw(self), pigpmwmifilter.into_param().abi()).ok()
    }
    pub unsafe fn SetUserEnabled(&self, vbenabled: i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetUserEnabled)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(vbenabled)).ok()
    }
    pub unsafe fn SetComputerEnabled(&self, vbenabled: i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetComputerEnabled)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(vbenabled)).ok()
    }
    pub unsafe fn IsUserEnabled(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).base__.IsUserEnabled)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn IsComputerEnabled(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).base__.IsComputerEnabled)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn GetSecurityInfo(&self) -> ::windows_core::Result<IGPMSecurityInfo> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetSecurityInfo)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IGPMSecurityInfo>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn SetSecurityInfo<'a, Param0: ::windows_core::IntoParam<'a, IGPMSecurityInfo>>(&self, psecurityinfo: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetSecurityInfo)(::windows_core::Interface::as_raw(self), psecurityinfo.into_param().abi()).ok()
    }
    pub unsafe fn Delete(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Delete)(::windows_core::Interface::as_raw(self)).ok()
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn Backup<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrbackupdir: Param0, bstrcomment: Param1, pvargpmprogress: *const super::Com::VARIANT, pvargpmcancel: *mut super::Com::VARIANT, ppigpmresult: *mut ::core::option::Option<IGPMResult>) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Backup)(::windows_core::Interface::as_raw(self), bstrbackupdir.into_param().abi(), bstrcomment.into_param().abi(), ::core::mem::transmute(pvargpmprogress), ::core::mem::transmute(pvargpmcancel), ::core::mem::transmute(ppigpmresult)).ok()
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn Import<'a, Param1: ::windows_core::IntoParam<'a, IGPMBackup>>(&self, lflags: i32, pigpmbackup: Param1, pvarmigrationtable: *const super::Com::VARIANT, pvargpmprogress: *const super::Com::VARIANT, pvargpmcancel: *mut super::Com::VARIANT, ppigpmresult: *mut ::core::option::Option<IGPMResult>) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Import)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lflags), pigpmbackup.into_param().abi(), ::core::mem::transmute(pvarmigrationtable), ::core::mem::transmute(pvargpmprogress), ::core::mem::transmute(pvargpmcancel), ::core::mem::transmute(ppigpmresult)).ok()
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn GenerateReport(&self, gpmreporttype: GPMReportType, pvargpmprogress: *const super::Com::VARIANT, pvargpmcancel: *mut super::Com::VARIANT, ppigpmresult: *mut ::core::option::Option<IGPMResult>) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GenerateReport)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(gpmreporttype), ::core::mem::transmute(pvargpmprogress), ::core::mem::transmute(pvargpmcancel), ::core::mem::transmute(ppigpmresult)).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn GenerateReportToFile<'a, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, gpmreporttype: GPMReportType, bstrtargetfilepath: Param1) -> ::windows_core::Result<IGPMResult> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GenerateReportToFile)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(gpmreporttype), bstrtargetfilepath.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IGPMResult>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn CopyTo<'a, Param1: ::windows_core::IntoParam<'a, IGPMDomain>>(&self, lflags: i32, pigpmdomain: Param1, pvarnewdisplayname: *const super::Com::VARIANT, pvarmigrationtable: *const super::Com::VARIANT, pvargpmprogress: *const super::Com::VARIANT, pvargpmcancel: *mut super::Com::VARIANT, ppigpmresult: *mut ::core::option::Option<IGPMResult>) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.CopyTo)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lflags), pigpmdomain.into_param().abi(), ::core::mem::transmute(pvarnewdisplayname), ::core::mem::transmute(pvarmigrationtable), ::core::mem::transmute(pvargpmprogress), ::core::mem::transmute(pvargpmcancel), ::core::mem::transmute(ppigpmresult)).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn SetSecurityDescriptor<'a, Param1: ::windows_core::IntoParam<'a, super::Com::IDispatch>>(&self, lflags: i32, psd: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetSecurityDescriptor)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lflags), psd.into_param().abi()).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn GetSecurityDescriptor(&self, lflags: i32) -> ::windows_core::Result<super::Com::IDispatch> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetSecurityDescriptor)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lflags), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::IDispatch>(result__)
    }
    pub unsafe fn IsACLConsistent(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).base__.IsACLConsistent)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn MakeACLConsistent(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.MakeACLConsistent)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Description(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).Description)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetDescription<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, newval: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetDescription)(::windows_core::Interface::as_raw(self), newval.into_param().abi()).ok()
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IGPMGPO2> for ::windows_core::IUnknown {
    fn from(value: IGPMGPO2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IGPMGPO2> for ::windows_core::IUnknown {
    fn from(value: &IGPMGPO2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IGPMGPO2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IGPMGPO2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IGPMGPO2> for super::Com::IDispatch {
    fn from(value: IGPMGPO2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IGPMGPO2> for super::Com::IDispatch {
    fn from(value: &IGPMGPO2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for IGPMGPO2 {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a IGPMGPO2 {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IGPMGPO2> for IGPMGPO {
    fn from(value: IGPMGPO2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IGPMGPO2> for IGPMGPO {
    fn from(value: &IGPMGPO2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, IGPMGPO> for IGPMGPO2 {
    fn into_param(self) -> ::windows_core::Param<'a, IGPMGPO> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, IGPMGPO> for &'a IGPMGPO2 {
    fn into_param(self) -> ::windows_core::Param<'a, IGPMGPO> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for IGPMGPO2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for IGPMGPO2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for IGPMGPO2 {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for IGPMGPO2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IGPMGPO2").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for IGPMGPO2 {
    type Vtable = IGPMGPO2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8a66a210_b78b_4d99_88e2_c306a817c925);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct IGPMGPO2_Vtbl {
    pub base__: IGPMGPO_Vtbl,
    pub Description: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newval: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct IGPMGPO3(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl IGPMGPO3 {
    pub unsafe fn DisplayName(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.DisplayName)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetDisplayName<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, newval: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.SetDisplayName)(::windows_core::Interface::as_raw(self), newval.into_param().abi()).ok()
    }
    pub unsafe fn Path(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.Path)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn ID(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.ID)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn DomainName(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.DomainName)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn CreationTime(&self) -> ::windows_core::Result<f64> {
        let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.CreationTime)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<f64>(result__)
    }
    pub unsafe fn ModificationTime(&self) -> ::windows_core::Result<f64> {
        let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.ModificationTime)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<f64>(result__)
    }
    pub unsafe fn UserDSVersionNumber(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.UserDSVersionNumber)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn ComputerDSVersionNumber(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.ComputerDSVersionNumber)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn UserSysvolVersionNumber(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.UserSysvolVersionNumber)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn ComputerSysvolVersionNumber(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.ComputerSysvolVersionNumber)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn GetWMIFilter(&self) -> ::windows_core::Result<IGPMWMIFilter> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.GetWMIFilter)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IGPMWMIFilter>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn SetWMIFilter<'a, Param0: ::windows_core::IntoParam<'a, IGPMWMIFilter>>(&self, pigpmwmifilter: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.SetWMIFilter)(::windows_core::Interface::as_raw(self), pigpmwmifilter.into_param().abi()).ok()
    }
    pub unsafe fn SetUserEnabled(&self, vbenabled: i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.SetUserEnabled)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(vbenabled)).ok()
    }
    pub unsafe fn SetComputerEnabled(&self, vbenabled: i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.SetComputerEnabled)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(vbenabled)).ok()
    }
    pub unsafe fn IsUserEnabled(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.IsUserEnabled)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn IsComputerEnabled(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.IsComputerEnabled)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn GetSecurityInfo(&self) -> ::windows_core::Result<IGPMSecurityInfo> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.GetSecurityInfo)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IGPMSecurityInfo>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn SetSecurityInfo<'a, Param0: ::windows_core::IntoParam<'a, IGPMSecurityInfo>>(&self, psecurityinfo: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.SetSecurityInfo)(::windows_core::Interface::as_raw(self), psecurityinfo.into_param().abi()).ok()
    }
    pub unsafe fn Delete(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.Delete)(::windows_core::Interface::as_raw(self)).ok()
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn Backup<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrbackupdir: Param0, bstrcomment: Param1, pvargpmprogress: *const super::Com::VARIANT, pvargpmcancel: *mut super::Com::VARIANT, ppigpmresult: *mut ::core::option::Option<IGPMResult>) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.Backup)(::windows_core::Interface::as_raw(self), bstrbackupdir.into_param().abi(), bstrcomment.into_param().abi(), ::core::mem::transmute(pvargpmprogress), ::core::mem::transmute(pvargpmcancel), ::core::mem::transmute(ppigpmresult)).ok()
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn Import<'a, Param1: ::windows_core::IntoParam<'a, IGPMBackup>>(&self, lflags: i32, pigpmbackup: Param1, pvarmigrationtable: *const super::Com::VARIANT, pvargpmprogress: *const super::Com::VARIANT, pvargpmcancel: *mut super::Com::VARIANT, ppigpmresult: *mut ::core::option::Option<IGPMResult>) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.Import)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lflags), pigpmbackup.into_param().abi(), ::core::mem::transmute(pvarmigrationtable), ::core::mem::transmute(pvargpmprogress), ::core::mem::transmute(pvargpmcancel), ::core::mem::transmute(ppigpmresult)).ok()
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn GenerateReport(&self, gpmreporttype: GPMReportType, pvargpmprogress: *const super::Com::VARIANT, pvargpmcancel: *mut super::Com::VARIANT, ppigpmresult: *mut ::core::option::Option<IGPMResult>) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.GenerateReport)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(gpmreporttype), ::core::mem::transmute(pvargpmprogress), ::core::mem::transmute(pvargpmcancel), ::core::mem::transmute(ppigpmresult)).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn GenerateReportToFile<'a, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, gpmreporttype: GPMReportType, bstrtargetfilepath: Param1) -> ::windows_core::Result<IGPMResult> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.GenerateReportToFile)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(gpmreporttype), bstrtargetfilepath.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IGPMResult>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn CopyTo<'a, Param1: ::windows_core::IntoParam<'a, IGPMDomain>>(&self, lflags: i32, pigpmdomain: Param1, pvarnewdisplayname: *const super::Com::VARIANT, pvarmigrationtable: *const super::Com::VARIANT, pvargpmprogress: *const super::Com::VARIANT, pvargpmcancel: *mut super::Com::VARIANT, ppigpmresult: *mut ::core::option::Option<IGPMResult>) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.CopyTo)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lflags), pigpmdomain.into_param().abi(), ::core::mem::transmute(pvarnewdisplayname), ::core::mem::transmute(pvarmigrationtable), ::core::mem::transmute(pvargpmprogress), ::core::mem::transmute(pvargpmcancel), ::core::mem::transmute(ppigpmresult)).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn SetSecurityDescriptor<'a, Param1: ::windows_core::IntoParam<'a, super::Com::IDispatch>>(&self, lflags: i32, psd: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.SetSecurityDescriptor)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lflags), psd.into_param().abi()).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn GetSecurityDescriptor(&self, lflags: i32) -> ::windows_core::Result<super::Com::IDispatch> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.GetSecurityDescriptor)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lflags), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::IDispatch>(result__)
    }
    pub unsafe fn IsACLConsistent(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.IsACLConsistent)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn MakeACLConsistent(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.MakeACLConsistent)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Description(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.Description)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetDescription<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, newval: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetDescription)(::windows_core::Interface::as_raw(self), newval.into_param().abi()).ok()
    }
    pub unsafe fn InfrastructureDC(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).InfrastructureDC)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetInfrastructureDC<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, newval: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetInfrastructureDC)(::windows_core::Interface::as_raw(self), newval.into_param().abi()).ok()
    }
    pub unsafe fn SetInfrastructureFlags(&self, dwflags: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetInfrastructureFlags)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwflags)).ok()
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IGPMGPO3> for ::windows_core::IUnknown {
    fn from(value: IGPMGPO3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IGPMGPO3> for ::windows_core::IUnknown {
    fn from(value: &IGPMGPO3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IGPMGPO3 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IGPMGPO3 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IGPMGPO3> for super::Com::IDispatch {
    fn from(value: IGPMGPO3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IGPMGPO3> for super::Com::IDispatch {
    fn from(value: &IGPMGPO3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for IGPMGPO3 {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a IGPMGPO3 {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IGPMGPO3> for IGPMGPO {
    fn from(value: IGPMGPO3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IGPMGPO3> for IGPMGPO {
    fn from(value: &IGPMGPO3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, IGPMGPO> for IGPMGPO3 {
    fn into_param(self) -> ::windows_core::Param<'a, IGPMGPO> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, IGPMGPO> for &'a IGPMGPO3 {
    fn into_param(self) -> ::windows_core::Param<'a, IGPMGPO> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IGPMGPO3> for IGPMGPO2 {
    fn from(value: IGPMGPO3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IGPMGPO3> for IGPMGPO2 {
    fn from(value: &IGPMGPO3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, IGPMGPO2> for IGPMGPO3 {
    fn into_param(self) -> ::windows_core::Param<'a, IGPMGPO2> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, IGPMGPO2> for &'a IGPMGPO3 {
    fn into_param(self) -> ::windows_core::Param<'a, IGPMGPO2> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for IGPMGPO3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for IGPMGPO3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for IGPMGPO3 {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for IGPMGPO3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IGPMGPO3").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for IGPMGPO3 {
    type Vtable = IGPMGPO3_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7cf123a1_f94a_4112_bfae_6aa1db9cb248);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct IGPMGPO3_Vtbl {
    pub base__: IGPMGPO2_Vtbl,
    pub InfrastructureDC: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SetInfrastructureDC: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newval: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub SetInfrastructureFlags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwflags: u32) -> ::windows_core::HRESULT,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct IGPMGPOCollection(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl IGPMGPOCollection {
    pub unsafe fn Count(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).Count)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn get_Item(&self, lindex: i32) -> ::windows_core::Result<super::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).get_Item)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lindex), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::VARIANT>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn _NewEnum(&self) -> ::windows_core::Result<super::Ole::IEnumVARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self)._NewEnum)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Ole::IEnumVARIANT>(result__)
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IGPMGPOCollection> for ::windows_core::IUnknown {
    fn from(value: IGPMGPOCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IGPMGPOCollection> for ::windows_core::IUnknown {
    fn from(value: &IGPMGPOCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IGPMGPOCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IGPMGPOCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IGPMGPOCollection> for super::Com::IDispatch {
    fn from(value: IGPMGPOCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IGPMGPOCollection> for super::Com::IDispatch {
    fn from(value: &IGPMGPOCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for IGPMGPOCollection {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a IGPMGPOCollection {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for IGPMGPOCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for IGPMGPOCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for IGPMGPOCollection {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for IGPMGPOCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IGPMGPOCollection").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for IGPMGPOCollection {
    type Vtable = IGPMGPOCollection_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf0f0d5cf_70ca_4c39_9e29_b642f8726c01);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct IGPMGPOCollection_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub get_Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lindex: i32, pval: *mut super::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    get_Item: usize,
    #[cfg(feature = "win32-system")]
    pub _NewEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppigpmgpos: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    _NewEnum: usize,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct IGPMGPOLink(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl IGPMGPOLink {
    pub unsafe fn GPOID(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).GPOID)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn GPODomain(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).GPODomain)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn Enabled(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).Enabled)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn SetEnabled(&self, newval: i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetEnabled)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(newval)).ok()
    }
    pub unsafe fn Enforced(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).Enforced)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn SetEnforced(&self, newval: i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetEnforced)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(newval)).ok()
    }
    pub unsafe fn SOMLinkOrder(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).SOMLinkOrder)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn SOM(&self) -> ::windows_core::Result<IGPMSOM> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).SOM)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IGPMSOM>(result__)
    }
    pub unsafe fn Delete(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Delete)(::windows_core::Interface::as_raw(self)).ok()
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IGPMGPOLink> for ::windows_core::IUnknown {
    fn from(value: IGPMGPOLink) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IGPMGPOLink> for ::windows_core::IUnknown {
    fn from(value: &IGPMGPOLink) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IGPMGPOLink {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IGPMGPOLink {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IGPMGPOLink> for super::Com::IDispatch {
    fn from(value: IGPMGPOLink) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IGPMGPOLink> for super::Com::IDispatch {
    fn from(value: &IGPMGPOLink) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for IGPMGPOLink {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a IGPMGPOLink {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for IGPMGPOLink {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for IGPMGPOLink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for IGPMGPOLink {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for IGPMGPOLink {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IGPMGPOLink").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for IGPMGPOLink {
    type Vtable = IGPMGPOLink_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x434b99bd_5de7_478a_809c_c251721df70c);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct IGPMGPOLink_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub GPOID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub GPODomain: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub Enabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut i16) -> ::windows_core::HRESULT,
    pub SetEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newval: i16) -> ::windows_core::HRESULT,
    pub Enforced: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut i16) -> ::windows_core::HRESULT,
    pub SetEnforced: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newval: i16) -> ::windows_core::HRESULT,
    pub SOMLinkOrder: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lval: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(feature = "win32-system")]
    pub SOM: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppigpmsom: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    SOM: usize,
    pub Delete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct IGPMGPOLinksCollection(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl IGPMGPOLinksCollection {
    pub unsafe fn Count(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).Count)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn get_Item(&self, lindex: i32) -> ::windows_core::Result<super::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).get_Item)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lindex), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::VARIANT>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn _NewEnum(&self) -> ::windows_core::Result<super::Ole::IEnumVARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self)._NewEnum)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Ole::IEnumVARIANT>(result__)
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IGPMGPOLinksCollection> for ::windows_core::IUnknown {
    fn from(value: IGPMGPOLinksCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IGPMGPOLinksCollection> for ::windows_core::IUnknown {
    fn from(value: &IGPMGPOLinksCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IGPMGPOLinksCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IGPMGPOLinksCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IGPMGPOLinksCollection> for super::Com::IDispatch {
    fn from(value: IGPMGPOLinksCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IGPMGPOLinksCollection> for super::Com::IDispatch {
    fn from(value: &IGPMGPOLinksCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for IGPMGPOLinksCollection {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a IGPMGPOLinksCollection {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for IGPMGPOLinksCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for IGPMGPOLinksCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for IGPMGPOLinksCollection {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for IGPMGPOLinksCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IGPMGPOLinksCollection").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for IGPMGPOLinksCollection {
    type Vtable = IGPMGPOLinksCollection_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x189d7b68_16bd_4d0d_a2ec_2e6aa2288c7f);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct IGPMGPOLinksCollection_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub get_Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lindex: i32, pval: *mut super::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    get_Item: usize,
    #[cfg(feature = "win32-system")]
    pub _NewEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppigpmlinks: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    _NewEnum: usize,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct IGPMMapEntry(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl IGPMMapEntry {
    pub unsafe fn Source(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).Source)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn Destination(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).Destination)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn DestinationOption(&self) -> ::windows_core::Result<GPMDestinationOption> {
        let mut result__ = ::core::mem::MaybeUninit::<GPMDestinationOption>::zeroed();
        (::windows_core::Interface::vtable(self).DestinationOption)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<GPMDestinationOption>(result__)
    }
    pub unsafe fn EntryType(&self) -> ::windows_core::Result<GPMEntryType> {
        let mut result__ = ::core::mem::MaybeUninit::<GPMEntryType>::zeroed();
        (::windows_core::Interface::vtable(self).EntryType)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<GPMEntryType>(result__)
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IGPMMapEntry> for ::windows_core::IUnknown {
    fn from(value: IGPMMapEntry) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IGPMMapEntry> for ::windows_core::IUnknown {
    fn from(value: &IGPMMapEntry) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IGPMMapEntry {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IGPMMapEntry {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IGPMMapEntry> for super::Com::IDispatch {
    fn from(value: IGPMMapEntry) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IGPMMapEntry> for super::Com::IDispatch {
    fn from(value: &IGPMMapEntry) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for IGPMMapEntry {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a IGPMMapEntry {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for IGPMMapEntry {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for IGPMMapEntry {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for IGPMMapEntry {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for IGPMMapEntry {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IGPMMapEntry").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for IGPMMapEntry {
    type Vtable = IGPMMapEntry_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8e79ad06_2381_4444_be4c_ff693e6e6f2b);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct IGPMMapEntry_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Source: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrsource: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub Destination: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrdestination: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub DestinationOption: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pgpmdestoption: *mut GPMDestinationOption) -> ::windows_core::HRESULT,
    pub EntryType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pgpmentrytype: *mut GPMEntryType) -> ::windows_core::HRESULT,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct IGPMMapEntryCollection(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl IGPMMapEntryCollection {
    pub unsafe fn Count(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).Count)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn get_Item(&self, lindex: i32) -> ::windows_core::Result<super::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).get_Item)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lindex), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::VARIANT>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn _NewEnum(&self) -> ::windows_core::Result<super::Ole::IEnumVARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self)._NewEnum)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Ole::IEnumVARIANT>(result__)
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IGPMMapEntryCollection> for ::windows_core::IUnknown {
    fn from(value: IGPMMapEntryCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IGPMMapEntryCollection> for ::windows_core::IUnknown {
    fn from(value: &IGPMMapEntryCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IGPMMapEntryCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IGPMMapEntryCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IGPMMapEntryCollection> for super::Com::IDispatch {
    fn from(value: IGPMMapEntryCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IGPMMapEntryCollection> for super::Com::IDispatch {
    fn from(value: &IGPMMapEntryCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for IGPMMapEntryCollection {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a IGPMMapEntryCollection {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for IGPMMapEntryCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for IGPMMapEntryCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for IGPMMapEntryCollection {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for IGPMMapEntryCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IGPMMapEntryCollection").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for IGPMMapEntryCollection {
    type Vtable = IGPMMapEntryCollection_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xbb0bf49b_e53f_443f_b807_8be22bfb6d42);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct IGPMMapEntryCollection_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub get_Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lindex: i32, pval: *mut super::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    get_Item: usize,
    #[cfg(feature = "win32-system")]
    pub _NewEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    _NewEnum: usize,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct IGPMMigrationTable(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl IGPMMigrationTable {
    pub unsafe fn Save<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrmigrationtablepath: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Save)(::windows_core::Interface::as_raw(self), bstrmigrationtablepath.into_param().abi()).ok()
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn Add<'a, Param1: ::windows_core::IntoParam<'a, super::Com::VARIANT>>(&self, lflags: i32, var: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Add)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lflags), var.into_param().abi()).ok()
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn AddEntry<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrsource: Param0, gpmentrytype: GPMEntryType, pvardestination: *const super::Com::VARIANT) -> ::windows_core::Result<IGPMMapEntry> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).AddEntry)(::windows_core::Interface::as_raw(self), bstrsource.into_param().abi(), ::core::mem::transmute(gpmentrytype), ::core::mem::transmute(pvardestination), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IGPMMapEntry>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn GetEntry<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrsource: Param0) -> ::windows_core::Result<IGPMMapEntry> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetEntry)(::windows_core::Interface::as_raw(self), bstrsource.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IGPMMapEntry>(result__)
    }
    pub unsafe fn DeleteEntry<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrsource: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).DeleteEntry)(::windows_core::Interface::as_raw(self), bstrsource.into_param().abi()).ok()
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn UpdateDestination<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrsource: Param0, pvardestination: *const super::Com::VARIANT) -> ::windows_core::Result<IGPMMapEntry> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).UpdateDestination)(::windows_core::Interface::as_raw(self), bstrsource.into_param().abi(), ::core::mem::transmute(pvardestination), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IGPMMapEntry>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn Validate(&self) -> ::windows_core::Result<IGPMResult> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Validate)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IGPMResult>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn GetEntries(&self) -> ::windows_core::Result<IGPMMapEntryCollection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetEntries)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IGPMMapEntryCollection>(result__)
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IGPMMigrationTable> for ::windows_core::IUnknown {
    fn from(value: IGPMMigrationTable) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IGPMMigrationTable> for ::windows_core::IUnknown {
    fn from(value: &IGPMMigrationTable) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IGPMMigrationTable {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IGPMMigrationTable {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IGPMMigrationTable> for super::Com::IDispatch {
    fn from(value: IGPMMigrationTable) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IGPMMigrationTable> for super::Com::IDispatch {
    fn from(value: &IGPMMigrationTable) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for IGPMMigrationTable {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a IGPMMigrationTable {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for IGPMMigrationTable {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for IGPMMigrationTable {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for IGPMMigrationTable {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for IGPMMigrationTable {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IGPMMigrationTable").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for IGPMMigrationTable {
    type Vtable = IGPMMigrationTable_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x48f823b1_efaf_470b_b6ed_40d14ee1a4ec);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct IGPMMigrationTable_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Save: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrmigrationtablepath: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub Add: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lflags: i32, var: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    Add: usize,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub AddEntry: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrsource: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, gpmentrytype: GPMEntryType, pvardestination: *const super::Com::VARIANT, ppentry: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    AddEntry: usize,
    #[cfg(feature = "win32-system")]
    pub GetEntry: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrsource: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, ppentry: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    GetEntry: usize,
    pub DeleteEntry: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrsource: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub UpdateDestination: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrsource: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, pvardestination: *const super::Com::VARIANT, ppentry: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    UpdateDestination: usize,
    #[cfg(feature = "win32-system")]
    pub Validate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppresult: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    Validate: usize,
    #[cfg(feature = "win32-system")]
    pub GetEntries: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppentries: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    GetEntries: usize,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct IGPMPermission(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl IGPMPermission {
    pub unsafe fn Inherited(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).Inherited)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn Inheritable(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).Inheritable)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn Denied(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).Denied)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn Permission(&self) -> ::windows_core::Result<GPMPermissionType> {
        let mut result__ = ::core::mem::MaybeUninit::<GPMPermissionType>::zeroed();
        (::windows_core::Interface::vtable(self).Permission)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<GPMPermissionType>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn Trustee(&self) -> ::windows_core::Result<IGPMTrustee> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Trustee)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IGPMTrustee>(result__)
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IGPMPermission> for ::windows_core::IUnknown {
    fn from(value: IGPMPermission) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IGPMPermission> for ::windows_core::IUnknown {
    fn from(value: &IGPMPermission) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IGPMPermission {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IGPMPermission {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IGPMPermission> for super::Com::IDispatch {
    fn from(value: IGPMPermission) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IGPMPermission> for super::Com::IDispatch {
    fn from(value: &IGPMPermission) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for IGPMPermission {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a IGPMPermission {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for IGPMPermission {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for IGPMPermission {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for IGPMPermission {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for IGPMPermission {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IGPMPermission").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for IGPMPermission {
    type Vtable = IGPMPermission_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x35ebca40_e1a1_4a02_8905_d79416fb464a);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct IGPMPermission_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Inherited: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut i16) -> ::windows_core::HRESULT,
    pub Inheritable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut i16) -> ::windows_core::HRESULT,
    pub Denied: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut i16) -> ::windows_core::HRESULT,
    pub Permission: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut GPMPermissionType) -> ::windows_core::HRESULT,
    #[cfg(feature = "win32-system")]
    pub Trustee: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppigpmtrustee: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    Trustee: usize,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct IGPMRSOP(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl IGPMRSOP {
    pub unsafe fn Mode(&self) -> ::windows_core::Result<GPMRSOPMode> {
        let mut result__ = ::core::mem::MaybeUninit::<GPMRSOPMode>::zeroed();
        (::windows_core::Interface::vtable(self).Mode)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<GPMRSOPMode>(result__)
    }
    pub unsafe fn Namespace(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).Namespace)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetLoggingComputer<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrval: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetLoggingComputer)(::windows_core::Interface::as_raw(self), bstrval.into_param().abi()).ok()
    }
    pub unsafe fn LoggingComputer(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).LoggingComputer)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetLoggingUser<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrval: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetLoggingUser)(::windows_core::Interface::as_raw(self), bstrval.into_param().abi()).ok()
    }
    pub unsafe fn LoggingUser(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).LoggingUser)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetLoggingFlags(&self, lval: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetLoggingFlags)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lval)).ok()
    }
    pub unsafe fn LoggingFlags(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).LoggingFlags)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetPlanningFlags(&self, lval: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetPlanningFlags)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lval)).ok()
    }
    pub unsafe fn PlanningFlags(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).PlanningFlags)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetPlanningDomainController<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrval: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetPlanningDomainController)(::windows_core::Interface::as_raw(self), bstrval.into_param().abi()).ok()
    }
    pub unsafe fn PlanningDomainController(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).PlanningDomainController)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetPlanningSiteName<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrval: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetPlanningSiteName)(::windows_core::Interface::as_raw(self), bstrval.into_param().abi()).ok()
    }
    pub unsafe fn PlanningSiteName(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).PlanningSiteName)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetPlanningUser<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrval: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetPlanningUser)(::windows_core::Interface::as_raw(self), bstrval.into_param().abi()).ok()
    }
    pub unsafe fn PlanningUser(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).PlanningUser)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetPlanningUserSOM<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrval: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetPlanningUserSOM)(::windows_core::Interface::as_raw(self), bstrval.into_param().abi()).ok()
    }
    pub unsafe fn PlanningUserSOM(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).PlanningUserSOM)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn SetPlanningUserWMIFilters<'a, Param0: ::windows_core::IntoParam<'a, super::Com::VARIANT>>(&self, varval: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetPlanningUserWMIFilters)(::windows_core::Interface::as_raw(self), varval.into_param().abi()).ok()
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn PlanningUserWMIFilters(&self) -> ::windows_core::Result<super::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).PlanningUserWMIFilters)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn SetPlanningUserSecurityGroups<'a, Param0: ::windows_core::IntoParam<'a, super::Com::VARIANT>>(&self, varval: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetPlanningUserSecurityGroups)(::windows_core::Interface::as_raw(self), varval.into_param().abi()).ok()
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn PlanningUserSecurityGroups(&self) -> ::windows_core::Result<super::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).PlanningUserSecurityGroups)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::VARIANT>(result__)
    }
    pub unsafe fn SetPlanningComputer<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrval: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetPlanningComputer)(::windows_core::Interface::as_raw(self), bstrval.into_param().abi()).ok()
    }
    pub unsafe fn PlanningComputer(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).PlanningComputer)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetPlanningComputerSOM<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrval: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetPlanningComputerSOM)(::windows_core::Interface::as_raw(self), bstrval.into_param().abi()).ok()
    }
    pub unsafe fn PlanningComputerSOM(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).PlanningComputerSOM)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn SetPlanningComputerWMIFilters<'a, Param0: ::windows_core::IntoParam<'a, super::Com::VARIANT>>(&self, varval: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetPlanningComputerWMIFilters)(::windows_core::Interface::as_raw(self), varval.into_param().abi()).ok()
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn PlanningComputerWMIFilters(&self) -> ::windows_core::Result<super::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).PlanningComputerWMIFilters)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn SetPlanningComputerSecurityGroups<'a, Param0: ::windows_core::IntoParam<'a, super::Com::VARIANT>>(&self, varval: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetPlanningComputerSecurityGroups)(::windows_core::Interface::as_raw(self), varval.into_param().abi()).ok()
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn PlanningComputerSecurityGroups(&self) -> ::windows_core::Result<super::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).PlanningComputerSecurityGroups)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn LoggingEnumerateUsers(&self) -> ::windows_core::Result<super::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).LoggingEnumerateUsers)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::VARIANT>(result__)
    }
    pub unsafe fn CreateQueryResults(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).CreateQueryResults)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn ReleaseQueryResults(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ReleaseQueryResults)(::windows_core::Interface::as_raw(self)).ok()
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn GenerateReport(&self, gpmreporttype: GPMReportType, pvargpmprogress: *const super::Com::VARIANT, pvargpmcancel: *mut super::Com::VARIANT, ppigpmresult: *mut ::core::option::Option<IGPMResult>) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GenerateReport)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(gpmreporttype), ::core::mem::transmute(pvargpmprogress), ::core::mem::transmute(pvargpmcancel), ::core::mem::transmute(ppigpmresult)).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn GenerateReportToFile<'a, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, gpmreporttype: GPMReportType, bstrtargetfilepath: Param1) -> ::windows_core::Result<IGPMResult> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GenerateReportToFile)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(gpmreporttype), bstrtargetfilepath.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IGPMResult>(result__)
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IGPMRSOP> for ::windows_core::IUnknown {
    fn from(value: IGPMRSOP) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IGPMRSOP> for ::windows_core::IUnknown {
    fn from(value: &IGPMRSOP) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IGPMRSOP {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IGPMRSOP {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IGPMRSOP> for super::Com::IDispatch {
    fn from(value: IGPMRSOP) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IGPMRSOP> for super::Com::IDispatch {
    fn from(value: &IGPMRSOP) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for IGPMRSOP {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a IGPMRSOP {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for IGPMRSOP {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for IGPMRSOP {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for IGPMRSOP {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for IGPMRSOP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IGPMRSOP").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for IGPMRSOP {
    type Vtable = IGPMRSOP_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x49ed785a_3237_4ff2_b1f0_fdf5a8d5a1ee);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct IGPMRSOP_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Mode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut GPMRSOPMode) -> ::windows_core::HRESULT,
    pub Namespace: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrval: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SetLoggingComputer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrval: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub LoggingComputer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrval: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SetLoggingUser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrval: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub LoggingUser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrval: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SetLoggingFlags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lval: i32) -> ::windows_core::HRESULT,
    pub LoggingFlags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lval: *mut i32) -> ::windows_core::HRESULT,
    pub SetPlanningFlags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lval: i32) -> ::windows_core::HRESULT,
    pub PlanningFlags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lval: *mut i32) -> ::windows_core::HRESULT,
    pub SetPlanningDomainController: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrval: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub PlanningDomainController: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrval: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SetPlanningSiteName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrval: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub PlanningSiteName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrval: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SetPlanningUser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrval: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub PlanningUser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrval: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SetPlanningUserSOM: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrval: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub PlanningUserSOM: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrval: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub SetPlanningUserWMIFilters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, varval: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    SetPlanningUserWMIFilters: usize,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub PlanningUserWMIFilters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, varval: *mut super::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    PlanningUserWMIFilters: usize,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub SetPlanningUserSecurityGroups: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, varval: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    SetPlanningUserSecurityGroups: usize,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub PlanningUserSecurityGroups: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, varval: *mut super::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    PlanningUserSecurityGroups: usize,
    pub SetPlanningComputer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrval: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub PlanningComputer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrval: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SetPlanningComputerSOM: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrval: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub PlanningComputerSOM: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrval: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub SetPlanningComputerWMIFilters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, varval: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    SetPlanningComputerWMIFilters: usize,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub PlanningComputerWMIFilters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, varval: *mut super::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    PlanningComputerWMIFilters: usize,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub SetPlanningComputerSecurityGroups: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, varval: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    SetPlanningComputerSecurityGroups: usize,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub PlanningComputerSecurityGroups: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, varval: *mut super::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    PlanningComputerSecurityGroups: usize,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub LoggingEnumerateUsers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, varval: *mut super::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    LoggingEnumerateUsers: usize,
    pub CreateQueryResults: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub ReleaseQueryResults: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub GenerateReport: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, gpmreporttype: GPMReportType, pvargpmprogress: *const super::Com::VARIANT, pvargpmcancel: *mut super::Com::VARIANT, ppigpmresult: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    GenerateReport: usize,
    #[cfg(feature = "win32-system")]
    pub GenerateReportToFile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, gpmreporttype: GPMReportType, bstrtargetfilepath: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, ppigpmresult: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    GenerateReportToFile: usize,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct IGPMResult(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl IGPMResult {
    #[cfg(feature = "win32-system")]
    pub unsafe fn Status(&self) -> ::windows_core::Result<IGPMStatusMsgCollection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Status)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IGPMStatusMsgCollection>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn Result(&self) -> ::windows_core::Result<super::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).Result)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::VARIANT>(result__)
    }
    pub unsafe fn OverallStatus(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).OverallStatus)(::windows_core::Interface::as_raw(self)).ok()
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IGPMResult> for ::windows_core::IUnknown {
    fn from(value: IGPMResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IGPMResult> for ::windows_core::IUnknown {
    fn from(value: &IGPMResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IGPMResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IGPMResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IGPMResult> for super::Com::IDispatch {
    fn from(value: IGPMResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IGPMResult> for super::Com::IDispatch {
    fn from(value: &IGPMResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for IGPMResult {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a IGPMResult {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for IGPMResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for IGPMResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for IGPMResult {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for IGPMResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IGPMResult").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for IGPMResult {
    type Vtable = IGPMResult_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x86dff7e9_f76f_42ab_9570_cebc6be8a52d);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct IGPMResult_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    #[cfg(feature = "win32-system")]
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppigpmstatusmsgcollection: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    Status: usize,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub Result: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvarresult: *mut super::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    Result: usize,
    pub OverallStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct IGPMSOM(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl IGPMSOM {
    pub unsafe fn GPOInheritanceBlocked(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).GPOInheritanceBlocked)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn SetGPOInheritanceBlocked(&self, newval: i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetGPOInheritanceBlocked)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(newval)).ok()
    }
    pub unsafe fn Name(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).Name)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn Path(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).Path)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn CreateGPOLink<'a, Param1: ::windows_core::IntoParam<'a, IGPMGPO>>(&self, llinkpos: i32, pgpo: Param1) -> ::windows_core::Result<IGPMGPOLink> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).CreateGPOLink)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(llinkpos), pgpo.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IGPMGPOLink>(result__)
    }
    pub unsafe fn Type(&self) -> ::windows_core::Result<GPMSOMType> {
        let mut result__ = ::core::mem::MaybeUninit::<GPMSOMType>::zeroed();
        (::windows_core::Interface::vtable(self).Type)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<GPMSOMType>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn GetGPOLinks(&self) -> ::windows_core::Result<IGPMGPOLinksCollection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetGPOLinks)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IGPMGPOLinksCollection>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn GetInheritedGPOLinks(&self) -> ::windows_core::Result<IGPMGPOLinksCollection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetInheritedGPOLinks)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IGPMGPOLinksCollection>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn GetSecurityInfo(&self) -> ::windows_core::Result<IGPMSecurityInfo> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetSecurityInfo)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IGPMSecurityInfo>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn SetSecurityInfo<'a, Param0: ::windows_core::IntoParam<'a, IGPMSecurityInfo>>(&self, psecurityinfo: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetSecurityInfo)(::windows_core::Interface::as_raw(self), psecurityinfo.into_param().abi()).ok()
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IGPMSOM> for ::windows_core::IUnknown {
    fn from(value: IGPMSOM) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IGPMSOM> for ::windows_core::IUnknown {
    fn from(value: &IGPMSOM) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IGPMSOM {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IGPMSOM {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IGPMSOM> for super::Com::IDispatch {
    fn from(value: IGPMSOM) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IGPMSOM> for super::Com::IDispatch {
    fn from(value: &IGPMSOM) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for IGPMSOM {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a IGPMSOM {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for IGPMSOM {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for IGPMSOM {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for IGPMSOM {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for IGPMSOM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IGPMSOM").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for IGPMSOM {
    type Vtable = IGPMSOM_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc0a7f09e_05a1_4f0c_8158_9e5c33684f6b);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct IGPMSOM_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub GPOInheritanceBlocked: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut i16) -> ::windows_core::HRESULT,
    pub SetGPOInheritanceBlocked: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newval: i16) -> ::windows_core::HRESULT,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub Path: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    #[cfg(feature = "win32-system")]
    pub CreateGPOLink: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, llinkpos: i32, pgpo: ::windows_core::RawPtr, ppnewgpolink: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    CreateGPOLink: usize,
    pub Type: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut GPMSOMType) -> ::windows_core::HRESULT,
    #[cfg(feature = "win32-system")]
    pub GetGPOLinks: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppgpolinks: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    GetGPOLinks: usize,
    #[cfg(feature = "win32-system")]
    pub GetInheritedGPOLinks: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppgpolinks: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    GetInheritedGPOLinks: usize,
    #[cfg(feature = "win32-system")]
    pub GetSecurityInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppsecurityinfo: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    GetSecurityInfo: usize,
    #[cfg(feature = "win32-system")]
    pub SetSecurityInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psecurityinfo: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    SetSecurityInfo: usize,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct IGPMSOMCollection(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl IGPMSOMCollection {
    pub unsafe fn Count(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).Count)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn get_Item(&self, lindex: i32) -> ::windows_core::Result<super::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).get_Item)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lindex), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::VARIANT>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn _NewEnum(&self) -> ::windows_core::Result<super::Ole::IEnumVARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self)._NewEnum)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Ole::IEnumVARIANT>(result__)
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IGPMSOMCollection> for ::windows_core::IUnknown {
    fn from(value: IGPMSOMCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IGPMSOMCollection> for ::windows_core::IUnknown {
    fn from(value: &IGPMSOMCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IGPMSOMCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IGPMSOMCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IGPMSOMCollection> for super::Com::IDispatch {
    fn from(value: IGPMSOMCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IGPMSOMCollection> for super::Com::IDispatch {
    fn from(value: &IGPMSOMCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for IGPMSOMCollection {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a IGPMSOMCollection {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for IGPMSOMCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for IGPMSOMCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for IGPMSOMCollection {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for IGPMSOMCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IGPMSOMCollection").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for IGPMSOMCollection {
    type Vtable = IGPMSOMCollection_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xadc1688e_00e4_4495_abba_bed200df0cab);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct IGPMSOMCollection_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub get_Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lindex: i32, pval: *mut super::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    get_Item: usize,
    #[cfg(feature = "win32-system")]
    pub _NewEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppigpmsom: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    _NewEnum: usize,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct IGPMSearchCriteria(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl IGPMSearchCriteria {
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn Add<'a, Param2: ::windows_core::IntoParam<'a, super::Com::VARIANT>>(&self, searchproperty: GPMSearchProperty, searchoperation: GPMSearchOperation, varvalue: Param2) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Add)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(searchproperty), ::core::mem::transmute(searchoperation), varvalue.into_param().abi()).ok()
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IGPMSearchCriteria> for ::windows_core::IUnknown {
    fn from(value: IGPMSearchCriteria) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IGPMSearchCriteria> for ::windows_core::IUnknown {
    fn from(value: &IGPMSearchCriteria) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IGPMSearchCriteria {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IGPMSearchCriteria {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IGPMSearchCriteria> for super::Com::IDispatch {
    fn from(value: IGPMSearchCriteria) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IGPMSearchCriteria> for super::Com::IDispatch {
    fn from(value: &IGPMSearchCriteria) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for IGPMSearchCriteria {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a IGPMSearchCriteria {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for IGPMSearchCriteria {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for IGPMSearchCriteria {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for IGPMSearchCriteria {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for IGPMSearchCriteria {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IGPMSearchCriteria").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for IGPMSearchCriteria {
    type Vtable = IGPMSearchCriteria_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd6f11c42_829b_48d4_83f5_3615b67dfc22);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct IGPMSearchCriteria_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub Add: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, searchproperty: GPMSearchProperty, searchoperation: GPMSearchOperation, varvalue: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    Add: usize,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct IGPMSecurityInfo(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl IGPMSecurityInfo {
    pub unsafe fn Count(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).Count)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn get_Item(&self, lindex: i32) -> ::windows_core::Result<super::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).get_Item)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lindex), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::VARIANT>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn _NewEnum(&self) -> ::windows_core::Result<super::Ole::IEnumVARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self)._NewEnum)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Ole::IEnumVARIANT>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn Add<'a, Param0: ::windows_core::IntoParam<'a, IGPMPermission>>(&self, pperm: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Add)(::windows_core::Interface::as_raw(self), pperm.into_param().abi()).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn Remove<'a, Param0: ::windows_core::IntoParam<'a, IGPMPermission>>(&self, pperm: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Remove)(::windows_core::Interface::as_raw(self), pperm.into_param().abi()).ok()
    }
    pub unsafe fn RemoveTrustee<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrtrustee: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).RemoveTrustee)(::windows_core::Interface::as_raw(self), bstrtrustee.into_param().abi()).ok()
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IGPMSecurityInfo> for ::windows_core::IUnknown {
    fn from(value: IGPMSecurityInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IGPMSecurityInfo> for ::windows_core::IUnknown {
    fn from(value: &IGPMSecurityInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IGPMSecurityInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IGPMSecurityInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IGPMSecurityInfo> for super::Com::IDispatch {
    fn from(value: IGPMSecurityInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IGPMSecurityInfo> for super::Com::IDispatch {
    fn from(value: &IGPMSecurityInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for IGPMSecurityInfo {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a IGPMSecurityInfo {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for IGPMSecurityInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for IGPMSecurityInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for IGPMSecurityInfo {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for IGPMSecurityInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IGPMSecurityInfo").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for IGPMSecurityInfo {
    type Vtable = IGPMSecurityInfo_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb6c31ed4_1c93_4d3e_ae84_eb6d61161b60);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct IGPMSecurityInfo_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub get_Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lindex: i32, pval: *mut super::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    get_Item: usize,
    #[cfg(feature = "win32-system")]
    pub _NewEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    _NewEnum: usize,
    #[cfg(feature = "win32-system")]
    pub Add: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pperm: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    Add: usize,
    #[cfg(feature = "win32-system")]
    pub Remove: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pperm: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    Remove: usize,
    pub RemoveTrustee: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrtrustee: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct IGPMSitesContainer(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl IGPMSitesContainer {
    pub unsafe fn DomainController(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).DomainController)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn Domain(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).Domain)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn Forest(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).Forest)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn GetSite<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrsitename: Param0) -> ::windows_core::Result<IGPMSOM> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetSite)(::windows_core::Interface::as_raw(self), bstrsitename.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IGPMSOM>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn SearchSites<'a, Param0: ::windows_core::IntoParam<'a, IGPMSearchCriteria>>(&self, pigpmsearchcriteria: Param0) -> ::windows_core::Result<IGPMSOMCollection> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).SearchSites)(::windows_core::Interface::as_raw(self), pigpmsearchcriteria.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IGPMSOMCollection>(result__)
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IGPMSitesContainer> for ::windows_core::IUnknown {
    fn from(value: IGPMSitesContainer) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IGPMSitesContainer> for ::windows_core::IUnknown {
    fn from(value: &IGPMSitesContainer) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IGPMSitesContainer {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IGPMSitesContainer {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IGPMSitesContainer> for super::Com::IDispatch {
    fn from(value: IGPMSitesContainer) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IGPMSitesContainer> for super::Com::IDispatch {
    fn from(value: &IGPMSitesContainer) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for IGPMSitesContainer {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a IGPMSitesContainer {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for IGPMSitesContainer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for IGPMSitesContainer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for IGPMSitesContainer {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for IGPMSitesContainer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IGPMSitesContainer").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for IGPMSitesContainer {
    type Vtable = IGPMSitesContainer_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4725a899_2782_4d27_a6bb_d499246ffd72);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct IGPMSitesContainer_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub DomainController: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub Domain: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub Forest: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    #[cfg(feature = "win32-system")]
    pub GetSite: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrsitename: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, ppsom: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    GetSite: usize,
    #[cfg(feature = "win32-system")]
    pub SearchSites: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pigpmsearchcriteria: ::windows_core::RawPtr, ppigpmsomcollection: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    SearchSites: usize,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct IGPMStarterGPO(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl IGPMStarterGPO {
    pub unsafe fn DisplayName(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).DisplayName)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetDisplayName<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, newval: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetDisplayName)(::windows_core::Interface::as_raw(self), newval.into_param().abi()).ok()
    }
    pub unsafe fn Description(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).Description)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetDescription<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, newval: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetDescription)(::windows_core::Interface::as_raw(self), newval.into_param().abi()).ok()
    }
    pub unsafe fn Author(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).Author)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn Product(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).Product)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn CreationTime(&self) -> ::windows_core::Result<f64> {
        let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
        (::windows_core::Interface::vtable(self).CreationTime)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<f64>(result__)
    }
    pub unsafe fn ID(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).ID)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn ModifiedTime(&self) -> ::windows_core::Result<f64> {
        let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
        (::windows_core::Interface::vtable(self).ModifiedTime)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<f64>(result__)
    }
    pub unsafe fn Type(&self) -> ::windows_core::Result<GPMStarterGPOType> {
        let mut result__ = ::core::mem::MaybeUninit::<GPMStarterGPOType>::zeroed();
        (::windows_core::Interface::vtable(self).Type)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<GPMStarterGPOType>(result__)
    }
    pub unsafe fn ComputerVersion(&self) -> ::windows_core::Result<u16> {
        let mut result__ = ::core::mem::MaybeUninit::<u16>::zeroed();
        (::windows_core::Interface::vtable(self).ComputerVersion)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u16>(result__)
    }
    pub unsafe fn UserVersion(&self) -> ::windows_core::Result<u16> {
        let mut result__ = ::core::mem::MaybeUninit::<u16>::zeroed();
        (::windows_core::Interface::vtable(self).UserVersion)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u16>(result__)
    }
    pub unsafe fn StarterGPOVersion(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).StarterGPOVersion)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn Delete(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Delete)(::windows_core::Interface::as_raw(self)).ok()
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn Save<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrsavefile: Param0, boverwrite: i16, bsaveassystem: i16, bstrlanguage: *const super::Com::VARIANT, bstrauthor: *const super::Com::VARIANT, bstrproduct: *const super::Com::VARIANT, bstruniqueid: *const super::Com::VARIANT, bstrversion: *const super::Com::VARIANT, pvargpmprogress: *const super::Com::VARIANT, pvargpmcancel: *mut super::Com::VARIANT, ppigpmresult: *mut ::core::option::Option<IGPMResult>) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Save)(::windows_core::Interface::as_raw(self), bstrsavefile.into_param().abi(), ::core::mem::transmute(boverwrite), ::core::mem::transmute(bsaveassystem), ::core::mem::transmute(bstrlanguage), ::core::mem::transmute(bstrauthor), ::core::mem::transmute(bstrproduct), ::core::mem::transmute(bstruniqueid), ::core::mem::transmute(bstrversion), ::core::mem::transmute(pvargpmprogress), ::core::mem::transmute(pvargpmcancel), ::core::mem::transmute(ppigpmresult)).ok()
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn Backup<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrbackupdir: Param0, bstrcomment: Param1, pvargpmprogress: *const super::Com::VARIANT, pvargpmcancel: *mut super::Com::VARIANT, ppigpmresult: *mut ::core::option::Option<IGPMResult>) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Backup)(::windows_core::Interface::as_raw(self), bstrbackupdir.into_param().abi(), bstrcomment.into_param().abi(), ::core::mem::transmute(pvargpmprogress), ::core::mem::transmute(pvargpmcancel), ::core::mem::transmute(ppigpmresult)).ok()
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn CopyTo(&self, pvarnewdisplayname: *const super::Com::VARIANT, pvargpmprogress: *const super::Com::VARIANT, pvargpmcancel: *const super::Com::VARIANT) -> ::windows_core::Result<IGPMResult> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).CopyTo)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pvarnewdisplayname), ::core::mem::transmute(pvargpmprogress), ::core::mem::transmute(pvargpmcancel), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IGPMResult>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn GenerateReport(&self, gpmreporttype: GPMReportType, pvargpmprogress: *const super::Com::VARIANT, pvargpmcancel: *const super::Com::VARIANT) -> ::windows_core::Result<IGPMResult> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GenerateReport)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(gpmreporttype), ::core::mem::transmute(pvargpmprogress), ::core::mem::transmute(pvargpmcancel), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IGPMResult>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn GenerateReportToFile<'a, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, gpmreporttype: GPMReportType, bstrtargetfilepath: Param1) -> ::windows_core::Result<IGPMResult> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GenerateReportToFile)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(gpmreporttype), bstrtargetfilepath.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IGPMResult>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn GetSecurityInfo(&self) -> ::windows_core::Result<IGPMSecurityInfo> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetSecurityInfo)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IGPMSecurityInfo>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn SetSecurityInfo<'a, Param0: ::windows_core::IntoParam<'a, IGPMSecurityInfo>>(&self, psecurityinfo: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetSecurityInfo)(::windows_core::Interface::as_raw(self), psecurityinfo.into_param().abi()).ok()
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IGPMStarterGPO> for ::windows_core::IUnknown {
    fn from(value: IGPMStarterGPO) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IGPMStarterGPO> for ::windows_core::IUnknown {
    fn from(value: &IGPMStarterGPO) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IGPMStarterGPO {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IGPMStarterGPO {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IGPMStarterGPO> for super::Com::IDispatch {
    fn from(value: IGPMStarterGPO) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IGPMStarterGPO> for super::Com::IDispatch {
    fn from(value: &IGPMStarterGPO) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for IGPMStarterGPO {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a IGPMStarterGPO {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for IGPMStarterGPO {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for IGPMStarterGPO {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for IGPMStarterGPO {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for IGPMStarterGPO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IGPMStarterGPO").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for IGPMStarterGPO {
    type Vtable = IGPMStarterGPO_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xdfc3f61b_8880_4490_9337_d29c7ba8c2f0);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct IGPMStarterGPO_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub DisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SetDisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newval: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub Description: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newval: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub Author: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub Product: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub CreationTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut f64) -> ::windows_core::HRESULT,
    pub ID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub ModifiedTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut f64) -> ::windows_core::HRESULT,
    pub Type: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut GPMStarterGPOType) -> ::windows_core::HRESULT,
    pub ComputerVersion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut u16) -> ::windows_core::HRESULT,
    pub UserVersion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut u16) -> ::windows_core::HRESULT,
    pub StarterGPOVersion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub Delete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub Save: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrsavefile: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, boverwrite: i16, bsaveassystem: i16, bstrlanguage: *const super::Com::VARIANT, bstrauthor: *const super::Com::VARIANT, bstrproduct: *const super::Com::VARIANT, bstruniqueid: *const super::Com::VARIANT, bstrversion: *const super::Com::VARIANT, pvargpmprogress: *const super::Com::VARIANT, pvargpmcancel: *mut super::Com::VARIANT, ppigpmresult: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    Save: usize,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub Backup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrbackupdir: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, bstrcomment: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, pvargpmprogress: *const super::Com::VARIANT, pvargpmcancel: *mut super::Com::VARIANT, ppigpmresult: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    Backup: usize,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub CopyTo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvarnewdisplayname: *const super::Com::VARIANT, pvargpmprogress: *const super::Com::VARIANT, pvargpmcancel: *const super::Com::VARIANT, ppigpmresult: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    CopyTo: usize,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub GenerateReport: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, gpmreporttype: GPMReportType, pvargpmprogress: *const super::Com::VARIANT, pvargpmcancel: *const super::Com::VARIANT, ppigpmresult: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    GenerateReport: usize,
    #[cfg(feature = "win32-system")]
    pub GenerateReportToFile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, gpmreporttype: GPMReportType, bstrtargetfilepath: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, ppigpmresult: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    GenerateReportToFile: usize,
    #[cfg(feature = "win32-system")]
    pub GetSecurityInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppsecurityinfo: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    GetSecurityInfo: usize,
    #[cfg(feature = "win32-system")]
    pub SetSecurityInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psecurityinfo: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    SetSecurityInfo: usize,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct IGPMStarterGPOBackup(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl IGPMStarterGPOBackup {
    pub unsafe fn BackupDir(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).BackupDir)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn Comment(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).Comment)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn DisplayName(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).DisplayName)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn Domain(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).Domain)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn StarterGPOID(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).StarterGPOID)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn ID(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).ID)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn Timestamp(&self) -> ::windows_core::Result<f64> {
        let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
        (::windows_core::Interface::vtable(self).Timestamp)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<f64>(result__)
    }
    pub unsafe fn Type(&self) -> ::windows_core::Result<GPMStarterGPOType> {
        let mut result__ = ::core::mem::MaybeUninit::<GPMStarterGPOType>::zeroed();
        (::windows_core::Interface::vtable(self).Type)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<GPMStarterGPOType>(result__)
    }
    pub unsafe fn Delete(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Delete)(::windows_core::Interface::as_raw(self)).ok()
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn GenerateReport(&self, gpmreporttype: GPMReportType, pvargpmprogress: *const super::Com::VARIANT, pvargpmcancel: *mut super::Com::VARIANT, ppigpmresult: *mut ::core::option::Option<IGPMResult>) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GenerateReport)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(gpmreporttype), ::core::mem::transmute(pvargpmprogress), ::core::mem::transmute(pvargpmcancel), ::core::mem::transmute(ppigpmresult)).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn GenerateReportToFile<'a, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, gpmreporttype: GPMReportType, bstrtargetfilepath: Param1) -> ::windows_core::Result<IGPMResult> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GenerateReportToFile)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(gpmreporttype), bstrtargetfilepath.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IGPMResult>(result__)
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IGPMStarterGPOBackup> for ::windows_core::IUnknown {
    fn from(value: IGPMStarterGPOBackup) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IGPMStarterGPOBackup> for ::windows_core::IUnknown {
    fn from(value: &IGPMStarterGPOBackup) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IGPMStarterGPOBackup {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IGPMStarterGPOBackup {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IGPMStarterGPOBackup> for super::Com::IDispatch {
    fn from(value: IGPMStarterGPOBackup) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IGPMStarterGPOBackup> for super::Com::IDispatch {
    fn from(value: &IGPMStarterGPOBackup) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for IGPMStarterGPOBackup {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a IGPMStarterGPOBackup {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for IGPMStarterGPOBackup {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for IGPMStarterGPOBackup {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for IGPMStarterGPOBackup {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for IGPMStarterGPOBackup {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IGPMStarterGPOBackup").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for IGPMStarterGPOBackup {
    type Vtable = IGPMStarterGPOBackup_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x51d98eda_a87e_43dd_b80a_0b66ef1938d6);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct IGPMStarterGPOBackup_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub BackupDir: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrbackupdir: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub Comment: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrcomment: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub DisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrdisplayname: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub Domain: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrtemplatedomain: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub StarterGPOID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrtemplateid: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub ID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrid: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub Timestamp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptimestamp: *mut f64) -> ::windows_core::HRESULT,
    pub Type: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptype: *mut GPMStarterGPOType) -> ::windows_core::HRESULT,
    pub Delete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub GenerateReport: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, gpmreporttype: GPMReportType, pvargpmprogress: *const super::Com::VARIANT, pvargpmcancel: *mut super::Com::VARIANT, ppigpmresult: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    GenerateReport: usize,
    #[cfg(feature = "win32-system")]
    pub GenerateReportToFile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, gpmreporttype: GPMReportType, bstrtargetfilepath: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, ppigpmresult: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    GenerateReportToFile: usize,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct IGPMStarterGPOBackupCollection(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl IGPMStarterGPOBackupCollection {
    pub unsafe fn Count(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).Count)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn get_Item(&self, lindex: i32) -> ::windows_core::Result<super::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).get_Item)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lindex), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::VARIANT>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn _NewEnum(&self) -> ::windows_core::Result<super::Ole::IEnumVARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self)._NewEnum)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Ole::IEnumVARIANT>(result__)
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IGPMStarterGPOBackupCollection> for ::windows_core::IUnknown {
    fn from(value: IGPMStarterGPOBackupCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IGPMStarterGPOBackupCollection> for ::windows_core::IUnknown {
    fn from(value: &IGPMStarterGPOBackupCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IGPMStarterGPOBackupCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IGPMStarterGPOBackupCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IGPMStarterGPOBackupCollection> for super::Com::IDispatch {
    fn from(value: IGPMStarterGPOBackupCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IGPMStarterGPOBackupCollection> for super::Com::IDispatch {
    fn from(value: &IGPMStarterGPOBackupCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for IGPMStarterGPOBackupCollection {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a IGPMStarterGPOBackupCollection {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for IGPMStarterGPOBackupCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for IGPMStarterGPOBackupCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for IGPMStarterGPOBackupCollection {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for IGPMStarterGPOBackupCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IGPMStarterGPOBackupCollection").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for IGPMStarterGPOBackupCollection {
    type Vtable = IGPMStarterGPOBackupCollection_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc998031d_add0_4bb5_8dea_298505d8423b);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct IGPMStarterGPOBackupCollection_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub get_Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lindex: i32, pval: *mut super::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    get_Item: usize,
    #[cfg(feature = "win32-system")]
    pub _NewEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppigpmtmplbackup: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    _NewEnum: usize,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct IGPMStarterGPOCollection(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl IGPMStarterGPOCollection {
    pub unsafe fn Count(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).Count)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn get_Item(&self, lindex: i32) -> ::windows_core::Result<super::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).get_Item)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lindex), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::VARIANT>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn _NewEnum(&self) -> ::windows_core::Result<super::Ole::IEnumVARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self)._NewEnum)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Ole::IEnumVARIANT>(result__)
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IGPMStarterGPOCollection> for ::windows_core::IUnknown {
    fn from(value: IGPMStarterGPOCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IGPMStarterGPOCollection> for ::windows_core::IUnknown {
    fn from(value: &IGPMStarterGPOCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IGPMStarterGPOCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IGPMStarterGPOCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IGPMStarterGPOCollection> for super::Com::IDispatch {
    fn from(value: IGPMStarterGPOCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IGPMStarterGPOCollection> for super::Com::IDispatch {
    fn from(value: &IGPMStarterGPOCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for IGPMStarterGPOCollection {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a IGPMStarterGPOCollection {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for IGPMStarterGPOCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for IGPMStarterGPOCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for IGPMStarterGPOCollection {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for IGPMStarterGPOCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IGPMStarterGPOCollection").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for IGPMStarterGPOCollection {
    type Vtable = IGPMStarterGPOCollection_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2e522729_2219_44ad_933a_64dfd650c423);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct IGPMStarterGPOCollection_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub get_Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lindex: i32, pval: *mut super::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    get_Item: usize,
    #[cfg(feature = "win32-system")]
    pub _NewEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppigpmtemplates: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    _NewEnum: usize,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct IGPMStatusMessage(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl IGPMStatusMessage {
    pub unsafe fn ObjectPath(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).ObjectPath)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn ErrorCode(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ErrorCode)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn ExtensionName(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).ExtensionName)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SettingsName(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).SettingsName)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn OperationCode(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).OperationCode)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Message(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).Message)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IGPMStatusMessage> for ::windows_core::IUnknown {
    fn from(value: IGPMStatusMessage) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IGPMStatusMessage> for ::windows_core::IUnknown {
    fn from(value: &IGPMStatusMessage) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IGPMStatusMessage {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IGPMStatusMessage {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IGPMStatusMessage> for super::Com::IDispatch {
    fn from(value: IGPMStatusMessage) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IGPMStatusMessage> for super::Com::IDispatch {
    fn from(value: &IGPMStatusMessage) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for IGPMStatusMessage {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a IGPMStatusMessage {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for IGPMStatusMessage {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for IGPMStatusMessage {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for IGPMStatusMessage {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for IGPMStatusMessage {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IGPMStatusMessage").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for IGPMStatusMessage {
    type Vtable = IGPMStatusMessage_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8496c22f_f3de_4a1f_8f58_603caaa93d7b);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct IGPMStatusMessage_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub ObjectPath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub ErrorCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub ExtensionName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SettingsName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub OperationCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Message: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct IGPMStatusMsgCollection(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl IGPMStatusMsgCollection {
    pub unsafe fn Count(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).Count)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn get_Item(&self, lindex: i32) -> ::windows_core::Result<super::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).get_Item)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lindex), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::VARIANT>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn _NewEnum(&self) -> ::windows_core::Result<super::Ole::IEnumVARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self)._NewEnum)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Ole::IEnumVARIANT>(result__)
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IGPMStatusMsgCollection> for ::windows_core::IUnknown {
    fn from(value: IGPMStatusMsgCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IGPMStatusMsgCollection> for ::windows_core::IUnknown {
    fn from(value: &IGPMStatusMsgCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IGPMStatusMsgCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IGPMStatusMsgCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IGPMStatusMsgCollection> for super::Com::IDispatch {
    fn from(value: IGPMStatusMsgCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IGPMStatusMsgCollection> for super::Com::IDispatch {
    fn from(value: &IGPMStatusMsgCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for IGPMStatusMsgCollection {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a IGPMStatusMsgCollection {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for IGPMStatusMsgCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for IGPMStatusMsgCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for IGPMStatusMsgCollection {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for IGPMStatusMsgCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IGPMStatusMsgCollection").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for IGPMStatusMsgCollection {
    type Vtable = IGPMStatusMsgCollection_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9b6e1af0_1a92_40f3_a59d_f36ac1f728b7);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct IGPMStatusMsgCollection_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub get_Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lindex: i32, pval: *mut super::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    get_Item: usize,
    #[cfg(feature = "win32-system")]
    pub _NewEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    _NewEnum: usize,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct IGPMTrustee(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl IGPMTrustee {
    pub unsafe fn TrusteeSid(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).TrusteeSid)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn TrusteeName(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).TrusteeName)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn TrusteeDomain(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).TrusteeDomain)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn TrusteeDSPath(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).TrusteeDSPath)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn TrusteeType(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).TrusteeType)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IGPMTrustee> for ::windows_core::IUnknown {
    fn from(value: IGPMTrustee) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IGPMTrustee> for ::windows_core::IUnknown {
    fn from(value: &IGPMTrustee) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IGPMTrustee {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IGPMTrustee {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IGPMTrustee> for super::Com::IDispatch {
    fn from(value: IGPMTrustee) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IGPMTrustee> for super::Com::IDispatch {
    fn from(value: &IGPMTrustee) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for IGPMTrustee {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a IGPMTrustee {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for IGPMTrustee {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for IGPMTrustee {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for IGPMTrustee {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for IGPMTrustee {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IGPMTrustee").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for IGPMTrustee {
    type Vtable = IGPMTrustee_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3b466da8_c1a4_4b2a_999a_befcdd56cefb);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct IGPMTrustee_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub TrusteeSid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrval: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub TrusteeName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrval: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub TrusteeDomain: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrval: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub TrusteeDSPath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub TrusteeType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lval: *mut i32) -> ::windows_core::HRESULT,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct IGPMWMIFilter(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl IGPMWMIFilter {
    pub unsafe fn Path(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).Path)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetName<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, newval: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetName)(::windows_core::Interface::as_raw(self), newval.into_param().abi()).ok()
    }
    pub unsafe fn Name(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).Name)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetDescription<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, newval: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetDescription)(::windows_core::Interface::as_raw(self), newval.into_param().abi()).ok()
    }
    pub unsafe fn Description(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).Description)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn GetQueryList(&self) -> ::windows_core::Result<super::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).GetQueryList)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::VARIANT>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn GetSecurityInfo(&self) -> ::windows_core::Result<IGPMSecurityInfo> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetSecurityInfo)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IGPMSecurityInfo>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn SetSecurityInfo<'a, Param0: ::windows_core::IntoParam<'a, IGPMSecurityInfo>>(&self, psecurityinfo: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetSecurityInfo)(::windows_core::Interface::as_raw(self), psecurityinfo.into_param().abi()).ok()
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IGPMWMIFilter> for ::windows_core::IUnknown {
    fn from(value: IGPMWMIFilter) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IGPMWMIFilter> for ::windows_core::IUnknown {
    fn from(value: &IGPMWMIFilter) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IGPMWMIFilter {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IGPMWMIFilter {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IGPMWMIFilter> for super::Com::IDispatch {
    fn from(value: IGPMWMIFilter) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IGPMWMIFilter> for super::Com::IDispatch {
    fn from(value: &IGPMWMIFilter) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for IGPMWMIFilter {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a IGPMWMIFilter {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for IGPMWMIFilter {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for IGPMWMIFilter {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for IGPMWMIFilter {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for IGPMWMIFilter {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IGPMWMIFilter").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for IGPMWMIFilter {
    type Vtable = IGPMWMIFilter_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xef2ff9b4_3c27_459a_b979_038305cec75d);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct IGPMWMIFilter_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Path: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newval: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newval: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub Description: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub GetQueryList: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pqrylist: *mut super::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    GetQueryList: usize,
    #[cfg(feature = "win32-system")]
    pub GetSecurityInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppsecurityinfo: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    GetSecurityInfo: usize,
    #[cfg(feature = "win32-system")]
    pub SetSecurityInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psecurityinfo: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    SetSecurityInfo: usize,
}
#[cfg(feature = "win32-system")]
#[repr(transparent)]
pub struct IGPMWMIFilterCollection(::windows_core::IUnknown);
#[cfg(feature = "win32-system")]
impl IGPMWMIFilterCollection {
    pub unsafe fn Count(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).Count)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub unsafe fn get_Item(&self, lindex: i32) -> ::windows_core::Result<super::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).get_Item)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lindex), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::VARIANT>(result__)
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn _NewEnum(&self) -> ::windows_core::Result<super::Ole::IEnumVARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self)._NewEnum)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Ole::IEnumVARIANT>(result__)
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IGPMWMIFilterCollection> for ::windows_core::IUnknown {
    fn from(value: IGPMWMIFilterCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IGPMWMIFilterCollection> for ::windows_core::IUnknown {
    fn from(value: &IGPMWMIFilterCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IGPMWMIFilterCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IGPMWMIFilterCollection {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<IGPMWMIFilterCollection> for super::Com::IDispatch {
    fn from(value: IGPMWMIFilterCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "win32-system")]
impl ::core::convert::From<&IGPMWMIFilterCollection> for super::Com::IDispatch {
    fn from(value: &IGPMWMIFilterCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for IGPMWMIFilterCollection {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl<'a> ::windows_core::IntoParam<'a, super::Com::IDispatch> for &'a IGPMWMIFilterCollection {
    fn into_param(self) -> ::windows_core::Param<'a, super::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "win32-system")]
impl ::core::clone::Clone for IGPMWMIFilterCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::PartialEq for IGPMWMIFilterCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "win32-system")]
impl ::core::cmp::Eq for IGPMWMIFilterCollection {}
#[cfg(feature = "win32-system")]
impl ::core::fmt::Debug for IGPMWMIFilterCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IGPMWMIFilterCollection").field(&self.0).finish()
    }
}
#[cfg(feature = "win32-system")]
unsafe impl ::windows_core::Interface for IGPMWMIFilterCollection {
    type Vtable = IGPMWMIFilterCollection_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5782d582_1a36_4661_8a94_c3c32551945b);
}
#[cfg(feature = "win32-system")]
#[repr(C)]
#[doc(hidden)]
pub struct IGPMWMIFilterCollection_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "win32-system", feature = "win32-system"))]
    pub get_Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lindex: i32, pval: *mut super::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "win32-system", feature = "win32-system")))]
    get_Item: usize,
    #[cfg(feature = "win32-system")]
    pub _NewEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    _NewEnum: usize,
}
#[repr(transparent)]
pub struct IGroupPolicyObject(::windows_core::IUnknown);
impl IGroupPolicyObject {
    pub unsafe fn New<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, pszdomainname: Param0, pszdisplayname: Param1, dwflags: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).New)(::windows_core::Interface::as_raw(self), pszdomainname.into_param().abi(), pszdisplayname.into_param().abi(), ::core::mem::transmute(dwflags)).ok()
    }
    pub unsafe fn OpenDSGPO<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, pszpath: Param0, dwflags: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).OpenDSGPO)(::windows_core::Interface::as_raw(self), pszpath.into_param().abi(), ::core::mem::transmute(dwflags)).ok()
    }
    pub unsafe fn OpenLocalMachineGPO(&self, dwflags: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).OpenLocalMachineGPO)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwflags)).ok()
    }
    pub unsafe fn OpenRemoteMachineGPO<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, pszcomputername: Param0, dwflags: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).OpenRemoteMachineGPO)(::windows_core::Interface::as_raw(self), pszcomputername.into_param().abi(), ::core::mem::transmute(dwflags)).ok()
    }
    pub unsafe fn Save<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(&self, bmachine: Param0, badd: Param1, pguidextension: *mut ::windows_core::GUID, pguid: *mut ::windows_core::GUID) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Save)(::windows_core::Interface::as_raw(self), bmachine.into_param().abi(), badd.into_param().abi(), ::core::mem::transmute(pguidextension), ::core::mem::transmute(pguid)).ok()
    }
    pub unsafe fn Delete(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Delete)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn GetName(&self, pszname: &mut [u16]) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetName)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(::windows_core::as_mut_ptr_or_null(pszname)), pszname.len() as _).ok()
    }
    pub unsafe fn GetDisplayName(&self, pszname: &mut [u16]) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetDisplayName)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(::windows_core::as_mut_ptr_or_null(pszname)), pszname.len() as _).ok()
    }
    pub unsafe fn SetDisplayName<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, pszname: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetDisplayName)(::windows_core::Interface::as_raw(self), pszname.into_param().abi()).ok()
    }
    pub unsafe fn GetPath(&self, pszpath: &mut [u16]) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetPath)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(::windows_core::as_mut_ptr_or_null(pszpath)), pszpath.len() as _).ok()
    }
    pub unsafe fn GetDSPath(&self, dwsection: u32, pszpath: &mut [u16]) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetDSPath)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwsection), ::core::mem::transmute(::windows_core::as_mut_ptr_or_null(pszpath)), pszpath.len() as _).ok()
    }
    pub unsafe fn GetFileSysPath(&self, dwsection: u32, pszpath: &mut [u16]) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetFileSysPath)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwsection), ::core::mem::transmute(::windows_core::as_mut_ptr_or_null(pszpath)), pszpath.len() as _).ok()
    }
    #[cfg(feature = "win32-system")]
    pub unsafe fn GetRegistryKey(&self, dwsection: u32, hkey: *mut super::Registry::HKEY) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetRegistryKey)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwsection), ::core::mem::transmute(hkey)).ok()
    }
    pub unsafe fn GetOptions(&self, dwoptions: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetOptions)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwoptions)).ok()
    }
    pub unsafe fn SetOptions(&self, dwoptions: u32, dwmask: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetOptions)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwoptions), ::core::mem::transmute(dwmask)).ok()
    }
    pub unsafe fn GetType(&self, gpotype: *mut GROUP_POLICY_OBJECT_TYPE) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetType)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(gpotype)).ok()
    }
    pub unsafe fn GetMachineName(&self, pszname: &mut [u16]) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetMachineName)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(::windows_core::as_mut_ptr_or_null(pszname)), pszname.len() as _).ok()
    }
    #[cfg(feature = "win32-ui")]
    pub unsafe fn GetPropertySheetPages(&self, hpages: *mut *mut ::win32_ui::Controls::HPROPSHEETPAGE, upagecount: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetPropertySheetPages)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(hpages), ::core::mem::transmute(upagecount)).ok()
    }
}
impl ::core::convert::From<IGroupPolicyObject> for ::windows_core::IUnknown {
    fn from(value: IGroupPolicyObject) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IGroupPolicyObject> for ::windows_core::IUnknown {
    fn from(value: &IGroupPolicyObject) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IGroupPolicyObject {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IGroupPolicyObject {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IGroupPolicyObject {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IGroupPolicyObject {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IGroupPolicyObject {}
impl ::core::fmt::Debug for IGroupPolicyObject {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IGroupPolicyObject").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IGroupPolicyObject {
    type Vtable = IGroupPolicyObject_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xea502723_a23d_11d1_a7d3_0000f87571e3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGroupPolicyObject_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub New: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszdomainname: ::windows_core::PCWSTR, pszdisplayname: ::windows_core::PCWSTR, dwflags: u32) -> ::windows_core::HRESULT,
    pub OpenDSGPO: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszpath: ::windows_core::PCWSTR, dwflags: u32) -> ::windows_core::HRESULT,
    pub OpenLocalMachineGPO: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwflags: u32) -> ::windows_core::HRESULT,
    pub OpenRemoteMachineGPO: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszcomputername: ::windows_core::PCWSTR, dwflags: u32) -> ::windows_core::HRESULT,
    pub Save: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bmachine: ::win32_foundation::BOOL, badd: ::win32_foundation::BOOL, pguidextension: *mut ::windows_core::GUID, pguid: *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub Delete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszname: ::windows_core::PWSTR, cchmaxlength: i32) -> ::windows_core::HRESULT,
    pub GetDisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszname: ::windows_core::PWSTR, cchmaxlength: i32) -> ::windows_core::HRESULT,
    pub SetDisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszname: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    pub GetPath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszpath: ::windows_core::PWSTR, cchmaxlength: i32) -> ::windows_core::HRESULT,
    pub GetDSPath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwsection: u32, pszpath: ::windows_core::PWSTR, cchmaxpath: i32) -> ::windows_core::HRESULT,
    pub GetFileSysPath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwsection: u32, pszpath: ::windows_core::PWSTR, cchmaxpath: i32) -> ::windows_core::HRESULT,
    #[cfg(feature = "win32-system")]
    pub GetRegistryKey: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwsection: u32, hkey: *mut super::Registry::HKEY) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-system"))]
    GetRegistryKey: usize,
    pub GetOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwoptions: *mut u32) -> ::windows_core::HRESULT,
    pub SetOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwoptions: u32, dwmask: u32) -> ::windows_core::HRESULT,
    pub GetType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, gpotype: *mut GROUP_POLICY_OBJECT_TYPE) -> ::windows_core::HRESULT,
    pub GetMachineName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszname: ::windows_core::PWSTR, cchmaxlength: i32) -> ::windows_core::HRESULT,
    #[cfg(feature = "win32-ui")]
    pub GetPropertySheetPages: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hpages: *mut *mut ::win32_ui::Controls::HPROPSHEETPAGE, upagecount: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "win32-ui"))]
    GetPropertySheetPages: usize,
}
#[repr(C)]
pub struct INSTALLDATA {
    pub Type: INSTALLSPECTYPE,
    pub Spec: INSTALLSPEC,
}
impl ::core::marker::Copy for INSTALLDATA {}
impl ::core::clone::Clone for INSTALLDATA {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows_core::Abi for INSTALLDATA {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for INSTALLDATA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<INSTALLDATA>()) == 0 }
    }
}
impl ::core::cmp::Eq for INSTALLDATA {}
impl ::core::default::Default for INSTALLDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub union INSTALLSPEC {
    pub AppName: INSTALLSPEC_0,
    pub FileExt: ::windows_core::PWSTR,
    pub ProgId: ::windows_core::PWSTR,
    pub COMClass: INSTALLSPEC_1,
}
impl ::core::marker::Copy for INSTALLSPEC {}
impl ::core::clone::Clone for INSTALLSPEC {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows_core::Abi for INSTALLSPEC {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for INSTALLSPEC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<INSTALLSPEC>()) == 0 }
    }
}
impl ::core::cmp::Eq for INSTALLSPEC {}
impl ::core::default::Default for INSTALLSPEC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct INSTALLSPEC_0 {
    pub Name: ::windows_core::PWSTR,
    pub GPOId: ::windows_core::GUID,
}
impl ::core::marker::Copy for INSTALLSPEC_0 {}
impl ::core::clone::Clone for INSTALLSPEC_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for INSTALLSPEC_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("INSTALLSPEC_0").field("Name", &self.Name).field("GPOId", &self.GPOId).finish()
    }
}
unsafe impl ::windows_core::Abi for INSTALLSPEC_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for INSTALLSPEC_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<INSTALLSPEC_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for INSTALLSPEC_0 {}
impl ::core::default::Default for INSTALLSPEC_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct INSTALLSPEC_1 {
    pub Clsid: ::windows_core::GUID,
    pub ClsCtx: u32,
}
impl ::core::marker::Copy for INSTALLSPEC_1 {}
impl ::core::clone::Clone for INSTALLSPEC_1 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for INSTALLSPEC_1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("INSTALLSPEC_1").field("Clsid", &self.Clsid).field("ClsCtx", &self.ClsCtx).finish()
    }
}
unsafe impl ::windows_core::Abi for INSTALLSPEC_1 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for INSTALLSPEC_1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<INSTALLSPEC_1>()) == 0 }
    }
}
impl ::core::cmp::Eq for INSTALLSPEC_1 {}
impl ::core::default::Default for INSTALLSPEC_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct INSTALLSPECTYPE(pub i32);
pub const APPNAME: INSTALLSPECTYPE = INSTALLSPECTYPE(1i32);
pub const FILEEXT: INSTALLSPECTYPE = INSTALLSPECTYPE(2i32);
pub const PROGID: INSTALLSPECTYPE = INSTALLSPECTYPE(3i32);
pub const COMCLASS: INSTALLSPECTYPE = INSTALLSPECTYPE(4i32);
impl ::core::marker::Copy for INSTALLSPECTYPE {}
impl ::core::clone::Clone for INSTALLSPECTYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for INSTALLSPECTYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for INSTALLSPECTYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for INSTALLSPECTYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INSTALLSPECTYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
pub struct IRSOPInformation(::windows_core::IUnknown);
impl IRSOPInformation {
    pub unsafe fn GetNamespace(&self, dwsection: u32, pszname: &mut [u16]) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetNamespace)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(dwsection), ::core::mem::transmute(::windows_core::as_mut_ptr_or_null(pszname)), pszname.len() as _).ok()
    }
    pub unsafe fn GetFlags(&self, pdwflags: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetFlags)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pdwflags)).ok()
    }
    pub unsafe fn GetEventLogEntryText<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param2: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, pszeventsource: Param0, pszeventlogname: Param1, pszeventtime: Param2, dweventid: u32) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::PWSTR>::zeroed();
        (::windows_core::Interface::vtable(self).GetEventLogEntryText)(::windows_core::Interface::as_raw(self), pszeventsource.into_param().abi(), pszeventlogname.into_param().abi(), pszeventtime.into_param().abi(), ::core::mem::transmute(dweventid), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::PWSTR>(result__)
    }
}
impl ::core::convert::From<IRSOPInformation> for ::windows_core::IUnknown {
    fn from(value: IRSOPInformation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IRSOPInformation> for ::windows_core::IUnknown {
    fn from(value: &IRSOPInformation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IRSOPInformation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IRSOPInformation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IRSOPInformation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IRSOPInformation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRSOPInformation {}
impl ::core::fmt::Debug for IRSOPInformation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRSOPInformation").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IRSOPInformation {
    type Vtable = IRSOPInformation_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9a5a81b5_d9c7_49ef_9d11_ddf50968c48d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRSOPInformation_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub GetNamespace: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwsection: u32, pszname: ::windows_core::PWSTR, cchmaxlength: i32) -> ::windows_core::HRESULT,
    pub GetFlags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwflags: *mut u32) -> ::windows_core::HRESULT,
    pub GetEventLogEntryText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszeventsource: ::windows_core::PCWSTR, pszeventlogname: ::windows_core::PCWSTR, pszeventtime: ::windows_core::PCWSTR, dweventid: u32, ppsztext: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT,
}
#[inline]
pub unsafe fn ImportRSoPData<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(lpnamespace: Param0, lpfilename: Param1) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ImportRSoPData(lpnamespace: ::windows_core::PCWSTR, lpfilename: ::windows_core::PCWSTR) -> ::windows_core::HRESULT;
        }
        ImportRSoPData(lpnamespace.into_param().abi(), lpfilename.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn InstallApplication(pinstallinfo: *const INSTALLDATA) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InstallApplication(pinstallinfo: *const INSTALLDATA) -> u32;
        }
        ::core::mem::transmute(InstallApplication(::core::mem::transmute(pinstallinfo)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[repr(C)]
pub struct LOCALMANAGEDAPPLICATION {
    pub pszDeploymentName: ::windows_core::PWSTR,
    pub pszPolicyName: ::windows_core::PWSTR,
    pub pszProductId: ::windows_core::PWSTR,
    pub dwState: u32,
}
impl ::core::marker::Copy for LOCALMANAGEDAPPLICATION {}
impl ::core::clone::Clone for LOCALMANAGEDAPPLICATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for LOCALMANAGEDAPPLICATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("LOCALMANAGEDAPPLICATION").field("pszDeploymentName", &self.pszDeploymentName).field("pszPolicyName", &self.pszPolicyName).field("pszProductId", &self.pszProductId).field("dwState", &self.dwState).finish()
    }
}
unsafe impl ::windows_core::Abi for LOCALMANAGEDAPPLICATION {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for LOCALMANAGEDAPPLICATION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<LOCALMANAGEDAPPLICATION>()) == 0 }
    }
}
impl ::core::cmp::Eq for LOCALMANAGEDAPPLICATION {}
impl ::core::default::Default for LOCALMANAGEDAPPLICATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const LOCALSTATE_ASSIGNED: u32 = 1u32;
pub const LOCALSTATE_ORPHANED: u32 = 32u32;
pub const LOCALSTATE_POLICYREMOVE_ORPHAN: u32 = 8u32;
pub const LOCALSTATE_POLICYREMOVE_UNINSTALL: u32 = 16u32;
pub const LOCALSTATE_PUBLISHED: u32 = 2u32;
pub const LOCALSTATE_UNINSTALLED: u32 = 64u32;
pub const LOCALSTATE_UNINSTALL_UNMANAGED: u32 = 4u32;
#[inline]
pub unsafe fn LeaveCriticalPolicySection<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HANDLE>>(hsection: Param0) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn LeaveCriticalPolicySection(hsection: ::win32_foundation::HANDLE) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(LeaveCriticalPolicySection(hsection.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[repr(C)]
pub struct MANAGEDAPPLICATION {
    pub pszPackageName: ::windows_core::PWSTR,
    pub pszPublisher: ::windows_core::PWSTR,
    pub dwVersionHi: u32,
    pub dwVersionLo: u32,
    pub dwRevision: u32,
    pub GpoId: ::windows_core::GUID,
    pub pszPolicyName: ::windows_core::PWSTR,
    pub ProductId: ::windows_core::GUID,
    pub Language: u16,
    pub pszOwner: ::windows_core::PWSTR,
    pub pszCompany: ::windows_core::PWSTR,
    pub pszComments: ::windows_core::PWSTR,
    pub pszContact: ::windows_core::PWSTR,
    pub pszSupportUrl: ::windows_core::PWSTR,
    pub dwPathType: u32,
    pub bInstalled: ::win32_foundation::BOOL,
}
impl ::core::marker::Copy for MANAGEDAPPLICATION {}
impl ::core::clone::Clone for MANAGEDAPPLICATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MANAGEDAPPLICATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MANAGEDAPPLICATION")
            .field("pszPackageName", &self.pszPackageName)
            .field("pszPublisher", &self.pszPublisher)
            .field("dwVersionHi", &self.dwVersionHi)
            .field("dwVersionLo", &self.dwVersionLo)
            .field("dwRevision", &self.dwRevision)
            .field("GpoId", &self.GpoId)
            .field("pszPolicyName", &self.pszPolicyName)
            .field("ProductId", &self.ProductId)
            .field("Language", &self.Language)
            .field("pszOwner", &self.pszOwner)
            .field("pszCompany", &self.pszCompany)
            .field("pszComments", &self.pszComments)
            .field("pszContact", &self.pszContact)
            .field("pszSupportUrl", &self.pszSupportUrl)
            .field("dwPathType", &self.dwPathType)
            .field("bInstalled", &self.bInstalled)
            .finish()
    }
}
unsafe impl ::windows_core::Abi for MANAGEDAPPLICATION {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MANAGEDAPPLICATION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MANAGEDAPPLICATION>()) == 0 }
    }
}
impl ::core::cmp::Eq for MANAGEDAPPLICATION {}
impl ::core::default::Default for MANAGEDAPPLICATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const MANAGED_APPS_FROMCATEGORY: u32 = 2u32;
pub const MANAGED_APPS_INFOLEVEL_DEFAULT: u32 = 65536u32;
pub const MANAGED_APPS_USERAPPLICATIONS: u32 = 1u32;
pub const MANAGED_APPTYPE_SETUPEXE: u32 = 2u32;
pub const MANAGED_APPTYPE_UNSUPPORTED: u32 = 3u32;
pub const MANAGED_APPTYPE_WINDOWSINSTALLER: u32 = 1u32;
pub const NODEID_Machine: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8fc0b737_a0e1_11d1_a7d3_0000f87571e3);
pub const NODEID_MachineSWSettings: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8fc0b73a_a0e1_11d1_a7d3_0000f87571e3);
pub const NODEID_RSOPMachine: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xbd4c1a2e_0b7a_4a62_a6b0_c0577539c97e);
pub const NODEID_RSOPMachineSWSettings: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6a76273e_eb8e_45db_94c5_25663a5f2c1a);
pub const NODEID_RSOPUser: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xab87364f_0cec_4cd8_9bf8_898f34628fb8);
pub const NODEID_RSOPUserSWSettings: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe52c5ce3_fd27_4402_84de_d9a5f2858910);
pub const NODEID_User: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8fc0b738_a0e1_11d1_a7d3_0000f87571e3);
pub const NODEID_UserSWSettings: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8fc0b73c_a0e1_11d1_a7d3_0000f87571e3);
#[cfg(all(feature = "win32-system", feature = "win32-system"))]
pub type PFNGENERATEGROUPPOLICY = ::core::option::Option<unsafe extern "system" fn(dwflags: u32, pbabort: *mut ::win32_foundation::BOOL, pwszsite: ::windows_core::PCWSTR, pcomputertarget: *const RSOP_TARGET, pusertarget: *const RSOP_TARGET) -> u32>;
#[cfg(feature = "win32-system")]
pub type PFNPROCESSGROUPPOLICY = ::core::option::Option<unsafe extern "system" fn(dwflags: u32, htoken: ::win32_foundation::HANDLE, hkeyroot: super::Registry::HKEY, pdeletedgpolist: *const GROUP_POLICY_OBJECTA, pchangedgpolist: *const GROUP_POLICY_OBJECTA, phandle: usize, pbabort: *mut ::win32_foundation::BOOL, pstatuscallback: PFNSTATUSMESSAGECALLBACK) -> u32>;
#[cfg(all(feature = "win32-system", feature = "win32-system"))]
pub type PFNPROCESSGROUPPOLICYEX = ::core::option::Option<unsafe extern "system" fn(dwflags: u32, htoken: ::win32_foundation::HANDLE, hkeyroot: super::Registry::HKEY, pdeletedgpolist: *const GROUP_POLICY_OBJECTA, pchangedgpolist: *const GROUP_POLICY_OBJECTA, phandle: usize, pbabort: *mut ::win32_foundation::BOOL, pstatuscallback: PFNSTATUSMESSAGECALLBACK, pwbemservices: ::core::option::Option<super::Wmi::IWbemServices>, prsopstatus: *mut ::windows_core::HRESULT) -> u32>;
pub type PFNSTATUSMESSAGECALLBACK = ::core::option::Option<unsafe extern "system" fn(bverbose: ::win32_foundation::BOOL, lpmessage: ::windows_core::PCWSTR) -> u32>;
pub const PI_APPLYPOLICY: u32 = 2u32;
pub const PI_NOUI: u32 = 1u32;
#[repr(C)]
pub struct POLICYSETTINGSTATUSINFO {
    pub szKey: ::windows_core::PWSTR,
    pub szEventSource: ::windows_core::PWSTR,
    pub szEventLogName: ::windows_core::PWSTR,
    pub dwEventID: u32,
    pub dwErrorCode: u32,
    pub status: SETTINGSTATUS,
    pub timeLogged: ::win32_foundation::SYSTEMTIME,
}
impl ::core::marker::Copy for POLICYSETTINGSTATUSINFO {}
impl ::core::clone::Clone for POLICYSETTINGSTATUSINFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for POLICYSETTINGSTATUSINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("POLICYSETTINGSTATUSINFO").field("szKey", &self.szKey).field("szEventSource", &self.szEventSource).field("szEventLogName", &self.szEventLogName).field("dwEventID", &self.dwEventID).field("dwErrorCode", &self.dwErrorCode).field("status", &self.status).field("timeLogged", &self.timeLogged).finish()
    }
}
unsafe impl ::windows_core::Abi for POLICYSETTINGSTATUSINFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for POLICYSETTINGSTATUSINFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<POLICYSETTINGSTATUSINFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for POLICYSETTINGSTATUSINFO {}
impl ::core::default::Default for POLICYSETTINGSTATUSINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const PT_MANDATORY: u32 = 4u32;
pub const PT_ROAMING: u32 = 2u32;
pub const PT_ROAMING_PREEXISTING: u32 = 8u32;
pub const PT_TEMPORARY: u32 = 1u32;
#[inline]
pub unsafe fn ProcessGroupPolicyCompleted(extensionid: *const ::windows_core::GUID, pasynchandle: usize, dwstatus: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ProcessGroupPolicyCompleted(extensionid: *const ::windows_core::GUID, pasynchandle: usize, dwstatus: u32) -> u32;
        }
        ::core::mem::transmute(ProcessGroupPolicyCompleted(::core::mem::transmute(extensionid), ::core::mem::transmute(pasynchandle), ::core::mem::transmute(dwstatus)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn ProcessGroupPolicyCompletedEx(extensionid: *const ::windows_core::GUID, pasynchandle: usize, dwstatus: u32, rsopstatus: ::windows_core::HRESULT) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ProcessGroupPolicyCompletedEx(extensionid: *const ::windows_core::GUID, pasynchandle: usize, dwstatus: u32, rsopstatus: ::windows_core::HRESULT) -> u32;
        }
        ::core::mem::transmute(ProcessGroupPolicyCompletedEx(::core::mem::transmute(extensionid), ::core::mem::transmute(pasynchandle), ::core::mem::transmute(dwstatus), ::core::mem::transmute(rsopstatus)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub const RP_FORCE: u32 = 1u32;
pub const RP_SYNC: u32 = 2u32;
pub const RSOP_COMPUTER_ACCESS_DENIED: u32 = 2u32;
pub const RSOP_INFO_FLAG_DIAGNOSTIC_MODE: u32 = 1u32;
pub const RSOP_NO_COMPUTER: u32 = 65536u32;
pub const RSOP_NO_USER: u32 = 131072u32;
pub const RSOP_PLANNING_ASSUME_COMP_WQLFILTER_TRUE: u32 = 16u32;
pub const RSOP_PLANNING_ASSUME_LOOPBACK_MERGE: u32 = 2u32;
pub const RSOP_PLANNING_ASSUME_LOOPBACK_REPLACE: u32 = 4u32;
pub const RSOP_PLANNING_ASSUME_SLOW_LINK: u32 = 1u32;
pub const RSOP_PLANNING_ASSUME_USER_WQLFILTER_TRUE: u32 = 8u32;
#[repr(C)]
#[cfg(all(feature = "win32-system", feature = "win32-system"))]
pub struct RSOP_TARGET {
    pub pwszAccountName: ::windows_core::PWSTR,
    pub pwszNewSOM: ::windows_core::PWSTR,
    pub psaSecurityGroups: *mut super::Com::SAFEARRAY,
    pub pRsopToken: *mut ::core::ffi::c_void,
    pub pGPOList: *mut GROUP_POLICY_OBJECTA,
    pub pWbemServices: ::core::option::Option<super::Wmi::IWbemServices>,
}
#[cfg(all(feature = "win32-system", feature = "win32-system"))]
impl ::core::clone::Clone for RSOP_TARGET {
    fn clone(&self) -> Self {
        Self {
            pwszAccountName: self.pwszAccountName,
            pwszNewSOM: self.pwszNewSOM,
            psaSecurityGroups: self.psaSecurityGroups,
            pRsopToken: self.pRsopToken,
            pGPOList: self.pGPOList,
            pWbemServices: self.pWbemServices.clone(),
        }
    }
}
#[cfg(all(feature = "win32-system", feature = "win32-system"))]
impl ::core::fmt::Debug for RSOP_TARGET {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RSOP_TARGET").field("pwszAccountName", &self.pwszAccountName).field("pwszNewSOM", &self.pwszNewSOM).field("psaSecurityGroups", &self.psaSecurityGroups).field("pRsopToken", &self.pRsopToken).field("pGPOList", &self.pGPOList).field("pWbemServices", &self.pWbemServices).finish()
    }
}
#[cfg(all(feature = "win32-system", feature = "win32-system"))]
unsafe impl ::windows_core::Abi for RSOP_TARGET {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
#[cfg(all(feature = "win32-system", feature = "win32-system"))]
impl ::core::cmp::PartialEq for RSOP_TARGET {
    fn eq(&self, other: &Self) -> bool {
        self.pwszAccountName == other.pwszAccountName && self.pwszNewSOM == other.pwszNewSOM && self.psaSecurityGroups == other.psaSecurityGroups && self.pRsopToken == other.pRsopToken && self.pGPOList == other.pGPOList && self.pWbemServices == other.pWbemServices
    }
}
#[cfg(all(feature = "win32-system", feature = "win32-system"))]
impl ::core::cmp::Eq for RSOP_TARGET {}
#[cfg(all(feature = "win32-system", feature = "win32-system"))]
impl ::core::default::Default for RSOP_TARGET {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const RSOP_TEMPNAMESPACE_EXISTS: u32 = 4u32;
pub const RSOP_USER_ACCESS_DENIED: u32 = 1u32;
#[inline]
pub unsafe fn RefreshPolicy<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(bmachine: Param0) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RefreshPolicy(bmachine: ::win32_foundation::BOOL) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(RefreshPolicy(bmachine.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn RefreshPolicyEx<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(bmachine: Param0, dwoptions: u32) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RefreshPolicyEx(bmachine: ::win32_foundation::BOOL, dwoptions: u32) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(RefreshPolicyEx(bmachine.into_param().abi(), ::core::mem::transmute(dwoptions)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn RegisterGPNotification<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HANDLE>, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(hevent: Param0, bmachine: Param1) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RegisterGPNotification(hevent: ::win32_foundation::HANDLE, bmachine: ::win32_foundation::BOOL) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(RegisterGPNotification(hevent.into_param().abi(), bmachine.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "win32-security")]
#[inline]
pub unsafe fn RsopAccessCheckByType<'a, Param0: ::windows_core::IntoParam<'a, ::win32_security::PSECURITY_DESCRIPTOR>, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::PSID>>(psecuritydescriptor: Param0, pprincipalselfsid: Param1, prsoptoken: *const ::core::ffi::c_void, dwdesiredaccessmask: u32, pobjecttypelist: &[::win32_security::OBJECT_TYPE_LIST], pgenericmapping: *const ::win32_security::GENERIC_MAPPING, pprivilegeset: *const ::win32_security::PRIVILEGE_SET, pdwprivilegesetlength: *const u32, pdwgrantedaccessmask: *mut u32, pbaccessstatus: *mut i32) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RsopAccessCheckByType(psecuritydescriptor: ::win32_security::PSECURITY_DESCRIPTOR, pprincipalselfsid: ::win32_foundation::PSID, prsoptoken: *const ::core::ffi::c_void, dwdesiredaccessmask: u32, pobjecttypelist: *const ::win32_security::OBJECT_TYPE_LIST, objecttypelistlength: u32, pgenericmapping: *const ::win32_security::GENERIC_MAPPING, pprivilegeset: *const ::win32_security::PRIVILEGE_SET, pdwprivilegesetlength: *const u32, pdwgrantedaccessmask: *mut u32, pbaccessstatus: *mut i32) -> ::windows_core::HRESULT;
        }
        RsopAccessCheckByType(psecuritydescriptor.into_param().abi(), pprincipalselfsid.into_param().abi(), ::core::mem::transmute(prsoptoken), ::core::mem::transmute(dwdesiredaccessmask), ::core::mem::transmute(::windows_core::as_ptr_or_null(pobjecttypelist)), pobjecttypelist.len() as _, ::core::mem::transmute(pgenericmapping), ::core::mem::transmute(pprivilegeset), ::core::mem::transmute(pdwprivilegesetlength), ::core::mem::transmute(pdwgrantedaccessmask), ::core::mem::transmute(pbaccessstatus)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn RsopFileAccessCheck<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(pszfilename: Param0, prsoptoken: *const ::core::ffi::c_void, dwdesiredaccessmask: u32, pdwgrantedaccessmask: *mut u32, pbaccessstatus: *mut i32) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RsopFileAccessCheck(pszfilename: ::windows_core::PCWSTR, prsoptoken: *const ::core::ffi::c_void, dwdesiredaccessmask: u32, pdwgrantedaccessmask: *mut u32, pbaccessstatus: *mut i32) -> ::windows_core::HRESULT;
        }
        RsopFileAccessCheck(pszfilename.into_param().abi(), ::core::mem::transmute(prsoptoken), ::core::mem::transmute(dwdesiredaccessmask), ::core::mem::transmute(pdwgrantedaccessmask), ::core::mem::transmute(pbaccessstatus)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "win32-system")]
#[inline]
pub unsafe fn RsopResetPolicySettingStatus<'a, Param1: ::windows_core::IntoParam<'a, super::Wmi::IWbemServices>, Param2: ::windows_core::IntoParam<'a, super::Wmi::IWbemClassObject>>(dwflags: u32, pservices: Param1, psettinginstance: Param2) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RsopResetPolicySettingStatus(dwflags: u32, pservices: ::windows_core::RawPtr, psettinginstance: ::windows_core::RawPtr) -> ::windows_core::HRESULT;
        }
        RsopResetPolicySettingStatus(::core::mem::transmute(dwflags), pservices.into_param().abi(), psettinginstance.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "win32-system")]
#[inline]
pub unsafe fn RsopSetPolicySettingStatus<'a, Param1: ::windows_core::IntoParam<'a, super::Wmi::IWbemServices>, Param2: ::windows_core::IntoParam<'a, super::Wmi::IWbemClassObject>>(dwflags: u32, pservices: Param1, psettinginstance: Param2, pstatus: &[POLICYSETTINGSTATUSINFO]) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RsopSetPolicySettingStatus(dwflags: u32, pservices: ::windows_core::RawPtr, psettinginstance: ::windows_core::RawPtr, ninfo: u32, pstatus: *const POLICYSETTINGSTATUSINFO) -> ::windows_core::HRESULT;
        }
        RsopSetPolicySettingStatus(::core::mem::transmute(dwflags), pservices.into_param().abi(), psettinginstance.into_param().abi(), pstatus.len() as _, ::core::mem::transmute(::windows_core::as_ptr_or_null(pstatus))).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct SETTINGSTATUS(pub i32);
pub const RSOPUnspecified: SETTINGSTATUS = SETTINGSTATUS(0i32);
pub const RSOPApplied: SETTINGSTATUS = SETTINGSTATUS(1i32);
pub const RSOPIgnored: SETTINGSTATUS = SETTINGSTATUS(2i32);
pub const RSOPFailed: SETTINGSTATUS = SETTINGSTATUS(3i32);
pub const RSOPSubsettingFailed: SETTINGSTATUS = SETTINGSTATUS(4i32);
impl ::core::marker::Copy for SETTINGSTATUS {}
impl ::core::clone::Clone for SETTINGSTATUS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SETTINGSTATUS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for SETTINGSTATUS {
    type Abi = Self;
}
impl ::core::fmt::Debug for SETTINGSTATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SETTINGSTATUS").field(&self.0).finish()
    }
}
#[inline]
pub unsafe fn UninstallApplication<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(productcode: Param0, dwstatus: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn UninstallApplication(productcode: ::windows_core::PCWSTR, dwstatus: u32) -> u32;
        }
        ::core::mem::transmute(UninstallApplication(productcode.into_param().abi(), ::core::mem::transmute(dwstatus)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn UnregisterGPNotification<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HANDLE>>(hevent: Param0) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn UnregisterGPNotification(hevent: ::win32_foundation::HANDLE) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(UnregisterGPNotification(hevent.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
