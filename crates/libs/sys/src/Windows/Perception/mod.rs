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
impl ::windows_sys::core::Interface for IPerceptionTimestamp {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2277656580, data2: 41518, data3: 19163, data4: [186, 38, 215, 142, 246, 57, 188, 244] };
}
#[repr(C)]
pub struct IPerceptionTimestamp2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub SystemRelativeTargetTime: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SystemRelativeTargetTime: usize,
}
impl ::windows_sys::core::Interface for IPerceptionTimestamp2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3813980141, data2: 11217, data3: 16823, data4: [158, 208, 116, 161, 92, 53, 69, 55] };
}
#[repr(C)]
pub struct IPerceptionTimestampHelperStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub FromHistoricalTargetTime: unsafe extern "system" fn(this: *mut *mut Self, targettime: super::Foundation::DateTime, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FromHistoricalTargetTime: usize,
}
impl ::windows_sys::core::Interface for IPerceptionTimestampHelperStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1202065876, data2: 43487, data3: 20188, data4: [133, 93, 244, 211, 57, 217, 103, 172] };
}
#[repr(C)]
pub struct IPerceptionTimestampHelperStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub FromSystemRelativeTargetTime: unsafe extern "system" fn(this: *mut *mut Self, targettime: super::Foundation::TimeSpan, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FromSystemRelativeTargetTime: usize,
}
impl ::windows_sys::core::Interface for IPerceptionTimestampHelperStatics2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1943119870, data2: 16313, data3: 17777, data4: [135, 212, 60, 146, 10, 94, 134, 235] };
}
pub type PerceptionTimestamp = *mut ::core::ffi::c_void;
