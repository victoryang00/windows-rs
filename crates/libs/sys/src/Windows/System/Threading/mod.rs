#[cfg(feature = "System_Threading_Core")]
pub mod Core;
#[repr(C)]
pub struct IThreadPoolStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub RunAsync: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RunAsync: usize,
    #[cfg(feature = "Foundation")]
    pub RunWithPriorityAsync: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, priority: WorkItemPriority, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RunWithPriorityAsync: usize,
    #[cfg(feature = "Foundation")]
    pub RunWithPriorityAndOptionsAsync: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, priority: WorkItemPriority, options: WorkItemOptions, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RunWithPriorityAndOptionsAsync: usize,
}
impl ::windows_sys::core::Interface for IThreadPoolStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3065997277, data2: 33981, data3: 17656, data4: [172, 28, 147, 235, 203, 157, 186, 145] };
}
#[repr(C)]
pub struct IThreadPoolTimer {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub Period: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Period: usize,
    #[cfg(feature = "Foundation")]
    pub Delay: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Delay: usize,
    pub Cancel: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IThreadPoolTimer {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1498332792, data2: 21994, data3: 19080, data4: [165, 13, 52, 2, 174, 31, 156, 242] };
}
#[repr(C)]
pub struct IThreadPoolTimerStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub CreatePeriodicTimer: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, period: super::super::Foundation::TimeSpan, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreatePeriodicTimer: usize,
    #[cfg(feature = "Foundation")]
    pub CreateTimer: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, delay: super::super::Foundation::TimeSpan, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateTimer: usize,
    #[cfg(feature = "Foundation")]
    pub CreatePeriodicTimerWithCompletion: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, period: super::super::Foundation::TimeSpan, destroyed: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreatePeriodicTimerWithCompletion: usize,
    #[cfg(feature = "Foundation")]
    pub CreateTimerWithCompletion: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, delay: super::super::Foundation::TimeSpan, destroyed: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateTimerWithCompletion: usize,
}
impl ::windows_sys::core::Interface for IThreadPoolTimerStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 445291778, data2: 58498, data3: 17947, data4: [184, 199, 142, 250, 209, 204, 229, 144] };
}
pub type ThreadPoolTimer = *mut ::core::ffi::c_void;
pub type TimerDestroyedHandler = *mut ::core::ffi::c_void;
pub type TimerElapsedHandler = *mut ::core::ffi::c_void;
pub type WorkItemHandler = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"System_Threading\"`*"]
#[repr(transparent)]
pub struct WorkItemOptions(pub u32);
impl WorkItemOptions {
    pub const None: Self = Self(0u32);
    pub const TimeSliced: Self = Self(1u32);
}
impl ::core::marker::Copy for WorkItemOptions {}
impl ::core::clone::Clone for WorkItemOptions {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"System_Threading\"`*"]
#[repr(transparent)]
pub struct WorkItemPriority(pub i32);
impl WorkItemPriority {
    pub const Low: Self = Self(-1i32);
    pub const Normal: Self = Self(0i32);
    pub const High: Self = Self(1i32);
}
impl ::core::marker::Copy for WorkItemPriority {}
impl ::core::clone::Clone for WorkItemPriority {
    fn clone(&self) -> Self {
        *self
    }
}
