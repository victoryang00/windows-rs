#[cfg(feature = "UI")]
pub mod UI;
pub const ACCCTRL_DEFAULT_PROVIDER: &str = "Windows NT Access Provider";
pub const ACCCTRL_DEFAULT_PROVIDERA: &str = "Windows NT Access Provider";
pub const ACCCTRL_DEFAULT_PROVIDERW: &str = "Windows NT Access Provider";
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct ACCESS_MODE(pub i32);
pub const NOT_USED_ACCESS: ACCESS_MODE = ACCESS_MODE(0i32);
pub const GRANT_ACCESS: ACCESS_MODE = ACCESS_MODE(1i32);
pub const SET_ACCESS: ACCESS_MODE = ACCESS_MODE(2i32);
pub const DENY_ACCESS: ACCESS_MODE = ACCESS_MODE(3i32);
pub const REVOKE_ACCESS: ACCESS_MODE = ACCESS_MODE(4i32);
pub const SET_AUDIT_SUCCESS: ACCESS_MODE = ACCESS_MODE(5i32);
pub const SET_AUDIT_FAILURE: ACCESS_MODE = ACCESS_MODE(6i32);
impl ::core::marker::Copy for ACCESS_MODE {}
impl ::core::clone::Clone for ACCESS_MODE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ACCESS_MODE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for ACCESS_MODE {
    type Abi = Self;
}
impl ::core::fmt::Debug for ACCESS_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ACCESS_MODE").field(&self.0).finish()
    }
}
#[repr(C)]
pub struct ACTRL_ACCESSA {
    pub cEntries: u32,
    pub pPropertyAccessList: *mut ACTRL_PROPERTY_ENTRYA,
}
impl ::core::marker::Copy for ACTRL_ACCESSA {}
impl ::core::clone::Clone for ACTRL_ACCESSA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ACTRL_ACCESSA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ACTRL_ACCESSA").field("cEntries", &self.cEntries).field("pPropertyAccessList", &self.pPropertyAccessList).finish()
    }
}
unsafe impl ::windows_core::Abi for ACTRL_ACCESSA {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for ACTRL_ACCESSA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<ACTRL_ACCESSA>()) == 0 }
    }
}
impl ::core::cmp::Eq for ACTRL_ACCESSA {}
impl ::core::default::Default for ACTRL_ACCESSA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct ACTRL_ACCESSW {
    pub cEntries: u32,
    pub pPropertyAccessList: *mut ACTRL_PROPERTY_ENTRYW,
}
impl ::core::marker::Copy for ACTRL_ACCESSW {}
impl ::core::clone::Clone for ACTRL_ACCESSW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ACTRL_ACCESSW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ACTRL_ACCESSW").field("cEntries", &self.cEntries).field("pPropertyAccessList", &self.pPropertyAccessList).finish()
    }
}
unsafe impl ::windows_core::Abi for ACTRL_ACCESSW {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for ACTRL_ACCESSW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<ACTRL_ACCESSW>()) == 0 }
    }
}
impl ::core::cmp::Eq for ACTRL_ACCESSW {}
impl ::core::default::Default for ACTRL_ACCESSW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct ACTRL_ACCESS_ENTRYA {
    pub Trustee: TRUSTEE_A,
    pub fAccessFlags: ACTRL_ACCESS_ENTRY_ACCESS_FLAGS,
    pub Access: u32,
    pub ProvSpecificAccess: u32,
    pub Inheritance: super::ACE_FLAGS,
    pub lpInheritProperty: ::windows_core::PSTR,
}
impl ::core::marker::Copy for ACTRL_ACCESS_ENTRYA {}
impl ::core::clone::Clone for ACTRL_ACCESS_ENTRYA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ACTRL_ACCESS_ENTRYA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ACTRL_ACCESS_ENTRYA").field("Trustee", &self.Trustee).field("fAccessFlags", &self.fAccessFlags).field("Access", &self.Access).field("ProvSpecificAccess", &self.ProvSpecificAccess).field("Inheritance", &self.Inheritance).field("lpInheritProperty", &self.lpInheritProperty).finish()
    }
}
unsafe impl ::windows_core::Abi for ACTRL_ACCESS_ENTRYA {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for ACTRL_ACCESS_ENTRYA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<ACTRL_ACCESS_ENTRYA>()) == 0 }
    }
}
impl ::core::cmp::Eq for ACTRL_ACCESS_ENTRYA {}
impl ::core::default::Default for ACTRL_ACCESS_ENTRYA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct ACTRL_ACCESS_ENTRYW {
    pub Trustee: TRUSTEE_W,
    pub fAccessFlags: ACTRL_ACCESS_ENTRY_ACCESS_FLAGS,
    pub Access: u32,
    pub ProvSpecificAccess: u32,
    pub Inheritance: super::ACE_FLAGS,
    pub lpInheritProperty: ::windows_core::PWSTR,
}
impl ::core::marker::Copy for ACTRL_ACCESS_ENTRYW {}
impl ::core::clone::Clone for ACTRL_ACCESS_ENTRYW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ACTRL_ACCESS_ENTRYW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ACTRL_ACCESS_ENTRYW").field("Trustee", &self.Trustee).field("fAccessFlags", &self.fAccessFlags).field("Access", &self.Access).field("ProvSpecificAccess", &self.ProvSpecificAccess).field("Inheritance", &self.Inheritance).field("lpInheritProperty", &self.lpInheritProperty).finish()
    }
}
unsafe impl ::windows_core::Abi for ACTRL_ACCESS_ENTRYW {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for ACTRL_ACCESS_ENTRYW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<ACTRL_ACCESS_ENTRYW>()) == 0 }
    }
}
impl ::core::cmp::Eq for ACTRL_ACCESS_ENTRYW {}
impl ::core::default::Default for ACTRL_ACCESS_ENTRYW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct ACTRL_ACCESS_ENTRY_ACCESS_FLAGS(pub u32);
pub const ACTRL_ACCESS_ALLOWED: ACTRL_ACCESS_ENTRY_ACCESS_FLAGS = ACTRL_ACCESS_ENTRY_ACCESS_FLAGS(1u32);
pub const ACTRL_ACCESS_DENIED: ACTRL_ACCESS_ENTRY_ACCESS_FLAGS = ACTRL_ACCESS_ENTRY_ACCESS_FLAGS(2u32);
pub const ACTRL_AUDIT_SUCCESS: ACTRL_ACCESS_ENTRY_ACCESS_FLAGS = ACTRL_ACCESS_ENTRY_ACCESS_FLAGS(4u32);
pub const ACTRL_AUDIT_FAILURE: ACTRL_ACCESS_ENTRY_ACCESS_FLAGS = ACTRL_ACCESS_ENTRY_ACCESS_FLAGS(8u32);
impl ::core::marker::Copy for ACTRL_ACCESS_ENTRY_ACCESS_FLAGS {}
impl ::core::clone::Clone for ACTRL_ACCESS_ENTRY_ACCESS_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ACTRL_ACCESS_ENTRY_ACCESS_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for ACTRL_ACCESS_ENTRY_ACCESS_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for ACTRL_ACCESS_ENTRY_ACCESS_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ACTRL_ACCESS_ENTRY_ACCESS_FLAGS").field(&self.0).finish()
    }
}
#[repr(C)]
pub struct ACTRL_ACCESS_ENTRY_LISTA {
    pub cEntries: u32,
    pub pAccessList: *mut ACTRL_ACCESS_ENTRYA,
}
impl ::core::marker::Copy for ACTRL_ACCESS_ENTRY_LISTA {}
impl ::core::clone::Clone for ACTRL_ACCESS_ENTRY_LISTA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ACTRL_ACCESS_ENTRY_LISTA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ACTRL_ACCESS_ENTRY_LISTA").field("cEntries", &self.cEntries).field("pAccessList", &self.pAccessList).finish()
    }
}
unsafe impl ::windows_core::Abi for ACTRL_ACCESS_ENTRY_LISTA {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for ACTRL_ACCESS_ENTRY_LISTA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<ACTRL_ACCESS_ENTRY_LISTA>()) == 0 }
    }
}
impl ::core::cmp::Eq for ACTRL_ACCESS_ENTRY_LISTA {}
impl ::core::default::Default for ACTRL_ACCESS_ENTRY_LISTA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct ACTRL_ACCESS_ENTRY_LISTW {
    pub cEntries: u32,
    pub pAccessList: *mut ACTRL_ACCESS_ENTRYW,
}
impl ::core::marker::Copy for ACTRL_ACCESS_ENTRY_LISTW {}
impl ::core::clone::Clone for ACTRL_ACCESS_ENTRY_LISTW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ACTRL_ACCESS_ENTRY_LISTW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ACTRL_ACCESS_ENTRY_LISTW").field("cEntries", &self.cEntries).field("pAccessList", &self.pAccessList).finish()
    }
}
unsafe impl ::windows_core::Abi for ACTRL_ACCESS_ENTRY_LISTW {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for ACTRL_ACCESS_ENTRY_LISTW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<ACTRL_ACCESS_ENTRY_LISTW>()) == 0 }
    }
}
impl ::core::cmp::Eq for ACTRL_ACCESS_ENTRY_LISTW {}
impl ::core::default::Default for ACTRL_ACCESS_ENTRY_LISTW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct ACTRL_ACCESS_INFOA {
    pub fAccessPermission: u32,
    pub lpAccessPermissionName: ::windows_core::PSTR,
}
impl ::core::marker::Copy for ACTRL_ACCESS_INFOA {}
impl ::core::clone::Clone for ACTRL_ACCESS_INFOA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ACTRL_ACCESS_INFOA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ACTRL_ACCESS_INFOA").field("fAccessPermission", &self.fAccessPermission).field("lpAccessPermissionName", &self.lpAccessPermissionName).finish()
    }
}
unsafe impl ::windows_core::Abi for ACTRL_ACCESS_INFOA {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for ACTRL_ACCESS_INFOA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<ACTRL_ACCESS_INFOA>()) == 0 }
    }
}
impl ::core::cmp::Eq for ACTRL_ACCESS_INFOA {}
impl ::core::default::Default for ACTRL_ACCESS_INFOA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct ACTRL_ACCESS_INFOW {
    pub fAccessPermission: u32,
    pub lpAccessPermissionName: ::windows_core::PWSTR,
}
impl ::core::marker::Copy for ACTRL_ACCESS_INFOW {}
impl ::core::clone::Clone for ACTRL_ACCESS_INFOW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ACTRL_ACCESS_INFOW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ACTRL_ACCESS_INFOW").field("fAccessPermission", &self.fAccessPermission).field("lpAccessPermissionName", &self.lpAccessPermissionName).finish()
    }
}
unsafe impl ::windows_core::Abi for ACTRL_ACCESS_INFOW {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for ACTRL_ACCESS_INFOW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<ACTRL_ACCESS_INFOW>()) == 0 }
    }
}
impl ::core::cmp::Eq for ACTRL_ACCESS_INFOW {}
impl ::core::default::Default for ACTRL_ACCESS_INFOW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const ACTRL_ACCESS_NO_OPTIONS: u32 = 0u32;
pub const ACTRL_ACCESS_PROTECTED: u32 = 1u32;
pub const ACTRL_ACCESS_SUPPORTS_OBJECT_ENTRIES: u32 = 1u32;
pub const ACTRL_CHANGE_ACCESS: u32 = 536870912u32;
pub const ACTRL_CHANGE_OWNER: u32 = 1073741824u32;
#[repr(C)]
pub struct ACTRL_CONTROL_INFOA {
    pub lpControlId: ::windows_core::PSTR,
    pub lpControlName: ::windows_core::PSTR,
}
impl ::core::marker::Copy for ACTRL_CONTROL_INFOA {}
impl ::core::clone::Clone for ACTRL_CONTROL_INFOA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ACTRL_CONTROL_INFOA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ACTRL_CONTROL_INFOA").field("lpControlId", &self.lpControlId).field("lpControlName", &self.lpControlName).finish()
    }
}
unsafe impl ::windows_core::Abi for ACTRL_CONTROL_INFOA {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for ACTRL_CONTROL_INFOA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<ACTRL_CONTROL_INFOA>()) == 0 }
    }
}
impl ::core::cmp::Eq for ACTRL_CONTROL_INFOA {}
impl ::core::default::Default for ACTRL_CONTROL_INFOA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct ACTRL_CONTROL_INFOW {
    pub lpControlId: ::windows_core::PWSTR,
    pub lpControlName: ::windows_core::PWSTR,
}
impl ::core::marker::Copy for ACTRL_CONTROL_INFOW {}
impl ::core::clone::Clone for ACTRL_CONTROL_INFOW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ACTRL_CONTROL_INFOW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ACTRL_CONTROL_INFOW").field("lpControlId", &self.lpControlId).field("lpControlName", &self.lpControlName).finish()
    }
}
unsafe impl ::windows_core::Abi for ACTRL_CONTROL_INFOW {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for ACTRL_CONTROL_INFOW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<ACTRL_CONTROL_INFOW>()) == 0 }
    }
}
impl ::core::cmp::Eq for ACTRL_CONTROL_INFOW {}
impl ::core::default::Default for ACTRL_CONTROL_INFOW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const ACTRL_DELETE: u32 = 134217728u32;
pub const ACTRL_DIR_CREATE_CHILD: u32 = 4u32;
pub const ACTRL_DIR_CREATE_OBJECT: u32 = 2u32;
pub const ACTRL_DIR_DELETE_CHILD: u32 = 64u32;
pub const ACTRL_DIR_LIST: u32 = 1u32;
pub const ACTRL_DIR_TRAVERSE: u32 = 32u32;
pub const ACTRL_FILE_APPEND: u32 = 4u32;
pub const ACTRL_FILE_CREATE_PIPE: u32 = 512u32;
pub const ACTRL_FILE_EXECUTE: u32 = 32u32;
pub const ACTRL_FILE_READ: u32 = 1u32;
pub const ACTRL_FILE_READ_ATTRIB: u32 = 128u32;
pub const ACTRL_FILE_READ_PROP: u32 = 8u32;
pub const ACTRL_FILE_WRITE: u32 = 2u32;
pub const ACTRL_FILE_WRITE_ATTRIB: u32 = 256u32;
pub const ACTRL_FILE_WRITE_PROP: u32 = 16u32;
pub const ACTRL_KERNEL_ALERT: u32 = 1024u32;
pub const ACTRL_KERNEL_CONTROL: u32 = 512u32;
pub const ACTRL_KERNEL_DIMPERSONATE: u32 = 32768u32;
pub const ACTRL_KERNEL_DUP_HANDLE: u32 = 32u32;
pub const ACTRL_KERNEL_GET_CONTEXT: u32 = 2048u32;
pub const ACTRL_KERNEL_GET_INFO: u32 = 256u32;
pub const ACTRL_KERNEL_IMPERSONATE: u32 = 16384u32;
pub const ACTRL_KERNEL_PROCESS: u32 = 64u32;
pub const ACTRL_KERNEL_SET_CONTEXT: u32 = 4096u32;
pub const ACTRL_KERNEL_SET_INFO: u32 = 128u32;
pub const ACTRL_KERNEL_TERMINATE: u32 = 1u32;
pub const ACTRL_KERNEL_THREAD: u32 = 2u32;
pub const ACTRL_KERNEL_TOKEN: u32 = 8192u32;
pub const ACTRL_KERNEL_VM: u32 = 4u32;
pub const ACTRL_KERNEL_VM_READ: u32 = 8u32;
pub const ACTRL_KERNEL_VM_WRITE: u32 = 16u32;
#[repr(C)]
pub struct ACTRL_OVERLAPPED {
    pub Anonymous: ACTRL_OVERLAPPED_0,
    pub Reserved2: u32,
    pub hEvent: ::win32_foundation::HANDLE,
}
impl ::core::marker::Copy for ACTRL_OVERLAPPED {}
impl ::core::clone::Clone for ACTRL_OVERLAPPED {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows_core::Abi for ACTRL_OVERLAPPED {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for ACTRL_OVERLAPPED {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<ACTRL_OVERLAPPED>()) == 0 }
    }
}
impl ::core::cmp::Eq for ACTRL_OVERLAPPED {}
impl ::core::default::Default for ACTRL_OVERLAPPED {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub union ACTRL_OVERLAPPED_0 {
    pub Provider: *mut ::core::ffi::c_void,
    pub Reserved1: u32,
}
impl ::core::marker::Copy for ACTRL_OVERLAPPED_0 {}
impl ::core::clone::Clone for ACTRL_OVERLAPPED_0 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows_core::Abi for ACTRL_OVERLAPPED_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for ACTRL_OVERLAPPED_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<ACTRL_OVERLAPPED_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for ACTRL_OVERLAPPED_0 {}
impl ::core::default::Default for ACTRL_OVERLAPPED_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const ACTRL_PERM_1: u32 = 1u32;
pub const ACTRL_PERM_10: u32 = 512u32;
pub const ACTRL_PERM_11: u32 = 1024u32;
pub const ACTRL_PERM_12: u32 = 2048u32;
pub const ACTRL_PERM_13: u32 = 4096u32;
pub const ACTRL_PERM_14: u32 = 8192u32;
pub const ACTRL_PERM_15: u32 = 16384u32;
pub const ACTRL_PERM_16: u32 = 32768u32;
pub const ACTRL_PERM_17: u32 = 65536u32;
pub const ACTRL_PERM_18: u32 = 131072u32;
pub const ACTRL_PERM_19: u32 = 262144u32;
pub const ACTRL_PERM_2: u32 = 2u32;
pub const ACTRL_PERM_20: u32 = 524288u32;
pub const ACTRL_PERM_3: u32 = 4u32;
pub const ACTRL_PERM_4: u32 = 8u32;
pub const ACTRL_PERM_5: u32 = 16u32;
pub const ACTRL_PERM_6: u32 = 32u32;
pub const ACTRL_PERM_7: u32 = 64u32;
pub const ACTRL_PERM_8: u32 = 128u32;
pub const ACTRL_PERM_9: u32 = 256u32;
pub const ACTRL_PRINT_JADMIN: u32 = 16u32;
pub const ACTRL_PRINT_PADMIN: u32 = 4u32;
pub const ACTRL_PRINT_PUSE: u32 = 8u32;
pub const ACTRL_PRINT_SADMIN: u32 = 1u32;
pub const ACTRL_PRINT_SLIST: u32 = 2u32;
#[repr(C)]
pub struct ACTRL_PROPERTY_ENTRYA {
    pub lpProperty: ::windows_core::PSTR,
    pub pAccessEntryList: *mut ACTRL_ACCESS_ENTRY_LISTA,
    pub fListFlags: u32,
}
impl ::core::marker::Copy for ACTRL_PROPERTY_ENTRYA {}
impl ::core::clone::Clone for ACTRL_PROPERTY_ENTRYA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ACTRL_PROPERTY_ENTRYA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ACTRL_PROPERTY_ENTRYA").field("lpProperty", &self.lpProperty).field("pAccessEntryList", &self.pAccessEntryList).field("fListFlags", &self.fListFlags).finish()
    }
}
unsafe impl ::windows_core::Abi for ACTRL_PROPERTY_ENTRYA {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for ACTRL_PROPERTY_ENTRYA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<ACTRL_PROPERTY_ENTRYA>()) == 0 }
    }
}
impl ::core::cmp::Eq for ACTRL_PROPERTY_ENTRYA {}
impl ::core::default::Default for ACTRL_PROPERTY_ENTRYA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct ACTRL_PROPERTY_ENTRYW {
    pub lpProperty: ::windows_core::PWSTR,
    pub pAccessEntryList: *mut ACTRL_ACCESS_ENTRY_LISTW,
    pub fListFlags: u32,
}
impl ::core::marker::Copy for ACTRL_PROPERTY_ENTRYW {}
impl ::core::clone::Clone for ACTRL_PROPERTY_ENTRYW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ACTRL_PROPERTY_ENTRYW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ACTRL_PROPERTY_ENTRYW").field("lpProperty", &self.lpProperty).field("pAccessEntryList", &self.pAccessEntryList).field("fListFlags", &self.fListFlags).finish()
    }
}
unsafe impl ::windows_core::Abi for ACTRL_PROPERTY_ENTRYW {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for ACTRL_PROPERTY_ENTRYW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<ACTRL_PROPERTY_ENTRYW>()) == 0 }
    }
}
impl ::core::cmp::Eq for ACTRL_PROPERTY_ENTRYW {}
impl ::core::default::Default for ACTRL_PROPERTY_ENTRYW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const ACTRL_READ_CONTROL: u32 = 268435456u32;
pub const ACTRL_REG_CREATE_CHILD: u32 = 4u32;
pub const ACTRL_REG_LINK: u32 = 32u32;
pub const ACTRL_REG_LIST: u32 = 8u32;
pub const ACTRL_REG_NOTIFY: u32 = 16u32;
pub const ACTRL_REG_QUERY: u32 = 1u32;
pub const ACTRL_REG_SET: u32 = 2u32;
pub const ACTRL_RESERVED: u32 = 0u32;
pub const ACTRL_STD_RIGHTS_ALL: u32 = 4160749568u32;
pub const ACTRL_SVC_GET_INFO: u32 = 1u32;
pub const ACTRL_SVC_INTERROGATE: u32 = 128u32;
pub const ACTRL_SVC_LIST: u32 = 8u32;
pub const ACTRL_SVC_PAUSE: u32 = 64u32;
pub const ACTRL_SVC_SET_INFO: u32 = 2u32;
pub const ACTRL_SVC_START: u32 = 16u32;
pub const ACTRL_SVC_STATUS: u32 = 4u32;
pub const ACTRL_SVC_STOP: u32 = 32u32;
pub const ACTRL_SVC_UCONTROL: u32 = 256u32;
pub const ACTRL_SYNCHRONIZE: u32 = 2147483648u32;
pub const ACTRL_SYSTEM_ACCESS: u32 = 67108864u32;
pub const ACTRL_WIN_CLIPBRD: u32 = 1u32;
pub const ACTRL_WIN_CREATE: u32 = 4u32;
pub const ACTRL_WIN_EXIT: u32 = 256u32;
pub const ACTRL_WIN_GLOBAL_ATOMS: u32 = 2u32;
pub const ACTRL_WIN_LIST: u32 = 16u32;
pub const ACTRL_WIN_LIST_DESK: u32 = 8u32;
pub const ACTRL_WIN_READ_ATTRIBS: u32 = 32u32;
pub const ACTRL_WIN_SCREEN: u32 = 128u32;
pub const ACTRL_WIN_WRITE_ATTRIBS: u32 = 64u32;
pub const APF_AuditFailure: u32 = 0u32;
pub const APF_AuditSuccess: u32 = 1u32;
pub const APF_ValidFlags: u32 = 1u32;
pub const AP_ParamTypeBits: u32 = 8u32;
pub const AP_ParamTypeMask: i32 = 255i32;
#[repr(C)]
pub struct AUDIT_IP_ADDRESS {
    pub pIpAddress: [u8; 128],
}
impl ::core::marker::Copy for AUDIT_IP_ADDRESS {}
impl ::core::clone::Clone for AUDIT_IP_ADDRESS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for AUDIT_IP_ADDRESS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("AUDIT_IP_ADDRESS").field("pIpAddress", &self.pIpAddress).finish()
    }
}
unsafe impl ::windows_core::Abi for AUDIT_IP_ADDRESS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for AUDIT_IP_ADDRESS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<AUDIT_IP_ADDRESS>()) == 0 }
    }
}
impl ::core::cmp::Eq for AUDIT_IP_ADDRESS {}
impl ::core::default::Default for AUDIT_IP_ADDRESS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct AUDIT_OBJECT_TYPE {
    pub ObjectType: ::windows_core::GUID,
    pub Flags: u16,
    pub Level: u16,
    pub AccessMask: u32,
}
impl ::core::marker::Copy for AUDIT_OBJECT_TYPE {}
impl ::core::clone::Clone for AUDIT_OBJECT_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for AUDIT_OBJECT_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("AUDIT_OBJECT_TYPE").field("ObjectType", &self.ObjectType).field("Flags", &self.Flags).field("Level", &self.Level).field("AccessMask", &self.AccessMask).finish()
    }
}
unsafe impl ::windows_core::Abi for AUDIT_OBJECT_TYPE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for AUDIT_OBJECT_TYPE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<AUDIT_OBJECT_TYPE>()) == 0 }
    }
}
impl ::core::cmp::Eq for AUDIT_OBJECT_TYPE {}
impl ::core::default::Default for AUDIT_OBJECT_TYPE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct AUDIT_OBJECT_TYPES {
    pub Count: u16,
    pub Flags: u16,
    pub pObjectTypes: *mut AUDIT_OBJECT_TYPE,
}
impl ::core::marker::Copy for AUDIT_OBJECT_TYPES {}
impl ::core::clone::Clone for AUDIT_OBJECT_TYPES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for AUDIT_OBJECT_TYPES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("AUDIT_OBJECT_TYPES").field("Count", &self.Count).field("Flags", &self.Flags).field("pObjectTypes", &self.pObjectTypes).finish()
    }
}
unsafe impl ::windows_core::Abi for AUDIT_OBJECT_TYPES {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for AUDIT_OBJECT_TYPES {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<AUDIT_OBJECT_TYPES>()) == 0 }
    }
}
impl ::core::cmp::Eq for AUDIT_OBJECT_TYPES {}
impl ::core::default::Default for AUDIT_OBJECT_TYPES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct AUDIT_PARAM {
    pub Type: AUDIT_PARAM_TYPE,
    pub Length: u32,
    pub Flags: u32,
    pub Anonymous1: AUDIT_PARAM_0,
    pub Anonymous2: AUDIT_PARAM_1,
}
impl ::core::marker::Copy for AUDIT_PARAM {}
impl ::core::clone::Clone for AUDIT_PARAM {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows_core::Abi for AUDIT_PARAM {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for AUDIT_PARAM {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<AUDIT_PARAM>()) == 0 }
    }
}
impl ::core::cmp::Eq for AUDIT_PARAM {}
impl ::core::default::Default for AUDIT_PARAM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub union AUDIT_PARAM_0 {
    pub Data0: usize,
    pub String: ::windows_core::PWSTR,
    pub u: usize,
    pub psid: *mut super::SID,
    pub pguid: *mut ::windows_core::GUID,
    pub LogonId_LowPart: u32,
    pub pObjectTypes: *mut AUDIT_OBJECT_TYPES,
    pub pIpAddress: *mut AUDIT_IP_ADDRESS,
}
impl ::core::marker::Copy for AUDIT_PARAM_0 {}
impl ::core::clone::Clone for AUDIT_PARAM_0 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows_core::Abi for AUDIT_PARAM_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for AUDIT_PARAM_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<AUDIT_PARAM_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for AUDIT_PARAM_0 {}
impl ::core::default::Default for AUDIT_PARAM_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub union AUDIT_PARAM_1 {
    pub Data1: usize,
    pub LogonId_HighPart: i32,
}
impl ::core::marker::Copy for AUDIT_PARAM_1 {}
impl ::core::clone::Clone for AUDIT_PARAM_1 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows_core::Abi for AUDIT_PARAM_1 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for AUDIT_PARAM_1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<AUDIT_PARAM_1>()) == 0 }
    }
}
impl ::core::cmp::Eq for AUDIT_PARAM_1 {}
impl ::core::default::Default for AUDIT_PARAM_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct AUDIT_PARAMS {
    pub Length: u32,
    pub Flags: u32,
    pub Count: u16,
    pub Parameters: *mut AUDIT_PARAM,
}
impl ::core::marker::Copy for AUDIT_PARAMS {}
impl ::core::clone::Clone for AUDIT_PARAMS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for AUDIT_PARAMS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("AUDIT_PARAMS").field("Length", &self.Length).field("Flags", &self.Flags).field("Count", &self.Count).field("Parameters", &self.Parameters).finish()
    }
}
unsafe impl ::windows_core::Abi for AUDIT_PARAMS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for AUDIT_PARAMS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<AUDIT_PARAMS>()) == 0 }
    }
}
impl ::core::cmp::Eq for AUDIT_PARAMS {}
impl ::core::default::Default for AUDIT_PARAMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct AUDIT_PARAM_TYPE(pub i32);
pub const APT_None: AUDIT_PARAM_TYPE = AUDIT_PARAM_TYPE(1i32);
pub const APT_String: AUDIT_PARAM_TYPE = AUDIT_PARAM_TYPE(2i32);
pub const APT_Ulong: AUDIT_PARAM_TYPE = AUDIT_PARAM_TYPE(3i32);
pub const APT_Pointer: AUDIT_PARAM_TYPE = AUDIT_PARAM_TYPE(4i32);
pub const APT_Sid: AUDIT_PARAM_TYPE = AUDIT_PARAM_TYPE(5i32);
pub const APT_LogonId: AUDIT_PARAM_TYPE = AUDIT_PARAM_TYPE(6i32);
pub const APT_ObjectTypeList: AUDIT_PARAM_TYPE = AUDIT_PARAM_TYPE(7i32);
pub const APT_Luid: AUDIT_PARAM_TYPE = AUDIT_PARAM_TYPE(8i32);
pub const APT_Guid: AUDIT_PARAM_TYPE = AUDIT_PARAM_TYPE(9i32);
pub const APT_Time: AUDIT_PARAM_TYPE = AUDIT_PARAM_TYPE(10i32);
pub const APT_Int64: AUDIT_PARAM_TYPE = AUDIT_PARAM_TYPE(11i32);
pub const APT_IpAddress: AUDIT_PARAM_TYPE = AUDIT_PARAM_TYPE(12i32);
pub const APT_LogonIdWithSid: AUDIT_PARAM_TYPE = AUDIT_PARAM_TYPE(13i32);
impl ::core::marker::Copy for AUDIT_PARAM_TYPE {}
impl ::core::clone::Clone for AUDIT_PARAM_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AUDIT_PARAM_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for AUDIT_PARAM_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for AUDIT_PARAM_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AUDIT_PARAM_TYPE").field(&self.0).finish()
    }
}
pub const AUDIT_TYPE_LEGACY: u32 = 1u32;
pub const AUDIT_TYPE_WMI: u32 = 2u32;
pub const AUTHZP_WPD_EVENT: u32 = 16u32;
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct AUTHZ_ACCESS_CHECK_FLAGS(pub u32);
pub const AUTHZ_ACCESS_CHECK_NO_DEEP_COPY_SD: AUTHZ_ACCESS_CHECK_FLAGS = AUTHZ_ACCESS_CHECK_FLAGS(1u32);
impl ::core::marker::Copy for AUTHZ_ACCESS_CHECK_FLAGS {}
impl ::core::clone::Clone for AUTHZ_ACCESS_CHECK_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AUTHZ_ACCESS_CHECK_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for AUTHZ_ACCESS_CHECK_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for AUTHZ_ACCESS_CHECK_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AUTHZ_ACCESS_CHECK_FLAGS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct AUTHZ_ACCESS_CHECK_RESULTS_HANDLE(pub isize);
impl AUTHZ_ACCESS_CHECK_RESULTS_HANDLE {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 || self.0 == 0
    }
}
impl ::core::default::Default for AUTHZ_ACCESS_CHECK_RESULTS_HANDLE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for AUTHZ_ACCESS_CHECK_RESULTS_HANDLE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for AUTHZ_ACCESS_CHECK_RESULTS_HANDLE {}
impl ::core::fmt::Debug for AUTHZ_ACCESS_CHECK_RESULTS_HANDLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AUTHZ_ACCESS_CHECK_RESULTS_HANDLE").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Abi for AUTHZ_ACCESS_CHECK_RESULTS_HANDLE {
    type Abi = Self;
}
#[repr(C)]
pub struct AUTHZ_ACCESS_REPLY {
    pub ResultListLength: u32,
    pub GrantedAccessMask: *mut u32,
    pub SaclEvaluationResults: *mut AUTHZ_GENERATE_RESULTS,
    pub Error: *mut u32,
}
impl ::core::marker::Copy for AUTHZ_ACCESS_REPLY {}
impl ::core::clone::Clone for AUTHZ_ACCESS_REPLY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for AUTHZ_ACCESS_REPLY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("AUTHZ_ACCESS_REPLY").field("ResultListLength", &self.ResultListLength).field("GrantedAccessMask", &self.GrantedAccessMask).field("SaclEvaluationResults", &self.SaclEvaluationResults).field("Error", &self.Error).finish()
    }
}
unsafe impl ::windows_core::Abi for AUTHZ_ACCESS_REPLY {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for AUTHZ_ACCESS_REPLY {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<AUTHZ_ACCESS_REPLY>()) == 0 }
    }
}
impl ::core::cmp::Eq for AUTHZ_ACCESS_REPLY {}
impl ::core::default::Default for AUTHZ_ACCESS_REPLY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct AUTHZ_ACCESS_REQUEST {
    pub DesiredAccess: u32,
    pub PrincipalSelfSid: ::win32_foundation::PSID,
    pub ObjectTypeList: *mut super::OBJECT_TYPE_LIST,
    pub ObjectTypeListLength: u32,
    pub OptionalArguments: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for AUTHZ_ACCESS_REQUEST {}
impl ::core::clone::Clone for AUTHZ_ACCESS_REQUEST {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for AUTHZ_ACCESS_REQUEST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("AUTHZ_ACCESS_REQUEST").field("DesiredAccess", &self.DesiredAccess).field("PrincipalSelfSid", &self.PrincipalSelfSid).field("ObjectTypeList", &self.ObjectTypeList).field("ObjectTypeListLength", &self.ObjectTypeListLength).field("OptionalArguments", &self.OptionalArguments).finish()
    }
}
unsafe impl ::windows_core::Abi for AUTHZ_ACCESS_REQUEST {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for AUTHZ_ACCESS_REQUEST {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<AUTHZ_ACCESS_REQUEST>()) == 0 }
    }
}
impl ::core::cmp::Eq for AUTHZ_ACCESS_REQUEST {}
impl ::core::default::Default for AUTHZ_ACCESS_REQUEST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const AUTHZ_ALLOW_MULTIPLE_SOURCE_INSTANCES: u32 = 1u32;
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct AUTHZ_AUDIT_EVENT_HANDLE(pub isize);
impl AUTHZ_AUDIT_EVENT_HANDLE {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 || self.0 == 0
    }
}
impl ::core::default::Default for AUTHZ_AUDIT_EVENT_HANDLE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for AUTHZ_AUDIT_EVENT_HANDLE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for AUTHZ_AUDIT_EVENT_HANDLE {}
impl ::core::fmt::Debug for AUTHZ_AUDIT_EVENT_HANDLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AUTHZ_AUDIT_EVENT_HANDLE").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Abi for AUTHZ_AUDIT_EVENT_HANDLE {
    type Abi = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct AUTHZ_AUDIT_EVENT_INFORMATION_CLASS(pub i32);
pub const AuthzAuditEventInfoFlags: AUTHZ_AUDIT_EVENT_INFORMATION_CLASS = AUTHZ_AUDIT_EVENT_INFORMATION_CLASS(1i32);
pub const AuthzAuditEventInfoOperationType: AUTHZ_AUDIT_EVENT_INFORMATION_CLASS = AUTHZ_AUDIT_EVENT_INFORMATION_CLASS(2i32);
pub const AuthzAuditEventInfoObjectType: AUTHZ_AUDIT_EVENT_INFORMATION_CLASS = AUTHZ_AUDIT_EVENT_INFORMATION_CLASS(3i32);
pub const AuthzAuditEventInfoObjectName: AUTHZ_AUDIT_EVENT_INFORMATION_CLASS = AUTHZ_AUDIT_EVENT_INFORMATION_CLASS(4i32);
pub const AuthzAuditEventInfoAdditionalInfo: AUTHZ_AUDIT_EVENT_INFORMATION_CLASS = AUTHZ_AUDIT_EVENT_INFORMATION_CLASS(5i32);
impl ::core::marker::Copy for AUTHZ_AUDIT_EVENT_INFORMATION_CLASS {}
impl ::core::clone::Clone for AUTHZ_AUDIT_EVENT_INFORMATION_CLASS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AUTHZ_AUDIT_EVENT_INFORMATION_CLASS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for AUTHZ_AUDIT_EVENT_INFORMATION_CLASS {
    type Abi = Self;
}
impl ::core::fmt::Debug for AUTHZ_AUDIT_EVENT_INFORMATION_CLASS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AUTHZ_AUDIT_EVENT_INFORMATION_CLASS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct AUTHZ_AUDIT_EVENT_TYPE_HANDLE(pub isize);
impl AUTHZ_AUDIT_EVENT_TYPE_HANDLE {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 || self.0 == 0
    }
}
impl ::core::default::Default for AUTHZ_AUDIT_EVENT_TYPE_HANDLE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for AUTHZ_AUDIT_EVENT_TYPE_HANDLE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for AUTHZ_AUDIT_EVENT_TYPE_HANDLE {}
impl ::core::fmt::Debug for AUTHZ_AUDIT_EVENT_TYPE_HANDLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AUTHZ_AUDIT_EVENT_TYPE_HANDLE").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Abi for AUTHZ_AUDIT_EVENT_TYPE_HANDLE {
    type Abi = Self;
}
#[repr(C)]
pub struct AUTHZ_AUDIT_EVENT_TYPE_LEGACY {
    pub CategoryId: u16,
    pub AuditId: u16,
    pub ParameterCount: u16,
}
impl ::core::marker::Copy for AUTHZ_AUDIT_EVENT_TYPE_LEGACY {}
impl ::core::clone::Clone for AUTHZ_AUDIT_EVENT_TYPE_LEGACY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for AUTHZ_AUDIT_EVENT_TYPE_LEGACY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("AUTHZ_AUDIT_EVENT_TYPE_LEGACY").field("CategoryId", &self.CategoryId).field("AuditId", &self.AuditId).field("ParameterCount", &self.ParameterCount).finish()
    }
}
unsafe impl ::windows_core::Abi for AUTHZ_AUDIT_EVENT_TYPE_LEGACY {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for AUTHZ_AUDIT_EVENT_TYPE_LEGACY {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<AUTHZ_AUDIT_EVENT_TYPE_LEGACY>()) == 0 }
    }
}
impl ::core::cmp::Eq for AUTHZ_AUDIT_EVENT_TYPE_LEGACY {}
impl ::core::default::Default for AUTHZ_AUDIT_EVENT_TYPE_LEGACY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct AUTHZ_AUDIT_EVENT_TYPE_OLD {
    pub Version: u32,
    pub dwFlags: u32,
    pub RefCount: i32,
    pub hAudit: usize,
    pub LinkId: ::win32_foundation::LUID,
    pub u: AUTHZ_AUDIT_EVENT_TYPE_UNION,
}
impl ::core::marker::Copy for AUTHZ_AUDIT_EVENT_TYPE_OLD {}
impl ::core::clone::Clone for AUTHZ_AUDIT_EVENT_TYPE_OLD {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows_core::Abi for AUTHZ_AUDIT_EVENT_TYPE_OLD {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for AUTHZ_AUDIT_EVENT_TYPE_OLD {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<AUTHZ_AUDIT_EVENT_TYPE_OLD>()) == 0 }
    }
}
impl ::core::cmp::Eq for AUTHZ_AUDIT_EVENT_TYPE_OLD {}
impl ::core::default::Default for AUTHZ_AUDIT_EVENT_TYPE_OLD {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub union AUTHZ_AUDIT_EVENT_TYPE_UNION {
    pub Legacy: AUTHZ_AUDIT_EVENT_TYPE_LEGACY,
}
impl ::core::marker::Copy for AUTHZ_AUDIT_EVENT_TYPE_UNION {}
impl ::core::clone::Clone for AUTHZ_AUDIT_EVENT_TYPE_UNION {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows_core::Abi for AUTHZ_AUDIT_EVENT_TYPE_UNION {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for AUTHZ_AUDIT_EVENT_TYPE_UNION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<AUTHZ_AUDIT_EVENT_TYPE_UNION>()) == 0 }
    }
}
impl ::core::cmp::Eq for AUTHZ_AUDIT_EVENT_TYPE_UNION {}
impl ::core::default::Default for AUTHZ_AUDIT_EVENT_TYPE_UNION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const AUTHZ_AUDIT_INSTANCE_INFORMATION: u32 = 2u32;
#[repr(C)]
pub struct AUTHZ_CAP_CHANGE_SUBSCRIPTION_HANDLE__ {
    pub unused: i32,
}
impl ::core::marker::Copy for AUTHZ_CAP_CHANGE_SUBSCRIPTION_HANDLE__ {}
impl ::core::clone::Clone for AUTHZ_CAP_CHANGE_SUBSCRIPTION_HANDLE__ {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for AUTHZ_CAP_CHANGE_SUBSCRIPTION_HANDLE__ {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("AUTHZ_CAP_CHANGE_SUBSCRIPTION_HANDLE__").field("unused", &self.unused).finish()
    }
}
unsafe impl ::windows_core::Abi for AUTHZ_CAP_CHANGE_SUBSCRIPTION_HANDLE__ {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for AUTHZ_CAP_CHANGE_SUBSCRIPTION_HANDLE__ {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<AUTHZ_CAP_CHANGE_SUBSCRIPTION_HANDLE__>()) == 0 }
    }
}
impl ::core::cmp::Eq for AUTHZ_CAP_CHANGE_SUBSCRIPTION_HANDLE__ {}
impl ::core::default::Default for AUTHZ_CAP_CHANGE_SUBSCRIPTION_HANDLE__ {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct AUTHZ_CLIENT_CONTEXT_HANDLE(pub isize);
impl AUTHZ_CLIENT_CONTEXT_HANDLE {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 || self.0 == 0
    }
}
impl ::core::default::Default for AUTHZ_CLIENT_CONTEXT_HANDLE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for AUTHZ_CLIENT_CONTEXT_HANDLE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for AUTHZ_CLIENT_CONTEXT_HANDLE {}
impl ::core::fmt::Debug for AUTHZ_CLIENT_CONTEXT_HANDLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AUTHZ_CLIENT_CONTEXT_HANDLE").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Abi for AUTHZ_CLIENT_CONTEXT_HANDLE {
    type Abi = Self;
}
pub const AUTHZ_COMPUTE_PRIVILEGES: u32 = 8u32;
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct AUTHZ_CONTEXT_INFORMATION_CLASS(pub i32);
pub const AuthzContextInfoUserSid: AUTHZ_CONTEXT_INFORMATION_CLASS = AUTHZ_CONTEXT_INFORMATION_CLASS(1i32);
pub const AuthzContextInfoGroupsSids: AUTHZ_CONTEXT_INFORMATION_CLASS = AUTHZ_CONTEXT_INFORMATION_CLASS(2i32);
pub const AuthzContextInfoRestrictedSids: AUTHZ_CONTEXT_INFORMATION_CLASS = AUTHZ_CONTEXT_INFORMATION_CLASS(3i32);
pub const AuthzContextInfoPrivileges: AUTHZ_CONTEXT_INFORMATION_CLASS = AUTHZ_CONTEXT_INFORMATION_CLASS(4i32);
pub const AuthzContextInfoExpirationTime: AUTHZ_CONTEXT_INFORMATION_CLASS = AUTHZ_CONTEXT_INFORMATION_CLASS(5i32);
pub const AuthzContextInfoServerContext: AUTHZ_CONTEXT_INFORMATION_CLASS = AUTHZ_CONTEXT_INFORMATION_CLASS(6i32);
pub const AuthzContextInfoIdentifier: AUTHZ_CONTEXT_INFORMATION_CLASS = AUTHZ_CONTEXT_INFORMATION_CLASS(7i32);
pub const AuthzContextInfoSource: AUTHZ_CONTEXT_INFORMATION_CLASS = AUTHZ_CONTEXT_INFORMATION_CLASS(8i32);
pub const AuthzContextInfoAll: AUTHZ_CONTEXT_INFORMATION_CLASS = AUTHZ_CONTEXT_INFORMATION_CLASS(9i32);
pub const AuthzContextInfoAuthenticationId: AUTHZ_CONTEXT_INFORMATION_CLASS = AUTHZ_CONTEXT_INFORMATION_CLASS(10i32);
pub const AuthzContextInfoSecurityAttributes: AUTHZ_CONTEXT_INFORMATION_CLASS = AUTHZ_CONTEXT_INFORMATION_CLASS(11i32);
pub const AuthzContextInfoDeviceSids: AUTHZ_CONTEXT_INFORMATION_CLASS = AUTHZ_CONTEXT_INFORMATION_CLASS(12i32);
pub const AuthzContextInfoUserClaims: AUTHZ_CONTEXT_INFORMATION_CLASS = AUTHZ_CONTEXT_INFORMATION_CLASS(13i32);
pub const AuthzContextInfoDeviceClaims: AUTHZ_CONTEXT_INFORMATION_CLASS = AUTHZ_CONTEXT_INFORMATION_CLASS(14i32);
pub const AuthzContextInfoAppContainerSid: AUTHZ_CONTEXT_INFORMATION_CLASS = AUTHZ_CONTEXT_INFORMATION_CLASS(15i32);
pub const AuthzContextInfoCapabilitySids: AUTHZ_CONTEXT_INFORMATION_CLASS = AUTHZ_CONTEXT_INFORMATION_CLASS(16i32);
impl ::core::marker::Copy for AUTHZ_CONTEXT_INFORMATION_CLASS {}
impl ::core::clone::Clone for AUTHZ_CONTEXT_INFORMATION_CLASS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AUTHZ_CONTEXT_INFORMATION_CLASS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for AUTHZ_CONTEXT_INFORMATION_CLASS {
    type Abi = Self;
}
impl ::core::fmt::Debug for AUTHZ_CONTEXT_INFORMATION_CLASS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AUTHZ_CONTEXT_INFORMATION_CLASS").field(&self.0).finish()
    }
}
pub const AUTHZ_FLAG_ALLOW_MULTIPLE_SOURCE_INSTANCES: u32 = 1u32;
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct AUTHZ_GENERATE_RESULTS(pub u32);
pub const AUTHZ_GENERATE_SUCCESS_AUDIT: AUTHZ_GENERATE_RESULTS = AUTHZ_GENERATE_RESULTS(1u32);
pub const AUTHZ_GENERATE_FAILURE_AUDIT: AUTHZ_GENERATE_RESULTS = AUTHZ_GENERATE_RESULTS(2u32);
impl ::core::marker::Copy for AUTHZ_GENERATE_RESULTS {}
impl ::core::clone::Clone for AUTHZ_GENERATE_RESULTS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AUTHZ_GENERATE_RESULTS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for AUTHZ_GENERATE_RESULTS {
    type Abi = Self;
}
impl ::core::fmt::Debug for AUTHZ_GENERATE_RESULTS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AUTHZ_GENERATE_RESULTS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct AUTHZ_INITIALIZE_OBJECT_ACCESS_AUDIT_EVENT_FLAGS(pub u32);
pub const AUTHZ_NO_SUCCESS_AUDIT: AUTHZ_INITIALIZE_OBJECT_ACCESS_AUDIT_EVENT_FLAGS = AUTHZ_INITIALIZE_OBJECT_ACCESS_AUDIT_EVENT_FLAGS(1u32);
pub const AUTHZ_NO_FAILURE_AUDIT: AUTHZ_INITIALIZE_OBJECT_ACCESS_AUDIT_EVENT_FLAGS = AUTHZ_INITIALIZE_OBJECT_ACCESS_AUDIT_EVENT_FLAGS(2u32);
pub const AUTHZ_NO_ALLOC_STRINGS: AUTHZ_INITIALIZE_OBJECT_ACCESS_AUDIT_EVENT_FLAGS = AUTHZ_INITIALIZE_OBJECT_ACCESS_AUDIT_EVENT_FLAGS(4u32);
impl ::core::marker::Copy for AUTHZ_INITIALIZE_OBJECT_ACCESS_AUDIT_EVENT_FLAGS {}
impl ::core::clone::Clone for AUTHZ_INITIALIZE_OBJECT_ACCESS_AUDIT_EVENT_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AUTHZ_INITIALIZE_OBJECT_ACCESS_AUDIT_EVENT_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for AUTHZ_INITIALIZE_OBJECT_ACCESS_AUDIT_EVENT_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for AUTHZ_INITIALIZE_OBJECT_ACCESS_AUDIT_EVENT_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AUTHZ_INITIALIZE_OBJECT_ACCESS_AUDIT_EVENT_FLAGS").field(&self.0).finish()
    }
}
#[repr(C)]
pub struct AUTHZ_INIT_INFO {
    pub version: u16,
    pub szResourceManagerName: ::windows_core::PCWSTR,
    pub pfnDynamicAccessCheck: PFN_AUTHZ_DYNAMIC_ACCESS_CHECK,
    pub pfnComputeDynamicGroups: PFN_AUTHZ_COMPUTE_DYNAMIC_GROUPS,
    pub pfnFreeDynamicGroups: PFN_AUTHZ_FREE_DYNAMIC_GROUPS,
    pub pfnGetCentralAccessPolicy: PFN_AUTHZ_GET_CENTRAL_ACCESS_POLICY,
    pub pfnFreeCentralAccessPolicy: PFN_AUTHZ_FREE_CENTRAL_ACCESS_POLICY,
}
impl ::core::marker::Copy for AUTHZ_INIT_INFO {}
impl ::core::clone::Clone for AUTHZ_INIT_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for AUTHZ_INIT_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("AUTHZ_INIT_INFO")
            .field("version", &self.version)
            .field("szResourceManagerName", &self.szResourceManagerName)
            .field("pfnDynamicAccessCheck", &self.pfnDynamicAccessCheck.map(|f| f as usize))
            .field("pfnComputeDynamicGroups", &self.pfnComputeDynamicGroups.map(|f| f as usize))
            .field("pfnFreeDynamicGroups", &self.pfnFreeDynamicGroups.map(|f| f as usize))
            .field("pfnGetCentralAccessPolicy", &self.pfnGetCentralAccessPolicy.map(|f| f as usize))
            .field("pfnFreeCentralAccessPolicy", &self.pfnFreeCentralAccessPolicy.map(|f| f as usize))
            .finish()
    }
}
unsafe impl ::windows_core::Abi for AUTHZ_INIT_INFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for AUTHZ_INIT_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<AUTHZ_INIT_INFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for AUTHZ_INIT_INFO {}
impl ::core::default::Default for AUTHZ_INIT_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const AUTHZ_INIT_INFO_VERSION_V1: u32 = 1u32;
pub const AUTHZ_MIGRATED_LEGACY_PUBLISHER: u32 = 2u32;
#[repr(C)]
pub struct AUTHZ_REGISTRATION_OBJECT_TYPE_NAME_OFFSET {
    pub szObjectTypeName: ::windows_core::PWSTR,
    pub dwOffset: u32,
}
impl ::core::marker::Copy for AUTHZ_REGISTRATION_OBJECT_TYPE_NAME_OFFSET {}
impl ::core::clone::Clone for AUTHZ_REGISTRATION_OBJECT_TYPE_NAME_OFFSET {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for AUTHZ_REGISTRATION_OBJECT_TYPE_NAME_OFFSET {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("AUTHZ_REGISTRATION_OBJECT_TYPE_NAME_OFFSET").field("szObjectTypeName", &self.szObjectTypeName).field("dwOffset", &self.dwOffset).finish()
    }
}
unsafe impl ::windows_core::Abi for AUTHZ_REGISTRATION_OBJECT_TYPE_NAME_OFFSET {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for AUTHZ_REGISTRATION_OBJECT_TYPE_NAME_OFFSET {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<AUTHZ_REGISTRATION_OBJECT_TYPE_NAME_OFFSET>()) == 0 }
    }
}
impl ::core::cmp::Eq for AUTHZ_REGISTRATION_OBJECT_TYPE_NAME_OFFSET {}
impl ::core::default::Default for AUTHZ_REGISTRATION_OBJECT_TYPE_NAME_OFFSET {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const AUTHZ_REQUIRE_S4U_LOGON: u32 = 4u32;
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct AUTHZ_RESOURCE_MANAGER_FLAGS(pub u32);
pub const AUTHZ_RM_FLAG_NO_AUDIT: AUTHZ_RESOURCE_MANAGER_FLAGS = AUTHZ_RESOURCE_MANAGER_FLAGS(1u32);
pub const AUTHZ_RM_FLAG_INITIALIZE_UNDER_IMPERSONATION: AUTHZ_RESOURCE_MANAGER_FLAGS = AUTHZ_RESOURCE_MANAGER_FLAGS(2u32);
pub const AUTHZ_RM_FLAG_NO_CENTRAL_ACCESS_POLICIES: AUTHZ_RESOURCE_MANAGER_FLAGS = AUTHZ_RESOURCE_MANAGER_FLAGS(4u32);
impl ::core::marker::Copy for AUTHZ_RESOURCE_MANAGER_FLAGS {}
impl ::core::clone::Clone for AUTHZ_RESOURCE_MANAGER_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AUTHZ_RESOURCE_MANAGER_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for AUTHZ_RESOURCE_MANAGER_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for AUTHZ_RESOURCE_MANAGER_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AUTHZ_RESOURCE_MANAGER_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for AUTHZ_RESOURCE_MANAGER_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for AUTHZ_RESOURCE_MANAGER_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for AUTHZ_RESOURCE_MANAGER_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for AUTHZ_RESOURCE_MANAGER_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for AUTHZ_RESOURCE_MANAGER_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct AUTHZ_RESOURCE_MANAGER_HANDLE(pub isize);
impl AUTHZ_RESOURCE_MANAGER_HANDLE {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 || self.0 == 0
    }
}
impl ::core::default::Default for AUTHZ_RESOURCE_MANAGER_HANDLE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for AUTHZ_RESOURCE_MANAGER_HANDLE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for AUTHZ_RESOURCE_MANAGER_HANDLE {}
impl ::core::fmt::Debug for AUTHZ_RESOURCE_MANAGER_HANDLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AUTHZ_RESOURCE_MANAGER_HANDLE").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Abi for AUTHZ_RESOURCE_MANAGER_HANDLE {
    type Abi = Self;
}
#[repr(C)]
pub struct AUTHZ_RPC_INIT_INFO_CLIENT {
    pub version: u16,
    pub ObjectUuid: ::windows_core::PWSTR,
    pub ProtSeq: ::windows_core::PWSTR,
    pub NetworkAddr: ::windows_core::PWSTR,
    pub Endpoint: ::windows_core::PWSTR,
    pub Options: ::windows_core::PWSTR,
    pub ServerSpn: ::windows_core::PWSTR,
}
impl ::core::marker::Copy for AUTHZ_RPC_INIT_INFO_CLIENT {}
impl ::core::clone::Clone for AUTHZ_RPC_INIT_INFO_CLIENT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for AUTHZ_RPC_INIT_INFO_CLIENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("AUTHZ_RPC_INIT_INFO_CLIENT").field("version", &self.version).field("ObjectUuid", &self.ObjectUuid).field("ProtSeq", &self.ProtSeq).field("NetworkAddr", &self.NetworkAddr).field("Endpoint", &self.Endpoint).field("Options", &self.Options).field("ServerSpn", &self.ServerSpn).finish()
    }
}
unsafe impl ::windows_core::Abi for AUTHZ_RPC_INIT_INFO_CLIENT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for AUTHZ_RPC_INIT_INFO_CLIENT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<AUTHZ_RPC_INIT_INFO_CLIENT>()) == 0 }
    }
}
impl ::core::cmp::Eq for AUTHZ_RPC_INIT_INFO_CLIENT {}
impl ::core::default::Default for AUTHZ_RPC_INIT_INFO_CLIENT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const AUTHZ_RPC_INIT_INFO_CLIENT_VERSION_V1: u32 = 1u32;
#[repr(C)]
pub struct AUTHZ_SECURITY_ATTRIBUTES_INFORMATION {
    pub Version: u16,
    pub Reserved: u16,
    pub AttributeCount: u32,
    pub Attribute: AUTHZ_SECURITY_ATTRIBUTES_INFORMATION_0,
}
impl ::core::marker::Copy for AUTHZ_SECURITY_ATTRIBUTES_INFORMATION {}
impl ::core::clone::Clone for AUTHZ_SECURITY_ATTRIBUTES_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows_core::Abi for AUTHZ_SECURITY_ATTRIBUTES_INFORMATION {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for AUTHZ_SECURITY_ATTRIBUTES_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<AUTHZ_SECURITY_ATTRIBUTES_INFORMATION>()) == 0 }
    }
}
impl ::core::cmp::Eq for AUTHZ_SECURITY_ATTRIBUTES_INFORMATION {}
impl ::core::default::Default for AUTHZ_SECURITY_ATTRIBUTES_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub union AUTHZ_SECURITY_ATTRIBUTES_INFORMATION_0 {
    pub pAttributeV1: *mut AUTHZ_SECURITY_ATTRIBUTE_V1,
}
impl ::core::marker::Copy for AUTHZ_SECURITY_ATTRIBUTES_INFORMATION_0 {}
impl ::core::clone::Clone for AUTHZ_SECURITY_ATTRIBUTES_INFORMATION_0 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows_core::Abi for AUTHZ_SECURITY_ATTRIBUTES_INFORMATION_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for AUTHZ_SECURITY_ATTRIBUTES_INFORMATION_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<AUTHZ_SECURITY_ATTRIBUTES_INFORMATION_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for AUTHZ_SECURITY_ATTRIBUTES_INFORMATION_0 {}
impl ::core::default::Default for AUTHZ_SECURITY_ATTRIBUTES_INFORMATION_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const AUTHZ_SECURITY_ATTRIBUTES_INFORMATION_VERSION: u32 = 1u32;
pub const AUTHZ_SECURITY_ATTRIBUTES_INFORMATION_VERSION_V1: u32 = 1u32;
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct AUTHZ_SECURITY_ATTRIBUTE_FLAGS(pub u32);
pub const AUTHZ_SECURITY_ATTRIBUTE_NON_INHERITABLE: AUTHZ_SECURITY_ATTRIBUTE_FLAGS = AUTHZ_SECURITY_ATTRIBUTE_FLAGS(1u32);
pub const AUTHZ_SECURITY_ATTRIBUTE_VALUE_CASE_SENSITIVE: AUTHZ_SECURITY_ATTRIBUTE_FLAGS = AUTHZ_SECURITY_ATTRIBUTE_FLAGS(2u32);
impl ::core::marker::Copy for AUTHZ_SECURITY_ATTRIBUTE_FLAGS {}
impl ::core::clone::Clone for AUTHZ_SECURITY_ATTRIBUTE_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AUTHZ_SECURITY_ATTRIBUTE_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for AUTHZ_SECURITY_ATTRIBUTE_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for AUTHZ_SECURITY_ATTRIBUTE_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AUTHZ_SECURITY_ATTRIBUTE_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for AUTHZ_SECURITY_ATTRIBUTE_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for AUTHZ_SECURITY_ATTRIBUTE_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for AUTHZ_SECURITY_ATTRIBUTE_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for AUTHZ_SECURITY_ATTRIBUTE_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for AUTHZ_SECURITY_ATTRIBUTE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(C)]
pub struct AUTHZ_SECURITY_ATTRIBUTE_FQBN_VALUE {
    pub Version: u64,
    pub pName: ::windows_core::PWSTR,
}
impl ::core::marker::Copy for AUTHZ_SECURITY_ATTRIBUTE_FQBN_VALUE {}
impl ::core::clone::Clone for AUTHZ_SECURITY_ATTRIBUTE_FQBN_VALUE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for AUTHZ_SECURITY_ATTRIBUTE_FQBN_VALUE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("AUTHZ_SECURITY_ATTRIBUTE_FQBN_VALUE").field("Version", &self.Version).field("pName", &self.pName).finish()
    }
}
unsafe impl ::windows_core::Abi for AUTHZ_SECURITY_ATTRIBUTE_FQBN_VALUE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for AUTHZ_SECURITY_ATTRIBUTE_FQBN_VALUE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<AUTHZ_SECURITY_ATTRIBUTE_FQBN_VALUE>()) == 0 }
    }
}
impl ::core::cmp::Eq for AUTHZ_SECURITY_ATTRIBUTE_FQBN_VALUE {}
impl ::core::default::Default for AUTHZ_SECURITY_ATTRIBUTE_FQBN_VALUE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct AUTHZ_SECURITY_ATTRIBUTE_OCTET_STRING_VALUE {
    pub pValue: *mut ::core::ffi::c_void,
    pub ValueLength: u32,
}
impl ::core::marker::Copy for AUTHZ_SECURITY_ATTRIBUTE_OCTET_STRING_VALUE {}
impl ::core::clone::Clone for AUTHZ_SECURITY_ATTRIBUTE_OCTET_STRING_VALUE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for AUTHZ_SECURITY_ATTRIBUTE_OCTET_STRING_VALUE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("AUTHZ_SECURITY_ATTRIBUTE_OCTET_STRING_VALUE").field("pValue", &self.pValue).field("ValueLength", &self.ValueLength).finish()
    }
}
unsafe impl ::windows_core::Abi for AUTHZ_SECURITY_ATTRIBUTE_OCTET_STRING_VALUE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for AUTHZ_SECURITY_ATTRIBUTE_OCTET_STRING_VALUE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<AUTHZ_SECURITY_ATTRIBUTE_OCTET_STRING_VALUE>()) == 0 }
    }
}
impl ::core::cmp::Eq for AUTHZ_SECURITY_ATTRIBUTE_OCTET_STRING_VALUE {}
impl ::core::default::Default for AUTHZ_SECURITY_ATTRIBUTE_OCTET_STRING_VALUE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct AUTHZ_SECURITY_ATTRIBUTE_OPERATION(pub i32);
pub const AUTHZ_SECURITY_ATTRIBUTE_OPERATION_NONE: AUTHZ_SECURITY_ATTRIBUTE_OPERATION = AUTHZ_SECURITY_ATTRIBUTE_OPERATION(0i32);
pub const AUTHZ_SECURITY_ATTRIBUTE_OPERATION_REPLACE_ALL: AUTHZ_SECURITY_ATTRIBUTE_OPERATION = AUTHZ_SECURITY_ATTRIBUTE_OPERATION(1i32);
pub const AUTHZ_SECURITY_ATTRIBUTE_OPERATION_ADD: AUTHZ_SECURITY_ATTRIBUTE_OPERATION = AUTHZ_SECURITY_ATTRIBUTE_OPERATION(2i32);
pub const AUTHZ_SECURITY_ATTRIBUTE_OPERATION_DELETE: AUTHZ_SECURITY_ATTRIBUTE_OPERATION = AUTHZ_SECURITY_ATTRIBUTE_OPERATION(3i32);
pub const AUTHZ_SECURITY_ATTRIBUTE_OPERATION_REPLACE: AUTHZ_SECURITY_ATTRIBUTE_OPERATION = AUTHZ_SECURITY_ATTRIBUTE_OPERATION(4i32);
impl ::core::marker::Copy for AUTHZ_SECURITY_ATTRIBUTE_OPERATION {}
impl ::core::clone::Clone for AUTHZ_SECURITY_ATTRIBUTE_OPERATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AUTHZ_SECURITY_ATTRIBUTE_OPERATION {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for AUTHZ_SECURITY_ATTRIBUTE_OPERATION {
    type Abi = Self;
}
impl ::core::fmt::Debug for AUTHZ_SECURITY_ATTRIBUTE_OPERATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AUTHZ_SECURITY_ATTRIBUTE_OPERATION").field(&self.0).finish()
    }
}
pub const AUTHZ_SECURITY_ATTRIBUTE_TYPE_BOOLEAN: u32 = 6u32;
pub const AUTHZ_SECURITY_ATTRIBUTE_TYPE_FQBN: u32 = 4u32;
pub const AUTHZ_SECURITY_ATTRIBUTE_TYPE_INT64: u32 = 1u32;
pub const AUTHZ_SECURITY_ATTRIBUTE_TYPE_INVALID: u32 = 0u32;
pub const AUTHZ_SECURITY_ATTRIBUTE_TYPE_OCTET_STRING: u32 = 16u32;
pub const AUTHZ_SECURITY_ATTRIBUTE_TYPE_SID: u32 = 5u32;
pub const AUTHZ_SECURITY_ATTRIBUTE_TYPE_STRING: u32 = 3u32;
pub const AUTHZ_SECURITY_ATTRIBUTE_TYPE_UINT64: u32 = 2u32;
#[repr(C)]
pub struct AUTHZ_SECURITY_ATTRIBUTE_V1 {
    pub pName: ::windows_core::PWSTR,
    pub ValueType: u16,
    pub Reserved: u16,
    pub Flags: AUTHZ_SECURITY_ATTRIBUTE_FLAGS,
    pub ValueCount: u32,
    pub Values: AUTHZ_SECURITY_ATTRIBUTE_V1_0,
}
impl ::core::marker::Copy for AUTHZ_SECURITY_ATTRIBUTE_V1 {}
impl ::core::clone::Clone for AUTHZ_SECURITY_ATTRIBUTE_V1 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows_core::Abi for AUTHZ_SECURITY_ATTRIBUTE_V1 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for AUTHZ_SECURITY_ATTRIBUTE_V1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<AUTHZ_SECURITY_ATTRIBUTE_V1>()) == 0 }
    }
}
impl ::core::cmp::Eq for AUTHZ_SECURITY_ATTRIBUTE_V1 {}
impl ::core::default::Default for AUTHZ_SECURITY_ATTRIBUTE_V1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub union AUTHZ_SECURITY_ATTRIBUTE_V1_0 {
    pub pInt64: *mut i64,
    pub pUint64: *mut u64,
    pub ppString: *mut ::windows_core::PWSTR,
    pub pFqbn: *mut AUTHZ_SECURITY_ATTRIBUTE_FQBN_VALUE,
    pub pOctetString: *mut AUTHZ_SECURITY_ATTRIBUTE_OCTET_STRING_VALUE,
}
impl ::core::marker::Copy for AUTHZ_SECURITY_ATTRIBUTE_V1_0 {}
impl ::core::clone::Clone for AUTHZ_SECURITY_ATTRIBUTE_V1_0 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows_core::Abi for AUTHZ_SECURITY_ATTRIBUTE_V1_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for AUTHZ_SECURITY_ATTRIBUTE_V1_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<AUTHZ_SECURITY_ATTRIBUTE_V1_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for AUTHZ_SECURITY_ATTRIBUTE_V1_0 {}
impl ::core::default::Default for AUTHZ_SECURITY_ATTRIBUTE_V1_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct AUTHZ_SECURITY_EVENT_PROVIDER_HANDLE(pub isize);
impl AUTHZ_SECURITY_EVENT_PROVIDER_HANDLE {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 || self.0 == 0
    }
}
impl ::core::default::Default for AUTHZ_SECURITY_EVENT_PROVIDER_HANDLE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for AUTHZ_SECURITY_EVENT_PROVIDER_HANDLE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for AUTHZ_SECURITY_EVENT_PROVIDER_HANDLE {}
impl ::core::fmt::Debug for AUTHZ_SECURITY_EVENT_PROVIDER_HANDLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AUTHZ_SECURITY_EVENT_PROVIDER_HANDLE").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Abi for AUTHZ_SECURITY_EVENT_PROVIDER_HANDLE {
    type Abi = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct AUTHZ_SID_OPERATION(pub i32);
pub const AUTHZ_SID_OPERATION_NONE: AUTHZ_SID_OPERATION = AUTHZ_SID_OPERATION(0i32);
pub const AUTHZ_SID_OPERATION_REPLACE_ALL: AUTHZ_SID_OPERATION = AUTHZ_SID_OPERATION(1i32);
pub const AUTHZ_SID_OPERATION_ADD: AUTHZ_SID_OPERATION = AUTHZ_SID_OPERATION(2i32);
pub const AUTHZ_SID_OPERATION_DELETE: AUTHZ_SID_OPERATION = AUTHZ_SID_OPERATION(3i32);
pub const AUTHZ_SID_OPERATION_REPLACE: AUTHZ_SID_OPERATION = AUTHZ_SID_OPERATION(4i32);
impl ::core::marker::Copy for AUTHZ_SID_OPERATION {}
impl ::core::clone::Clone for AUTHZ_SID_OPERATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AUTHZ_SID_OPERATION {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for AUTHZ_SID_OPERATION {
    type Abi = Self;
}
impl ::core::fmt::Debug for AUTHZ_SID_OPERATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AUTHZ_SID_OPERATION").field(&self.0).finish()
    }
}
pub const AUTHZ_SKIP_TOKEN_GROUPS: u32 = 2u32;
#[repr(C)]
pub struct AUTHZ_SOURCE_SCHEMA_REGISTRATION {
    pub dwFlags: u32,
    pub szEventSourceName: ::windows_core::PWSTR,
    pub szEventMessageFile: ::windows_core::PWSTR,
    pub szEventSourceXmlSchemaFile: ::windows_core::PWSTR,
    pub szEventAccessStringsFile: ::windows_core::PWSTR,
    pub szExecutableImagePath: ::windows_core::PWSTR,
    pub Anonymous: AUTHZ_SOURCE_SCHEMA_REGISTRATION_0,
    pub dwObjectTypeNameCount: u32,
    pub ObjectTypeNames: [AUTHZ_REGISTRATION_OBJECT_TYPE_NAME_OFFSET; 1],
}
impl ::core::marker::Copy for AUTHZ_SOURCE_SCHEMA_REGISTRATION {}
impl ::core::clone::Clone for AUTHZ_SOURCE_SCHEMA_REGISTRATION {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows_core::Abi for AUTHZ_SOURCE_SCHEMA_REGISTRATION {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for AUTHZ_SOURCE_SCHEMA_REGISTRATION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<AUTHZ_SOURCE_SCHEMA_REGISTRATION>()) == 0 }
    }
}
impl ::core::cmp::Eq for AUTHZ_SOURCE_SCHEMA_REGISTRATION {}
impl ::core::default::Default for AUTHZ_SOURCE_SCHEMA_REGISTRATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub union AUTHZ_SOURCE_SCHEMA_REGISTRATION_0 {
    pub pReserved: *mut ::core::ffi::c_void,
    pub pProviderGuid: *mut ::windows_core::GUID,
}
impl ::core::marker::Copy for AUTHZ_SOURCE_SCHEMA_REGISTRATION_0 {}
impl ::core::clone::Clone for AUTHZ_SOURCE_SCHEMA_REGISTRATION_0 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows_core::Abi for AUTHZ_SOURCE_SCHEMA_REGISTRATION_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for AUTHZ_SOURCE_SCHEMA_REGISTRATION_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<AUTHZ_SOURCE_SCHEMA_REGISTRATION_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for AUTHZ_SOURCE_SCHEMA_REGISTRATION_0 {}
impl ::core::default::Default for AUTHZ_SOURCE_SCHEMA_REGISTRATION_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const AUTHZ_WPD_CATEGORY_FLAG: u32 = 16u32;
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct AZ_PROP_CONSTANTS(pub i32);
pub const AZ_PROP_NAME: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(1i32);
pub const AZ_PROP_DESCRIPTION: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(2i32);
pub const AZ_PROP_WRITABLE: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(3i32);
pub const AZ_PROP_APPLICATION_DATA: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(4i32);
pub const AZ_PROP_CHILD_CREATE: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(5i32);
pub const AZ_MAX_APPLICATION_NAME_LENGTH: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(512i32);
pub const AZ_MAX_OPERATION_NAME_LENGTH: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(64i32);
pub const AZ_MAX_TASK_NAME_LENGTH: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(64i32);
pub const AZ_MAX_SCOPE_NAME_LENGTH: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(65536i32);
pub const AZ_MAX_GROUP_NAME_LENGTH: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(64i32);
pub const AZ_MAX_ROLE_NAME_LENGTH: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(64i32);
pub const AZ_MAX_NAME_LENGTH: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(65536i32);
pub const AZ_MAX_DESCRIPTION_LENGTH: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(1024i32);
pub const AZ_MAX_APPLICATION_DATA_LENGTH: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(4096i32);
pub const AZ_SUBMIT_FLAG_ABORT: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(1i32);
pub const AZ_SUBMIT_FLAG_FLUSH: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(2i32);
pub const AZ_MAX_POLICY_URL_LENGTH: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(65536i32);
pub const AZ_AZSTORE_FLAG_CREATE: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(1i32);
pub const AZ_AZSTORE_FLAG_MANAGE_STORE_ONLY: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(2i32);
pub const AZ_AZSTORE_FLAG_BATCH_UPDATE: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(4i32);
pub const AZ_AZSTORE_FLAG_AUDIT_IS_CRITICAL: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(8i32);
pub const AZ_AZSTORE_FORCE_APPLICATION_CLOSE: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(16i32);
pub const AZ_AZSTORE_NT6_FUNCTION_LEVEL: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(32i32);
pub const AZ_AZSTORE_FLAG_MANAGE_ONLY_PASSIVE_SUBMIT: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(32768i32);
pub const AZ_PROP_AZSTORE_DOMAIN_TIMEOUT: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(100i32);
pub const AZ_AZSTORE_DEFAULT_DOMAIN_TIMEOUT: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(15000i32);
pub const AZ_PROP_AZSTORE_SCRIPT_ENGINE_TIMEOUT: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(101i32);
pub const AZ_AZSTORE_MIN_DOMAIN_TIMEOUT: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(500i32);
pub const AZ_AZSTORE_MIN_SCRIPT_ENGINE_TIMEOUT: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(5000i32);
pub const AZ_AZSTORE_DEFAULT_SCRIPT_ENGINE_TIMEOUT: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(45000i32);
pub const AZ_PROP_AZSTORE_MAX_SCRIPT_ENGINES: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(102i32);
pub const AZ_AZSTORE_DEFAULT_MAX_SCRIPT_ENGINES: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(120i32);
pub const AZ_PROP_AZSTORE_MAJOR_VERSION: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(103i32);
pub const AZ_PROP_AZSTORE_MINOR_VERSION: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(104i32);
pub const AZ_PROP_AZSTORE_TARGET_MACHINE: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(105i32);
pub const AZ_PROP_AZTORE_IS_ADAM_INSTANCE: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(106i32);
pub const AZ_PROP_OPERATION_ID: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(200i32);
pub const AZ_PROP_TASK_OPERATIONS: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(300i32);
pub const AZ_PROP_TASK_BIZRULE: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(301i32);
pub const AZ_PROP_TASK_BIZRULE_LANGUAGE: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(302i32);
pub const AZ_PROP_TASK_TASKS: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(303i32);
pub const AZ_PROP_TASK_BIZRULE_IMPORTED_PATH: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(304i32);
pub const AZ_PROP_TASK_IS_ROLE_DEFINITION: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(305i32);
pub const AZ_MAX_TASK_BIZRULE_LENGTH: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(65536i32);
pub const AZ_MAX_TASK_BIZRULE_LANGUAGE_LENGTH: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(64i32);
pub const AZ_MAX_TASK_BIZRULE_IMPORTED_PATH_LENGTH: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(512i32);
pub const AZ_MAX_BIZRULE_STRING: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(65536i32);
pub const AZ_PROP_GROUP_TYPE: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(400i32);
pub const AZ_GROUPTYPE_LDAP_QUERY: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(1i32);
pub const AZ_GROUPTYPE_BASIC: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(2i32);
pub const AZ_GROUPTYPE_BIZRULE: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(3i32);
pub const AZ_PROP_GROUP_APP_MEMBERS: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(401i32);
pub const AZ_PROP_GROUP_APP_NON_MEMBERS: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(402i32);
pub const AZ_PROP_GROUP_LDAP_QUERY: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(403i32);
pub const AZ_MAX_GROUP_LDAP_QUERY_LENGTH: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(4096i32);
pub const AZ_PROP_GROUP_MEMBERS: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(404i32);
pub const AZ_PROP_GROUP_NON_MEMBERS: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(405i32);
pub const AZ_PROP_GROUP_MEMBERS_NAME: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(406i32);
pub const AZ_PROP_GROUP_NON_MEMBERS_NAME: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(407i32);
pub const AZ_PROP_GROUP_BIZRULE: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(408i32);
pub const AZ_PROP_GROUP_BIZRULE_LANGUAGE: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(409i32);
pub const AZ_PROP_GROUP_BIZRULE_IMPORTED_PATH: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(410i32);
pub const AZ_MAX_GROUP_BIZRULE_LENGTH: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(65536i32);
pub const AZ_MAX_GROUP_BIZRULE_LANGUAGE_LENGTH: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(64i32);
pub const AZ_MAX_GROUP_BIZRULE_IMPORTED_PATH_LENGTH: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(512i32);
pub const AZ_PROP_ROLE_APP_MEMBERS: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(500i32);
pub const AZ_PROP_ROLE_MEMBERS: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(501i32);
pub const AZ_PROP_ROLE_OPERATIONS: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(502i32);
pub const AZ_PROP_ROLE_TASKS: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(504i32);
pub const AZ_PROP_ROLE_MEMBERS_NAME: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(505i32);
pub const AZ_PROP_SCOPE_BIZRULES_WRITABLE: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(600i32);
pub const AZ_PROP_SCOPE_CAN_BE_DELEGATED: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(601i32);
pub const AZ_PROP_CLIENT_CONTEXT_USER_DN: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(700i32);
pub const AZ_PROP_CLIENT_CONTEXT_USER_SAM_COMPAT: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(701i32);
pub const AZ_PROP_CLIENT_CONTEXT_USER_DISPLAY: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(702i32);
pub const AZ_PROP_CLIENT_CONTEXT_USER_GUID: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(703i32);
pub const AZ_PROP_CLIENT_CONTEXT_USER_CANONICAL: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(704i32);
pub const AZ_PROP_CLIENT_CONTEXT_USER_UPN: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(705i32);
pub const AZ_PROP_CLIENT_CONTEXT_USER_DNS_SAM_COMPAT: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(707i32);
pub const AZ_PROP_CLIENT_CONTEXT_ROLE_FOR_ACCESS_CHECK: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(708i32);
pub const AZ_PROP_CLIENT_CONTEXT_LDAP_QUERY_DN: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(709i32);
pub const AZ_PROP_APPLICATION_AUTHZ_INTERFACE_CLSID: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(800i32);
pub const AZ_PROP_APPLICATION_VERSION: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(801i32);
pub const AZ_MAX_APPLICATION_VERSION_LENGTH: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(512i32);
pub const AZ_PROP_APPLICATION_NAME: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(802i32);
pub const AZ_PROP_APPLICATION_BIZRULE_ENABLED: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(803i32);
pub const AZ_PROP_APPLY_STORE_SACL: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(900i32);
pub const AZ_PROP_GENERATE_AUDITS: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(901i32);
pub const AZ_PROP_POLICY_ADMINS: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(902i32);
pub const AZ_PROP_POLICY_READERS: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(903i32);
pub const AZ_PROP_DELEGATED_POLICY_USERS: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(904i32);
pub const AZ_PROP_POLICY_ADMINS_NAME: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(905i32);
pub const AZ_PROP_POLICY_READERS_NAME: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(906i32);
pub const AZ_PROP_DELEGATED_POLICY_USERS_NAME: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(907i32);
pub const AZ_CLIENT_CONTEXT_SKIP_GROUP: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(1i32);
pub const AZ_CLIENT_CONTEXT_SKIP_LDAP_QUERY: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(1i32);
pub const AZ_CLIENT_CONTEXT_GET_GROUP_RECURSIVE: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(2i32);
pub const AZ_CLIENT_CONTEXT_GET_GROUPS_STORE_LEVEL_ONLY: AZ_PROP_CONSTANTS = AZ_PROP_CONSTANTS(2i32);
impl ::core::marker::Copy for AZ_PROP_CONSTANTS {}
impl ::core::clone::Clone for AZ_PROP_CONSTANTS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AZ_PROP_CONSTANTS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for AZ_PROP_CONSTANTS {
    type Abi = Self;
}
impl ::core::fmt::Debug for AZ_PROP_CONSTANTS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AZ_PROP_CONSTANTS").field(&self.0).finish()
    }
}
#[inline]
pub unsafe fn AuthzAccessCheck<'a, Param1: ::windows_core::IntoParam<'a, AUTHZ_CLIENT_CONTEXT_HANDLE>, Param3: ::windows_core::IntoParam<'a, AUTHZ_AUDIT_EVENT_HANDLE>, Param4: ::windows_core::IntoParam<'a, super::PSECURITY_DESCRIPTOR>>(flags: AUTHZ_ACCESS_CHECK_FLAGS, hauthzclientcontext: Param1, prequest: *const AUTHZ_ACCESS_REQUEST, hauditevent: Param3, psecuritydescriptor: Param4, optionalsecuritydescriptorarray: &[super::PSECURITY_DESCRIPTOR], preply: *mut AUTHZ_ACCESS_REPLY, phaccesscheckresults: *mut isize) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AuthzAccessCheck(flags: AUTHZ_ACCESS_CHECK_FLAGS, hauthzclientcontext: AUTHZ_CLIENT_CONTEXT_HANDLE, prequest: *const AUTHZ_ACCESS_REQUEST, hauditevent: AUTHZ_AUDIT_EVENT_HANDLE, psecuritydescriptor: super::PSECURITY_DESCRIPTOR, optionalsecuritydescriptorarray: *const super::PSECURITY_DESCRIPTOR, optionalsecuritydescriptorcount: u32, preply: *mut AUTHZ_ACCESS_REPLY, phaccesscheckresults: *mut isize) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(AuthzAccessCheck(::core::mem::transmute(flags), hauthzclientcontext.into_param().abi(), ::core::mem::transmute(prequest), hauditevent.into_param().abi(), psecuritydescriptor.into_param().abi(), ::core::mem::transmute(::windows_core::as_ptr_or_null(optionalsecuritydescriptorarray)), optionalsecuritydescriptorarray.len() as _, ::core::mem::transmute(preply), ::core::mem::transmute(phaccesscheckresults)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn AuthzAddSidsToContext<'a, Param0: ::windows_core::IntoParam<'a, AUTHZ_CLIENT_CONTEXT_HANDLE>>(hauthzclientcontext: Param0, sids: *const super::SID_AND_ATTRIBUTES, sidcount: u32, restrictedsids: *const super::SID_AND_ATTRIBUTES, restrictedsidcount: u32, phnewauthzclientcontext: *mut isize) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AuthzAddSidsToContext(hauthzclientcontext: AUTHZ_CLIENT_CONTEXT_HANDLE, sids: *const super::SID_AND_ATTRIBUTES, sidcount: u32, restrictedsids: *const super::SID_AND_ATTRIBUTES, restrictedsidcount: u32, phnewauthzclientcontext: *mut isize) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(AuthzAddSidsToContext(hauthzclientcontext.into_param().abi(), ::core::mem::transmute(sids), ::core::mem::transmute(sidcount), ::core::mem::transmute(restrictedsids), ::core::mem::transmute(restrictedsidcount), ::core::mem::transmute(phnewauthzclientcontext)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn AuthzCachedAccessCheck<'a, Param1: ::windows_core::IntoParam<'a, AUTHZ_ACCESS_CHECK_RESULTS_HANDLE>, Param3: ::windows_core::IntoParam<'a, AUTHZ_AUDIT_EVENT_HANDLE>>(flags: u32, haccesscheckresults: Param1, prequest: *const AUTHZ_ACCESS_REQUEST, hauditevent: Param3, preply: *mut AUTHZ_ACCESS_REPLY) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AuthzCachedAccessCheck(flags: u32, haccesscheckresults: AUTHZ_ACCESS_CHECK_RESULTS_HANDLE, prequest: *const AUTHZ_ACCESS_REQUEST, hauditevent: AUTHZ_AUDIT_EVENT_HANDLE, preply: *mut AUTHZ_ACCESS_REPLY) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(AuthzCachedAccessCheck(::core::mem::transmute(flags), haccesscheckresults.into_param().abi(), ::core::mem::transmute(prequest), hauditevent.into_param().abi(), ::core::mem::transmute(preply)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn AuthzEnumerateSecurityEventSources(dwflags: u32, buffer: *mut AUTHZ_SOURCE_SCHEMA_REGISTRATION, pdwcount: *mut u32, pdwlength: *mut u32) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AuthzEnumerateSecurityEventSources(dwflags: u32, buffer: *mut AUTHZ_SOURCE_SCHEMA_REGISTRATION, pdwcount: *mut u32, pdwlength: *mut u32) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(AuthzEnumerateSecurityEventSources(::core::mem::transmute(dwflags), ::core::mem::transmute(buffer), ::core::mem::transmute(pdwcount), ::core::mem::transmute(pdwlength)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn AuthzEvaluateSacl<'a, Param0: ::windows_core::IntoParam<'a, AUTHZ_CLIENT_CONTEXT_HANDLE>, Param4: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(authzclientcontext: Param0, prequest: *const AUTHZ_ACCESS_REQUEST, sacl: *const super::ACL, grantedaccess: u32, accessgranted: Param4, pbgenerateaudit: *mut ::win32_foundation::BOOL) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AuthzEvaluateSacl(authzclientcontext: AUTHZ_CLIENT_CONTEXT_HANDLE, prequest: *const AUTHZ_ACCESS_REQUEST, sacl: *const super::ACL, grantedaccess: u32, accessgranted: ::win32_foundation::BOOL, pbgenerateaudit: *mut ::win32_foundation::BOOL) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(AuthzEvaluateSacl(authzclientcontext.into_param().abi(), ::core::mem::transmute(prequest), ::core::mem::transmute(sacl), ::core::mem::transmute(grantedaccess), accessgranted.into_param().abi(), ::core::mem::transmute(pbgenerateaudit)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn AuthzFreeAuditEvent<'a, Param0: ::windows_core::IntoParam<'a, AUTHZ_AUDIT_EVENT_HANDLE>>(hauditevent: Param0) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AuthzFreeAuditEvent(hauditevent: AUTHZ_AUDIT_EVENT_HANDLE) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(AuthzFreeAuditEvent(hauditevent.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn AuthzFreeCentralAccessPolicyCache() -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AuthzFreeCentralAccessPolicyCache() -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(AuthzFreeCentralAccessPolicyCache())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn AuthzFreeContext<'a, Param0: ::windows_core::IntoParam<'a, AUTHZ_CLIENT_CONTEXT_HANDLE>>(hauthzclientcontext: Param0) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AuthzFreeContext(hauthzclientcontext: AUTHZ_CLIENT_CONTEXT_HANDLE) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(AuthzFreeContext(hauthzclientcontext.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn AuthzFreeHandle<'a, Param0: ::windows_core::IntoParam<'a, AUTHZ_ACCESS_CHECK_RESULTS_HANDLE>>(haccesscheckresults: Param0) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AuthzFreeHandle(haccesscheckresults: AUTHZ_ACCESS_CHECK_RESULTS_HANDLE) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(AuthzFreeHandle(haccesscheckresults.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn AuthzFreeResourceManager<'a, Param0: ::windows_core::IntoParam<'a, AUTHZ_RESOURCE_MANAGER_HANDLE>>(hauthzresourcemanager: Param0) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AuthzFreeResourceManager(hauthzresourcemanager: AUTHZ_RESOURCE_MANAGER_HANDLE) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(AuthzFreeResourceManager(hauthzresourcemanager.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn AuthzGetInformationFromContext<'a, Param0: ::windows_core::IntoParam<'a, AUTHZ_CLIENT_CONTEXT_HANDLE>>(hauthzclientcontext: Param0, infoclass: AUTHZ_CONTEXT_INFORMATION_CLASS, buffersize: u32, psizerequired: *mut u32, buffer: *mut ::core::ffi::c_void) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AuthzGetInformationFromContext(hauthzclientcontext: AUTHZ_CLIENT_CONTEXT_HANDLE, infoclass: AUTHZ_CONTEXT_INFORMATION_CLASS, buffersize: u32, psizerequired: *mut u32, buffer: *mut ::core::ffi::c_void) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(AuthzGetInformationFromContext(hauthzclientcontext.into_param().abi(), ::core::mem::transmute(infoclass), ::core::mem::transmute(buffersize), ::core::mem::transmute(psizerequired), ::core::mem::transmute(buffer)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn AuthzInitializeCompoundContext<'a, Param0: ::windows_core::IntoParam<'a, AUTHZ_CLIENT_CONTEXT_HANDLE>, Param1: ::windows_core::IntoParam<'a, AUTHZ_CLIENT_CONTEXT_HANDLE>>(usercontext: Param0, devicecontext: Param1, phcompoundcontext: *mut isize) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AuthzInitializeCompoundContext(usercontext: AUTHZ_CLIENT_CONTEXT_HANDLE, devicecontext: AUTHZ_CLIENT_CONTEXT_HANDLE, phcompoundcontext: *mut isize) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(AuthzInitializeCompoundContext(usercontext.into_param().abi(), devicecontext.into_param().abi(), ::core::mem::transmute(phcompoundcontext)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn AuthzInitializeContextFromAuthzContext<'a, Param1: ::windows_core::IntoParam<'a, AUTHZ_CLIENT_CONTEXT_HANDLE>, Param3: ::windows_core::IntoParam<'a, ::win32_foundation::LUID>>(flags: u32, hauthzclientcontext: Param1, pexpirationtime: *const i64, identifier: Param3, dynamicgroupargs: *const ::core::ffi::c_void, phnewauthzclientcontext: *mut isize) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AuthzInitializeContextFromAuthzContext(flags: u32, hauthzclientcontext: AUTHZ_CLIENT_CONTEXT_HANDLE, pexpirationtime: *const i64, identifier: ::win32_foundation::LUID, dynamicgroupargs: *const ::core::ffi::c_void, phnewauthzclientcontext: *mut isize) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(AuthzInitializeContextFromAuthzContext(::core::mem::transmute(flags), hauthzclientcontext.into_param().abi(), ::core::mem::transmute(pexpirationtime), identifier.into_param().abi(), ::core::mem::transmute(dynamicgroupargs), ::core::mem::transmute(phnewauthzclientcontext)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn AuthzInitializeContextFromSid<'a, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::PSID>, Param2: ::windows_core::IntoParam<'a, AUTHZ_RESOURCE_MANAGER_HANDLE>, Param4: ::windows_core::IntoParam<'a, ::win32_foundation::LUID>>(flags: u32, usersid: Param1, hauthzresourcemanager: Param2, pexpirationtime: *const i64, identifier: Param4, dynamicgroupargs: *const ::core::ffi::c_void, phauthzclientcontext: *mut isize) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AuthzInitializeContextFromSid(flags: u32, usersid: ::win32_foundation::PSID, hauthzresourcemanager: AUTHZ_RESOURCE_MANAGER_HANDLE, pexpirationtime: *const i64, identifier: ::win32_foundation::LUID, dynamicgroupargs: *const ::core::ffi::c_void, phauthzclientcontext: *mut isize) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(AuthzInitializeContextFromSid(::core::mem::transmute(flags), usersid.into_param().abi(), hauthzresourcemanager.into_param().abi(), ::core::mem::transmute(pexpirationtime), identifier.into_param().abi(), ::core::mem::transmute(dynamicgroupargs), ::core::mem::transmute(phauthzclientcontext)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn AuthzInitializeContextFromToken<'a, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::HANDLE>, Param2: ::windows_core::IntoParam<'a, AUTHZ_RESOURCE_MANAGER_HANDLE>, Param4: ::windows_core::IntoParam<'a, ::win32_foundation::LUID>>(flags: u32, tokenhandle: Param1, hauthzresourcemanager: Param2, pexpirationtime: *const i64, identifier: Param4, dynamicgroupargs: *const ::core::ffi::c_void, phauthzclientcontext: *mut isize) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AuthzInitializeContextFromToken(flags: u32, tokenhandle: ::win32_foundation::HANDLE, hauthzresourcemanager: AUTHZ_RESOURCE_MANAGER_HANDLE, pexpirationtime: *const i64, identifier: ::win32_foundation::LUID, dynamicgroupargs: *const ::core::ffi::c_void, phauthzclientcontext: *mut isize) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(AuthzInitializeContextFromToken(::core::mem::transmute(flags), tokenhandle.into_param().abi(), hauthzresourcemanager.into_param().abi(), ::core::mem::transmute(pexpirationtime), identifier.into_param().abi(), ::core::mem::transmute(dynamicgroupargs), ::core::mem::transmute(phauthzclientcontext)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn AuthzInitializeObjectAccessAuditEvent<'a, Param1: ::windows_core::IntoParam<'a, AUTHZ_AUDIT_EVENT_TYPE_HANDLE>, Param2: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param3: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param4: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param5: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(flags: AUTHZ_INITIALIZE_OBJECT_ACCESS_AUDIT_EVENT_FLAGS, hauditeventtype: Param1, szoperationtype: Param2, szobjecttype: Param3, szobjectname: Param4, szadditionalinfo: Param5, phauditevent: *mut isize, dwadditionalparametercount: u32) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AuthzInitializeObjectAccessAuditEvent(flags: AUTHZ_INITIALIZE_OBJECT_ACCESS_AUDIT_EVENT_FLAGS, hauditeventtype: AUTHZ_AUDIT_EVENT_TYPE_HANDLE, szoperationtype: ::windows_core::PCWSTR, szobjecttype: ::windows_core::PCWSTR, szobjectname: ::windows_core::PCWSTR, szadditionalinfo: ::windows_core::PCWSTR, phauditevent: *mut isize, dwadditionalparametercount: u32) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(AuthzInitializeObjectAccessAuditEvent(::core::mem::transmute(flags), hauditeventtype.into_param().abi(), szoperationtype.into_param().abi(), szobjecttype.into_param().abi(), szobjectname.into_param().abi(), szadditionalinfo.into_param().abi(), ::core::mem::transmute(phauditevent), ::core::mem::transmute(dwadditionalparametercount)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn AuthzInitializeObjectAccessAuditEvent2<'a, Param1: ::windows_core::IntoParam<'a, AUTHZ_AUDIT_EVENT_TYPE_HANDLE>, Param2: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param3: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param4: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param5: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param6: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(flags: u32, hauditeventtype: Param1, szoperationtype: Param2, szobjecttype: Param3, szobjectname: Param4, szadditionalinfo: Param5, szadditionalinfo2: Param6, phauditevent: *mut isize, dwadditionalparametercount: u32) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AuthzInitializeObjectAccessAuditEvent2(flags: u32, hauditeventtype: AUTHZ_AUDIT_EVENT_TYPE_HANDLE, szoperationtype: ::windows_core::PCWSTR, szobjecttype: ::windows_core::PCWSTR, szobjectname: ::windows_core::PCWSTR, szadditionalinfo: ::windows_core::PCWSTR, szadditionalinfo2: ::windows_core::PCWSTR, phauditevent: *mut isize, dwadditionalparametercount: u32) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(AuthzInitializeObjectAccessAuditEvent2(::core::mem::transmute(flags), hauditeventtype.into_param().abi(), szoperationtype.into_param().abi(), szobjecttype.into_param().abi(), szobjectname.into_param().abi(), szadditionalinfo.into_param().abi(), szadditionalinfo2.into_param().abi(), ::core::mem::transmute(phauditevent), ::core::mem::transmute(dwadditionalparametercount)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn AuthzInitializeRemoteResourceManager(prpcinitinfo: *const AUTHZ_RPC_INIT_INFO_CLIENT, phauthzresourcemanager: *mut AUTHZ_RESOURCE_MANAGER_HANDLE) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AuthzInitializeRemoteResourceManager(prpcinitinfo: *const AUTHZ_RPC_INIT_INFO_CLIENT, phauthzresourcemanager: *mut AUTHZ_RESOURCE_MANAGER_HANDLE) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(AuthzInitializeRemoteResourceManager(::core::mem::transmute(prpcinitinfo), ::core::mem::transmute(phauthzresourcemanager)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn AuthzInitializeResourceManager<'a, Param4: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(flags: u32, pfndynamicaccesscheck: PFN_AUTHZ_DYNAMIC_ACCESS_CHECK, pfncomputedynamicgroups: PFN_AUTHZ_COMPUTE_DYNAMIC_GROUPS, pfnfreedynamicgroups: PFN_AUTHZ_FREE_DYNAMIC_GROUPS, szresourcemanagername: Param4, phauthzresourcemanager: *mut AUTHZ_RESOURCE_MANAGER_HANDLE) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AuthzInitializeResourceManager(flags: u32, pfndynamicaccesscheck: ::windows_core::RawPtr, pfncomputedynamicgroups: ::windows_core::RawPtr, pfnfreedynamicgroups: ::windows_core::RawPtr, szresourcemanagername: ::windows_core::PCWSTR, phauthzresourcemanager: *mut AUTHZ_RESOURCE_MANAGER_HANDLE) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(AuthzInitializeResourceManager(::core::mem::transmute(flags), ::core::mem::transmute(pfndynamicaccesscheck), ::core::mem::transmute(pfncomputedynamicgroups), ::core::mem::transmute(pfnfreedynamicgroups), szresourcemanagername.into_param().abi(), ::core::mem::transmute(phauthzresourcemanager)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn AuthzInitializeResourceManagerEx(flags: AUTHZ_RESOURCE_MANAGER_FLAGS, pauthzinitinfo: *const AUTHZ_INIT_INFO, phauthzresourcemanager: *mut AUTHZ_RESOURCE_MANAGER_HANDLE) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AuthzInitializeResourceManagerEx(flags: AUTHZ_RESOURCE_MANAGER_FLAGS, pauthzinitinfo: *const AUTHZ_INIT_INFO, phauthzresourcemanager: *mut AUTHZ_RESOURCE_MANAGER_HANDLE) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(AuthzInitializeResourceManagerEx(::core::mem::transmute(flags), ::core::mem::transmute(pauthzinitinfo), ::core::mem::transmute(phauthzresourcemanager)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn AuthzInstallSecurityEventSource(dwflags: u32, pregistration: *const AUTHZ_SOURCE_SCHEMA_REGISTRATION) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AuthzInstallSecurityEventSource(dwflags: u32, pregistration: *const AUTHZ_SOURCE_SCHEMA_REGISTRATION) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(AuthzInstallSecurityEventSource(::core::mem::transmute(dwflags), ::core::mem::transmute(pregistration)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn AuthzModifyClaims<'a, Param0: ::windows_core::IntoParam<'a, AUTHZ_CLIENT_CONTEXT_HANDLE>>(hauthzclientcontext: Param0, claimclass: AUTHZ_CONTEXT_INFORMATION_CLASS, pclaimoperations: *const AUTHZ_SECURITY_ATTRIBUTE_OPERATION, pclaims: *const AUTHZ_SECURITY_ATTRIBUTES_INFORMATION) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AuthzModifyClaims(hauthzclientcontext: AUTHZ_CLIENT_CONTEXT_HANDLE, claimclass: AUTHZ_CONTEXT_INFORMATION_CLASS, pclaimoperations: *const AUTHZ_SECURITY_ATTRIBUTE_OPERATION, pclaims: *const AUTHZ_SECURITY_ATTRIBUTES_INFORMATION) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(AuthzModifyClaims(hauthzclientcontext.into_param().abi(), ::core::mem::transmute(claimclass), ::core::mem::transmute(pclaimoperations), ::core::mem::transmute(pclaims)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn AuthzModifySecurityAttributes<'a, Param0: ::windows_core::IntoParam<'a, AUTHZ_CLIENT_CONTEXT_HANDLE>>(hauthzclientcontext: Param0, poperations: *const AUTHZ_SECURITY_ATTRIBUTE_OPERATION, pattributes: *const AUTHZ_SECURITY_ATTRIBUTES_INFORMATION) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AuthzModifySecurityAttributes(hauthzclientcontext: AUTHZ_CLIENT_CONTEXT_HANDLE, poperations: *const AUTHZ_SECURITY_ATTRIBUTE_OPERATION, pattributes: *const AUTHZ_SECURITY_ATTRIBUTES_INFORMATION) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(AuthzModifySecurityAttributes(hauthzclientcontext.into_param().abi(), ::core::mem::transmute(poperations), ::core::mem::transmute(pattributes)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn AuthzModifySids<'a, Param0: ::windows_core::IntoParam<'a, AUTHZ_CLIENT_CONTEXT_HANDLE>>(hauthzclientcontext: Param0, sidclass: AUTHZ_CONTEXT_INFORMATION_CLASS, psidoperations: *const AUTHZ_SID_OPERATION, psids: *const super::TOKEN_GROUPS) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AuthzModifySids(hauthzclientcontext: AUTHZ_CLIENT_CONTEXT_HANDLE, sidclass: AUTHZ_CONTEXT_INFORMATION_CLASS, psidoperations: *const AUTHZ_SID_OPERATION, psids: *const super::TOKEN_GROUPS) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(AuthzModifySids(hauthzclientcontext.into_param().abi(), ::core::mem::transmute(sidclass), ::core::mem::transmute(psidoperations), ::core::mem::transmute(psids)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn AuthzOpenObjectAudit<'a, Param1: ::windows_core::IntoParam<'a, AUTHZ_CLIENT_CONTEXT_HANDLE>, Param3: ::windows_core::IntoParam<'a, AUTHZ_AUDIT_EVENT_HANDLE>, Param4: ::windows_core::IntoParam<'a, super::PSECURITY_DESCRIPTOR>>(flags: u32, hauthzclientcontext: Param1, prequest: *const AUTHZ_ACCESS_REQUEST, hauditevent: Param3, psecuritydescriptor: Param4, optionalsecuritydescriptorarray: &[super::PSECURITY_DESCRIPTOR], preply: *const AUTHZ_ACCESS_REPLY) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AuthzOpenObjectAudit(flags: u32, hauthzclientcontext: AUTHZ_CLIENT_CONTEXT_HANDLE, prequest: *const AUTHZ_ACCESS_REQUEST, hauditevent: AUTHZ_AUDIT_EVENT_HANDLE, psecuritydescriptor: super::PSECURITY_DESCRIPTOR, optionalsecuritydescriptorarray: *const super::PSECURITY_DESCRIPTOR, optionalsecuritydescriptorcount: u32, preply: *const AUTHZ_ACCESS_REPLY) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(AuthzOpenObjectAudit(::core::mem::transmute(flags), hauthzclientcontext.into_param().abi(), ::core::mem::transmute(prequest), hauditevent.into_param().abi(), psecuritydescriptor.into_param().abi(), ::core::mem::transmute(::windows_core::as_ptr_or_null(optionalsecuritydescriptorarray)), optionalsecuritydescriptorarray.len() as _, ::core::mem::transmute(preply)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_Threading")]
#[inline]
pub unsafe fn AuthzRegisterCapChangeNotification(phcapchangesubscription: *mut *mut AUTHZ_CAP_CHANGE_SUBSCRIPTION_HANDLE__, pfncapchangecallback: ::win32_system::Threading::LPTHREAD_START_ROUTINE, pcallbackcontext: *const ::core::ffi::c_void) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AuthzRegisterCapChangeNotification(phcapchangesubscription: *mut *mut AUTHZ_CAP_CHANGE_SUBSCRIPTION_HANDLE__, pfncapchangecallback: ::windows_core::RawPtr, pcallbackcontext: *const ::core::ffi::c_void) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(AuthzRegisterCapChangeNotification(::core::mem::transmute(phcapchangesubscription), ::core::mem::transmute(pfncapchangecallback), ::core::mem::transmute(pcallbackcontext)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn AuthzRegisterSecurityEventSource<'a, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(dwflags: u32, szeventsourcename: Param1, pheventprovider: *mut isize) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AuthzRegisterSecurityEventSource(dwflags: u32, szeventsourcename: ::windows_core::PCWSTR, pheventprovider: *mut isize) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(AuthzRegisterSecurityEventSource(::core::mem::transmute(dwflags), szeventsourcename.into_param().abi(), ::core::mem::transmute(pheventprovider)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn AuthzReportSecurityEvent<'a, Param1: ::windows_core::IntoParam<'a, AUTHZ_SECURITY_EVENT_PROVIDER_HANDLE>, Param3: ::windows_core::IntoParam<'a, ::win32_foundation::PSID>>(dwflags: u32, heventprovider: Param1, dwauditid: u32, pusersid: Param3, dwcount: u32) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AuthzReportSecurityEvent(dwflags: u32, heventprovider: AUTHZ_SECURITY_EVENT_PROVIDER_HANDLE, dwauditid: u32, pusersid: ::win32_foundation::PSID, dwcount: u32) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(AuthzReportSecurityEvent(::core::mem::transmute(dwflags), heventprovider.into_param().abi(), ::core::mem::transmute(dwauditid), pusersid.into_param().abi(), ::core::mem::transmute(dwcount)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn AuthzReportSecurityEventFromParams<'a, Param1: ::windows_core::IntoParam<'a, AUTHZ_SECURITY_EVENT_PROVIDER_HANDLE>, Param3: ::windows_core::IntoParam<'a, ::win32_foundation::PSID>>(dwflags: u32, heventprovider: Param1, dwauditid: u32, pusersid: Param3, pparams: *const AUDIT_PARAMS) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AuthzReportSecurityEventFromParams(dwflags: u32, heventprovider: AUTHZ_SECURITY_EVENT_PROVIDER_HANDLE, dwauditid: u32, pusersid: ::win32_foundation::PSID, pparams: *const AUDIT_PARAMS) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(AuthzReportSecurityEventFromParams(::core::mem::transmute(dwflags), heventprovider.into_param().abi(), ::core::mem::transmute(dwauditid), pusersid.into_param().abi(), ::core::mem::transmute(pparams)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn AuthzSetAppContainerInformation<'a, Param0: ::windows_core::IntoParam<'a, AUTHZ_CLIENT_CONTEXT_HANDLE>, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::PSID>>(hauthzclientcontext: Param0, pappcontainersid: Param1, pcapabilitysids: &[super::SID_AND_ATTRIBUTES]) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AuthzSetAppContainerInformation(hauthzclientcontext: AUTHZ_CLIENT_CONTEXT_HANDLE, pappcontainersid: ::win32_foundation::PSID, capabilitycount: u32, pcapabilitysids: *const super::SID_AND_ATTRIBUTES) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(AuthzSetAppContainerInformation(hauthzclientcontext.into_param().abi(), pappcontainersid.into_param().abi(), pcapabilitysids.len() as _, ::core::mem::transmute(::windows_core::as_ptr_or_null(pcapabilitysids))))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn AuthzUninstallSecurityEventSource<'a, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(dwflags: u32, szeventsourcename: Param1) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AuthzUninstallSecurityEventSource(dwflags: u32, szeventsourcename: ::windows_core::PCWSTR) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(AuthzUninstallSecurityEventSource(::core::mem::transmute(dwflags), szeventsourcename.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn AuthzUnregisterCapChangeNotification(hcapchangesubscription: *const AUTHZ_CAP_CHANGE_SUBSCRIPTION_HANDLE__) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AuthzUnregisterCapChangeNotification(hcapchangesubscription: *const AUTHZ_CAP_CHANGE_SUBSCRIPTION_HANDLE__) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(AuthzUnregisterCapChangeNotification(::core::mem::transmute(hcapchangesubscription)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn AuthzUnregisterSecurityEventSource(dwflags: u32, pheventprovider: *mut isize) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AuthzUnregisterSecurityEventSource(dwflags: u32, pheventprovider: *mut isize) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(AuthzUnregisterSecurityEventSource(::core::mem::transmute(dwflags), ::core::mem::transmute(pheventprovider)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub const AzAuthorizationStore: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb2bcff59_a757_4b0b_a1bc_ea69981da69e);
pub const AzBizRuleContext: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5c2dc96f_8d51_434b_b33c_379bccae77c3);
pub const AzPrincipalLocator: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x483afb5d_70df_4e16_abdc_a1de4d015a3e);
#[inline]
pub unsafe fn BuildExplicitAccessWithNameA<'a, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCSTR>>(pexplicitaccess: *mut EXPLICIT_ACCESS_A, ptrusteename: Param1, accesspermissions: u32, accessmode: ACCESS_MODE, inheritance: super::ACE_FLAGS) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn BuildExplicitAccessWithNameA(pexplicitaccess: *mut EXPLICIT_ACCESS_A, ptrusteename: ::windows_core::PCSTR, accesspermissions: u32, accessmode: ACCESS_MODE, inheritance: super::ACE_FLAGS);
        }
        BuildExplicitAccessWithNameA(::core::mem::transmute(pexplicitaccess), ptrusteename.into_param().abi(), ::core::mem::transmute(accesspermissions), ::core::mem::transmute(accessmode), ::core::mem::transmute(inheritance))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn BuildExplicitAccessWithNameW<'a, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(pexplicitaccess: *mut EXPLICIT_ACCESS_W, ptrusteename: Param1, accesspermissions: u32, accessmode: ACCESS_MODE, inheritance: super::ACE_FLAGS) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn BuildExplicitAccessWithNameW(pexplicitaccess: *mut EXPLICIT_ACCESS_W, ptrusteename: ::windows_core::PCWSTR, accesspermissions: u32, accessmode: ACCESS_MODE, inheritance: super::ACE_FLAGS);
        }
        BuildExplicitAccessWithNameW(::core::mem::transmute(pexplicitaccess), ptrusteename.into_param().abi(), ::core::mem::transmute(accesspermissions), ::core::mem::transmute(accessmode), ::core::mem::transmute(inheritance))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn BuildImpersonateExplicitAccessWithNameA<'a, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCSTR>>(pexplicitaccess: *mut EXPLICIT_ACCESS_A, ptrusteename: Param1, ptrustee: *const TRUSTEE_A, accesspermissions: u32, accessmode: ACCESS_MODE, inheritance: u32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn BuildImpersonateExplicitAccessWithNameA(pexplicitaccess: *mut EXPLICIT_ACCESS_A, ptrusteename: ::windows_core::PCSTR, ptrustee: *const TRUSTEE_A, accesspermissions: u32, accessmode: ACCESS_MODE, inheritance: u32);
        }
        BuildImpersonateExplicitAccessWithNameA(::core::mem::transmute(pexplicitaccess), ptrusteename.into_param().abi(), ::core::mem::transmute(ptrustee), ::core::mem::transmute(accesspermissions), ::core::mem::transmute(accessmode), ::core::mem::transmute(inheritance))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn BuildImpersonateExplicitAccessWithNameW<'a, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(pexplicitaccess: *mut EXPLICIT_ACCESS_W, ptrusteename: Param1, ptrustee: *const TRUSTEE_W, accesspermissions: u32, accessmode: ACCESS_MODE, inheritance: u32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn BuildImpersonateExplicitAccessWithNameW(pexplicitaccess: *mut EXPLICIT_ACCESS_W, ptrusteename: ::windows_core::PCWSTR, ptrustee: *const TRUSTEE_W, accesspermissions: u32, accessmode: ACCESS_MODE, inheritance: u32);
        }
        BuildImpersonateExplicitAccessWithNameW(::core::mem::transmute(pexplicitaccess), ptrusteename.into_param().abi(), ::core::mem::transmute(ptrustee), ::core::mem::transmute(accesspermissions), ::core::mem::transmute(accessmode), ::core::mem::transmute(inheritance))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn BuildImpersonateTrusteeA(ptrustee: *mut TRUSTEE_A, pimpersonatetrustee: *const TRUSTEE_A) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn BuildImpersonateTrusteeA(ptrustee: *mut TRUSTEE_A, pimpersonatetrustee: *const TRUSTEE_A);
        }
        BuildImpersonateTrusteeA(::core::mem::transmute(ptrustee), ::core::mem::transmute(pimpersonatetrustee))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn BuildImpersonateTrusteeW(ptrustee: *mut TRUSTEE_W, pimpersonatetrustee: *const TRUSTEE_W) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn BuildImpersonateTrusteeW(ptrustee: *mut TRUSTEE_W, pimpersonatetrustee: *const TRUSTEE_W);
        }
        BuildImpersonateTrusteeW(::core::mem::transmute(ptrustee), ::core::mem::transmute(pimpersonatetrustee))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn BuildSecurityDescriptorA<'a, Param6: ::windows_core::IntoParam<'a, super::PSECURITY_DESCRIPTOR>>(powner: *const TRUSTEE_A, pgroup: *const TRUSTEE_A, plistofaccessentries: &[EXPLICIT_ACCESS_A], plistofauditentries: &[EXPLICIT_ACCESS_A], poldsd: Param6, psizenewsd: *mut u32, pnewsd: *mut super::PSECURITY_DESCRIPTOR) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn BuildSecurityDescriptorA(powner: *const TRUSTEE_A, pgroup: *const TRUSTEE_A, ccountofaccessentries: u32, plistofaccessentries: *const EXPLICIT_ACCESS_A, ccountofauditentries: u32, plistofauditentries: *const EXPLICIT_ACCESS_A, poldsd: super::PSECURITY_DESCRIPTOR, psizenewsd: *mut u32, pnewsd: *mut super::PSECURITY_DESCRIPTOR) -> u32;
        }
        ::core::mem::transmute(BuildSecurityDescriptorA(::core::mem::transmute(powner), ::core::mem::transmute(pgroup), plistofaccessentries.len() as _, ::core::mem::transmute(::windows_core::as_ptr_or_null(plistofaccessentries)), plistofauditentries.len() as _, ::core::mem::transmute(::windows_core::as_ptr_or_null(plistofauditentries)), poldsd.into_param().abi(), ::core::mem::transmute(psizenewsd), ::core::mem::transmute(pnewsd)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn BuildSecurityDescriptorW<'a, Param6: ::windows_core::IntoParam<'a, super::PSECURITY_DESCRIPTOR>>(powner: *const TRUSTEE_W, pgroup: *const TRUSTEE_W, plistofaccessentries: &[EXPLICIT_ACCESS_W], plistofauditentries: &[EXPLICIT_ACCESS_W], poldsd: Param6, psizenewsd: *mut u32, pnewsd: *mut super::PSECURITY_DESCRIPTOR) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn BuildSecurityDescriptorW(powner: *const TRUSTEE_W, pgroup: *const TRUSTEE_W, ccountofaccessentries: u32, plistofaccessentries: *const EXPLICIT_ACCESS_W, ccountofauditentries: u32, plistofauditentries: *const EXPLICIT_ACCESS_W, poldsd: super::PSECURITY_DESCRIPTOR, psizenewsd: *mut u32, pnewsd: *mut super::PSECURITY_DESCRIPTOR) -> u32;
        }
        ::core::mem::transmute(BuildSecurityDescriptorW(::core::mem::transmute(powner), ::core::mem::transmute(pgroup), plistofaccessentries.len() as _, ::core::mem::transmute(::windows_core::as_ptr_or_null(plistofaccessentries)), plistofauditentries.len() as _, ::core::mem::transmute(::windows_core::as_ptr_or_null(plistofauditentries)), poldsd.into_param().abi(), ::core::mem::transmute(psizenewsd), ::core::mem::transmute(pnewsd)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn BuildTrusteeWithNameA<'a, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCSTR>>(ptrustee: *mut TRUSTEE_A, pname: Param1) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn BuildTrusteeWithNameA(ptrustee: *mut TRUSTEE_A, pname: ::windows_core::PCSTR);
        }
        BuildTrusteeWithNameA(::core::mem::transmute(ptrustee), pname.into_param().abi())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn BuildTrusteeWithNameW<'a, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(ptrustee: *mut TRUSTEE_W, pname: Param1) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn BuildTrusteeWithNameW(ptrustee: *mut TRUSTEE_W, pname: ::windows_core::PCWSTR);
        }
        BuildTrusteeWithNameW(::core::mem::transmute(ptrustee), pname.into_param().abi())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn BuildTrusteeWithObjectsAndNameA<'a, Param3: ::windows_core::IntoParam<'a, ::windows_core::PCSTR>, Param4: ::windows_core::IntoParam<'a, ::windows_core::PCSTR>, Param5: ::windows_core::IntoParam<'a, ::windows_core::PCSTR>>(ptrustee: *mut TRUSTEE_A, pobjname: *const OBJECTS_AND_NAME_A, objecttype: SE_OBJECT_TYPE, objecttypename: Param3, inheritedobjecttypename: Param4, name: Param5) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn BuildTrusteeWithObjectsAndNameA(ptrustee: *mut TRUSTEE_A, pobjname: *const OBJECTS_AND_NAME_A, objecttype: SE_OBJECT_TYPE, objecttypename: ::windows_core::PCSTR, inheritedobjecttypename: ::windows_core::PCSTR, name: ::windows_core::PCSTR);
        }
        BuildTrusteeWithObjectsAndNameA(::core::mem::transmute(ptrustee), ::core::mem::transmute(pobjname), ::core::mem::transmute(objecttype), objecttypename.into_param().abi(), inheritedobjecttypename.into_param().abi(), name.into_param().abi())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn BuildTrusteeWithObjectsAndNameW<'a, Param3: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param4: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param5: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(ptrustee: *mut TRUSTEE_W, pobjname: *const OBJECTS_AND_NAME_W, objecttype: SE_OBJECT_TYPE, objecttypename: Param3, inheritedobjecttypename: Param4, name: Param5) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn BuildTrusteeWithObjectsAndNameW(ptrustee: *mut TRUSTEE_W, pobjname: *const OBJECTS_AND_NAME_W, objecttype: SE_OBJECT_TYPE, objecttypename: ::windows_core::PCWSTR, inheritedobjecttypename: ::windows_core::PCWSTR, name: ::windows_core::PCWSTR);
        }
        BuildTrusteeWithObjectsAndNameW(::core::mem::transmute(ptrustee), ::core::mem::transmute(pobjname), ::core::mem::transmute(objecttype), objecttypename.into_param().abi(), inheritedobjecttypename.into_param().abi(), name.into_param().abi())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn BuildTrusteeWithObjectsAndSidA<'a, Param4: ::windows_core::IntoParam<'a, ::win32_foundation::PSID>>(ptrustee: *mut TRUSTEE_A, pobjsid: *const OBJECTS_AND_SID, pobjectguid: *const ::windows_core::GUID, pinheritedobjectguid: *const ::windows_core::GUID, psid: Param4) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn BuildTrusteeWithObjectsAndSidA(ptrustee: *mut TRUSTEE_A, pobjsid: *const OBJECTS_AND_SID, pobjectguid: *const ::windows_core::GUID, pinheritedobjectguid: *const ::windows_core::GUID, psid: ::win32_foundation::PSID);
        }
        BuildTrusteeWithObjectsAndSidA(::core::mem::transmute(ptrustee), ::core::mem::transmute(pobjsid), ::core::mem::transmute(pobjectguid), ::core::mem::transmute(pinheritedobjectguid), psid.into_param().abi())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn BuildTrusteeWithObjectsAndSidW<'a, Param4: ::windows_core::IntoParam<'a, ::win32_foundation::PSID>>(ptrustee: *mut TRUSTEE_W, pobjsid: *const OBJECTS_AND_SID, pobjectguid: *const ::windows_core::GUID, pinheritedobjectguid: *const ::windows_core::GUID, psid: Param4) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn BuildTrusteeWithObjectsAndSidW(ptrustee: *mut TRUSTEE_W, pobjsid: *const OBJECTS_AND_SID, pobjectguid: *const ::windows_core::GUID, pinheritedobjectguid: *const ::windows_core::GUID, psid: ::win32_foundation::PSID);
        }
        BuildTrusteeWithObjectsAndSidW(::core::mem::transmute(ptrustee), ::core::mem::transmute(pobjsid), ::core::mem::transmute(pobjectguid), ::core::mem::transmute(pinheritedobjectguid), psid.into_param().abi())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn BuildTrusteeWithSidA<'a, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::PSID>>(ptrustee: *mut TRUSTEE_A, psid: Param1) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn BuildTrusteeWithSidA(ptrustee: *mut TRUSTEE_A, psid: ::win32_foundation::PSID);
        }
        BuildTrusteeWithSidA(::core::mem::transmute(ptrustee), psid.into_param().abi())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn BuildTrusteeWithSidW<'a, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::PSID>>(ptrustee: *mut TRUSTEE_W, psid: Param1) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn BuildTrusteeWithSidW(ptrustee: *mut TRUSTEE_W, psid: ::win32_foundation::PSID);
        }
        BuildTrusteeWithSidW(::core::mem::transmute(ptrustee), psid.into_param().abi())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn ConvertSecurityDescriptorToStringSecurityDescriptorA<'a, Param0: ::windows_core::IntoParam<'a, super::PSECURITY_DESCRIPTOR>>(securitydescriptor: Param0, requestedstringsdrevision: u32, securityinformation: u32, stringsecuritydescriptor: *mut ::windows_core::PSTR, stringsecuritydescriptorlen: *mut u32) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ConvertSecurityDescriptorToStringSecurityDescriptorA(securitydescriptor: super::PSECURITY_DESCRIPTOR, requestedstringsdrevision: u32, securityinformation: u32, stringsecuritydescriptor: *mut ::windows_core::PSTR, stringsecuritydescriptorlen: *mut u32) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(ConvertSecurityDescriptorToStringSecurityDescriptorA(securitydescriptor.into_param().abi(), ::core::mem::transmute(requestedstringsdrevision), ::core::mem::transmute(securityinformation), ::core::mem::transmute(stringsecuritydescriptor), ::core::mem::transmute(stringsecuritydescriptorlen)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn ConvertSecurityDescriptorToStringSecurityDescriptorW<'a, Param0: ::windows_core::IntoParam<'a, super::PSECURITY_DESCRIPTOR>>(securitydescriptor: Param0, requestedstringsdrevision: u32, securityinformation: u32, stringsecuritydescriptor: *mut ::windows_core::PWSTR, stringsecuritydescriptorlen: *mut u32) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ConvertSecurityDescriptorToStringSecurityDescriptorW(securitydescriptor: super::PSECURITY_DESCRIPTOR, requestedstringsdrevision: u32, securityinformation: u32, stringsecuritydescriptor: *mut ::windows_core::PWSTR, stringsecuritydescriptorlen: *mut u32) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(ConvertSecurityDescriptorToStringSecurityDescriptorW(securitydescriptor.into_param().abi(), ::core::mem::transmute(requestedstringsdrevision), ::core::mem::transmute(securityinformation), ::core::mem::transmute(stringsecuritydescriptor), ::core::mem::transmute(stringsecuritydescriptorlen)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn ConvertSidToStringSidA<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::PSID>>(sid: Param0, stringsid: *mut ::windows_core::PSTR) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ConvertSidToStringSidA(sid: ::win32_foundation::PSID, stringsid: *mut ::windows_core::PSTR) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(ConvertSidToStringSidA(sid.into_param().abi(), ::core::mem::transmute(stringsid)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn ConvertSidToStringSidW<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::PSID>>(sid: Param0, stringsid: *mut ::windows_core::PWSTR) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ConvertSidToStringSidW(sid: ::win32_foundation::PSID, stringsid: *mut ::windows_core::PWSTR) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(ConvertSidToStringSidW(sid.into_param().abi(), ::core::mem::transmute(stringsid)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn ConvertStringSecurityDescriptorToSecurityDescriptorA<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCSTR>>(stringsecuritydescriptor: Param0, stringsdrevision: u32, securitydescriptor: *mut super::PSECURITY_DESCRIPTOR, securitydescriptorsize: *mut u32) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ConvertStringSecurityDescriptorToSecurityDescriptorA(stringsecuritydescriptor: ::windows_core::PCSTR, stringsdrevision: u32, securitydescriptor: *mut super::PSECURITY_DESCRIPTOR, securitydescriptorsize: *mut u32) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(ConvertStringSecurityDescriptorToSecurityDescriptorA(stringsecuritydescriptor.into_param().abi(), ::core::mem::transmute(stringsdrevision), ::core::mem::transmute(securitydescriptor), ::core::mem::transmute(securitydescriptorsize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn ConvertStringSecurityDescriptorToSecurityDescriptorW<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(stringsecuritydescriptor: Param0, stringsdrevision: u32, securitydescriptor: *mut super::PSECURITY_DESCRIPTOR, securitydescriptorsize: *mut u32) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ConvertStringSecurityDescriptorToSecurityDescriptorW(stringsecuritydescriptor: ::windows_core::PCWSTR, stringsdrevision: u32, securitydescriptor: *mut super::PSECURITY_DESCRIPTOR, securitydescriptorsize: *mut u32) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(ConvertStringSecurityDescriptorToSecurityDescriptorW(stringsecuritydescriptor.into_param().abi(), ::core::mem::transmute(stringsdrevision), ::core::mem::transmute(securitydescriptor), ::core::mem::transmute(securitydescriptorsize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn ConvertStringSidToSidA<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCSTR>>(stringsid: Param0, sid: *mut ::win32_foundation::PSID) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ConvertStringSidToSidA(stringsid: ::windows_core::PCSTR, sid: *mut ::win32_foundation::PSID) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(ConvertStringSidToSidA(stringsid.into_param().abi(), ::core::mem::transmute(sid)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn ConvertStringSidToSidW<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(stringsid: Param0, sid: *mut ::win32_foundation::PSID) -> ::win32_foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ConvertStringSidToSidW(stringsid: ::windows_core::PCWSTR, sid: *mut ::win32_foundation::PSID) -> ::win32_foundation::BOOL;
        }
        ::core::mem::transmute(ConvertStringSidToSidW(stringsid.into_param().abi(), ::core::mem::transmute(sid)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[repr(C)]
pub struct EXPLICIT_ACCESS_A {
    pub grfAccessPermissions: u32,
    pub grfAccessMode: ACCESS_MODE,
    pub grfInheritance: super::ACE_FLAGS,
    pub Trustee: TRUSTEE_A,
}
impl ::core::marker::Copy for EXPLICIT_ACCESS_A {}
impl ::core::clone::Clone for EXPLICIT_ACCESS_A {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for EXPLICIT_ACCESS_A {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EXPLICIT_ACCESS_A").field("grfAccessPermissions", &self.grfAccessPermissions).field("grfAccessMode", &self.grfAccessMode).field("grfInheritance", &self.grfInheritance).field("Trustee", &self.Trustee).finish()
    }
}
unsafe impl ::windows_core::Abi for EXPLICIT_ACCESS_A {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for EXPLICIT_ACCESS_A {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<EXPLICIT_ACCESS_A>()) == 0 }
    }
}
impl ::core::cmp::Eq for EXPLICIT_ACCESS_A {}
impl ::core::default::Default for EXPLICIT_ACCESS_A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct EXPLICIT_ACCESS_W {
    pub grfAccessPermissions: u32,
    pub grfAccessMode: ACCESS_MODE,
    pub grfInheritance: super::ACE_FLAGS,
    pub Trustee: TRUSTEE_W,
}
impl ::core::marker::Copy for EXPLICIT_ACCESS_W {}
impl ::core::clone::Clone for EXPLICIT_ACCESS_W {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for EXPLICIT_ACCESS_W {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EXPLICIT_ACCESS_W").field("grfAccessPermissions", &self.grfAccessPermissions).field("grfAccessMode", &self.grfAccessMode).field("grfInheritance", &self.grfInheritance).field("Trustee", &self.Trustee).finish()
    }
}
unsafe impl ::windows_core::Abi for EXPLICIT_ACCESS_W {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for EXPLICIT_ACCESS_W {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<EXPLICIT_ACCESS_W>()) == 0 }
    }
}
impl ::core::cmp::Eq for EXPLICIT_ACCESS_W {}
impl ::core::default::Default for EXPLICIT_ACCESS_W {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct FN_OBJECT_MGR_FUNCTIONS {
    pub Placeholder: u32,
}
impl ::core::marker::Copy for FN_OBJECT_MGR_FUNCTIONS {}
impl ::core::clone::Clone for FN_OBJECT_MGR_FUNCTIONS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for FN_OBJECT_MGR_FUNCTIONS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FN_OBJECT_MGR_FUNCTIONS").field("Placeholder", &self.Placeholder).finish()
    }
}
unsafe impl ::windows_core::Abi for FN_OBJECT_MGR_FUNCTIONS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for FN_OBJECT_MGR_FUNCTIONS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FN_OBJECT_MGR_FUNCTIONS>()) == 0 }
    }
}
impl ::core::cmp::Eq for FN_OBJECT_MGR_FUNCTIONS {}
impl ::core::default::Default for FN_OBJECT_MGR_FUNCTIONS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub type FN_PROGRESS = ::core::option::Option<unsafe extern "system" fn(pobjectname: ::windows_core::PCWSTR, status: u32, pinvokesetting: *mut PROG_INVOKE_SETTING, args: *const ::core::ffi::c_void, securityset: ::win32_foundation::BOOL)>;
#[inline]
pub unsafe fn FreeInheritedFromArray(pinheritarray: &[INHERITED_FROMW], pfnarray: *const FN_OBJECT_MGR_FUNCTIONS) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FreeInheritedFromArray(pinheritarray: *const INHERITED_FROMW, acecnt: u16, pfnarray: *const FN_OBJECT_MGR_FUNCTIONS) -> u32;
        }
        ::core::mem::transmute(FreeInheritedFromArray(::core::mem::transmute(::windows_core::as_ptr_or_null(pinheritarray)), pinheritarray.len() as _, ::core::mem::transmute(pfnarray)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn GetAuditedPermissionsFromAclA(pacl: *const super::ACL, ptrustee: *const TRUSTEE_A, psuccessfulauditedrights: *mut u32, pfailedauditrights: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetAuditedPermissionsFromAclA(pacl: *const super::ACL, ptrustee: *const TRUSTEE_A, psuccessfulauditedrights: *mut u32, pfailedauditrights: *mut u32) -> u32;
        }
        ::core::mem::transmute(GetAuditedPermissionsFromAclA(::core::mem::transmute(pacl), ::core::mem::transmute(ptrustee), ::core::mem::transmute(psuccessfulauditedrights), ::core::mem::transmute(pfailedauditrights)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn GetAuditedPermissionsFromAclW(pacl: *const super::ACL, ptrustee: *const TRUSTEE_W, psuccessfulauditedrights: *mut u32, pfailedauditrights: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetAuditedPermissionsFromAclW(pacl: *const super::ACL, ptrustee: *const TRUSTEE_W, psuccessfulauditedrights: *mut u32, pfailedauditrights: *mut u32) -> u32;
        }
        ::core::mem::transmute(GetAuditedPermissionsFromAclW(::core::mem::transmute(pacl), ::core::mem::transmute(ptrustee), ::core::mem::transmute(psuccessfulauditedrights), ::core::mem::transmute(pfailedauditrights)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn GetEffectiveRightsFromAclA(pacl: *const super::ACL, ptrustee: *const TRUSTEE_A, paccessrights: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetEffectiveRightsFromAclA(pacl: *const super::ACL, ptrustee: *const TRUSTEE_A, paccessrights: *mut u32) -> u32;
        }
        ::core::mem::transmute(GetEffectiveRightsFromAclA(::core::mem::transmute(pacl), ::core::mem::transmute(ptrustee), ::core::mem::transmute(paccessrights)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn GetEffectiveRightsFromAclW(pacl: *const super::ACL, ptrustee: *const TRUSTEE_W, paccessrights: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetEffectiveRightsFromAclW(pacl: *const super::ACL, ptrustee: *const TRUSTEE_W, paccessrights: *mut u32) -> u32;
        }
        ::core::mem::transmute(GetEffectiveRightsFromAclW(::core::mem::transmute(pacl), ::core::mem::transmute(ptrustee), ::core::mem::transmute(paccessrights)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn GetExplicitEntriesFromAclA(pacl: *const super::ACL, pccountofexplicitentries: *mut u32, plistofexplicitentries: *mut *mut EXPLICIT_ACCESS_A) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetExplicitEntriesFromAclA(pacl: *const super::ACL, pccountofexplicitentries: *mut u32, plistofexplicitentries: *mut *mut EXPLICIT_ACCESS_A) -> u32;
        }
        ::core::mem::transmute(GetExplicitEntriesFromAclA(::core::mem::transmute(pacl), ::core::mem::transmute(pccountofexplicitentries), ::core::mem::transmute(plistofexplicitentries)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn GetExplicitEntriesFromAclW(pacl: *const super::ACL, pccountofexplicitentries: *mut u32, plistofexplicitentries: *mut *mut EXPLICIT_ACCESS_W) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetExplicitEntriesFromAclW(pacl: *const super::ACL, pccountofexplicitentries: *mut u32, plistofexplicitentries: *mut *mut EXPLICIT_ACCESS_W) -> u32;
        }
        ::core::mem::transmute(GetExplicitEntriesFromAclW(::core::mem::transmute(pacl), ::core::mem::transmute(pccountofexplicitentries), ::core::mem::transmute(plistofexplicitentries)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn GetInheritanceSourceA<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCSTR>, Param3: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(pobjectname: Param0, objecttype: SE_OBJECT_TYPE, securityinfo: u32, container: Param3, pobjectclassguids: &[*const ::windows_core::GUID], pacl: *const super::ACL, pfnarray: *const FN_OBJECT_MGR_FUNCTIONS, pgenericmapping: *const super::GENERIC_MAPPING, pinheritarray: *mut INHERITED_FROMA) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetInheritanceSourceA(pobjectname: ::windows_core::PCSTR, objecttype: SE_OBJECT_TYPE, securityinfo: u32, container: ::win32_foundation::BOOL, pobjectclassguids: *const *const ::windows_core::GUID, guidcount: u32, pacl: *const super::ACL, pfnarray: *const FN_OBJECT_MGR_FUNCTIONS, pgenericmapping: *const super::GENERIC_MAPPING, pinheritarray: *mut INHERITED_FROMA) -> u32;
        }
        ::core::mem::transmute(GetInheritanceSourceA(pobjectname.into_param().abi(), ::core::mem::transmute(objecttype), ::core::mem::transmute(securityinfo), container.into_param().abi(), ::core::mem::transmute(::windows_core::as_ptr_or_null(pobjectclassguids)), pobjectclassguids.len() as _, ::core::mem::transmute(pacl), ::core::mem::transmute(pfnarray), ::core::mem::transmute(pgenericmapping), ::core::mem::transmute(pinheritarray)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn GetInheritanceSourceW<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param3: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(pobjectname: Param0, objecttype: SE_OBJECT_TYPE, securityinfo: u32, container: Param3, pobjectclassguids: &[*const ::windows_core::GUID], pacl: *const super::ACL, pfnarray: *const FN_OBJECT_MGR_FUNCTIONS, pgenericmapping: *const super::GENERIC_MAPPING, pinheritarray: *mut INHERITED_FROMW) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetInheritanceSourceW(pobjectname: ::windows_core::PCWSTR, objecttype: SE_OBJECT_TYPE, securityinfo: u32, container: ::win32_foundation::BOOL, pobjectclassguids: *const *const ::windows_core::GUID, guidcount: u32, pacl: *const super::ACL, pfnarray: *const FN_OBJECT_MGR_FUNCTIONS, pgenericmapping: *const super::GENERIC_MAPPING, pinheritarray: *mut INHERITED_FROMW) -> u32;
        }
        ::core::mem::transmute(GetInheritanceSourceW(pobjectname.into_param().abi(), ::core::mem::transmute(objecttype), ::core::mem::transmute(securityinfo), container.into_param().abi(), ::core::mem::transmute(::windows_core::as_ptr_or_null(pobjectclassguids)), pobjectclassguids.len() as _, ::core::mem::transmute(pacl), ::core::mem::transmute(pfnarray), ::core::mem::transmute(pgenericmapping), ::core::mem::transmute(pinheritarray)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn GetMultipleTrusteeA(ptrustee: *const TRUSTEE_A) -> *mut TRUSTEE_A {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetMultipleTrusteeA(ptrustee: *const TRUSTEE_A) -> *mut TRUSTEE_A;
        }
        ::core::mem::transmute(GetMultipleTrusteeA(::core::mem::transmute(ptrustee)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn GetMultipleTrusteeOperationA(ptrustee: *const TRUSTEE_A) -> MULTIPLE_TRUSTEE_OPERATION {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetMultipleTrusteeOperationA(ptrustee: *const TRUSTEE_A) -> MULTIPLE_TRUSTEE_OPERATION;
        }
        ::core::mem::transmute(GetMultipleTrusteeOperationA(::core::mem::transmute(ptrustee)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn GetMultipleTrusteeOperationW(ptrustee: *const TRUSTEE_W) -> MULTIPLE_TRUSTEE_OPERATION {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetMultipleTrusteeOperationW(ptrustee: *const TRUSTEE_W) -> MULTIPLE_TRUSTEE_OPERATION;
        }
        ::core::mem::transmute(GetMultipleTrusteeOperationW(::core::mem::transmute(ptrustee)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn GetMultipleTrusteeW(ptrustee: *const TRUSTEE_W) -> *mut TRUSTEE_W {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetMultipleTrusteeW(ptrustee: *const TRUSTEE_W) -> *mut TRUSTEE_W;
        }
        ::core::mem::transmute(GetMultipleTrusteeW(::core::mem::transmute(ptrustee)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn GetNamedSecurityInfoA<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCSTR>>(pobjectname: Param0, objecttype: SE_OBJECT_TYPE, securityinfo: super::OBJECT_SECURITY_INFORMATION, ppsidowner: *mut ::win32_foundation::PSID, ppsidgroup: *mut ::win32_foundation::PSID, ppdacl: *mut *mut super::ACL, ppsacl: *mut *mut super::ACL, ppsecuritydescriptor: *mut super::PSECURITY_DESCRIPTOR) -> ::win32_foundation::WIN32_ERROR {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetNamedSecurityInfoA(pobjectname: ::windows_core::PCSTR, objecttype: SE_OBJECT_TYPE, securityinfo: super::OBJECT_SECURITY_INFORMATION, ppsidowner: *mut ::win32_foundation::PSID, ppsidgroup: *mut ::win32_foundation::PSID, ppdacl: *mut *mut super::ACL, ppsacl: *mut *mut super::ACL, ppsecuritydescriptor: *mut super::PSECURITY_DESCRIPTOR) -> ::win32_foundation::WIN32_ERROR;
        }
        ::core::mem::transmute(GetNamedSecurityInfoA(pobjectname.into_param().abi(), ::core::mem::transmute(objecttype), ::core::mem::transmute(securityinfo), ::core::mem::transmute(ppsidowner), ::core::mem::transmute(ppsidgroup), ::core::mem::transmute(ppdacl), ::core::mem::transmute(ppsacl), ::core::mem::transmute(ppsecuritydescriptor)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn GetNamedSecurityInfoW<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(pobjectname: Param0, objecttype: SE_OBJECT_TYPE, securityinfo: super::OBJECT_SECURITY_INFORMATION, ppsidowner: *mut ::win32_foundation::PSID, ppsidgroup: *mut ::win32_foundation::PSID, ppdacl: *mut *mut super::ACL, ppsacl: *mut *mut super::ACL, ppsecuritydescriptor: *mut super::PSECURITY_DESCRIPTOR) -> ::win32_foundation::WIN32_ERROR {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetNamedSecurityInfoW(pobjectname: ::windows_core::PCWSTR, objecttype: SE_OBJECT_TYPE, securityinfo: super::OBJECT_SECURITY_INFORMATION, ppsidowner: *mut ::win32_foundation::PSID, ppsidgroup: *mut ::win32_foundation::PSID, ppdacl: *mut *mut super::ACL, ppsacl: *mut *mut super::ACL, ppsecuritydescriptor: *mut super::PSECURITY_DESCRIPTOR) -> ::win32_foundation::WIN32_ERROR;
        }
        ::core::mem::transmute(GetNamedSecurityInfoW(pobjectname.into_param().abi(), ::core::mem::transmute(objecttype), ::core::mem::transmute(securityinfo), ::core::mem::transmute(ppsidowner), ::core::mem::transmute(ppsidgroup), ::core::mem::transmute(ppdacl), ::core::mem::transmute(ppsacl), ::core::mem::transmute(ppsecuritydescriptor)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn GetSecurityInfo<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HANDLE>>(handle: Param0, objecttype: SE_OBJECT_TYPE, securityinfo: u32, ppsidowner: *mut ::win32_foundation::PSID, ppsidgroup: *mut ::win32_foundation::PSID, ppdacl: *mut *mut super::ACL, ppsacl: *mut *mut super::ACL, ppsecuritydescriptor: *mut super::PSECURITY_DESCRIPTOR) -> ::win32_foundation::WIN32_ERROR {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetSecurityInfo(handle: ::win32_foundation::HANDLE, objecttype: SE_OBJECT_TYPE, securityinfo: u32, ppsidowner: *mut ::win32_foundation::PSID, ppsidgroup: *mut ::win32_foundation::PSID, ppdacl: *mut *mut super::ACL, ppsacl: *mut *mut super::ACL, ppsecuritydescriptor: *mut super::PSECURITY_DESCRIPTOR) -> ::win32_foundation::WIN32_ERROR;
        }
        ::core::mem::transmute(GetSecurityInfo(handle.into_param().abi(), ::core::mem::transmute(objecttype), ::core::mem::transmute(securityinfo), ::core::mem::transmute(ppsidowner), ::core::mem::transmute(ppsidgroup), ::core::mem::transmute(ppdacl), ::core::mem::transmute(ppsacl), ::core::mem::transmute(ppsecuritydescriptor)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn GetTrusteeFormA(ptrustee: *const TRUSTEE_A) -> TRUSTEE_FORM {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetTrusteeFormA(ptrustee: *const TRUSTEE_A) -> TRUSTEE_FORM;
        }
        ::core::mem::transmute(GetTrusteeFormA(::core::mem::transmute(ptrustee)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn GetTrusteeFormW(ptrustee: *const TRUSTEE_W) -> TRUSTEE_FORM {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetTrusteeFormW(ptrustee: *const TRUSTEE_W) -> TRUSTEE_FORM;
        }
        ::core::mem::transmute(GetTrusteeFormW(::core::mem::transmute(ptrustee)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn GetTrusteeNameA(ptrustee: *const TRUSTEE_A) -> ::windows_core::PSTR {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetTrusteeNameA(ptrustee: *const TRUSTEE_A) -> ::windows_core::PSTR;
        }
        ::core::mem::transmute(GetTrusteeNameA(::core::mem::transmute(ptrustee)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn GetTrusteeNameW(ptrustee: *const TRUSTEE_W) -> ::windows_core::PWSTR {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetTrusteeNameW(ptrustee: *const TRUSTEE_W) -> ::windows_core::PWSTR;
        }
        ::core::mem::transmute(GetTrusteeNameW(::core::mem::transmute(ptrustee)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn GetTrusteeTypeA(ptrustee: *const TRUSTEE_A) -> TRUSTEE_TYPE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetTrusteeTypeA(ptrustee: *const TRUSTEE_A) -> TRUSTEE_TYPE;
        }
        ::core::mem::transmute(GetTrusteeTypeA(::core::mem::transmute(ptrustee)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn GetTrusteeTypeW(ptrustee: *const TRUSTEE_W) -> TRUSTEE_TYPE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetTrusteeTypeW(ptrustee: *const TRUSTEE_W) -> TRUSTEE_TYPE;
        }
        ::core::mem::transmute(GetTrusteeTypeW(::core::mem::transmute(ptrustee)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IAzApplication(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IAzApplication {
    pub unsafe fn Name(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).Name)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetName<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrname: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetName)(::windows_core::Interface::as_raw(self), bstrname.into_param().abi()).ok()
    }
    pub unsafe fn Description(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).Description)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetDescription<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrdescription: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetDescription)(::windows_core::Interface::as_raw(self), bstrdescription.into_param().abi()).ok()
    }
    pub unsafe fn ApplicationData(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).ApplicationData)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetApplicationData<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrapplicationdata: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetApplicationData)(::windows_core::Interface::as_raw(self), bstrapplicationdata.into_param().abi()).ok()
    }
    pub unsafe fn AuthzInterfaceClsid(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).AuthzInterfaceClsid)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetAuthzInterfaceClsid<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrprop: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetAuthzInterfaceClsid)(::windows_core::Interface::as_raw(self), bstrprop.into_param().abi()).ok()
    }
    pub unsafe fn Version(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).Version)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetVersion<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrprop: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetVersion)(::windows_core::Interface::as_raw(self), bstrprop.into_param().abi()).ok()
    }
    pub unsafe fn GenerateAudits(&self) -> ::windows_core::Result<::win32_foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::<::win32_foundation::BOOL>::zeroed();
        (::windows_core::Interface::vtable(self).GenerateAudits)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BOOL>(result__)
    }
    pub unsafe fn SetGenerateAudits<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(&self, bprop: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetGenerateAudits)(::windows_core::Interface::as_raw(self), bprop.into_param().abi()).ok()
    }
    pub unsafe fn ApplyStoreSacl(&self) -> ::windows_core::Result<::win32_foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::<::win32_foundation::BOOL>::zeroed();
        (::windows_core::Interface::vtable(self).ApplyStoreSacl)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BOOL>(result__)
    }
    pub unsafe fn SetApplyStoreSacl<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(&self, bprop: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetApplyStoreSacl)(::windows_core::Interface::as_raw(self), bprop.into_param().abi()).ok()
    }
    pub unsafe fn Writable(&self) -> ::windows_core::Result<::win32_foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::<::win32_foundation::BOOL>::zeroed();
        (::windows_core::Interface::vtable(self).Writable)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BOOL>(result__)
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetProperty<'a, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, lpropid: i32, varreserved: Param1) -> ::windows_core::Result<::win32_system::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).GetProperty)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lpropid), varreserved.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_system::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetProperty<'a, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>, Param2: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, lpropid: i32, varprop: Param1, varreserved: Param2) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetProperty)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lpropid), varprop.into_param().abi(), varreserved.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn PolicyAdministrators(&self) -> ::windows_core::Result<::win32_system::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).PolicyAdministrators)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_system::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn PolicyReaders(&self) -> ::windows_core::Result<::win32_system::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).PolicyReaders)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_system::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AddPolicyAdministrator<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, bstradmin: Param0, varreserved: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).AddPolicyAdministrator)(::windows_core::Interface::as_raw(self), bstradmin.into_param().abi(), varreserved.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DeletePolicyAdministrator<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, bstradmin: Param0, varreserved: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).DeletePolicyAdministrator)(::windows_core::Interface::as_raw(self), bstradmin.into_param().abi(), varreserved.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AddPolicyReader<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, bstrreader: Param0, varreserved: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).AddPolicyReader)(::windows_core::Interface::as_raw(self), bstrreader.into_param().abi(), varreserved.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DeletePolicyReader<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, bstrreader: Param0, varreserved: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).DeletePolicyReader)(::windows_core::Interface::as_raw(self), bstrreader.into_param().abi(), varreserved.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Scopes(&self) -> ::windows_core::Result<IAzScopes> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Scopes)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IAzScopes>(result__)
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn OpenScope<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, bstrscopename: Param0, varreserved: Param1) -> ::windows_core::Result<IAzScope> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).OpenScope)(::windows_core::Interface::as_raw(self), bstrscopename.into_param().abi(), varreserved.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IAzScope>(result__)
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn CreateScope<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, bstrscopename: Param0, varreserved: Param1) -> ::windows_core::Result<IAzScope> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).CreateScope)(::windows_core::Interface::as_raw(self), bstrscopename.into_param().abi(), varreserved.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IAzScope>(result__)
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DeleteScope<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, bstrscopename: Param0, varreserved: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).DeleteScope)(::windows_core::Interface::as_raw(self), bstrscopename.into_param().abi(), varreserved.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Operations(&self) -> ::windows_core::Result<IAzOperations> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Operations)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IAzOperations>(result__)
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn OpenOperation<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, bstroperationname: Param0, varreserved: Param1) -> ::windows_core::Result<IAzOperation> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).OpenOperation)(::windows_core::Interface::as_raw(self), bstroperationname.into_param().abi(), varreserved.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IAzOperation>(result__)
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn CreateOperation<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, bstroperationname: Param0, varreserved: Param1) -> ::windows_core::Result<IAzOperation> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).CreateOperation)(::windows_core::Interface::as_raw(self), bstroperationname.into_param().abi(), varreserved.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IAzOperation>(result__)
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DeleteOperation<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, bstroperationname: Param0, varreserved: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).DeleteOperation)(::windows_core::Interface::as_raw(self), bstroperationname.into_param().abi(), varreserved.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Tasks(&self) -> ::windows_core::Result<IAzTasks> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Tasks)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IAzTasks>(result__)
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn OpenTask<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, bstrtaskname: Param0, varreserved: Param1) -> ::windows_core::Result<IAzTask> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).OpenTask)(::windows_core::Interface::as_raw(self), bstrtaskname.into_param().abi(), varreserved.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IAzTask>(result__)
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn CreateTask<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, bstrtaskname: Param0, varreserved: Param1) -> ::windows_core::Result<IAzTask> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).CreateTask)(::windows_core::Interface::as_raw(self), bstrtaskname.into_param().abi(), varreserved.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IAzTask>(result__)
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DeleteTask<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, bstrtaskname: Param0, varreserved: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).DeleteTask)(::windows_core::Interface::as_raw(self), bstrtaskname.into_param().abi(), varreserved.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ApplicationGroups(&self) -> ::windows_core::Result<IAzApplicationGroups> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).ApplicationGroups)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IAzApplicationGroups>(result__)
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn OpenApplicationGroup<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, bstrgroupname: Param0, varreserved: Param1) -> ::windows_core::Result<IAzApplicationGroup> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).OpenApplicationGroup)(::windows_core::Interface::as_raw(self), bstrgroupname.into_param().abi(), varreserved.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IAzApplicationGroup>(result__)
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn CreateApplicationGroup<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, bstrgroupname: Param0, varreserved: Param1) -> ::windows_core::Result<IAzApplicationGroup> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).CreateApplicationGroup)(::windows_core::Interface::as_raw(self), bstrgroupname.into_param().abi(), varreserved.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IAzApplicationGroup>(result__)
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DeleteApplicationGroup<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, bstrgroupname: Param0, varreserved: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).DeleteApplicationGroup)(::windows_core::Interface::as_raw(self), bstrgroupname.into_param().abi(), varreserved.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Roles(&self) -> ::windows_core::Result<IAzRoles> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Roles)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IAzRoles>(result__)
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn OpenRole<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, bstrrolename: Param0, varreserved: Param1) -> ::windows_core::Result<IAzRole> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).OpenRole)(::windows_core::Interface::as_raw(self), bstrrolename.into_param().abi(), varreserved.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IAzRole>(result__)
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn CreateRole<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, bstrrolename: Param0, varreserved: Param1) -> ::windows_core::Result<IAzRole> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).CreateRole)(::windows_core::Interface::as_raw(self), bstrrolename.into_param().abi(), varreserved.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IAzRole>(result__)
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DeleteRole<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, bstrrolename: Param0, varreserved: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).DeleteRole)(::windows_core::Interface::as_raw(self), bstrrolename.into_param().abi(), varreserved.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn InitializeClientContextFromToken<'a, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, ulltokenhandle: u64, varreserved: Param1) -> ::windows_core::Result<IAzClientContext> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).InitializeClientContextFromToken)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(ulltokenhandle), varreserved.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IAzClientContext>(result__)
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AddPropertyItem<'a, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>, Param2: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, lpropid: i32, varprop: Param1, varreserved: Param2) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).AddPropertyItem)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lpropid), varprop.into_param().abi(), varreserved.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DeletePropertyItem<'a, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>, Param2: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, lpropid: i32, varprop: Param1, varreserved: Param2) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).DeletePropertyItem)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lpropid), varprop.into_param().abi(), varreserved.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Submit<'a, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, lflags: i32, varreserved: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Submit)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lflags), varreserved.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn InitializeClientContextFromName<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param2: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, clientname: Param0, domainname: Param1, varreserved: Param2) -> ::windows_core::Result<IAzClientContext> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).InitializeClientContextFromName)(::windows_core::Interface::as_raw(self), clientname.into_param().abi(), domainname.into_param().abi(), varreserved.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IAzClientContext>(result__)
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DelegatedPolicyUsers(&self) -> ::windows_core::Result<::win32_system::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).DelegatedPolicyUsers)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_system::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AddDelegatedPolicyUser<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, bstrdelegatedpolicyuser: Param0, varreserved: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).AddDelegatedPolicyUser)(::windows_core::Interface::as_raw(self), bstrdelegatedpolicyuser.into_param().abi(), varreserved.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DeleteDelegatedPolicyUser<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, bstrdelegatedpolicyuser: Param0, varreserved: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).DeleteDelegatedPolicyUser)(::windows_core::Interface::as_raw(self), bstrdelegatedpolicyuser.into_param().abi(), varreserved.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn InitializeClientContextFromStringSid<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param2: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, sidstring: Param0, loptions: i32, varreserved: Param2) -> ::windows_core::Result<IAzClientContext> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).InitializeClientContextFromStringSid)(::windows_core::Interface::as_raw(self), sidstring.into_param().abi(), ::core::mem::transmute(loptions), varreserved.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IAzClientContext>(result__)
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn PolicyAdministratorsName(&self) -> ::windows_core::Result<::win32_system::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).PolicyAdministratorsName)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_system::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn PolicyReadersName(&self) -> ::windows_core::Result<::win32_system::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).PolicyReadersName)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_system::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AddPolicyAdministratorName<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, bstradmin: Param0, varreserved: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).AddPolicyAdministratorName)(::windows_core::Interface::as_raw(self), bstradmin.into_param().abi(), varreserved.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DeletePolicyAdministratorName<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, bstradmin: Param0, varreserved: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).DeletePolicyAdministratorName)(::windows_core::Interface::as_raw(self), bstradmin.into_param().abi(), varreserved.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AddPolicyReaderName<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, bstrreader: Param0, varreserved: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).AddPolicyReaderName)(::windows_core::Interface::as_raw(self), bstrreader.into_param().abi(), varreserved.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DeletePolicyReaderName<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, bstrreader: Param0, varreserved: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).DeletePolicyReaderName)(::windows_core::Interface::as_raw(self), bstrreader.into_param().abi(), varreserved.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DelegatedPolicyUsersName(&self) -> ::windows_core::Result<::win32_system::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).DelegatedPolicyUsersName)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_system::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AddDelegatedPolicyUserName<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, bstrdelegatedpolicyuser: Param0, varreserved: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).AddDelegatedPolicyUserName)(::windows_core::Interface::as_raw(self), bstrdelegatedpolicyuser.into_param().abi(), varreserved.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DeleteDelegatedPolicyUserName<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, bstrdelegatedpolicyuser: Param0, varreserved: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).DeleteDelegatedPolicyUserName)(::windows_core::Interface::as_raw(self), bstrdelegatedpolicyuser.into_param().abi(), varreserved.into_param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IAzApplication> for ::windows_core::IUnknown {
    fn from(value: IAzApplication) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IAzApplication> for ::windows_core::IUnknown {
    fn from(value: &IAzApplication) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IAzApplication {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IAzApplication {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IAzApplication> for ::win32_system::Com::IDispatch {
    fn from(value: IAzApplication) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IAzApplication> for ::win32_system::Com::IDispatch {
    fn from(value: &IAzApplication) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for IAzApplication {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for &'a IAzApplication {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IAzApplication {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IAzApplication {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IAzApplication {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IAzApplication {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAzApplication").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IAzApplication {
    type Vtable = IAzApplication_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x987bc7c7_b813_4d27_bede_6ba5ae867e95);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IAzApplication_Vtbl {
    pub base__: ::win32_system::Com::IDispatch_Vtbl,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrname: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub Description: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrdescription: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrdescription: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub ApplicationData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrapplicationdata: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SetApplicationData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrapplicationdata: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub AuthzInterfaceClsid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrprop: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SetAuthzInterfaceClsid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrprop: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub Version: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrprop: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SetVersion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrprop: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub GenerateAudits: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbprop: *mut ::win32_foundation::BOOL) -> ::windows_core::HRESULT,
    pub SetGenerateAudits: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bprop: ::win32_foundation::BOOL) -> ::windows_core::HRESULT,
    pub ApplyStoreSacl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbprop: *mut ::win32_foundation::BOOL) -> ::windows_core::HRESULT,
    pub SetApplyStoreSacl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bprop: ::win32_foundation::BOOL) -> ::windows_core::HRESULT,
    pub Writable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfprop: *mut ::win32_foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub GetProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpropid: i32, varreserved: ::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>, pvarprop: *mut ::win32_system::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    GetProperty: usize,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SetProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpropid: i32, varprop: ::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>, varreserved: ::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SetProperty: usize,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub PolicyAdministrators: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvaradmins: *mut ::win32_system::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    PolicyAdministrators: usize,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub PolicyReaders: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvarreaders: *mut ::win32_system::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    PolicyReaders: usize,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub AddPolicyAdministrator: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstradmin: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    AddPolicyAdministrator: usize,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub DeletePolicyAdministrator: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstradmin: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    DeletePolicyAdministrator: usize,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub AddPolicyReader: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrreader: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    AddPolicyReader: usize,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub DeletePolicyReader: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrreader: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    DeletePolicyReader: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Scopes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppscopecollection: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Scopes: usize,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub OpenScope: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrscopename: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>, ppscope: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    OpenScope: usize,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub CreateScope: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrscopename: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>, ppscope: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    CreateScope: usize,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub DeleteScope: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrscopename: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    DeleteScope: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Operations: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppoperationcollection: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Operations: usize,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub OpenOperation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstroperationname: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>, ppoperation: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    OpenOperation: usize,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub CreateOperation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstroperationname: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>, ppoperation: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    CreateOperation: usize,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub DeleteOperation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstroperationname: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    DeleteOperation: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Tasks: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pptaskcollection: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Tasks: usize,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub OpenTask: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrtaskname: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>, pptask: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    OpenTask: usize,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub CreateTask: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrtaskname: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>, pptask: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    CreateTask: usize,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub DeleteTask: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrtaskname: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    DeleteTask: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub ApplicationGroups: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppgroupcollection: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ApplicationGroups: usize,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub OpenApplicationGroup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrgroupname: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>, ppgroup: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    OpenApplicationGroup: usize,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub CreateApplicationGroup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrgroupname: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>, ppgroup: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    CreateApplicationGroup: usize,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub DeleteApplicationGroup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrgroupname: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    DeleteApplicationGroup: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Roles: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pprolecollection: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Roles: usize,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub OpenRole: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrrolename: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>, pprole: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    OpenRole: usize,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub CreateRole: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrrolename: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>, pprole: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    CreateRole: usize,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub DeleteRole: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrrolename: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    DeleteRole: usize,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub InitializeClientContextFromToken: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulltokenhandle: u64, varreserved: ::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>, ppclientcontext: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    InitializeClientContextFromToken: usize,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub AddPropertyItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpropid: i32, varprop: ::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>, varreserved: ::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    AddPropertyItem: usize,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub DeletePropertyItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpropid: i32, varprop: ::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>, varreserved: ::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    DeletePropertyItem: usize,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Submit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lflags: i32, varreserved: ::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Submit: usize,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub InitializeClientContextFromName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, clientname: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, domainname: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>, ppclientcontext: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    InitializeClientContextFromName: usize,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub DelegatedPolicyUsers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvardelegatedpolicyusers: *mut ::win32_system::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    DelegatedPolicyUsers: usize,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub AddDelegatedPolicyUser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrdelegatedpolicyuser: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    AddDelegatedPolicyUser: usize,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub DeleteDelegatedPolicyUser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrdelegatedpolicyuser: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    DeleteDelegatedPolicyUser: usize,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub InitializeClientContextFromStringSid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sidstring: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, loptions: i32, varreserved: ::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>, ppclientcontext: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    InitializeClientContextFromStringSid: usize,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub PolicyAdministratorsName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvaradmins: *mut ::win32_system::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    PolicyAdministratorsName: usize,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub PolicyReadersName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvarreaders: *mut ::win32_system::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    PolicyReadersName: usize,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub AddPolicyAdministratorName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstradmin: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    AddPolicyAdministratorName: usize,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub DeletePolicyAdministratorName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstradmin: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    DeletePolicyAdministratorName: usize,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub AddPolicyReaderName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrreader: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    AddPolicyReaderName: usize,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub DeletePolicyReaderName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrreader: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    DeletePolicyReaderName: usize,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub DelegatedPolicyUsersName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvardelegatedpolicyusers: *mut ::win32_system::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    DelegatedPolicyUsersName: usize,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub AddDelegatedPolicyUserName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrdelegatedpolicyuser: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    AddDelegatedPolicyUserName: usize,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub DeleteDelegatedPolicyUserName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrdelegatedpolicyuser: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    DeleteDelegatedPolicyUserName: usize,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IAzApplication2(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IAzApplication2 {
    pub unsafe fn Name(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.Name)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetName<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrname: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetName)(::windows_core::Interface::as_raw(self), bstrname.into_param().abi()).ok()
    }
    pub unsafe fn Description(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.Description)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetDescription<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrdescription: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetDescription)(::windows_core::Interface::as_raw(self), bstrdescription.into_param().abi()).ok()
    }
    pub unsafe fn ApplicationData(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.ApplicationData)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetApplicationData<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrapplicationdata: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetApplicationData)(::windows_core::Interface::as_raw(self), bstrapplicationdata.into_param().abi()).ok()
    }
    pub unsafe fn AuthzInterfaceClsid(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.AuthzInterfaceClsid)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetAuthzInterfaceClsid<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrprop: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetAuthzInterfaceClsid)(::windows_core::Interface::as_raw(self), bstrprop.into_param().abi()).ok()
    }
    pub unsafe fn Version(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.Version)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetVersion<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrprop: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetVersion)(::windows_core::Interface::as_raw(self), bstrprop.into_param().abi()).ok()
    }
    pub unsafe fn GenerateAudits(&self) -> ::windows_core::Result<::win32_foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::<::win32_foundation::BOOL>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GenerateAudits)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BOOL>(result__)
    }
    pub unsafe fn SetGenerateAudits<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(&self, bprop: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetGenerateAudits)(::windows_core::Interface::as_raw(self), bprop.into_param().abi()).ok()
    }
    pub unsafe fn ApplyStoreSacl(&self) -> ::windows_core::Result<::win32_foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::<::win32_foundation::BOOL>::zeroed();
        (::windows_core::Interface::vtable(self).base__.ApplyStoreSacl)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BOOL>(result__)
    }
    pub unsafe fn SetApplyStoreSacl<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(&self, bprop: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetApplyStoreSacl)(::windows_core::Interface::as_raw(self), bprop.into_param().abi()).ok()
    }
    pub unsafe fn Writable(&self) -> ::windows_core::Result<::win32_foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::<::win32_foundation::BOOL>::zeroed();
        (::windows_core::Interface::vtable(self).base__.Writable)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BOOL>(result__)
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetProperty<'a, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, lpropid: i32, varreserved: Param1) -> ::windows_core::Result<::win32_system::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetProperty)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lpropid), varreserved.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_system::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetProperty<'a, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>, Param2: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, lpropid: i32, varprop: Param1, varreserved: Param2) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetProperty)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lpropid), varprop.into_param().abi(), varreserved.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn PolicyAdministrators(&self) -> ::windows_core::Result<::win32_system::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.PolicyAdministrators)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_system::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn PolicyReaders(&self) -> ::windows_core::Result<::win32_system::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.PolicyReaders)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_system::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AddPolicyAdministrator<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, bstradmin: Param0, varreserved: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.AddPolicyAdministrator)(::windows_core::Interface::as_raw(self), bstradmin.into_param().abi(), varreserved.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DeletePolicyAdministrator<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, bstradmin: Param0, varreserved: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.DeletePolicyAdministrator)(::windows_core::Interface::as_raw(self), bstradmin.into_param().abi(), varreserved.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AddPolicyReader<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, bstrreader: Param0, varreserved: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.AddPolicyReader)(::windows_core::Interface::as_raw(self), bstrreader.into_param().abi(), varreserved.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DeletePolicyReader<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, bstrreader: Param0, varreserved: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.DeletePolicyReader)(::windows_core::Interface::as_raw(self), bstrreader.into_param().abi(), varreserved.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Scopes(&self) -> ::windows_core::Result<IAzScopes> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.Scopes)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IAzScopes>(result__)
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn OpenScope<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, bstrscopename: Param0, varreserved: Param1) -> ::windows_core::Result<IAzScope> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.OpenScope)(::windows_core::Interface::as_raw(self), bstrscopename.into_param().abi(), varreserved.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IAzScope>(result__)
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn CreateScope<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, bstrscopename: Param0, varreserved: Param1) -> ::windows_core::Result<IAzScope> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.CreateScope)(::windows_core::Interface::as_raw(self), bstrscopename.into_param().abi(), varreserved.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IAzScope>(result__)
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DeleteScope<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, bstrscopename: Param0, varreserved: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.DeleteScope)(::windows_core::Interface::as_raw(self), bstrscopename.into_param().abi(), varreserved.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Operations(&self) -> ::windows_core::Result<IAzOperations> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.Operations)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IAzOperations>(result__)
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn OpenOperation<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, bstroperationname: Param0, varreserved: Param1) -> ::windows_core::Result<IAzOperation> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.OpenOperation)(::windows_core::Interface::as_raw(self), bstroperationname.into_param().abi(), varreserved.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IAzOperation>(result__)
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn CreateOperation<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, bstroperationname: Param0, varreserved: Param1) -> ::windows_core::Result<IAzOperation> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.CreateOperation)(::windows_core::Interface::as_raw(self), bstroperationname.into_param().abi(), varreserved.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IAzOperation>(result__)
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DeleteOperation<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, bstroperationname: Param0, varreserved: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.DeleteOperation)(::windows_core::Interface::as_raw(self), bstroperationname.into_param().abi(), varreserved.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Tasks(&self) -> ::windows_core::Result<IAzTasks> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.Tasks)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IAzTasks>(result__)
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn OpenTask<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, bstrtaskname: Param0, varreserved: Param1) -> ::windows_core::Result<IAzTask> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.OpenTask)(::windows_core::Interface::as_raw(self), bstrtaskname.into_param().abi(), varreserved.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IAzTask>(result__)
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn CreateTask<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, bstrtaskname: Param0, varreserved: Param1) -> ::windows_core::Result<IAzTask> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.CreateTask)(::windows_core::Interface::as_raw(self), bstrtaskname.into_param().abi(), varreserved.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IAzTask>(result__)
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DeleteTask<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, bstrtaskname: Param0, varreserved: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.DeleteTask)(::windows_core::Interface::as_raw(self), bstrtaskname.into_param().abi(), varreserved.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ApplicationGroups(&self) -> ::windows_core::Result<IAzApplicationGroups> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.ApplicationGroups)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IAzApplicationGroups>(result__)
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn OpenApplicationGroup<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, bstrgroupname: Param0, varreserved: Param1) -> ::windows_core::Result<IAzApplicationGroup> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.OpenApplicationGroup)(::windows_core::Interface::as_raw(self), bstrgroupname.into_param().abi(), varreserved.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IAzApplicationGroup>(result__)
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn CreateApplicationGroup<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, bstrgroupname: Param0, varreserved: Param1) -> ::windows_core::Result<IAzApplicationGroup> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.CreateApplicationGroup)(::windows_core::Interface::as_raw(self), bstrgroupname.into_param().abi(), varreserved.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IAzApplicationGroup>(result__)
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DeleteApplicationGroup<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, bstrgroupname: Param0, varreserved: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.DeleteApplicationGroup)(::windows_core::Interface::as_raw(self), bstrgroupname.into_param().abi(), varreserved.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Roles(&self) -> ::windows_core::Result<IAzRoles> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.Roles)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IAzRoles>(result__)
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn OpenRole<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, bstrrolename: Param0, varreserved: Param1) -> ::windows_core::Result<IAzRole> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.OpenRole)(::windows_core::Interface::as_raw(self), bstrrolename.into_param().abi(), varreserved.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IAzRole>(result__)
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn CreateRole<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, bstrrolename: Param0, varreserved: Param1) -> ::windows_core::Result<IAzRole> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.CreateRole)(::windows_core::Interface::as_raw(self), bstrrolename.into_param().abi(), varreserved.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IAzRole>(result__)
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DeleteRole<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, bstrrolename: Param0, varreserved: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.DeleteRole)(::windows_core::Interface::as_raw(self), bstrrolename.into_param().abi(), varreserved.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn InitializeClientContextFromToken<'a, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, ulltokenhandle: u64, varreserved: Param1) -> ::windows_core::Result<IAzClientContext> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.InitializeClientContextFromToken)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(ulltokenhandle), varreserved.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IAzClientContext>(result__)
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AddPropertyItem<'a, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>, Param2: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, lpropid: i32, varprop: Param1, varreserved: Param2) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.AddPropertyItem)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lpropid), varprop.into_param().abi(), varreserved.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DeletePropertyItem<'a, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>, Param2: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, lpropid: i32, varprop: Param1, varreserved: Param2) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.DeletePropertyItem)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lpropid), varprop.into_param().abi(), varreserved.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Submit<'a, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, lflags: i32, varreserved: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Submit)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lflags), varreserved.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn InitializeClientContextFromName<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param2: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, clientname: Param0, domainname: Param1, varreserved: Param2) -> ::windows_core::Result<IAzClientContext> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.InitializeClientContextFromName)(::windows_core::Interface::as_raw(self), clientname.into_param().abi(), domainname.into_param().abi(), varreserved.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IAzClientContext>(result__)
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DelegatedPolicyUsers(&self) -> ::windows_core::Result<::win32_system::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.DelegatedPolicyUsers)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_system::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AddDelegatedPolicyUser<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, bstrdelegatedpolicyuser: Param0, varreserved: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.AddDelegatedPolicyUser)(::windows_core::Interface::as_raw(self), bstrdelegatedpolicyuser.into_param().abi(), varreserved.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DeleteDelegatedPolicyUser<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, bstrdelegatedpolicyuser: Param0, varreserved: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.DeleteDelegatedPolicyUser)(::windows_core::Interface::as_raw(self), bstrdelegatedpolicyuser.into_param().abi(), varreserved.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn InitializeClientContextFromStringSid<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param2: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, sidstring: Param0, loptions: i32, varreserved: Param2) -> ::windows_core::Result<IAzClientContext> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.InitializeClientContextFromStringSid)(::windows_core::Interface::as_raw(self), sidstring.into_param().abi(), ::core::mem::transmute(loptions), varreserved.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IAzClientContext>(result__)
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn PolicyAdministratorsName(&self) -> ::windows_core::Result<::win32_system::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.PolicyAdministratorsName)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_system::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn PolicyReadersName(&self) -> ::windows_core::Result<::win32_system::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.PolicyReadersName)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_system::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AddPolicyAdministratorName<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, bstradmin: Param0, varreserved: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.AddPolicyAdministratorName)(::windows_core::Interface::as_raw(self), bstradmin.into_param().abi(), varreserved.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DeletePolicyAdministratorName<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, bstradmin: Param0, varreserved: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.DeletePolicyAdministratorName)(::windows_core::Interface::as_raw(self), bstradmin.into_param().abi(), varreserved.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AddPolicyReaderName<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, bstrreader: Param0, varreserved: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.AddPolicyReaderName)(::windows_core::Interface::as_raw(self), bstrreader.into_param().abi(), varreserved.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DeletePolicyReaderName<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, bstrreader: Param0, varreserved: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.DeletePolicyReaderName)(::windows_core::Interface::as_raw(self), bstrreader.into_param().abi(), varreserved.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DelegatedPolicyUsersName(&self) -> ::windows_core::Result<::win32_system::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.DelegatedPolicyUsersName)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_system::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AddDelegatedPolicyUserName<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, bstrdelegatedpolicyuser: Param0, varreserved: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.AddDelegatedPolicyUserName)(::windows_core::Interface::as_raw(self), bstrdelegatedpolicyuser.into_param().abi(), varreserved.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DeleteDelegatedPolicyUserName<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, bstrdelegatedpolicyuser: Param0, varreserved: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.DeleteDelegatedPolicyUserName)(::windows_core::Interface::as_raw(self), bstrdelegatedpolicyuser.into_param().abi(), varreserved.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn InitializeClientContextFromToken2<'a, Param2: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, ultokenhandlelowpart: u32, ultokenhandlehighpart: u32, varreserved: Param2) -> ::windows_core::Result<IAzClientContext2> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).InitializeClientContextFromToken2)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(ultokenhandlelowpart), ::core::mem::transmute(ultokenhandlehighpart), varreserved.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IAzClientContext2>(result__)
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn InitializeClientContext2<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, identifyingstring: Param0, varreserved: Param1) -> ::windows_core::Result<IAzClientContext2> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).InitializeClientContext2)(::windows_core::Interface::as_raw(self), identifyingstring.into_param().abi(), varreserved.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IAzClientContext2>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IAzApplication2> for ::windows_core::IUnknown {
    fn from(value: IAzApplication2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IAzApplication2> for ::windows_core::IUnknown {
    fn from(value: &IAzApplication2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IAzApplication2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IAzApplication2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IAzApplication2> for ::win32_system::Com::IDispatch {
    fn from(value: IAzApplication2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IAzApplication2> for ::win32_system::Com::IDispatch {
    fn from(value: &IAzApplication2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for IAzApplication2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for &'a IAzApplication2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IAzApplication2> for IAzApplication {
    fn from(value: IAzApplication2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IAzApplication2> for IAzApplication {
    fn from(value: &IAzApplication2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, IAzApplication> for IAzApplication2 {
    fn into_param(self) -> ::windows_core::Param<'a, IAzApplication> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, IAzApplication> for &'a IAzApplication2 {
    fn into_param(self) -> ::windows_core::Param<'a, IAzApplication> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IAzApplication2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IAzApplication2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IAzApplication2 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IAzApplication2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAzApplication2").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IAzApplication2 {
    type Vtable = IAzApplication2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x086a68af_a249_437c_b18d_d4d86d6a9660);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IAzApplication2_Vtbl {
    pub base__: IAzApplication_Vtbl,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub InitializeClientContextFromToken2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ultokenhandlelowpart: u32, ultokenhandlehighpart: u32, varreserved: ::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>, ppclientcontext: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    InitializeClientContextFromToken2: usize,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub InitializeClientContext2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, identifyingstring: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>, ppclientcontext: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    InitializeClientContext2: usize,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IAzApplication3(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IAzApplication3 {
    pub unsafe fn Name(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.Name)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetName<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrname: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.SetName)(::windows_core::Interface::as_raw(self), bstrname.into_param().abi()).ok()
    }
    pub unsafe fn Description(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.Description)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetDescription<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrdescription: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.SetDescription)(::windows_core::Interface::as_raw(self), bstrdescription.into_param().abi()).ok()
    }
    pub unsafe fn ApplicationData(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.ApplicationData)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetApplicationData<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrapplicationdata: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.SetApplicationData)(::windows_core::Interface::as_raw(self), bstrapplicationdata.into_param().abi()).ok()
    }
    pub unsafe fn AuthzInterfaceClsid(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.AuthzInterfaceClsid)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetAuthzInterfaceClsid<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrprop: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.SetAuthzInterfaceClsid)(::windows_core::Interface::as_raw(self), bstrprop.into_param().abi()).ok()
    }
    pub unsafe fn Version(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.Version)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetVersion<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrprop: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.SetVersion)(::windows_core::Interface::as_raw(self), bstrprop.into_param().abi()).ok()
    }
    pub unsafe fn GenerateAudits(&self) -> ::windows_core::Result<::win32_foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::<::win32_foundation::BOOL>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.GenerateAudits)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BOOL>(result__)
    }
    pub unsafe fn SetGenerateAudits<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(&self, bprop: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.SetGenerateAudits)(::windows_core::Interface::as_raw(self), bprop.into_param().abi()).ok()
    }
    pub unsafe fn ApplyStoreSacl(&self) -> ::windows_core::Result<::win32_foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::<::win32_foundation::BOOL>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.ApplyStoreSacl)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BOOL>(result__)
    }
    pub unsafe fn SetApplyStoreSacl<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(&self, bprop: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.SetApplyStoreSacl)(::windows_core::Interface::as_raw(self), bprop.into_param().abi()).ok()
    }
    pub unsafe fn Writable(&self) -> ::windows_core::Result<::win32_foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::<::win32_foundation::BOOL>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.Writable)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BOOL>(result__)
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetProperty<'a, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, lpropid: i32, varreserved: Param1) -> ::windows_core::Result<::win32_system::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.GetProperty)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lpropid), varreserved.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_system::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetProperty<'a, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>, Param2: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, lpropid: i32, varprop: Param1, varreserved: Param2) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.SetProperty)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lpropid), varprop.into_param().abi(), varreserved.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn PolicyAdministrators(&self) -> ::windows_core::Result<::win32_system::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.PolicyAdministrators)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_system::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn PolicyReaders(&self) -> ::windows_core::Result<::win32_system::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.PolicyReaders)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_system::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AddPolicyAdministrator<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, bstradmin: Param0, varreserved: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.AddPolicyAdministrator)(::windows_core::Interface::as_raw(self), bstradmin.into_param().abi(), varreserved.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DeletePolicyAdministrator<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, bstradmin: Param0, varreserved: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.DeletePolicyAdministrator)(::windows_core::Interface::as_raw(self), bstradmin.into_param().abi(), varreserved.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AddPolicyReader<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, bstrreader: Param0, varreserved: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.AddPolicyReader)(::windows_core::Interface::as_raw(self), bstrreader.into_param().abi(), varreserved.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DeletePolicyReader<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, bstrreader: Param0, varreserved: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.DeletePolicyReader)(::windows_core::Interface::as_raw(self), bstrreader.into_param().abi(), varreserved.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Scopes(&self) -> ::windows_core::Result<IAzScopes> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.Scopes)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IAzScopes>(result__)
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn OpenScope<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, bstrscopename: Param0, varreserved: Param1) -> ::windows_core::Result<IAzScope> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.OpenScope)(::windows_core::Interface::as_raw(self), bstrscopename.into_param().abi(), varreserved.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IAzScope>(result__)
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn CreateScope<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, bstrscopename: Param0, varreserved: Param1) -> ::windows_core::Result<IAzScope> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.CreateScope)(::windows_core::Interface::as_raw(self), bstrscopename.into_param().abi(), varreserved.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IAzScope>(result__)
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DeleteScope<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, bstrscopename: Param0, varreserved: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.DeleteScope)(::windows_core::Interface::as_raw(self), bstrscopename.into_param().abi(), varreserved.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Operations(&self) -> ::windows_core::Result<IAzOperations> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.Operations)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IAzOperations>(result__)
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn OpenOperation<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, bstroperationname: Param0, varreserved: Param1) -> ::windows_core::Result<IAzOperation> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.OpenOperation)(::windows_core::Interface::as_raw(self), bstroperationname.into_param().abi(), varreserved.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IAzOperation>(result__)
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn CreateOperation<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, bstroperationname: Param0, varreserved: Param1) -> ::windows_core::Result<IAzOperation> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.CreateOperation)(::windows_core::Interface::as_raw(self), bstroperationname.into_param().abi(), varreserved.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IAzOperation>(result__)
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DeleteOperation<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, bstroperationname: Param0, varreserved: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.DeleteOperation)(::windows_core::Interface::as_raw(self), bstroperationname.into_param().abi(), varreserved.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Tasks(&self) -> ::windows_core::Result<IAzTasks> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.Tasks)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IAzTasks>(result__)
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn OpenTask<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, bstrtaskname: Param0, varreserved: Param1) -> ::windows_core::Result<IAzTask> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.OpenTask)(::windows_core::Interface::as_raw(self), bstrtaskname.into_param().abi(), varreserved.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IAzTask>(result__)
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn CreateTask<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, bstrtaskname: Param0, varreserved: Param1) -> ::windows_core::Result<IAzTask> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.CreateTask)(::windows_core::Interface::as_raw(self), bstrtaskname.into_param().abi(), varreserved.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IAzTask>(result__)
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DeleteTask<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, bstrtaskname: Param0, varreserved: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.DeleteTask)(::windows_core::Interface::as_raw(self), bstrtaskname.into_param().abi(), varreserved.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ApplicationGroups(&self) -> ::windows_core::Result<IAzApplicationGroups> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.ApplicationGroups)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IAzApplicationGroups>(result__)
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn OpenApplicationGroup<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, bstrgroupname: Param0, varreserved: Param1) -> ::windows_core::Result<IAzApplicationGroup> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.OpenApplicationGroup)(::windows_core::Interface::as_raw(self), bstrgroupname.into_param().abi(), varreserved.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IAzApplicationGroup>(result__)
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn CreateApplicationGroup<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, bstrgroupname: Param0, varreserved: Param1) -> ::windows_core::Result<IAzApplicationGroup> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.CreateApplicationGroup)(::windows_core::Interface::as_raw(self), bstrgroupname.into_param().abi(), varreserved.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IAzApplicationGroup>(result__)
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DeleteApplicationGroup<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, bstrgroupname: Param0, varreserved: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.DeleteApplicationGroup)(::windows_core::Interface::as_raw(self), bstrgroupname.into_param().abi(), varreserved.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Roles(&self) -> ::windows_core::Result<IAzRoles> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.Roles)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IAzRoles>(result__)
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn OpenRole<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, bstrrolename: Param0, varreserved: Param1) -> ::windows_core::Result<IAzRole> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.OpenRole)(::windows_core::Interface::as_raw(self), bstrrolename.into_param().abi(), varreserved.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IAzRole>(result__)
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn CreateRole<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, bstrrolename: Param0, varreserved: Param1) -> ::windows_core::Result<IAzRole> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.CreateRole)(::windows_core::Interface::as_raw(self), bstrrolename.into_param().abi(), varreserved.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IAzRole>(result__)
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DeleteRole<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, bstrrolename: Param0, varreserved: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.DeleteRole)(::windows_core::Interface::as_raw(self), bstrrolename.into_param().abi(), varreserved.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn InitializeClientContextFromToken<'a, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, ulltokenhandle: u64, varreserved: Param1) -> ::windows_core::Result<IAzClientContext> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.InitializeClientContextFromToken)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(ulltokenhandle), varreserved.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IAzClientContext>(result__)
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AddPropertyItem<'a, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>, Param2: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, lpropid: i32, varprop: Param1, varreserved: Param2) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.AddPropertyItem)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lpropid), varprop.into_param().abi(), varreserved.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DeletePropertyItem<'a, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>, Param2: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, lpropid: i32, varprop: Param1, varreserved: Param2) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.DeletePropertyItem)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lpropid), varprop.into_param().abi(), varreserved.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Submit<'a, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, lflags: i32, varreserved: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.Submit)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lflags), varreserved.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn InitializeClientContextFromName<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param2: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, clientname: Param0, domainname: Param1, varreserved: Param2) -> ::windows_core::Result<IAzClientContext> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.InitializeClientContextFromName)(::windows_core::Interface::as_raw(self), clientname.into_param().abi(), domainname.into_param().abi(), varreserved.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IAzClientContext>(result__)
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DelegatedPolicyUsers(&self) -> ::windows_core::Result<::win32_system::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.DelegatedPolicyUsers)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_system::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AddDelegatedPolicyUser<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, bstrdelegatedpolicyuser: Param0, varreserved: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.AddDelegatedPolicyUser)(::windows_core::Interface::as_raw(self), bstrdelegatedpolicyuser.into_param().abi(), varreserved.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DeleteDelegatedPolicyUser<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, bstrdelegatedpolicyuser: Param0, varreserved: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.DeleteDelegatedPolicyUser)(::windows_core::Interface::as_raw(self), bstrdelegatedpolicyuser.into_param().abi(), varreserved.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn InitializeClientContextFromStringSid<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param2: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, sidstring: Param0, loptions: i32, varreserved: Param2) -> ::windows_core::Result<IAzClientContext> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.InitializeClientContextFromStringSid)(::windows_core::Interface::as_raw(self), sidstring.into_param().abi(), ::core::mem::transmute(loptions), varreserved.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IAzClientContext>(result__)
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn PolicyAdministratorsName(&self) -> ::windows_core::Result<::win32_system::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.PolicyAdministratorsName)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_system::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn PolicyReadersName(&self) -> ::windows_core::Result<::win32_system::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.PolicyReadersName)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_system::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AddPolicyAdministratorName<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, bstradmin: Param0, varreserved: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.AddPolicyAdministratorName)(::windows_core::Interface::as_raw(self), bstradmin.into_param().abi(), varreserved.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DeletePolicyAdministratorName<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, bstradmin: Param0, varreserved: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.DeletePolicyAdministratorName)(::windows_core::Interface::as_raw(self), bstradmin.into_param().abi(), varreserved.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AddPolicyReaderName<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, bstrreader: Param0, varreserved: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.AddPolicyReaderName)(::windows_core::Interface::as_raw(self), bstrreader.into_param().abi(), varreserved.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DeletePolicyReaderName<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, bstrreader: Param0, varreserved: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.DeletePolicyReaderName)(::windows_core::Interface::as_raw(self), bstrreader.into_param().abi(), varreserved.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DelegatedPolicyUsersName(&self) -> ::windows_core::Result<::win32_system::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.DelegatedPolicyUsersName)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_system::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AddDelegatedPolicyUserName<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, bstrdelegatedpolicyuser: Param0, varreserved: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.AddDelegatedPolicyUserName)(::windows_core::Interface::as_raw(self), bstrdelegatedpolicyuser.into_param().abi(), varreserved.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DeleteDelegatedPolicyUserName<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, bstrdelegatedpolicyuser: Param0, varreserved: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.DeleteDelegatedPolicyUserName)(::windows_core::Interface::as_raw(self), bstrdelegatedpolicyuser.into_param().abi(), varreserved.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn InitializeClientContextFromToken2<'a, Param2: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, ultokenhandlelowpart: u32, ultokenhandlehighpart: u32, varreserved: Param2) -> ::windows_core::Result<IAzClientContext2> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.InitializeClientContextFromToken2)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(ultokenhandlelowpart), ::core::mem::transmute(ultokenhandlehighpart), varreserved.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IAzClientContext2>(result__)
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn InitializeClientContext2<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, identifyingstring: Param0, varreserved: Param1) -> ::windows_core::Result<IAzClientContext2> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.InitializeClientContext2)(::windows_core::Interface::as_raw(self), identifyingstring.into_param().abi(), varreserved.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IAzClientContext2>(result__)
    }
    pub unsafe fn ScopeExists<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrscopename: Param0) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).ScopeExists)(::windows_core::Interface::as_raw(self), bstrscopename.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OpenScope2<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrscopename: Param0) -> ::windows_core::Result<IAzScope2> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).OpenScope2)(::windows_core::Interface::as_raw(self), bstrscopename.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IAzScope2>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateScope2<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrscopename: Param0) -> ::windows_core::Result<IAzScope2> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).CreateScope2)(::windows_core::Interface::as_raw(self), bstrscopename.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IAzScope2>(result__)
    }
    pub unsafe fn DeleteScope2<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrscopename: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).DeleteScope2)(::windows_core::Interface::as_raw(self), bstrscopename.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn RoleDefinitions(&self) -> ::windows_core::Result<IAzRoleDefinitions> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).RoleDefinitions)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IAzRoleDefinitions>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateRoleDefinition<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrroledefinitionname: Param0) -> ::windows_core::Result<IAzRoleDefinition> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).CreateRoleDefinition)(::windows_core::Interface::as_raw(self), bstrroledefinitionname.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IAzRoleDefinition>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OpenRoleDefinition<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrroledefinitionname: Param0) -> ::windows_core::Result<IAzRoleDefinition> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).OpenRoleDefinition)(::windows_core::Interface::as_raw(self), bstrroledefinitionname.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IAzRoleDefinition>(result__)
    }
    pub unsafe fn DeleteRoleDefinition<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrroledefinitionname: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).DeleteRoleDefinition)(::windows_core::Interface::as_raw(self), bstrroledefinitionname.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn RoleAssignments(&self) -> ::windows_core::Result<IAzRoleAssignments> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).RoleAssignments)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IAzRoleAssignments>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateRoleAssignment<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrroleassignmentname: Param0) -> ::windows_core::Result<IAzRoleAssignment> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).CreateRoleAssignment)(::windows_core::Interface::as_raw(self), bstrroleassignmentname.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IAzRoleAssignment>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OpenRoleAssignment<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrroleassignmentname: Param0) -> ::windows_core::Result<IAzRoleAssignment> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).OpenRoleAssignment)(::windows_core::Interface::as_raw(self), bstrroleassignmentname.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IAzRoleAssignment>(result__)
    }
    pub unsafe fn DeleteRoleAssignment<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrroleassignmentname: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).DeleteRoleAssignment)(::windows_core::Interface::as_raw(self), bstrroleassignmentname.into_param().abi()).ok()
    }
    pub unsafe fn BizRulesEnabled(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).BizRulesEnabled)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn SetBizRulesEnabled(&self, benabled: i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetBizRulesEnabled)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(benabled)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IAzApplication3> for ::windows_core::IUnknown {
    fn from(value: IAzApplication3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IAzApplication3> for ::windows_core::IUnknown {
    fn from(value: &IAzApplication3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IAzApplication3 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IAzApplication3 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IAzApplication3> for ::win32_system::Com::IDispatch {
    fn from(value: IAzApplication3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IAzApplication3> for ::win32_system::Com::IDispatch {
    fn from(value: &IAzApplication3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for IAzApplication3 {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for &'a IAzApplication3 {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IAzApplication3> for IAzApplication {
    fn from(value: IAzApplication3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IAzApplication3> for IAzApplication {
    fn from(value: &IAzApplication3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, IAzApplication> for IAzApplication3 {
    fn into_param(self) -> ::windows_core::Param<'a, IAzApplication> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, IAzApplication> for &'a IAzApplication3 {
    fn into_param(self) -> ::windows_core::Param<'a, IAzApplication> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IAzApplication3> for IAzApplication2 {
    fn from(value: IAzApplication3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IAzApplication3> for IAzApplication2 {
    fn from(value: &IAzApplication3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, IAzApplication2> for IAzApplication3 {
    fn into_param(self) -> ::windows_core::Param<'a, IAzApplication2> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, IAzApplication2> for &'a IAzApplication3 {
    fn into_param(self) -> ::windows_core::Param<'a, IAzApplication2> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IAzApplication3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IAzApplication3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IAzApplication3 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IAzApplication3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAzApplication3").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IAzApplication3 {
    type Vtable = IAzApplication3_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x181c845e_7196_4a7d_ac2e_020c0bb7a303);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IAzApplication3_Vtbl {
    pub base__: IAzApplication2_Vtbl,
    pub ScopeExists: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrscopename: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, pbexist: *mut i16) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub OpenScope2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrscopename: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, ppscope2: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    OpenScope2: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateScope2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrscopename: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, ppscope2: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateScope2: usize,
    pub DeleteScope2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrscopename: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub RoleDefinitions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pproledefinitions: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    RoleDefinitions: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateRoleDefinition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrroledefinitionname: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, pproledefinitions: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateRoleDefinition: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub OpenRoleDefinition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrroledefinitionname: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, pproledefinitions: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    OpenRoleDefinition: usize,
    pub DeleteRoleDefinition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrroledefinitionname: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub RoleAssignments: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pproleassignments: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    RoleAssignments: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateRoleAssignment: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrroleassignmentname: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, pproleassignment: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateRoleAssignment: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub OpenRoleAssignment: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrroleassignmentname: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, pproleassignment: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    OpenRoleAssignment: usize,
    pub DeleteRoleAssignment: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrroleassignmentname: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub BizRulesEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbenabled: *mut i16) -> ::windows_core::HRESULT,
    pub SetBizRulesEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, benabled: i16) -> ::windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IAzApplicationGroup(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IAzApplicationGroup {
    pub unsafe fn Name(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).Name)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetName<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrname: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetName)(::windows_core::Interface::as_raw(self), bstrname.into_param().abi()).ok()
    }
    pub unsafe fn Type(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).Type)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetType(&self, lprop: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetType)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lprop)).ok()
    }
    pub unsafe fn LdapQuery(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).LdapQuery)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetLdapQuery<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrprop: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetLdapQuery)(::windows_core::Interface::as_raw(self), bstrprop.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AppMembers(&self) -> ::windows_core::Result<::win32_system::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).AppMembers)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_system::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AppNonMembers(&self) -> ::windows_core::Result<::win32_system::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).AppNonMembers)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_system::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Members(&self) -> ::windows_core::Result<::win32_system::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).Members)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_system::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn NonMembers(&self) -> ::windows_core::Result<::win32_system::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).NonMembers)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_system::Com::VARIANT>(result__)
    }
    pub unsafe fn Description(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).Description)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetDescription<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrdescription: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetDescription)(::windows_core::Interface::as_raw(self), bstrdescription.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AddAppMember<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, bstrprop: Param0, varreserved: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).AddAppMember)(::windows_core::Interface::as_raw(self), bstrprop.into_param().abi(), varreserved.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DeleteAppMember<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, bstrprop: Param0, varreserved: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).DeleteAppMember)(::windows_core::Interface::as_raw(self), bstrprop.into_param().abi(), varreserved.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AddAppNonMember<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, bstrprop: Param0, varreserved: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).AddAppNonMember)(::windows_core::Interface::as_raw(self), bstrprop.into_param().abi(), varreserved.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DeleteAppNonMember<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, bstrprop: Param0, varreserved: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).DeleteAppNonMember)(::windows_core::Interface::as_raw(self), bstrprop.into_param().abi(), varreserved.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AddMember<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, bstrprop: Param0, varreserved: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).AddMember)(::windows_core::Interface::as_raw(self), bstrprop.into_param().abi(), varreserved.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DeleteMember<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, bstrprop: Param0, varreserved: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).DeleteMember)(::windows_core::Interface::as_raw(self), bstrprop.into_param().abi(), varreserved.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AddNonMember<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, bstrprop: Param0, varreserved: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).AddNonMember)(::windows_core::Interface::as_raw(self), bstrprop.into_param().abi(), varreserved.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DeleteNonMember<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, bstrprop: Param0, varreserved: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).DeleteNonMember)(::windows_core::Interface::as_raw(self), bstrprop.into_param().abi(), varreserved.into_param().abi()).ok()
    }
    pub unsafe fn Writable(&self) -> ::windows_core::Result<::win32_foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::<::win32_foundation::BOOL>::zeroed();
        (::windows_core::Interface::vtable(self).Writable)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BOOL>(result__)
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetProperty<'a, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, lpropid: i32, varreserved: Param1) -> ::windows_core::Result<::win32_system::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).GetProperty)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lpropid), varreserved.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_system::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetProperty<'a, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>, Param2: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, lpropid: i32, varprop: Param1, varreserved: Param2) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetProperty)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lpropid), varprop.into_param().abi(), varreserved.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AddPropertyItem<'a, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>, Param2: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, lpropid: i32, varprop: Param1, varreserved: Param2) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).AddPropertyItem)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lpropid), varprop.into_param().abi(), varreserved.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DeletePropertyItem<'a, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>, Param2: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, lpropid: i32, varprop: Param1, varreserved: Param2) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).DeletePropertyItem)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lpropid), varprop.into_param().abi(), varreserved.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Submit<'a, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, lflags: i32, varreserved: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Submit)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lflags), varreserved.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AddMemberName<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, bstrprop: Param0, varreserved: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).AddMemberName)(::windows_core::Interface::as_raw(self), bstrprop.into_param().abi(), varreserved.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DeleteMemberName<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, bstrprop: Param0, varreserved: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).DeleteMemberName)(::windows_core::Interface::as_raw(self), bstrprop.into_param().abi(), varreserved.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AddNonMemberName<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, bstrprop: Param0, varreserved: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).AddNonMemberName)(::windows_core::Interface::as_raw(self), bstrprop.into_param().abi(), varreserved.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DeleteNonMemberName<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, bstrprop: Param0, varreserved: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).DeleteNonMemberName)(::windows_core::Interface::as_raw(self), bstrprop.into_param().abi(), varreserved.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn MembersName(&self) -> ::windows_core::Result<::win32_system::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).MembersName)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_system::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn NonMembersName(&self) -> ::windows_core::Result<::win32_system::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).NonMembersName)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_system::Com::VARIANT>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IAzApplicationGroup> for ::windows_core::IUnknown {
    fn from(value: IAzApplicationGroup) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IAzApplicationGroup> for ::windows_core::IUnknown {
    fn from(value: &IAzApplicationGroup) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IAzApplicationGroup {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IAzApplicationGroup {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IAzApplicationGroup> for ::win32_system::Com::IDispatch {
    fn from(value: IAzApplicationGroup) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IAzApplicationGroup> for ::win32_system::Com::IDispatch {
    fn from(value: &IAzApplicationGroup) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for IAzApplicationGroup {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for &'a IAzApplicationGroup {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IAzApplicationGroup {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IAzApplicationGroup {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IAzApplicationGroup {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IAzApplicationGroup {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAzApplicationGroup").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IAzApplicationGroup {
    type Vtable = IAzApplicationGroup_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf1b744cd_58a6_4e06_9fbf_36f6d779e21e);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IAzApplicationGroup_Vtbl {
    pub base__: ::win32_system::Com::IDispatch_Vtbl,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrname: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub Type: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plprop: *mut i32) -> ::windows_core::HRESULT,
    pub SetType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lprop: i32) -> ::windows_core::HRESULT,
    pub LdapQuery: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrprop: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SetLdapQuery: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrprop: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub AppMembers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvarprop: *mut ::win32_system::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    AppMembers: usize,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub AppNonMembers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvarprop: *mut ::win32_system::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    AppNonMembers: usize,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Members: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvarprop: *mut ::win32_system::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Members: usize,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub NonMembers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvarprop: *mut ::win32_system::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    NonMembers: usize,
    pub Description: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrdescription: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrdescription: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub AddAppMember: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrprop: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    AddAppMember: usize,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub DeleteAppMember: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrprop: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    DeleteAppMember: usize,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub AddAppNonMember: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrprop: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    AddAppNonMember: usize,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub DeleteAppNonMember: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrprop: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    DeleteAppNonMember: usize,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub AddMember: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrprop: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    AddMember: usize,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub DeleteMember: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrprop: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    DeleteMember: usize,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub AddNonMember: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrprop: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    AddNonMember: usize,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub DeleteNonMember: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrprop: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    DeleteNonMember: usize,
    pub Writable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfprop: *mut ::win32_foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub GetProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpropid: i32, varreserved: ::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>, pvarprop: *mut ::win32_system::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    GetProperty: usize,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SetProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpropid: i32, varprop: ::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>, varreserved: ::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SetProperty: usize,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub AddPropertyItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpropid: i32, varprop: ::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>, varreserved: ::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    AddPropertyItem: usize,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub DeletePropertyItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpropid: i32, varprop: ::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>, varreserved: ::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    DeletePropertyItem: usize,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Submit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lflags: i32, varreserved: ::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Submit: usize,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub AddMemberName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrprop: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    AddMemberName: usize,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub DeleteMemberName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrprop: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    DeleteMemberName: usize,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub AddNonMemberName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrprop: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    AddNonMemberName: usize,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub DeleteNonMemberName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrprop: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    DeleteNonMemberName: usize,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub MembersName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvarprop: *mut ::win32_system::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    MembersName: usize,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub NonMembersName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvarprop: *mut ::win32_system::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    NonMembersName: usize,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IAzApplicationGroup2(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IAzApplicationGroup2 {
    pub unsafe fn Name(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.Name)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetName<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrname: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetName)(::windows_core::Interface::as_raw(self), bstrname.into_param().abi()).ok()
    }
    pub unsafe fn Type(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).base__.Type)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetType(&self, lprop: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetType)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lprop)).ok()
    }
    pub unsafe fn LdapQuery(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.LdapQuery)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetLdapQuery<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrprop: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetLdapQuery)(::windows_core::Interface::as_raw(self), bstrprop.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AppMembers(&self) -> ::windows_core::Result<::win32_system::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.AppMembers)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_system::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AppNonMembers(&self) -> ::windows_core::Result<::win32_system::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.AppNonMembers)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_system::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Members(&self) -> ::windows_core::Result<::win32_system::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.Members)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_system::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn NonMembers(&self) -> ::windows_core::Result<::win32_system::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.NonMembers)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_system::Com::VARIANT>(result__)
    }
    pub unsafe fn Description(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.Description)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetDescription<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrdescription: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetDescription)(::windows_core::Interface::as_raw(self), bstrdescription.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AddAppMember<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, bstrprop: Param0, varreserved: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.AddAppMember)(::windows_core::Interface::as_raw(self), bstrprop.into_param().abi(), varreserved.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DeleteAppMember<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, bstrprop: Param0, varreserved: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.DeleteAppMember)(::windows_core::Interface::as_raw(self), bstrprop.into_param().abi(), varreserved.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AddAppNonMember<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, bstrprop: Param0, varreserved: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.AddAppNonMember)(::windows_core::Interface::as_raw(self), bstrprop.into_param().abi(), varreserved.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DeleteAppNonMember<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, bstrprop: Param0, varreserved: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.DeleteAppNonMember)(::windows_core::Interface::as_raw(self), bstrprop.into_param().abi(), varreserved.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AddMember<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, bstrprop: Param0, varreserved: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.AddMember)(::windows_core::Interface::as_raw(self), bstrprop.into_param().abi(), varreserved.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DeleteMember<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, bstrprop: Param0, varreserved: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.DeleteMember)(::windows_core::Interface::as_raw(self), bstrprop.into_param().abi(), varreserved.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AddNonMember<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, bstrprop: Param0, varreserved: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.AddNonMember)(::windows_core::Interface::as_raw(self), bstrprop.into_param().abi(), varreserved.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DeleteNonMember<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, bstrprop: Param0, varreserved: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.DeleteNonMember)(::windows_core::Interface::as_raw(self), bstrprop.into_param().abi(), varreserved.into_param().abi()).ok()
    }
    pub unsafe fn Writable(&self) -> ::windows_core::Result<::win32_foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::<::win32_foundation::BOOL>::zeroed();
        (::windows_core::Interface::vtable(self).base__.Writable)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BOOL>(result__)
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetProperty<'a, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, lpropid: i32, varreserved: Param1) -> ::windows_core::Result<::win32_system::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetProperty)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lpropid), varreserved.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_system::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetProperty<'a, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>, Param2: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, lpropid: i32, varprop: Param1, varreserved: Param2) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetProperty)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lpropid), varprop.into_param().abi(), varreserved.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AddPropertyItem<'a, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>, Param2: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, lpropid: i32, varprop: Param1, varreserved: Param2) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.AddPropertyItem)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lpropid), varprop.into_param().abi(), varreserved.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DeletePropertyItem<'a, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>, Param2: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, lpropid: i32, varprop: Param1, varreserved: Param2) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.DeletePropertyItem)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lpropid), varprop.into_param().abi(), varreserved.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Submit<'a, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, lflags: i32, varreserved: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Submit)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lflags), varreserved.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AddMemberName<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, bstrprop: Param0, varreserved: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.AddMemberName)(::windows_core::Interface::as_raw(self), bstrprop.into_param().abi(), varreserved.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DeleteMemberName<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, bstrprop: Param0, varreserved: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.DeleteMemberName)(::windows_core::Interface::as_raw(self), bstrprop.into_param().abi(), varreserved.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AddNonMemberName<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, bstrprop: Param0, varreserved: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.AddNonMemberName)(::windows_core::Interface::as_raw(self), bstrprop.into_param().abi(), varreserved.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DeleteNonMemberName<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, bstrprop: Param0, varreserved: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.DeleteNonMemberName)(::windows_core::Interface::as_raw(self), bstrprop.into_param().abi(), varreserved.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn MembersName(&self) -> ::windows_core::Result<::win32_system::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.MembersName)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_system::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn NonMembersName(&self) -> ::windows_core::Result<::win32_system::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.NonMembersName)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_system::Com::VARIANT>(result__)
    }
    pub unsafe fn BizRule(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).BizRule)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetBizRule<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrprop: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetBizRule)(::windows_core::Interface::as_raw(self), bstrprop.into_param().abi()).ok()
    }
    pub unsafe fn BizRuleLanguage(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).BizRuleLanguage)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetBizRuleLanguage<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrprop: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetBizRuleLanguage)(::windows_core::Interface::as_raw(self), bstrprop.into_param().abi()).ok()
    }
    pub unsafe fn BizRuleImportedPath(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).BizRuleImportedPath)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetBizRuleImportedPath<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrprop: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetBizRuleImportedPath)(::windows_core::Interface::as_raw(self), bstrprop.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn RoleAssignments<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrscopename: Param0, brecursive: i16) -> ::windows_core::Result<IAzRoleAssignments> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).RoleAssignments)(::windows_core::Interface::as_raw(self), bstrscopename.into_param().abi(), ::core::mem::transmute(brecursive), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IAzRoleAssignments>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IAzApplicationGroup2> for ::windows_core::IUnknown {
    fn from(value: IAzApplicationGroup2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IAzApplicationGroup2> for ::windows_core::IUnknown {
    fn from(value: &IAzApplicationGroup2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IAzApplicationGroup2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IAzApplicationGroup2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IAzApplicationGroup2> for ::win32_system::Com::IDispatch {
    fn from(value: IAzApplicationGroup2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IAzApplicationGroup2> for ::win32_system::Com::IDispatch {
    fn from(value: &IAzApplicationGroup2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for IAzApplicationGroup2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for &'a IAzApplicationGroup2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IAzApplicationGroup2> for IAzApplicationGroup {
    fn from(value: IAzApplicationGroup2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IAzApplicationGroup2> for IAzApplicationGroup {
    fn from(value: &IAzApplicationGroup2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, IAzApplicationGroup> for IAzApplicationGroup2 {
    fn into_param(self) -> ::windows_core::Param<'a, IAzApplicationGroup> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, IAzApplicationGroup> for &'a IAzApplicationGroup2 {
    fn into_param(self) -> ::windows_core::Param<'a, IAzApplicationGroup> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IAzApplicationGroup2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IAzApplicationGroup2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IAzApplicationGroup2 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IAzApplicationGroup2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAzApplicationGroup2").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IAzApplicationGroup2 {
    type Vtable = IAzApplicationGroup2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3f0613fc_b71a_464e_a11d_5b881a56cefa);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IAzApplicationGroup2_Vtbl {
    pub base__: IAzApplicationGroup_Vtbl,
    pub BizRule: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrprop: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SetBizRule: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrprop: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub BizRuleLanguage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrprop: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SetBizRuleLanguage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrprop: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub BizRuleImportedPath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrprop: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SetBizRuleImportedPath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrprop: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub RoleAssignments: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrscopename: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, brecursive: i16, pproleassignments: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    RoleAssignments: usize,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IAzApplicationGroups(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IAzApplicationGroups {
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn get_Item(&self, index: i32) -> ::windows_core::Result<::win32_system::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).get_Item)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(index), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_system::Com::VARIANT>(result__)
    }
    pub unsafe fn Count(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).Count)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn _NewEnum(&self) -> ::windows_core::Result<::windows_core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
        (::windows_core::Interface::vtable(self)._NewEnum)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::IUnknown>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IAzApplicationGroups> for ::windows_core::IUnknown {
    fn from(value: IAzApplicationGroups) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IAzApplicationGroups> for ::windows_core::IUnknown {
    fn from(value: &IAzApplicationGroups) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IAzApplicationGroups {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IAzApplicationGroups {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IAzApplicationGroups> for ::win32_system::Com::IDispatch {
    fn from(value: IAzApplicationGroups) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IAzApplicationGroups> for ::win32_system::Com::IDispatch {
    fn from(value: &IAzApplicationGroups) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for IAzApplicationGroups {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for &'a IAzApplicationGroups {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IAzApplicationGroups {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IAzApplicationGroups {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IAzApplicationGroups {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IAzApplicationGroups {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAzApplicationGroups").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IAzApplicationGroups {
    type Vtable = IAzApplicationGroups_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4ce66ad5_9f3c_469d_a911_b99887a7e685);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IAzApplicationGroups_Vtbl {
    pub base__: ::win32_system::Com::IDispatch_Vtbl,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub get_Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: i32, pvarobtptr: *mut ::win32_system::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    get_Item: usize,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows_core::HRESULT,
    pub _NewEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenumptr: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IAzApplications(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IAzApplications {
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn get_Item(&self, index: i32) -> ::windows_core::Result<::win32_system::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).get_Item)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(index), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_system::Com::VARIANT>(result__)
    }
    pub unsafe fn Count(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).Count)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn _NewEnum(&self) -> ::windows_core::Result<::windows_core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
        (::windows_core::Interface::vtable(self)._NewEnum)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::IUnknown>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IAzApplications> for ::windows_core::IUnknown {
    fn from(value: IAzApplications) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IAzApplications> for ::windows_core::IUnknown {
    fn from(value: &IAzApplications) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IAzApplications {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IAzApplications {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IAzApplications> for ::win32_system::Com::IDispatch {
    fn from(value: IAzApplications) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IAzApplications> for ::win32_system::Com::IDispatch {
    fn from(value: &IAzApplications) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for IAzApplications {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for &'a IAzApplications {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IAzApplications {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IAzApplications {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IAzApplications {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IAzApplications {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAzApplications").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IAzApplications {
    type Vtable = IAzApplications_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x929b11a9_95c5_4a84_a29a_20ad42c2f16c);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IAzApplications_Vtbl {
    pub base__: ::win32_system::Com::IDispatch_Vtbl,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub get_Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: i32, pvarobtptr: *mut ::win32_system::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    get_Item: usize,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows_core::HRESULT,
    pub _NewEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenumptr: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IAzAuthorizationStore(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IAzAuthorizationStore {
    pub unsafe fn Description(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).Description)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetDescription<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrdescription: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetDescription)(::windows_core::Interface::as_raw(self), bstrdescription.into_param().abi()).ok()
    }
    pub unsafe fn ApplicationData(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).ApplicationData)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetApplicationData<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrapplicationdata: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetApplicationData)(::windows_core::Interface::as_raw(self), bstrapplicationdata.into_param().abi()).ok()
    }
    pub unsafe fn DomainTimeout(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).DomainTimeout)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetDomainTimeout(&self, lprop: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetDomainTimeout)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lprop)).ok()
    }
    pub unsafe fn ScriptEngineTimeout(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).ScriptEngineTimeout)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetScriptEngineTimeout(&self, lprop: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetScriptEngineTimeout)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lprop)).ok()
    }
    pub unsafe fn MaxScriptEngines(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).MaxScriptEngines)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetMaxScriptEngines(&self, lprop: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetMaxScriptEngines)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lprop)).ok()
    }
    pub unsafe fn GenerateAudits(&self) -> ::windows_core::Result<::win32_foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::<::win32_foundation::BOOL>::zeroed();
        (::windows_core::Interface::vtable(self).GenerateAudits)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BOOL>(result__)
    }
    pub unsafe fn SetGenerateAudits<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(&self, bprop: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetGenerateAudits)(::windows_core::Interface::as_raw(self), bprop.into_param().abi()).ok()
    }
    pub unsafe fn Writable(&self) -> ::windows_core::Result<::win32_foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::<::win32_foundation::BOOL>::zeroed();
        (::windows_core::Interface::vtable(self).Writable)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BOOL>(result__)
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetProperty<'a, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, lpropid: i32, varreserved: Param1) -> ::windows_core::Result<::win32_system::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).GetProperty)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lpropid), varreserved.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_system::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetProperty<'a, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>, Param2: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, lpropid: i32, varprop: Param1, varreserved: Param2) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetProperty)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lpropid), varprop.into_param().abi(), varreserved.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AddPropertyItem<'a, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>, Param2: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, lpropid: AZ_PROP_CONSTANTS, varprop: Param1, varreserved: Param2) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).AddPropertyItem)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lpropid), varprop.into_param().abi(), varreserved.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DeletePropertyItem<'a, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>, Param2: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, lpropid: i32, varprop: Param1, varreserved: Param2) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).DeletePropertyItem)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lpropid), varprop.into_param().abi(), varreserved.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn PolicyAdministrators(&self) -> ::windows_core::Result<::win32_system::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).PolicyAdministrators)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_system::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn PolicyReaders(&self) -> ::windows_core::Result<::win32_system::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).PolicyReaders)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_system::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AddPolicyAdministrator<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, bstradmin: Param0, varreserved: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).AddPolicyAdministrator)(::windows_core::Interface::as_raw(self), bstradmin.into_param().abi(), varreserved.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DeletePolicyAdministrator<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, bstradmin: Param0, varreserved: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).DeletePolicyAdministrator)(::windows_core::Interface::as_raw(self), bstradmin.into_param().abi(), varreserved.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AddPolicyReader<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, bstrreader: Param0, varreserved: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).AddPolicyReader)(::windows_core::Interface::as_raw(self), bstrreader.into_param().abi(), varreserved.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DeletePolicyReader<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, bstrreader: Param0, varreserved: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).DeletePolicyReader)(::windows_core::Interface::as_raw(self), bstrreader.into_param().abi(), varreserved.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Initialize<'a, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param2: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, lflags: AZ_PROP_CONSTANTS, bstrpolicyurl: Param1, varreserved: Param2) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Initialize)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lflags), bstrpolicyurl.into_param().abi(), varreserved.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn UpdateCache<'a, Param0: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, varreserved: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).UpdateCache)(::windows_core::Interface::as_raw(self), varreserved.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Delete<'a, Param0: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, varreserved: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Delete)(::windows_core::Interface::as_raw(self), varreserved.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Applications(&self) -> ::windows_core::Result<IAzApplications> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Applications)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IAzApplications>(result__)
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn OpenApplication<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, bstrapplicationname: Param0, varreserved: Param1) -> ::windows_core::Result<IAzApplication> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).OpenApplication)(::windows_core::Interface::as_raw(self), bstrapplicationname.into_param().abi(), varreserved.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IAzApplication>(result__)
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn CreateApplication<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, bstrapplicationname: Param0, varreserved: Param1) -> ::windows_core::Result<IAzApplication> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).CreateApplication)(::windows_core::Interface::as_raw(self), bstrapplicationname.into_param().abi(), varreserved.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IAzApplication>(result__)
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DeleteApplication<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, bstrapplicationname: Param0, varreserved: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).DeleteApplication)(::windows_core::Interface::as_raw(self), bstrapplicationname.into_param().abi(), varreserved.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ApplicationGroups(&self) -> ::windows_core::Result<IAzApplicationGroups> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).ApplicationGroups)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IAzApplicationGroups>(result__)
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn CreateApplicationGroup<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, bstrgroupname: Param0, varreserved: Param1) -> ::windows_core::Result<IAzApplicationGroup> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).CreateApplicationGroup)(::windows_core::Interface::as_raw(self), bstrgroupname.into_param().abi(), varreserved.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IAzApplicationGroup>(result__)
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn OpenApplicationGroup<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, bstrgroupname: Param0, varreserved: Param1) -> ::windows_core::Result<IAzApplicationGroup> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).OpenApplicationGroup)(::windows_core::Interface::as_raw(self), bstrgroupname.into_param().abi(), varreserved.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IAzApplicationGroup>(result__)
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DeleteApplicationGroup<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, bstrgroupname: Param0, varreserved: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).DeleteApplicationGroup)(::windows_core::Interface::as_raw(self), bstrgroupname.into_param().abi(), varreserved.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Submit<'a, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, lflags: i32, varreserved: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Submit)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lflags), varreserved.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DelegatedPolicyUsers(&self) -> ::windows_core::Result<::win32_system::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).DelegatedPolicyUsers)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_system::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AddDelegatedPolicyUser<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, bstrdelegatedpolicyuser: Param0, varreserved: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).AddDelegatedPolicyUser)(::windows_core::Interface::as_raw(self), bstrdelegatedpolicyuser.into_param().abi(), varreserved.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DeleteDelegatedPolicyUser<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, bstrdelegatedpolicyuser: Param0, varreserved: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).DeleteDelegatedPolicyUser)(::windows_core::Interface::as_raw(self), bstrdelegatedpolicyuser.into_param().abi(), varreserved.into_param().abi()).ok()
    }
    pub unsafe fn TargetMachine(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).TargetMachine)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn ApplyStoreSacl(&self) -> ::windows_core::Result<::win32_foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::<::win32_foundation::BOOL>::zeroed();
        (::windows_core::Interface::vtable(self).ApplyStoreSacl)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BOOL>(result__)
    }
    pub unsafe fn SetApplyStoreSacl<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(&self, bapplystoresacl: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetApplyStoreSacl)(::windows_core::Interface::as_raw(self), bapplystoresacl.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn PolicyAdministratorsName(&self) -> ::windows_core::Result<::win32_system::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).PolicyAdministratorsName)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_system::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn PolicyReadersName(&self) -> ::windows_core::Result<::win32_system::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).PolicyReadersName)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_system::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AddPolicyAdministratorName<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, bstradmin: Param0, varreserved: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).AddPolicyAdministratorName)(::windows_core::Interface::as_raw(self), bstradmin.into_param().abi(), varreserved.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DeletePolicyAdministratorName<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, bstradmin: Param0, varreserved: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).DeletePolicyAdministratorName)(::windows_core::Interface::as_raw(self), bstradmin.into_param().abi(), varreserved.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AddPolicyReaderName<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, bstrreader: Param0, varreserved: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).AddPolicyReaderName)(::windows_core::Interface::as_raw(self), bstrreader.into_param().abi(), varreserved.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DeletePolicyReaderName<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, bstrreader: Param0, varreserved: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).DeletePolicyReaderName)(::windows_core::Interface::as_raw(self), bstrreader.into_param().abi(), varreserved.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DelegatedPolicyUsersName(&self) -> ::windows_core::Result<::win32_system::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).DelegatedPolicyUsersName)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_system::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AddDelegatedPolicyUserName<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, bstrdelegatedpolicyuser: Param0, varreserved: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).AddDelegatedPolicyUserName)(::windows_core::Interface::as_raw(self), bstrdelegatedpolicyuser.into_param().abi(), varreserved.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DeleteDelegatedPolicyUserName<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, bstrdelegatedpolicyuser: Param0, varreserved: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).DeleteDelegatedPolicyUserName)(::windows_core::Interface::as_raw(self), bstrdelegatedpolicyuser.into_param().abi(), varreserved.into_param().abi()).ok()
    }
    pub unsafe fn CloseApplication<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrapplicationname: Param0, lflag: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).CloseApplication)(::windows_core::Interface::as_raw(self), bstrapplicationname.into_param().abi(), ::core::mem::transmute(lflag)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IAzAuthorizationStore> for ::windows_core::IUnknown {
    fn from(value: IAzAuthorizationStore) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IAzAuthorizationStore> for ::windows_core::IUnknown {
    fn from(value: &IAzAuthorizationStore) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IAzAuthorizationStore {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IAzAuthorizationStore {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IAzAuthorizationStore> for ::win32_system::Com::IDispatch {
    fn from(value: IAzAuthorizationStore) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IAzAuthorizationStore> for ::win32_system::Com::IDispatch {
    fn from(value: &IAzAuthorizationStore) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for IAzAuthorizationStore {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for &'a IAzAuthorizationStore {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IAzAuthorizationStore {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IAzAuthorizationStore {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IAzAuthorizationStore {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IAzAuthorizationStore {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAzAuthorizationStore").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IAzAuthorizationStore {
    type Vtable = IAzAuthorizationStore_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xedbd9ca9_9b82_4f6a_9e8b_98301e450f14);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IAzAuthorizationStore_Vtbl {
    pub base__: ::win32_system::Com::IDispatch_Vtbl,
    pub Description: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrdescription: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrdescription: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub ApplicationData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrapplicationdata: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SetApplicationData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrapplicationdata: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub DomainTimeout: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plprop: *mut i32) -> ::windows_core::HRESULT,
    pub SetDomainTimeout: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lprop: i32) -> ::windows_core::HRESULT,
    pub ScriptEngineTimeout: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plprop: *mut i32) -> ::windows_core::HRESULT,
    pub SetScriptEngineTimeout: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lprop: i32) -> ::windows_core::HRESULT,
    pub MaxScriptEngines: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plprop: *mut i32) -> ::windows_core::HRESULT,
    pub SetMaxScriptEngines: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lprop: i32) -> ::windows_core::HRESULT,
    pub GenerateAudits: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbprop: *mut ::win32_foundation::BOOL) -> ::windows_core::HRESULT,
    pub SetGenerateAudits: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bprop: ::win32_foundation::BOOL) -> ::windows_core::HRESULT,
    pub Writable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfprop: *mut ::win32_foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub GetProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpropid: i32, varreserved: ::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>, pvarprop: *mut ::win32_system::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    GetProperty: usize,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SetProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpropid: i32, varprop: ::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>, varreserved: ::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SetProperty: usize,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub AddPropertyItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpropid: AZ_PROP_CONSTANTS, varprop: ::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>, varreserved: ::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    AddPropertyItem: usize,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub DeletePropertyItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpropid: i32, varprop: ::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>, varreserved: ::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    DeletePropertyItem: usize,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub PolicyAdministrators: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvaradmins: *mut ::win32_system::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    PolicyAdministrators: usize,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub PolicyReaders: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvarreaders: *mut ::win32_system::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    PolicyReaders: usize,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub AddPolicyAdministrator: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstradmin: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    AddPolicyAdministrator: usize,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub DeletePolicyAdministrator: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstradmin: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    DeletePolicyAdministrator: usize,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub AddPolicyReader: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrreader: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    AddPolicyReader: usize,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub DeletePolicyReader: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrreader: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    DeletePolicyReader: usize,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Initialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lflags: AZ_PROP_CONSTANTS, bstrpolicyurl: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Initialize: usize,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub UpdateCache: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, varreserved: ::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    UpdateCache: usize,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Delete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, varreserved: ::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Delete: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Applications: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppappcollection: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Applications: usize,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub OpenApplication: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrapplicationname: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>, ppapplication: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    OpenApplication: usize,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub CreateApplication: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrapplicationname: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>, ppapplication: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    CreateApplication: usize,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub DeleteApplication: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrapplicationname: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    DeleteApplication: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub ApplicationGroups: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppgroupcollection: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ApplicationGroups: usize,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub CreateApplicationGroup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrgroupname: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>, ppgroup: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    CreateApplicationGroup: usize,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub OpenApplicationGroup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrgroupname: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>, ppgroup: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    OpenApplicationGroup: usize,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub DeleteApplicationGroup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrgroupname: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    DeleteApplicationGroup: usize,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Submit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lflags: i32, varreserved: ::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Submit: usize,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub DelegatedPolicyUsers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvardelegatedpolicyusers: *mut ::win32_system::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    DelegatedPolicyUsers: usize,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub AddDelegatedPolicyUser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrdelegatedpolicyuser: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    AddDelegatedPolicyUser: usize,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub DeleteDelegatedPolicyUser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrdelegatedpolicyuser: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    DeleteDelegatedPolicyUser: usize,
    pub TargetMachine: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrtargetmachine: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub ApplyStoreSacl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbapplystoresacl: *mut ::win32_foundation::BOOL) -> ::windows_core::HRESULT,
    pub SetApplyStoreSacl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bapplystoresacl: ::win32_foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub PolicyAdministratorsName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvaradmins: *mut ::win32_system::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    PolicyAdministratorsName: usize,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub PolicyReadersName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvarreaders: *mut ::win32_system::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    PolicyReadersName: usize,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub AddPolicyAdministratorName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstradmin: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    AddPolicyAdministratorName: usize,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub DeletePolicyAdministratorName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstradmin: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    DeletePolicyAdministratorName: usize,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub AddPolicyReaderName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrreader: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    AddPolicyReaderName: usize,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub DeletePolicyReaderName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrreader: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    DeletePolicyReaderName: usize,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub DelegatedPolicyUsersName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvardelegatedpolicyusers: *mut ::win32_system::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    DelegatedPolicyUsersName: usize,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub AddDelegatedPolicyUserName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrdelegatedpolicyuser: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    AddDelegatedPolicyUserName: usize,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub DeleteDelegatedPolicyUserName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrdelegatedpolicyuser: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    DeleteDelegatedPolicyUserName: usize,
    pub CloseApplication: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrapplicationname: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, lflag: i32) -> ::windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IAzAuthorizationStore2(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IAzAuthorizationStore2 {
    pub unsafe fn Description(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.Description)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetDescription<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrdescription: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetDescription)(::windows_core::Interface::as_raw(self), bstrdescription.into_param().abi()).ok()
    }
    pub unsafe fn ApplicationData(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.ApplicationData)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetApplicationData<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrapplicationdata: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetApplicationData)(::windows_core::Interface::as_raw(self), bstrapplicationdata.into_param().abi()).ok()
    }
    pub unsafe fn DomainTimeout(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).base__.DomainTimeout)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetDomainTimeout(&self, lprop: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetDomainTimeout)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lprop)).ok()
    }
    pub unsafe fn ScriptEngineTimeout(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).base__.ScriptEngineTimeout)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetScriptEngineTimeout(&self, lprop: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetScriptEngineTimeout)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lprop)).ok()
    }
    pub unsafe fn MaxScriptEngines(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).base__.MaxScriptEngines)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetMaxScriptEngines(&self, lprop: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetMaxScriptEngines)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lprop)).ok()
    }
    pub unsafe fn GenerateAudits(&self) -> ::windows_core::Result<::win32_foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::<::win32_foundation::BOOL>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GenerateAudits)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BOOL>(result__)
    }
    pub unsafe fn SetGenerateAudits<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(&self, bprop: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetGenerateAudits)(::windows_core::Interface::as_raw(self), bprop.into_param().abi()).ok()
    }
    pub unsafe fn Writable(&self) -> ::windows_core::Result<::win32_foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::<::win32_foundation::BOOL>::zeroed();
        (::windows_core::Interface::vtable(self).base__.Writable)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BOOL>(result__)
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetProperty<'a, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, lpropid: i32, varreserved: Param1) -> ::windows_core::Result<::win32_system::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetProperty)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lpropid), varreserved.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_system::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetProperty<'a, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>, Param2: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, lpropid: i32, varprop: Param1, varreserved: Param2) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetProperty)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lpropid), varprop.into_param().abi(), varreserved.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AddPropertyItem<'a, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>, Param2: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, lpropid: AZ_PROP_CONSTANTS, varprop: Param1, varreserved: Param2) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.AddPropertyItem)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lpropid), varprop.into_param().abi(), varreserved.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DeletePropertyItem<'a, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>, Param2: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, lpropid: i32, varprop: Param1, varreserved: Param2) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.DeletePropertyItem)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lpropid), varprop.into_param().abi(), varreserved.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn PolicyAdministrators(&self) -> ::windows_core::Result<::win32_system::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.PolicyAdministrators)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_system::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn PolicyReaders(&self) -> ::windows_core::Result<::win32_system::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.PolicyReaders)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_system::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AddPolicyAdministrator<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, bstradmin: Param0, varreserved: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.AddPolicyAdministrator)(::windows_core::Interface::as_raw(self), bstradmin.into_param().abi(), varreserved.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DeletePolicyAdministrator<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, bstradmin: Param0, varreserved: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.DeletePolicyAdministrator)(::windows_core::Interface::as_raw(self), bstradmin.into_param().abi(), varreserved.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AddPolicyReader<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, bstrreader: Param0, varreserved: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.AddPolicyReader)(::windows_core::Interface::as_raw(self), bstrreader.into_param().abi(), varreserved.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DeletePolicyReader<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, bstrreader: Param0, varreserved: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.DeletePolicyReader)(::windows_core::Interface::as_raw(self), bstrreader.into_param().abi(), varreserved.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Initialize<'a, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param2: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, lflags: AZ_PROP_CONSTANTS, bstrpolicyurl: Param1, varreserved: Param2) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Initialize)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lflags), bstrpolicyurl.into_param().abi(), varreserved.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn UpdateCache<'a, Param0: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, varreserved: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.UpdateCache)(::windows_core::Interface::as_raw(self), varreserved.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Delete<'a, Param0: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, varreserved: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Delete)(::windows_core::Interface::as_raw(self), varreserved.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Applications(&self) -> ::windows_core::Result<IAzApplications> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.Applications)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IAzApplications>(result__)
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn OpenApplication<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, bstrapplicationname: Param0, varreserved: Param1) -> ::windows_core::Result<IAzApplication> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.OpenApplication)(::windows_core::Interface::as_raw(self), bstrapplicationname.into_param().abi(), varreserved.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IAzApplication>(result__)
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn CreateApplication<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, bstrapplicationname: Param0, varreserved: Param1) -> ::windows_core::Result<IAzApplication> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.CreateApplication)(::windows_core::Interface::as_raw(self), bstrapplicationname.into_param().abi(), varreserved.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IAzApplication>(result__)
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DeleteApplication<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, bstrapplicationname: Param0, varreserved: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.DeleteApplication)(::windows_core::Interface::as_raw(self), bstrapplicationname.into_param().abi(), varreserved.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ApplicationGroups(&self) -> ::windows_core::Result<IAzApplicationGroups> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.ApplicationGroups)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IAzApplicationGroups>(result__)
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn CreateApplicationGroup<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, bstrgroupname: Param0, varreserved: Param1) -> ::windows_core::Result<IAzApplicationGroup> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.CreateApplicationGroup)(::windows_core::Interface::as_raw(self), bstrgroupname.into_param().abi(), varreserved.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IAzApplicationGroup>(result__)
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn OpenApplicationGroup<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, bstrgroupname: Param0, varreserved: Param1) -> ::windows_core::Result<IAzApplicationGroup> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.OpenApplicationGroup)(::windows_core::Interface::as_raw(self), bstrgroupname.into_param().abi(), varreserved.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IAzApplicationGroup>(result__)
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DeleteApplicationGroup<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, bstrgroupname: Param0, varreserved: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.DeleteApplicationGroup)(::windows_core::Interface::as_raw(self), bstrgroupname.into_param().abi(), varreserved.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Submit<'a, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, lflags: i32, varreserved: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Submit)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lflags), varreserved.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DelegatedPolicyUsers(&self) -> ::windows_core::Result<::win32_system::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.DelegatedPolicyUsers)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_system::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AddDelegatedPolicyUser<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, bstrdelegatedpolicyuser: Param0, varreserved: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.AddDelegatedPolicyUser)(::windows_core::Interface::as_raw(self), bstrdelegatedpolicyuser.into_param().abi(), varreserved.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DeleteDelegatedPolicyUser<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, bstrdelegatedpolicyuser: Param0, varreserved: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.DeleteDelegatedPolicyUser)(::windows_core::Interface::as_raw(self), bstrdelegatedpolicyuser.into_param().abi(), varreserved.into_param().abi()).ok()
    }
    pub unsafe fn TargetMachine(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.TargetMachine)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn ApplyStoreSacl(&self) -> ::windows_core::Result<::win32_foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::<::win32_foundation::BOOL>::zeroed();
        (::windows_core::Interface::vtable(self).base__.ApplyStoreSacl)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BOOL>(result__)
    }
    pub unsafe fn SetApplyStoreSacl<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(&self, bapplystoresacl: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetApplyStoreSacl)(::windows_core::Interface::as_raw(self), bapplystoresacl.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn PolicyAdministratorsName(&self) -> ::windows_core::Result<::win32_system::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.PolicyAdministratorsName)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_system::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn PolicyReadersName(&self) -> ::windows_core::Result<::win32_system::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.PolicyReadersName)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_system::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AddPolicyAdministratorName<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, bstradmin: Param0, varreserved: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.AddPolicyAdministratorName)(::windows_core::Interface::as_raw(self), bstradmin.into_param().abi(), varreserved.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DeletePolicyAdministratorName<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, bstradmin: Param0, varreserved: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.DeletePolicyAdministratorName)(::windows_core::Interface::as_raw(self), bstradmin.into_param().abi(), varreserved.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AddPolicyReaderName<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, bstrreader: Param0, varreserved: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.AddPolicyReaderName)(::windows_core::Interface::as_raw(self), bstrreader.into_param().abi(), varreserved.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DeletePolicyReaderName<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, bstrreader: Param0, varreserved: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.DeletePolicyReaderName)(::windows_core::Interface::as_raw(self), bstrreader.into_param().abi(), varreserved.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DelegatedPolicyUsersName(&self) -> ::windows_core::Result<::win32_system::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.DelegatedPolicyUsersName)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_system::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AddDelegatedPolicyUserName<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, bstrdelegatedpolicyuser: Param0, varreserved: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.AddDelegatedPolicyUserName)(::windows_core::Interface::as_raw(self), bstrdelegatedpolicyuser.into_param().abi(), varreserved.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DeleteDelegatedPolicyUserName<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, bstrdelegatedpolicyuser: Param0, varreserved: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.DeleteDelegatedPolicyUserName)(::windows_core::Interface::as_raw(self), bstrdelegatedpolicyuser.into_param().abi(), varreserved.into_param().abi()).ok()
    }
    pub unsafe fn CloseApplication<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrapplicationname: Param0, lflag: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.CloseApplication)(::windows_core::Interface::as_raw(self), bstrapplicationname.into_param().abi(), ::core::mem::transmute(lflag)).ok()
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn OpenApplication2<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, bstrapplicationname: Param0, varreserved: Param1) -> ::windows_core::Result<IAzApplication2> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).OpenApplication2)(::windows_core::Interface::as_raw(self), bstrapplicationname.into_param().abi(), varreserved.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IAzApplication2>(result__)
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn CreateApplication2<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, bstrapplicationname: Param0, varreserved: Param1) -> ::windows_core::Result<IAzApplication2> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).CreateApplication2)(::windows_core::Interface::as_raw(self), bstrapplicationname.into_param().abi(), varreserved.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IAzApplication2>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IAzAuthorizationStore2> for ::windows_core::IUnknown {
    fn from(value: IAzAuthorizationStore2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IAzAuthorizationStore2> for ::windows_core::IUnknown {
    fn from(value: &IAzAuthorizationStore2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IAzAuthorizationStore2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IAzAuthorizationStore2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IAzAuthorizationStore2> for ::win32_system::Com::IDispatch {
    fn from(value: IAzAuthorizationStore2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IAzAuthorizationStore2> for ::win32_system::Com::IDispatch {
    fn from(value: &IAzAuthorizationStore2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for IAzAuthorizationStore2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for &'a IAzAuthorizationStore2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IAzAuthorizationStore2> for IAzAuthorizationStore {
    fn from(value: IAzAuthorizationStore2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IAzAuthorizationStore2> for IAzAuthorizationStore {
    fn from(value: &IAzAuthorizationStore2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, IAzAuthorizationStore> for IAzAuthorizationStore2 {
    fn into_param(self) -> ::windows_core::Param<'a, IAzAuthorizationStore> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, IAzAuthorizationStore> for &'a IAzAuthorizationStore2 {
    fn into_param(self) -> ::windows_core::Param<'a, IAzAuthorizationStore> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IAzAuthorizationStore2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IAzAuthorizationStore2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IAzAuthorizationStore2 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IAzAuthorizationStore2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAzAuthorizationStore2").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IAzAuthorizationStore2 {
    type Vtable = IAzAuthorizationStore2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb11e5584_d577_4273_b6c5_0973e0f8e80d);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IAzAuthorizationStore2_Vtbl {
    pub base__: IAzAuthorizationStore_Vtbl,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub OpenApplication2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrapplicationname: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>, ppapplication: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    OpenApplication2: usize,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub CreateApplication2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrapplicationname: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>, ppapplication: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    CreateApplication2: usize,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IAzAuthorizationStore3(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IAzAuthorizationStore3 {
    pub unsafe fn Description(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.Description)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetDescription<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrdescription: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.SetDescription)(::windows_core::Interface::as_raw(self), bstrdescription.into_param().abi()).ok()
    }
    pub unsafe fn ApplicationData(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.ApplicationData)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetApplicationData<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrapplicationdata: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.SetApplicationData)(::windows_core::Interface::as_raw(self), bstrapplicationdata.into_param().abi()).ok()
    }
    pub unsafe fn DomainTimeout(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.DomainTimeout)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetDomainTimeout(&self, lprop: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.SetDomainTimeout)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lprop)).ok()
    }
    pub unsafe fn ScriptEngineTimeout(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.ScriptEngineTimeout)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetScriptEngineTimeout(&self, lprop: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.SetScriptEngineTimeout)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lprop)).ok()
    }
    pub unsafe fn MaxScriptEngines(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.MaxScriptEngines)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetMaxScriptEngines(&self, lprop: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.SetMaxScriptEngines)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lprop)).ok()
    }
    pub unsafe fn GenerateAudits(&self) -> ::windows_core::Result<::win32_foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::<::win32_foundation::BOOL>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.GenerateAudits)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BOOL>(result__)
    }
    pub unsafe fn SetGenerateAudits<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(&self, bprop: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.SetGenerateAudits)(::windows_core::Interface::as_raw(self), bprop.into_param().abi()).ok()
    }
    pub unsafe fn Writable(&self) -> ::windows_core::Result<::win32_foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::<::win32_foundation::BOOL>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.Writable)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BOOL>(result__)
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetProperty<'a, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, lpropid: i32, varreserved: Param1) -> ::windows_core::Result<::win32_system::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.GetProperty)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lpropid), varreserved.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_system::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetProperty<'a, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>, Param2: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, lpropid: i32, varprop: Param1, varreserved: Param2) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.SetProperty)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lpropid), varprop.into_param().abi(), varreserved.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AddPropertyItem<'a, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>, Param2: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, lpropid: AZ_PROP_CONSTANTS, varprop: Param1, varreserved: Param2) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.AddPropertyItem)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lpropid), varprop.into_param().abi(), varreserved.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DeletePropertyItem<'a, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>, Param2: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, lpropid: i32, varprop: Param1, varreserved: Param2) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.DeletePropertyItem)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lpropid), varprop.into_param().abi(), varreserved.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn PolicyAdministrators(&self) -> ::windows_core::Result<::win32_system::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.PolicyAdministrators)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_system::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn PolicyReaders(&self) -> ::windows_core::Result<::win32_system::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.PolicyReaders)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_system::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AddPolicyAdministrator<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, bstradmin: Param0, varreserved: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.AddPolicyAdministrator)(::windows_core::Interface::as_raw(self), bstradmin.into_param().abi(), varreserved.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DeletePolicyAdministrator<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, bstradmin: Param0, varreserved: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.DeletePolicyAdministrator)(::windows_core::Interface::as_raw(self), bstradmin.into_param().abi(), varreserved.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AddPolicyReader<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, bstrreader: Param0, varreserved: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.AddPolicyReader)(::windows_core::Interface::as_raw(self), bstrreader.into_param().abi(), varreserved.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DeletePolicyReader<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, bstrreader: Param0, varreserved: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.DeletePolicyReader)(::windows_core::Interface::as_raw(self), bstrreader.into_param().abi(), varreserved.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Initialize<'a, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param2: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, lflags: AZ_PROP_CONSTANTS, bstrpolicyurl: Param1, varreserved: Param2) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.Initialize)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lflags), bstrpolicyurl.into_param().abi(), varreserved.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn UpdateCache<'a, Param0: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, varreserved: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.UpdateCache)(::windows_core::Interface::as_raw(self), varreserved.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Delete<'a, Param0: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, varreserved: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.Delete)(::windows_core::Interface::as_raw(self), varreserved.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Applications(&self) -> ::windows_core::Result<IAzApplications> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.Applications)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IAzApplications>(result__)
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn OpenApplication<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, bstrapplicationname: Param0, varreserved: Param1) -> ::windows_core::Result<IAzApplication> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.OpenApplication)(::windows_core::Interface::as_raw(self), bstrapplicationname.into_param().abi(), varreserved.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IAzApplication>(result__)
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn CreateApplication<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, bstrapplicationname: Param0, varreserved: Param1) -> ::windows_core::Result<IAzApplication> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.CreateApplication)(::windows_core::Interface::as_raw(self), bstrapplicationname.into_param().abi(), varreserved.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IAzApplication>(result__)
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DeleteApplication<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, bstrapplicationname: Param0, varreserved: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.DeleteApplication)(::windows_core::Interface::as_raw(self), bstrapplicationname.into_param().abi(), varreserved.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ApplicationGroups(&self) -> ::windows_core::Result<IAzApplicationGroups> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.ApplicationGroups)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IAzApplicationGroups>(result__)
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn CreateApplicationGroup<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, bstrgroupname: Param0, varreserved: Param1) -> ::windows_core::Result<IAzApplicationGroup> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.CreateApplicationGroup)(::windows_core::Interface::as_raw(self), bstrgroupname.into_param().abi(), varreserved.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IAzApplicationGroup>(result__)
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn OpenApplicationGroup<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, bstrgroupname: Param0, varreserved: Param1) -> ::windows_core::Result<IAzApplicationGroup> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.OpenApplicationGroup)(::windows_core::Interface::as_raw(self), bstrgroupname.into_param().abi(), varreserved.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IAzApplicationGroup>(result__)
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DeleteApplicationGroup<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, bstrgroupname: Param0, varreserved: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.DeleteApplicationGroup)(::windows_core::Interface::as_raw(self), bstrgroupname.into_param().abi(), varreserved.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Submit<'a, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, lflags: i32, varreserved: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.Submit)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lflags), varreserved.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DelegatedPolicyUsers(&self) -> ::windows_core::Result<::win32_system::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.DelegatedPolicyUsers)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_system::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AddDelegatedPolicyUser<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, bstrdelegatedpolicyuser: Param0, varreserved: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.AddDelegatedPolicyUser)(::windows_core::Interface::as_raw(self), bstrdelegatedpolicyuser.into_param().abi(), varreserved.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DeleteDelegatedPolicyUser<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, bstrdelegatedpolicyuser: Param0, varreserved: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.DeleteDelegatedPolicyUser)(::windows_core::Interface::as_raw(self), bstrdelegatedpolicyuser.into_param().abi(), varreserved.into_param().abi()).ok()
    }
    pub unsafe fn TargetMachine(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.TargetMachine)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn ApplyStoreSacl(&self) -> ::windows_core::Result<::win32_foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::<::win32_foundation::BOOL>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.ApplyStoreSacl)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BOOL>(result__)
    }
    pub unsafe fn SetApplyStoreSacl<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(&self, bapplystoresacl: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.SetApplyStoreSacl)(::windows_core::Interface::as_raw(self), bapplystoresacl.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn PolicyAdministratorsName(&self) -> ::windows_core::Result<::win32_system::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.PolicyAdministratorsName)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_system::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn PolicyReadersName(&self) -> ::windows_core::Result<::win32_system::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.PolicyReadersName)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_system::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AddPolicyAdministratorName<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, bstradmin: Param0, varreserved: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.AddPolicyAdministratorName)(::windows_core::Interface::as_raw(self), bstradmin.into_param().abi(), varreserved.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DeletePolicyAdministratorName<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, bstradmin: Param0, varreserved: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.DeletePolicyAdministratorName)(::windows_core::Interface::as_raw(self), bstradmin.into_param().abi(), varreserved.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AddPolicyReaderName<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, bstrreader: Param0, varreserved: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.AddPolicyReaderName)(::windows_core::Interface::as_raw(self), bstrreader.into_param().abi(), varreserved.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DeletePolicyReaderName<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, bstrreader: Param0, varreserved: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.DeletePolicyReaderName)(::windows_core::Interface::as_raw(self), bstrreader.into_param().abi(), varreserved.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DelegatedPolicyUsersName(&self) -> ::windows_core::Result<::win32_system::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.DelegatedPolicyUsersName)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_system::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AddDelegatedPolicyUserName<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, bstrdelegatedpolicyuser: Param0, varreserved: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.AddDelegatedPolicyUserName)(::windows_core::Interface::as_raw(self), bstrdelegatedpolicyuser.into_param().abi(), varreserved.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DeleteDelegatedPolicyUserName<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, bstrdelegatedpolicyuser: Param0, varreserved: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.DeleteDelegatedPolicyUserName)(::windows_core::Interface::as_raw(self), bstrdelegatedpolicyuser.into_param().abi(), varreserved.into_param().abi()).ok()
    }
    pub unsafe fn CloseApplication<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrapplicationname: Param0, lflag: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.CloseApplication)(::windows_core::Interface::as_raw(self), bstrapplicationname.into_param().abi(), ::core::mem::transmute(lflag)).ok()
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn OpenApplication2<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, bstrapplicationname: Param0, varreserved: Param1) -> ::windows_core::Result<IAzApplication2> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.OpenApplication2)(::windows_core::Interface::as_raw(self), bstrapplicationname.into_param().abi(), varreserved.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IAzApplication2>(result__)
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn CreateApplication2<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, bstrapplicationname: Param0, varreserved: Param1) -> ::windows_core::Result<IAzApplication2> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.CreateApplication2)(::windows_core::Interface::as_raw(self), bstrapplicationname.into_param().abi(), varreserved.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IAzApplication2>(result__)
    }
    pub unsafe fn IsUpdateNeeded(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).IsUpdateNeeded)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn BizruleGroupSupported(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).BizruleGroupSupported)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn UpgradeStoresFunctionalLevel(&self, lfunctionallevel: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).UpgradeStoresFunctionalLevel)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lfunctionallevel)).ok()
    }
    pub unsafe fn IsFunctionalLevelUpgradeSupported(&self, lfunctionallevel: i32) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).IsFunctionalLevelUpgradeSupported)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lfunctionallevel), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn GetSchemaVersion(&self, plmajorversion: *mut i32, plminorversion: *mut i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetSchemaVersion)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(plmajorversion), ::core::mem::transmute(plminorversion)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IAzAuthorizationStore3> for ::windows_core::IUnknown {
    fn from(value: IAzAuthorizationStore3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IAzAuthorizationStore3> for ::windows_core::IUnknown {
    fn from(value: &IAzAuthorizationStore3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IAzAuthorizationStore3 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IAzAuthorizationStore3 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IAzAuthorizationStore3> for ::win32_system::Com::IDispatch {
    fn from(value: IAzAuthorizationStore3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IAzAuthorizationStore3> for ::win32_system::Com::IDispatch {
    fn from(value: &IAzAuthorizationStore3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for IAzAuthorizationStore3 {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for &'a IAzAuthorizationStore3 {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IAzAuthorizationStore3> for IAzAuthorizationStore {
    fn from(value: IAzAuthorizationStore3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IAzAuthorizationStore3> for IAzAuthorizationStore {
    fn from(value: &IAzAuthorizationStore3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, IAzAuthorizationStore> for IAzAuthorizationStore3 {
    fn into_param(self) -> ::windows_core::Param<'a, IAzAuthorizationStore> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, IAzAuthorizationStore> for &'a IAzAuthorizationStore3 {
    fn into_param(self) -> ::windows_core::Param<'a, IAzAuthorizationStore> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IAzAuthorizationStore3> for IAzAuthorizationStore2 {
    fn from(value: IAzAuthorizationStore3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IAzAuthorizationStore3> for IAzAuthorizationStore2 {
    fn from(value: &IAzAuthorizationStore3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, IAzAuthorizationStore2> for IAzAuthorizationStore3 {
    fn into_param(self) -> ::windows_core::Param<'a, IAzAuthorizationStore2> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, IAzAuthorizationStore2> for &'a IAzAuthorizationStore3 {
    fn into_param(self) -> ::windows_core::Param<'a, IAzAuthorizationStore2> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IAzAuthorizationStore3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IAzAuthorizationStore3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IAzAuthorizationStore3 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IAzAuthorizationStore3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAzAuthorizationStore3").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IAzAuthorizationStore3 {
    type Vtable = IAzAuthorizationStore3_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xabc08425_0c86_4fa0_9be3_7189956c926e);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IAzAuthorizationStore3_Vtbl {
    pub base__: IAzAuthorizationStore2_Vtbl,
    pub IsUpdateNeeded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbisupdateneeded: *mut i16) -> ::windows_core::HRESULT,
    pub BizruleGroupSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbsupported: *mut i16) -> ::windows_core::HRESULT,
    pub UpgradeStoresFunctionalLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lfunctionallevel: i32) -> ::windows_core::HRESULT,
    pub IsFunctionalLevelUpgradeSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lfunctionallevel: i32, pbsupported: *mut i16) -> ::windows_core::HRESULT,
    pub GetSchemaVersion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plmajorversion: *mut i32, plminorversion: *mut i32) -> ::windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IAzBizRuleContext(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IAzBizRuleContext {
    pub unsafe fn SetBusinessRuleResult<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(&self, bresult: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetBusinessRuleResult)(::windows_core::Interface::as_raw(self), bresult.into_param().abi()).ok()
    }
    pub unsafe fn SetBusinessRuleString<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrbusinessrulestring: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetBusinessRuleString)(::windows_core::Interface::as_raw(self), bstrbusinessrulestring.into_param().abi()).ok()
    }
    pub unsafe fn BusinessRuleString(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).BusinessRuleString)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetParameter<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrparametername: Param0) -> ::windows_core::Result<::win32_system::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).GetParameter)(::windows_core::Interface::as_raw(self), bstrparametername.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_system::Com::VARIANT>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IAzBizRuleContext> for ::windows_core::IUnknown {
    fn from(value: IAzBizRuleContext) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IAzBizRuleContext> for ::windows_core::IUnknown {
    fn from(value: &IAzBizRuleContext) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IAzBizRuleContext {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IAzBizRuleContext {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IAzBizRuleContext> for ::win32_system::Com::IDispatch {
    fn from(value: IAzBizRuleContext) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IAzBizRuleContext> for ::win32_system::Com::IDispatch {
    fn from(value: &IAzBizRuleContext) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for IAzBizRuleContext {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for &'a IAzBizRuleContext {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IAzBizRuleContext {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IAzBizRuleContext {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IAzBizRuleContext {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IAzBizRuleContext {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAzBizRuleContext").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IAzBizRuleContext {
    type Vtable = IAzBizRuleContext_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe192f17d_d59f_455e_a152_940316cd77b2);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IAzBizRuleContext_Vtbl {
    pub base__: ::win32_system::Com::IDispatch_Vtbl,
    pub SetBusinessRuleResult: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bresult: ::win32_foundation::BOOL) -> ::windows_core::HRESULT,
    pub SetBusinessRuleString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrbusinessrulestring: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub BusinessRuleString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrbusinessrulestring: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub GetParameter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrparametername: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, pvarparametervalue: *mut ::win32_system::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    GetParameter: usize,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IAzBizRuleInterfaces(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IAzBizRuleInterfaces {
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AddInterface<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param2: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, bstrinterfacename: Param0, linterfaceflag: i32, varinterface: Param2) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).AddInterface)(::windows_core::Interface::as_raw(self), bstrinterfacename.into_param().abi(), ::core::mem::transmute(linterfaceflag), varinterface.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AddInterfaces<'a, Param0: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>, Param2: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, varinterfacenames: Param0, varinterfaceflags: Param1, varinterfaces: Param2) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).AddInterfaces)(::windows_core::Interface::as_raw(self), varinterfacenames.into_param().abi(), varinterfaceflags.into_param().abi(), varinterfaces.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetInterfaceValue<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrinterfacename: Param0, linterfaceflag: *mut i32, varinterface: *mut ::win32_system::Com::VARIANT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetInterfaceValue)(::windows_core::Interface::as_raw(self), bstrinterfacename.into_param().abi(), ::core::mem::transmute(linterfaceflag), ::core::mem::transmute(varinterface)).ok()
    }
    pub unsafe fn Remove<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrinterfacename: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Remove)(::windows_core::Interface::as_raw(self), bstrinterfacename.into_param().abi()).ok()
    }
    pub unsafe fn RemoveAll(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).RemoveAll)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Count(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).Count)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IAzBizRuleInterfaces> for ::windows_core::IUnknown {
    fn from(value: IAzBizRuleInterfaces) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IAzBizRuleInterfaces> for ::windows_core::IUnknown {
    fn from(value: &IAzBizRuleInterfaces) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IAzBizRuleInterfaces {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IAzBizRuleInterfaces {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IAzBizRuleInterfaces> for ::win32_system::Com::IDispatch {
    fn from(value: IAzBizRuleInterfaces) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IAzBizRuleInterfaces> for ::win32_system::Com::IDispatch {
    fn from(value: &IAzBizRuleInterfaces) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for IAzBizRuleInterfaces {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for &'a IAzBizRuleInterfaces {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IAzBizRuleInterfaces {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IAzBizRuleInterfaces {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IAzBizRuleInterfaces {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IAzBizRuleInterfaces {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAzBizRuleInterfaces").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IAzBizRuleInterfaces {
    type Vtable = IAzBizRuleInterfaces_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe94128c7_e9da_44cc_b0bd_53036f3aab3d);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IAzBizRuleInterfaces_Vtbl {
    pub base__: ::win32_system::Com::IDispatch_Vtbl,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub AddInterface: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrinterfacename: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, linterfaceflag: i32, varinterface: ::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    AddInterface: usize,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub AddInterfaces: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, varinterfacenames: ::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>, varinterfaceflags: ::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>, varinterfaces: ::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    AddInterfaces: usize,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub GetInterfaceValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrinterfacename: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, linterfaceflag: *mut i32, varinterface: *mut ::win32_system::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    GetInterfaceValue: usize,
    pub Remove: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrinterfacename: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub RemoveAll: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plcount: *mut u32) -> ::windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IAzBizRuleParameters(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IAzBizRuleParameters {
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AddParameter<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, bstrparametername: Param0, varparametervalue: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).AddParameter)(::windows_core::Interface::as_raw(self), bstrparametername.into_param().abi(), varparametervalue.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AddParameters<'a, Param0: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, varparameternames: Param0, varparametervalues: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).AddParameters)(::windows_core::Interface::as_raw(self), varparameternames.into_param().abi(), varparametervalues.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetParameterValue<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrparametername: Param0) -> ::windows_core::Result<::win32_system::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).GetParameterValue)(::windows_core::Interface::as_raw(self), bstrparametername.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_system::Com::VARIANT>(result__)
    }
    pub unsafe fn Remove<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, varparametername: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Remove)(::windows_core::Interface::as_raw(self), varparametername.into_param().abi()).ok()
    }
    pub unsafe fn RemoveAll(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).RemoveAll)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Count(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).Count)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IAzBizRuleParameters> for ::windows_core::IUnknown {
    fn from(value: IAzBizRuleParameters) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IAzBizRuleParameters> for ::windows_core::IUnknown {
    fn from(value: &IAzBizRuleParameters) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IAzBizRuleParameters {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IAzBizRuleParameters {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IAzBizRuleParameters> for ::win32_system::Com::IDispatch {
    fn from(value: IAzBizRuleParameters) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IAzBizRuleParameters> for ::win32_system::Com::IDispatch {
    fn from(value: &IAzBizRuleParameters) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for IAzBizRuleParameters {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for &'a IAzBizRuleParameters {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IAzBizRuleParameters {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IAzBizRuleParameters {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IAzBizRuleParameters {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IAzBizRuleParameters {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAzBizRuleParameters").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IAzBizRuleParameters {
    type Vtable = IAzBizRuleParameters_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xfc17685f_e25d_4dcd_bae1_276ec9533cb5);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IAzBizRuleParameters_Vtbl {
    pub base__: ::win32_system::Com::IDispatch_Vtbl,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub AddParameter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrparametername: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, varparametervalue: ::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    AddParameter: usize,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub AddParameters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, varparameternames: ::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>, varparametervalues: ::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    AddParameters: usize,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub GetParameterValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrparametername: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, pvarparametervalue: *mut ::win32_system::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    GetParameterValue: usize,
    pub Remove: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, varparametername: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub RemoveAll: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plcount: *mut u32) -> ::windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IAzClientContext(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IAzClientContext {
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AccessCheck<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>, Param2: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>, Param3: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>, Param4: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>, Param5: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>, Param6: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>, Param7: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(
        &self,
        bstrobjectname: Param0,
        varscopenames: Param1,
        varoperations: Param2,
        varparameternames: Param3,
        varparametervalues: Param4,
        varinterfacenames: Param5,
        varinterfaceflags: Param6,
        varinterfaces: Param7,
    ) -> ::windows_core::Result<::win32_system::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).AccessCheck)(::windows_core::Interface::as_raw(self), bstrobjectname.into_param().abi(), varscopenames.into_param().abi(), varoperations.into_param().abi(), varparameternames.into_param().abi(), varparametervalues.into_param().abi(), varinterfacenames.into_param().abi(), varinterfaceflags.into_param().abi(), varinterfaces.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_system::Com::VARIANT>(result__)
    }
    pub unsafe fn GetBusinessRuleString(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).GetBusinessRuleString)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn UserDn(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).UserDn)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn UserSamCompat(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).UserSamCompat)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn UserDisplay(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).UserDisplay)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn UserGuid(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).UserGuid)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn UserCanonical(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).UserCanonical)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn UserUpn(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).UserUpn)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn UserDnsSamCompat(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).UserDnsSamCompat)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetProperty<'a, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, lpropid: i32, varreserved: Param1) -> ::windows_core::Result<::win32_system::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).GetProperty)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lpropid), varreserved.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_system::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetRoles<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrscopename: Param0) -> ::windows_core::Result<::win32_system::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).GetRoles)(::windows_core::Interface::as_raw(self), bstrscopename.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_system::Com::VARIANT>(result__)
    }
    pub unsafe fn RoleForAccessCheck(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).RoleForAccessCheck)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetRoleForAccessCheck<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrprop: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetRoleForAccessCheck)(::windows_core::Interface::as_raw(self), bstrprop.into_param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IAzClientContext> for ::windows_core::IUnknown {
    fn from(value: IAzClientContext) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IAzClientContext> for ::windows_core::IUnknown {
    fn from(value: &IAzClientContext) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IAzClientContext {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IAzClientContext {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IAzClientContext> for ::win32_system::Com::IDispatch {
    fn from(value: IAzClientContext) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IAzClientContext> for ::win32_system::Com::IDispatch {
    fn from(value: &IAzClientContext) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for IAzClientContext {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for &'a IAzClientContext {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IAzClientContext {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IAzClientContext {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IAzClientContext {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IAzClientContext {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAzClientContext").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IAzClientContext {
    type Vtable = IAzClientContext_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xeff1f00b_488a_466d_afd9_a401c5f9eef5);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IAzClientContext_Vtbl {
    pub base__: ::win32_system::Com::IDispatch_Vtbl,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub AccessCheck: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrobjectname: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, varscopenames: ::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>, varoperations: ::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>, varparameternames: ::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>, varparametervalues: ::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>, varinterfacenames: ::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>, varinterfaceflags: ::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>, varinterfaces: ::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>, pvarresults: *mut ::win32_system::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    AccessCheck: usize,
    pub GetBusinessRuleString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrbusinessrulestring: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub UserDn: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrprop: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub UserSamCompat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrprop: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub UserDisplay: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrprop: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub UserGuid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrprop: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub UserCanonical: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrprop: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub UserUpn: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrprop: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub UserDnsSamCompat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrprop: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub GetProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpropid: i32, varreserved: ::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>, pvarprop: *mut ::win32_system::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    GetProperty: usize,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub GetRoles: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrscopename: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, pvarrolenames: *mut ::win32_system::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    GetRoles: usize,
    pub RoleForAccessCheck: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrprop: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SetRoleForAccessCheck: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrprop: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IAzClientContext2(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IAzClientContext2 {
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AccessCheck<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>, Param2: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>, Param3: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>, Param4: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>, Param5: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>, Param6: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>, Param7: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(
        &self,
        bstrobjectname: Param0,
        varscopenames: Param1,
        varoperations: Param2,
        varparameternames: Param3,
        varparametervalues: Param4,
        varinterfacenames: Param5,
        varinterfaceflags: Param6,
        varinterfaces: Param7,
    ) -> ::windows_core::Result<::win32_system::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.AccessCheck)(::windows_core::Interface::as_raw(self), bstrobjectname.into_param().abi(), varscopenames.into_param().abi(), varoperations.into_param().abi(), varparameternames.into_param().abi(), varparametervalues.into_param().abi(), varinterfacenames.into_param().abi(), varinterfaceflags.into_param().abi(), varinterfaces.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_system::Com::VARIANT>(result__)
    }
    pub unsafe fn GetBusinessRuleString(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetBusinessRuleString)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn UserDn(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.UserDn)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn UserSamCompat(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.UserSamCompat)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn UserDisplay(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.UserDisplay)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn UserGuid(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.UserGuid)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn UserCanonical(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.UserCanonical)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn UserUpn(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.UserUpn)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn UserDnsSamCompat(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.UserDnsSamCompat)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetProperty<'a, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, lpropid: i32, varreserved: Param1) -> ::windows_core::Result<::win32_system::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetProperty)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lpropid), varreserved.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_system::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetRoles<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrscopename: Param0) -> ::windows_core::Result<::win32_system::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetRoles)(::windows_core::Interface::as_raw(self), bstrscopename.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_system::Com::VARIANT>(result__)
    }
    pub unsafe fn RoleForAccessCheck(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.RoleForAccessCheck)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetRoleForAccessCheck<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrprop: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetRoleForAccessCheck)(::windows_core::Interface::as_raw(self), bstrprop.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetAssignedScopesPage(&self, loptions: i32, pagesize: i32, pvarcursor: *mut ::win32_system::Com::VARIANT, pvarscopenames: *mut ::win32_system::Com::VARIANT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetAssignedScopesPage)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(loptions), ::core::mem::transmute(pagesize), ::core::mem::transmute(pvarcursor), ::core::mem::transmute(pvarscopenames)).ok()
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AddRoles<'a, Param0: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, varroles: Param0, bstrscopename: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).AddRoles)(::windows_core::Interface::as_raw(self), varroles.into_param().abi(), bstrscopename.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AddApplicationGroups<'a, Param0: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, varapplicationgroups: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).AddApplicationGroups)(::windows_core::Interface::as_raw(self), varapplicationgroups.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AddStringSids<'a, Param0: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, varstringsids: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).AddStringSids)(::windows_core::Interface::as_raw(self), varstringsids.into_param().abi()).ok()
    }
    pub unsafe fn SetLDAPQueryDN<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrldapquerydn: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetLDAPQueryDN)(::windows_core::Interface::as_raw(self), bstrldapquerydn.into_param().abi()).ok()
    }
    pub unsafe fn LDAPQueryDN(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).LDAPQueryDN)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IAzClientContext2> for ::windows_core::IUnknown {
    fn from(value: IAzClientContext2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IAzClientContext2> for ::windows_core::IUnknown {
    fn from(value: &IAzClientContext2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IAzClientContext2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IAzClientContext2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IAzClientContext2> for ::win32_system::Com::IDispatch {
    fn from(value: IAzClientContext2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IAzClientContext2> for ::win32_system::Com::IDispatch {
    fn from(value: &IAzClientContext2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for IAzClientContext2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for &'a IAzClientContext2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IAzClientContext2> for IAzClientContext {
    fn from(value: IAzClientContext2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IAzClientContext2> for IAzClientContext {
    fn from(value: &IAzClientContext2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, IAzClientContext> for IAzClientContext2 {
    fn into_param(self) -> ::windows_core::Param<'a, IAzClientContext> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, IAzClientContext> for &'a IAzClientContext2 {
    fn into_param(self) -> ::windows_core::Param<'a, IAzClientContext> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IAzClientContext2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IAzClientContext2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IAzClientContext2 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IAzClientContext2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAzClientContext2").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IAzClientContext2 {
    type Vtable = IAzClientContext2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2b0c92b8_208a_488a_8f81_e4edb22111cd);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IAzClientContext2_Vtbl {
    pub base__: IAzClientContext_Vtbl,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub GetAssignedScopesPage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, loptions: i32, pagesize: i32, pvarcursor: *mut ::win32_system::Com::VARIANT, pvarscopenames: *mut ::win32_system::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    GetAssignedScopesPage: usize,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub AddRoles: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, varroles: ::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>, bstrscopename: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    AddRoles: usize,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub AddApplicationGroups: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, varapplicationgroups: ::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    AddApplicationGroups: usize,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub AddStringSids: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, varstringsids: ::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    AddStringSids: usize,
    pub SetLDAPQueryDN: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrldapquerydn: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub LDAPQueryDN: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrldapquerydn: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IAzClientContext3(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IAzClientContext3 {
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AccessCheck<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>, Param2: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>, Param3: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>, Param4: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>, Param5: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>, Param6: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>, Param7: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(
        &self,
        bstrobjectname: Param0,
        varscopenames: Param1,
        varoperations: Param2,
        varparameternames: Param3,
        varparametervalues: Param4,
        varinterfacenames: Param5,
        varinterfaceflags: Param6,
        varinterfaces: Param7,
    ) -> ::windows_core::Result<::win32_system::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.AccessCheck)(::windows_core::Interface::as_raw(self), bstrobjectname.into_param().abi(), varscopenames.into_param().abi(), varoperations.into_param().abi(), varparameternames.into_param().abi(), varparametervalues.into_param().abi(), varinterfacenames.into_param().abi(), varinterfaceflags.into_param().abi(), varinterfaces.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_system::Com::VARIANT>(result__)
    }
    pub unsafe fn GetBusinessRuleString(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.GetBusinessRuleString)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn UserDn(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.UserDn)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn UserSamCompat(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.UserSamCompat)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn UserDisplay(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.UserDisplay)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn UserGuid(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.UserGuid)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn UserCanonical(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.UserCanonical)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn UserUpn(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.UserUpn)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn UserDnsSamCompat(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.UserDnsSamCompat)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetProperty<'a, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, lpropid: i32, varreserved: Param1) -> ::windows_core::Result<::win32_system::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.GetProperty)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lpropid), varreserved.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_system::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetRoles<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrscopename: Param0) -> ::windows_core::Result<::win32_system::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.GetRoles)(::windows_core::Interface::as_raw(self), bstrscopename.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_system::Com::VARIANT>(result__)
    }
    pub unsafe fn RoleForAccessCheck(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.RoleForAccessCheck)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetRoleForAccessCheck<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrprop: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.SetRoleForAccessCheck)(::windows_core::Interface::as_raw(self), bstrprop.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetAssignedScopesPage(&self, loptions: i32, pagesize: i32, pvarcursor: *mut ::win32_system::Com::VARIANT, pvarscopenames: *mut ::win32_system::Com::VARIANT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetAssignedScopesPage)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(loptions), ::core::mem::transmute(pagesize), ::core::mem::transmute(pvarcursor), ::core::mem::transmute(pvarscopenames)).ok()
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AddRoles<'a, Param0: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, varroles: Param0, bstrscopename: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.AddRoles)(::windows_core::Interface::as_raw(self), varroles.into_param().abi(), bstrscopename.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AddApplicationGroups<'a, Param0: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, varapplicationgroups: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.AddApplicationGroups)(::windows_core::Interface::as_raw(self), varapplicationgroups.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AddStringSids<'a, Param0: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, varstringsids: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.AddStringSids)(::windows_core::Interface::as_raw(self), varstringsids.into_param().abi()).ok()
    }
    pub unsafe fn SetLDAPQueryDN<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrldapquerydn: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetLDAPQueryDN)(::windows_core::Interface::as_raw(self), bstrldapquerydn.into_param().abi()).ok()
    }
    pub unsafe fn LDAPQueryDN(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.LDAPQueryDN)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn AccessCheck2<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrobjectname: Param0, bstrscopename: Param1, loperation: i32) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).AccessCheck2)(::windows_core::Interface::as_raw(self), bstrobjectname.into_param().abi(), bstrscopename.into_param().abi(), ::core::mem::transmute(loperation), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn IsInRoleAssignment<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrscopename: Param0, bstrrolename: Param1) -> ::windows_core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::<i16>::zeroed();
        (::windows_core::Interface::vtable(self).IsInRoleAssignment)(::windows_core::Interface::as_raw(self), bstrscopename.into_param().abi(), bstrrolename.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetOperations<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrscopename: Param0) -> ::windows_core::Result<IAzOperations> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetOperations)(::windows_core::Interface::as_raw(self), bstrscopename.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IAzOperations>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTasks<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrscopename: Param0) -> ::windows_core::Result<IAzTasks> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetTasks)(::windows_core::Interface::as_raw(self), bstrscopename.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IAzTasks>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn BizRuleParameters(&self) -> ::windows_core::Result<IAzBizRuleParameters> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).BizRuleParameters)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IAzBizRuleParameters>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn BizRuleInterfaces(&self) -> ::windows_core::Result<IAzBizRuleInterfaces> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).BizRuleInterfaces)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IAzBizRuleInterfaces>(result__)
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetGroups<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrscopename: Param0, uloptions: AZ_PROP_CONSTANTS) -> ::windows_core::Result<::win32_system::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).GetGroups)(::windows_core::Interface::as_raw(self), bstrscopename.into_param().abi(), ::core::mem::transmute(uloptions), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_system::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Sids(&self) -> ::windows_core::Result<::win32_system::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).Sids)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_system::Com::VARIANT>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IAzClientContext3> for ::windows_core::IUnknown {
    fn from(value: IAzClientContext3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IAzClientContext3> for ::windows_core::IUnknown {
    fn from(value: &IAzClientContext3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IAzClientContext3 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IAzClientContext3 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IAzClientContext3> for ::win32_system::Com::IDispatch {
    fn from(value: IAzClientContext3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IAzClientContext3> for ::win32_system::Com::IDispatch {
    fn from(value: &IAzClientContext3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for IAzClientContext3 {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for &'a IAzClientContext3 {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IAzClientContext3> for IAzClientContext {
    fn from(value: IAzClientContext3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IAzClientContext3> for IAzClientContext {
    fn from(value: &IAzClientContext3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, IAzClientContext> for IAzClientContext3 {
    fn into_param(self) -> ::windows_core::Param<'a, IAzClientContext> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, IAzClientContext> for &'a IAzClientContext3 {
    fn into_param(self) -> ::windows_core::Param<'a, IAzClientContext> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IAzClientContext3> for IAzClientContext2 {
    fn from(value: IAzClientContext3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IAzClientContext3> for IAzClientContext2 {
    fn from(value: &IAzClientContext3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, IAzClientContext2> for IAzClientContext3 {
    fn into_param(self) -> ::windows_core::Param<'a, IAzClientContext2> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, IAzClientContext2> for &'a IAzClientContext3 {
    fn into_param(self) -> ::windows_core::Param<'a, IAzClientContext2> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IAzClientContext3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IAzClientContext3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IAzClientContext3 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IAzClientContext3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAzClientContext3").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IAzClientContext3 {
    type Vtable = IAzClientContext3_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x11894fde_1deb_4b4b_8907_6d1cda1f5d4f);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IAzClientContext3_Vtbl {
    pub base__: IAzClientContext2_Vtbl,
    pub AccessCheck2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrobjectname: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, bstrscopename: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, loperation: i32, plresult: *mut u32) -> ::windows_core::HRESULT,
    pub IsInRoleAssignment: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrscopename: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, bstrrolename: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, pbisinrole: *mut i16) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetOperations: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrscopename: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, ppoperationcollection: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetOperations: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetTasks: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrscopename: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, pptaskcollection: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetTasks: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub BizRuleParameters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppbizruleparam: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    BizRuleParameters: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub BizRuleInterfaces: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppbizruleinterfaces: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    BizRuleInterfaces: usize,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub GetGroups: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrscopename: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, uloptions: AZ_PROP_CONSTANTS, pgrouparray: *mut ::win32_system::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    GetGroups: usize,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Sids: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstringsidarray: *mut ::win32_system::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Sids: usize,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IAzNameResolver(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IAzNameResolver {
    pub unsafe fn NameFromSid<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrsid: Param0, psidtype: *mut i32, pbstrname: *mut ::win32_foundation::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).NameFromSid)(::windows_core::Interface::as_raw(self), bstrsid.into_param().abi(), ::core::mem::transmute(psidtype), ::core::mem::transmute(pbstrname)).ok()
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn NamesFromSids<'a, Param0: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, vsids: Param0, pvsidtypes: *mut ::win32_system::Com::VARIANT, pvnames: *mut ::win32_system::Com::VARIANT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).NamesFromSids)(::windows_core::Interface::as_raw(self), vsids.into_param().abi(), ::core::mem::transmute(pvsidtypes), ::core::mem::transmute(pvnames)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IAzNameResolver> for ::windows_core::IUnknown {
    fn from(value: IAzNameResolver) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IAzNameResolver> for ::windows_core::IUnknown {
    fn from(value: &IAzNameResolver) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IAzNameResolver {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IAzNameResolver {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IAzNameResolver> for ::win32_system::Com::IDispatch {
    fn from(value: IAzNameResolver) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IAzNameResolver> for ::win32_system::Com::IDispatch {
    fn from(value: &IAzNameResolver) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for IAzNameResolver {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for &'a IAzNameResolver {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IAzNameResolver {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IAzNameResolver {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IAzNameResolver {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IAzNameResolver {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAzNameResolver").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IAzNameResolver {
    type Vtable = IAzNameResolver_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x504d0f15_73e2_43df_a870_a64f40714f53);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IAzNameResolver_Vtbl {
    pub base__: ::win32_system::Com::IDispatch_Vtbl,
    pub NameFromSid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrsid: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, psidtype: *mut i32, pbstrname: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub NamesFromSids: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, vsids: ::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>, pvsidtypes: *mut ::win32_system::Com::VARIANT, pvnames: *mut ::win32_system::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    NamesFromSids: usize,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IAzObjectPicker(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IAzObjectPicker {
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetPrincipals<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HWND>, Param1: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, hparentwnd: Param0, bstrtitle: Param1, pvsidtypes: *mut ::win32_system::Com::VARIANT, pvnames: *mut ::win32_system::Com::VARIANT, pvsids: *mut ::win32_system::Com::VARIANT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetPrincipals)(::windows_core::Interface::as_raw(self), hparentwnd.into_param().abi(), bstrtitle.into_param().abi(), ::core::mem::transmute(pvsidtypes), ::core::mem::transmute(pvnames), ::core::mem::transmute(pvsids)).ok()
    }
    pub unsafe fn Name(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).Name)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IAzObjectPicker> for ::windows_core::IUnknown {
    fn from(value: IAzObjectPicker) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IAzObjectPicker> for ::windows_core::IUnknown {
    fn from(value: &IAzObjectPicker) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IAzObjectPicker {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IAzObjectPicker {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IAzObjectPicker> for ::win32_system::Com::IDispatch {
    fn from(value: IAzObjectPicker) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IAzObjectPicker> for ::win32_system::Com::IDispatch {
    fn from(value: &IAzObjectPicker) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for IAzObjectPicker {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for &'a IAzObjectPicker {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IAzObjectPicker {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IAzObjectPicker {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IAzObjectPicker {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IAzObjectPicker {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAzObjectPicker").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IAzObjectPicker {
    type Vtable = IAzObjectPicker_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x63130a48_699a_42d8_bf01_c62ac3fb79f9);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IAzObjectPicker_Vtbl {
    pub base__: ::win32_system::Com::IDispatch_Vtbl,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub GetPrincipals: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hparentwnd: ::win32_foundation::HWND, bstrtitle: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, pvsidtypes: *mut ::win32_system::Com::VARIANT, pvnames: *mut ::win32_system::Com::VARIANT, pvsids: *mut ::win32_system::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    GetPrincipals: usize,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrname: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IAzOperation(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IAzOperation {
    pub unsafe fn Name(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).Name)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetName<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrname: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetName)(::windows_core::Interface::as_raw(self), bstrname.into_param().abi()).ok()
    }
    pub unsafe fn Description(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).Description)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetDescription<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrdescription: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetDescription)(::windows_core::Interface::as_raw(self), bstrdescription.into_param().abi()).ok()
    }
    pub unsafe fn ApplicationData(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).ApplicationData)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetApplicationData<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrapplicationdata: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetApplicationData)(::windows_core::Interface::as_raw(self), bstrapplicationdata.into_param().abi()).ok()
    }
    pub unsafe fn OperationID(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).OperationID)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetOperationID(&self, lprop: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetOperationID)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lprop)).ok()
    }
    pub unsafe fn Writable(&self) -> ::windows_core::Result<::win32_foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::<::win32_foundation::BOOL>::zeroed();
        (::windows_core::Interface::vtable(self).Writable)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BOOL>(result__)
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetProperty<'a, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, lpropid: i32, varreserved: Param1) -> ::windows_core::Result<::win32_system::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).GetProperty)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lpropid), varreserved.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_system::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetProperty<'a, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>, Param2: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, lpropid: i32, varprop: Param1, varreserved: Param2) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetProperty)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lpropid), varprop.into_param().abi(), varreserved.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Submit<'a, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, lflags: i32, varreserved: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Submit)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lflags), varreserved.into_param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IAzOperation> for ::windows_core::IUnknown {
    fn from(value: IAzOperation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IAzOperation> for ::windows_core::IUnknown {
    fn from(value: &IAzOperation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IAzOperation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IAzOperation {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IAzOperation> for ::win32_system::Com::IDispatch {
    fn from(value: IAzOperation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IAzOperation> for ::win32_system::Com::IDispatch {
    fn from(value: &IAzOperation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for IAzOperation {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for &'a IAzOperation {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IAzOperation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IAzOperation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IAzOperation {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IAzOperation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAzOperation").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IAzOperation {
    type Vtable = IAzOperation_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5e56b24f_ea01_4d61_be44_c49b5e4eaf74);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IAzOperation_Vtbl {
    pub base__: ::win32_system::Com::IDispatch_Vtbl,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrname: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub Description: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrdescription: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrdescription: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub ApplicationData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrapplicationdata: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SetApplicationData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrapplicationdata: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub OperationID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plprop: *mut i32) -> ::windows_core::HRESULT,
    pub SetOperationID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lprop: i32) -> ::windows_core::HRESULT,
    pub Writable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfprop: *mut ::win32_foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub GetProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpropid: i32, varreserved: ::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>, pvarprop: *mut ::win32_system::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    GetProperty: usize,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SetProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpropid: i32, varprop: ::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>, varreserved: ::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SetProperty: usize,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Submit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lflags: i32, varreserved: ::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Submit: usize,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IAzOperation2(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IAzOperation2 {
    pub unsafe fn Name(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.Name)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetName<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrname: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetName)(::windows_core::Interface::as_raw(self), bstrname.into_param().abi()).ok()
    }
    pub unsafe fn Description(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.Description)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetDescription<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrdescription: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetDescription)(::windows_core::Interface::as_raw(self), bstrdescription.into_param().abi()).ok()
    }
    pub unsafe fn ApplicationData(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.ApplicationData)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetApplicationData<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrapplicationdata: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetApplicationData)(::windows_core::Interface::as_raw(self), bstrapplicationdata.into_param().abi()).ok()
    }
    pub unsafe fn OperationID(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).base__.OperationID)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetOperationID(&self, lprop: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetOperationID)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lprop)).ok()
    }
    pub unsafe fn Writable(&self) -> ::windows_core::Result<::win32_foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::<::win32_foundation::BOOL>::zeroed();
        (::windows_core::Interface::vtable(self).base__.Writable)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BOOL>(result__)
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetProperty<'a, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, lpropid: i32, varreserved: Param1) -> ::windows_core::Result<::win32_system::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetProperty)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lpropid), varreserved.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_system::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetProperty<'a, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>, Param2: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, lpropid: i32, varprop: Param1, varreserved: Param2) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetProperty)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lpropid), varprop.into_param().abi(), varreserved.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Submit<'a, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, lflags: i32, varreserved: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Submit)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lflags), varreserved.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn RoleAssignments<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrscopename: Param0, brecursive: i16) -> ::windows_core::Result<IAzRoleAssignments> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).RoleAssignments)(::windows_core::Interface::as_raw(self), bstrscopename.into_param().abi(), ::core::mem::transmute(brecursive), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IAzRoleAssignments>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IAzOperation2> for ::windows_core::IUnknown {
    fn from(value: IAzOperation2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IAzOperation2> for ::windows_core::IUnknown {
    fn from(value: &IAzOperation2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IAzOperation2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IAzOperation2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IAzOperation2> for ::win32_system::Com::IDispatch {
    fn from(value: IAzOperation2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IAzOperation2> for ::win32_system::Com::IDispatch {
    fn from(value: &IAzOperation2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for IAzOperation2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for &'a IAzOperation2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IAzOperation2> for IAzOperation {
    fn from(value: IAzOperation2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IAzOperation2> for IAzOperation {
    fn from(value: &IAzOperation2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, IAzOperation> for IAzOperation2 {
    fn into_param(self) -> ::windows_core::Param<'a, IAzOperation> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, IAzOperation> for &'a IAzOperation2 {
    fn into_param(self) -> ::windows_core::Param<'a, IAzOperation> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IAzOperation2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IAzOperation2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IAzOperation2 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IAzOperation2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAzOperation2").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IAzOperation2 {
    type Vtable = IAzOperation2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1f5ea01f_44a2_4184_9c48_a75b4dcc8ccc);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IAzOperation2_Vtbl {
    pub base__: IAzOperation_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub RoleAssignments: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrscopename: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, brecursive: i16, pproleassignments: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    RoleAssignments: usize,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IAzOperations(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IAzOperations {
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn get_Item(&self, index: i32) -> ::windows_core::Result<::win32_system::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).get_Item)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(index), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_system::Com::VARIANT>(result__)
    }
    pub unsafe fn Count(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).Count)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn _NewEnum(&self) -> ::windows_core::Result<::windows_core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
        (::windows_core::Interface::vtable(self)._NewEnum)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::IUnknown>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IAzOperations> for ::windows_core::IUnknown {
    fn from(value: IAzOperations) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IAzOperations> for ::windows_core::IUnknown {
    fn from(value: &IAzOperations) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IAzOperations {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IAzOperations {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IAzOperations> for ::win32_system::Com::IDispatch {
    fn from(value: IAzOperations) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IAzOperations> for ::win32_system::Com::IDispatch {
    fn from(value: &IAzOperations) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for IAzOperations {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for &'a IAzOperations {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IAzOperations {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IAzOperations {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IAzOperations {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IAzOperations {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAzOperations").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IAzOperations {
    type Vtable = IAzOperations_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x90ef9c07_9706_49d9_af80_0438a5f3ec35);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IAzOperations_Vtbl {
    pub base__: ::win32_system::Com::IDispatch_Vtbl,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub get_Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: i32, pvarobtptr: *mut ::win32_system::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    get_Item: usize,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows_core::HRESULT,
    pub _NewEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenumptr: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IAzPrincipalLocator(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IAzPrincipalLocator {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn NameResolver(&self) -> ::windows_core::Result<IAzNameResolver> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).NameResolver)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IAzNameResolver>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ObjectPicker(&self) -> ::windows_core::Result<IAzObjectPicker> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).ObjectPicker)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IAzObjectPicker>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IAzPrincipalLocator> for ::windows_core::IUnknown {
    fn from(value: IAzPrincipalLocator) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IAzPrincipalLocator> for ::windows_core::IUnknown {
    fn from(value: &IAzPrincipalLocator) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IAzPrincipalLocator {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IAzPrincipalLocator {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IAzPrincipalLocator> for ::win32_system::Com::IDispatch {
    fn from(value: IAzPrincipalLocator) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IAzPrincipalLocator> for ::win32_system::Com::IDispatch {
    fn from(value: &IAzPrincipalLocator) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for IAzPrincipalLocator {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for &'a IAzPrincipalLocator {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IAzPrincipalLocator {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IAzPrincipalLocator {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IAzPrincipalLocator {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IAzPrincipalLocator {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAzPrincipalLocator").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IAzPrincipalLocator {
    type Vtable = IAzPrincipalLocator_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe5c3507d_ad6a_4992_9c7f_74ab480b44cc);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IAzPrincipalLocator_Vtbl {
    pub base__: ::win32_system::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub NameResolver: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppnameresolver: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    NameResolver: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub ObjectPicker: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppobjectpicker: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ObjectPicker: usize,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IAzRole(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IAzRole {
    pub unsafe fn Name(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).Name)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetName<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrname: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetName)(::windows_core::Interface::as_raw(self), bstrname.into_param().abi()).ok()
    }
    pub unsafe fn Description(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).Description)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetDescription<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrdescription: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetDescription)(::windows_core::Interface::as_raw(self), bstrdescription.into_param().abi()).ok()
    }
    pub unsafe fn ApplicationData(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).ApplicationData)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetApplicationData<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrapplicationdata: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetApplicationData)(::windows_core::Interface::as_raw(self), bstrapplicationdata.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AddAppMember<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, bstrprop: Param0, varreserved: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).AddAppMember)(::windows_core::Interface::as_raw(self), bstrprop.into_param().abi(), varreserved.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DeleteAppMember<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, bstrprop: Param0, varreserved: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).DeleteAppMember)(::windows_core::Interface::as_raw(self), bstrprop.into_param().abi(), varreserved.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AddTask<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, bstrprop: Param0, varreserved: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).AddTask)(::windows_core::Interface::as_raw(self), bstrprop.into_param().abi(), varreserved.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DeleteTask<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, bstrprop: Param0, varreserved: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).DeleteTask)(::windows_core::Interface::as_raw(self), bstrprop.into_param().abi(), varreserved.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AddOperation<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, bstrprop: Param0, varreserved: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).AddOperation)(::windows_core::Interface::as_raw(self), bstrprop.into_param().abi(), varreserved.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DeleteOperation<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, bstrprop: Param0, varreserved: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).DeleteOperation)(::windows_core::Interface::as_raw(self), bstrprop.into_param().abi(), varreserved.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AddMember<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, bstrprop: Param0, varreserved: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).AddMember)(::windows_core::Interface::as_raw(self), bstrprop.into_param().abi(), varreserved.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DeleteMember<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, bstrprop: Param0, varreserved: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).DeleteMember)(::windows_core::Interface::as_raw(self), bstrprop.into_param().abi(), varreserved.into_param().abi()).ok()
    }
    pub unsafe fn Writable(&self) -> ::windows_core::Result<::win32_foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::<::win32_foundation::BOOL>::zeroed();
        (::windows_core::Interface::vtable(self).Writable)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BOOL>(result__)
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetProperty<'a, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, lpropid: i32, varreserved: Param1) -> ::windows_core::Result<::win32_system::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).GetProperty)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lpropid), varreserved.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_system::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetProperty<'a, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>, Param2: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, lpropid: i32, varprop: Param1, varreserved: Param2) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetProperty)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lpropid), varprop.into_param().abi(), varreserved.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AppMembers(&self) -> ::windows_core::Result<::win32_system::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).AppMembers)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_system::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Members(&self) -> ::windows_core::Result<::win32_system::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).Members)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_system::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Operations(&self) -> ::windows_core::Result<::win32_system::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).Operations)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_system::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Tasks(&self) -> ::windows_core::Result<::win32_system::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).Tasks)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_system::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AddPropertyItem<'a, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>, Param2: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, lpropid: i32, varprop: Param1, varreserved: Param2) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).AddPropertyItem)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lpropid), varprop.into_param().abi(), varreserved.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DeletePropertyItem<'a, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>, Param2: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, lpropid: i32, varprop: Param1, varreserved: Param2) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).DeletePropertyItem)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lpropid), varprop.into_param().abi(), varreserved.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Submit<'a, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, lflags: i32, varreserved: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Submit)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lflags), varreserved.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AddMemberName<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, bstrprop: Param0, varreserved: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).AddMemberName)(::windows_core::Interface::as_raw(self), bstrprop.into_param().abi(), varreserved.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DeleteMemberName<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, bstrprop: Param0, varreserved: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).DeleteMemberName)(::windows_core::Interface::as_raw(self), bstrprop.into_param().abi(), varreserved.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn MembersName(&self) -> ::windows_core::Result<::win32_system::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).MembersName)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_system::Com::VARIANT>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IAzRole> for ::windows_core::IUnknown {
    fn from(value: IAzRole) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IAzRole> for ::windows_core::IUnknown {
    fn from(value: &IAzRole) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IAzRole {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IAzRole {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IAzRole> for ::win32_system::Com::IDispatch {
    fn from(value: IAzRole) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IAzRole> for ::win32_system::Com::IDispatch {
    fn from(value: &IAzRole) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for IAzRole {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for &'a IAzRole {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IAzRole {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IAzRole {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IAzRole {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IAzRole {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAzRole").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IAzRole {
    type Vtable = IAzRole_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x859e0d8d_62d7_41d8_a034_c0cd5d43fdfa);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IAzRole_Vtbl {
    pub base__: ::win32_system::Com::IDispatch_Vtbl,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrname: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub Description: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrdescription: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrdescription: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub ApplicationData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrapplicationdata: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SetApplicationData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrapplicationdata: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub AddAppMember: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrprop: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    AddAppMember: usize,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub DeleteAppMember: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrprop: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    DeleteAppMember: usize,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub AddTask: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrprop: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    AddTask: usize,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub DeleteTask: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrprop: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    DeleteTask: usize,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub AddOperation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrprop: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    AddOperation: usize,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub DeleteOperation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrprop: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    DeleteOperation: usize,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub AddMember: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrprop: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    AddMember: usize,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub DeleteMember: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrprop: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    DeleteMember: usize,
    pub Writable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfprop: *mut ::win32_foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub GetProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpropid: i32, varreserved: ::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>, pvarprop: *mut ::win32_system::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    GetProperty: usize,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SetProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpropid: i32, varprop: ::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>, varreserved: ::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SetProperty: usize,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub AppMembers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvarprop: *mut ::win32_system::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    AppMembers: usize,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Members: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvarprop: *mut ::win32_system::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Members: usize,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Operations: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvarprop: *mut ::win32_system::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Operations: usize,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Tasks: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvarprop: *mut ::win32_system::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Tasks: usize,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub AddPropertyItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpropid: i32, varprop: ::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>, varreserved: ::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    AddPropertyItem: usize,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub DeletePropertyItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpropid: i32, varprop: ::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>, varreserved: ::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    DeletePropertyItem: usize,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Submit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lflags: i32, varreserved: ::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Submit: usize,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub AddMemberName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrprop: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    AddMemberName: usize,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub DeleteMemberName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrprop: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    DeleteMemberName: usize,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub MembersName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvarprop: *mut ::win32_system::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    MembersName: usize,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IAzRoleAssignment(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IAzRoleAssignment {
    pub unsafe fn Name(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.Name)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetName<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrname: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetName)(::windows_core::Interface::as_raw(self), bstrname.into_param().abi()).ok()
    }
    pub unsafe fn Description(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.Description)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetDescription<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrdescription: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetDescription)(::windows_core::Interface::as_raw(self), bstrdescription.into_param().abi()).ok()
    }
    pub unsafe fn ApplicationData(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.ApplicationData)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetApplicationData<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrapplicationdata: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetApplicationData)(::windows_core::Interface::as_raw(self), bstrapplicationdata.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AddAppMember<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, bstrprop: Param0, varreserved: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.AddAppMember)(::windows_core::Interface::as_raw(self), bstrprop.into_param().abi(), varreserved.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DeleteAppMember<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, bstrprop: Param0, varreserved: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.DeleteAppMember)(::windows_core::Interface::as_raw(self), bstrprop.into_param().abi(), varreserved.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AddTask<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, bstrprop: Param0, varreserved: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.AddTask)(::windows_core::Interface::as_raw(self), bstrprop.into_param().abi(), varreserved.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DeleteTask<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, bstrprop: Param0, varreserved: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.DeleteTask)(::windows_core::Interface::as_raw(self), bstrprop.into_param().abi(), varreserved.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AddOperation<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, bstrprop: Param0, varreserved: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.AddOperation)(::windows_core::Interface::as_raw(self), bstrprop.into_param().abi(), varreserved.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DeleteOperation<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, bstrprop: Param0, varreserved: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.DeleteOperation)(::windows_core::Interface::as_raw(self), bstrprop.into_param().abi(), varreserved.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AddMember<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, bstrprop: Param0, varreserved: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.AddMember)(::windows_core::Interface::as_raw(self), bstrprop.into_param().abi(), varreserved.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DeleteMember<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, bstrprop: Param0, varreserved: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.DeleteMember)(::windows_core::Interface::as_raw(self), bstrprop.into_param().abi(), varreserved.into_param().abi()).ok()
    }
    pub unsafe fn Writable(&self) -> ::windows_core::Result<::win32_foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::<::win32_foundation::BOOL>::zeroed();
        (::windows_core::Interface::vtable(self).base__.Writable)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BOOL>(result__)
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetProperty<'a, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, lpropid: i32, varreserved: Param1) -> ::windows_core::Result<::win32_system::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetProperty)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lpropid), varreserved.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_system::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetProperty<'a, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>, Param2: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, lpropid: i32, varprop: Param1, varreserved: Param2) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetProperty)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lpropid), varprop.into_param().abi(), varreserved.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AppMembers(&self) -> ::windows_core::Result<::win32_system::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.AppMembers)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_system::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Members(&self) -> ::windows_core::Result<::win32_system::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.Members)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_system::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Operations(&self) -> ::windows_core::Result<::win32_system::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.Operations)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_system::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Tasks(&self) -> ::windows_core::Result<::win32_system::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.Tasks)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_system::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AddPropertyItem<'a, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>, Param2: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, lpropid: i32, varprop: Param1, varreserved: Param2) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.AddPropertyItem)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lpropid), varprop.into_param().abi(), varreserved.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DeletePropertyItem<'a, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>, Param2: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, lpropid: i32, varprop: Param1, varreserved: Param2) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.DeletePropertyItem)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lpropid), varprop.into_param().abi(), varreserved.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Submit<'a, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, lflags: i32, varreserved: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Submit)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lflags), varreserved.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AddMemberName<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, bstrprop: Param0, varreserved: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.AddMemberName)(::windows_core::Interface::as_raw(self), bstrprop.into_param().abi(), varreserved.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DeleteMemberName<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, bstrprop: Param0, varreserved: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.DeleteMemberName)(::windows_core::Interface::as_raw(self), bstrprop.into_param().abi(), varreserved.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn MembersName(&self) -> ::windows_core::Result<::win32_system::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.MembersName)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_system::Com::VARIANT>(result__)
    }
    pub unsafe fn AddRoleDefinition<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrroledefinition: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).AddRoleDefinition)(::windows_core::Interface::as_raw(self), bstrroledefinition.into_param().abi()).ok()
    }
    pub unsafe fn DeleteRoleDefinition<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrroledefinition: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).DeleteRoleDefinition)(::windows_core::Interface::as_raw(self), bstrroledefinition.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn RoleDefinitions(&self) -> ::windows_core::Result<IAzRoleDefinitions> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).RoleDefinitions)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IAzRoleDefinitions>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Scope(&self) -> ::windows_core::Result<IAzScope> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Scope)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IAzScope>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IAzRoleAssignment> for ::windows_core::IUnknown {
    fn from(value: IAzRoleAssignment) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IAzRoleAssignment> for ::windows_core::IUnknown {
    fn from(value: &IAzRoleAssignment) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IAzRoleAssignment {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IAzRoleAssignment {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IAzRoleAssignment> for ::win32_system::Com::IDispatch {
    fn from(value: IAzRoleAssignment) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IAzRoleAssignment> for ::win32_system::Com::IDispatch {
    fn from(value: &IAzRoleAssignment) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for IAzRoleAssignment {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for &'a IAzRoleAssignment {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IAzRoleAssignment> for IAzRole {
    fn from(value: IAzRoleAssignment) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IAzRoleAssignment> for IAzRole {
    fn from(value: &IAzRoleAssignment) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, IAzRole> for IAzRoleAssignment {
    fn into_param(self) -> ::windows_core::Param<'a, IAzRole> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, IAzRole> for &'a IAzRoleAssignment {
    fn into_param(self) -> ::windows_core::Param<'a, IAzRole> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IAzRoleAssignment {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IAzRoleAssignment {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IAzRoleAssignment {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IAzRoleAssignment {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAzRoleAssignment").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IAzRoleAssignment {
    type Vtable = IAzRoleAssignment_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x55647d31_0d5a_4fa3_b4ac_2b5f9ad5ab76);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IAzRoleAssignment_Vtbl {
    pub base__: IAzRole_Vtbl,
    pub AddRoleDefinition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrroledefinition: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub DeleteRoleDefinition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrroledefinition: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub RoleDefinitions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pproledefinitions: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    RoleDefinitions: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Scope: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppscope: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Scope: usize,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IAzRoleAssignments(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IAzRoleAssignments {
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn get_Item(&self, index: i32) -> ::windows_core::Result<::win32_system::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).get_Item)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(index), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_system::Com::VARIANT>(result__)
    }
    pub unsafe fn Count(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).Count)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn _NewEnum(&self) -> ::windows_core::Result<::windows_core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
        (::windows_core::Interface::vtable(self)._NewEnum)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::IUnknown>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IAzRoleAssignments> for ::windows_core::IUnknown {
    fn from(value: IAzRoleAssignments) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IAzRoleAssignments> for ::windows_core::IUnknown {
    fn from(value: &IAzRoleAssignments) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IAzRoleAssignments {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IAzRoleAssignments {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IAzRoleAssignments> for ::win32_system::Com::IDispatch {
    fn from(value: IAzRoleAssignments) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IAzRoleAssignments> for ::win32_system::Com::IDispatch {
    fn from(value: &IAzRoleAssignments) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for IAzRoleAssignments {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for &'a IAzRoleAssignments {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IAzRoleAssignments {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IAzRoleAssignments {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IAzRoleAssignments {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IAzRoleAssignments {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAzRoleAssignments").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IAzRoleAssignments {
    type Vtable = IAzRoleAssignments_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9c80b900_fceb_4d73_a0f4_c83b0bbf2481);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IAzRoleAssignments_Vtbl {
    pub base__: ::win32_system::Com::IDispatch_Vtbl,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub get_Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: i32, pvarobtptr: *mut ::win32_system::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    get_Item: usize,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows_core::HRESULT,
    pub _NewEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenumptr: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IAzRoleDefinition(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IAzRoleDefinition {
    pub unsafe fn Name(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.Name)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetName<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrname: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetName)(::windows_core::Interface::as_raw(self), bstrname.into_param().abi()).ok()
    }
    pub unsafe fn Description(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.Description)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetDescription<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrdescription: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetDescription)(::windows_core::Interface::as_raw(self), bstrdescription.into_param().abi()).ok()
    }
    pub unsafe fn ApplicationData(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.ApplicationData)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetApplicationData<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrapplicationdata: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetApplicationData)(::windows_core::Interface::as_raw(self), bstrapplicationdata.into_param().abi()).ok()
    }
    pub unsafe fn BizRule(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.BizRule)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetBizRule<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrprop: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetBizRule)(::windows_core::Interface::as_raw(self), bstrprop.into_param().abi()).ok()
    }
    pub unsafe fn BizRuleLanguage(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.BizRuleLanguage)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetBizRuleLanguage<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrprop: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetBizRuleLanguage)(::windows_core::Interface::as_raw(self), bstrprop.into_param().abi()).ok()
    }
    pub unsafe fn BizRuleImportedPath(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.BizRuleImportedPath)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetBizRuleImportedPath<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrprop: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetBizRuleImportedPath)(::windows_core::Interface::as_raw(self), bstrprop.into_param().abi()).ok()
    }
    pub unsafe fn IsRoleDefinition(&self) -> ::windows_core::Result<::win32_foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::<::win32_foundation::BOOL>::zeroed();
        (::windows_core::Interface::vtable(self).base__.IsRoleDefinition)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BOOL>(result__)
    }
    pub unsafe fn SetIsRoleDefinition<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(&self, fprop: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetIsRoleDefinition)(::windows_core::Interface::as_raw(self), fprop.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Operations(&self) -> ::windows_core::Result<::win32_system::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.Operations)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_system::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Tasks(&self) -> ::windows_core::Result<::win32_system::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.Tasks)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_system::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AddOperation<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, bstrop: Param0, varreserved: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.AddOperation)(::windows_core::Interface::as_raw(self), bstrop.into_param().abi(), varreserved.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DeleteOperation<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, bstrop: Param0, varreserved: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.DeleteOperation)(::windows_core::Interface::as_raw(self), bstrop.into_param().abi(), varreserved.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AddTask<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, bstrtask: Param0, varreserved: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.AddTask)(::windows_core::Interface::as_raw(self), bstrtask.into_param().abi(), varreserved.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DeleteTask<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, bstrtask: Param0, varreserved: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.DeleteTask)(::windows_core::Interface::as_raw(self), bstrtask.into_param().abi(), varreserved.into_param().abi()).ok()
    }
    pub unsafe fn Writable(&self) -> ::windows_core::Result<::win32_foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::<::win32_foundation::BOOL>::zeroed();
        (::windows_core::Interface::vtable(self).base__.Writable)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BOOL>(result__)
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetProperty<'a, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, lpropid: i32, varreserved: Param1) -> ::windows_core::Result<::win32_system::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetProperty)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lpropid), varreserved.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_system::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetProperty<'a, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>, Param2: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, lpropid: i32, varprop: Param1, varreserved: Param2) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetProperty)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lpropid), varprop.into_param().abi(), varreserved.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AddPropertyItem<'a, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>, Param2: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, lpropid: i32, varprop: Param1, varreserved: Param2) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.AddPropertyItem)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lpropid), varprop.into_param().abi(), varreserved.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DeletePropertyItem<'a, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>, Param2: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, lpropid: i32, varprop: Param1, varreserved: Param2) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.DeletePropertyItem)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lpropid), varprop.into_param().abi(), varreserved.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Submit<'a, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, lflags: i32, varreserved: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Submit)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lflags), varreserved.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn RoleAssignments<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrscopename: Param0, brecursive: i16) -> ::windows_core::Result<IAzRoleAssignments> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).RoleAssignments)(::windows_core::Interface::as_raw(self), bstrscopename.into_param().abi(), ::core::mem::transmute(brecursive), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IAzRoleAssignments>(result__)
    }
    pub unsafe fn AddRoleDefinition<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrroledefinition: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).AddRoleDefinition)(::windows_core::Interface::as_raw(self), bstrroledefinition.into_param().abi()).ok()
    }
    pub unsafe fn DeleteRoleDefinition<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrroledefinition: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).DeleteRoleDefinition)(::windows_core::Interface::as_raw(self), bstrroledefinition.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn RoleDefinitions(&self) -> ::windows_core::Result<IAzRoleDefinitions> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).RoleDefinitions)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IAzRoleDefinitions>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IAzRoleDefinition> for ::windows_core::IUnknown {
    fn from(value: IAzRoleDefinition) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IAzRoleDefinition> for ::windows_core::IUnknown {
    fn from(value: &IAzRoleDefinition) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IAzRoleDefinition {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IAzRoleDefinition {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IAzRoleDefinition> for ::win32_system::Com::IDispatch {
    fn from(value: IAzRoleDefinition) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IAzRoleDefinition> for ::win32_system::Com::IDispatch {
    fn from(value: &IAzRoleDefinition) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for IAzRoleDefinition {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for &'a IAzRoleDefinition {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IAzRoleDefinition> for IAzTask {
    fn from(value: IAzRoleDefinition) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IAzRoleDefinition> for IAzTask {
    fn from(value: &IAzRoleDefinition) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, IAzTask> for IAzRoleDefinition {
    fn into_param(self) -> ::windows_core::Param<'a, IAzTask> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, IAzTask> for &'a IAzRoleDefinition {
    fn into_param(self) -> ::windows_core::Param<'a, IAzTask> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IAzRoleDefinition {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IAzRoleDefinition {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IAzRoleDefinition {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IAzRoleDefinition {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAzRoleDefinition").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IAzRoleDefinition {
    type Vtable = IAzRoleDefinition_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd97fcea1_2599_44f1_9fc3_58e9fbe09466);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IAzRoleDefinition_Vtbl {
    pub base__: IAzTask_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub RoleAssignments: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrscopename: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, brecursive: i16, pproleassignments: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    RoleAssignments: usize,
    pub AddRoleDefinition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrroledefinition: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub DeleteRoleDefinition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrroledefinition: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub RoleDefinitions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pproledefinitions: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    RoleDefinitions: usize,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IAzRoleDefinitions(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IAzRoleDefinitions {
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn get_Item(&self, index: i32) -> ::windows_core::Result<::win32_system::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).get_Item)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(index), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_system::Com::VARIANT>(result__)
    }
    pub unsafe fn Count(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).Count)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn _NewEnum(&self) -> ::windows_core::Result<::windows_core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
        (::windows_core::Interface::vtable(self)._NewEnum)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::IUnknown>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IAzRoleDefinitions> for ::windows_core::IUnknown {
    fn from(value: IAzRoleDefinitions) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IAzRoleDefinitions> for ::windows_core::IUnknown {
    fn from(value: &IAzRoleDefinitions) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IAzRoleDefinitions {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IAzRoleDefinitions {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IAzRoleDefinitions> for ::win32_system::Com::IDispatch {
    fn from(value: IAzRoleDefinitions) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IAzRoleDefinitions> for ::win32_system::Com::IDispatch {
    fn from(value: &IAzRoleDefinitions) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for IAzRoleDefinitions {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for &'a IAzRoleDefinitions {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IAzRoleDefinitions {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IAzRoleDefinitions {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IAzRoleDefinitions {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IAzRoleDefinitions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAzRoleDefinitions").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IAzRoleDefinitions {
    type Vtable = IAzRoleDefinitions_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x881f25a5_d755_4550_957a_d503a3b34001);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IAzRoleDefinitions_Vtbl {
    pub base__: ::win32_system::Com::IDispatch_Vtbl,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub get_Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: i32, pvarobtptr: *mut ::win32_system::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    get_Item: usize,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows_core::HRESULT,
    pub _NewEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenumptr: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IAzRoles(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IAzRoles {
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn get_Item(&self, index: i32) -> ::windows_core::Result<::win32_system::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).get_Item)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(index), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_system::Com::VARIANT>(result__)
    }
    pub unsafe fn Count(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).Count)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn _NewEnum(&self) -> ::windows_core::Result<::windows_core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
        (::windows_core::Interface::vtable(self)._NewEnum)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::IUnknown>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IAzRoles> for ::windows_core::IUnknown {
    fn from(value: IAzRoles) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IAzRoles> for ::windows_core::IUnknown {
    fn from(value: &IAzRoles) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IAzRoles {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IAzRoles {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IAzRoles> for ::win32_system::Com::IDispatch {
    fn from(value: IAzRoles) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IAzRoles> for ::win32_system::Com::IDispatch {
    fn from(value: &IAzRoles) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for IAzRoles {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for &'a IAzRoles {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IAzRoles {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IAzRoles {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IAzRoles {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IAzRoles {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAzRoles").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IAzRoles {
    type Vtable = IAzRoles_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x95e0f119_13b4_4dae_b65f_2f7d60d822e4);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IAzRoles_Vtbl {
    pub base__: ::win32_system::Com::IDispatch_Vtbl,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub get_Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: i32, pvarobtptr: *mut ::win32_system::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    get_Item: usize,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows_core::HRESULT,
    pub _NewEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenumptr: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IAzScope(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IAzScope {
    pub unsafe fn Name(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).Name)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetName<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrname: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetName)(::windows_core::Interface::as_raw(self), bstrname.into_param().abi()).ok()
    }
    pub unsafe fn Description(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).Description)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetDescription<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrdescription: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetDescription)(::windows_core::Interface::as_raw(self), bstrdescription.into_param().abi()).ok()
    }
    pub unsafe fn ApplicationData(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).ApplicationData)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetApplicationData<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrapplicationdata: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetApplicationData)(::windows_core::Interface::as_raw(self), bstrapplicationdata.into_param().abi()).ok()
    }
    pub unsafe fn Writable(&self) -> ::windows_core::Result<::win32_foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::<::win32_foundation::BOOL>::zeroed();
        (::windows_core::Interface::vtable(self).Writable)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BOOL>(result__)
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetProperty<'a, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, lpropid: i32, varreserved: Param1) -> ::windows_core::Result<::win32_system::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).GetProperty)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lpropid), varreserved.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_system::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetProperty<'a, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>, Param2: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, lpropid: i32, varprop: Param1, varreserved: Param2) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetProperty)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lpropid), varprop.into_param().abi(), varreserved.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AddPropertyItem<'a, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>, Param2: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, lpropid: i32, varprop: Param1, varreserved: Param2) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).AddPropertyItem)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lpropid), varprop.into_param().abi(), varreserved.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DeletePropertyItem<'a, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>, Param2: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, lpropid: i32, varprop: Param1, varreserved: Param2) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).DeletePropertyItem)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lpropid), varprop.into_param().abi(), varreserved.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn PolicyAdministrators(&self) -> ::windows_core::Result<::win32_system::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).PolicyAdministrators)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_system::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn PolicyReaders(&self) -> ::windows_core::Result<::win32_system::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).PolicyReaders)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_system::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AddPolicyAdministrator<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, bstradmin: Param0, varreserved: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).AddPolicyAdministrator)(::windows_core::Interface::as_raw(self), bstradmin.into_param().abi(), varreserved.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DeletePolicyAdministrator<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, bstradmin: Param0, varreserved: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).DeletePolicyAdministrator)(::windows_core::Interface::as_raw(self), bstradmin.into_param().abi(), varreserved.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AddPolicyReader<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, bstrreader: Param0, varreserved: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).AddPolicyReader)(::windows_core::Interface::as_raw(self), bstrreader.into_param().abi(), varreserved.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DeletePolicyReader<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, bstrreader: Param0, varreserved: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).DeletePolicyReader)(::windows_core::Interface::as_raw(self), bstrreader.into_param().abi(), varreserved.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ApplicationGroups(&self) -> ::windows_core::Result<IAzApplicationGroups> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).ApplicationGroups)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IAzApplicationGroups>(result__)
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn OpenApplicationGroup<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, bstrgroupname: Param0, varreserved: Param1) -> ::windows_core::Result<IAzApplicationGroup> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).OpenApplicationGroup)(::windows_core::Interface::as_raw(self), bstrgroupname.into_param().abi(), varreserved.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IAzApplicationGroup>(result__)
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn CreateApplicationGroup<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, bstrgroupname: Param0, varreserved: Param1) -> ::windows_core::Result<IAzApplicationGroup> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).CreateApplicationGroup)(::windows_core::Interface::as_raw(self), bstrgroupname.into_param().abi(), varreserved.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IAzApplicationGroup>(result__)
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DeleteApplicationGroup<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, bstrgroupname: Param0, varreserved: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).DeleteApplicationGroup)(::windows_core::Interface::as_raw(self), bstrgroupname.into_param().abi(), varreserved.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Roles(&self) -> ::windows_core::Result<IAzRoles> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Roles)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IAzRoles>(result__)
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn OpenRole<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, bstrrolename: Param0, varreserved: Param1) -> ::windows_core::Result<IAzRole> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).OpenRole)(::windows_core::Interface::as_raw(self), bstrrolename.into_param().abi(), varreserved.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IAzRole>(result__)
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn CreateRole<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, bstrrolename: Param0, varreserved: Param1) -> ::windows_core::Result<IAzRole> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).CreateRole)(::windows_core::Interface::as_raw(self), bstrrolename.into_param().abi(), varreserved.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IAzRole>(result__)
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DeleteRole<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, bstrrolename: Param0, varreserved: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).DeleteRole)(::windows_core::Interface::as_raw(self), bstrrolename.into_param().abi(), varreserved.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Tasks(&self) -> ::windows_core::Result<IAzTasks> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Tasks)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IAzTasks>(result__)
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn OpenTask<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, bstrtaskname: Param0, varreserved: Param1) -> ::windows_core::Result<IAzTask> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).OpenTask)(::windows_core::Interface::as_raw(self), bstrtaskname.into_param().abi(), varreserved.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IAzTask>(result__)
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn CreateTask<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, bstrtaskname: Param0, varreserved: Param1) -> ::windows_core::Result<IAzTask> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).CreateTask)(::windows_core::Interface::as_raw(self), bstrtaskname.into_param().abi(), varreserved.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IAzTask>(result__)
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DeleteTask<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, bstrtaskname: Param0, varreserved: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).DeleteTask)(::windows_core::Interface::as_raw(self), bstrtaskname.into_param().abi(), varreserved.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Submit<'a, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, lflags: i32, varreserved: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Submit)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lflags), varreserved.into_param().abi()).ok()
    }
    pub unsafe fn CanBeDelegated(&self) -> ::windows_core::Result<::win32_foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::<::win32_foundation::BOOL>::zeroed();
        (::windows_core::Interface::vtable(self).CanBeDelegated)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BOOL>(result__)
    }
    pub unsafe fn BizrulesWritable(&self) -> ::windows_core::Result<::win32_foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::<::win32_foundation::BOOL>::zeroed();
        (::windows_core::Interface::vtable(self).BizrulesWritable)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BOOL>(result__)
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn PolicyAdministratorsName(&self) -> ::windows_core::Result<::win32_system::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).PolicyAdministratorsName)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_system::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn PolicyReadersName(&self) -> ::windows_core::Result<::win32_system::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).PolicyReadersName)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_system::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AddPolicyAdministratorName<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, bstradmin: Param0, varreserved: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).AddPolicyAdministratorName)(::windows_core::Interface::as_raw(self), bstradmin.into_param().abi(), varreserved.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DeletePolicyAdministratorName<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, bstradmin: Param0, varreserved: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).DeletePolicyAdministratorName)(::windows_core::Interface::as_raw(self), bstradmin.into_param().abi(), varreserved.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AddPolicyReaderName<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, bstrreader: Param0, varreserved: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).AddPolicyReaderName)(::windows_core::Interface::as_raw(self), bstrreader.into_param().abi(), varreserved.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DeletePolicyReaderName<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, bstrreader: Param0, varreserved: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).DeletePolicyReaderName)(::windows_core::Interface::as_raw(self), bstrreader.into_param().abi(), varreserved.into_param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IAzScope> for ::windows_core::IUnknown {
    fn from(value: IAzScope) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IAzScope> for ::windows_core::IUnknown {
    fn from(value: &IAzScope) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IAzScope {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IAzScope {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IAzScope> for ::win32_system::Com::IDispatch {
    fn from(value: IAzScope) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IAzScope> for ::win32_system::Com::IDispatch {
    fn from(value: &IAzScope) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for IAzScope {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for &'a IAzScope {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IAzScope {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IAzScope {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IAzScope {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IAzScope {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAzScope").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IAzScope {
    type Vtable = IAzScope_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x00e52487_e08d_4514_b62e_877d5645f5ab);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IAzScope_Vtbl {
    pub base__: ::win32_system::Com::IDispatch_Vtbl,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrname: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub Description: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrdescription: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrdescription: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub ApplicationData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrapplicationdata: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SetApplicationData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrapplicationdata: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub Writable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfprop: *mut ::win32_foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub GetProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpropid: i32, varreserved: ::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>, pvarprop: *mut ::win32_system::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    GetProperty: usize,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SetProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpropid: i32, varprop: ::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>, varreserved: ::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SetProperty: usize,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub AddPropertyItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpropid: i32, varprop: ::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>, varreserved: ::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    AddPropertyItem: usize,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub DeletePropertyItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpropid: i32, varprop: ::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>, varreserved: ::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    DeletePropertyItem: usize,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub PolicyAdministrators: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvaradmins: *mut ::win32_system::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    PolicyAdministrators: usize,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub PolicyReaders: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvarreaders: *mut ::win32_system::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    PolicyReaders: usize,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub AddPolicyAdministrator: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstradmin: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    AddPolicyAdministrator: usize,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub DeletePolicyAdministrator: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstradmin: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    DeletePolicyAdministrator: usize,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub AddPolicyReader: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrreader: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    AddPolicyReader: usize,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub DeletePolicyReader: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrreader: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    DeletePolicyReader: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub ApplicationGroups: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppgroupcollection: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ApplicationGroups: usize,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub OpenApplicationGroup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrgroupname: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>, ppgroup: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    OpenApplicationGroup: usize,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub CreateApplicationGroup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrgroupname: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>, ppgroup: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    CreateApplicationGroup: usize,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub DeleteApplicationGroup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrgroupname: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    DeleteApplicationGroup: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Roles: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pprolecollection: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Roles: usize,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub OpenRole: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrrolename: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>, pprole: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    OpenRole: usize,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub CreateRole: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrrolename: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>, pprole: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    CreateRole: usize,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub DeleteRole: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrrolename: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    DeleteRole: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Tasks: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pptaskcollection: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Tasks: usize,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub OpenTask: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrtaskname: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>, pptask: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    OpenTask: usize,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub CreateTask: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrtaskname: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>, pptask: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    CreateTask: usize,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub DeleteTask: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrtaskname: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    DeleteTask: usize,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Submit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lflags: i32, varreserved: ::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Submit: usize,
    pub CanBeDelegated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfprop: *mut ::win32_foundation::BOOL) -> ::windows_core::HRESULT,
    pub BizrulesWritable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfprop: *mut ::win32_foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub PolicyAdministratorsName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvaradmins: *mut ::win32_system::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    PolicyAdministratorsName: usize,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub PolicyReadersName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvarreaders: *mut ::win32_system::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    PolicyReadersName: usize,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub AddPolicyAdministratorName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstradmin: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    AddPolicyAdministratorName: usize,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub DeletePolicyAdministratorName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstradmin: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    DeletePolicyAdministratorName: usize,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub AddPolicyReaderName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrreader: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    AddPolicyReaderName: usize,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub DeletePolicyReaderName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrreader: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    DeletePolicyReaderName: usize,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IAzScope2(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IAzScope2 {
    pub unsafe fn Name(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.Name)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetName<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrname: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetName)(::windows_core::Interface::as_raw(self), bstrname.into_param().abi()).ok()
    }
    pub unsafe fn Description(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.Description)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetDescription<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrdescription: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetDescription)(::windows_core::Interface::as_raw(self), bstrdescription.into_param().abi()).ok()
    }
    pub unsafe fn ApplicationData(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.ApplicationData)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetApplicationData<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrapplicationdata: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetApplicationData)(::windows_core::Interface::as_raw(self), bstrapplicationdata.into_param().abi()).ok()
    }
    pub unsafe fn Writable(&self) -> ::windows_core::Result<::win32_foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::<::win32_foundation::BOOL>::zeroed();
        (::windows_core::Interface::vtable(self).base__.Writable)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BOOL>(result__)
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetProperty<'a, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, lpropid: i32, varreserved: Param1) -> ::windows_core::Result<::win32_system::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetProperty)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lpropid), varreserved.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_system::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetProperty<'a, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>, Param2: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, lpropid: i32, varprop: Param1, varreserved: Param2) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetProperty)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lpropid), varprop.into_param().abi(), varreserved.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AddPropertyItem<'a, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>, Param2: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, lpropid: i32, varprop: Param1, varreserved: Param2) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.AddPropertyItem)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lpropid), varprop.into_param().abi(), varreserved.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DeletePropertyItem<'a, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>, Param2: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, lpropid: i32, varprop: Param1, varreserved: Param2) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.DeletePropertyItem)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lpropid), varprop.into_param().abi(), varreserved.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn PolicyAdministrators(&self) -> ::windows_core::Result<::win32_system::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.PolicyAdministrators)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_system::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn PolicyReaders(&self) -> ::windows_core::Result<::win32_system::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.PolicyReaders)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_system::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AddPolicyAdministrator<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, bstradmin: Param0, varreserved: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.AddPolicyAdministrator)(::windows_core::Interface::as_raw(self), bstradmin.into_param().abi(), varreserved.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DeletePolicyAdministrator<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, bstradmin: Param0, varreserved: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.DeletePolicyAdministrator)(::windows_core::Interface::as_raw(self), bstradmin.into_param().abi(), varreserved.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AddPolicyReader<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, bstrreader: Param0, varreserved: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.AddPolicyReader)(::windows_core::Interface::as_raw(self), bstrreader.into_param().abi(), varreserved.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DeletePolicyReader<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, bstrreader: Param0, varreserved: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.DeletePolicyReader)(::windows_core::Interface::as_raw(self), bstrreader.into_param().abi(), varreserved.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ApplicationGroups(&self) -> ::windows_core::Result<IAzApplicationGroups> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.ApplicationGroups)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IAzApplicationGroups>(result__)
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn OpenApplicationGroup<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, bstrgroupname: Param0, varreserved: Param1) -> ::windows_core::Result<IAzApplicationGroup> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.OpenApplicationGroup)(::windows_core::Interface::as_raw(self), bstrgroupname.into_param().abi(), varreserved.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IAzApplicationGroup>(result__)
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn CreateApplicationGroup<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, bstrgroupname: Param0, varreserved: Param1) -> ::windows_core::Result<IAzApplicationGroup> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.CreateApplicationGroup)(::windows_core::Interface::as_raw(self), bstrgroupname.into_param().abi(), varreserved.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IAzApplicationGroup>(result__)
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DeleteApplicationGroup<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, bstrgroupname: Param0, varreserved: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.DeleteApplicationGroup)(::windows_core::Interface::as_raw(self), bstrgroupname.into_param().abi(), varreserved.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Roles(&self) -> ::windows_core::Result<IAzRoles> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.Roles)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IAzRoles>(result__)
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn OpenRole<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, bstrrolename: Param0, varreserved: Param1) -> ::windows_core::Result<IAzRole> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.OpenRole)(::windows_core::Interface::as_raw(self), bstrrolename.into_param().abi(), varreserved.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IAzRole>(result__)
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn CreateRole<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, bstrrolename: Param0, varreserved: Param1) -> ::windows_core::Result<IAzRole> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.CreateRole)(::windows_core::Interface::as_raw(self), bstrrolename.into_param().abi(), varreserved.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IAzRole>(result__)
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DeleteRole<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, bstrrolename: Param0, varreserved: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.DeleteRole)(::windows_core::Interface::as_raw(self), bstrrolename.into_param().abi(), varreserved.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Tasks(&self) -> ::windows_core::Result<IAzTasks> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.Tasks)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IAzTasks>(result__)
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn OpenTask<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, bstrtaskname: Param0, varreserved: Param1) -> ::windows_core::Result<IAzTask> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.OpenTask)(::windows_core::Interface::as_raw(self), bstrtaskname.into_param().abi(), varreserved.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IAzTask>(result__)
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn CreateTask<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, bstrtaskname: Param0, varreserved: Param1) -> ::windows_core::Result<IAzTask> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).base__.CreateTask)(::windows_core::Interface::as_raw(self), bstrtaskname.into_param().abi(), varreserved.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IAzTask>(result__)
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DeleteTask<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, bstrtaskname: Param0, varreserved: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.DeleteTask)(::windows_core::Interface::as_raw(self), bstrtaskname.into_param().abi(), varreserved.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Submit<'a, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, lflags: i32, varreserved: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Submit)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lflags), varreserved.into_param().abi()).ok()
    }
    pub unsafe fn CanBeDelegated(&self) -> ::windows_core::Result<::win32_foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::<::win32_foundation::BOOL>::zeroed();
        (::windows_core::Interface::vtable(self).base__.CanBeDelegated)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BOOL>(result__)
    }
    pub unsafe fn BizrulesWritable(&self) -> ::windows_core::Result<::win32_foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::<::win32_foundation::BOOL>::zeroed();
        (::windows_core::Interface::vtable(self).base__.BizrulesWritable)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BOOL>(result__)
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn PolicyAdministratorsName(&self) -> ::windows_core::Result<::win32_system::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.PolicyAdministratorsName)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_system::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn PolicyReadersName(&self) -> ::windows_core::Result<::win32_system::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.PolicyReadersName)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_system::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AddPolicyAdministratorName<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, bstradmin: Param0, varreserved: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.AddPolicyAdministratorName)(::windows_core::Interface::as_raw(self), bstradmin.into_param().abi(), varreserved.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DeletePolicyAdministratorName<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, bstradmin: Param0, varreserved: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.DeletePolicyAdministratorName)(::windows_core::Interface::as_raw(self), bstradmin.into_param().abi(), varreserved.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AddPolicyReaderName<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, bstrreader: Param0, varreserved: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.AddPolicyReaderName)(::windows_core::Interface::as_raw(self), bstrreader.into_param().abi(), varreserved.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DeletePolicyReaderName<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, bstrreader: Param0, varreserved: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.DeletePolicyReaderName)(::windows_core::Interface::as_raw(self), bstrreader.into_param().abi(), varreserved.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn RoleDefinitions(&self) -> ::windows_core::Result<IAzRoleDefinitions> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).RoleDefinitions)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IAzRoleDefinitions>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateRoleDefinition<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrroledefinitionname: Param0) -> ::windows_core::Result<IAzRoleDefinition> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).CreateRoleDefinition)(::windows_core::Interface::as_raw(self), bstrroledefinitionname.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IAzRoleDefinition>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OpenRoleDefinition<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrroledefinitionname: Param0) -> ::windows_core::Result<IAzRoleDefinition> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).OpenRoleDefinition)(::windows_core::Interface::as_raw(self), bstrroledefinitionname.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IAzRoleDefinition>(result__)
    }
    pub unsafe fn DeleteRoleDefinition<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrroledefinitionname: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).DeleteRoleDefinition)(::windows_core::Interface::as_raw(self), bstrroledefinitionname.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn RoleAssignments(&self) -> ::windows_core::Result<IAzRoleAssignments> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).RoleAssignments)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IAzRoleAssignments>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateRoleAssignment<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrroleassignmentname: Param0) -> ::windows_core::Result<IAzRoleAssignment> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).CreateRoleAssignment)(::windows_core::Interface::as_raw(self), bstrroleassignmentname.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IAzRoleAssignment>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OpenRoleAssignment<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrroleassignmentname: Param0) -> ::windows_core::Result<IAzRoleAssignment> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).OpenRoleAssignment)(::windows_core::Interface::as_raw(self), bstrroleassignmentname.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IAzRoleAssignment>(result__)
    }
    pub unsafe fn DeleteRoleAssignment<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrroleassignmentname: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).DeleteRoleAssignment)(::windows_core::Interface::as_raw(self), bstrroleassignmentname.into_param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IAzScope2> for ::windows_core::IUnknown {
    fn from(value: IAzScope2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IAzScope2> for ::windows_core::IUnknown {
    fn from(value: &IAzScope2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IAzScope2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IAzScope2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IAzScope2> for ::win32_system::Com::IDispatch {
    fn from(value: IAzScope2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IAzScope2> for ::win32_system::Com::IDispatch {
    fn from(value: &IAzScope2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for IAzScope2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for &'a IAzScope2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IAzScope2> for IAzScope {
    fn from(value: IAzScope2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IAzScope2> for IAzScope {
    fn from(value: &IAzScope2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, IAzScope> for IAzScope2 {
    fn into_param(self) -> ::windows_core::Param<'a, IAzScope> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, IAzScope> for &'a IAzScope2 {
    fn into_param(self) -> ::windows_core::Param<'a, IAzScope> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IAzScope2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IAzScope2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IAzScope2 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IAzScope2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAzScope2").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IAzScope2 {
    type Vtable = IAzScope2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xee9fe8c9_c9f3_40e2_aa12_d1d8599727fd);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IAzScope2_Vtbl {
    pub base__: IAzScope_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub RoleDefinitions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pproledefinitions: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    RoleDefinitions: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateRoleDefinition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrroledefinitionname: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, pproledefinitions: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateRoleDefinition: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub OpenRoleDefinition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrroledefinitionname: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, pproledefinitions: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    OpenRoleDefinition: usize,
    pub DeleteRoleDefinition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrroledefinitionname: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub RoleAssignments: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pproleassignments: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    RoleAssignments: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateRoleAssignment: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrroleassignmentname: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, pproleassignment: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateRoleAssignment: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub OpenRoleAssignment: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrroleassignmentname: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, pproleassignment: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    OpenRoleAssignment: usize,
    pub DeleteRoleAssignment: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrroleassignmentname: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IAzScopes(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IAzScopes {
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn get_Item(&self, index: i32) -> ::windows_core::Result<::win32_system::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).get_Item)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(index), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_system::Com::VARIANT>(result__)
    }
    pub unsafe fn Count(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).Count)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn _NewEnum(&self) -> ::windows_core::Result<::windows_core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
        (::windows_core::Interface::vtable(self)._NewEnum)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::IUnknown>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IAzScopes> for ::windows_core::IUnknown {
    fn from(value: IAzScopes) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IAzScopes> for ::windows_core::IUnknown {
    fn from(value: &IAzScopes) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IAzScopes {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IAzScopes {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IAzScopes> for ::win32_system::Com::IDispatch {
    fn from(value: IAzScopes) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IAzScopes> for ::win32_system::Com::IDispatch {
    fn from(value: &IAzScopes) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for IAzScopes {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for &'a IAzScopes {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IAzScopes {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IAzScopes {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IAzScopes {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IAzScopes {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAzScopes").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IAzScopes {
    type Vtable = IAzScopes_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x78e14853_9f5e_406d_9b91_6bdba6973510);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IAzScopes_Vtbl {
    pub base__: ::win32_system::Com::IDispatch_Vtbl,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub get_Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: i32, pvarobtptr: *mut ::win32_system::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    get_Item: usize,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows_core::HRESULT,
    pub _NewEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenumptr: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IAzTask(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IAzTask {
    pub unsafe fn Name(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).Name)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetName<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrname: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetName)(::windows_core::Interface::as_raw(self), bstrname.into_param().abi()).ok()
    }
    pub unsafe fn Description(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).Description)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetDescription<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrdescription: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetDescription)(::windows_core::Interface::as_raw(self), bstrdescription.into_param().abi()).ok()
    }
    pub unsafe fn ApplicationData(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).ApplicationData)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetApplicationData<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrapplicationdata: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetApplicationData)(::windows_core::Interface::as_raw(self), bstrapplicationdata.into_param().abi()).ok()
    }
    pub unsafe fn BizRule(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).BizRule)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetBizRule<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrprop: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetBizRule)(::windows_core::Interface::as_raw(self), bstrprop.into_param().abi()).ok()
    }
    pub unsafe fn BizRuleLanguage(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).BizRuleLanguage)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetBizRuleLanguage<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrprop: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetBizRuleLanguage)(::windows_core::Interface::as_raw(self), bstrprop.into_param().abi()).ok()
    }
    pub unsafe fn BizRuleImportedPath(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).BizRuleImportedPath)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetBizRuleImportedPath<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrprop: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetBizRuleImportedPath)(::windows_core::Interface::as_raw(self), bstrprop.into_param().abi()).ok()
    }
    pub unsafe fn IsRoleDefinition(&self) -> ::windows_core::Result<::win32_foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::<::win32_foundation::BOOL>::zeroed();
        (::windows_core::Interface::vtable(self).IsRoleDefinition)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BOOL>(result__)
    }
    pub unsafe fn SetIsRoleDefinition<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(&self, fprop: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetIsRoleDefinition)(::windows_core::Interface::as_raw(self), fprop.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Operations(&self) -> ::windows_core::Result<::win32_system::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).Operations)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_system::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Tasks(&self) -> ::windows_core::Result<::win32_system::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).Tasks)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_system::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AddOperation<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, bstrop: Param0, varreserved: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).AddOperation)(::windows_core::Interface::as_raw(self), bstrop.into_param().abi(), varreserved.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DeleteOperation<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, bstrop: Param0, varreserved: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).DeleteOperation)(::windows_core::Interface::as_raw(self), bstrop.into_param().abi(), varreserved.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AddTask<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, bstrtask: Param0, varreserved: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).AddTask)(::windows_core::Interface::as_raw(self), bstrtask.into_param().abi(), varreserved.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DeleteTask<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, bstrtask: Param0, varreserved: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).DeleteTask)(::windows_core::Interface::as_raw(self), bstrtask.into_param().abi(), varreserved.into_param().abi()).ok()
    }
    pub unsafe fn Writable(&self) -> ::windows_core::Result<::win32_foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::<::win32_foundation::BOOL>::zeroed();
        (::windows_core::Interface::vtable(self).Writable)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BOOL>(result__)
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetProperty<'a, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, lpropid: i32, varreserved: Param1) -> ::windows_core::Result<::win32_system::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).GetProperty)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lpropid), varreserved.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_system::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetProperty<'a, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>, Param2: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, lpropid: i32, varprop: Param1, varreserved: Param2) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetProperty)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lpropid), varprop.into_param().abi(), varreserved.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AddPropertyItem<'a, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>, Param2: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, lpropid: i32, varprop: Param1, varreserved: Param2) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).AddPropertyItem)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lpropid), varprop.into_param().abi(), varreserved.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DeletePropertyItem<'a, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>, Param2: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, lpropid: i32, varprop: Param1, varreserved: Param2) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).DeletePropertyItem)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lpropid), varprop.into_param().abi(), varreserved.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Submit<'a, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, lflags: i32, varreserved: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Submit)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lflags), varreserved.into_param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IAzTask> for ::windows_core::IUnknown {
    fn from(value: IAzTask) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IAzTask> for ::windows_core::IUnknown {
    fn from(value: &IAzTask) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IAzTask {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IAzTask {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IAzTask> for ::win32_system::Com::IDispatch {
    fn from(value: IAzTask) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IAzTask> for ::win32_system::Com::IDispatch {
    fn from(value: &IAzTask) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for IAzTask {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for &'a IAzTask {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IAzTask {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IAzTask {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IAzTask {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IAzTask {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAzTask").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IAzTask {
    type Vtable = IAzTask_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xcb94e592_2e0e_4a6c_a336_b89a6dc1e388);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IAzTask_Vtbl {
    pub base__: ::win32_system::Com::IDispatch_Vtbl,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrname: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub Description: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrdescription: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrdescription: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub ApplicationData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrapplicationdata: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SetApplicationData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrapplicationdata: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub BizRule: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrprop: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SetBizRule: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrprop: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub BizRuleLanguage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrprop: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SetBizRuleLanguage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrprop: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub BizRuleImportedPath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrprop: *mut ::win32_foundation::BSTR) -> ::windows_core::HRESULT,
    pub SetBizRuleImportedPath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrprop: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>) -> ::windows_core::HRESULT,
    pub IsRoleDefinition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfprop: *mut ::win32_foundation::BOOL) -> ::windows_core::HRESULT,
    pub SetIsRoleDefinition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fprop: ::win32_foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Operations: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvarprop: *mut ::win32_system::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Operations: usize,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Tasks: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvarprop: *mut ::win32_system::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Tasks: usize,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub AddOperation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrop: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    AddOperation: usize,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub DeleteOperation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrop: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    DeleteOperation: usize,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub AddTask: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrtask: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    AddTask: usize,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub DeleteTask: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrtask: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    DeleteTask: usize,
    pub Writable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfprop: *mut ::win32_foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub GetProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpropid: i32, varreserved: ::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>, pvarprop: *mut ::win32_system::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    GetProperty: usize,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SetProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpropid: i32, varprop: ::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>, varreserved: ::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SetProperty: usize,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub AddPropertyItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpropid: i32, varprop: ::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>, varreserved: ::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    AddPropertyItem: usize,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub DeletePropertyItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpropid: i32, varprop: ::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>, varreserved: ::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    DeletePropertyItem: usize,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Submit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lflags: i32, varreserved: ::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Submit: usize,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IAzTask2(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IAzTask2 {
    pub unsafe fn Name(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.Name)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetName<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrname: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetName)(::windows_core::Interface::as_raw(self), bstrname.into_param().abi()).ok()
    }
    pub unsafe fn Description(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.Description)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetDescription<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrdescription: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetDescription)(::windows_core::Interface::as_raw(self), bstrdescription.into_param().abi()).ok()
    }
    pub unsafe fn ApplicationData(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.ApplicationData)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetApplicationData<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrapplicationdata: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetApplicationData)(::windows_core::Interface::as_raw(self), bstrapplicationdata.into_param().abi()).ok()
    }
    pub unsafe fn BizRule(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.BizRule)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetBizRule<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrprop: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetBizRule)(::windows_core::Interface::as_raw(self), bstrprop.into_param().abi()).ok()
    }
    pub unsafe fn BizRuleLanguage(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.BizRuleLanguage)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetBizRuleLanguage<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrprop: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetBizRuleLanguage)(::windows_core::Interface::as_raw(self), bstrprop.into_param().abi()).ok()
    }
    pub unsafe fn BizRuleImportedPath(&self) -> ::windows_core::Result<::win32_foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.BizRuleImportedPath)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BSTR>(result__)
    }
    pub unsafe fn SetBizRuleImportedPath<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrprop: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetBizRuleImportedPath)(::windows_core::Interface::as_raw(self), bstrprop.into_param().abi()).ok()
    }
    pub unsafe fn IsRoleDefinition(&self) -> ::windows_core::Result<::win32_foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::<::win32_foundation::BOOL>::zeroed();
        (::windows_core::Interface::vtable(self).base__.IsRoleDefinition)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BOOL>(result__)
    }
    pub unsafe fn SetIsRoleDefinition<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(&self, fprop: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetIsRoleDefinition)(::windows_core::Interface::as_raw(self), fprop.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Operations(&self) -> ::windows_core::Result<::win32_system::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.Operations)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_system::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Tasks(&self) -> ::windows_core::Result<::win32_system::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.Tasks)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_system::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AddOperation<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, bstrop: Param0, varreserved: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.AddOperation)(::windows_core::Interface::as_raw(self), bstrop.into_param().abi(), varreserved.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DeleteOperation<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, bstrop: Param0, varreserved: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.DeleteOperation)(::windows_core::Interface::as_raw(self), bstrop.into_param().abi(), varreserved.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AddTask<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, bstrtask: Param0, varreserved: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.AddTask)(::windows_core::Interface::as_raw(self), bstrtask.into_param().abi(), varreserved.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DeleteTask<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, bstrtask: Param0, varreserved: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.DeleteTask)(::windows_core::Interface::as_raw(self), bstrtask.into_param().abi(), varreserved.into_param().abi()).ok()
    }
    pub unsafe fn Writable(&self) -> ::windows_core::Result<::win32_foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::<::win32_foundation::BOOL>::zeroed();
        (::windows_core::Interface::vtable(self).base__.Writable)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::BOOL>(result__)
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetProperty<'a, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, lpropid: i32, varreserved: Param1) -> ::windows_core::Result<::win32_system::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetProperty)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lpropid), varreserved.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_system::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetProperty<'a, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>, Param2: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, lpropid: i32, varprop: Param1, varreserved: Param2) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetProperty)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lpropid), varprop.into_param().abi(), varreserved.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AddPropertyItem<'a, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>, Param2: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, lpropid: i32, varprop: Param1, varreserved: Param2) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.AddPropertyItem)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lpropid), varprop.into_param().abi(), varreserved.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DeletePropertyItem<'a, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>, Param2: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, lpropid: i32, varprop: Param1, varreserved: Param2) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.DeletePropertyItem)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lpropid), varprop.into_param().abi(), varreserved.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Submit<'a, Param1: ::windows_core::IntoParam<'a, ::win32_system::Com::VARIANT>>(&self, lflags: i32, varreserved: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Submit)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(lflags), varreserved.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn RoleAssignments<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::BSTR>>(&self, bstrscopename: Param0, brecursive: i16) -> ::windows_core::Result<IAzRoleAssignments> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).RoleAssignments)(::windows_core::Interface::as_raw(self), bstrscopename.into_param().abi(), ::core::mem::transmute(brecursive), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IAzRoleAssignments>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IAzTask2> for ::windows_core::IUnknown {
    fn from(value: IAzTask2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IAzTask2> for ::windows_core::IUnknown {
    fn from(value: &IAzTask2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IAzTask2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IAzTask2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IAzTask2> for ::win32_system::Com::IDispatch {
    fn from(value: IAzTask2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IAzTask2> for ::win32_system::Com::IDispatch {
    fn from(value: &IAzTask2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for IAzTask2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for &'a IAzTask2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IAzTask2> for IAzTask {
    fn from(value: IAzTask2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IAzTask2> for IAzTask {
    fn from(value: &IAzTask2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, IAzTask> for IAzTask2 {
    fn into_param(self) -> ::windows_core::Param<'a, IAzTask> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, IAzTask> for &'a IAzTask2 {
    fn into_param(self) -> ::windows_core::Param<'a, IAzTask> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IAzTask2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IAzTask2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IAzTask2 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IAzTask2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAzTask2").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IAzTask2 {
    type Vtable = IAzTask2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x03a9a5ee_48c8_4832_9025_aad503c46526);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IAzTask2_Vtbl {
    pub base__: IAzTask_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub RoleAssignments: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrscopename: ::core::mem::ManuallyDrop<::win32_foundation::BSTR>, brecursive: i16, pproleassignments: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    RoleAssignments: usize,
}
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IAzTasks(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IAzTasks {
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn get_Item(&self, index: i32) -> ::windows_core::Result<::win32_system::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::win32_system::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).get_Item)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(index), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_system::Com::VARIANT>(result__)
    }
    pub unsafe fn Count(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
        (::windows_core::Interface::vtable(self).Count)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn _NewEnum(&self) -> ::windows_core::Result<::windows_core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
        (::windows_core::Interface::vtable(self)._NewEnum)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::IUnknown>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IAzTasks> for ::windows_core::IUnknown {
    fn from(value: IAzTasks) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IAzTasks> for ::windows_core::IUnknown {
    fn from(value: &IAzTasks) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IAzTasks {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IAzTasks {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IAzTasks> for ::win32_system::Com::IDispatch {
    fn from(value: IAzTasks) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IAzTasks> for ::win32_system::Com::IDispatch {
    fn from(value: &IAzTasks) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for IAzTasks {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows_core::IntoParam<'a, ::win32_system::Com::IDispatch> for &'a IAzTasks {
    fn into_param(self) -> ::windows_core::Param<'a, ::win32_system::Com::IDispatch> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IAzTasks {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IAzTasks {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IAzTasks {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IAzTasks {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAzTasks").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IAzTasks {
    type Vtable = IAzTasks_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb338ccab_4c85_4388_8c0a_c58592bad398);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IAzTasks_Vtbl {
    pub base__: ::win32_system::Com::IDispatch_Vtbl,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub get_Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: i32, pvarobtptr: *mut ::win32_system::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    get_Item: usize,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows_core::HRESULT,
    pub _NewEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenumptr: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
pub const INHERITED_ACCESS_ENTRY: u32 = 16u32;
#[repr(C)]
pub struct INHERITED_FROMA {
    pub GenerationGap: i32,
    pub AncestorName: ::windows_core::PSTR,
}
impl ::core::marker::Copy for INHERITED_FROMA {}
impl ::core::clone::Clone for INHERITED_FROMA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for INHERITED_FROMA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("INHERITED_FROMA").field("GenerationGap", &self.GenerationGap).field("AncestorName", &self.AncestorName).finish()
    }
}
unsafe impl ::windows_core::Abi for INHERITED_FROMA {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for INHERITED_FROMA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<INHERITED_FROMA>()) == 0 }
    }
}
impl ::core::cmp::Eq for INHERITED_FROMA {}
impl ::core::default::Default for INHERITED_FROMA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct INHERITED_FROMW {
    pub GenerationGap: i32,
    pub AncestorName: ::windows_core::PWSTR,
}
impl ::core::marker::Copy for INHERITED_FROMW {}
impl ::core::clone::Clone for INHERITED_FROMW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for INHERITED_FROMW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("INHERITED_FROMW").field("GenerationGap", &self.GenerationGap).field("AncestorName", &self.AncestorName).finish()
    }
}
unsafe impl ::windows_core::Abi for INHERITED_FROMW {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for INHERITED_FROMW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<INHERITED_FROMW>()) == 0 }
    }
}
impl ::core::cmp::Eq for INHERITED_FROMW {}
impl ::core::default::Default for INHERITED_FROMW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const INHERITED_GRANDPARENT: u32 = 536870912u32;
pub const INHERITED_PARENT: u32 = 268435456u32;
#[inline]
pub unsafe fn LookupSecurityDescriptorPartsA<'a, Param6: ::windows_core::IntoParam<'a, super::PSECURITY_DESCRIPTOR>>(ppowner: *mut *mut TRUSTEE_A, ppgroup: *mut *mut TRUSTEE_A, pccountofaccessentries: *mut u32, pplistofaccessentries: *mut *mut EXPLICIT_ACCESS_A, pccountofauditentries: *mut u32, pplistofauditentries: *mut *mut EXPLICIT_ACCESS_A, psd: Param6) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn LookupSecurityDescriptorPartsA(ppowner: *mut *mut TRUSTEE_A, ppgroup: *mut *mut TRUSTEE_A, pccountofaccessentries: *mut u32, pplistofaccessentries: *mut *mut EXPLICIT_ACCESS_A, pccountofauditentries: *mut u32, pplistofauditentries: *mut *mut EXPLICIT_ACCESS_A, psd: super::PSECURITY_DESCRIPTOR) -> u32;
        }
        ::core::mem::transmute(LookupSecurityDescriptorPartsA(::core::mem::transmute(ppowner), ::core::mem::transmute(ppgroup), ::core::mem::transmute(pccountofaccessentries), ::core::mem::transmute(pplistofaccessentries), ::core::mem::transmute(pccountofauditentries), ::core::mem::transmute(pplistofauditentries), psd.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn LookupSecurityDescriptorPartsW<'a, Param6: ::windows_core::IntoParam<'a, super::PSECURITY_DESCRIPTOR>>(ppowner: *mut *mut TRUSTEE_W, ppgroup: *mut *mut TRUSTEE_W, pccountofaccessentries: *mut u32, pplistofaccessentries: *mut *mut EXPLICIT_ACCESS_W, pccountofauditentries: *mut u32, pplistofauditentries: *mut *mut EXPLICIT_ACCESS_W, psd: Param6) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn LookupSecurityDescriptorPartsW(ppowner: *mut *mut TRUSTEE_W, ppgroup: *mut *mut TRUSTEE_W, pccountofaccessentries: *mut u32, pplistofaccessentries: *mut *mut EXPLICIT_ACCESS_W, pccountofauditentries: *mut u32, pplistofauditentries: *mut *mut EXPLICIT_ACCESS_W, psd: super::PSECURITY_DESCRIPTOR) -> u32;
        }
        ::core::mem::transmute(LookupSecurityDescriptorPartsW(::core::mem::transmute(ppowner), ::core::mem::transmute(ppgroup), ::core::mem::transmute(pccountofaccessentries), ::core::mem::transmute(pplistofaccessentries), ::core::mem::transmute(pccountofauditentries), ::core::mem::transmute(pplistofauditentries), psd.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct MULTIPLE_TRUSTEE_OPERATION(pub i32);
pub const NO_MULTIPLE_TRUSTEE: MULTIPLE_TRUSTEE_OPERATION = MULTIPLE_TRUSTEE_OPERATION(0i32);
pub const TRUSTEE_IS_IMPERSONATE: MULTIPLE_TRUSTEE_OPERATION = MULTIPLE_TRUSTEE_OPERATION(1i32);
impl ::core::marker::Copy for MULTIPLE_TRUSTEE_OPERATION {}
impl ::core::clone::Clone for MULTIPLE_TRUSTEE_OPERATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MULTIPLE_TRUSTEE_OPERATION {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for MULTIPLE_TRUSTEE_OPERATION {
    type Abi = Self;
}
impl ::core::fmt::Debug for MULTIPLE_TRUSTEE_OPERATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MULTIPLE_TRUSTEE_OPERATION").field(&self.0).finish()
    }
}
#[repr(C)]
pub struct OBJECTS_AND_NAME_A {
    pub ObjectsPresent: super::SYSTEM_AUDIT_OBJECT_ACE_FLAGS,
    pub ObjectType: SE_OBJECT_TYPE,
    pub ObjectTypeName: ::windows_core::PSTR,
    pub InheritedObjectTypeName: ::windows_core::PSTR,
    pub ptstrName: ::windows_core::PSTR,
}
impl ::core::marker::Copy for OBJECTS_AND_NAME_A {}
impl ::core::clone::Clone for OBJECTS_AND_NAME_A {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for OBJECTS_AND_NAME_A {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("OBJECTS_AND_NAME_A").field("ObjectsPresent", &self.ObjectsPresent).field("ObjectType", &self.ObjectType).field("ObjectTypeName", &self.ObjectTypeName).field("InheritedObjectTypeName", &self.InheritedObjectTypeName).field("ptstrName", &self.ptstrName).finish()
    }
}
unsafe impl ::windows_core::Abi for OBJECTS_AND_NAME_A {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for OBJECTS_AND_NAME_A {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<OBJECTS_AND_NAME_A>()) == 0 }
    }
}
impl ::core::cmp::Eq for OBJECTS_AND_NAME_A {}
impl ::core::default::Default for OBJECTS_AND_NAME_A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct OBJECTS_AND_NAME_W {
    pub ObjectsPresent: super::SYSTEM_AUDIT_OBJECT_ACE_FLAGS,
    pub ObjectType: SE_OBJECT_TYPE,
    pub ObjectTypeName: ::windows_core::PWSTR,
    pub InheritedObjectTypeName: ::windows_core::PWSTR,
    pub ptstrName: ::windows_core::PWSTR,
}
impl ::core::marker::Copy for OBJECTS_AND_NAME_W {}
impl ::core::clone::Clone for OBJECTS_AND_NAME_W {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for OBJECTS_AND_NAME_W {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("OBJECTS_AND_NAME_W").field("ObjectsPresent", &self.ObjectsPresent).field("ObjectType", &self.ObjectType).field("ObjectTypeName", &self.ObjectTypeName).field("InheritedObjectTypeName", &self.InheritedObjectTypeName).field("ptstrName", &self.ptstrName).finish()
    }
}
unsafe impl ::windows_core::Abi for OBJECTS_AND_NAME_W {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for OBJECTS_AND_NAME_W {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<OBJECTS_AND_NAME_W>()) == 0 }
    }
}
impl ::core::cmp::Eq for OBJECTS_AND_NAME_W {}
impl ::core::default::Default for OBJECTS_AND_NAME_W {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct OBJECTS_AND_SID {
    pub ObjectsPresent: super::SYSTEM_AUDIT_OBJECT_ACE_FLAGS,
    pub ObjectTypeGuid: ::windows_core::GUID,
    pub InheritedObjectTypeGuid: ::windows_core::GUID,
    pub pSid: *mut super::SID,
}
impl ::core::marker::Copy for OBJECTS_AND_SID {}
impl ::core::clone::Clone for OBJECTS_AND_SID {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for OBJECTS_AND_SID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("OBJECTS_AND_SID").field("ObjectsPresent", &self.ObjectsPresent).field("ObjectTypeGuid", &self.ObjectTypeGuid).field("InheritedObjectTypeGuid", &self.InheritedObjectTypeGuid).field("pSid", &self.pSid).finish()
    }
}
unsafe impl ::windows_core::Abi for OBJECTS_AND_SID {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for OBJECTS_AND_SID {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<OBJECTS_AND_SID>()) == 0 }
    }
}
impl ::core::cmp::Eq for OBJECTS_AND_SID {}
impl ::core::default::Default for OBJECTS_AND_SID {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const OLESCRIPT_E_SYNTAX: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147352319i32);
pub type PFN_AUTHZ_COMPUTE_DYNAMIC_GROUPS = ::core::option::Option<unsafe extern "system" fn(hauthzclientcontext: AUTHZ_CLIENT_CONTEXT_HANDLE, args: *const ::core::ffi::c_void, psidattrarray: *mut *mut super::SID_AND_ATTRIBUTES, psidcount: *mut u32, prestrictedsidattrarray: *mut *mut super::SID_AND_ATTRIBUTES, prestrictedsidcount: *mut u32) -> ::win32_foundation::BOOL>;
pub type PFN_AUTHZ_DYNAMIC_ACCESS_CHECK = ::core::option::Option<unsafe extern "system" fn(hauthzclientcontext: AUTHZ_CLIENT_CONTEXT_HANDLE, pace: *const super::ACE_HEADER, pargs: *const ::core::ffi::c_void, pbaceapplicable: *mut ::win32_foundation::BOOL) -> ::win32_foundation::BOOL>;
pub type PFN_AUTHZ_FREE_CENTRAL_ACCESS_POLICY = ::core::option::Option<unsafe extern "system" fn(pcentralaccesspolicy: *const ::core::ffi::c_void)>;
pub type PFN_AUTHZ_FREE_DYNAMIC_GROUPS = ::core::option::Option<unsafe extern "system" fn(psidattrarray: *const super::SID_AND_ATTRIBUTES)>;
pub type PFN_AUTHZ_GET_CENTRAL_ACCESS_POLICY = ::core::option::Option<unsafe extern "system" fn(hauthzclientcontext: AUTHZ_CLIENT_CONTEXT_HANDLE, capid: ::win32_foundation::PSID, pargs: *const ::core::ffi::c_void, pcentralaccesspolicyapplicable: *mut ::win32_foundation::BOOL, ppcentralaccesspolicy: *mut *mut ::core::ffi::c_void) -> ::win32_foundation::BOOL>;
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PROG_INVOKE_SETTING(pub i32);
pub const ProgressInvokeNever: PROG_INVOKE_SETTING = PROG_INVOKE_SETTING(1i32);
pub const ProgressInvokeEveryObject: PROG_INVOKE_SETTING = PROG_INVOKE_SETTING(2i32);
pub const ProgressInvokeOnError: PROG_INVOKE_SETTING = PROG_INVOKE_SETTING(3i32);
pub const ProgressCancelOperation: PROG_INVOKE_SETTING = PROG_INVOKE_SETTING(4i32);
pub const ProgressRetryOperation: PROG_INVOKE_SETTING = PROG_INVOKE_SETTING(5i32);
pub const ProgressInvokePrePostError: PROG_INVOKE_SETTING = PROG_INVOKE_SETTING(6i32);
impl ::core::marker::Copy for PROG_INVOKE_SETTING {}
impl ::core::clone::Clone for PROG_INVOKE_SETTING {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PROG_INVOKE_SETTING {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for PROG_INVOKE_SETTING {
    type Abi = Self;
}
impl ::core::fmt::Debug for PROG_INVOKE_SETTING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PROG_INVOKE_SETTING").field(&self.0).finish()
    }
}
pub const SDDL_ACCESS_ALLOWED: &str = "A";
pub const SDDL_ACCESS_CONTROL_ASSISTANCE_OPS: &str = "AA";
pub const SDDL_ACCESS_DENIED: &str = "D";
pub const SDDL_ACCESS_FILTER: &str = "FL";
pub const SDDL_ACCOUNT_OPERATORS: &str = "AO";
pub const SDDL_ACE_BEGIN: &str = "(";
pub const SDDL_ACE_COND_ATTRIBUTE_PREFIX: &str = "@";
pub const SDDL_ACE_COND_BEGIN: &str = "(";
pub const SDDL_ACE_COND_BLOB_PREFIX: &str = "#";
pub const SDDL_ACE_COND_DEVICE_ATTRIBUTE_PREFIX: &str = "@DEVICE.";
pub const SDDL_ACE_COND_END: &str = ")";
pub const SDDL_ACE_COND_RESOURCE_ATTRIBUTE_PREFIX: &str = "@RESOURCE.";
pub const SDDL_ACE_COND_SID_PREFIX: &str = "SID";
pub const SDDL_ACE_COND_TOKEN_ATTRIBUTE_PREFIX: &str = "@TOKEN.";
pub const SDDL_ACE_COND_USER_ATTRIBUTE_PREFIX: &str = "@USER.";
pub const SDDL_ACE_END: &str = ")";
pub const SDDL_ALARM: &str = "AL";
pub const SDDL_ALIAS_PREW2KCOMPACC: &str = "RU";
pub const SDDL_ALIAS_SIZE: u32 = 2u32;
pub const SDDL_ALL_APP_PACKAGES: &str = "AC";
pub const SDDL_ANONYMOUS: &str = "AN";
pub const SDDL_AUDIT: &str = "AU";
pub const SDDL_AUDIT_FAILURE: &str = "FA";
pub const SDDL_AUDIT_SUCCESS: &str = "SA";
pub const SDDL_AUTHENTICATED_USERS: &str = "AU";
pub const SDDL_AUTHORITY_ASSERTED: &str = "AS";
pub const SDDL_AUTO_INHERITED: &str = "AI";
pub const SDDL_AUTO_INHERIT_REQ: &str = "AR";
pub const SDDL_BACKUP_OPERATORS: &str = "BO";
pub const SDDL_BLOB: &str = "TX";
pub const SDDL_BOOLEAN: &str = "TB";
pub const SDDL_BUILTIN_ADMINISTRATORS: &str = "BA";
pub const SDDL_BUILTIN_GUESTS: &str = "BG";
pub const SDDL_BUILTIN_USERS: &str = "BU";
pub const SDDL_CALLBACK_ACCESS_ALLOWED: &str = "XA";
pub const SDDL_CALLBACK_ACCESS_DENIED: &str = "XD";
pub const SDDL_CALLBACK_AUDIT: &str = "XU";
pub const SDDL_CALLBACK_OBJECT_ACCESS_ALLOWED: &str = "ZA";
pub const SDDL_CERTSVC_DCOM_ACCESS: &str = "CD";
pub const SDDL_CERT_SERV_ADMINISTRATORS: &str = "CA";
pub const SDDL_CLONEABLE_CONTROLLERS: &str = "CN";
pub const SDDL_CONTAINER_INHERIT: &str = "CI";
pub const SDDL_CONTROL_ACCESS: &str = "CR";
pub const SDDL_CREATE_CHILD: &str = "CC";
pub const SDDL_CREATOR_GROUP: &str = "CG";
pub const SDDL_CREATOR_OWNER: &str = "CO";
pub const SDDL_CRITICAL: &str = "CR";
pub const SDDL_CRYPTO_OPERATORS: &str = "CY";
pub const SDDL_DACL: &str = "D";
pub const SDDL_DELETE_CHILD: &str = "DC";
pub const SDDL_DELETE_TREE: &str = "DT";
pub const SDDL_DELIMINATOR: &str = ":";
pub const SDDL_DOMAIN_ADMINISTRATORS: &str = "DA";
pub const SDDL_DOMAIN_COMPUTERS: &str = "DC";
pub const SDDL_DOMAIN_DOMAIN_CONTROLLERS: &str = "DD";
pub const SDDL_DOMAIN_GUESTS: &str = "DG";
pub const SDDL_DOMAIN_USERS: &str = "DU";
pub const SDDL_ENTERPRISE_ADMINS: &str = "EA";
pub const SDDL_ENTERPRISE_DOMAIN_CONTROLLERS: &str = "ED";
pub const SDDL_ENTERPRISE_KEY_ADMINS: &str = "EK";
pub const SDDL_ENTERPRISE_RO_DCs: &str = "RO";
pub const SDDL_EVENT_LOG_READERS: &str = "ER";
pub const SDDL_EVERYONE: &str = "WD";
pub const SDDL_FILE_ALL: &str = "FA";
pub const SDDL_FILE_EXECUTE: &str = "FX";
pub const SDDL_FILE_READ: &str = "FR";
pub const SDDL_FILE_WRITE: &str = "FW";
pub const SDDL_GENERIC_ALL: &str = "GA";
pub const SDDL_GENERIC_EXECUTE: &str = "GX";
pub const SDDL_GENERIC_READ: &str = "GR";
pub const SDDL_GENERIC_WRITE: &str = "GW";
pub const SDDL_GROUP: &str = "G";
pub const SDDL_GROUP_POLICY_ADMINS: &str = "PA";
pub const SDDL_HYPER_V_ADMINS: &str = "HA";
pub const SDDL_IIS_USERS: &str = "IS";
pub const SDDL_INHERITED: &str = "ID";
pub const SDDL_INHERIT_ONLY: &str = "IO";
pub const SDDL_INT: &str = "TI";
pub const SDDL_INTERACTIVE: &str = "IU";
pub const SDDL_KEY_ADMINS: &str = "KA";
pub const SDDL_KEY_ALL: &str = "KA";
pub const SDDL_KEY_EXECUTE: &str = "KX";
pub const SDDL_KEY_READ: &str = "KR";
pub const SDDL_KEY_WRITE: &str = "KW";
pub const SDDL_LIST_CHILDREN: &str = "LC";
pub const SDDL_LIST_OBJECT: &str = "LO";
pub const SDDL_LOCAL_ADMIN: &str = "LA";
pub const SDDL_LOCAL_GUEST: &str = "LG";
pub const SDDL_LOCAL_SERVICE: &str = "LS";
pub const SDDL_LOCAL_SYSTEM: &str = "SY";
pub const SDDL_MANDATORY_LABEL: &str = "ML";
pub const SDDL_ML_HIGH: &str = "HI";
pub const SDDL_ML_LOW: &str = "LW";
pub const SDDL_ML_MEDIUM: &str = "ME";
pub const SDDL_ML_MEDIUM_PLUS: &str = "MP";
pub const SDDL_ML_SYSTEM: &str = "SI";
pub const SDDL_NETWORK: &str = "NU";
pub const SDDL_NETWORK_CONFIGURATION_OPS: &str = "NO";
pub const SDDL_NETWORK_SERVICE: &str = "NS";
pub const SDDL_NO_EXECUTE_UP: &str = "NX";
pub const SDDL_NO_PROPAGATE: &str = "NP";
pub const SDDL_NO_READ_UP: &str = "NR";
pub const SDDL_NO_WRITE_UP: &str = "NW";
pub const SDDL_NULL_ACL: &str = "NO_ACCESS_CONTROL";
pub const SDDL_OBJECT_ACCESS_ALLOWED: &str = "OA";
pub const SDDL_OBJECT_ACCESS_DENIED: &str = "OD";
pub const SDDL_OBJECT_ALARM: &str = "OL";
pub const SDDL_OBJECT_AUDIT: &str = "OU";
pub const SDDL_OBJECT_INHERIT: &str = "OI";
pub const SDDL_OWNER: &str = "O";
pub const SDDL_OWNER_RIGHTS: &str = "OW";
pub const SDDL_PERFLOG_USERS: &str = "LU";
pub const SDDL_PERFMON_USERS: &str = "MU";
pub const SDDL_PERSONAL_SELF: &str = "PS";
pub const SDDL_POWER_USERS: &str = "PU";
pub const SDDL_PRINTER_OPERATORS: &str = "PO";
pub const SDDL_PROCESS_TRUST_LABEL: &str = "TL";
pub const SDDL_PROTECTED: &str = "P";
pub const SDDL_PROTECTED_USERS: &str = "AP";
pub const SDDL_RAS_SERVERS: &str = "RS";
pub const SDDL_RDS_ENDPOINT_SERVERS: &str = "ES";
pub const SDDL_RDS_MANAGEMENT_SERVERS: &str = "MS";
pub const SDDL_RDS_REMOTE_ACCESS_SERVERS: &str = "RA";
pub const SDDL_READ_CONTROL: &str = "RC";
pub const SDDL_READ_PROPERTY: &str = "RP";
pub const SDDL_REMOTE_DESKTOP: &str = "RD";
pub const SDDL_REMOTE_MANAGEMENT_USERS: &str = "RM";
pub const SDDL_REPLICATOR: &str = "RE";
pub const SDDL_RESOURCE_ATTRIBUTE: &str = "RA";
pub const SDDL_RESTRICTED_CODE: &str = "RC";
pub const SDDL_REVISION: u32 = 1u32;
pub const SDDL_REVISION_1: u32 = 1u32;
pub const SDDL_SACL: &str = "S";
pub const SDDL_SCHEMA_ADMINISTRATORS: &str = "SA";
pub const SDDL_SCOPED_POLICY_ID: &str = "SP";
pub const SDDL_SELF_WRITE: &str = "SW";
pub const SDDL_SEPERATOR: &str = ";";
pub const SDDL_SERVER_OPERATORS: &str = "SO";
pub const SDDL_SERVICE: &str = "SU";
pub const SDDL_SERVICE_ASSERTED: &str = "SS";
pub const SDDL_SID: &str = "TD";
pub const SDDL_SPACE: &str = " ";
pub const SDDL_STANDARD_DELETE: &str = "SD";
pub const SDDL_TRUST_PROTECTED_FILTER: &str = "TP";
pub const SDDL_UINT: &str = "TU";
pub const SDDL_USER_MODE_DRIVERS: &str = "UD";
pub const SDDL_WRITE_DAC: &str = "WD";
pub const SDDL_WRITE_OWNER: &str = "WO";
pub const SDDL_WRITE_PROPERTY: &str = "WP";
pub const SDDL_WRITE_RESTRICTED_CODE: &str = "WR";
pub const SDDL_WSTRING: &str = "TS";
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct SE_OBJECT_TYPE(pub i32);
pub const SE_UNKNOWN_OBJECT_TYPE: SE_OBJECT_TYPE = SE_OBJECT_TYPE(0i32);
pub const SE_FILE_OBJECT: SE_OBJECT_TYPE = SE_OBJECT_TYPE(1i32);
pub const SE_SERVICE: SE_OBJECT_TYPE = SE_OBJECT_TYPE(2i32);
pub const SE_PRINTER: SE_OBJECT_TYPE = SE_OBJECT_TYPE(3i32);
pub const SE_REGISTRY_KEY: SE_OBJECT_TYPE = SE_OBJECT_TYPE(4i32);
pub const SE_LMSHARE: SE_OBJECT_TYPE = SE_OBJECT_TYPE(5i32);
pub const SE_KERNEL_OBJECT: SE_OBJECT_TYPE = SE_OBJECT_TYPE(6i32);
pub const SE_WINDOW_OBJECT: SE_OBJECT_TYPE = SE_OBJECT_TYPE(7i32);
pub const SE_DS_OBJECT: SE_OBJECT_TYPE = SE_OBJECT_TYPE(8i32);
pub const SE_DS_OBJECT_ALL: SE_OBJECT_TYPE = SE_OBJECT_TYPE(9i32);
pub const SE_PROVIDER_DEFINED_OBJECT: SE_OBJECT_TYPE = SE_OBJECT_TYPE(10i32);
pub const SE_WMIGUID_OBJECT: SE_OBJECT_TYPE = SE_OBJECT_TYPE(11i32);
pub const SE_REGISTRY_WOW64_32KEY: SE_OBJECT_TYPE = SE_OBJECT_TYPE(12i32);
pub const SE_REGISTRY_WOW64_64KEY: SE_OBJECT_TYPE = SE_OBJECT_TYPE(13i32);
impl ::core::marker::Copy for SE_OBJECT_TYPE {}
impl ::core::clone::Clone for SE_OBJECT_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SE_OBJECT_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for SE_OBJECT_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for SE_OBJECT_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SE_OBJECT_TYPE").field(&self.0).finish()
    }
}
#[inline]
pub unsafe fn SetEntriesInAclA(plistofexplicitentries: &[EXPLICIT_ACCESS_A], oldacl: *const super::ACL, newacl: *mut *mut super::ACL) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetEntriesInAclA(ccountofexplicitentries: u32, plistofexplicitentries: *const EXPLICIT_ACCESS_A, oldacl: *const super::ACL, newacl: *mut *mut super::ACL) -> u32;
        }
        ::core::mem::transmute(SetEntriesInAclA(plistofexplicitentries.len() as _, ::core::mem::transmute(::windows_core::as_ptr_or_null(plistofexplicitentries)), ::core::mem::transmute(oldacl), ::core::mem::transmute(newacl)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn SetEntriesInAclW(plistofexplicitentries: &[EXPLICIT_ACCESS_W], oldacl: *const super::ACL, newacl: *mut *mut super::ACL) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetEntriesInAclW(ccountofexplicitentries: u32, plistofexplicitentries: *const EXPLICIT_ACCESS_W, oldacl: *const super::ACL, newacl: *mut *mut super::ACL) -> u32;
        }
        ::core::mem::transmute(SetEntriesInAclW(plistofexplicitentries.len() as _, ::core::mem::transmute(::windows_core::as_ptr_or_null(plistofexplicitentries)), ::core::mem::transmute(oldacl), ::core::mem::transmute(newacl)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn SetNamedSecurityInfoA<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCSTR>, Param3: ::windows_core::IntoParam<'a, ::win32_foundation::PSID>, Param4: ::windows_core::IntoParam<'a, ::win32_foundation::PSID>>(pobjectname: Param0, objecttype: SE_OBJECT_TYPE, securityinfo: super::OBJECT_SECURITY_INFORMATION, psidowner: Param3, psidgroup: Param4, pdacl: *const super::ACL, psacl: *const super::ACL) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetNamedSecurityInfoA(pobjectname: ::windows_core::PCSTR, objecttype: SE_OBJECT_TYPE, securityinfo: super::OBJECT_SECURITY_INFORMATION, psidowner: ::win32_foundation::PSID, psidgroup: ::win32_foundation::PSID, pdacl: *const super::ACL, psacl: *const super::ACL) -> u32;
        }
        ::core::mem::transmute(SetNamedSecurityInfoA(pobjectname.into_param().abi(), ::core::mem::transmute(objecttype), ::core::mem::transmute(securityinfo), psidowner.into_param().abi(), psidgroup.into_param().abi(), ::core::mem::transmute(pdacl), ::core::mem::transmute(psacl)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn SetNamedSecurityInfoW<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param3: ::windows_core::IntoParam<'a, ::win32_foundation::PSID>, Param4: ::windows_core::IntoParam<'a, ::win32_foundation::PSID>>(pobjectname: Param0, objecttype: SE_OBJECT_TYPE, securityinfo: super::OBJECT_SECURITY_INFORMATION, psidowner: Param3, psidgroup: Param4, pdacl: *const super::ACL, psacl: *const super::ACL) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetNamedSecurityInfoW(pobjectname: ::windows_core::PCWSTR, objecttype: SE_OBJECT_TYPE, securityinfo: super::OBJECT_SECURITY_INFORMATION, psidowner: ::win32_foundation::PSID, psidgroup: ::win32_foundation::PSID, pdacl: *const super::ACL, psacl: *const super::ACL) -> u32;
        }
        ::core::mem::transmute(SetNamedSecurityInfoW(pobjectname.into_param().abi(), ::core::mem::transmute(objecttype), ::core::mem::transmute(securityinfo), psidowner.into_param().abi(), psidgroup.into_param().abi(), ::core::mem::transmute(pdacl), ::core::mem::transmute(psacl)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn SetSecurityInfo<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HANDLE>, Param3: ::windows_core::IntoParam<'a, ::win32_foundation::PSID>, Param4: ::windows_core::IntoParam<'a, ::win32_foundation::PSID>>(handle: Param0, objecttype: SE_OBJECT_TYPE, securityinfo: u32, psidowner: Param3, psidgroup: Param4, pdacl: *const super::ACL, psacl: *const super::ACL) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetSecurityInfo(handle: ::win32_foundation::HANDLE, objecttype: SE_OBJECT_TYPE, securityinfo: u32, psidowner: ::win32_foundation::PSID, psidgroup: ::win32_foundation::PSID, pdacl: *const super::ACL, psacl: *const super::ACL) -> u32;
        }
        ::core::mem::transmute(SetSecurityInfo(handle.into_param().abi(), ::core::mem::transmute(objecttype), ::core::mem::transmute(securityinfo), psidowner.into_param().abi(), psidgroup.into_param().abi(), ::core::mem::transmute(pdacl), ::core::mem::transmute(psacl)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct TREE_SEC_INFO(pub u32);
pub const TREE_SEC_INFO_SET: TREE_SEC_INFO = TREE_SEC_INFO(1u32);
pub const TREE_SEC_INFO_RESET: TREE_SEC_INFO = TREE_SEC_INFO(2u32);
pub const TREE_SEC_INFO_RESET_KEEP_EXPLICIT: TREE_SEC_INFO = TREE_SEC_INFO(3u32);
impl ::core::marker::Copy for TREE_SEC_INFO {}
impl ::core::clone::Clone for TREE_SEC_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TREE_SEC_INFO {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for TREE_SEC_INFO {
    type Abi = Self;
}
impl ::core::fmt::Debug for TREE_SEC_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TREE_SEC_INFO").field(&self.0).finish()
    }
}
#[repr(C)]
pub struct TRUSTEE_A {
    pub pMultipleTrustee: *mut TRUSTEE_A,
    pub MultipleTrusteeOperation: MULTIPLE_TRUSTEE_OPERATION,
    pub TrusteeForm: TRUSTEE_FORM,
    pub TrusteeType: TRUSTEE_TYPE,
    pub ptstrName: ::windows_core::PSTR,
}
impl ::core::marker::Copy for TRUSTEE_A {}
impl ::core::clone::Clone for TRUSTEE_A {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for TRUSTEE_A {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TRUSTEE_A").field("pMultipleTrustee", &self.pMultipleTrustee).field("MultipleTrusteeOperation", &self.MultipleTrusteeOperation).field("TrusteeForm", &self.TrusteeForm).field("TrusteeType", &self.TrusteeType).field("ptstrName", &self.ptstrName).finish()
    }
}
unsafe impl ::windows_core::Abi for TRUSTEE_A {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for TRUSTEE_A {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<TRUSTEE_A>()) == 0 }
    }
}
impl ::core::cmp::Eq for TRUSTEE_A {}
impl ::core::default::Default for TRUSTEE_A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct TRUSTEE_ACCESSA {
    pub lpProperty: ::windows_core::PSTR,
    pub Access: u32,
    pub fAccessFlags: u32,
    pub fReturnedAccess: u32,
}
impl ::core::marker::Copy for TRUSTEE_ACCESSA {}
impl ::core::clone::Clone for TRUSTEE_ACCESSA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for TRUSTEE_ACCESSA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TRUSTEE_ACCESSA").field("lpProperty", &self.lpProperty).field("Access", &self.Access).field("fAccessFlags", &self.fAccessFlags).field("fReturnedAccess", &self.fReturnedAccess).finish()
    }
}
unsafe impl ::windows_core::Abi for TRUSTEE_ACCESSA {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for TRUSTEE_ACCESSA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<TRUSTEE_ACCESSA>()) == 0 }
    }
}
impl ::core::cmp::Eq for TRUSTEE_ACCESSA {}
impl ::core::default::Default for TRUSTEE_ACCESSA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct TRUSTEE_ACCESSW {
    pub lpProperty: ::windows_core::PWSTR,
    pub Access: u32,
    pub fAccessFlags: u32,
    pub fReturnedAccess: u32,
}
impl ::core::marker::Copy for TRUSTEE_ACCESSW {}
impl ::core::clone::Clone for TRUSTEE_ACCESSW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for TRUSTEE_ACCESSW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TRUSTEE_ACCESSW").field("lpProperty", &self.lpProperty).field("Access", &self.Access).field("fAccessFlags", &self.fAccessFlags).field("fReturnedAccess", &self.fReturnedAccess).finish()
    }
}
unsafe impl ::windows_core::Abi for TRUSTEE_ACCESSW {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for TRUSTEE_ACCESSW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<TRUSTEE_ACCESSW>()) == 0 }
    }
}
impl ::core::cmp::Eq for TRUSTEE_ACCESSW {}
impl ::core::default::Default for TRUSTEE_ACCESSW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const TRUSTEE_ACCESS_ALL: i32 = -1i32;
pub const TRUSTEE_ACCESS_ALLOWED: i32 = 1i32;
pub const TRUSTEE_ACCESS_EXPLICIT: i32 = 1i32;
pub const TRUSTEE_ACCESS_READ: i32 = 2i32;
pub const TRUSTEE_ACCESS_WRITE: i32 = 4i32;
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct TRUSTEE_FORM(pub i32);
pub const TRUSTEE_IS_SID: TRUSTEE_FORM = TRUSTEE_FORM(0i32);
pub const TRUSTEE_IS_NAME: TRUSTEE_FORM = TRUSTEE_FORM(1i32);
pub const TRUSTEE_BAD_FORM: TRUSTEE_FORM = TRUSTEE_FORM(2i32);
pub const TRUSTEE_IS_OBJECTS_AND_SID: TRUSTEE_FORM = TRUSTEE_FORM(3i32);
pub const TRUSTEE_IS_OBJECTS_AND_NAME: TRUSTEE_FORM = TRUSTEE_FORM(4i32);
impl ::core::marker::Copy for TRUSTEE_FORM {}
impl ::core::clone::Clone for TRUSTEE_FORM {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TRUSTEE_FORM {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for TRUSTEE_FORM {
    type Abi = Self;
}
impl ::core::fmt::Debug for TRUSTEE_FORM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TRUSTEE_FORM").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct TRUSTEE_TYPE(pub i32);
pub const TRUSTEE_IS_UNKNOWN: TRUSTEE_TYPE = TRUSTEE_TYPE(0i32);
pub const TRUSTEE_IS_USER: TRUSTEE_TYPE = TRUSTEE_TYPE(1i32);
pub const TRUSTEE_IS_GROUP: TRUSTEE_TYPE = TRUSTEE_TYPE(2i32);
pub const TRUSTEE_IS_DOMAIN: TRUSTEE_TYPE = TRUSTEE_TYPE(3i32);
pub const TRUSTEE_IS_ALIAS: TRUSTEE_TYPE = TRUSTEE_TYPE(4i32);
pub const TRUSTEE_IS_WELL_KNOWN_GROUP: TRUSTEE_TYPE = TRUSTEE_TYPE(5i32);
pub const TRUSTEE_IS_DELETED: TRUSTEE_TYPE = TRUSTEE_TYPE(6i32);
pub const TRUSTEE_IS_INVALID: TRUSTEE_TYPE = TRUSTEE_TYPE(7i32);
pub const TRUSTEE_IS_COMPUTER: TRUSTEE_TYPE = TRUSTEE_TYPE(8i32);
impl ::core::marker::Copy for TRUSTEE_TYPE {}
impl ::core::clone::Clone for TRUSTEE_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TRUSTEE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for TRUSTEE_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for TRUSTEE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TRUSTEE_TYPE").field(&self.0).finish()
    }
}
#[repr(C)]
pub struct TRUSTEE_W {
    pub pMultipleTrustee: *mut TRUSTEE_W,
    pub MultipleTrusteeOperation: MULTIPLE_TRUSTEE_OPERATION,
    pub TrusteeForm: TRUSTEE_FORM,
    pub TrusteeType: TRUSTEE_TYPE,
    pub ptstrName: ::windows_core::PWSTR,
}
impl ::core::marker::Copy for TRUSTEE_W {}
impl ::core::clone::Clone for TRUSTEE_W {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for TRUSTEE_W {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TRUSTEE_W").field("pMultipleTrustee", &self.pMultipleTrustee).field("MultipleTrusteeOperation", &self.MultipleTrusteeOperation).field("TrusteeForm", &self.TrusteeForm).field("TrusteeType", &self.TrusteeType).field("ptstrName", &self.ptstrName).finish()
    }
}
unsafe impl ::windows_core::Abi for TRUSTEE_W {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for TRUSTEE_W {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<TRUSTEE_W>()) == 0 }
    }
}
impl ::core::cmp::Eq for TRUSTEE_W {}
impl ::core::default::Default for TRUSTEE_W {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[inline]
pub unsafe fn TreeResetNamedSecurityInfoA<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCSTR>, Param3: ::windows_core::IntoParam<'a, ::win32_foundation::PSID>, Param4: ::windows_core::IntoParam<'a, ::win32_foundation::PSID>, Param7: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(pobjectname: Param0, objecttype: SE_OBJECT_TYPE, securityinfo: u32, powner: Param3, pgroup: Param4, pdacl: *const super::ACL, psacl: *const super::ACL, keepexplicit: Param7, fnprogress: FN_PROGRESS, progressinvokesetting: PROG_INVOKE_SETTING, args: *const ::core::ffi::c_void) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn TreeResetNamedSecurityInfoA(pobjectname: ::windows_core::PCSTR, objecttype: SE_OBJECT_TYPE, securityinfo: u32, powner: ::win32_foundation::PSID, pgroup: ::win32_foundation::PSID, pdacl: *const super::ACL, psacl: *const super::ACL, keepexplicit: ::win32_foundation::BOOL, fnprogress: ::windows_core::RawPtr, progressinvokesetting: PROG_INVOKE_SETTING, args: *const ::core::ffi::c_void) -> u32;
        }
        ::core::mem::transmute(TreeResetNamedSecurityInfoA(pobjectname.into_param().abi(), ::core::mem::transmute(objecttype), ::core::mem::transmute(securityinfo), powner.into_param().abi(), pgroup.into_param().abi(), ::core::mem::transmute(pdacl), ::core::mem::transmute(psacl), keepexplicit.into_param().abi(), ::core::mem::transmute(fnprogress), ::core::mem::transmute(progressinvokesetting), ::core::mem::transmute(args)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn TreeResetNamedSecurityInfoW<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param3: ::windows_core::IntoParam<'a, ::win32_foundation::PSID>, Param4: ::windows_core::IntoParam<'a, ::win32_foundation::PSID>, Param7: ::windows_core::IntoParam<'a, ::win32_foundation::BOOL>>(pobjectname: Param0, objecttype: SE_OBJECT_TYPE, securityinfo: u32, powner: Param3, pgroup: Param4, pdacl: *const super::ACL, psacl: *const super::ACL, keepexplicit: Param7, fnprogress: FN_PROGRESS, progressinvokesetting: PROG_INVOKE_SETTING, args: *const ::core::ffi::c_void) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn TreeResetNamedSecurityInfoW(pobjectname: ::windows_core::PCWSTR, objecttype: SE_OBJECT_TYPE, securityinfo: u32, powner: ::win32_foundation::PSID, pgroup: ::win32_foundation::PSID, pdacl: *const super::ACL, psacl: *const super::ACL, keepexplicit: ::win32_foundation::BOOL, fnprogress: ::windows_core::RawPtr, progressinvokesetting: PROG_INVOKE_SETTING, args: *const ::core::ffi::c_void) -> u32;
        }
        ::core::mem::transmute(TreeResetNamedSecurityInfoW(pobjectname.into_param().abi(), ::core::mem::transmute(objecttype), ::core::mem::transmute(securityinfo), powner.into_param().abi(), pgroup.into_param().abi(), ::core::mem::transmute(pdacl), ::core::mem::transmute(psacl), keepexplicit.into_param().abi(), ::core::mem::transmute(fnprogress), ::core::mem::transmute(progressinvokesetting), ::core::mem::transmute(args)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn TreeSetNamedSecurityInfoA<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCSTR>, Param3: ::windows_core::IntoParam<'a, ::win32_foundation::PSID>, Param4: ::windows_core::IntoParam<'a, ::win32_foundation::PSID>>(pobjectname: Param0, objecttype: SE_OBJECT_TYPE, securityinfo: u32, powner: Param3, pgroup: Param4, pdacl: *const super::ACL, psacl: *const super::ACL, dwaction: TREE_SEC_INFO, fnprogress: FN_PROGRESS, progressinvokesetting: PROG_INVOKE_SETTING, args: *const ::core::ffi::c_void) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn TreeSetNamedSecurityInfoA(pobjectname: ::windows_core::PCSTR, objecttype: SE_OBJECT_TYPE, securityinfo: u32, powner: ::win32_foundation::PSID, pgroup: ::win32_foundation::PSID, pdacl: *const super::ACL, psacl: *const super::ACL, dwaction: TREE_SEC_INFO, fnprogress: ::windows_core::RawPtr, progressinvokesetting: PROG_INVOKE_SETTING, args: *const ::core::ffi::c_void) -> u32;
        }
        ::core::mem::transmute(TreeSetNamedSecurityInfoA(pobjectname.into_param().abi(), ::core::mem::transmute(objecttype), ::core::mem::transmute(securityinfo), powner.into_param().abi(), pgroup.into_param().abi(), ::core::mem::transmute(pdacl), ::core::mem::transmute(psacl), ::core::mem::transmute(dwaction), ::core::mem::transmute(fnprogress), ::core::mem::transmute(progressinvokesetting), ::core::mem::transmute(args)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn TreeSetNamedSecurityInfoW<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param3: ::windows_core::IntoParam<'a, ::win32_foundation::PSID>, Param4: ::windows_core::IntoParam<'a, ::win32_foundation::PSID>>(pobjectname: Param0, objecttype: SE_OBJECT_TYPE, securityinfo: u32, powner: Param3, pgroup: Param4, pdacl: *const super::ACL, psacl: *const super::ACL, dwaction: TREE_SEC_INFO, fnprogress: FN_PROGRESS, progressinvokesetting: PROG_INVOKE_SETTING, args: *const ::core::ffi::c_void) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn TreeSetNamedSecurityInfoW(pobjectname: ::windows_core::PCWSTR, objecttype: SE_OBJECT_TYPE, securityinfo: u32, powner: ::win32_foundation::PSID, pgroup: ::win32_foundation::PSID, pdacl: *const super::ACL, psacl: *const super::ACL, dwaction: TREE_SEC_INFO, fnprogress: ::windows_core::RawPtr, progressinvokesetting: PROG_INVOKE_SETTING, args: *const ::core::ffi::c_void) -> u32;
        }
        ::core::mem::transmute(TreeSetNamedSecurityInfoW(pobjectname.into_param().abi(), ::core::mem::transmute(objecttype), ::core::mem::transmute(securityinfo), powner.into_param().abi(), pgroup.into_param().abi(), ::core::mem::transmute(pdacl), ::core::mem::transmute(psacl), ::core::mem::transmute(dwaction), ::core::mem::transmute(fnprogress), ::core::mem::transmute(progressinvokesetting), ::core::mem::transmute(args)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub const _AUTHZ_SS_MAXSIZE: u32 = 128u32;
