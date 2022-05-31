pub const E_SURFACE_CONTENTS_LOST: u32 = 2150301728u32;
#[repr(transparent)]
pub struct IDesktopWindowXamlSourceNative(::windows_core::IUnknown);
impl IDesktopWindowXamlSourceNative {
    pub unsafe fn AttachToWindow<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HWND>>(&self, parentwnd: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).AttachToWindow)(::windows_core::Interface::as_raw(self), parentwnd.into_param().abi()).ok()
    }
    pub unsafe fn WindowHandle(&self) -> ::windows_core::Result<::win32_foundation::HWND> {
        let mut result__ = ::core::mem::MaybeUninit::<::win32_foundation::HWND>::zeroed();
        (::windows_core::Interface::vtable(self).WindowHandle)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::HWND>(result__)
    }
}
impl ::core::convert::From<IDesktopWindowXamlSourceNative> for ::windows_core::IUnknown {
    fn from(value: IDesktopWindowXamlSourceNative) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDesktopWindowXamlSourceNative> for ::windows_core::IUnknown {
    fn from(value: &IDesktopWindowXamlSourceNative) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IDesktopWindowXamlSourceNative {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IDesktopWindowXamlSourceNative {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDesktopWindowXamlSourceNative {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDesktopWindowXamlSourceNative {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDesktopWindowXamlSourceNative {}
impl ::core::fmt::Debug for IDesktopWindowXamlSourceNative {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDesktopWindowXamlSourceNative").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IDesktopWindowXamlSourceNative {
    type Vtable = IDesktopWindowXamlSourceNative_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3cbcf1bf_2f76_4e9c_96ab_e84b37972554);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDesktopWindowXamlSourceNative_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub AttachToWindow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, parentwnd: ::win32_foundation::HWND) -> ::windows_core::HRESULT,
    pub WindowHandle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwnd: *mut ::win32_foundation::HWND) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IDesktopWindowXamlSourceNative2(::windows_core::IUnknown);
impl IDesktopWindowXamlSourceNative2 {
    pub unsafe fn AttachToWindow<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HWND>>(&self, parentwnd: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.AttachToWindow)(::windows_core::Interface::as_raw(self), parentwnd.into_param().abi()).ok()
    }
    pub unsafe fn WindowHandle(&self) -> ::windows_core::Result<::win32_foundation::HWND> {
        let mut result__ = ::core::mem::MaybeUninit::<::win32_foundation::HWND>::zeroed();
        (::windows_core::Interface::vtable(self).base__.WindowHandle)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::HWND>(result__)
    }
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub unsafe fn PreTranslateMessage(&self, message: *const ::win32_ui::WindowsAndMessaging::MSG, result: *mut ::win32_foundation::BOOL) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).PreTranslateMessage)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(message), ::core::mem::transmute(result)).ok()
    }
}
impl ::core::convert::From<IDesktopWindowXamlSourceNative2> for ::windows_core::IUnknown {
    fn from(value: IDesktopWindowXamlSourceNative2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDesktopWindowXamlSourceNative2> for ::windows_core::IUnknown {
    fn from(value: &IDesktopWindowXamlSourceNative2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IDesktopWindowXamlSourceNative2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IDesktopWindowXamlSourceNative2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDesktopWindowXamlSourceNative2> for IDesktopWindowXamlSourceNative {
    fn from(value: IDesktopWindowXamlSourceNative2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDesktopWindowXamlSourceNative2> for IDesktopWindowXamlSourceNative {
    fn from(value: &IDesktopWindowXamlSourceNative2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, IDesktopWindowXamlSourceNative> for IDesktopWindowXamlSourceNative2 {
    fn into_param(self) -> ::windows_core::Param<'a, IDesktopWindowXamlSourceNative> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, IDesktopWindowXamlSourceNative> for &'a IDesktopWindowXamlSourceNative2 {
    fn into_param(self) -> ::windows_core::Param<'a, IDesktopWindowXamlSourceNative> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDesktopWindowXamlSourceNative2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDesktopWindowXamlSourceNative2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDesktopWindowXamlSourceNative2 {}
impl ::core::fmt::Debug for IDesktopWindowXamlSourceNative2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDesktopWindowXamlSourceNative2").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IDesktopWindowXamlSourceNative2 {
    type Vtable = IDesktopWindowXamlSourceNative2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe3dcd8c7_3057_4692_99c3_7b7720afda31);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDesktopWindowXamlSourceNative2_Vtbl {
    pub base__: IDesktopWindowXamlSourceNative_Vtbl,
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub PreTranslateMessage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, message: *const ::win32_ui::WindowsAndMessaging::MSG, result: *mut ::win32_foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_WindowsAndMessaging"))]
    PreTranslateMessage: usize,
}
#[repr(transparent)]
pub struct IFindReferenceTargetsCallback(::windows_core::IUnknown);
impl IFindReferenceTargetsCallback {
    pub unsafe fn FoundTrackerTarget<'a, Param0: ::windows_core::IntoParam<'a, IReferenceTrackerTarget>>(&self, target: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).FoundTrackerTarget)(::windows_core::Interface::as_raw(self), target.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IFindReferenceTargetsCallback> for ::windows_core::IUnknown {
    fn from(value: IFindReferenceTargetsCallback) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IFindReferenceTargetsCallback> for ::windows_core::IUnknown {
    fn from(value: &IFindReferenceTargetsCallback) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IFindReferenceTargetsCallback {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IFindReferenceTargetsCallback {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IFindReferenceTargetsCallback {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IFindReferenceTargetsCallback {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFindReferenceTargetsCallback {}
impl ::core::fmt::Debug for IFindReferenceTargetsCallback {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFindReferenceTargetsCallback").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IFindReferenceTargetsCallback {
    type Vtable = IFindReferenceTargetsCallback_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x04b3486c_4687_4229_8d14_505ab584dd88);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFindReferenceTargetsCallback_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub FoundTrackerTarget: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, target: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IReferenceTracker(::windows_core::IUnknown);
impl IReferenceTracker {
    pub unsafe fn ConnectFromTrackerSource(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ConnectFromTrackerSource)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn DisconnectFromTrackerSource(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).DisconnectFromTrackerSource)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn FindTrackerTargets<'a, Param0: ::windows_core::IntoParam<'a, IFindReferenceTargetsCallback>>(&self, callback: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).FindTrackerTargets)(::windows_core::Interface::as_raw(self), callback.into_param().abi()).ok()
    }
    pub unsafe fn GetReferenceTrackerManager(&self) -> ::windows_core::Result<IReferenceTrackerManager> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetReferenceTrackerManager)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IReferenceTrackerManager>(result__)
    }
    pub unsafe fn AddRefFromTrackerSource(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).AddRefFromTrackerSource)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn ReleaseFromTrackerSource(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ReleaseFromTrackerSource)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn PegFromTrackerSource(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).PegFromTrackerSource)(::windows_core::Interface::as_raw(self)).ok()
    }
}
impl ::core::convert::From<IReferenceTracker> for ::windows_core::IUnknown {
    fn from(value: IReferenceTracker) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IReferenceTracker> for ::windows_core::IUnknown {
    fn from(value: &IReferenceTracker) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IReferenceTracker {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IReferenceTracker {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IReferenceTracker {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IReferenceTracker {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IReferenceTracker {}
impl ::core::fmt::Debug for IReferenceTracker {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IReferenceTracker").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IReferenceTracker {
    type Vtable = IReferenceTracker_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x11d3b13a_180e_4789_a8be_7712882893e6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IReferenceTracker_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub ConnectFromTrackerSource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub DisconnectFromTrackerSource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub FindTrackerTargets: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, callback: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetReferenceTrackerManager: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub AddRefFromTrackerSource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub ReleaseFromTrackerSource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub PegFromTrackerSource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IReferenceTrackerExtension(::windows_core::IUnknown);
impl IReferenceTrackerExtension {}
impl ::core::convert::From<IReferenceTrackerExtension> for ::windows_core::IUnknown {
    fn from(value: IReferenceTrackerExtension) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IReferenceTrackerExtension> for ::windows_core::IUnknown {
    fn from(value: &IReferenceTrackerExtension) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IReferenceTrackerExtension {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IReferenceTrackerExtension {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IReferenceTrackerExtension {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IReferenceTrackerExtension {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IReferenceTrackerExtension {}
impl ::core::fmt::Debug for IReferenceTrackerExtension {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IReferenceTrackerExtension").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IReferenceTrackerExtension {
    type Vtable = IReferenceTrackerExtension_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4e897caa_59d5_4613_8f8c_f7ebd1f399b0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IReferenceTrackerExtension_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
}
#[repr(transparent)]
pub struct IReferenceTrackerHost(::windows_core::IUnknown);
impl IReferenceTrackerHost {
    pub unsafe fn DisconnectUnusedReferenceSources(&self, options: XAML_REFERENCETRACKER_DISCONNECT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).DisconnectUnusedReferenceSources)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(options)).ok()
    }
    pub unsafe fn ReleaseDisconnectedReferenceSources(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ReleaseDisconnectedReferenceSources)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn NotifyEndOfReferenceTrackingOnThread(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).NotifyEndOfReferenceTrackingOnThread)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn GetTrackerTarget<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::IUnknown>>(&self, unknown: Param0) -> ::windows_core::Result<IReferenceTrackerTarget> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
        (::windows_core::Interface::vtable(self).GetTrackerTarget)(::windows_core::Interface::as_raw(self), unknown.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IReferenceTrackerTarget>(result__)
    }
    pub unsafe fn AddMemoryPressure(&self, bytesallocated: u64) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).AddMemoryPressure)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(bytesallocated)).ok()
    }
    pub unsafe fn RemoveMemoryPressure(&self, bytesallocated: u64) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).RemoveMemoryPressure)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(bytesallocated)).ok()
    }
}
impl ::core::convert::From<IReferenceTrackerHost> for ::windows_core::IUnknown {
    fn from(value: IReferenceTrackerHost) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IReferenceTrackerHost> for ::windows_core::IUnknown {
    fn from(value: &IReferenceTrackerHost) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IReferenceTrackerHost {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IReferenceTrackerHost {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IReferenceTrackerHost {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IReferenceTrackerHost {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IReferenceTrackerHost {}
impl ::core::fmt::Debug for IReferenceTrackerHost {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IReferenceTrackerHost").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IReferenceTrackerHost {
    type Vtable = IReferenceTrackerHost_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x29a71c6a_3c42_4416_a39d_e2825a07a773);
}
#[repr(C)]
#[doc(hidden)]
pub struct IReferenceTrackerHost_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub DisconnectUnusedReferenceSources: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, options: XAML_REFERENCETRACKER_DISCONNECT) -> ::windows_core::HRESULT,
    pub ReleaseDisconnectedReferenceSources: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub NotifyEndOfReferenceTrackingOnThread: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetTrackerTarget: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, unknown: *mut ::core::ffi::c_void, newreference: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub AddMemoryPressure: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bytesallocated: u64) -> ::windows_core::HRESULT,
    pub RemoveMemoryPressure: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bytesallocated: u64) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IReferenceTrackerManager(::windows_core::IUnknown);
impl IReferenceTrackerManager {
    pub unsafe fn ReferenceTrackingStarted(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ReferenceTrackingStarted)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn FindTrackerTargetsCompleted(&self, findfailed: u8) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).FindTrackerTargetsCompleted)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(findfailed)).ok()
    }
    pub unsafe fn ReferenceTrackingCompleted(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ReferenceTrackingCompleted)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SetReferenceTrackerHost<'a, Param0: ::windows_core::IntoParam<'a, IReferenceTrackerHost>>(&self, value: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetReferenceTrackerHost)(::windows_core::Interface::as_raw(self), value.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IReferenceTrackerManager> for ::windows_core::IUnknown {
    fn from(value: IReferenceTrackerManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IReferenceTrackerManager> for ::windows_core::IUnknown {
    fn from(value: &IReferenceTrackerManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IReferenceTrackerManager {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IReferenceTrackerManager {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IReferenceTrackerManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IReferenceTrackerManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IReferenceTrackerManager {}
impl ::core::fmt::Debug for IReferenceTrackerManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IReferenceTrackerManager").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IReferenceTrackerManager {
    type Vtable = IReferenceTrackerManager_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3cf184b4_7ccb_4dda_8455_7e6ce99a3298);
}
#[repr(C)]
#[doc(hidden)]
pub struct IReferenceTrackerManager_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub ReferenceTrackingStarted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub FindTrackerTargetsCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, findfailed: u8) -> ::windows_core::HRESULT,
    pub ReferenceTrackingCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetReferenceTrackerHost: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IReferenceTrackerTarget(::windows_core::IUnknown);
impl IReferenceTrackerTarget {
    pub unsafe fn AddRefFromReferenceTracker(&self) -> u32 {
        ::core::mem::transmute((::windows_core::Interface::vtable(self).AddRefFromReferenceTracker)(::windows_core::Interface::as_raw(self)))
    }
    pub unsafe fn ReleaseFromReferenceTracker(&self) -> u32 {
        ::core::mem::transmute((::windows_core::Interface::vtable(self).ReleaseFromReferenceTracker)(::windows_core::Interface::as_raw(self)))
    }
    pub unsafe fn Peg(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Peg)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Unpeg(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Unpeg)(::windows_core::Interface::as_raw(self)).ok()
    }
}
impl ::core::convert::From<IReferenceTrackerTarget> for ::windows_core::IUnknown {
    fn from(value: IReferenceTrackerTarget) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IReferenceTrackerTarget> for ::windows_core::IUnknown {
    fn from(value: &IReferenceTrackerTarget) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IReferenceTrackerTarget {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IReferenceTrackerTarget {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IReferenceTrackerTarget {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IReferenceTrackerTarget {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IReferenceTrackerTarget {}
impl ::core::fmt::Debug for IReferenceTrackerTarget {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IReferenceTrackerTarget").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IReferenceTrackerTarget {
    type Vtable = IReferenceTrackerTarget_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x64bd43f8_bfee_4ec4_b7eb_2935158dae21);
}
#[repr(C)]
#[doc(hidden)]
pub struct IReferenceTrackerTarget_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub AddRefFromReferenceTracker: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub ReleaseFromReferenceTracker: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub Peg: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Unpeg: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct ISurfaceImageSourceManagerNative(::windows_core::IUnknown);
impl ISurfaceImageSourceManagerNative {
    pub unsafe fn FlushAllSurfacesWithDevice<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::IUnknown>>(&self, device: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).FlushAllSurfacesWithDevice)(::windows_core::Interface::as_raw(self), device.into_param().abi()).ok()
    }
}
impl ::core::convert::From<ISurfaceImageSourceManagerNative> for ::windows_core::IUnknown {
    fn from(value: ISurfaceImageSourceManagerNative) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISurfaceImageSourceManagerNative> for ::windows_core::IUnknown {
    fn from(value: &ISurfaceImageSourceManagerNative) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ISurfaceImageSourceManagerNative {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ISurfaceImageSourceManagerNative {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ISurfaceImageSourceManagerNative {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISurfaceImageSourceManagerNative {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISurfaceImageSourceManagerNative {}
impl ::core::fmt::Debug for ISurfaceImageSourceManagerNative {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISurfaceImageSourceManagerNative").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for ISurfaceImageSourceManagerNative {
    type Vtable = ISurfaceImageSourceManagerNative_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4c8798b7_1d88_4a0f_b59b_b93f600de8c8);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISurfaceImageSourceManagerNative_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub FlushAllSurfacesWithDevice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, device: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct ISurfaceImageSourceNative(::windows_core::IUnknown);
impl ISurfaceImageSourceNative {
    #[cfg(feature = "Win32_Graphics_Dxgi")]
    pub unsafe fn SetDevice<'a, Param0: ::windows_core::IntoParam<'a, ::win32_graphics::Dxgi::IDXGIDevice>>(&self, device: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetDevice)(::windows_core::Interface::as_raw(self), device.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Graphics_Dxgi")]
    pub unsafe fn BeginDraw<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::RECT>>(&self, updaterect: Param0, surface: *mut ::core::option::Option<::win32_graphics::Dxgi::IDXGISurface>, offset: *mut ::win32_foundation::POINT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).BeginDraw)(::windows_core::Interface::as_raw(self), updaterect.into_param().abi(), ::core::mem::transmute(surface), ::core::mem::transmute(offset)).ok()
    }
    pub unsafe fn EndDraw(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).EndDraw)(::windows_core::Interface::as_raw(self)).ok()
    }
}
impl ::core::convert::From<ISurfaceImageSourceNative> for ::windows_core::IUnknown {
    fn from(value: ISurfaceImageSourceNative) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISurfaceImageSourceNative> for ::windows_core::IUnknown {
    fn from(value: &ISurfaceImageSourceNative) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ISurfaceImageSourceNative {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ISurfaceImageSourceNative {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ISurfaceImageSourceNative {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISurfaceImageSourceNative {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISurfaceImageSourceNative {}
impl ::core::fmt::Debug for ISurfaceImageSourceNative {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISurfaceImageSourceNative").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for ISurfaceImageSourceNative {
    type Vtable = ISurfaceImageSourceNative_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf2e9edc1_d307_4525_9886_0fafaa44163c);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISurfaceImageSourceNative_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    #[cfg(feature = "Win32_Graphics_Dxgi")]
    pub SetDevice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, device: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi"))]
    SetDevice: usize,
    #[cfg(feature = "Win32_Graphics_Dxgi")]
    pub BeginDraw: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, updaterect: ::win32_foundation::RECT, surface: *mut ::windows_core::RawPtr, offset: *mut ::win32_foundation::POINT) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi"))]
    BeginDraw: usize,
    pub EndDraw: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct ISurfaceImageSourceNativeWithD2D(::windows_core::IUnknown);
impl ISurfaceImageSourceNativeWithD2D {
    pub unsafe fn SetDevice<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::IUnknown>>(&self, device: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetDevice)(::windows_core::Interface::as_raw(self), device.into_param().abi()).ok()
    }
    pub unsafe fn BeginDraw(&self, updaterect: *const ::win32_foundation::RECT, iid: *const ::windows_core::GUID, updateobject: *mut *mut ::core::ffi::c_void, offset: *mut ::win32_foundation::POINT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).BeginDraw)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(updaterect), ::core::mem::transmute(iid), ::core::mem::transmute(updateobject), ::core::mem::transmute(offset)).ok()
    }
    pub unsafe fn EndDraw(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).EndDraw)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SuspendDraw(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SuspendDraw)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn ResumeDraw(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ResumeDraw)(::windows_core::Interface::as_raw(self)).ok()
    }
}
impl ::core::convert::From<ISurfaceImageSourceNativeWithD2D> for ::windows_core::IUnknown {
    fn from(value: ISurfaceImageSourceNativeWithD2D) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISurfaceImageSourceNativeWithD2D> for ::windows_core::IUnknown {
    fn from(value: &ISurfaceImageSourceNativeWithD2D) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ISurfaceImageSourceNativeWithD2D {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ISurfaceImageSourceNativeWithD2D {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ISurfaceImageSourceNativeWithD2D {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISurfaceImageSourceNativeWithD2D {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISurfaceImageSourceNativeWithD2D {}
impl ::core::fmt::Debug for ISurfaceImageSourceNativeWithD2D {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISurfaceImageSourceNativeWithD2D").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for ISurfaceImageSourceNativeWithD2D {
    type Vtable = ISurfaceImageSourceNativeWithD2D_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x54298223_41e1_4a41_9c08_02e8256864a1);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISurfaceImageSourceNativeWithD2D_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub SetDevice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, device: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub BeginDraw: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, updaterect: *const ::win32_foundation::RECT, iid: *const ::windows_core::GUID, updateobject: *mut *mut ::core::ffi::c_void, offset: *mut ::win32_foundation::POINT) -> ::windows_core::HRESULT,
    pub EndDraw: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SuspendDraw: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub ResumeDraw: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct ISwapChainBackgroundPanelNative(::windows_core::IUnknown);
impl ISwapChainBackgroundPanelNative {
    #[cfg(feature = "Win32_Graphics_Dxgi")]
    pub unsafe fn SetSwapChain<'a, Param0: ::windows_core::IntoParam<'a, ::win32_graphics::Dxgi::IDXGISwapChain>>(&self, swapchain: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetSwapChain)(::windows_core::Interface::as_raw(self), swapchain.into_param().abi()).ok()
    }
}
impl ::core::convert::From<ISwapChainBackgroundPanelNative> for ::windows_core::IUnknown {
    fn from(value: ISwapChainBackgroundPanelNative) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISwapChainBackgroundPanelNative> for ::windows_core::IUnknown {
    fn from(value: &ISwapChainBackgroundPanelNative) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ISwapChainBackgroundPanelNative {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ISwapChainBackgroundPanelNative {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ISwapChainBackgroundPanelNative {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISwapChainBackgroundPanelNative {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISwapChainBackgroundPanelNative {}
impl ::core::fmt::Debug for ISwapChainBackgroundPanelNative {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISwapChainBackgroundPanelNative").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for ISwapChainBackgroundPanelNative {
    type Vtable = ISwapChainBackgroundPanelNative_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x43bebd4e_add5_4035_8f85_5608d08e9dc9);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISwapChainBackgroundPanelNative_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    #[cfg(feature = "Win32_Graphics_Dxgi")]
    pub SetSwapChain: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, swapchain: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi"))]
    SetSwapChain: usize,
}
#[repr(transparent)]
pub struct ISwapChainPanelNative(::windows_core::IUnknown);
impl ISwapChainPanelNative {
    #[cfg(feature = "Win32_Graphics_Dxgi")]
    pub unsafe fn SetSwapChain<'a, Param0: ::windows_core::IntoParam<'a, ::win32_graphics::Dxgi::IDXGISwapChain>>(&self, swapchain: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetSwapChain)(::windows_core::Interface::as_raw(self), swapchain.into_param().abi()).ok()
    }
}
impl ::core::convert::From<ISwapChainPanelNative> for ::windows_core::IUnknown {
    fn from(value: ISwapChainPanelNative) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISwapChainPanelNative> for ::windows_core::IUnknown {
    fn from(value: &ISwapChainPanelNative) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ISwapChainPanelNative {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ISwapChainPanelNative {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ISwapChainPanelNative {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISwapChainPanelNative {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISwapChainPanelNative {}
impl ::core::fmt::Debug for ISwapChainPanelNative {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISwapChainPanelNative").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for ISwapChainPanelNative {
    type Vtable = ISwapChainPanelNative_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf92f19d2_3ade_45a6_a20c_f6f1ea90554b);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISwapChainPanelNative_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    #[cfg(feature = "Win32_Graphics_Dxgi")]
    pub SetSwapChain: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, swapchain: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi"))]
    SetSwapChain: usize,
}
#[repr(transparent)]
pub struct ISwapChainPanelNative2(::windows_core::IUnknown);
impl ISwapChainPanelNative2 {
    #[cfg(feature = "Win32_Graphics_Dxgi")]
    pub unsafe fn SetSwapChain<'a, Param0: ::windows_core::IntoParam<'a, ::win32_graphics::Dxgi::IDXGISwapChain>>(&self, swapchain: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetSwapChain)(::windows_core::Interface::as_raw(self), swapchain.into_param().abi()).ok()
    }
    pub unsafe fn SetSwapChainHandle<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::HANDLE>>(&self, swapchainhandle: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetSwapChainHandle)(::windows_core::Interface::as_raw(self), swapchainhandle.into_param().abi()).ok()
    }
}
impl ::core::convert::From<ISwapChainPanelNative2> for ::windows_core::IUnknown {
    fn from(value: ISwapChainPanelNative2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISwapChainPanelNative2> for ::windows_core::IUnknown {
    fn from(value: &ISwapChainPanelNative2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ISwapChainPanelNative2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ISwapChainPanelNative2 {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ISwapChainPanelNative2> for ISwapChainPanelNative {
    fn from(value: ISwapChainPanelNative2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISwapChainPanelNative2> for ISwapChainPanelNative {
    fn from(value: &ISwapChainPanelNative2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ISwapChainPanelNative> for ISwapChainPanelNative2 {
    fn into_param(self) -> ::windows_core::Param<'a, ISwapChainPanelNative> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ISwapChainPanelNative> for &'a ISwapChainPanelNative2 {
    fn into_param(self) -> ::windows_core::Param<'a, ISwapChainPanelNative> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ISwapChainPanelNative2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISwapChainPanelNative2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISwapChainPanelNative2 {}
impl ::core::fmt::Debug for ISwapChainPanelNative2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISwapChainPanelNative2").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for ISwapChainPanelNative2 {
    type Vtable = ISwapChainPanelNative2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd5a2f60c_37b2_44a2_937b_8d8eb9726821);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISwapChainPanelNative2_Vtbl {
    pub base__: ISwapChainPanelNative_Vtbl,
    pub SetSwapChainHandle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, swapchainhandle: ::win32_foundation::HANDLE) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct ITrackerOwner(::windows_core::IUnknown);
impl ITrackerOwner {
    pub unsafe fn CreateTrackerHandle(&self) -> ::windows_core::Result<*mut TrackerHandle__> {
        let mut result__ = ::core::mem::MaybeUninit::<*mut TrackerHandle__>::zeroed();
        (::windows_core::Interface::vtable(self).CreateTrackerHandle)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<*mut TrackerHandle__>(result__)
    }
    pub unsafe fn DeleteTrackerHandle(&self, handle: *const TrackerHandle__) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).DeleteTrackerHandle)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(handle)).ok()
    }
    pub unsafe fn SetTrackerValue<'a, Param1: ::windows_core::IntoParam<'a, ::windows_core::IUnknown>>(&self, handle: *const TrackerHandle__, value: Param1) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetTrackerValue)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(handle), value.into_param().abi()).ok()
    }
    pub unsafe fn TryGetSafeTrackerValue(&self, handle: *const TrackerHandle__, returnvalue: *mut ::core::option::Option<::windows_core::IUnknown>) -> u8 {
        ::core::mem::transmute((::windows_core::Interface::vtable(self).TryGetSafeTrackerValue)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(handle), ::core::mem::transmute(returnvalue)))
    }
}
impl ::core::convert::From<ITrackerOwner> for ::windows_core::IUnknown {
    fn from(value: ITrackerOwner) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITrackerOwner> for ::windows_core::IUnknown {
    fn from(value: &ITrackerOwner) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for ITrackerOwner {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a ITrackerOwner {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITrackerOwner {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITrackerOwner {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITrackerOwner {}
impl ::core::fmt::Debug for ITrackerOwner {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITrackerOwner").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for ITrackerOwner {
    type Vtable = ITrackerOwner_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xeb24c20b_9816_4ac7_8cff_36f67a118f4e);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITrackerOwner_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub CreateTrackerHandle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, returnvalue: *mut *mut TrackerHandle__) -> ::windows_core::HRESULT,
    pub DeleteTrackerHandle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handle: *const TrackerHandle__) -> ::windows_core::HRESULT,
    pub SetTrackerValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handle: *const TrackerHandle__, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub TryGetSafeTrackerValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handle: *const TrackerHandle__, returnvalue: *mut *mut ::core::ffi::c_void) -> u8,
}
#[repr(transparent)]
pub struct IVirtualSurfaceImageSourceNative(::windows_core::IUnknown);
impl IVirtualSurfaceImageSourceNative {
    #[cfg(feature = "Win32_Graphics_Dxgi")]
    pub unsafe fn SetDevice<'a, Param0: ::windows_core::IntoParam<'a, ::win32_graphics::Dxgi::IDXGIDevice>>(&self, device: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetDevice)(::windows_core::Interface::as_raw(self), device.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Graphics_Dxgi")]
    pub unsafe fn BeginDraw<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::RECT>>(&self, updaterect: Param0, surface: *mut ::core::option::Option<::win32_graphics::Dxgi::IDXGISurface>, offset: *mut ::win32_foundation::POINT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.BeginDraw)(::windows_core::Interface::as_raw(self), updaterect.into_param().abi(), ::core::mem::transmute(surface), ::core::mem::transmute(offset)).ok()
    }
    pub unsafe fn EndDraw(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.EndDraw)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Invalidate<'a, Param0: ::windows_core::IntoParam<'a, ::win32_foundation::RECT>>(&self, updaterect: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Invalidate)(::windows_core::Interface::as_raw(self), updaterect.into_param().abi()).ok()
    }
    pub unsafe fn GetUpdateRectCount(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows_core::Interface::vtable(self).GetUpdateRectCount)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn GetUpdateRects(&self, updates: &mut [::win32_foundation::RECT]) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetUpdateRects)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(::windows_core::as_mut_ptr_or_null(updates)), updates.len() as _).ok()
    }
    pub unsafe fn GetVisibleBounds(&self) -> ::windows_core::Result<::win32_foundation::RECT> {
        let mut result__ = ::core::mem::MaybeUninit::<::win32_foundation::RECT>::zeroed();
        (::windows_core::Interface::vtable(self).GetVisibleBounds)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::win32_foundation::RECT>(result__)
    }
    pub unsafe fn RegisterForUpdatesNeeded<'a, Param0: ::windows_core::IntoParam<'a, IVirtualSurfaceUpdatesCallbackNative>>(&self, callback: Param0) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).RegisterForUpdatesNeeded)(::windows_core::Interface::as_raw(self), callback.into_param().abi()).ok()
    }
    pub unsafe fn Resize(&self, newwidth: i32, newheight: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Resize)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(newwidth), ::core::mem::transmute(newheight)).ok()
    }
}
impl ::core::convert::From<IVirtualSurfaceImageSourceNative> for ::windows_core::IUnknown {
    fn from(value: IVirtualSurfaceImageSourceNative) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IVirtualSurfaceImageSourceNative> for ::windows_core::IUnknown {
    fn from(value: &IVirtualSurfaceImageSourceNative) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IVirtualSurfaceImageSourceNative {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IVirtualSurfaceImageSourceNative {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IVirtualSurfaceImageSourceNative> for ISurfaceImageSourceNative {
    fn from(value: IVirtualSurfaceImageSourceNative) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IVirtualSurfaceImageSourceNative> for ISurfaceImageSourceNative {
    fn from(value: &IVirtualSurfaceImageSourceNative) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ISurfaceImageSourceNative> for IVirtualSurfaceImageSourceNative {
    fn into_param(self) -> ::windows_core::Param<'a, ISurfaceImageSourceNative> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ISurfaceImageSourceNative> for &'a IVirtualSurfaceImageSourceNative {
    fn into_param(self) -> ::windows_core::Param<'a, ISurfaceImageSourceNative> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IVirtualSurfaceImageSourceNative {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IVirtualSurfaceImageSourceNative {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IVirtualSurfaceImageSourceNative {}
impl ::core::fmt::Debug for IVirtualSurfaceImageSourceNative {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IVirtualSurfaceImageSourceNative").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IVirtualSurfaceImageSourceNative {
    type Vtable = IVirtualSurfaceImageSourceNative_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe9550983_360b_4f53_b391_afd695078691);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVirtualSurfaceImageSourceNative_Vtbl {
    pub base__: ISurfaceImageSourceNative_Vtbl,
    pub Invalidate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, updaterect: ::win32_foundation::RECT) -> ::windows_core::HRESULT,
    pub GetUpdateRectCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32) -> ::windows_core::HRESULT,
    pub GetUpdateRects: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, updates: *mut ::win32_foundation::RECT, count: u32) -> ::windows_core::HRESULT,
    pub GetVisibleBounds: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bounds: *mut ::win32_foundation::RECT) -> ::windows_core::HRESULT,
    pub RegisterForUpdatesNeeded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, callback: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Resize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newwidth: i32, newheight: i32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IVirtualSurfaceUpdatesCallbackNative(::windows_core::IUnknown);
impl IVirtualSurfaceUpdatesCallbackNative {
    pub unsafe fn UpdatesNeeded(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).UpdatesNeeded)(::windows_core::Interface::as_raw(self)).ok()
    }
}
impl ::core::convert::From<IVirtualSurfaceUpdatesCallbackNative> for ::windows_core::IUnknown {
    fn from(value: IVirtualSurfaceUpdatesCallbackNative) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IVirtualSurfaceUpdatesCallbackNative> for ::windows_core::IUnknown {
    fn from(value: &IVirtualSurfaceUpdatesCallbackNative) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IVirtualSurfaceUpdatesCallbackNative {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IVirtualSurfaceUpdatesCallbackNative {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IVirtualSurfaceUpdatesCallbackNative {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IVirtualSurfaceUpdatesCallbackNative {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IVirtualSurfaceUpdatesCallbackNative {}
impl ::core::fmt::Debug for IVirtualSurfaceUpdatesCallbackNative {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IVirtualSurfaceUpdatesCallbackNative").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::Interface for IVirtualSurfaceUpdatesCallbackNative {
    type Vtable = IVirtualSurfaceUpdatesCallbackNative_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xdbf2e947_8e6c_4254_9eee_7738f71386c9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVirtualSurfaceUpdatesCallbackNative_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    pub UpdatesNeeded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(C)]
pub struct TrackerHandle__ {
    pub unused: i32,
}
impl ::core::marker::Copy for TrackerHandle__ {}
impl ::core::clone::Clone for TrackerHandle__ {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for TrackerHandle__ {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TrackerHandle__").field("unused", &self.unused).finish()
    }
}
unsafe impl ::windows_core::Abi for TrackerHandle__ {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for TrackerHandle__ {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<TrackerHandle__>()) == 0 }
    }
}
impl ::core::cmp::Eq for TrackerHandle__ {}
impl ::core::default::Default for TrackerHandle__ {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct XAML_REFERENCETRACKER_DISCONNECT(pub i32);
pub const XAML_REFERENCETRACKER_DISCONNECT_DEFAULT: XAML_REFERENCETRACKER_DISCONNECT = XAML_REFERENCETRACKER_DISCONNECT(0i32);
pub const XAML_REFERENCETRACKER_DISCONNECT_SUSPEND: XAML_REFERENCETRACKER_DISCONNECT = XAML_REFERENCETRACKER_DISCONNECT(1i32);
impl ::core::marker::Copy for XAML_REFERENCETRACKER_DISCONNECT {}
impl ::core::clone::Clone for XAML_REFERENCETRACKER_DISCONNECT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for XAML_REFERENCETRACKER_DISCONNECT {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for XAML_REFERENCETRACKER_DISCONNECT {
    type Abi = Self;
}
impl ::core::fmt::Debug for XAML_REFERENCETRACKER_DISCONNECT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("XAML_REFERENCETRACKER_DISCONNECT").field(&self.0).finish()
    }
}
