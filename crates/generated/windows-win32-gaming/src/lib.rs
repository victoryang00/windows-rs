
#![allow(non_snake_case, non_upper_case_globals, non_camel_case_types)]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CheckGamingPrivilegeSilently<'a, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param2: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(privilegeid: u32, scope: Param1, policy: Param2) -> ::windows_core::Result<super::Foundation::BOOL> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CheckGamingPrivilegeSilently(privilegeid: u32, scope: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, policy: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, hasprivilege: *mut super::Foundation::BOOL) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<super::Foundation::BOOL>::zeroed();
        CheckGamingPrivilegeSilently(::core::mem::transmute(privilegeid), scope.into_param().abi(), policy.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Foundation::BOOL>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CheckGamingPrivilegeSilentlyForUser<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::IInspectable>, Param2: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param3: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(user: Param0, privilegeid: u32, scope: Param2, policy: Param3) -> ::windows_core::Result<super::Foundation::BOOL> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CheckGamingPrivilegeSilentlyForUser(user: *mut ::core::ffi::c_void, privilegeid: u32, scope: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, policy: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, hasprivilege: *mut super::Foundation::BOOL) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<super::Foundation::BOOL>::zeroed();
        CheckGamingPrivilegeSilentlyForUser(user.into_param().abi(), ::core::mem::transmute(privilegeid), scope.into_param().abi(), policy.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Foundation::BOOL>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CheckGamingPrivilegeWithUI<'a, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param2: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param3: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(privilegeid: u32, scope: Param1, policy: Param2, friendlymessage: Param3, completionroutine: GameUICompletionRoutine, context: *const ::core::ffi::c_void) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CheckGamingPrivilegeWithUI(privilegeid: u32, scope: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, policy: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, friendlymessage: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, completionroutine: ::windows_core::RawPtr, context: *const ::core::ffi::c_void) -> ::windows_core::HRESULT;
        }
        CheckGamingPrivilegeWithUI(::core::mem::transmute(privilegeid), scope.into_param().abi(), policy.into_param().abi(), friendlymessage.into_param().abi(), ::core::mem::transmute(completionroutine), ::core::mem::transmute(context)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CheckGamingPrivilegeWithUIForUser<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::IInspectable>, Param2: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param3: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param4: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(user: Param0, privilegeid: u32, scope: Param2, policy: Param3, friendlymessage: Param4, completionroutine: GameUICompletionRoutine, context: *const ::core::ffi::c_void) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CheckGamingPrivilegeWithUIForUser(user: *mut ::core::ffi::c_void, privilegeid: u32, scope: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, policy: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, friendlymessage: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, completionroutine: ::windows_core::RawPtr, context: *const ::core::ffi::c_void) -> ::windows_core::HRESULT;
        }
        CheckGamingPrivilegeWithUIForUser(user.into_param().abi(), ::core::mem::transmute(privilegeid), scope.into_param().abi(), policy.into_param().abi(), friendlymessage.into_param().abi(), ::core::mem::transmute(completionroutine), ::core::mem::transmute(context)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct GAMESTATS_OPEN_RESULT(pub i32);
pub const GAMESTATS_OPEN_CREATED: GAMESTATS_OPEN_RESULT = GAMESTATS_OPEN_RESULT(0i32);
pub const GAMESTATS_OPEN_OPENED: GAMESTATS_OPEN_RESULT = GAMESTATS_OPEN_RESULT(1i32);
impl ::core::marker::Copy for GAMESTATS_OPEN_RESULT {}
impl ::core::clone::Clone for GAMESTATS_OPEN_RESULT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for GAMESTATS_OPEN_RESULT {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for GAMESTATS_OPEN_RESULT {
    type Abi = Self;
}
impl ::core::fmt::Debug for GAMESTATS_OPEN_RESULT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GAMESTATS_OPEN_RESULT").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct GAMESTATS_OPEN_TYPE(pub i32);
pub const GAMESTATS_OPEN_OPENORCREATE: GAMESTATS_OPEN_TYPE = GAMESTATS_OPEN_TYPE(0i32);
pub const GAMESTATS_OPEN_OPENONLY: GAMESTATS_OPEN_TYPE = GAMESTATS_OPEN_TYPE(1i32);
impl ::core::marker::Copy for GAMESTATS_OPEN_TYPE {}
impl ::core::clone::Clone for GAMESTATS_OPEN_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for GAMESTATS_OPEN_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for GAMESTATS_OPEN_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for GAMESTATS_OPEN_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GAMESTATS_OPEN_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct GAME_INSTALL_SCOPE(pub i32);
pub const GIS_NOT_INSTALLED: GAME_INSTALL_SCOPE = GAME_INSTALL_SCOPE(1i32);
pub const GIS_CURRENT_USER: GAME_INSTALL_SCOPE = GAME_INSTALL_SCOPE(2i32);
pub const GIS_ALL_USERS: GAME_INSTALL_SCOPE = GAME_INSTALL_SCOPE(3i32);
impl ::core::marker::Copy for GAME_INSTALL_SCOPE {}
impl ::core::clone::Clone for GAME_INSTALL_SCOPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for GAME_INSTALL_SCOPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for GAME_INSTALL_SCOPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for GAME_INSTALL_SCOPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GAME_INSTALL_SCOPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct GAMING_DEVICE_DEVICE_ID(pub i32);
pub const GAMING_DEVICE_DEVICE_ID_NONE: GAMING_DEVICE_DEVICE_ID = GAMING_DEVICE_DEVICE_ID(0i32);
pub const GAMING_DEVICE_DEVICE_ID_XBOX_ONE: GAMING_DEVICE_DEVICE_ID = GAMING_DEVICE_DEVICE_ID(1988865574i32);
pub const GAMING_DEVICE_DEVICE_ID_XBOX_ONE_S: GAMING_DEVICE_DEVICE_ID = GAMING_DEVICE_DEVICE_ID(712204761i32);
pub const GAMING_DEVICE_DEVICE_ID_XBOX_ONE_X: GAMING_DEVICE_DEVICE_ID = GAMING_DEVICE_DEVICE_ID(1523980231i32);
pub const GAMING_DEVICE_DEVICE_ID_XBOX_ONE_X_DEVKIT: GAMING_DEVICE_DEVICE_ID = GAMING_DEVICE_DEVICE_ID(284675555i32);
impl ::core::marker::Copy for GAMING_DEVICE_DEVICE_ID {}
impl ::core::clone::Clone for GAMING_DEVICE_DEVICE_ID {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for GAMING_DEVICE_DEVICE_ID {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for GAMING_DEVICE_DEVICE_ID {
    type Abi = Self;
}
impl ::core::fmt::Debug for GAMING_DEVICE_DEVICE_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GAMING_DEVICE_DEVICE_ID").field(&self.0).finish()
    }
}
#[repr(C)]
pub struct GAMING_DEVICE_MODEL_INFORMATION {
    pub vendorId: GAMING_DEVICE_VENDOR_ID,
    pub deviceId: GAMING_DEVICE_DEVICE_ID,
}
impl ::core::marker::Copy for GAMING_DEVICE_MODEL_INFORMATION {}
impl ::core::clone::Clone for GAMING_DEVICE_MODEL_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for GAMING_DEVICE_MODEL_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GAMING_DEVICE_MODEL_INFORMATION").field("vendorId", &self.vendorId).field("deviceId", &self.deviceId).finish()
    }
}
unsafe impl ::windows_core::Abi for GAMING_DEVICE_MODEL_INFORMATION {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for GAMING_DEVICE_MODEL_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<GAMING_DEVICE_MODEL_INFORMATION>()) == 0 }
    }
}
impl ::core::cmp::Eq for GAMING_DEVICE_MODEL_INFORMATION {}
impl ::core::default::Default for GAMING_DEVICE_MODEL_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct GAMING_DEVICE_VENDOR_ID(pub i32);
pub const GAMING_DEVICE_VENDOR_ID_NONE: GAMING_DEVICE_VENDOR_ID = GAMING_DEVICE_VENDOR_ID(0i32);
pub const GAMING_DEVICE_VENDOR_ID_MICROSOFT: GAMING_DEVICE_VENDOR_ID = GAMING_DEVICE_VENDOR_ID(-1024700366i32);
impl ::core::marker::Copy for GAMING_DEVICE_VENDOR_ID {}
impl ::core::clone::Clone for GAMING_DEVICE_VENDOR_ID {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for GAMING_DEVICE_VENDOR_ID {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for GAMING_DEVICE_VENDOR_ID {
    type Abi = Self;
}
impl ::core::fmt::Debug for GAMING_DEVICE_VENDOR_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GAMING_DEVICE_VENDOR_ID").field(&self.0).finish()
    }
}
pub const GameExplorer: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9a5ea990_3034_4d6f_9128_01f3c61022bc);
pub const GameStatistics: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xdbc85a2c_c0dc_4961_b6e2_d28b62c11ad4);
pub type GameUICompletionRoutine = ::core::option::Option<unsafe extern "system" fn(returncode: ::windows_core::HRESULT, context: *const ::core::ffi::c_void)>;
#[inline]
pub unsafe fn GetExpandedResourceExclusiveCpuCount() -> ::windows_core::Result<u32> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetExpandedResourceExclusiveCpuCount(exclusivecpucount: *mut u32) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        GetExpandedResourceExclusiveCpuCount(::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn GetGamingDeviceModelInformation() -> ::windows_core::Result<GAMING_DEVICE_MODEL_INFORMATION> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetGamingDeviceModelInformation(information: *mut GAMING_DEVICE_MODEL_INFORMATION) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<GAMING_DEVICE_MODEL_INFORMATION>::zeroed();
        GetGamingDeviceModelInformation(::core::mem::transmute(result__.as_mut_ptr())).from_abi::<GAMING_DEVICE_MODEL_INFORMATION>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HasExpandedResources() -> ::windows_core::Result<super::Foundation::BOOL> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HasExpandedResources(hasexpandedresources: *mut super::Foundation::BOOL) -> ::windows_core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<super::Foundation::BOOL>::zeroed();
        HasExpandedResources(::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Foundation::BOOL>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub const ID_GDF_THUMBNAIL_STR: &str = "__GDF_THUMBNAIL";
pub const ID_GDF_XML_STR: &str = "__GDF_XML";
#[repr(transparent)]
pub struct IGameExplorer(::windows_core::IUnknown);
impl IGameExplorer {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddGame<'a, Param0: ::windows_core::IntoParam<'a, super::Foundation::BSTR>, Param1: ::windows_core::IntoParam<'a, super::Foundation::BSTR>>(&self, bstrgdfbinarypath: Param0, bstrgameinstalldirectory: Param1, installscope: GAME_INSTALL_SCOPE, pguidinstanceid: *mut ::windows_core::GUID) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).AddGame)(::windows_core::Interface::as_raw(self), bstrgdfbinarypath.into_param().abi(), bstrgameinstalldirectory.into_param().abi(), ::core::mem::transmute(installscope), ::core::mem::transmute(pguidinstanceid)).ok()
    }
    pub unsafe fn RemoveGame<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::GUID>>(&self, guidinstanceid: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).RemoveGame)(::windows_core::Interface::as_raw(self), guidinstanceid.into_param().abi()).ok()
    }
    pub unsafe fn UpdateGame<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::GUID>>(&self, guidinstanceid: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).UpdateGame)(::windows_core::Interface::as_raw(self), guidinstanceid.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn VerifyAccess<'a, Param0: ::windows_core::IntoParam<'a, super::Foundation::BSTR>>(&self, bstrgdfbinarypath: Param0) -> ::windows_core::Result<super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::<super::Foundation::BOOL>::zeroed();
        (::windows_core::Interface::vtable(self).VerifyAccess)(::windows_core::Interface::as_raw(self), bstrgdfbinarypath.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Foundation::BOOL>(result__)
    }
}
impl ::core::convert::From<IGameExplorer> for ::windows_core::IUnknown {
    fn from(value: IGameExplorer) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IGameExplorer> for ::windows_core::IUnknown {
    fn from(value: &IGameExplorer) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IGameExplorer {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IGameExplorer {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IGameExplorer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IGameExplorer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IGameExplorer {}
impl ::core::fmt::Debug for IGameExplorer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IGameExplorer").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IGameExplorer {
    type Vtable = IGameExplorer_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe7b2fb72_d728_49b3_a5f2_18ebf5f1349e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGameExplorer_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub AddGame: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrgdfbinarypath: ::core::mem::ManuallyDrop<super::Foundation::BSTR>, bstrgameinstalldirectory: ::core::mem::ManuallyDrop<super::Foundation::BSTR>, installscope: GAME_INSTALL_SCOPE, pguidinstanceid: *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    AddGame: usize,
    pub RemoveGame: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guidinstanceid: ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub UpdateGame: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guidinstanceid: ::windows_core::GUID) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub VerifyAccess: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrgdfbinarypath: ::core::mem::ManuallyDrop<super::Foundation::BSTR>, pfhasaccess: *mut super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    VerifyAccess: usize,
}
#[repr(transparent)]
pub struct IGameExplorer2(::windows_core::IUnknown);
impl IGameExplorer2 {
    pub unsafe fn InstallGame<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, binarygdfpath: Param0, installdirectory: Param1, installscope: GAME_INSTALL_SCOPE) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).InstallGame)(::windows_core::Interface::as_raw(self), binarygdfpath.into_param().abi(), installdirectory.into_param().abi(), ::core::mem::transmute(installscope)).ok()
    }
    pub unsafe fn UninstallGame<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, binarygdfpath: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).UninstallGame)(::windows_core::Interface::as_raw(self), binarygdfpath.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CheckAccess<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, binarygdfpath: Param0) -> ::windows_core::Result<super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::<super::Foundation::BOOL>::zeroed();
        (::windows_core::Interface::vtable(self).CheckAccess)(::windows_core::Interface::as_raw(self), binarygdfpath.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Foundation::BOOL>(result__)
    }
}
impl ::core::convert::From<IGameExplorer2> for ::windows_core::IUnknown {
    fn from(value: IGameExplorer2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IGameExplorer2> for ::windows_core::IUnknown {
    fn from(value: &IGameExplorer2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IGameExplorer2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IGameExplorer2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IGameExplorer2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IGameExplorer2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IGameExplorer2 {}
impl ::core::fmt::Debug for IGameExplorer2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IGameExplorer2").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IGameExplorer2 {
    type Vtable = IGameExplorer2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x86874aa7_a1ed_450d_a7eb_b89e20b2fff3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGameExplorer2_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub InstallGame: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, binarygdfpath: ::windows_core::PCWSTR, installdirectory: ::windows_core::PCWSTR, installscope: GAME_INSTALL_SCOPE) -> ::windows_core::HRESULT,
    pub UninstallGame: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, binarygdfpath: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub CheckAccess: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, binarygdfpath: ::windows_core::PCWSTR, phasaccess: *mut super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CheckAccess: usize,
}
#[repr(transparent)]
pub struct IGameStatistics(::windows_core::IUnknown);
impl IGameStatistics {
    pub unsafe fn GetMaxCategoryLength(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).GetMaxCategoryLength)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn GetMaxNameLength(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).GetMaxNameLength)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn GetMaxValueLength(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).GetMaxValueLength)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn GetMaxCategories(&self) -> ::windows_core::Result<u16> {
        let mut result__ = ::core::mem::MaybeUninit::<u16>::zeroed();
        (::windows_core::Interface::vtable(self).GetMaxCategories)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u16>(result__)
    }
    pub unsafe fn GetMaxStatsPerCategory(&self) -> ::windows_core::Result<u16> {
        let mut result__ = ::core::mem::MaybeUninit::<u16>::zeroed();
        (::windows_core::Interface::vtable(self).GetMaxStatsPerCategory)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u16>(result__)
    }
    pub unsafe fn SetCategoryTitle<'a, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, categoryindex: u16, title: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetCategoryTitle)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(categoryindex), title.into_param().abi()).ok()
    }
    pub unsafe fn GetCategoryTitle(&self, categoryindex: u16) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::PWSTR>::zeroed();
        (::windows_core::Interface::vtable(self).GetCategoryTitle)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(categoryindex), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::PWSTR>(result__)
    }
    pub unsafe fn GetStatistic(&self, categoryindex: u16, statindex: u16, pname: *mut ::windows_core::PWSTR, pvalue: *mut ::windows_core::PWSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetStatistic)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(categoryindex), ::core::mem::transmute(statindex), ::core::mem::transmute(pname), ::core::mem::transmute(pvalue)).ok()
    }
    pub unsafe fn SetStatistic<'a, Param2: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param3: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, categoryindex: u16, statindex: u16, name: Param2, value: Param3) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetStatistic)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(categoryindex), ::core::mem::transmute(statindex), name.into_param().abi(), value.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Save<'a, Param0: ::windows_core::IntoParam<'a, super::Foundation::BOOL>>(&self, trackchanges: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Save)(::windows_core::Interface::as_raw(self), trackchanges.into_param().abi()).ok()
    }
    pub unsafe fn SetLastPlayedCategory(&self, categoryindex: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetLastPlayedCategory)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(categoryindex)).ok()
    }
    pub unsafe fn GetLastPlayedCategory(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).GetLastPlayedCategory)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
}
impl ::core::convert::From<IGameStatistics> for ::windows_core::IUnknown {
    fn from(value: IGameStatistics) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IGameStatistics> for ::windows_core::IUnknown {
    fn from(value: &IGameStatistics) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IGameStatistics {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IGameStatistics {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IGameStatistics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IGameStatistics {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IGameStatistics {}
impl ::core::fmt::Debug for IGameStatistics {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IGameStatistics").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IGameStatistics {
    type Vtable = IGameStatistics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3887c9ca_04a0_42ae_bc4c_5fa6c7721145);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGameStatistics_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub GetMaxCategoryLength: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cch: *mut u32) -> ::windows_core::HRESULT,
    pub GetMaxNameLength: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cch: *mut u32) -> ::windows_core::HRESULT,
    pub GetMaxValueLength: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cch: *mut u32) -> ::windows_core::HRESULT,
    pub GetMaxCategories: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmax: *mut u16) -> ::windows_core::HRESULT,
    pub GetMaxStatsPerCategory: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmax: *mut u16) -> ::windows_core::HRESULT,
    pub SetCategoryTitle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, categoryindex: u16, title: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    pub GetCategoryTitle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, categoryindex: u16, ptitle: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT,
    pub GetStatistic: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, categoryindex: u16, statindex: u16, pname: *mut ::windows_core::PWSTR, pvalue: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT,
    pub SetStatistic: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, categoryindex: u16, statindex: u16, name: ::windows_core::PCWSTR, value: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Save: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, trackchanges: super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Save: usize,
    pub SetLastPlayedCategory: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, categoryindex: u32) -> ::windows_core::HRESULT,
    pub GetLastPlayedCategory: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcategoryindex: *mut u32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IGameStatisticsMgr(::windows_core::IUnknown);
impl IGameStatisticsMgr {
    pub unsafe fn GetGameStatistics<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, gdfbinarypath: Param0, opentype: GAMESTATS_OPEN_TYPE, popenresult: *mut GAMESTATS_OPEN_RESULT, ppistats: *mut ::core::option::Option<IGameStatistics>) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetGameStatistics)(::windows_core::Interface::as_raw(self), gdfbinarypath.into_param().abi(), ::core::mem::transmute(opentype), ::core::mem::transmute(popenresult), ::core::mem::transmute(ppistats)).ok()
    }
    pub unsafe fn RemoveGameStatistics<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, gdfbinarypath: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).RemoveGameStatistics)(::windows_core::Interface::as_raw(self), gdfbinarypath.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IGameStatisticsMgr> for ::windows_core::IUnknown {
    fn from(value: IGameStatisticsMgr) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IGameStatisticsMgr> for ::windows_core::IUnknown {
    fn from(value: &IGameStatisticsMgr) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IGameStatisticsMgr {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IGameStatisticsMgr {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IGameStatisticsMgr {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IGameStatisticsMgr {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IGameStatisticsMgr {}
impl ::core::fmt::Debug for IGameStatisticsMgr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IGameStatisticsMgr").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IGameStatisticsMgr {
    type Vtable = IGameStatisticsMgr_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xaff3ea11_e70e_407d_95dd_35e612c41ce2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGameStatisticsMgr_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub GetGameStatistics: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, gdfbinarypath: ::windows_core::PCWSTR, opentype: GAMESTATS_OPEN_TYPE, popenresult: *mut GAMESTATS_OPEN_RESULT, ppistats: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub RemoveGameStatistics: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, gdfbinarypath: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IXblIdpAuthManager(::windows_core::IUnknown);
impl IXblIdpAuthManager {
    pub unsafe fn SetGamerAccount<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, msaaccountid: Param0, xuid: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetGamerAccount)(::windows_core::Interface::as_raw(self), msaaccountid.into_param().abi(), xuid.into_param().abi()).ok()
    }
    pub unsafe fn GetGamerAccount(&self, msaaccountid: *mut ::windows_core::PWSTR, xuid: *mut ::windows_core::PWSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetGamerAccount)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(msaaccountid), ::core::mem::transmute(xuid)).ok()
    }
    pub unsafe fn SetAppViewInitialized<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, appsid: Param0, msaaccountid: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetAppViewInitialized)(::windows_core::Interface::as_raw(self), appsid.into_param().abi(), msaaccountid.into_param().abi()).ok()
    }
    pub unsafe fn GetEnvironment(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::PWSTR>::zeroed();
        (::windows_core::Interface::vtable(self).GetEnvironment)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::PWSTR>(result__)
    }
    pub unsafe fn GetSandbox(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::PWSTR>::zeroed();
        (::windows_core::Interface::vtable(self).GetSandbox)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetTokenAndSignatureWithTokenResult<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param2: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param3: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param4: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param5: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param6: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param9: ::windows_core::IntoParam<'a, super::Foundation::BOOL>>(&self, msaaccountid: Param0, appsid: Param1, msatarget: Param2, msapolicy: Param3, httpmethod: Param4, uri: Param5, headers: Param6, body: &[u8], forcerefresh: Param9) -> ::windows_core::Result<IXblIdpAuthTokenResult> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetTokenAndSignatureWithTokenResult)(::windows_core::Interface::as_raw(self), msaaccountid.into_param().abi(), appsid.into_param().abi(), msatarget.into_param().abi(), msapolicy.into_param().abi(), httpmethod.into_param().abi(), uri.into_param().abi(), headers.into_param().abi(), ::core::mem::transmute(::windows_core::as_ptr_or_null(body)), body.len() as _, forcerefresh.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IXblIdpAuthTokenResult>(result__)
    }
}
impl ::core::convert::From<IXblIdpAuthManager> for ::windows_core::IUnknown {
    fn from(value: IXblIdpAuthManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IXblIdpAuthManager> for ::windows_core::IUnknown {
    fn from(value: &IXblIdpAuthManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IXblIdpAuthManager {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IXblIdpAuthManager {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IXblIdpAuthManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IXblIdpAuthManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IXblIdpAuthManager {}
impl ::core::fmt::Debug for IXblIdpAuthManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXblIdpAuthManager").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IXblIdpAuthManager {
    type Vtable = IXblIdpAuthManager_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xeb5ddb08_8bbf_449b_ac21_b02ddeb3b136);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXblIdpAuthManager_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub SetGamerAccount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, msaaccountid: ::windows_core::PCWSTR, xuid: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    pub GetGamerAccount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, msaaccountid: *mut ::windows_core::PWSTR, xuid: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT,
    pub SetAppViewInitialized: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, appsid: ::windows_core::PCWSTR, msaaccountid: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    pub GetEnvironment: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, environment: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT,
    pub GetSandbox: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sandbox: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetTokenAndSignatureWithTokenResult: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, msaaccountid: ::windows_core::PCWSTR, appsid: ::windows_core::PCWSTR, msatarget: ::windows_core::PCWSTR, msapolicy: ::windows_core::PCWSTR, httpmethod: ::windows_core::PCWSTR, uri: ::windows_core::PCWSTR, headers: ::windows_core::PCWSTR, body: *const u8, bodysize: u32, forcerefresh: super::Foundation::BOOL, result: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetTokenAndSignatureWithTokenResult: usize,
}
#[repr(transparent)]
pub struct IXblIdpAuthTokenResult(::windows_core::IUnknown);
impl IXblIdpAuthTokenResult {
    pub unsafe fn GetStatus(&self) -> ::windows_core::Result<XBL_IDP_AUTH_TOKEN_STATUS> {
        let mut result__ = ::core::mem::MaybeUninit::<XBL_IDP_AUTH_TOKEN_STATUS>::zeroed();
        (::windows_core::Interface::vtable(self).GetStatus)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<XBL_IDP_AUTH_TOKEN_STATUS>(result__)
    }
    pub unsafe fn GetErrorCode(&self) -> ::windows_core::Result<::windows_core::HRESULT> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::HRESULT>::zeroed();
        (::windows_core::Interface::vtable(self).GetErrorCode)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::HRESULT>(result__)
    }
    pub unsafe fn GetToken(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::PWSTR>::zeroed();
        (::windows_core::Interface::vtable(self).GetToken)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::PWSTR>(result__)
    }
    pub unsafe fn GetSignature(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::PWSTR>::zeroed();
        (::windows_core::Interface::vtable(self).GetSignature)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::PWSTR>(result__)
    }
    pub unsafe fn GetSandbox(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::PWSTR>::zeroed();
        (::windows_core::Interface::vtable(self).GetSandbox)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::PWSTR>(result__)
    }
    pub unsafe fn GetEnvironment(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::PWSTR>::zeroed();
        (::windows_core::Interface::vtable(self).GetEnvironment)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::PWSTR>(result__)
    }
    pub unsafe fn GetMsaAccountId(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::PWSTR>::zeroed();
        (::windows_core::Interface::vtable(self).GetMsaAccountId)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::PWSTR>(result__)
    }
    pub unsafe fn GetXuid(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::PWSTR>::zeroed();
        (::windows_core::Interface::vtable(self).GetXuid)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::PWSTR>(result__)
    }
    pub unsafe fn GetGamertag(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::PWSTR>::zeroed();
        (::windows_core::Interface::vtable(self).GetGamertag)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::PWSTR>(result__)
    }
    pub unsafe fn GetAgeGroup(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::PWSTR>::zeroed();
        (::windows_core::Interface::vtable(self).GetAgeGroup)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::PWSTR>(result__)
    }
    pub unsafe fn GetPrivileges(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::PWSTR>::zeroed();
        (::windows_core::Interface::vtable(self).GetPrivileges)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::PWSTR>(result__)
    }
    pub unsafe fn GetMsaTarget(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::PWSTR>::zeroed();
        (::windows_core::Interface::vtable(self).GetMsaTarget)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::PWSTR>(result__)
    }
    pub unsafe fn GetMsaPolicy(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::PWSTR>::zeroed();
        (::windows_core::Interface::vtable(self).GetMsaPolicy)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::PWSTR>(result__)
    }
    pub unsafe fn GetMsaAppId(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::PWSTR>::zeroed();
        (::windows_core::Interface::vtable(self).GetMsaAppId)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::PWSTR>(result__)
    }
    pub unsafe fn GetRedirect(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::PWSTR>::zeroed();
        (::windows_core::Interface::vtable(self).GetRedirect)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::PWSTR>(result__)
    }
    pub unsafe fn GetMessage(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::PWSTR>::zeroed();
        (::windows_core::Interface::vtable(self).GetMessage)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::PWSTR>(result__)
    }
    pub unsafe fn GetHelpId(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::PWSTR>::zeroed();
        (::windows_core::Interface::vtable(self).GetHelpId)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::PWSTR>(result__)
    }
    pub unsafe fn GetEnforcementBans(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::PWSTR>::zeroed();
        (::windows_core::Interface::vtable(self).GetEnforcementBans)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::PWSTR>(result__)
    }
    pub unsafe fn GetRestrictions(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::PWSTR>::zeroed();
        (::windows_core::Interface::vtable(self).GetRestrictions)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::PWSTR>(result__)
    }
    pub unsafe fn GetTitleRestrictions(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::PWSTR>::zeroed();
        (::windows_core::Interface::vtable(self).GetTitleRestrictions)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::PWSTR>(result__)
    }
}
impl ::core::convert::From<IXblIdpAuthTokenResult> for ::windows_core::IUnknown {
    fn from(value: IXblIdpAuthTokenResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IXblIdpAuthTokenResult> for ::windows_core::IUnknown {
    fn from(value: &IXblIdpAuthTokenResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IXblIdpAuthTokenResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IXblIdpAuthTokenResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IXblIdpAuthTokenResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IXblIdpAuthTokenResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IXblIdpAuthTokenResult {}
impl ::core::fmt::Debug for IXblIdpAuthTokenResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXblIdpAuthTokenResult").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IXblIdpAuthTokenResult {
    type Vtable = IXblIdpAuthTokenResult_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x46ce0225_f267_4d68_b299_b2762552dec1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXblIdpAuthTokenResult_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub GetStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, status: *mut XBL_IDP_AUTH_TOKEN_STATUS) -> ::windows_core::HRESULT,
    pub GetErrorCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, errorcode: *mut ::windows_core::HRESULT) -> ::windows_core::HRESULT,
    pub GetToken: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT,
    pub GetSignature: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, signature: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT,
    pub GetSandbox: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sandbox: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT,
    pub GetEnvironment: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, environment: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT,
    pub GetMsaAccountId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, msaaccountid: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT,
    pub GetXuid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, xuid: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT,
    pub GetGamertag: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, gamertag: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT,
    pub GetAgeGroup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, agegroup: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT,
    pub GetPrivileges: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, privileges: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT,
    pub GetMsaTarget: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, msatarget: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT,
    pub GetMsaPolicy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, msapolicy: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT,
    pub GetMsaAppId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, msaappid: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT,
    pub GetRedirect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, redirect: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT,
    pub GetMessage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, message: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT,
    pub GetHelpId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, helpid: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT,
    pub GetEnforcementBans: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, enforcementbans: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT,
    pub GetRestrictions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, restrictions: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT,
    pub GetTitleRestrictions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, titlerestrictions: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IXblIdpAuthTokenResult2(::windows_core::IUnknown);
impl IXblIdpAuthTokenResult2 {
    pub unsafe fn GetModernGamertag(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::PWSTR>::zeroed();
        (::windows_core::Interface::vtable(self).GetModernGamertag)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::PWSTR>(result__)
    }
    pub unsafe fn GetModernGamertagSuffix(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::PWSTR>::zeroed();
        (::windows_core::Interface::vtable(self).GetModernGamertagSuffix)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::PWSTR>(result__)
    }
    pub unsafe fn GetUniqueModernGamertag(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::PWSTR>::zeroed();
        (::windows_core::Interface::vtable(self).GetUniqueModernGamertag)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::PWSTR>(result__)
    }
}
impl ::core::convert::From<IXblIdpAuthTokenResult2> for ::windows_core::IUnknown {
    fn from(value: IXblIdpAuthTokenResult2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IXblIdpAuthTokenResult2> for ::windows_core::IUnknown {
    fn from(value: &IXblIdpAuthTokenResult2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IXblIdpAuthTokenResult2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IXblIdpAuthTokenResult2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IXblIdpAuthTokenResult2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IXblIdpAuthTokenResult2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IXblIdpAuthTokenResult2 {}
impl ::core::fmt::Debug for IXblIdpAuthTokenResult2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXblIdpAuthTokenResult2").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IXblIdpAuthTokenResult2 {
    type Vtable = IXblIdpAuthTokenResult2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x75d760b0_60b9_412d_994f_26b2cd5f7812);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXblIdpAuthTokenResult2_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub GetModernGamertag: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT,
    pub GetModernGamertagSuffix: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT,
    pub GetUniqueModernGamertag: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct KnownGamingPrivileges(pub i32);
pub const XPRIVILEGE_BROADCAST: KnownGamingPrivileges = KnownGamingPrivileges(190i32);
pub const XPRIVILEGE_VIEW_FRIENDS_LIST: KnownGamingPrivileges = KnownGamingPrivileges(197i32);
pub const XPRIVILEGE_GAME_DVR: KnownGamingPrivileges = KnownGamingPrivileges(198i32);
pub const XPRIVILEGE_SHARE_KINECT_CONTENT: KnownGamingPrivileges = KnownGamingPrivileges(199i32);
pub const XPRIVILEGE_MULTIPLAYER_PARTIES: KnownGamingPrivileges = KnownGamingPrivileges(203i32);
pub const XPRIVILEGE_COMMUNICATION_VOICE_INGAME: KnownGamingPrivileges = KnownGamingPrivileges(205i32);
pub const XPRIVILEGE_COMMUNICATION_VOICE_SKYPE: KnownGamingPrivileges = KnownGamingPrivileges(206i32);
pub const XPRIVILEGE_CLOUD_GAMING_MANAGE_SESSION: KnownGamingPrivileges = KnownGamingPrivileges(207i32);
pub const XPRIVILEGE_CLOUD_GAMING_JOIN_SESSION: KnownGamingPrivileges = KnownGamingPrivileges(208i32);
pub const XPRIVILEGE_CLOUD_SAVED_GAMES: KnownGamingPrivileges = KnownGamingPrivileges(209i32);
pub const XPRIVILEGE_SHARE_CONTENT: KnownGamingPrivileges = KnownGamingPrivileges(211i32);
pub const XPRIVILEGE_PREMIUM_CONTENT: KnownGamingPrivileges = KnownGamingPrivileges(214i32);
pub const XPRIVILEGE_SUBSCRIPTION_CONTENT: KnownGamingPrivileges = KnownGamingPrivileges(219i32);
pub const XPRIVILEGE_SOCIAL_NETWORK_SHARING: KnownGamingPrivileges = KnownGamingPrivileges(220i32);
pub const XPRIVILEGE_PREMIUM_VIDEO: KnownGamingPrivileges = KnownGamingPrivileges(224i32);
pub const XPRIVILEGE_VIDEO_COMMUNICATIONS: KnownGamingPrivileges = KnownGamingPrivileges(235i32);
pub const XPRIVILEGE_PURCHASE_CONTENT: KnownGamingPrivileges = KnownGamingPrivileges(245i32);
pub const XPRIVILEGE_USER_CREATED_CONTENT: KnownGamingPrivileges = KnownGamingPrivileges(247i32);
pub const XPRIVILEGE_PROFILE_VIEWING: KnownGamingPrivileges = KnownGamingPrivileges(249i32);
pub const XPRIVILEGE_COMMUNICATIONS: KnownGamingPrivileges = KnownGamingPrivileges(252i32);
pub const XPRIVILEGE_MULTIPLAYER_SESSIONS: KnownGamingPrivileges = KnownGamingPrivileges(254i32);
pub const XPRIVILEGE_ADD_FRIEND: KnownGamingPrivileges = KnownGamingPrivileges(255i32);
impl ::core::marker::Copy for KnownGamingPrivileges {}
impl ::core::clone::Clone for KnownGamingPrivileges {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for KnownGamingPrivileges {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for KnownGamingPrivileges {
    type Abi = Self;
}
impl ::core::fmt::Debug for KnownGamingPrivileges {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KnownGamingPrivileges").field(&self.0).finish()
    }
}
pub type PlayerPickerUICompletionRoutine = ::core::option::Option<unsafe extern "system" fn(returncode: ::windows_core::HRESULT, context: *const ::core::ffi::c_void, selectedxuids: *const ::windows_core::HSTRING, selectedxuidscount: usize)>;
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ProcessPendingGameUI<'a, Param0: ::windows_core::IntoParam<'a, super::Foundation::BOOL>>(waitforcompletion: Param0) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ProcessPendingGameUI(waitforcompletion: super::Foundation::BOOL) -> ::windows_core::HRESULT;
        }
        ProcessPendingGameUI(waitforcompletion.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn ReleaseExclusiveCpuSets() -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ReleaseExclusiveCpuSets() -> ::windows_core::HRESULT;
        }
        ReleaseExclusiveCpuSets().ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn ShowChangeFriendRelationshipUI<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(targetuserxuid: Param0, completionroutine: GameUICompletionRoutine, context: *const ::core::ffi::c_void) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ShowChangeFriendRelationshipUI(targetuserxuid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, completionroutine: ::windows_core::RawPtr, context: *const ::core::ffi::c_void) -> ::windows_core::HRESULT;
        }
        ShowChangeFriendRelationshipUI(targetuserxuid.into_param().abi(), ::core::mem::transmute(completionroutine), ::core::mem::transmute(context)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn ShowChangeFriendRelationshipUIForUser<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::IInspectable>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(user: Param0, targetuserxuid: Param1, completionroutine: GameUICompletionRoutine, context: *const ::core::ffi::c_void) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ShowChangeFriendRelationshipUIForUser(user: *mut ::core::ffi::c_void, targetuserxuid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, completionroutine: ::windows_core::RawPtr, context: *const ::core::ffi::c_void) -> ::windows_core::HRESULT;
        }
        ShowChangeFriendRelationshipUIForUser(user.into_param().abi(), targetuserxuid.into_param().abi(), ::core::mem::transmute(completionroutine), ::core::mem::transmute(context)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn ShowCustomizeUserProfileUI(completionroutine: GameUICompletionRoutine, context: *const ::core::ffi::c_void) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ShowCustomizeUserProfileUI(completionroutine: ::windows_core::RawPtr, context: *const ::core::ffi::c_void) -> ::windows_core::HRESULT;
        }
        ShowCustomizeUserProfileUI(::core::mem::transmute(completionroutine), ::core::mem::transmute(context)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn ShowCustomizeUserProfileUIForUser<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::IInspectable>>(user: Param0, completionroutine: GameUICompletionRoutine, context: *const ::core::ffi::c_void) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ShowCustomizeUserProfileUIForUser(user: *mut ::core::ffi::c_void, completionroutine: ::windows_core::RawPtr, context: *const ::core::ffi::c_void) -> ::windows_core::HRESULT;
        }
        ShowCustomizeUserProfileUIForUser(user.into_param().abi(), ::core::mem::transmute(completionroutine), ::core::mem::transmute(context)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn ShowFindFriendsUI(completionroutine: GameUICompletionRoutine, context: *const ::core::ffi::c_void) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ShowFindFriendsUI(completionroutine: ::windows_core::RawPtr, context: *const ::core::ffi::c_void) -> ::windows_core::HRESULT;
        }
        ShowFindFriendsUI(::core::mem::transmute(completionroutine), ::core::mem::transmute(context)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn ShowFindFriendsUIForUser<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::IInspectable>>(user: Param0, completionroutine: GameUICompletionRoutine, context: *const ::core::ffi::c_void) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ShowFindFriendsUIForUser(user: *mut ::core::ffi::c_void, completionroutine: ::windows_core::RawPtr, context: *const ::core::ffi::c_void) -> ::windows_core::HRESULT;
        }
        ShowFindFriendsUIForUser(user.into_param().abi(), ::core::mem::transmute(completionroutine), ::core::mem::transmute(context)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn ShowGameInfoUI(titleid: u32, completionroutine: GameUICompletionRoutine, context: *const ::core::ffi::c_void) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ShowGameInfoUI(titleid: u32, completionroutine: ::windows_core::RawPtr, context: *const ::core::ffi::c_void) -> ::windows_core::HRESULT;
        }
        ShowGameInfoUI(::core::mem::transmute(titleid), ::core::mem::transmute(completionroutine), ::core::mem::transmute(context)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn ShowGameInfoUIForUser<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::IInspectable>>(user: Param0, titleid: u32, completionroutine: GameUICompletionRoutine, context: *const ::core::ffi::c_void) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ShowGameInfoUIForUser(user: *mut ::core::ffi::c_void, titleid: u32, completionroutine: ::windows_core::RawPtr, context: *const ::core::ffi::c_void) -> ::windows_core::HRESULT;
        }
        ShowGameInfoUIForUser(user.into_param().abi(), ::core::mem::transmute(titleid), ::core::mem::transmute(completionroutine), ::core::mem::transmute(context)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn ShowGameInviteUI<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param2: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param3: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(serviceconfigurationid: Param0, sessiontemplatename: Param1, sessionid: Param2, invitationdisplaytext: Param3, completionroutine: GameUICompletionRoutine, context: *const ::core::ffi::c_void) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ShowGameInviteUI(serviceconfigurationid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, sessiontemplatename: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, sessionid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, invitationdisplaytext: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, completionroutine: ::windows_core::RawPtr, context: *const ::core::ffi::c_void) -> ::windows_core::HRESULT;
        }
        ShowGameInviteUI(serviceconfigurationid.into_param().abi(), sessiontemplatename.into_param().abi(), sessionid.into_param().abi(), invitationdisplaytext.into_param().abi(), ::core::mem::transmute(completionroutine), ::core::mem::transmute(context)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn ShowGameInviteUIForUser<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::IInspectable>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param2: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param3: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param4: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(user: Param0, serviceconfigurationid: Param1, sessiontemplatename: Param2, sessionid: Param3, invitationdisplaytext: Param4, completionroutine: GameUICompletionRoutine, context: *const ::core::ffi::c_void) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ShowGameInviteUIForUser(user: *mut ::core::ffi::c_void, serviceconfigurationid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, sessiontemplatename: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, sessionid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, invitationdisplaytext: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, completionroutine: ::windows_core::RawPtr, context: *const ::core::ffi::c_void) -> ::windows_core::HRESULT;
        }
        ShowGameInviteUIForUser(user.into_param().abi(), serviceconfigurationid.into_param().abi(), sessiontemplatename.into_param().abi(), sessionid.into_param().abi(), invitationdisplaytext.into_param().abi(), ::core::mem::transmute(completionroutine), ::core::mem::transmute(context)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn ShowGameInviteUIWithContext<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param2: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param3: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param4: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(serviceconfigurationid: Param0, sessiontemplatename: Param1, sessionid: Param2, invitationdisplaytext: Param3, customactivationcontext: Param4, completionroutine: GameUICompletionRoutine, context: *const ::core::ffi::c_void) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ShowGameInviteUIWithContext(serviceconfigurationid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, sessiontemplatename: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, sessionid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, invitationdisplaytext: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, customactivationcontext: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, completionroutine: ::windows_core::RawPtr, context: *const ::core::ffi::c_void) -> ::windows_core::HRESULT;
        }
        ShowGameInviteUIWithContext(serviceconfigurationid.into_param().abi(), sessiontemplatename.into_param().abi(), sessionid.into_param().abi(), invitationdisplaytext.into_param().abi(), customactivationcontext.into_param().abi(), ::core::mem::transmute(completionroutine), ::core::mem::transmute(context)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn ShowGameInviteUIWithContextForUser<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::IInspectable>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param2: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param3: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param4: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param5: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(user: Param0, serviceconfigurationid: Param1, sessiontemplatename: Param2, sessionid: Param3, invitationdisplaytext: Param4, customactivationcontext: Param5, completionroutine: GameUICompletionRoutine, context: *const ::core::ffi::c_void) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ShowGameInviteUIWithContextForUser(user: *mut ::core::ffi::c_void, serviceconfigurationid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, sessiontemplatename: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, sessionid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, invitationdisplaytext: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, customactivationcontext: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, completionroutine: ::windows_core::RawPtr, context: *const ::core::ffi::c_void) -> ::windows_core::HRESULT;
        }
        ShowGameInviteUIWithContextForUser(user.into_param().abi(), serviceconfigurationid.into_param().abi(), sessiontemplatename.into_param().abi(), sessionid.into_param().abi(), invitationdisplaytext.into_param().abi(), customactivationcontext.into_param().abi(), ::core::mem::transmute(completionroutine), ::core::mem::transmute(context)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn ShowPlayerPickerUI<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(promptdisplaytext: Param0, xuids: &[::windows_core::HSTRING], preselectedxuids: &[::windows_core::HSTRING], minselectioncount: usize, maxselectioncount: usize, completionroutine: PlayerPickerUICompletionRoutine, context: *const ::core::ffi::c_void) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ShowPlayerPickerUI(promptdisplaytext: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, xuids: *const ::core::mem::ManuallyDrop<::windows_core::HSTRING>, xuidscount: usize, preselectedxuids: *const ::core::mem::ManuallyDrop<::windows_core::HSTRING>, preselectedxuidscount: usize, minselectioncount: usize, maxselectioncount: usize, completionroutine: ::windows_core::RawPtr, context: *const ::core::ffi::c_void) -> ::windows_core::HRESULT;
        }
        ShowPlayerPickerUI(promptdisplaytext.into_param().abi(), ::core::mem::transmute(::windows_core::as_ptr_or_null(xuids)), xuids.len() as _, ::core::mem::transmute(::windows_core::as_ptr_or_null(preselectedxuids)), preselectedxuids.len() as _, ::core::mem::transmute(minselectioncount), ::core::mem::transmute(maxselectioncount), ::core::mem::transmute(completionroutine), ::core::mem::transmute(context)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn ShowPlayerPickerUIForUser<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::IInspectable>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(user: Param0, promptdisplaytext: Param1, xuids: &[::windows_core::HSTRING], preselectedxuids: &[::windows_core::HSTRING], minselectioncount: usize, maxselectioncount: usize, completionroutine: PlayerPickerUICompletionRoutine, context: *const ::core::ffi::c_void) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ShowPlayerPickerUIForUser(user: *mut ::core::ffi::c_void, promptdisplaytext: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, xuids: *const ::core::mem::ManuallyDrop<::windows_core::HSTRING>, xuidscount: usize, preselectedxuids: *const ::core::mem::ManuallyDrop<::windows_core::HSTRING>, preselectedxuidscount: usize, minselectioncount: usize, maxselectioncount: usize, completionroutine: ::windows_core::RawPtr, context: *const ::core::ffi::c_void) -> ::windows_core::HRESULT;
        }
        ShowPlayerPickerUIForUser(user.into_param().abi(), promptdisplaytext.into_param().abi(), ::core::mem::transmute(::windows_core::as_ptr_or_null(xuids)), xuids.len() as _, ::core::mem::transmute(::windows_core::as_ptr_or_null(preselectedxuids)), preselectedxuids.len() as _, ::core::mem::transmute(minselectioncount), ::core::mem::transmute(maxselectioncount), ::core::mem::transmute(completionroutine), ::core::mem::transmute(context)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn ShowProfileCardUI<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(targetuserxuid: Param0, completionroutine: GameUICompletionRoutine, context: *const ::core::ffi::c_void) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ShowProfileCardUI(targetuserxuid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, completionroutine: ::windows_core::RawPtr, context: *const ::core::ffi::c_void) -> ::windows_core::HRESULT;
        }
        ShowProfileCardUI(targetuserxuid.into_param().abi(), ::core::mem::transmute(completionroutine), ::core::mem::transmute(context)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn ShowProfileCardUIForUser<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::IInspectable>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(user: Param0, targetuserxuid: Param1, completionroutine: GameUICompletionRoutine, context: *const ::core::ffi::c_void) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ShowProfileCardUIForUser(user: *mut ::core::ffi::c_void, targetuserxuid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, completionroutine: ::windows_core::RawPtr, context: *const ::core::ffi::c_void) -> ::windows_core::HRESULT;
        }
        ShowProfileCardUIForUser(user.into_param().abi(), targetuserxuid.into_param().abi(), ::core::mem::transmute(completionroutine), ::core::mem::transmute(context)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn ShowTitleAchievementsUI(titleid: u32, completionroutine: GameUICompletionRoutine, context: *const ::core::ffi::c_void) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ShowTitleAchievementsUI(titleid: u32, completionroutine: ::windows_core::RawPtr, context: *const ::core::ffi::c_void) -> ::windows_core::HRESULT;
        }
        ShowTitleAchievementsUI(::core::mem::transmute(titleid), ::core::mem::transmute(completionroutine), ::core::mem::transmute(context)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn ShowTitleAchievementsUIForUser<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::IInspectable>>(user: Param0, titleid: u32, completionroutine: GameUICompletionRoutine, context: *const ::core::ffi::c_void) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ShowTitleAchievementsUIForUser(user: *mut ::core::ffi::c_void, titleid: u32, completionroutine: ::windows_core::RawPtr, context: *const ::core::ffi::c_void) -> ::windows_core::HRESULT;
        }
        ShowTitleAchievementsUIForUser(user.into_param().abi(), ::core::mem::transmute(titleid), ::core::mem::transmute(completionroutine), ::core::mem::transmute(context)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn ShowUserSettingsUI(completionroutine: GameUICompletionRoutine, context: *const ::core::ffi::c_void) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ShowUserSettingsUI(completionroutine: ::windows_core::RawPtr, context: *const ::core::ffi::c_void) -> ::windows_core::HRESULT;
        }
        ShowUserSettingsUI(::core::mem::transmute(completionroutine), ::core::mem::transmute(context)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn ShowUserSettingsUIForUser<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::IInspectable>>(user: Param0, completionroutine: GameUICompletionRoutine, context: *const ::core::ffi::c_void) -> ::windows_core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ShowUserSettingsUIForUser(user: *mut ::core::ffi::c_void, completionroutine: ::windows_core::RawPtr, context: *const ::core::ffi::c_void) -> ::windows_core::HRESULT;
        }
        ShowUserSettingsUIForUser(user.into_param().abi(), ::core::mem::transmute(completionroutine), ::core::mem::transmute(context)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn TryCancelPendingGameUI() -> super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn TryCancelPendingGameUI() -> super::Foundation::BOOL;
        }
        ::core::mem::transmute(TryCancelPendingGameUI())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct XBL_IDP_AUTH_TOKEN_STATUS(pub i32);
pub const XBL_IDP_AUTH_TOKEN_STATUS_SUCCESS: XBL_IDP_AUTH_TOKEN_STATUS = XBL_IDP_AUTH_TOKEN_STATUS(0i32);
pub const XBL_IDP_AUTH_TOKEN_STATUS_OFFLINE_SUCCESS: XBL_IDP_AUTH_TOKEN_STATUS = XBL_IDP_AUTH_TOKEN_STATUS(1i32);
pub const XBL_IDP_AUTH_TOKEN_STATUS_NO_ACCOUNT_SET: XBL_IDP_AUTH_TOKEN_STATUS = XBL_IDP_AUTH_TOKEN_STATUS(2i32);
pub const XBL_IDP_AUTH_TOKEN_STATUS_LOAD_MSA_ACCOUNT_FAILED: XBL_IDP_AUTH_TOKEN_STATUS = XBL_IDP_AUTH_TOKEN_STATUS(3i32);
pub const XBL_IDP_AUTH_TOKEN_STATUS_XBOX_VETO: XBL_IDP_AUTH_TOKEN_STATUS = XBL_IDP_AUTH_TOKEN_STATUS(4i32);
pub const XBL_IDP_AUTH_TOKEN_STATUS_MSA_INTERRUPT: XBL_IDP_AUTH_TOKEN_STATUS = XBL_IDP_AUTH_TOKEN_STATUS(5i32);
pub const XBL_IDP_AUTH_TOKEN_STATUS_OFFLINE_NO_CONSENT: XBL_IDP_AUTH_TOKEN_STATUS = XBL_IDP_AUTH_TOKEN_STATUS(6i32);
pub const XBL_IDP_AUTH_TOKEN_STATUS_VIEW_NOT_SET: XBL_IDP_AUTH_TOKEN_STATUS = XBL_IDP_AUTH_TOKEN_STATUS(7i32);
pub const XBL_IDP_AUTH_TOKEN_STATUS_UNKNOWN: XBL_IDP_AUTH_TOKEN_STATUS = XBL_IDP_AUTH_TOKEN_STATUS(-1i32);
impl ::core::marker::Copy for XBL_IDP_AUTH_TOKEN_STATUS {}
impl ::core::clone::Clone for XBL_IDP_AUTH_TOKEN_STATUS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for XBL_IDP_AUTH_TOKEN_STATUS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for XBL_IDP_AUTH_TOKEN_STATUS {
    type Abi = Self;
}
impl ::core::fmt::Debug for XBL_IDP_AUTH_TOKEN_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("XBL_IDP_AUTH_TOKEN_STATUS").field(&self.0).finish()
    }
}
pub const XblIdpAuthManager: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xce23534b_56d8_4978_86a2_7ee570640468);
pub const XblIdpAuthTokenResult: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9f493441_744a_410c_ae2b_9a22f7c7731f);
