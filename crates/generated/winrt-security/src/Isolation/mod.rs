#[cfg(feature = "winrt-foundation")]
#[repr(transparent)]
pub struct HostMessageReceivedCallback(pub ::windows_core::IUnknown);
#[cfg(feature = "winrt-foundation")]
impl HostMessageReceivedCallback {
    pub fn new<F: FnMut(&::windows_core::GUID, &::core::option::Option<::winrt_foundation::Collections::IVectorView<::windows_core::IInspectable>>) -> ::windows_core::Result<()> + ::core::marker::Send + 'static>(invoke: F) -> Self {
        let com = HostMessageReceivedCallbackBox::<F> { vtable: &HostMessageReceivedCallbackBox::<F>::VTABLE, count: ::windows_core::RefCount::new(1), invoke };
        unsafe { ::core::mem::transmute(::windows_core::alloc::boxed::Box::new(com)) }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn Invoke<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::GUID>, Param1: ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IVectorView<::windows_core::IInspectable>>>(&self, receiverid: Param0, message: Param1) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Invoke)(::windows_core::Interface::as_raw(this), receiverid.into_param().abi(), message.into_param().abi()).ok() }
    }
}
#[cfg(feature = "winrt-foundation")]
#[repr(C)]
struct HostMessageReceivedCallbackBox<F: FnMut(&::windows_core::GUID, &::core::option::Option<::winrt_foundation::Collections::IVectorView<::windows_core::IInspectable>>) -> ::windows_core::Result<()> + ::core::marker::Send + 'static> {
    vtable: *const HostMessageReceivedCallback_Vtbl,
    invoke: F,
    count: ::windows_core::RefCount,
}
#[cfg(feature = "winrt-foundation")]
impl<F: FnMut(&::windows_core::GUID, &::core::option::Option<::winrt_foundation::Collections::IVectorView<::windows_core::IInspectable>>) -> ::windows_core::Result<()> + ::core::marker::Send + 'static> HostMessageReceivedCallbackBox<F> {
    const VTABLE: HostMessageReceivedCallback_Vtbl = HostMessageReceivedCallback_Vtbl {
        base__: ::windows_core::IUnknownVtbl { QueryInterface: Self::QueryInterface, AddRef: Self::AddRef, Release: Self::Release },
        Invoke: Self::Invoke,
    };
    unsafe extern "system" fn QueryInterface(this: ::windows_core::RawPtr, iid: &::windows_core::GUID, interface: *mut *const ::core::ffi::c_void) -> ::windows_core::HRESULT {
        let this = this as *mut ::windows_core::RawPtr as *mut Self;
        *interface = if iid == &<HostMessageReceivedCallback as ::windows_core::Interface>::IID || iid == &<::windows_core::IUnknown as ::windows_core::Interface>::IID || iid == &<::windows_core::IAgileObject as ::windows_core::Interface>::IID { &mut (*this).vtable as *mut _ as _ } else { ::core::ptr::null_mut() };
        if (*interface).is_null() {
            ::windows_core::HRESULT(-2147467262)
        } else {
            (*this).count.add_ref();
            ::windows_core::HRESULT(0)
        }
    }
    unsafe extern "system" fn AddRef(this: ::windows_core::RawPtr) -> u32 {
        let this = this as *mut ::windows_core::RawPtr as *mut Self;
        (*this).count.add_ref()
    }
    unsafe extern "system" fn Release(this: ::windows_core::RawPtr) -> u32 {
        let this = this as *mut ::windows_core::RawPtr as *mut Self;
        let remaining = (*this).count.release();
        if remaining == 0 {
            ::windows_core::alloc::boxed::Box::from_raw(this);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(this: *mut ::core::ffi::c_void, receiverid: ::windows_core::GUID, message: ::windows_core::RawPtr) -> ::windows_core::HRESULT {
        let this = this as *mut ::windows_core::RawPtr as *mut Self;
        ((*this).invoke)(::core::mem::transmute(&receiverid), ::core::mem::transmute(&message)).into()
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::clone::Clone for HostMessageReceivedCallback {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::cmp::PartialEq for HostMessageReceivedCallback {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::cmp::Eq for HostMessageReceivedCallback {}
#[cfg(feature = "winrt-foundation")]
impl ::core::fmt::Debug for HostMessageReceivedCallback {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HostMessageReceivedCallback").field(&self.0).finish()
    }
}
#[cfg(feature = "winrt-foundation")]
unsafe impl ::windows_core::Interface for HostMessageReceivedCallback {
    type Vtable = HostMessageReceivedCallback_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xfaf26ffa_8ce1_4cc1_b278_322d31a5e4a3);
}
#[cfg(feature = "winrt-foundation")]
unsafe impl ::windows_core::RuntimeType for HostMessageReceivedCallback {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"{faf26ffa-8ce1-4cc1-b278-322d31a5e4a3}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[cfg(feature = "winrt-foundation")]
#[repr(C)]
#[doc(hidden)]
pub struct HostMessageReceivedCallback_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    #[cfg(feature = "winrt-foundation")]
    pub Invoke: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, receiverid: ::windows_core::GUID, message: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    Invoke: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IIsolatedWindowsEnvironment(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IIsolatedWindowsEnvironment {
    type Vtable = IIsolatedWindowsEnvironment_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x41d24597_c328_4467_b37f_4dfc6f60b6bc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IIsolatedWindowsEnvironment_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub StartProcessSilentlyAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hostexepath: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, arguments: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, activator: IsolatedWindowsEnvironmentActivator, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub StartProcessSilentlyWithTelemetryAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hostexepath: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, arguments: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, activator: IsolatedWindowsEnvironmentActivator, telemetryparameters: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ShareFolderAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hostfolder: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, requestoptions: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ShareFolderWithTelemetryAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hostfolder: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, requestoptions: ::windows_core::RawPtr, telemetryparameters: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub LaunchFileWithUIAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, appexepath: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, argumentstemplate: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, filepath: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub LaunchFileWithUIAndTelemetryAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, appexepath: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, argumentstemplate: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, filepath: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, telemetryparameters: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub TerminateAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub TerminateWithTelemetryAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, telemetryparameters: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-foundation")]
    pub RegisterMessageReceiver: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, receiverid: ::windows_core::GUID, messagereceivedcallback: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    RegisterMessageReceiver: usize,
    pub UnregisterMessageReceiver: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, receiverid: ::windows_core::GUID) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IIsolatedWindowsEnvironment2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IIsolatedWindowsEnvironment2 {
    type Vtable = IIsolatedWindowsEnvironment2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2d365f39_88bd_4ab4_93cf_7e2bcef337c0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IIsolatedWindowsEnvironment2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-foundation")]
    pub PostMessageToReceiverAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, receiverid: ::windows_core::GUID, message: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    PostMessageToReceiverAsync: usize,
    #[cfg(feature = "winrt-foundation")]
    pub PostMessageToReceiverWithTelemetryAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, receiverid: ::windows_core::GUID, message: ::windows_core::RawPtr, telemetryparameters: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    PostMessageToReceiverWithTelemetryAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IIsolatedWindowsEnvironment3(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IIsolatedWindowsEnvironment3 {
    type Vtable = IIsolatedWindowsEnvironment3_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xcb7fc7d2_d06e_4c26_8ada_dacdaaad03f5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IIsolatedWindowsEnvironment3_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub GetUserInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ShareFileAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filepath: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, options: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub ShareFileWithTelemetryAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filepath: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, options: ::windows_core::RawPtr, telemetryparameters: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IIsolatedWindowsEnvironmentCreateResult(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IIsolatedWindowsEnvironmentCreateResult {
    type Vtable = IIsolatedWindowsEnvironmentCreateResult_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xef9a5e58_dcd7_45c2_9c85_ab642a715e8e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IIsolatedWindowsEnvironmentCreateResult_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut IsolatedWindowsEnvironmentCreateStatus) -> ::windows_core::HRESULT,
    pub ExtendedError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::HRESULT) -> ::windows_core::HRESULT,
    pub Environment: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IIsolatedWindowsEnvironmentFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IIsolatedWindowsEnvironmentFactory {
    type Vtable = IIsolatedWindowsEnvironmentFactory_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1aca93e7_e804_454d_8466_f9897c20b0f6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IIsolatedWindowsEnvironmentFactory_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CreateAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, options: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CreateWithTelemetryAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, options: ::windows_core::RawPtr, telemetryparameters: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetById: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, environmentid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-foundation")]
    pub FindByOwnerId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, environmentownerid: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    FindByOwnerId: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IIsolatedWindowsEnvironmentFile(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IIsolatedWindowsEnvironmentFile {
    type Vtable = IIsolatedWindowsEnvironmentFile_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4d5ae1ef_029f_4101_8c35_fe91bf9cd5f0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IIsolatedWindowsEnvironmentFile_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub HostPath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Close: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IIsolatedWindowsEnvironmentFile2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IIsolatedWindowsEnvironmentFile2 {
    type Vtable = IIsolatedWindowsEnvironmentFile2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4eeb8dec_ad5d_4b0a_b754_f36c3d46d684);
}
#[repr(C)]
#[doc(hidden)]
pub struct IIsolatedWindowsEnvironmentFile2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub GuestPath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub IsReadOnly: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IIsolatedWindowsEnvironmentHostStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IIsolatedWindowsEnvironmentHostStatics {
    type Vtable = IIsolatedWindowsEnvironmentHostStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2c0e22c7_05a0_517a_b81c_6ee8790c381f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IIsolatedWindowsEnvironmentHostStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub IsReady: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(feature = "winrt-foundation")]
    pub HostErrors: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    HostErrors: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IIsolatedWindowsEnvironmentLaunchFileResult(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IIsolatedWindowsEnvironmentLaunchFileResult {
    type Vtable = IIsolatedWindowsEnvironmentLaunchFileResult_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x685d4176_f6e0_4569_b1aa_215c0ff5b257);
}
#[repr(C)]
#[doc(hidden)]
pub struct IIsolatedWindowsEnvironmentLaunchFileResult_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut IsolatedWindowsEnvironmentLaunchFileStatus) -> ::windows_core::HRESULT,
    pub ExtendedError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::HRESULT) -> ::windows_core::HRESULT,
    pub File: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IIsolatedWindowsEnvironmentOptions(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IIsolatedWindowsEnvironmentOptions {
    type Vtable = IIsolatedWindowsEnvironmentOptions_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb71d98f7_61f0_4008_b207_0bf9eb2d76f2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IIsolatedWindowsEnvironmentOptions_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub EnvironmentOwnerId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetEnvironmentOwnerId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub AllowedClipboardFormats: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut IsolatedWindowsEnvironmentAllowedClipboardFormats) -> ::windows_core::HRESULT,
    pub SetAllowedClipboardFormats: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: IsolatedWindowsEnvironmentAllowedClipboardFormats) -> ::windows_core::HRESULT,
    pub ClipboardCopyPasteDirections: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut IsolatedWindowsEnvironmentClipboardCopyPasteDirections) -> ::windows_core::HRESULT,
    pub SetClipboardCopyPasteDirections: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: IsolatedWindowsEnvironmentClipboardCopyPasteDirections) -> ::windows_core::HRESULT,
    pub AvailablePrinters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut IsolatedWindowsEnvironmentAvailablePrinters) -> ::windows_core::HRESULT,
    pub SetAvailablePrinters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: IsolatedWindowsEnvironmentAvailablePrinters) -> ::windows_core::HRESULT,
    pub SharedHostFolderPath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SharedFolderNameInEnvironment: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub ShareHostFolderForUntrustedItems: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sharedhostfolderpath: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, sharefoldernameinenvironment: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub PersistUserProfile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetPersistUserProfile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub AllowGraphicsHardwareAcceleration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetAllowGraphicsHardwareAcceleration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub AllowCameraAndMicrophoneAccess: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetAllowCameraAndMicrophoneAccess: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IIsolatedWindowsEnvironmentOptions2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IIsolatedWindowsEnvironmentOptions2 {
    type Vtable = IIsolatedWindowsEnvironmentOptions2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x10d7cc31_8b8f_4b9d_b22c_617103b55b08);
}
#[repr(C)]
#[doc(hidden)]
pub struct IIsolatedWindowsEnvironmentOptions2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub WindowAnnotationOverride: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetWindowAnnotationOverride: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IIsolatedWindowsEnvironmentOwnerRegistrationData(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IIsolatedWindowsEnvironmentOwnerRegistrationData {
    type Vtable = IIsolatedWindowsEnvironmentOwnerRegistrationData_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf888ec22_e8cf_56c0_b1df_90af4ad80e84);
}
#[repr(C)]
#[doc(hidden)]
pub struct IIsolatedWindowsEnvironmentOwnerRegistrationData_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-foundation")]
    pub ShareableFolders: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    ShareableFolders: usize,
    #[cfg(feature = "winrt-foundation")]
    pub ProcessesRunnableAsSystem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    ProcessesRunnableAsSystem: usize,
    #[cfg(feature = "winrt-foundation")]
    pub ProcessesRunnableAsUser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    ProcessesRunnableAsUser: usize,
    #[cfg(feature = "winrt-foundation")]
    pub ActivationFileExtensions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    ActivationFileExtensions: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IIsolatedWindowsEnvironmentOwnerRegistrationResult(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IIsolatedWindowsEnvironmentOwnerRegistrationResult {
    type Vtable = IIsolatedWindowsEnvironmentOwnerRegistrationResult_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6dab9451_6169_55df_8f51_790e99d7277d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IIsolatedWindowsEnvironmentOwnerRegistrationResult_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut IsolatedWindowsEnvironmentOwnerRegistrationStatus) -> ::windows_core::HRESULT,
    pub ExtendedError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::HRESULT) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IIsolatedWindowsEnvironmentOwnerRegistrationStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IIsolatedWindowsEnvironmentOwnerRegistrationStatics {
    type Vtable = IIsolatedWindowsEnvironmentOwnerRegistrationStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x10951754_204b_5ec9_9de3_df792d074a61);
}
#[repr(C)]
#[doc(hidden)]
pub struct IIsolatedWindowsEnvironmentOwnerRegistrationStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Register: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ownername: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, ownerregistrationdata: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub Unregister: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ownername: ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IIsolatedWindowsEnvironmentPostMessageResult(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IIsolatedWindowsEnvironmentPostMessageResult {
    type Vtable = IIsolatedWindowsEnvironmentPostMessageResult_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0dfa28fa_2ef0_4d8f_b341_3171b2df93b1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IIsolatedWindowsEnvironmentPostMessageResult_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut IsolatedWindowsEnvironmentPostMessageStatus) -> ::windows_core::HRESULT,
    pub ExtendedError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::HRESULT) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IIsolatedWindowsEnvironmentProcess(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IIsolatedWindowsEnvironmentProcess {
    type Vtable = IIsolatedWindowsEnvironmentProcess_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa858c3ef_8172_4f10_af93_cbe60af88d09);
}
#[repr(C)]
#[doc(hidden)]
pub struct IIsolatedWindowsEnvironmentProcess_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub State: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut IsolatedWindowsEnvironmentProcessState) -> ::windows_core::HRESULT,
    pub ExitCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub WaitForExit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub WaitForExitWithTimeout: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, timeoutmilliseconds: u32) -> ::windows_core::HRESULT,
    pub WaitForExitAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IIsolatedWindowsEnvironmentShareFileRequestOptions(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IIsolatedWindowsEnvironmentShareFileRequestOptions {
    type Vtable = IIsolatedWindowsEnvironmentShareFileRequestOptions_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc9190ed8_0fd0_4946_bb88_117a60737b61);
}
#[repr(C)]
#[doc(hidden)]
pub struct IIsolatedWindowsEnvironmentShareFileRequestOptions_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub AllowWrite: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetAllowWrite: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IIsolatedWindowsEnvironmentShareFileResult(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IIsolatedWindowsEnvironmentShareFileResult {
    type Vtable = IIsolatedWindowsEnvironmentShareFileResult_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xaec7caa7_9ac6_4bf5_8b91_5c1adf0d7d00);
}
#[repr(C)]
#[doc(hidden)]
pub struct IIsolatedWindowsEnvironmentShareFileResult_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut IsolatedWindowsEnvironmentShareFileStatus) -> ::windows_core::HRESULT,
    pub ExtendedError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::HRESULT) -> ::windows_core::HRESULT,
    pub File: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IIsolatedWindowsEnvironmentShareFolderRequestOptions(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IIsolatedWindowsEnvironmentShareFolderRequestOptions {
    type Vtable = IIsolatedWindowsEnvironmentShareFolderRequestOptions_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc405eb7d_7053_4f6a_9b87_746846ed19b2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IIsolatedWindowsEnvironmentShareFolderRequestOptions_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub AllowWrite: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetAllowWrite: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IIsolatedWindowsEnvironmentShareFolderResult(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IIsolatedWindowsEnvironmentShareFolderResult {
    type Vtable = IIsolatedWindowsEnvironmentShareFolderResult_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x556ba72e_ca9d_4211_b143_1cedc86eb2fe);
}
#[repr(C)]
#[doc(hidden)]
pub struct IIsolatedWindowsEnvironmentShareFolderResult_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut IsolatedWindowsEnvironmentShareFolderStatus) -> ::windows_core::HRESULT,
    pub ExtendedError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::HRESULT) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IIsolatedWindowsEnvironmentStartProcessResult(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IIsolatedWindowsEnvironmentStartProcessResult {
    type Vtable = IIsolatedWindowsEnvironmentStartProcessResult_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8fa1dc2f_57da_4bb5_9c06_fa072d2032e2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IIsolatedWindowsEnvironmentStartProcessResult_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut IsolatedWindowsEnvironmentStartProcessStatus) -> ::windows_core::HRESULT,
    pub ExtendedError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::HRESULT) -> ::windows_core::HRESULT,
    pub Process: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IIsolatedWindowsEnvironmentTelemetryParameters(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IIsolatedWindowsEnvironmentTelemetryParameters {
    type Vtable = IIsolatedWindowsEnvironmentTelemetryParameters_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xebdb3cab_7a3a_4524_a0f4_f96e284d33cd);
}
#[repr(C)]
#[doc(hidden)]
pub struct IIsolatedWindowsEnvironmentTelemetryParameters_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CorrelationId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub SetCorrelationId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows_core::GUID) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IIsolatedWindowsEnvironmentUserInfo(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IIsolatedWindowsEnvironmentUserInfo {
    type Vtable = IIsolatedWindowsEnvironmentUserInfo_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8a9c75ae_69ba_4001_96fc_19a02703b340);
}
#[repr(C)]
#[doc(hidden)]
pub struct IIsolatedWindowsEnvironmentUserInfo_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub EnvironmentUserSid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub EnvironmentUserName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub TryWaitForSignInAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IIsolatedWindowsHostMessengerStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IIsolatedWindowsHostMessengerStatics {
    type Vtable = IIsolatedWindowsHostMessengerStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x06e444bb_53c0_4889_8fa3_53592e37cf21);
}
#[repr(C)]
#[doc(hidden)]
pub struct IIsolatedWindowsHostMessengerStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-foundation")]
    pub PostMessageToReceiver: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, receiverid: ::windows_core::GUID, message: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    PostMessageToReceiver: usize,
    pub GetFileId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filepath: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IIsolatedWindowsHostMessengerStatics2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IIsolatedWindowsHostMessengerStatics2 {
    type Vtable = IIsolatedWindowsHostMessengerStatics2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x55ef9ebc_0444_42ad_832d_1b89c089d1ca);
}
#[repr(C)]
#[doc(hidden)]
pub struct IIsolatedWindowsHostMessengerStatics2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    #[cfg(feature = "winrt-foundation")]
    pub RegisterHostMessageReceiver: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, receiverid: ::windows_core::GUID, hostmessagereceivedcallback: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    RegisterHostMessageReceiver: usize,
    pub UnregisterHostMessageReceiver: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, receiverid: ::windows_core::GUID) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IsolatedWindowsEnvironment(::windows_core::IUnknown);
impl IsolatedWindowsEnvironment {
    pub fn Id(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Id)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn StartProcessSilentlyAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, hostexepath: Param0, arguments: Param1, activator: IsolatedWindowsEnvironmentActivator) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<IsolatedWindowsEnvironmentStartProcessResult>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).StartProcessSilentlyAsync)(::windows_core::Interface::as_raw(this), hostexepath.into_param().abi(), arguments.into_param().abi(), activator, result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<IsolatedWindowsEnvironmentStartProcessResult>>(result__)
        }
    }
    pub fn StartProcessSilentlyWithTelemetryAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param3: ::windows_core::IntoParam<'a, IsolatedWindowsEnvironmentTelemetryParameters>>(&self, hostexepath: Param0, arguments: Param1, activator: IsolatedWindowsEnvironmentActivator, telemetryparameters: Param3) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<IsolatedWindowsEnvironmentStartProcessResult>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).StartProcessSilentlyWithTelemetryAsync)(::windows_core::Interface::as_raw(this), hostexepath.into_param().abi(), arguments.into_param().abi(), activator, telemetryparameters.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<IsolatedWindowsEnvironmentStartProcessResult>>(result__)
        }
    }
    pub fn ShareFolderAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, IsolatedWindowsEnvironmentShareFolderRequestOptions>>(&self, hostfolder: Param0, requestoptions: Param1) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<IsolatedWindowsEnvironmentShareFolderResult>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ShareFolderAsync)(::windows_core::Interface::as_raw(this), hostfolder.into_param().abi(), requestoptions.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<IsolatedWindowsEnvironmentShareFolderResult>>(result__)
        }
    }
    pub fn ShareFolderWithTelemetryAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, IsolatedWindowsEnvironmentShareFolderRequestOptions>, Param2: ::windows_core::IntoParam<'a, IsolatedWindowsEnvironmentTelemetryParameters>>(&self, hostfolder: Param0, requestoptions: Param1, telemetryparameters: Param2) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<IsolatedWindowsEnvironmentShareFolderResult>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ShareFolderWithTelemetryAsync)(::windows_core::Interface::as_raw(this), hostfolder.into_param().abi(), requestoptions.into_param().abi(), telemetryparameters.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<IsolatedWindowsEnvironmentShareFolderResult>>(result__)
        }
    }
    pub fn LaunchFileWithUIAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param2: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, appexepath: Param0, argumentstemplate: Param1, filepath: Param2) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<IsolatedWindowsEnvironmentLaunchFileResult>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).LaunchFileWithUIAsync)(::windows_core::Interface::as_raw(this), appexepath.into_param().abi(), argumentstemplate.into_param().abi(), filepath.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<IsolatedWindowsEnvironmentLaunchFileResult>>(result__)
        }
    }
    pub fn LaunchFileWithUIAndTelemetryAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param2: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param3: ::windows_core::IntoParam<'a, IsolatedWindowsEnvironmentTelemetryParameters>>(&self, appexepath: Param0, argumentstemplate: Param1, filepath: Param2, telemetryparameters: Param3) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<IsolatedWindowsEnvironmentLaunchFileResult>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).LaunchFileWithUIAndTelemetryAsync)(::windows_core::Interface::as_raw(this), appexepath.into_param().abi(), argumentstemplate.into_param().abi(), filepath.into_param().abi(), telemetryparameters.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<IsolatedWindowsEnvironmentLaunchFileResult>>(result__)
        }
    }
    pub fn TerminateAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TerminateAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncAction>(result__)
        }
    }
    pub fn TerminateWithTelemetryAsync<'a, Param0: ::windows_core::IntoParam<'a, IsolatedWindowsEnvironmentTelemetryParameters>>(&self, telemetryparameters: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TerminateWithTelemetryAsync)(::windows_core::Interface::as_raw(this), telemetryparameters.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn RegisterMessageReceiver<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::GUID>, Param1: ::windows_core::IntoParam<'a, MessageReceivedCallback>>(&self, receiverid: Param0, messagereceivedcallback: Param1) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RegisterMessageReceiver)(::windows_core::Interface::as_raw(this), receiverid.into_param().abi(), messagereceivedcallback.into_param().abi()).ok() }
    }
    pub fn UnregisterMessageReceiver<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::GUID>>(&self, receiverid: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).UnregisterMessageReceiver)(::windows_core::Interface::as_raw(this), receiverid.into_param().abi()).ok() }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn PostMessageToReceiverAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::GUID>, Param1: ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<::windows_core::IInspectable>>>(&self, receiverid: Param0, message: Param1) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<IsolatedWindowsEnvironmentPostMessageResult>> {
        let this = &::windows_core::Interface::cast::<IIsolatedWindowsEnvironment2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).PostMessageToReceiverAsync)(::windows_core::Interface::as_raw(this), receiverid.into_param().abi(), message.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<IsolatedWindowsEnvironmentPostMessageResult>>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn PostMessageToReceiverWithTelemetryAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::GUID>, Param1: ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<::windows_core::IInspectable>>, Param2: ::windows_core::IntoParam<'a, IsolatedWindowsEnvironmentTelemetryParameters>>(&self, receiverid: Param0, message: Param1, telemetryparameters: Param2) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<IsolatedWindowsEnvironmentPostMessageResult>> {
        let this = &::windows_core::Interface::cast::<IIsolatedWindowsEnvironment2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).PostMessageToReceiverWithTelemetryAsync)(::windows_core::Interface::as_raw(this), receiverid.into_param().abi(), message.into_param().abi(), telemetryparameters.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<IsolatedWindowsEnvironmentPostMessageResult>>(result__)
        }
    }
    pub fn GetUserInfo(&self) -> ::windows_core::Result<IsolatedWindowsEnvironmentUserInfo> {
        let this = &::windows_core::Interface::cast::<IIsolatedWindowsEnvironment3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetUserInfo)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<IsolatedWindowsEnvironmentUserInfo>(result__)
        }
    }
    pub fn ShareFileAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, IsolatedWindowsEnvironmentShareFileRequestOptions>>(&self, filepath: Param0, options: Param1) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<IsolatedWindowsEnvironmentShareFileResult>> {
        let this = &::windows_core::Interface::cast::<IIsolatedWindowsEnvironment3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ShareFileAsync)(::windows_core::Interface::as_raw(this), filepath.into_param().abi(), options.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<IsolatedWindowsEnvironmentShareFileResult>>(result__)
        }
    }
    pub fn ShareFileWithTelemetryAsync<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, IsolatedWindowsEnvironmentShareFileRequestOptions>, Param2: ::windows_core::IntoParam<'a, IsolatedWindowsEnvironmentTelemetryParameters>>(&self, filepath: Param0, options: Param1, telemetryparameters: Param2) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<IsolatedWindowsEnvironmentShareFileResult>> {
        let this = &::windows_core::Interface::cast::<IIsolatedWindowsEnvironment3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ShareFileWithTelemetryAsync)(::windows_core::Interface::as_raw(this), filepath.into_param().abi(), options.into_param().abi(), telemetryparameters.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<IsolatedWindowsEnvironmentShareFileResult>>(result__)
        }
    }
    pub fn CreateAsync<'a, Param0: ::windows_core::IntoParam<'a, IsolatedWindowsEnvironmentOptions>>(options: Param0) -> ::windows_core::Result<::winrt_foundation::IAsyncOperationWithProgress<IsolatedWindowsEnvironmentCreateResult, IsolatedWindowsEnvironmentCreateProgress>> {
        Self::IIsolatedWindowsEnvironmentFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateAsync)(::windows_core::Interface::as_raw(this), options.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperationWithProgress<IsolatedWindowsEnvironmentCreateResult, IsolatedWindowsEnvironmentCreateProgress>>(result__)
        })
    }
    pub fn CreateWithTelemetryAsync<'a, Param0: ::windows_core::IntoParam<'a, IsolatedWindowsEnvironmentOptions>, Param1: ::windows_core::IntoParam<'a, IsolatedWindowsEnvironmentTelemetryParameters>>(options: Param0, telemetryparameters: Param1) -> ::windows_core::Result<::winrt_foundation::IAsyncOperationWithProgress<IsolatedWindowsEnvironmentCreateResult, IsolatedWindowsEnvironmentCreateProgress>> {
        Self::IIsolatedWindowsEnvironmentFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateWithTelemetryAsync)(::windows_core::Interface::as_raw(this), options.into_param().abi(), telemetryparameters.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperationWithProgress<IsolatedWindowsEnvironmentCreateResult, IsolatedWindowsEnvironmentCreateProgress>>(result__)
        })
    }
    pub fn GetById<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(environmentid: Param0) -> ::windows_core::Result<IsolatedWindowsEnvironment> {
        Self::IIsolatedWindowsEnvironmentFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetById)(::windows_core::Interface::as_raw(this), environmentid.into_param().abi(), result__.as_mut_ptr()).from_abi::<IsolatedWindowsEnvironment>(result__)
        })
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn FindByOwnerId<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(environmentownerid: Param0) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<IsolatedWindowsEnvironment>> {
        Self::IIsolatedWindowsEnvironmentFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).FindByOwnerId)(::windows_core::Interface::as_raw(this), environmentownerid.into_param().abi(), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<IsolatedWindowsEnvironment>>(result__)
        })
    }
    pub fn IIsolatedWindowsEnvironmentFactory<R, F: FnOnce(&IIsolatedWindowsEnvironmentFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<IsolatedWindowsEnvironment, IIsolatedWindowsEnvironmentFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for IsolatedWindowsEnvironment {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IsolatedWindowsEnvironment {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IsolatedWindowsEnvironment {}
impl ::core::fmt::Debug for IsolatedWindowsEnvironment {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IsolatedWindowsEnvironment").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for IsolatedWindowsEnvironment {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Security.Isolation.IsolatedWindowsEnvironment;{41d24597-c328-4467-b37f-4dfc6f60b6bc})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for IsolatedWindowsEnvironment {
    type Vtable = IIsolatedWindowsEnvironment_Vtbl;
    const IID: ::windows_core::GUID = <IIsolatedWindowsEnvironment as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for IsolatedWindowsEnvironment {
    const NAME: &'static str = "Windows.Security.Isolation.IsolatedWindowsEnvironment";
}
impl ::core::convert::From<IsolatedWindowsEnvironment> for ::windows_core::IUnknown {
    fn from(value: IsolatedWindowsEnvironment) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IsolatedWindowsEnvironment> for ::windows_core::IUnknown {
    fn from(value: &IsolatedWindowsEnvironment) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IsolatedWindowsEnvironment {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IsolatedWindowsEnvironment {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IsolatedWindowsEnvironment> for ::windows_core::IInspectable {
    fn from(value: IsolatedWindowsEnvironment) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IsolatedWindowsEnvironment> for ::windows_core::IInspectable {
    fn from(value: &IsolatedWindowsEnvironment) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for IsolatedWindowsEnvironment {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a IsolatedWindowsEnvironment {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for IsolatedWindowsEnvironment {}
unsafe impl ::core::marker::Sync for IsolatedWindowsEnvironment {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct IsolatedWindowsEnvironmentActivator(pub i32);
impl IsolatedWindowsEnvironmentActivator {
    pub const System: Self = Self(0i32);
    pub const User: Self = Self(1i32);
}
impl ::core::marker::Copy for IsolatedWindowsEnvironmentActivator {}
impl ::core::clone::Clone for IsolatedWindowsEnvironmentActivator {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for IsolatedWindowsEnvironmentActivator {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for IsolatedWindowsEnvironmentActivator {
    type Abi = Self;
}
impl ::core::fmt::Debug for IsolatedWindowsEnvironmentActivator {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IsolatedWindowsEnvironmentActivator").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for IsolatedWindowsEnvironmentActivator {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Security.Isolation.IsolatedWindowsEnvironmentActivator;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct IsolatedWindowsEnvironmentAllowedClipboardFormats(pub u32);
impl IsolatedWindowsEnvironmentAllowedClipboardFormats {
    pub const None: Self = Self(0u32);
    pub const Text: Self = Self(1u32);
    pub const Image: Self = Self(2u32);
}
impl ::core::marker::Copy for IsolatedWindowsEnvironmentAllowedClipboardFormats {}
impl ::core::clone::Clone for IsolatedWindowsEnvironmentAllowedClipboardFormats {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for IsolatedWindowsEnvironmentAllowedClipboardFormats {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for IsolatedWindowsEnvironmentAllowedClipboardFormats {
    type Abi = Self;
}
impl ::core::fmt::Debug for IsolatedWindowsEnvironmentAllowedClipboardFormats {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IsolatedWindowsEnvironmentAllowedClipboardFormats").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for IsolatedWindowsEnvironmentAllowedClipboardFormats {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for IsolatedWindowsEnvironmentAllowedClipboardFormats {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for IsolatedWindowsEnvironmentAllowedClipboardFormats {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for IsolatedWindowsEnvironmentAllowedClipboardFormats {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for IsolatedWindowsEnvironmentAllowedClipboardFormats {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
unsafe impl ::windows_core::RuntimeType for IsolatedWindowsEnvironmentAllowedClipboardFormats {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Security.Isolation.IsolatedWindowsEnvironmentAllowedClipboardFormats;u4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct IsolatedWindowsEnvironmentAvailablePrinters(pub u32);
impl IsolatedWindowsEnvironmentAvailablePrinters {
    pub const None: Self = Self(0u32);
    pub const Local: Self = Self(1u32);
    pub const Network: Self = Self(2u32);
    pub const SystemPrintToPdf: Self = Self(4u32);
    pub const SystemPrintToXps: Self = Self(8u32);
}
impl ::core::marker::Copy for IsolatedWindowsEnvironmentAvailablePrinters {}
impl ::core::clone::Clone for IsolatedWindowsEnvironmentAvailablePrinters {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for IsolatedWindowsEnvironmentAvailablePrinters {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for IsolatedWindowsEnvironmentAvailablePrinters {
    type Abi = Self;
}
impl ::core::fmt::Debug for IsolatedWindowsEnvironmentAvailablePrinters {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IsolatedWindowsEnvironmentAvailablePrinters").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for IsolatedWindowsEnvironmentAvailablePrinters {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for IsolatedWindowsEnvironmentAvailablePrinters {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for IsolatedWindowsEnvironmentAvailablePrinters {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for IsolatedWindowsEnvironmentAvailablePrinters {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for IsolatedWindowsEnvironmentAvailablePrinters {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
unsafe impl ::windows_core::RuntimeType for IsolatedWindowsEnvironmentAvailablePrinters {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Security.Isolation.IsolatedWindowsEnvironmentAvailablePrinters;u4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct IsolatedWindowsEnvironmentClipboardCopyPasteDirections(pub u32);
impl IsolatedWindowsEnvironmentClipboardCopyPasteDirections {
    pub const None: Self = Self(0u32);
    pub const HostToIsolatedWindowsEnvironment: Self = Self(1u32);
    pub const IsolatedWindowsEnvironmentToHost: Self = Self(2u32);
}
impl ::core::marker::Copy for IsolatedWindowsEnvironmentClipboardCopyPasteDirections {}
impl ::core::clone::Clone for IsolatedWindowsEnvironmentClipboardCopyPasteDirections {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for IsolatedWindowsEnvironmentClipboardCopyPasteDirections {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for IsolatedWindowsEnvironmentClipboardCopyPasteDirections {
    type Abi = Self;
}
impl ::core::fmt::Debug for IsolatedWindowsEnvironmentClipboardCopyPasteDirections {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IsolatedWindowsEnvironmentClipboardCopyPasteDirections").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for IsolatedWindowsEnvironmentClipboardCopyPasteDirections {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for IsolatedWindowsEnvironmentClipboardCopyPasteDirections {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for IsolatedWindowsEnvironmentClipboardCopyPasteDirections {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for IsolatedWindowsEnvironmentClipboardCopyPasteDirections {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for IsolatedWindowsEnvironmentClipboardCopyPasteDirections {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
unsafe impl ::windows_core::RuntimeType for IsolatedWindowsEnvironmentClipboardCopyPasteDirections {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Security.Isolation.IsolatedWindowsEnvironmentClipboardCopyPasteDirections;u4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(C)]
pub struct IsolatedWindowsEnvironmentCreateProgress {
    pub State: IsolatedWindowsEnvironmentProgressState,
    pub PercentComplete: u32,
}
impl ::core::marker::Copy for IsolatedWindowsEnvironmentCreateProgress {}
impl ::core::clone::Clone for IsolatedWindowsEnvironmentCreateProgress {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for IsolatedWindowsEnvironmentCreateProgress {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IsolatedWindowsEnvironmentCreateProgress").field("State", &self.State).field("PercentComplete", &self.PercentComplete).finish()
    }
}
unsafe impl ::windows_core::Abi for IsolatedWindowsEnvironmentCreateProgress {
    type Abi = Self;
}
unsafe impl ::windows_core::RuntimeType for IsolatedWindowsEnvironmentCreateProgress {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"struct(Windows.Security.Isolation.IsolatedWindowsEnvironmentCreateProgress;enum(Windows.Security.Isolation.IsolatedWindowsEnvironmentProgressState;i4);u4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
impl ::core::cmp::PartialEq for IsolatedWindowsEnvironmentCreateProgress {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IsolatedWindowsEnvironmentCreateProgress>()) == 0 }
    }
}
impl ::core::cmp::Eq for IsolatedWindowsEnvironmentCreateProgress {}
impl ::core::default::Default for IsolatedWindowsEnvironmentCreateProgress {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
pub struct IsolatedWindowsEnvironmentCreateResult(::windows_core::IUnknown);
impl IsolatedWindowsEnvironmentCreateResult {
    pub fn Status(&self) -> ::windows_core::Result<IsolatedWindowsEnvironmentCreateStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<IsolatedWindowsEnvironmentCreateStatus>::zeroed();
            (::windows_core::Interface::vtable(this).Status)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<IsolatedWindowsEnvironmentCreateStatus>(result__)
        }
    }
    pub fn ExtendedError(&self) -> ::windows_core::Result<::windows_core::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::HRESULT>::zeroed();
            (::windows_core::Interface::vtable(this).ExtendedError)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HRESULT>(result__)
        }
    }
    pub fn Environment(&self) -> ::windows_core::Result<IsolatedWindowsEnvironment> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Environment)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<IsolatedWindowsEnvironment>(result__)
        }
    }
}
impl ::core::clone::Clone for IsolatedWindowsEnvironmentCreateResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IsolatedWindowsEnvironmentCreateResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IsolatedWindowsEnvironmentCreateResult {}
impl ::core::fmt::Debug for IsolatedWindowsEnvironmentCreateResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IsolatedWindowsEnvironmentCreateResult").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for IsolatedWindowsEnvironmentCreateResult {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Security.Isolation.IsolatedWindowsEnvironmentCreateResult;{ef9a5e58-dcd7-45c2-9c85-ab642a715e8e})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for IsolatedWindowsEnvironmentCreateResult {
    type Vtable = IIsolatedWindowsEnvironmentCreateResult_Vtbl;
    const IID: ::windows_core::GUID = <IIsolatedWindowsEnvironmentCreateResult as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for IsolatedWindowsEnvironmentCreateResult {
    const NAME: &'static str = "Windows.Security.Isolation.IsolatedWindowsEnvironmentCreateResult";
}
impl ::core::convert::From<IsolatedWindowsEnvironmentCreateResult> for ::windows_core::IUnknown {
    fn from(value: IsolatedWindowsEnvironmentCreateResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IsolatedWindowsEnvironmentCreateResult> for ::windows_core::IUnknown {
    fn from(value: &IsolatedWindowsEnvironmentCreateResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IsolatedWindowsEnvironmentCreateResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IsolatedWindowsEnvironmentCreateResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IsolatedWindowsEnvironmentCreateResult> for ::windows_core::IInspectable {
    fn from(value: IsolatedWindowsEnvironmentCreateResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IsolatedWindowsEnvironmentCreateResult> for ::windows_core::IInspectable {
    fn from(value: &IsolatedWindowsEnvironmentCreateResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for IsolatedWindowsEnvironmentCreateResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a IsolatedWindowsEnvironmentCreateResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for IsolatedWindowsEnvironmentCreateResult {}
unsafe impl ::core::marker::Sync for IsolatedWindowsEnvironmentCreateResult {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct IsolatedWindowsEnvironmentCreateStatus(pub i32);
impl IsolatedWindowsEnvironmentCreateStatus {
    pub const Success: Self = Self(0i32);
    pub const FailureByPolicy: Self = Self(1i32);
    pub const UnknownFailure: Self = Self(2i32);
}
impl ::core::marker::Copy for IsolatedWindowsEnvironmentCreateStatus {}
impl ::core::clone::Clone for IsolatedWindowsEnvironmentCreateStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for IsolatedWindowsEnvironmentCreateStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for IsolatedWindowsEnvironmentCreateStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for IsolatedWindowsEnvironmentCreateStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IsolatedWindowsEnvironmentCreateStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for IsolatedWindowsEnvironmentCreateStatus {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Security.Isolation.IsolatedWindowsEnvironmentCreateStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct IsolatedWindowsEnvironmentFile(::windows_core::IUnknown);
impl IsolatedWindowsEnvironmentFile {
    pub fn Id(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::GUID>::zeroed();
            (::windows_core::Interface::vtable(this).Id)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::GUID>(result__)
        }
    }
    pub fn HostPath(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).HostPath)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn GuestPath(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IIsolatedWindowsEnvironmentFile2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).GuestPath)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn IsReadOnly(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IIsolatedWindowsEnvironmentFile2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsReadOnly)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
}
impl ::core::clone::Clone for IsolatedWindowsEnvironmentFile {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IsolatedWindowsEnvironmentFile {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IsolatedWindowsEnvironmentFile {}
impl ::core::fmt::Debug for IsolatedWindowsEnvironmentFile {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IsolatedWindowsEnvironmentFile").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for IsolatedWindowsEnvironmentFile {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Security.Isolation.IsolatedWindowsEnvironmentFile;{4d5ae1ef-029f-4101-8c35-fe91bf9cd5f0})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for IsolatedWindowsEnvironmentFile {
    type Vtable = IIsolatedWindowsEnvironmentFile_Vtbl;
    const IID: ::windows_core::GUID = <IIsolatedWindowsEnvironmentFile as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for IsolatedWindowsEnvironmentFile {
    const NAME: &'static str = "Windows.Security.Isolation.IsolatedWindowsEnvironmentFile";
}
impl ::core::convert::From<IsolatedWindowsEnvironmentFile> for ::windows_core::IUnknown {
    fn from(value: IsolatedWindowsEnvironmentFile) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IsolatedWindowsEnvironmentFile> for ::windows_core::IUnknown {
    fn from(value: &IsolatedWindowsEnvironmentFile) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IsolatedWindowsEnvironmentFile {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IsolatedWindowsEnvironmentFile {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IsolatedWindowsEnvironmentFile> for ::windows_core::IInspectable {
    fn from(value: IsolatedWindowsEnvironmentFile) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IsolatedWindowsEnvironmentFile> for ::windows_core::IInspectable {
    fn from(value: &IsolatedWindowsEnvironmentFile) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for IsolatedWindowsEnvironmentFile {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a IsolatedWindowsEnvironmentFile {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for IsolatedWindowsEnvironmentFile {}
unsafe impl ::core::marker::Sync for IsolatedWindowsEnvironmentFile {}
pub struct IsolatedWindowsEnvironmentHost;
impl IsolatedWindowsEnvironmentHost {
    pub fn IsReady() -> ::windows_core::Result<bool> {
        Self::IIsolatedWindowsEnvironmentHostStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IsReady)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        })
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn HostErrors() -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<IsolatedWindowsEnvironmentHostError>> {
        Self::IIsolatedWindowsEnvironmentHostStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).HostErrors)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<IsolatedWindowsEnvironmentHostError>>(result__)
        })
    }
    pub fn IIsolatedWindowsEnvironmentHostStatics<R, F: FnOnce(&IIsolatedWindowsEnvironmentHostStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<IsolatedWindowsEnvironmentHost, IIsolatedWindowsEnvironmentHostStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows_core::RuntimeName for IsolatedWindowsEnvironmentHost {
    const NAME: &'static str = "Windows.Security.Isolation.IsolatedWindowsEnvironmentHost";
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct IsolatedWindowsEnvironmentHostError(pub i32);
impl IsolatedWindowsEnvironmentHostError {
    pub const AdminPolicyIsDisabledOrNotPresent: Self = Self(0i32);
    pub const FeatureNotInstalled: Self = Self(1i32);
    pub const HardwareRequirementsNotMet: Self = Self(2i32);
    pub const RebootRequired: Self = Self(3i32);
    pub const UnknownError: Self = Self(4i32);
}
impl ::core::marker::Copy for IsolatedWindowsEnvironmentHostError {}
impl ::core::clone::Clone for IsolatedWindowsEnvironmentHostError {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for IsolatedWindowsEnvironmentHostError {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for IsolatedWindowsEnvironmentHostError {
    type Abi = Self;
}
impl ::core::fmt::Debug for IsolatedWindowsEnvironmentHostError {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IsolatedWindowsEnvironmentHostError").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for IsolatedWindowsEnvironmentHostError {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Security.Isolation.IsolatedWindowsEnvironmentHostError;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct IsolatedWindowsEnvironmentLaunchFileResult(::windows_core::IUnknown);
impl IsolatedWindowsEnvironmentLaunchFileResult {
    pub fn Status(&self) -> ::windows_core::Result<IsolatedWindowsEnvironmentLaunchFileStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<IsolatedWindowsEnvironmentLaunchFileStatus>::zeroed();
            (::windows_core::Interface::vtable(this).Status)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<IsolatedWindowsEnvironmentLaunchFileStatus>(result__)
        }
    }
    pub fn ExtendedError(&self) -> ::windows_core::Result<::windows_core::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::HRESULT>::zeroed();
            (::windows_core::Interface::vtable(this).ExtendedError)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HRESULT>(result__)
        }
    }
    pub fn File(&self) -> ::windows_core::Result<IsolatedWindowsEnvironmentFile> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).File)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<IsolatedWindowsEnvironmentFile>(result__)
        }
    }
}
impl ::core::clone::Clone for IsolatedWindowsEnvironmentLaunchFileResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IsolatedWindowsEnvironmentLaunchFileResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IsolatedWindowsEnvironmentLaunchFileResult {}
impl ::core::fmt::Debug for IsolatedWindowsEnvironmentLaunchFileResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IsolatedWindowsEnvironmentLaunchFileResult").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for IsolatedWindowsEnvironmentLaunchFileResult {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Security.Isolation.IsolatedWindowsEnvironmentLaunchFileResult;{685d4176-f6e0-4569-b1aa-215c0ff5b257})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for IsolatedWindowsEnvironmentLaunchFileResult {
    type Vtable = IIsolatedWindowsEnvironmentLaunchFileResult_Vtbl;
    const IID: ::windows_core::GUID = <IIsolatedWindowsEnvironmentLaunchFileResult as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for IsolatedWindowsEnvironmentLaunchFileResult {
    const NAME: &'static str = "Windows.Security.Isolation.IsolatedWindowsEnvironmentLaunchFileResult";
}
impl ::core::convert::From<IsolatedWindowsEnvironmentLaunchFileResult> for ::windows_core::IUnknown {
    fn from(value: IsolatedWindowsEnvironmentLaunchFileResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IsolatedWindowsEnvironmentLaunchFileResult> for ::windows_core::IUnknown {
    fn from(value: &IsolatedWindowsEnvironmentLaunchFileResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IsolatedWindowsEnvironmentLaunchFileResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IsolatedWindowsEnvironmentLaunchFileResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IsolatedWindowsEnvironmentLaunchFileResult> for ::windows_core::IInspectable {
    fn from(value: IsolatedWindowsEnvironmentLaunchFileResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IsolatedWindowsEnvironmentLaunchFileResult> for ::windows_core::IInspectable {
    fn from(value: &IsolatedWindowsEnvironmentLaunchFileResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for IsolatedWindowsEnvironmentLaunchFileResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a IsolatedWindowsEnvironmentLaunchFileResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for IsolatedWindowsEnvironmentLaunchFileResult {}
unsafe impl ::core::marker::Sync for IsolatedWindowsEnvironmentLaunchFileResult {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct IsolatedWindowsEnvironmentLaunchFileStatus(pub i32);
impl IsolatedWindowsEnvironmentLaunchFileStatus {
    pub const Success: Self = Self(0i32);
    pub const UnknownFailure: Self = Self(1i32);
    pub const EnvironmentUnavailable: Self = Self(2i32);
    pub const FileNotFound: Self = Self(3i32);
    pub const TimedOut: Self = Self(4i32);
    pub const AlreadySharedWithConflictingOptions: Self = Self(5i32);
}
impl ::core::marker::Copy for IsolatedWindowsEnvironmentLaunchFileStatus {}
impl ::core::clone::Clone for IsolatedWindowsEnvironmentLaunchFileStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for IsolatedWindowsEnvironmentLaunchFileStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for IsolatedWindowsEnvironmentLaunchFileStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for IsolatedWindowsEnvironmentLaunchFileStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IsolatedWindowsEnvironmentLaunchFileStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for IsolatedWindowsEnvironmentLaunchFileStatus {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Security.Isolation.IsolatedWindowsEnvironmentLaunchFileStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct IsolatedWindowsEnvironmentOptions(::windows_core::IUnknown);
impl IsolatedWindowsEnvironmentOptions {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<IsolatedWindowsEnvironmentOptions, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn EnvironmentOwnerId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).EnvironmentOwnerId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetEnvironmentOwnerId<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetEnvironmentOwnerId)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn AllowedClipboardFormats(&self) -> ::windows_core::Result<IsolatedWindowsEnvironmentAllowedClipboardFormats> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<IsolatedWindowsEnvironmentAllowedClipboardFormats>::zeroed();
            (::windows_core::Interface::vtable(this).AllowedClipboardFormats)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<IsolatedWindowsEnvironmentAllowedClipboardFormats>(result__)
        }
    }
    pub fn SetAllowedClipboardFormats(&self, value: IsolatedWindowsEnvironmentAllowedClipboardFormats) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetAllowedClipboardFormats)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ClipboardCopyPasteDirections(&self) -> ::windows_core::Result<IsolatedWindowsEnvironmentClipboardCopyPasteDirections> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<IsolatedWindowsEnvironmentClipboardCopyPasteDirections>::zeroed();
            (::windows_core::Interface::vtable(this).ClipboardCopyPasteDirections)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<IsolatedWindowsEnvironmentClipboardCopyPasteDirections>(result__)
        }
    }
    pub fn SetClipboardCopyPasteDirections(&self, value: IsolatedWindowsEnvironmentClipboardCopyPasteDirections) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetClipboardCopyPasteDirections)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn AvailablePrinters(&self) -> ::windows_core::Result<IsolatedWindowsEnvironmentAvailablePrinters> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<IsolatedWindowsEnvironmentAvailablePrinters>::zeroed();
            (::windows_core::Interface::vtable(this).AvailablePrinters)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<IsolatedWindowsEnvironmentAvailablePrinters>(result__)
        }
    }
    pub fn SetAvailablePrinters(&self, value: IsolatedWindowsEnvironmentAvailablePrinters) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetAvailablePrinters)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn SharedHostFolderPath(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).SharedHostFolderPath)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SharedFolderNameInEnvironment(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).SharedFolderNameInEnvironment)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn ShareHostFolderForUntrustedItems<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, sharedhostfolderpath: Param0, sharefoldernameinenvironment: Param1) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).ShareHostFolderForUntrustedItems)(::windows_core::Interface::as_raw(this), sharedhostfolderpath.into_param().abi(), sharefoldernameinenvironment.into_param().abi()).ok() }
    }
    pub fn PersistUserProfile(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).PersistUserProfile)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetPersistUserProfile(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetPersistUserProfile)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn AllowGraphicsHardwareAcceleration(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).AllowGraphicsHardwareAcceleration)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetAllowGraphicsHardwareAcceleration(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetAllowGraphicsHardwareAcceleration)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn AllowCameraAndMicrophoneAccess(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).AllowCameraAndMicrophoneAccess)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetAllowCameraAndMicrophoneAccess(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetAllowCameraAndMicrophoneAccess)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn WindowAnnotationOverride(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IIsolatedWindowsEnvironmentOptions2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).WindowAnnotationOverride)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn SetWindowAnnotationOverride<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<IIsolatedWindowsEnvironmentOptions2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetWindowAnnotationOverride)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
}
impl ::core::clone::Clone for IsolatedWindowsEnvironmentOptions {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IsolatedWindowsEnvironmentOptions {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IsolatedWindowsEnvironmentOptions {}
impl ::core::fmt::Debug for IsolatedWindowsEnvironmentOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IsolatedWindowsEnvironmentOptions").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for IsolatedWindowsEnvironmentOptions {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Security.Isolation.IsolatedWindowsEnvironmentOptions;{b71d98f7-61f0-4008-b207-0bf9eb2d76f2})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for IsolatedWindowsEnvironmentOptions {
    type Vtable = IIsolatedWindowsEnvironmentOptions_Vtbl;
    const IID: ::windows_core::GUID = <IIsolatedWindowsEnvironmentOptions as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for IsolatedWindowsEnvironmentOptions {
    const NAME: &'static str = "Windows.Security.Isolation.IsolatedWindowsEnvironmentOptions";
}
impl ::core::convert::From<IsolatedWindowsEnvironmentOptions> for ::windows_core::IUnknown {
    fn from(value: IsolatedWindowsEnvironmentOptions) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IsolatedWindowsEnvironmentOptions> for ::windows_core::IUnknown {
    fn from(value: &IsolatedWindowsEnvironmentOptions) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IsolatedWindowsEnvironmentOptions {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IsolatedWindowsEnvironmentOptions {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IsolatedWindowsEnvironmentOptions> for ::windows_core::IInspectable {
    fn from(value: IsolatedWindowsEnvironmentOptions) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IsolatedWindowsEnvironmentOptions> for ::windows_core::IInspectable {
    fn from(value: &IsolatedWindowsEnvironmentOptions) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for IsolatedWindowsEnvironmentOptions {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a IsolatedWindowsEnvironmentOptions {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for IsolatedWindowsEnvironmentOptions {}
unsafe impl ::core::marker::Sync for IsolatedWindowsEnvironmentOptions {}
pub struct IsolatedWindowsEnvironmentOwnerRegistration;
impl IsolatedWindowsEnvironmentOwnerRegistration {
    pub fn Register<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, IsolatedWindowsEnvironmentOwnerRegistrationData>>(ownername: Param0, ownerregistrationdata: Param1) -> ::windows_core::Result<IsolatedWindowsEnvironmentOwnerRegistrationResult> {
        Self::IIsolatedWindowsEnvironmentOwnerRegistrationStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Register)(::windows_core::Interface::as_raw(this), ownername.into_param().abi(), ownerregistrationdata.into_param().abi(), result__.as_mut_ptr()).from_abi::<IsolatedWindowsEnvironmentOwnerRegistrationResult>(result__)
        })
    }
    pub fn Unregister<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(ownername: Param0) -> ::windows_core::Result<()> {
        Self::IIsolatedWindowsEnvironmentOwnerRegistrationStatics(|this| unsafe { (::windows_core::Interface::vtable(this).Unregister)(::windows_core::Interface::as_raw(this), ownername.into_param().abi()).ok() })
    }
    pub fn IIsolatedWindowsEnvironmentOwnerRegistrationStatics<R, F: FnOnce(&IIsolatedWindowsEnvironmentOwnerRegistrationStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<IsolatedWindowsEnvironmentOwnerRegistration, IIsolatedWindowsEnvironmentOwnerRegistrationStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows_core::RuntimeName for IsolatedWindowsEnvironmentOwnerRegistration {
    const NAME: &'static str = "Windows.Security.Isolation.IsolatedWindowsEnvironmentOwnerRegistration";
}
#[repr(transparent)]
pub struct IsolatedWindowsEnvironmentOwnerRegistrationData(::windows_core::IUnknown);
impl IsolatedWindowsEnvironmentOwnerRegistrationData {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<IsolatedWindowsEnvironmentOwnerRegistrationData, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn ShareableFolders(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVector<::windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ShareableFolders)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVector<::windows_core::HSTRING>>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn ProcessesRunnableAsSystem(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVector<::windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ProcessesRunnableAsSystem)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVector<::windows_core::HSTRING>>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn ProcessesRunnableAsUser(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVector<::windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ProcessesRunnableAsUser)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVector<::windows_core::HSTRING>>(result__)
        }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn ActivationFileExtensions(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVector<::windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).ActivationFileExtensions)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVector<::windows_core::HSTRING>>(result__)
        }
    }
}
impl ::core::clone::Clone for IsolatedWindowsEnvironmentOwnerRegistrationData {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IsolatedWindowsEnvironmentOwnerRegistrationData {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IsolatedWindowsEnvironmentOwnerRegistrationData {}
impl ::core::fmt::Debug for IsolatedWindowsEnvironmentOwnerRegistrationData {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IsolatedWindowsEnvironmentOwnerRegistrationData").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for IsolatedWindowsEnvironmentOwnerRegistrationData {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Security.Isolation.IsolatedWindowsEnvironmentOwnerRegistrationData;{f888ec22-e8cf-56c0-b1df-90af4ad80e84})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for IsolatedWindowsEnvironmentOwnerRegistrationData {
    type Vtable = IIsolatedWindowsEnvironmentOwnerRegistrationData_Vtbl;
    const IID: ::windows_core::GUID = <IIsolatedWindowsEnvironmentOwnerRegistrationData as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for IsolatedWindowsEnvironmentOwnerRegistrationData {
    const NAME: &'static str = "Windows.Security.Isolation.IsolatedWindowsEnvironmentOwnerRegistrationData";
}
impl ::core::convert::From<IsolatedWindowsEnvironmentOwnerRegistrationData> for ::windows_core::IUnknown {
    fn from(value: IsolatedWindowsEnvironmentOwnerRegistrationData) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IsolatedWindowsEnvironmentOwnerRegistrationData> for ::windows_core::IUnknown {
    fn from(value: &IsolatedWindowsEnvironmentOwnerRegistrationData) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IsolatedWindowsEnvironmentOwnerRegistrationData {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IsolatedWindowsEnvironmentOwnerRegistrationData {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IsolatedWindowsEnvironmentOwnerRegistrationData> for ::windows_core::IInspectable {
    fn from(value: IsolatedWindowsEnvironmentOwnerRegistrationData) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IsolatedWindowsEnvironmentOwnerRegistrationData> for ::windows_core::IInspectable {
    fn from(value: &IsolatedWindowsEnvironmentOwnerRegistrationData) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for IsolatedWindowsEnvironmentOwnerRegistrationData {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a IsolatedWindowsEnvironmentOwnerRegistrationData {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for IsolatedWindowsEnvironmentOwnerRegistrationData {}
unsafe impl ::core::marker::Sync for IsolatedWindowsEnvironmentOwnerRegistrationData {}
#[repr(transparent)]
pub struct IsolatedWindowsEnvironmentOwnerRegistrationResult(::windows_core::IUnknown);
impl IsolatedWindowsEnvironmentOwnerRegistrationResult {
    pub fn Status(&self) -> ::windows_core::Result<IsolatedWindowsEnvironmentOwnerRegistrationStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<IsolatedWindowsEnvironmentOwnerRegistrationStatus>::zeroed();
            (::windows_core::Interface::vtable(this).Status)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<IsolatedWindowsEnvironmentOwnerRegistrationStatus>(result__)
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
impl ::core::clone::Clone for IsolatedWindowsEnvironmentOwnerRegistrationResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IsolatedWindowsEnvironmentOwnerRegistrationResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IsolatedWindowsEnvironmentOwnerRegistrationResult {}
impl ::core::fmt::Debug for IsolatedWindowsEnvironmentOwnerRegistrationResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IsolatedWindowsEnvironmentOwnerRegistrationResult").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for IsolatedWindowsEnvironmentOwnerRegistrationResult {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Security.Isolation.IsolatedWindowsEnvironmentOwnerRegistrationResult;{6dab9451-6169-55df-8f51-790e99d7277d})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for IsolatedWindowsEnvironmentOwnerRegistrationResult {
    type Vtable = IIsolatedWindowsEnvironmentOwnerRegistrationResult_Vtbl;
    const IID: ::windows_core::GUID = <IIsolatedWindowsEnvironmentOwnerRegistrationResult as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for IsolatedWindowsEnvironmentOwnerRegistrationResult {
    const NAME: &'static str = "Windows.Security.Isolation.IsolatedWindowsEnvironmentOwnerRegistrationResult";
}
impl ::core::convert::From<IsolatedWindowsEnvironmentOwnerRegistrationResult> for ::windows_core::IUnknown {
    fn from(value: IsolatedWindowsEnvironmentOwnerRegistrationResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IsolatedWindowsEnvironmentOwnerRegistrationResult> for ::windows_core::IUnknown {
    fn from(value: &IsolatedWindowsEnvironmentOwnerRegistrationResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IsolatedWindowsEnvironmentOwnerRegistrationResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IsolatedWindowsEnvironmentOwnerRegistrationResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IsolatedWindowsEnvironmentOwnerRegistrationResult> for ::windows_core::IInspectable {
    fn from(value: IsolatedWindowsEnvironmentOwnerRegistrationResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IsolatedWindowsEnvironmentOwnerRegistrationResult> for ::windows_core::IInspectable {
    fn from(value: &IsolatedWindowsEnvironmentOwnerRegistrationResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for IsolatedWindowsEnvironmentOwnerRegistrationResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a IsolatedWindowsEnvironmentOwnerRegistrationResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for IsolatedWindowsEnvironmentOwnerRegistrationResult {}
unsafe impl ::core::marker::Sync for IsolatedWindowsEnvironmentOwnerRegistrationResult {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct IsolatedWindowsEnvironmentOwnerRegistrationStatus(pub i32);
impl IsolatedWindowsEnvironmentOwnerRegistrationStatus {
    pub const Success: Self = Self(0i32);
    pub const InvalidArgument: Self = Self(1i32);
    pub const AccessDenied: Self = Self(2i32);
    pub const InsufficientMemory: Self = Self(3i32);
    pub const UnknownFailure: Self = Self(4i32);
}
impl ::core::marker::Copy for IsolatedWindowsEnvironmentOwnerRegistrationStatus {}
impl ::core::clone::Clone for IsolatedWindowsEnvironmentOwnerRegistrationStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for IsolatedWindowsEnvironmentOwnerRegistrationStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for IsolatedWindowsEnvironmentOwnerRegistrationStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for IsolatedWindowsEnvironmentOwnerRegistrationStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IsolatedWindowsEnvironmentOwnerRegistrationStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for IsolatedWindowsEnvironmentOwnerRegistrationStatus {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Security.Isolation.IsolatedWindowsEnvironmentOwnerRegistrationStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct IsolatedWindowsEnvironmentPostMessageResult(::windows_core::IUnknown);
impl IsolatedWindowsEnvironmentPostMessageResult {
    pub fn Status(&self) -> ::windows_core::Result<IsolatedWindowsEnvironmentPostMessageStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<IsolatedWindowsEnvironmentPostMessageStatus>::zeroed();
            (::windows_core::Interface::vtable(this).Status)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<IsolatedWindowsEnvironmentPostMessageStatus>(result__)
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
impl ::core::clone::Clone for IsolatedWindowsEnvironmentPostMessageResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IsolatedWindowsEnvironmentPostMessageResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IsolatedWindowsEnvironmentPostMessageResult {}
impl ::core::fmt::Debug for IsolatedWindowsEnvironmentPostMessageResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IsolatedWindowsEnvironmentPostMessageResult").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for IsolatedWindowsEnvironmentPostMessageResult {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Security.Isolation.IsolatedWindowsEnvironmentPostMessageResult;{0dfa28fa-2ef0-4d8f-b341-3171b2df93b1})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for IsolatedWindowsEnvironmentPostMessageResult {
    type Vtable = IIsolatedWindowsEnvironmentPostMessageResult_Vtbl;
    const IID: ::windows_core::GUID = <IIsolatedWindowsEnvironmentPostMessageResult as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for IsolatedWindowsEnvironmentPostMessageResult {
    const NAME: &'static str = "Windows.Security.Isolation.IsolatedWindowsEnvironmentPostMessageResult";
}
impl ::core::convert::From<IsolatedWindowsEnvironmentPostMessageResult> for ::windows_core::IUnknown {
    fn from(value: IsolatedWindowsEnvironmentPostMessageResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IsolatedWindowsEnvironmentPostMessageResult> for ::windows_core::IUnknown {
    fn from(value: &IsolatedWindowsEnvironmentPostMessageResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IsolatedWindowsEnvironmentPostMessageResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IsolatedWindowsEnvironmentPostMessageResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IsolatedWindowsEnvironmentPostMessageResult> for ::windows_core::IInspectable {
    fn from(value: IsolatedWindowsEnvironmentPostMessageResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IsolatedWindowsEnvironmentPostMessageResult> for ::windows_core::IInspectable {
    fn from(value: &IsolatedWindowsEnvironmentPostMessageResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for IsolatedWindowsEnvironmentPostMessageResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a IsolatedWindowsEnvironmentPostMessageResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for IsolatedWindowsEnvironmentPostMessageResult {}
unsafe impl ::core::marker::Sync for IsolatedWindowsEnvironmentPostMessageResult {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct IsolatedWindowsEnvironmentPostMessageStatus(pub i32);
impl IsolatedWindowsEnvironmentPostMessageStatus {
    pub const Success: Self = Self(0i32);
    pub const UnknownFailure: Self = Self(1i32);
    pub const EnvironmentUnavailable: Self = Self(2i32);
}
impl ::core::marker::Copy for IsolatedWindowsEnvironmentPostMessageStatus {}
impl ::core::clone::Clone for IsolatedWindowsEnvironmentPostMessageStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for IsolatedWindowsEnvironmentPostMessageStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for IsolatedWindowsEnvironmentPostMessageStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for IsolatedWindowsEnvironmentPostMessageStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IsolatedWindowsEnvironmentPostMessageStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for IsolatedWindowsEnvironmentPostMessageStatus {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Security.Isolation.IsolatedWindowsEnvironmentPostMessageStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct IsolatedWindowsEnvironmentProcess(::windows_core::IUnknown);
impl IsolatedWindowsEnvironmentProcess {
    pub fn State(&self) -> ::windows_core::Result<IsolatedWindowsEnvironmentProcessState> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<IsolatedWindowsEnvironmentProcessState>::zeroed();
            (::windows_core::Interface::vtable(this).State)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<IsolatedWindowsEnvironmentProcessState>(result__)
        }
    }
    pub fn ExitCode(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).ExitCode)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn WaitForExit(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).WaitForExit)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn WaitForExitWithTimeout(&self, timeoutmilliseconds: u32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).WaitForExitWithTimeout)(::windows_core::Interface::as_raw(this), timeoutmilliseconds).ok() }
    }
    pub fn WaitForExitAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).WaitForExitAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncAction>(result__)
        }
    }
}
impl ::core::clone::Clone for IsolatedWindowsEnvironmentProcess {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IsolatedWindowsEnvironmentProcess {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IsolatedWindowsEnvironmentProcess {}
impl ::core::fmt::Debug for IsolatedWindowsEnvironmentProcess {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IsolatedWindowsEnvironmentProcess").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for IsolatedWindowsEnvironmentProcess {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Security.Isolation.IsolatedWindowsEnvironmentProcess;{a858c3ef-8172-4f10-af93-cbe60af88d09})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for IsolatedWindowsEnvironmentProcess {
    type Vtable = IIsolatedWindowsEnvironmentProcess_Vtbl;
    const IID: ::windows_core::GUID = <IIsolatedWindowsEnvironmentProcess as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for IsolatedWindowsEnvironmentProcess {
    const NAME: &'static str = "Windows.Security.Isolation.IsolatedWindowsEnvironmentProcess";
}
impl ::core::convert::From<IsolatedWindowsEnvironmentProcess> for ::windows_core::IUnknown {
    fn from(value: IsolatedWindowsEnvironmentProcess) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IsolatedWindowsEnvironmentProcess> for ::windows_core::IUnknown {
    fn from(value: &IsolatedWindowsEnvironmentProcess) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IsolatedWindowsEnvironmentProcess {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IsolatedWindowsEnvironmentProcess {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IsolatedWindowsEnvironmentProcess> for ::windows_core::IInspectable {
    fn from(value: IsolatedWindowsEnvironmentProcess) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IsolatedWindowsEnvironmentProcess> for ::windows_core::IInspectable {
    fn from(value: &IsolatedWindowsEnvironmentProcess) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for IsolatedWindowsEnvironmentProcess {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a IsolatedWindowsEnvironmentProcess {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for IsolatedWindowsEnvironmentProcess {}
unsafe impl ::core::marker::Sync for IsolatedWindowsEnvironmentProcess {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct IsolatedWindowsEnvironmentProcessState(pub i32);
impl IsolatedWindowsEnvironmentProcessState {
    pub const Running: Self = Self(1i32);
    pub const Aborted: Self = Self(2i32);
    pub const Completed: Self = Self(3i32);
}
impl ::core::marker::Copy for IsolatedWindowsEnvironmentProcessState {}
impl ::core::clone::Clone for IsolatedWindowsEnvironmentProcessState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for IsolatedWindowsEnvironmentProcessState {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for IsolatedWindowsEnvironmentProcessState {
    type Abi = Self;
}
impl ::core::fmt::Debug for IsolatedWindowsEnvironmentProcessState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IsolatedWindowsEnvironmentProcessState").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for IsolatedWindowsEnvironmentProcessState {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Security.Isolation.IsolatedWindowsEnvironmentProcessState;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct IsolatedWindowsEnvironmentProgressState(pub i32);
impl IsolatedWindowsEnvironmentProgressState {
    pub const Queued: Self = Self(0i32);
    pub const Processing: Self = Self(1i32);
    pub const Completed: Self = Self(2i32);
}
impl ::core::marker::Copy for IsolatedWindowsEnvironmentProgressState {}
impl ::core::clone::Clone for IsolatedWindowsEnvironmentProgressState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for IsolatedWindowsEnvironmentProgressState {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for IsolatedWindowsEnvironmentProgressState {
    type Abi = Self;
}
impl ::core::fmt::Debug for IsolatedWindowsEnvironmentProgressState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IsolatedWindowsEnvironmentProgressState").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for IsolatedWindowsEnvironmentProgressState {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Security.Isolation.IsolatedWindowsEnvironmentProgressState;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct IsolatedWindowsEnvironmentShareFileRequestOptions(::windows_core::IUnknown);
impl IsolatedWindowsEnvironmentShareFileRequestOptions {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<IsolatedWindowsEnvironmentShareFileRequestOptions, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn AllowWrite(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).AllowWrite)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetAllowWrite(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetAllowWrite)(::windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl ::core::clone::Clone for IsolatedWindowsEnvironmentShareFileRequestOptions {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IsolatedWindowsEnvironmentShareFileRequestOptions {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IsolatedWindowsEnvironmentShareFileRequestOptions {}
impl ::core::fmt::Debug for IsolatedWindowsEnvironmentShareFileRequestOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IsolatedWindowsEnvironmentShareFileRequestOptions").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for IsolatedWindowsEnvironmentShareFileRequestOptions {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Security.Isolation.IsolatedWindowsEnvironmentShareFileRequestOptions;{c9190ed8-0fd0-4946-bb88-117a60737b61})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for IsolatedWindowsEnvironmentShareFileRequestOptions {
    type Vtable = IIsolatedWindowsEnvironmentShareFileRequestOptions_Vtbl;
    const IID: ::windows_core::GUID = <IIsolatedWindowsEnvironmentShareFileRequestOptions as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for IsolatedWindowsEnvironmentShareFileRequestOptions {
    const NAME: &'static str = "Windows.Security.Isolation.IsolatedWindowsEnvironmentShareFileRequestOptions";
}
impl ::core::convert::From<IsolatedWindowsEnvironmentShareFileRequestOptions> for ::windows_core::IUnknown {
    fn from(value: IsolatedWindowsEnvironmentShareFileRequestOptions) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IsolatedWindowsEnvironmentShareFileRequestOptions> for ::windows_core::IUnknown {
    fn from(value: &IsolatedWindowsEnvironmentShareFileRequestOptions) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IsolatedWindowsEnvironmentShareFileRequestOptions {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IsolatedWindowsEnvironmentShareFileRequestOptions {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IsolatedWindowsEnvironmentShareFileRequestOptions> for ::windows_core::IInspectable {
    fn from(value: IsolatedWindowsEnvironmentShareFileRequestOptions) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IsolatedWindowsEnvironmentShareFileRequestOptions> for ::windows_core::IInspectable {
    fn from(value: &IsolatedWindowsEnvironmentShareFileRequestOptions) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for IsolatedWindowsEnvironmentShareFileRequestOptions {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a IsolatedWindowsEnvironmentShareFileRequestOptions {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for IsolatedWindowsEnvironmentShareFileRequestOptions {}
unsafe impl ::core::marker::Sync for IsolatedWindowsEnvironmentShareFileRequestOptions {}
#[repr(transparent)]
pub struct IsolatedWindowsEnvironmentShareFileResult(::windows_core::IUnknown);
impl IsolatedWindowsEnvironmentShareFileResult {
    pub fn Status(&self) -> ::windows_core::Result<IsolatedWindowsEnvironmentShareFileStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<IsolatedWindowsEnvironmentShareFileStatus>::zeroed();
            (::windows_core::Interface::vtable(this).Status)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<IsolatedWindowsEnvironmentShareFileStatus>(result__)
        }
    }
    pub fn ExtendedError(&self) -> ::windows_core::Result<::windows_core::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::HRESULT>::zeroed();
            (::windows_core::Interface::vtable(this).ExtendedError)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HRESULT>(result__)
        }
    }
    pub fn File(&self) -> ::windows_core::Result<IsolatedWindowsEnvironmentFile> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).File)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<IsolatedWindowsEnvironmentFile>(result__)
        }
    }
}
impl ::core::clone::Clone for IsolatedWindowsEnvironmentShareFileResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IsolatedWindowsEnvironmentShareFileResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IsolatedWindowsEnvironmentShareFileResult {}
impl ::core::fmt::Debug for IsolatedWindowsEnvironmentShareFileResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IsolatedWindowsEnvironmentShareFileResult").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for IsolatedWindowsEnvironmentShareFileResult {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Security.Isolation.IsolatedWindowsEnvironmentShareFileResult;{aec7caa7-9ac6-4bf5-8b91-5c1adf0d7d00})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for IsolatedWindowsEnvironmentShareFileResult {
    type Vtable = IIsolatedWindowsEnvironmentShareFileResult_Vtbl;
    const IID: ::windows_core::GUID = <IIsolatedWindowsEnvironmentShareFileResult as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for IsolatedWindowsEnvironmentShareFileResult {
    const NAME: &'static str = "Windows.Security.Isolation.IsolatedWindowsEnvironmentShareFileResult";
}
impl ::core::convert::From<IsolatedWindowsEnvironmentShareFileResult> for ::windows_core::IUnknown {
    fn from(value: IsolatedWindowsEnvironmentShareFileResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IsolatedWindowsEnvironmentShareFileResult> for ::windows_core::IUnknown {
    fn from(value: &IsolatedWindowsEnvironmentShareFileResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IsolatedWindowsEnvironmentShareFileResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IsolatedWindowsEnvironmentShareFileResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IsolatedWindowsEnvironmentShareFileResult> for ::windows_core::IInspectable {
    fn from(value: IsolatedWindowsEnvironmentShareFileResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IsolatedWindowsEnvironmentShareFileResult> for ::windows_core::IInspectable {
    fn from(value: &IsolatedWindowsEnvironmentShareFileResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for IsolatedWindowsEnvironmentShareFileResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a IsolatedWindowsEnvironmentShareFileResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for IsolatedWindowsEnvironmentShareFileResult {}
unsafe impl ::core::marker::Sync for IsolatedWindowsEnvironmentShareFileResult {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct IsolatedWindowsEnvironmentShareFileStatus(pub i32);
impl IsolatedWindowsEnvironmentShareFileStatus {
    pub const Success: Self = Self(0i32);
    pub const UnknownFailure: Self = Self(1i32);
    pub const EnvironmentUnavailable: Self = Self(2i32);
    pub const AlreadySharedWithConflictingOptions: Self = Self(3i32);
    pub const FileNotFound: Self = Self(4i32);
    pub const AccessDenied: Self = Self(5i32);
}
impl ::core::marker::Copy for IsolatedWindowsEnvironmentShareFileStatus {}
impl ::core::clone::Clone for IsolatedWindowsEnvironmentShareFileStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for IsolatedWindowsEnvironmentShareFileStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for IsolatedWindowsEnvironmentShareFileStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for IsolatedWindowsEnvironmentShareFileStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IsolatedWindowsEnvironmentShareFileStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for IsolatedWindowsEnvironmentShareFileStatus {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Security.Isolation.IsolatedWindowsEnvironmentShareFileStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct IsolatedWindowsEnvironmentShareFolderRequestOptions(::windows_core::IUnknown);
impl IsolatedWindowsEnvironmentShareFolderRequestOptions {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<IsolatedWindowsEnvironmentShareFolderRequestOptions, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn AllowWrite(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).AllowWrite)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetAllowWrite(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetAllowWrite)(::windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl ::core::clone::Clone for IsolatedWindowsEnvironmentShareFolderRequestOptions {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IsolatedWindowsEnvironmentShareFolderRequestOptions {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IsolatedWindowsEnvironmentShareFolderRequestOptions {}
impl ::core::fmt::Debug for IsolatedWindowsEnvironmentShareFolderRequestOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IsolatedWindowsEnvironmentShareFolderRequestOptions").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for IsolatedWindowsEnvironmentShareFolderRequestOptions {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Security.Isolation.IsolatedWindowsEnvironmentShareFolderRequestOptions;{c405eb7d-7053-4f6a-9b87-746846ed19b2})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for IsolatedWindowsEnvironmentShareFolderRequestOptions {
    type Vtable = IIsolatedWindowsEnvironmentShareFolderRequestOptions_Vtbl;
    const IID: ::windows_core::GUID = <IIsolatedWindowsEnvironmentShareFolderRequestOptions as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for IsolatedWindowsEnvironmentShareFolderRequestOptions {
    const NAME: &'static str = "Windows.Security.Isolation.IsolatedWindowsEnvironmentShareFolderRequestOptions";
}
impl ::core::convert::From<IsolatedWindowsEnvironmentShareFolderRequestOptions> for ::windows_core::IUnknown {
    fn from(value: IsolatedWindowsEnvironmentShareFolderRequestOptions) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IsolatedWindowsEnvironmentShareFolderRequestOptions> for ::windows_core::IUnknown {
    fn from(value: &IsolatedWindowsEnvironmentShareFolderRequestOptions) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IsolatedWindowsEnvironmentShareFolderRequestOptions {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IsolatedWindowsEnvironmentShareFolderRequestOptions {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IsolatedWindowsEnvironmentShareFolderRequestOptions> for ::windows_core::IInspectable {
    fn from(value: IsolatedWindowsEnvironmentShareFolderRequestOptions) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IsolatedWindowsEnvironmentShareFolderRequestOptions> for ::windows_core::IInspectable {
    fn from(value: &IsolatedWindowsEnvironmentShareFolderRequestOptions) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for IsolatedWindowsEnvironmentShareFolderRequestOptions {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a IsolatedWindowsEnvironmentShareFolderRequestOptions {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for IsolatedWindowsEnvironmentShareFolderRequestOptions {}
unsafe impl ::core::marker::Sync for IsolatedWindowsEnvironmentShareFolderRequestOptions {}
#[repr(transparent)]
pub struct IsolatedWindowsEnvironmentShareFolderResult(::windows_core::IUnknown);
impl IsolatedWindowsEnvironmentShareFolderResult {
    pub fn Status(&self) -> ::windows_core::Result<IsolatedWindowsEnvironmentShareFolderStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<IsolatedWindowsEnvironmentShareFolderStatus>::zeroed();
            (::windows_core::Interface::vtable(this).Status)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<IsolatedWindowsEnvironmentShareFolderStatus>(result__)
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
impl ::core::clone::Clone for IsolatedWindowsEnvironmentShareFolderResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IsolatedWindowsEnvironmentShareFolderResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IsolatedWindowsEnvironmentShareFolderResult {}
impl ::core::fmt::Debug for IsolatedWindowsEnvironmentShareFolderResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IsolatedWindowsEnvironmentShareFolderResult").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for IsolatedWindowsEnvironmentShareFolderResult {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Security.Isolation.IsolatedWindowsEnvironmentShareFolderResult;{556ba72e-ca9d-4211-b143-1cedc86eb2fe})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for IsolatedWindowsEnvironmentShareFolderResult {
    type Vtable = IIsolatedWindowsEnvironmentShareFolderResult_Vtbl;
    const IID: ::windows_core::GUID = <IIsolatedWindowsEnvironmentShareFolderResult as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for IsolatedWindowsEnvironmentShareFolderResult {
    const NAME: &'static str = "Windows.Security.Isolation.IsolatedWindowsEnvironmentShareFolderResult";
}
impl ::core::convert::From<IsolatedWindowsEnvironmentShareFolderResult> for ::windows_core::IUnknown {
    fn from(value: IsolatedWindowsEnvironmentShareFolderResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IsolatedWindowsEnvironmentShareFolderResult> for ::windows_core::IUnknown {
    fn from(value: &IsolatedWindowsEnvironmentShareFolderResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IsolatedWindowsEnvironmentShareFolderResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IsolatedWindowsEnvironmentShareFolderResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IsolatedWindowsEnvironmentShareFolderResult> for ::windows_core::IInspectable {
    fn from(value: IsolatedWindowsEnvironmentShareFolderResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IsolatedWindowsEnvironmentShareFolderResult> for ::windows_core::IInspectable {
    fn from(value: &IsolatedWindowsEnvironmentShareFolderResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for IsolatedWindowsEnvironmentShareFolderResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a IsolatedWindowsEnvironmentShareFolderResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for IsolatedWindowsEnvironmentShareFolderResult {}
unsafe impl ::core::marker::Sync for IsolatedWindowsEnvironmentShareFolderResult {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct IsolatedWindowsEnvironmentShareFolderStatus(pub i32);
impl IsolatedWindowsEnvironmentShareFolderStatus {
    pub const Success: Self = Self(0i32);
    pub const UnknownFailure: Self = Self(1i32);
    pub const EnvironmentUnavailable: Self = Self(2i32);
    pub const FolderNotFound: Self = Self(3i32);
    pub const AccessDenied: Self = Self(4i32);
}
impl ::core::marker::Copy for IsolatedWindowsEnvironmentShareFolderStatus {}
impl ::core::clone::Clone for IsolatedWindowsEnvironmentShareFolderStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for IsolatedWindowsEnvironmentShareFolderStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for IsolatedWindowsEnvironmentShareFolderStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for IsolatedWindowsEnvironmentShareFolderStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IsolatedWindowsEnvironmentShareFolderStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for IsolatedWindowsEnvironmentShareFolderStatus {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Security.Isolation.IsolatedWindowsEnvironmentShareFolderStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct IsolatedWindowsEnvironmentStartProcessResult(::windows_core::IUnknown);
impl IsolatedWindowsEnvironmentStartProcessResult {
    pub fn Status(&self) -> ::windows_core::Result<IsolatedWindowsEnvironmentStartProcessStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<IsolatedWindowsEnvironmentStartProcessStatus>::zeroed();
            (::windows_core::Interface::vtable(this).Status)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<IsolatedWindowsEnvironmentStartProcessStatus>(result__)
        }
    }
    pub fn ExtendedError(&self) -> ::windows_core::Result<::windows_core::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::HRESULT>::zeroed();
            (::windows_core::Interface::vtable(this).ExtendedError)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HRESULT>(result__)
        }
    }
    pub fn Process(&self) -> ::windows_core::Result<IsolatedWindowsEnvironmentProcess> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Process)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<IsolatedWindowsEnvironmentProcess>(result__)
        }
    }
}
impl ::core::clone::Clone for IsolatedWindowsEnvironmentStartProcessResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IsolatedWindowsEnvironmentStartProcessResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IsolatedWindowsEnvironmentStartProcessResult {}
impl ::core::fmt::Debug for IsolatedWindowsEnvironmentStartProcessResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IsolatedWindowsEnvironmentStartProcessResult").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for IsolatedWindowsEnvironmentStartProcessResult {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Security.Isolation.IsolatedWindowsEnvironmentStartProcessResult;{8fa1dc2f-57da-4bb5-9c06-fa072d2032e2})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for IsolatedWindowsEnvironmentStartProcessResult {
    type Vtable = IIsolatedWindowsEnvironmentStartProcessResult_Vtbl;
    const IID: ::windows_core::GUID = <IIsolatedWindowsEnvironmentStartProcessResult as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for IsolatedWindowsEnvironmentStartProcessResult {
    const NAME: &'static str = "Windows.Security.Isolation.IsolatedWindowsEnvironmentStartProcessResult";
}
impl ::core::convert::From<IsolatedWindowsEnvironmentStartProcessResult> for ::windows_core::IUnknown {
    fn from(value: IsolatedWindowsEnvironmentStartProcessResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IsolatedWindowsEnvironmentStartProcessResult> for ::windows_core::IUnknown {
    fn from(value: &IsolatedWindowsEnvironmentStartProcessResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IsolatedWindowsEnvironmentStartProcessResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IsolatedWindowsEnvironmentStartProcessResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IsolatedWindowsEnvironmentStartProcessResult> for ::windows_core::IInspectable {
    fn from(value: IsolatedWindowsEnvironmentStartProcessResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IsolatedWindowsEnvironmentStartProcessResult> for ::windows_core::IInspectable {
    fn from(value: &IsolatedWindowsEnvironmentStartProcessResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for IsolatedWindowsEnvironmentStartProcessResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a IsolatedWindowsEnvironmentStartProcessResult {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for IsolatedWindowsEnvironmentStartProcessResult {}
unsafe impl ::core::marker::Sync for IsolatedWindowsEnvironmentStartProcessResult {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct IsolatedWindowsEnvironmentStartProcessStatus(pub i32);
impl IsolatedWindowsEnvironmentStartProcessStatus {
    pub const Success: Self = Self(0i32);
    pub const UnknownFailure: Self = Self(1i32);
    pub const EnvironmentUnavailable: Self = Self(2i32);
    pub const FileNotFound: Self = Self(3i32);
    pub const AppNotRegistered: Self = Self(4i32);
}
impl ::core::marker::Copy for IsolatedWindowsEnvironmentStartProcessStatus {}
impl ::core::clone::Clone for IsolatedWindowsEnvironmentStartProcessStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for IsolatedWindowsEnvironmentStartProcessStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for IsolatedWindowsEnvironmentStartProcessStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for IsolatedWindowsEnvironmentStartProcessStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IsolatedWindowsEnvironmentStartProcessStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for IsolatedWindowsEnvironmentStartProcessStatus {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Security.Isolation.IsolatedWindowsEnvironmentStartProcessStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct IsolatedWindowsEnvironmentTelemetryParameters(::windows_core::IUnknown);
impl IsolatedWindowsEnvironmentTelemetryParameters {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<IsolatedWindowsEnvironmentTelemetryParameters, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn CorrelationId(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::GUID>::zeroed();
            (::windows_core::Interface::vtable(this).CorrelationId)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::GUID>(result__)
        }
    }
    pub fn SetCorrelationId<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::GUID>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetCorrelationId)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
}
impl ::core::clone::Clone for IsolatedWindowsEnvironmentTelemetryParameters {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IsolatedWindowsEnvironmentTelemetryParameters {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IsolatedWindowsEnvironmentTelemetryParameters {}
impl ::core::fmt::Debug for IsolatedWindowsEnvironmentTelemetryParameters {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IsolatedWindowsEnvironmentTelemetryParameters").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for IsolatedWindowsEnvironmentTelemetryParameters {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Security.Isolation.IsolatedWindowsEnvironmentTelemetryParameters;{ebdb3cab-7a3a-4524-a0f4-f96e284d33cd})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for IsolatedWindowsEnvironmentTelemetryParameters {
    type Vtable = IIsolatedWindowsEnvironmentTelemetryParameters_Vtbl;
    const IID: ::windows_core::GUID = <IIsolatedWindowsEnvironmentTelemetryParameters as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for IsolatedWindowsEnvironmentTelemetryParameters {
    const NAME: &'static str = "Windows.Security.Isolation.IsolatedWindowsEnvironmentTelemetryParameters";
}
impl ::core::convert::From<IsolatedWindowsEnvironmentTelemetryParameters> for ::windows_core::IUnknown {
    fn from(value: IsolatedWindowsEnvironmentTelemetryParameters) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IsolatedWindowsEnvironmentTelemetryParameters> for ::windows_core::IUnknown {
    fn from(value: &IsolatedWindowsEnvironmentTelemetryParameters) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IsolatedWindowsEnvironmentTelemetryParameters {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IsolatedWindowsEnvironmentTelemetryParameters {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IsolatedWindowsEnvironmentTelemetryParameters> for ::windows_core::IInspectable {
    fn from(value: IsolatedWindowsEnvironmentTelemetryParameters) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IsolatedWindowsEnvironmentTelemetryParameters> for ::windows_core::IInspectable {
    fn from(value: &IsolatedWindowsEnvironmentTelemetryParameters) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for IsolatedWindowsEnvironmentTelemetryParameters {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a IsolatedWindowsEnvironmentTelemetryParameters {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for IsolatedWindowsEnvironmentTelemetryParameters {}
unsafe impl ::core::marker::Sync for IsolatedWindowsEnvironmentTelemetryParameters {}
#[repr(transparent)]
pub struct IsolatedWindowsEnvironmentUserInfo(::windows_core::IUnknown);
impl IsolatedWindowsEnvironmentUserInfo {
    pub fn EnvironmentUserSid(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).EnvironmentUserSid)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn EnvironmentUserName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).EnvironmentUserName)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn TryWaitForSignInAsync(&self) -> ::windows_core::Result<::winrt_foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).TryWaitForSignInAsync)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::IAsyncOperation<bool>>(result__)
        }
    }
}
impl ::core::clone::Clone for IsolatedWindowsEnvironmentUserInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IsolatedWindowsEnvironmentUserInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IsolatedWindowsEnvironmentUserInfo {}
impl ::core::fmt::Debug for IsolatedWindowsEnvironmentUserInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IsolatedWindowsEnvironmentUserInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for IsolatedWindowsEnvironmentUserInfo {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Security.Isolation.IsolatedWindowsEnvironmentUserInfo;{8a9c75ae-69ba-4001-96fc-19a02703b340})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for IsolatedWindowsEnvironmentUserInfo {
    type Vtable = IIsolatedWindowsEnvironmentUserInfo_Vtbl;
    const IID: ::windows_core::GUID = <IIsolatedWindowsEnvironmentUserInfo as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for IsolatedWindowsEnvironmentUserInfo {
    const NAME: &'static str = "Windows.Security.Isolation.IsolatedWindowsEnvironmentUserInfo";
}
impl ::core::convert::From<IsolatedWindowsEnvironmentUserInfo> for ::windows_core::IUnknown {
    fn from(value: IsolatedWindowsEnvironmentUserInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IsolatedWindowsEnvironmentUserInfo> for ::windows_core::IUnknown {
    fn from(value: &IsolatedWindowsEnvironmentUserInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IsolatedWindowsEnvironmentUserInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IsolatedWindowsEnvironmentUserInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IsolatedWindowsEnvironmentUserInfo> for ::windows_core::IInspectable {
    fn from(value: IsolatedWindowsEnvironmentUserInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IsolatedWindowsEnvironmentUserInfo> for ::windows_core::IInspectable {
    fn from(value: &IsolatedWindowsEnvironmentUserInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for IsolatedWindowsEnvironmentUserInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a IsolatedWindowsEnvironmentUserInfo {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for IsolatedWindowsEnvironmentUserInfo {}
unsafe impl ::core::marker::Sync for IsolatedWindowsEnvironmentUserInfo {}
pub struct IsolatedWindowsHostMessenger;
impl IsolatedWindowsHostMessenger {
    #[cfg(feature = "winrt-foundation")]
    pub fn PostMessageToReceiver<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::GUID>, Param1: ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IVectorView<::windows_core::IInspectable>>>(receiverid: Param0, message: Param1) -> ::windows_core::Result<()> {
        Self::IIsolatedWindowsHostMessengerStatics(|this| unsafe { (::windows_core::Interface::vtable(this).PostMessageToReceiver)(::windows_core::Interface::as_raw(this), receiverid.into_param().abi(), message.into_param().abi()).ok() })
    }
    pub fn GetFileId<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(filepath: Param0) -> ::windows_core::Result<::windows_core::GUID> {
        Self::IIsolatedWindowsHostMessengerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::GUID>::zeroed();
            (::windows_core::Interface::vtable(this).GetFileId)(::windows_core::Interface::as_raw(this), filepath.into_param().abi(), result__.as_mut_ptr()).from_abi::<::windows_core::GUID>(result__)
        })
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn RegisterHostMessageReceiver<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::GUID>, Param1: ::windows_core::IntoParam<'a, HostMessageReceivedCallback>>(receiverid: Param0, hostmessagereceivedcallback: Param1) -> ::windows_core::Result<()> {
        Self::IIsolatedWindowsHostMessengerStatics2(|this| unsafe { (::windows_core::Interface::vtable(this).RegisterHostMessageReceiver)(::windows_core::Interface::as_raw(this), receiverid.into_param().abi(), hostmessagereceivedcallback.into_param().abi()).ok() })
    }
    pub fn UnregisterHostMessageReceiver<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::GUID>>(receiverid: Param0) -> ::windows_core::Result<()> {
        Self::IIsolatedWindowsHostMessengerStatics2(|this| unsafe { (::windows_core::Interface::vtable(this).UnregisterHostMessageReceiver)(::windows_core::Interface::as_raw(this), receiverid.into_param().abi()).ok() })
    }
    pub fn IIsolatedWindowsHostMessengerStatics<R, F: FnOnce(&IIsolatedWindowsHostMessengerStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<IsolatedWindowsHostMessenger, IIsolatedWindowsHostMessengerStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IIsolatedWindowsHostMessengerStatics2<R, F: FnOnce(&IIsolatedWindowsHostMessengerStatics2) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<IsolatedWindowsHostMessenger, IIsolatedWindowsHostMessengerStatics2> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows_core::RuntimeName for IsolatedWindowsHostMessenger {
    const NAME: &'static str = "Windows.Security.Isolation.IsolatedWindowsHostMessenger";
}
#[cfg(feature = "winrt-foundation")]
#[repr(transparent)]
pub struct MessageReceivedCallback(pub ::windows_core::IUnknown);
#[cfg(feature = "winrt-foundation")]
impl MessageReceivedCallback {
    pub fn new<F: FnMut(&::windows_core::GUID, &::core::option::Option<::winrt_foundation::Collections::IVectorView<::windows_core::IInspectable>>) -> ::windows_core::Result<()> + ::core::marker::Send + 'static>(invoke: F) -> Self {
        let com = MessageReceivedCallbackBox::<F> { vtable: &MessageReceivedCallbackBox::<F>::VTABLE, count: ::windows_core::RefCount::new(1), invoke };
        unsafe { ::core::mem::transmute(::windows_core::alloc::boxed::Box::new(com)) }
    }
    #[cfg(feature = "winrt-foundation")]
    pub fn Invoke<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::GUID>, Param1: ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IVectorView<::windows_core::IInspectable>>>(&self, receiverid: Param0, message: Param1) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Invoke)(::windows_core::Interface::as_raw(this), receiverid.into_param().abi(), message.into_param().abi()).ok() }
    }
}
#[cfg(feature = "winrt-foundation")]
#[repr(C)]
struct MessageReceivedCallbackBox<F: FnMut(&::windows_core::GUID, &::core::option::Option<::winrt_foundation::Collections::IVectorView<::windows_core::IInspectable>>) -> ::windows_core::Result<()> + ::core::marker::Send + 'static> {
    vtable: *const MessageReceivedCallback_Vtbl,
    invoke: F,
    count: ::windows_core::RefCount,
}
#[cfg(feature = "winrt-foundation")]
impl<F: FnMut(&::windows_core::GUID, &::core::option::Option<::winrt_foundation::Collections::IVectorView<::windows_core::IInspectable>>) -> ::windows_core::Result<()> + ::core::marker::Send + 'static> MessageReceivedCallbackBox<F> {
    const VTABLE: MessageReceivedCallback_Vtbl = MessageReceivedCallback_Vtbl {
        base__: ::windows_core::IUnknownVtbl { QueryInterface: Self::QueryInterface, AddRef: Self::AddRef, Release: Self::Release },
        Invoke: Self::Invoke,
    };
    unsafe extern "system" fn QueryInterface(this: ::windows_core::RawPtr, iid: &::windows_core::GUID, interface: *mut *const ::core::ffi::c_void) -> ::windows_core::HRESULT {
        let this = this as *mut ::windows_core::RawPtr as *mut Self;
        *interface = if iid == &<MessageReceivedCallback as ::windows_core::Interface>::IID || iid == &<::windows_core::IUnknown as ::windows_core::Interface>::IID || iid == &<::windows_core::IAgileObject as ::windows_core::Interface>::IID { &mut (*this).vtable as *mut _ as _ } else { ::core::ptr::null_mut() };
        if (*interface).is_null() {
            ::windows_core::HRESULT(-2147467262)
        } else {
            (*this).count.add_ref();
            ::windows_core::HRESULT(0)
        }
    }
    unsafe extern "system" fn AddRef(this: ::windows_core::RawPtr) -> u32 {
        let this = this as *mut ::windows_core::RawPtr as *mut Self;
        (*this).count.add_ref()
    }
    unsafe extern "system" fn Release(this: ::windows_core::RawPtr) -> u32 {
        let this = this as *mut ::windows_core::RawPtr as *mut Self;
        let remaining = (*this).count.release();
        if remaining == 0 {
            ::windows_core::alloc::boxed::Box::from_raw(this);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(this: *mut ::core::ffi::c_void, receiverid: ::windows_core::GUID, message: ::windows_core::RawPtr) -> ::windows_core::HRESULT {
        let this = this as *mut ::windows_core::RawPtr as *mut Self;
        ((*this).invoke)(::core::mem::transmute(&receiverid), ::core::mem::transmute(&message)).into()
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::clone::Clone for MessageReceivedCallback {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::cmp::PartialEq for MessageReceivedCallback {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "winrt-foundation")]
impl ::core::cmp::Eq for MessageReceivedCallback {}
#[cfg(feature = "winrt-foundation")]
impl ::core::fmt::Debug for MessageReceivedCallback {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MessageReceivedCallback").field(&self.0).finish()
    }
}
#[cfg(feature = "winrt-foundation")]
unsafe impl ::windows_core::Interface for MessageReceivedCallback {
    type Vtable = MessageReceivedCallback_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf5b4c8ff_1d9d_4995_9fea_4d15257c0757);
}
#[cfg(feature = "winrt-foundation")]
unsafe impl ::windows_core::RuntimeType for MessageReceivedCallback {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"{f5b4c8ff-1d9d-4995-9fea-4d15257c0757}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
#[cfg(feature = "winrt-foundation")]
#[repr(C)]
#[doc(hidden)]
pub struct MessageReceivedCallback_Vtbl {
    pub base__: ::windows_core::IUnknownVtbl,
    #[cfg(feature = "winrt-foundation")]
    pub Invoke: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, receiverid: ::windows_core::GUID, message: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "winrt-foundation"))]
    Invoke: usize,
}
