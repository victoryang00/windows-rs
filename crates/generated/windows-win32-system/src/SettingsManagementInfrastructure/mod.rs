#[repr(transparent)]
pub struct IItemEnumerator(::windows_core::IUnknown);
impl IItemEnumerator {
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Current(&self) -> ::windows_core::Result<super::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).Current)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::VARIANT>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MoveNext(&self) -> ::windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::<super::super::Foundation::BOOL>::zeroed();
        (::windows_core::Interface::vtable(self).MoveNext)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    pub unsafe fn Reset(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Reset)(::windows_core::Interface::as_raw(self)).ok()
    }
}
impl ::core::convert::From<IItemEnumerator> for ::windows_core::IUnknown {
    fn from(value: IItemEnumerator) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IItemEnumerator> for ::windows_core::IUnknown {
    fn from(value: &IItemEnumerator) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IItemEnumerator {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IItemEnumerator {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IItemEnumerator {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IItemEnumerator {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IItemEnumerator {}
impl ::core::fmt::Debug for IItemEnumerator {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IItemEnumerator").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IItemEnumerator {
    type Vtable = IItemEnumerator_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9f7d7bb7_20b3_11da_81a5_0030f1642e3c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IItemEnumerator_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Current: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, item: *mut super::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Current: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub MoveNext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itemvalid: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    MoveNext: usize,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct ISettingsContext(::windows_core::IUnknown);
impl ISettingsContext {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Serialize<'a, Param0: ::windows_core::IntoParam<'a, super::Com::IStream>, Param1: ::windows_core::IntoParam<'a, ITargetInfo>>(&self, pstream: Param0, ptarget: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Serialize)(::windows_core::Interface::as_raw(self), pstream.into_param().abi(), ptarget.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Deserialize<'a, Param0: ::windows_core::IntoParam<'a, super::Com::IStream>, Param1: ::windows_core::IntoParam<'a, ITargetInfo>>(&self, pstream: Param0, ptarget: Param1, pppresults: *mut *mut ::core::option::Option<ISettingsResult>, pcresultcount: *mut usize) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Deserialize)(::windows_core::Interface::as_raw(self), pstream.into_param().abi(), ptarget.into_param().abi(), ::core::mem::transmute(pppresults), ::core::mem::transmute(pcresultcount)).ok()
    }
    pub unsafe fn SetUserData(&self, puserdata: *const ::core::ffi::c_void) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetUserData)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(puserdata)).ok()
    }
    pub unsafe fn GetUserData(&self) -> ::windows_core::Result<*mut ::core::ffi::c_void> {
        let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
        (::windows_core::Interface::vtable(self).GetUserData)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<*mut ::core::ffi::c_void>(result__)
    }
    pub unsafe fn GetNamespaces(&self) -> ::windows_core::Result<IItemEnumerator> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetNamespaces)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IItemEnumerator>(result__)
    }
    pub unsafe fn GetStoredSettings<'a, Param0: ::windows_core::IntoParam<'a, ISettingsIdentity>>(&self, pidentity: Param0, ppaddedsettings: *mut ::core::option::Option<IItemEnumerator>, ppmodifiedsettings: *mut ::core::option::Option<IItemEnumerator>, ppdeletedsettings: *mut ::core::option::Option<IItemEnumerator>) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetStoredSettings)(::windows_core::Interface::as_raw(self), pidentity.into_param().abi(), ::core::mem::transmute(ppaddedsettings), ::core::mem::transmute(ppmodifiedsettings), ::core::mem::transmute(ppdeletedsettings)).ok()
    }
    pub unsafe fn RevertSetting<'a, Param0: ::windows_core::IntoParam<'a, ISettingsIdentity>, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, pidentity: Param0, pwzsetting: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).RevertSetting)(::windows_core::Interface::as_raw(self), pidentity.into_param().abi(), pwzsetting.into_param().abi()).ok()
    }
}
impl ::core::convert::From<ISettingsContext> for ::windows_core::IUnknown {
    fn from(value: ISettingsContext) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISettingsContext> for ::windows_core::IUnknown {
    fn from(value: &ISettingsContext) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ISettingsContext {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ISettingsContext {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ISettingsContext {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISettingsContext {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISettingsContext {}
impl ::core::fmt::Debug for ISettingsContext {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISettingsContext").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for ISettingsContext {
    type Vtable = ISettingsContext_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9f7d7bbd_20b3_11da_81a5_0030f1642e3c);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISettingsContext_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Serialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstream: ::windows_core::RawPtr, ptarget: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Serialize: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Deserialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstream: ::windows_core::RawPtr, ptarget: ::windows_core::RawPtr, pppresults: *mut *mut ::windows_core::RawPtr, pcresultcount: *mut usize) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Deserialize: usize,
    pub SetUserData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, puserdata: *const ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetUserData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, puserdata: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetNamespaces: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppnamespaceids: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetStoredSettings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pidentity: ::windows_core::RawPtr, ppaddedsettings: *mut ::windows_core::RawPtr, ppmodifiedsettings: *mut ::windows_core::RawPtr, ppdeletedsettings: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub RevertSetting: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pidentity: ::windows_core::RawPtr, pwzsetting: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct ISettingsEngine(::windows_core::IUnknown);
impl ISettingsEngine {
    pub unsafe fn GetNamespaces(&self, flags: WcmNamespaceEnumerationFlags, reserved: *const ::core::ffi::c_void) -> ::windows_core::Result<IItemEnumerator> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetNamespaces)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(flags), ::core::mem::transmute(reserved), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IItemEnumerator>(result__)
    }
    pub unsafe fn GetNamespace<'a, Param0: ::windows_core::IntoParam<'a, ISettingsIdentity>>(&self, settingsid: Param0, access: WcmNamespaceAccess, reserved: *const ::core::ffi::c_void) -> ::windows_core::Result<ISettingsNamespace> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetNamespace)(::windows_core::Interface::as_raw(self), settingsid.into_param().abi(), ::core::mem::transmute(access), ::core::mem::transmute(reserved), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISettingsNamespace>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetErrorDescription(&self, hresult: i32) -> ::windows_core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::super::Foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).GetErrorDescription)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(hresult), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn CreateSettingsIdentity(&self) -> ::windows_core::Result<ISettingsIdentity> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).CreateSettingsIdentity)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISettingsIdentity>(result__)
    }
    pub unsafe fn GetStoreStatus(&self, reserved: *const ::core::ffi::c_void) -> ::windows_core::Result<WcmUserStatus> {
        let mut result__ = ::core::mem::MaybeUninit::<WcmUserStatus>::zeroed();
        (::windows_core::Interface::vtable(self).GetStoreStatus)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(reserved), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<WcmUserStatus>(result__)
    }
    pub unsafe fn LoadStore(&self, flags: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).LoadStore)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(flags)).ok()
    }
    pub unsafe fn UnloadStore(&self, reserved: *const ::core::ffi::c_void) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).UnloadStore)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(reserved)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn RegisterNamespace<'a, Param0: ::windows_core::IntoParam<'a, ISettingsIdentity>, Param1: ::windows_core::IntoParam<'a, super::Com::IStream>, Param2: ::windows_core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, settingsid: Param0, stream: Param1, pushsettings: Param2) -> ::windows_core::Result<super::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).RegisterNamespace)(::windows_core::Interface::as_raw(self), settingsid.into_param().abi(), stream.into_param().abi(), pushsettings.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::VARIANT>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn UnregisterNamespace<'a, Param0: ::windows_core::IntoParam<'a, ISettingsIdentity>, Param1: ::windows_core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, settingsid: Param0, removesettings: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).UnregisterNamespace)(::windows_core::Interface::as_raw(self), settingsid.into_param().abi(), removesettings.into_param().abi()).ok()
    }
    pub unsafe fn CreateTargetInfo(&self) -> ::windows_core::Result<ITargetInfo> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).CreateTargetInfo)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ITargetInfo>(result__)
    }
    pub unsafe fn GetTargetInfo(&self) -> ::windows_core::Result<ITargetInfo> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetTargetInfo)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ITargetInfo>(result__)
    }
    pub unsafe fn SetTargetInfo<'a, Param0: ::windows_core::IntoParam<'a, ITargetInfo>>(&self, target: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetTargetInfo)(::windows_core::Interface::as_raw(self), target.into_param().abi()).ok()
    }
    pub unsafe fn CreateSettingsContext(&self, flags: u32, reserved: *const ::core::ffi::c_void) -> ::windows_core::Result<ISettingsContext> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).CreateSettingsContext)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(flags), ::core::mem::transmute(reserved), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISettingsContext>(result__)
    }
    pub unsafe fn SetSettingsContext<'a, Param0: ::windows_core::IntoParam<'a, ISettingsContext>>(&self, settingscontext: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetSettingsContext)(::windows_core::Interface::as_raw(self), settingscontext.into_param().abi()).ok()
    }
    pub unsafe fn ApplySettingsContext<'a, Param0: ::windows_core::IntoParam<'a, ISettingsContext>>(&self, settingscontext: Param0, pppwzidentities: *mut *mut ::windows_core::PWSTR, pcidentities: *mut usize) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ApplySettingsContext)(::windows_core::Interface::as_raw(self), settingscontext.into_param().abi(), ::core::mem::transmute(pppwzidentities), ::core::mem::transmute(pcidentities)).ok()
    }
    pub unsafe fn GetSettingsContext(&self) -> ::windows_core::Result<ISettingsContext> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetSettingsContext)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISettingsContext>(result__)
    }
}
impl ::core::convert::From<ISettingsEngine> for ::windows_core::IUnknown {
    fn from(value: ISettingsEngine) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISettingsEngine> for ::windows_core::IUnknown {
    fn from(value: &ISettingsEngine) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ISettingsEngine {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ISettingsEngine {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ISettingsEngine {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISettingsEngine {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISettingsEngine {}
impl ::core::fmt::Debug for ISettingsEngine {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISettingsEngine").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for ISettingsEngine {
    type Vtable = ISettingsEngine_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9f7d7bb9_20b3_11da_81a5_0030f1642e3c);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISettingsEngine_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub GetNamespaces: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, flags: WcmNamespaceEnumerationFlags, reserved: *const ::core::ffi::c_void, namespaces: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetNamespace: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, settingsid: ::windows_core::RawPtr, access: WcmNamespaceAccess, reserved: *const ::core::ffi::c_void, namespaceitem: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetErrorDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hresult: i32, message: *mut super::super::Foundation::BSTR) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetErrorDescription: usize,
    pub CreateSettingsIdentity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, settingsid: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetStoreStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reserved: *const ::core::ffi::c_void, status: *mut WcmUserStatus) -> ::windows_core::HRESULT,
    pub LoadStore: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, flags: u32) -> ::windows_core::HRESULT,
    pub UnloadStore: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reserved: *const ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub RegisterNamespace: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, settingsid: ::windows_core::RawPtr, stream: ::windows_core::RawPtr, pushsettings: super::super::Foundation::BOOL, results: *mut super::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    RegisterNamespace: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub UnregisterNamespace: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, settingsid: ::windows_core::RawPtr, removesettings: super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    UnregisterNamespace: usize,
    pub CreateTargetInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, target: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetTargetInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, target: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetTargetInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, target: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CreateSettingsContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, flags: u32, reserved: *const ::core::ffi::c_void, settingscontext: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetSettingsContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, settingscontext: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ApplySettingsContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, settingscontext: ::windows_core::RawPtr, pppwzidentities: *mut *mut ::windows_core::PWSTR, pcidentities: *mut usize) -> ::windows_core::HRESULT,
    pub GetSettingsContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, settingscontext: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct ISettingsIdentity(::windows_core::IUnknown);
impl ISettingsIdentity {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetAttribute<'a, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, reserved: *const ::core::ffi::c_void, name: Param1) -> ::windows_core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::super::Foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).GetAttribute)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(reserved), name.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn SetAttribute<'a, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param2: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, reserved: *const ::core::ffi::c_void, name: Param1, value: Param2) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetAttribute)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(reserved), name.into_param().abi(), value.into_param().abi()).ok()
    }
    pub unsafe fn GetFlags(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).GetFlags)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn SetFlags(&self, flags: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetFlags)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(flags)).ok()
    }
}
impl ::core::convert::From<ISettingsIdentity> for ::windows_core::IUnknown {
    fn from(value: ISettingsIdentity) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISettingsIdentity> for ::windows_core::IUnknown {
    fn from(value: &ISettingsIdentity) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ISettingsIdentity {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ISettingsIdentity {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ISettingsIdentity {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISettingsIdentity {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISettingsIdentity {}
impl ::core::fmt::Debug for ISettingsIdentity {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISettingsIdentity").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for ISettingsIdentity {
    type Vtable = ISettingsIdentity_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9f7d7bb6_20b3_11da_81a5_0030f1642e3c);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISettingsIdentity_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub GetAttribute: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reserved: *const ::core::ffi::c_void, name: ::windows_core::PCWSTR, value: *mut super::super::Foundation::BSTR) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetAttribute: usize,
    pub SetAttribute: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reserved: *const ::core::ffi::c_void, name: ::windows_core::PCWSTR, value: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    pub GetFlags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, flags: *mut u32) -> ::windows_core::HRESULT,
    pub SetFlags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, flags: u32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct ISettingsItem(::windows_core::IUnknown);
impl ISettingsItem {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetName(&self) -> ::windows_core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::super::Foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).GetName)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetValue(&self) -> ::windows_core::Result<super::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).GetValue)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetValue(&self, value: *const super::Com::VARIANT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetValue)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(value)).ok()
    }
    pub unsafe fn GetSettingType(&self) -> ::windows_core::Result<WcmSettingType> {
        let mut result__ = ::core::mem::MaybeUninit::<WcmSettingType>::zeroed();
        (::windows_core::Interface::vtable(self).GetSettingType)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<WcmSettingType>(result__)
    }
    pub unsafe fn GetDataType(&self) -> ::windows_core::Result<WcmDataType> {
        let mut result__ = ::core::mem::MaybeUninit::<WcmDataType>::zeroed();
        (::windows_core::Interface::vtable(self).GetDataType)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<WcmDataType>(result__)
    }
    pub unsafe fn GetValueRaw(&self, data: *mut *mut u8, datasize: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetValueRaw)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(data), ::core::mem::transmute(datasize)).ok()
    }
    pub unsafe fn SetValueRaw(&self, datatype: i32, data: &[u8]) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetValueRaw)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(datatype), ::core::mem::transmute(::windows_core::as_ptr_or_null(data)), data.len() as _).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn HasChild(&self) -> ::windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::<super::super::Foundation::BOOL>::zeroed();
        (::windows_core::Interface::vtable(self).HasChild)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    pub unsafe fn Children(&self) -> ::windows_core::Result<IItemEnumerator> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Children)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IItemEnumerator>(result__)
    }
    pub unsafe fn GetChild<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, name: Param0) -> ::windows_core::Result<ISettingsItem> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetChild)(::windows_core::Interface::as_raw(self), name.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISettingsItem>(result__)
    }
    pub unsafe fn GetSettingByPath<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, path: Param0) -> ::windows_core::Result<ISettingsItem> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetSettingByPath)(::windows_core::Interface::as_raw(self), path.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISettingsItem>(result__)
    }
    pub unsafe fn CreateSettingByPath<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, path: Param0) -> ::windows_core::Result<ISettingsItem> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).CreateSettingByPath)(::windows_core::Interface::as_raw(self), path.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISettingsItem>(result__)
    }
    pub unsafe fn RemoveSettingByPath<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, path: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).RemoveSettingByPath)(::windows_core::Interface::as_raw(self), path.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetListKeyInformation(&self, keyname: *mut super::super::Foundation::BSTR, datatype: *mut WcmDataType) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetListKeyInformation)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(keyname), ::core::mem::transmute(datatype)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn CreateListElement(&self, keydata: *const super::Com::VARIANT) -> ::windows_core::Result<ISettingsItem> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).CreateListElement)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(keydata), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISettingsItem>(result__)
    }
    pub unsafe fn RemoveListElement<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, elementname: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).RemoveListElement)(::windows_core::Interface::as_raw(self), elementname.into_param().abi()).ok()
    }
    pub unsafe fn Attributes(&self) -> ::windows_core::Result<IItemEnumerator> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Attributes)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IItemEnumerator>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetAttribute<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, name: Param0) -> ::windows_core::Result<super::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).GetAttribute)(::windows_core::Interface::as_raw(self), name.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::VARIANT>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetPath(&self) -> ::windows_core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::super::Foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).GetPath)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn GetRestrictionFacets(&self) -> ::windows_core::Result<WcmRestrictionFacets> {
        let mut result__ = ::core::mem::MaybeUninit::<WcmRestrictionFacets>::zeroed();
        (::windows_core::Interface::vtable(self).GetRestrictionFacets)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<WcmRestrictionFacets>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetRestriction(&self, restrictionfacet: WcmRestrictionFacets) -> ::windows_core::Result<super::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).GetRestriction)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(restrictionfacet), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetKeyValue(&self) -> ::windows_core::Result<super::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).GetKeyValue)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::VARIANT>(result__)
    }
}
impl ::core::convert::From<ISettingsItem> for ::windows_core::IUnknown {
    fn from(value: ISettingsItem) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISettingsItem> for ::windows_core::IUnknown {
    fn from(value: &ISettingsItem) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ISettingsItem {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ISettingsItem {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ISettingsItem {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISettingsItem {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISettingsItem {}
impl ::core::fmt::Debug for ISettingsItem {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISettingsItem").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for ISettingsItem {
    type Vtable = ISettingsItem_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9f7d7bbb_20b3_11da_81a5_0030f1642e3c);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISettingsItem_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub GetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: *mut super::super::Foundation::BSTR) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetName: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub GetValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut super::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    GetValue: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SetValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *const super::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SetValue: usize,
    pub GetSettingType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: *mut WcmSettingType) -> ::windows_core::HRESULT,
    pub GetDataType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: *mut WcmDataType) -> ::windows_core::HRESULT,
    pub GetValueRaw: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, data: *mut *mut u8, datasize: *mut u32) -> ::windows_core::HRESULT,
    pub SetValueRaw: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, datatype: i32, data: *const u8, datasize: u32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub HasChild: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itemhaschild: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    HasChild: usize,
    pub Children: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, children: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetChild: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::windows_core::PCWSTR, child: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetSettingByPath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, path: ::windows_core::PCWSTR, setting: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CreateSettingByPath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, path: ::windows_core::PCWSTR, setting: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub RemoveSettingByPath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, path: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetListKeyInformation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, keyname: *mut super::super::Foundation::BSTR, datatype: *mut WcmDataType) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetListKeyInformation: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub CreateListElement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, keydata: *const super::Com::VARIANT, child: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    CreateListElement: usize,
    pub RemoveListElement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, elementname: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    pub Attributes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, attributes: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub GetAttribute: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::windows_core::PCWSTR, value: *mut super::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    GetAttribute: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetPath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, path: *mut super::super::Foundation::BSTR) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetPath: usize,
    pub GetRestrictionFacets: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, restrictionfacets: *mut WcmRestrictionFacets) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub GetRestriction: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, restrictionfacet: WcmRestrictionFacets, facetdata: *mut super::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    GetRestriction: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub GetKeyValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut super::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    GetKeyValue: usize,
}
#[repr(transparent)]
pub struct ISettingsNamespace(::windows_core::IUnknown);
impl ISettingsNamespace {
    pub unsafe fn GetIdentity(&self) -> ::windows_core::Result<ISettingsIdentity> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetIdentity)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISettingsIdentity>(result__)
    }
    pub unsafe fn Settings(&self) -> ::windows_core::Result<IItemEnumerator> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Settings)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IItemEnumerator>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Save<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, pushsettings: Param0) -> ::windows_core::Result<ISettingsResult> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).Save)(::windows_core::Interface::as_raw(self), pushsettings.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISettingsResult>(result__)
    }
    pub unsafe fn GetSettingByPath<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, path: Param0) -> ::windows_core::Result<ISettingsItem> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetSettingByPath)(::windows_core::Interface::as_raw(self), path.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISettingsItem>(result__)
    }
    pub unsafe fn CreateSettingByPath<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, path: Param0) -> ::windows_core::Result<ISettingsItem> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).CreateSettingByPath)(::windows_core::Interface::as_raw(self), path.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISettingsItem>(result__)
    }
    pub unsafe fn RemoveSettingByPath<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, path: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).RemoveSettingByPath)(::windows_core::Interface::as_raw(self), path.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetAttribute<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, name: Param0) -> ::windows_core::Result<super::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::Com::VARIANT>>::zeroed();
        (::windows_core::Interface::vtable(self).GetAttribute)(::windows_core::Interface::as_raw(self), name.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::VARIANT>(result__)
    }
}
impl ::core::convert::From<ISettingsNamespace> for ::windows_core::IUnknown {
    fn from(value: ISettingsNamespace) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISettingsNamespace> for ::windows_core::IUnknown {
    fn from(value: &ISettingsNamespace) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ISettingsNamespace {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ISettingsNamespace {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ISettingsNamespace {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISettingsNamespace {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISettingsNamespace {}
impl ::core::fmt::Debug for ISettingsNamespace {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISettingsNamespace").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for ISettingsNamespace {
    type Vtable = ISettingsNamespace_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9f7d7bba_20b3_11da_81a5_0030f1642e3c);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISettingsNamespace_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub GetIdentity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, settingsid: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Settings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, settings: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Save: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pushsettings: super::super::Foundation::BOOL, result: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Save: usize,
    pub GetSettingByPath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, path: ::windows_core::PCWSTR, setting: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CreateSettingByPath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, path: ::windows_core::PCWSTR, setting: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub RemoveSettingByPath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, path: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub GetAttribute: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::windows_core::PCWSTR, value: *mut super::Com::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    GetAttribute: usize,
}
#[repr(transparent)]
pub struct ISettingsResult(::windows_core::IUnknown);
impl ISettingsResult {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDescription(&self) -> ::windows_core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::super::Foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).GetDescription)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn GetErrorCode(&self) -> ::windows_core::Result<::windows_core::HRESULT> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::HRESULT>::zeroed();
        (::windows_core::Interface::vtable(self).GetErrorCode)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows_core::HRESULT>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetContextDescription(&self) -> ::windows_core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::super::Foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).GetContextDescription)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn GetLine(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).GetLine)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn GetColumn(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).GetColumn)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetSource(&self) -> ::windows_core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::super::Foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).GetSource)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
}
impl ::core::convert::From<ISettingsResult> for ::windows_core::IUnknown {
    fn from(value: ISettingsResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISettingsResult> for ::windows_core::IUnknown {
    fn from(value: &ISettingsResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ISettingsResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ISettingsResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ISettingsResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISettingsResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISettingsResult {}
impl ::core::fmt::Debug for ISettingsResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISettingsResult").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for ISettingsResult {
    type Vtable = ISettingsResult_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9f7d7bbc_20b3_11da_81a5_0030f1642e3c);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISettingsResult_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub GetDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, description: *mut super::super::Foundation::BSTR) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetDescription: usize,
    pub GetErrorCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hrout: *mut ::windows_core::HRESULT) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetContextDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, description: *mut super::super::Foundation::BSTR) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetContextDescription: usize,
    pub GetLine: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwline: *mut u32) -> ::windows_core::HRESULT,
    pub GetColumn: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwcolumn: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetSource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, file: *mut super::super::Foundation::BSTR) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetSource: usize,
}
#[repr(transparent)]
pub struct ITargetInfo(::windows_core::IUnknown);
impl ITargetInfo {
    pub unsafe fn GetTargetMode(&self) -> ::windows_core::Result<WcmTargetMode> {
        let mut result__ = ::core::mem::MaybeUninit::<WcmTargetMode>::zeroed();
        (::windows_core::Interface::vtable(self).GetTargetMode)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<WcmTargetMode>(result__)
    }
    pub unsafe fn SetTargetMode(&self, targetmode: WcmTargetMode) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetTargetMode)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(targetmode)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetTemporaryStoreLocation(&self) -> ::windows_core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::super::Foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).GetTemporaryStoreLocation)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn SetTemporaryStoreLocation<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, temporarystorelocation: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetTemporaryStoreLocation)(::windows_core::Interface::as_raw(self), temporarystorelocation.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetTargetID(&self) -> ::windows_core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::super::Foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).GetTargetID)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn SetTargetID<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::GUID>>(&self, targetid: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetTargetID)(::windows_core::Interface::as_raw(self), targetid.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetTargetProcessorArchitecture(&self) -> ::windows_core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::super::Foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).GetTargetProcessorArchitecture)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn SetTargetProcessorArchitecture<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, processorarchitecture: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetTargetProcessorArchitecture)(::windows_core::Interface::as_raw(self), processorarchitecture.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetProperty<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::BOOL>, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, offline: Param0, property: Param1) -> ::windows_core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::super::Foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).GetProperty)(::windows_core::Interface::as_raw(self), offline.into_param().abi(), property.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetProperty<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::BOOL>, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param2: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, offline: Param0, property: Param1, value: Param2) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetProperty)(::windows_core::Interface::as_raw(self), offline.into_param().abi(), property.into_param().abi(), value.into_param().abi()).ok()
    }
    pub unsafe fn GetEnumerator(&self) -> ::windows_core::Result<IItemEnumerator> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetEnumerator)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IItemEnumerator>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ExpandTarget<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::BOOL>, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, offline: Param0, location: Param1) -> ::windows_core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::super::Foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).ExpandTarget)(::windows_core::Interface::as_raw(self), offline.into_param().abi(), location.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ExpandTargetPath<'a, Param0: ::windows_core::IntoParam<'a, super::super::Foundation::BOOL>, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, offline: Param0, location: Param1) -> ::windows_core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::super::Foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).ExpandTargetPath)(::windows_core::Interface::as_raw(self), offline.into_param().abi(), location.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn SetModulePath<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, module: Param0, path: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetModulePath)(::windows_core::Interface::as_raw(self), module.into_param().abi(), path.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn LoadModule<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, module: Param0) -> ::windows_core::Result<super::super::Foundation::HINSTANCE> {
        let mut result__ = ::core::mem::MaybeUninit::<super::super::Foundation::HINSTANCE>::zeroed();
        (::windows_core::Interface::vtable(self).LoadModule)(::windows_core::Interface::as_raw(self), module.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::HINSTANCE>(result__)
    }
    pub unsafe fn SetWow64Context<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, installermodule: Param0, wow64context: *const u8) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetWow64Context)(::windows_core::Interface::as_raw(self), installermodule.into_param().abi(), ::core::mem::transmute(wow64context)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn TranslateWow64<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>, Param1: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, clientarchitecture: Param0, value: Param1) -> ::windows_core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::super::Foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).TranslateWow64)(::windows_core::Interface::as_raw(self), clientarchitecture.into_param().abi(), value.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn SetSchemaHiveLocation<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, pwzhivedir: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetSchemaHiveLocation)(::windows_core::Interface::as_raw(self), pwzhivedir.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetSchemaHiveLocation(&self) -> ::windows_core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::super::Foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).GetSchemaHiveLocation)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn SetSchemaHiveMountName<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::PCWSTR>>(&self, pwzmountname: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetSchemaHiveMountName)(::windows_core::Interface::as_raw(self), pwzmountname.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetSchemaHiveMountName(&self) -> ::windows_core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::super::Foundation::BSTR>>::zeroed();
        (::windows_core::Interface::vtable(self).GetSchemaHiveMountName)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
}
impl ::core::convert::From<ITargetInfo> for ::windows_core::IUnknown {
    fn from(value: ITargetInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITargetInfo> for ::windows_core::IUnknown {
    fn from(value: &ITargetInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ITargetInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ITargetInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITargetInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITargetInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITargetInfo {}
impl ::core::fmt::Debug for ITargetInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITargetInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for ITargetInfo {
    type Vtable = ITargetInfo_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9f7d7bb8_20b3_11da_81a5_0030f1642e3c);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITargetInfo_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub GetTargetMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, targetmode: *mut WcmTargetMode) -> ::windows_core::HRESULT,
    pub SetTargetMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, targetmode: WcmTargetMode) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetTemporaryStoreLocation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, temporarystorelocation: *mut super::super::Foundation::BSTR) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetTemporaryStoreLocation: usize,
    pub SetTemporaryStoreLocation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, temporarystorelocation: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetTargetID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, targetid: *mut super::super::Foundation::BSTR) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetTargetID: usize,
    pub SetTargetID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, targetid: ::windows_core::GUID) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetTargetProcessorArchitecture: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, processorarchitecture: *mut super::super::Foundation::BSTR) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetTargetProcessorArchitecture: usize,
    pub SetTargetProcessorArchitecture: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, processorarchitecture: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, offline: super::super::Foundation::BOOL, property: ::windows_core::PCWSTR, value: *mut super::super::Foundation::BSTR) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetProperty: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, offline: super::super::Foundation::BOOL, property: ::windows_core::PCWSTR, value: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetProperty: usize,
    pub GetEnumerator: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, enumerator: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub ExpandTarget: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, offline: super::super::Foundation::BOOL, location: ::windows_core::PCWSTR, expandedlocation: *mut super::super::Foundation::BSTR) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ExpandTarget: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ExpandTargetPath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, offline: super::super::Foundation::BOOL, location: ::windows_core::PCWSTR, expandedlocation: *mut super::super::Foundation::BSTR) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ExpandTargetPath: usize,
    pub SetModulePath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, module: ::windows_core::PCWSTR, path: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub LoadModule: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, module: ::windows_core::PCWSTR, modulehandle: *mut super::super::Foundation::HINSTANCE) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    LoadModule: usize,
    pub SetWow64Context: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, installermodule: ::windows_core::PCWSTR, wow64context: *const u8) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub TranslateWow64: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, clientarchitecture: ::windows_core::PCWSTR, value: ::windows_core::PCWSTR, translatedvalue: *mut super::super::Foundation::BSTR) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    TranslateWow64: usize,
    pub SetSchemaHiveLocation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwzhivedir: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetSchemaHiveLocation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phivelocation: *mut super::super::Foundation::BSTR) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetSchemaHiveLocation: usize,
    pub SetSchemaHiveMountName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwzmountname: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetSchemaHiveMountName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmountname: *mut super::super::Foundation::BSTR) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetSchemaHiveMountName: usize,
}
pub const LIMITED_VALIDATION_MODE: u32 = 1u32;
pub const LINK_STORE_TO_ENGINE_INSTANCE: u32 = 1u32;
pub const SettingsEngine: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9f7d7bb5_20b3_11da_81a5_0030f1642e3c);
pub const WCM_E_ABORTOPERATION: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145255384i32);
pub const WCM_E_ASSERTIONFAILED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145255398i32);
pub const WCM_E_ATTRIBUTENOTALLOWED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145255420i32);
pub const WCM_E_ATTRIBUTENOTFOUND: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145255421i32);
pub const WCM_E_CONFLICTINGASSERTION: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145255399i32);
pub const WCM_E_CYCLICREFERENCE: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145255389i32);
pub const WCM_E_DUPLICATENAME: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145255397i32);
pub const WCM_E_EXPRESSIONNOTFOUND: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145255408i32);
pub const WCM_E_HANDLERNOTFOUND: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145255394i32);
pub const WCM_E_INTERNALERROR: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145255424i32);
pub const WCM_E_INVALIDATTRIBUTECOMBINATION: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145255385i32);
pub const WCM_E_INVALIDDATATYPE: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145255416i32);
pub const WCM_E_INVALIDEXPRESSIONSYNTAX: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145255401i32);
pub const WCM_E_INVALIDHANDLERSYNTAX: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145255393i32);
pub const WCM_E_INVALIDKEY: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145255396i32);
pub const WCM_E_INVALIDLANGUAGEFORMAT: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145255410i32);
pub const WCM_E_INVALIDPATH: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145255413i32);
pub const WCM_E_INVALIDPROCESSORFORMAT: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145255382i32);
pub const WCM_E_INVALIDSTREAM: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145255395i32);
pub const WCM_E_INVALIDVALUE: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145255419i32);
pub const WCM_E_INVALIDVALUEFORMAT: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145255418i32);
pub const WCM_E_INVALIDVERSIONFORMAT: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145255411i32);
pub const WCM_E_KEYNOTCHANGEABLE: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145255409i32);
pub const WCM_E_MANIFESTCOMPILATIONFAILED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145255390i32);
pub const WCM_E_MISSINGCONFIGURATION: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145255383i32);
pub const WCM_E_MIXTYPEASSERTION: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145255388i32);
pub const WCM_E_NAMESPACEALREADYREGISTERED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145255403i32);
pub const WCM_E_NAMESPACENOTFOUND: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145255404i32);
pub const WCM_E_NOTIFICATIONNOTFOUND: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145255400i32);
pub const WCM_E_NOTPOSITIONED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145255415i32);
pub const WCM_E_NOTSUPPORTEDFUNCTION: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145255387i32);
pub const WCM_E_READONLYITEM: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145255414i32);
pub const WCM_E_RESTRICTIONFAILED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145255391i32);
pub const WCM_E_SOURCEMANEMPTYVALUE: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145255381i32);
pub const WCM_E_STATENODENOTALLOWED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145255422i32);
pub const WCM_E_STATENODENOTFOUND: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145255423i32);
pub const WCM_E_STORECORRUPTED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145255402i32);
pub const WCM_E_SUBSTITUTIONNOTFOUND: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145255407i32);
pub const WCM_E_TYPENOTSPECIFIED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145255417i32);
pub const WCM_E_UNKNOWNRESULT: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145251325i32);
pub const WCM_E_USERALREADYREGISTERED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145255406i32);
pub const WCM_E_USERNOTFOUND: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145255405i32);
pub const WCM_E_VALIDATIONFAILED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145255392i32);
pub const WCM_E_VALUETOOBIG: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145255386i32);
pub const WCM_E_WRONGESCAPESTRING: ::windows_core::HRESULT = ::windows_core::HRESULT(-2145255412i32);
pub const WCM_SETTINGS_ID_ARCHITECTURE: &str = "architecture";
pub const WCM_SETTINGS_ID_FLAG_DEFINITION: u32 = 1u32;
pub const WCM_SETTINGS_ID_FLAG_REFERENCE: u32 = 0u32;
pub const WCM_SETTINGS_ID_LANGUAGE: &str = "language";
pub const WCM_SETTINGS_ID_NAME: &str = "name";
pub const WCM_SETTINGS_ID_TOKEN: &str = "token";
pub const WCM_SETTINGS_ID_URI: &str = "uri";
pub const WCM_SETTINGS_ID_VERSION: &str = "version";
pub const WCM_SETTINGS_ID_VERSION_SCOPE: &str = "versionScope";
pub const WCM_S_ATTRIBUTENOTALLOWED: ::windows_core::HRESULT = ::windows_core::HRESULT(2232325i32);
pub const WCM_S_ATTRIBUTENOTFOUND: ::windows_core::HRESULT = ::windows_core::HRESULT(2232321i32);
pub const WCM_S_INTERNALERROR: ::windows_core::HRESULT = ::windows_core::HRESULT(2232320i32);
pub const WCM_S_INVALIDATTRIBUTECOMBINATION: ::windows_core::HRESULT = ::windows_core::HRESULT(2232324i32);
pub const WCM_S_LEGACYSETTINGWARNING: ::windows_core::HRESULT = ::windows_core::HRESULT(2232322i32);
pub const WCM_S_NAMESPACENOTFOUND: ::windows_core::HRESULT = ::windows_core::HRESULT(2232326i32);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WcmDataType(pub i32);
pub const dataTypeByte: WcmDataType = WcmDataType(1i32);
pub const dataTypeSByte: WcmDataType = WcmDataType(2i32);
pub const dataTypeUInt16: WcmDataType = WcmDataType(3i32);
pub const dataTypeInt16: WcmDataType = WcmDataType(4i32);
pub const dataTypeUInt32: WcmDataType = WcmDataType(5i32);
pub const dataTypeInt32: WcmDataType = WcmDataType(6i32);
pub const dataTypeUInt64: WcmDataType = WcmDataType(7i32);
pub const dataTypeInt64: WcmDataType = WcmDataType(8i32);
pub const dataTypeBoolean: WcmDataType = WcmDataType(11i32);
pub const dataTypeString: WcmDataType = WcmDataType(12i32);
pub const dataTypeFlagArray: WcmDataType = WcmDataType(32768i32);
impl ::core::marker::Copy for WcmDataType {}
impl ::core::clone::Clone for WcmDataType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WcmDataType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for WcmDataType {
    type Abi = Self;
}
impl ::core::fmt::Debug for WcmDataType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WcmDataType").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WcmNamespaceAccess(pub i32);
pub const ReadOnlyAccess: WcmNamespaceAccess = WcmNamespaceAccess(1i32);
pub const ReadWriteAccess: WcmNamespaceAccess = WcmNamespaceAccess(2i32);
impl ::core::marker::Copy for WcmNamespaceAccess {}
impl ::core::clone::Clone for WcmNamespaceAccess {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WcmNamespaceAccess {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for WcmNamespaceAccess {
    type Abi = Self;
}
impl ::core::fmt::Debug for WcmNamespaceAccess {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WcmNamespaceAccess").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WcmNamespaceEnumerationFlags(pub i32);
pub const SharedEnumeration: WcmNamespaceEnumerationFlags = WcmNamespaceEnumerationFlags(1i32);
pub const UserEnumeration: WcmNamespaceEnumerationFlags = WcmNamespaceEnumerationFlags(2i32);
pub const AllEnumeration: WcmNamespaceEnumerationFlags = WcmNamespaceEnumerationFlags(3i32);
impl ::core::marker::Copy for WcmNamespaceEnumerationFlags {}
impl ::core::clone::Clone for WcmNamespaceEnumerationFlags {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WcmNamespaceEnumerationFlags {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for WcmNamespaceEnumerationFlags {
    type Abi = Self;
}
impl ::core::fmt::Debug for WcmNamespaceEnumerationFlags {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WcmNamespaceEnumerationFlags").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WcmRestrictionFacets(pub i32);
pub const restrictionFacetMaxLength: WcmRestrictionFacets = WcmRestrictionFacets(1i32);
pub const restrictionFacetEnumeration: WcmRestrictionFacets = WcmRestrictionFacets(2i32);
pub const restrictionFacetMaxInclusive: WcmRestrictionFacets = WcmRestrictionFacets(4i32);
pub const restrictionFacetMinInclusive: WcmRestrictionFacets = WcmRestrictionFacets(8i32);
impl ::core::marker::Copy for WcmRestrictionFacets {}
impl ::core::clone::Clone for WcmRestrictionFacets {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WcmRestrictionFacets {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for WcmRestrictionFacets {
    type Abi = Self;
}
impl ::core::fmt::Debug for WcmRestrictionFacets {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WcmRestrictionFacets").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WcmSettingType(pub i32);
pub const settingTypeScalar: WcmSettingType = WcmSettingType(1i32);
pub const settingTypeComplex: WcmSettingType = WcmSettingType(2i32);
pub const settingTypeList: WcmSettingType = WcmSettingType(3i32);
impl ::core::marker::Copy for WcmSettingType {}
impl ::core::clone::Clone for WcmSettingType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WcmSettingType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for WcmSettingType {
    type Abi = Self;
}
impl ::core::fmt::Debug for WcmSettingType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WcmSettingType").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WcmTargetMode(pub i32);
pub const OfflineMode: WcmTargetMode = WcmTargetMode(1i32);
pub const OnlineMode: WcmTargetMode = WcmTargetMode(2i32);
impl ::core::marker::Copy for WcmTargetMode {}
impl ::core::clone::Clone for WcmTargetMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WcmTargetMode {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for WcmTargetMode {
    type Abi = Self;
}
impl ::core::fmt::Debug for WcmTargetMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WcmTargetMode").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WcmUserStatus(pub i32);
pub const UnknownStatus: WcmUserStatus = WcmUserStatus(0i32);
pub const UserRegistered: WcmUserStatus = WcmUserStatus(1i32);
pub const UserUnregistered: WcmUserStatus = WcmUserStatus(2i32);
pub const UserLoaded: WcmUserStatus = WcmUserStatus(3i32);
pub const UserUnloaded: WcmUserStatus = WcmUserStatus(4i32);
impl ::core::marker::Copy for WcmUserStatus {}
impl ::core::clone::Clone for WcmUserStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WcmUserStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for WcmUserStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for WcmUserStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WcmUserStatus").field(&self.0).finish()
    }
}
