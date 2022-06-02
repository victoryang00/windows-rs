#[cfg(feature = "Perception_Automation")]
pub mod Automation;
#[cfg(feature = "Perception_People")]
pub mod People;
#[cfg(feature = "Perception_Spatial")]
pub mod Spatial;
#[repr(C)]
pub struct IPerceptionTimestamp {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub TargetTime: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::Foundation::DateTime) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TargetTime: usize,
    #[cfg(feature = "Foundation")]
    pub PredictionAmount: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PredictionAmount: usize,
}
#[repr(C)]
pub struct IPerceptionTimestamp2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub SystemRelativeTargetTime: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SystemRelativeTargetTime: usize,
}
#[repr(C)]
pub struct IPerceptionTimestampHelperStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub FromHistoricalTargetTime: unsafe extern "system" fn(this: *mut *mut Self, targettime: super::Foundation::DateTime, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FromHistoricalTargetTime: usize,
}
#[repr(C)]
pub struct IPerceptionTimestampHelperStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub FromSystemRelativeTargetTime: unsafe extern "system" fn(this: *mut *mut Self, targettime: super::Foundation::TimeSpan, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FromSystemRelativeTargetTime: usize,
}
pub type PerceptionTimestamp = *mut ::core::ffi::c_void;
