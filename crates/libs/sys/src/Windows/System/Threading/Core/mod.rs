#[repr(C)]
pub struct IPreallocatedWorkItem {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub RunAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RunAsync: usize,
}
impl ::windows_sys::core::Interface for IPreallocatedWorkItem {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3067783676, data2: 48219, data3: 16410, data4: [168, 178, 110, 117, 77, 20, 218, 166] };
}
#[repr(C)]
pub struct IPreallocatedWorkItemFactory {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub CreateWorkItem: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateWorkItem: usize,
    #[cfg(feature = "Foundation")]
    pub CreateWorkItemWithPriority: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, priority: super::WorkItemPriority, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateWorkItemWithPriority: usize,
    #[cfg(feature = "Foundation")]
    pub CreateWorkItemWithPriorityAndOptions: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, priority: super::WorkItemPriority, options: super::WorkItemOptions, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateWorkItemWithPriorityAndOptions: usize,
}
impl ::windows_sys::core::Interface for IPreallocatedWorkItemFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3822267205, data2: 57322, data3: 18075, data4: [130, 197, 246, 227, 206, 253, 234, 251] };
}
#[repr(C)]
pub struct ISignalNotifier {
    pub base__: ::windows_sys::core::IInspectable,
    pub Enable: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Terminate: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ISignalNotifier {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 338189830, data2: 25511, data3: 18195, data4: [182, 217, 98, 246, 75, 86, 251, 139] };
}
#[repr(C)]
pub struct ISignalNotifierStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub AttachToEvent: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::HSTRING, handler: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub AttachToEventWithTimeout: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::HSTRING, handler: *mut ::core::ffi::c_void, timeout: super::super::super::Foundation::TimeSpan, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AttachToEventWithTimeout: usize,
    pub AttachToSemaphore: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::HSTRING, handler: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub AttachToSemaphoreWithTimeout: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::HSTRING, handler: *mut ::core::ffi::c_void, timeout: super::super::super::Foundation::TimeSpan, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AttachToSemaphoreWithTimeout: usize,
}
impl ::windows_sys::core::Interface for ISignalNotifierStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 474891622, data2: 33792, data3: 18131, data4: [161, 21, 125, 12, 13, 252, 159, 98] };
}
pub type PreallocatedWorkItem = *mut ::core::ffi::c_void;
pub type SignalHandler = *mut ::core::ffi::c_void;
pub type SignalNotifier = *mut ::core::ffi::c_void;
