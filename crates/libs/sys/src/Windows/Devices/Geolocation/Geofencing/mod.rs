pub type Geofence = *mut ::core::ffi::c_void;
pub type GeofenceMonitor = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Devices_Geolocation_Geofencing\"`*"]
#[repr(transparent)]
pub struct GeofenceMonitorStatus(pub i32);
impl GeofenceMonitorStatus {
    pub const Ready: Self = Self(0i32);
    pub const Initializing: Self = Self(1i32);
    pub const NoData: Self = Self(2i32);
    pub const Disabled: Self = Self(3i32);
    pub const NotInitialized: Self = Self(4i32);
    pub const NotAvailable: Self = Self(5i32);
}
impl ::core::marker::Copy for GeofenceMonitorStatus {}
impl ::core::clone::Clone for GeofenceMonitorStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Devices_Geolocation_Geofencing\"`*"]
#[repr(transparent)]
pub struct GeofenceRemovalReason(pub i32);
impl GeofenceRemovalReason {
    pub const Used: Self = Self(0i32);
    pub const Expired: Self = Self(1i32);
}
impl ::core::marker::Copy for GeofenceRemovalReason {}
impl ::core::clone::Clone for GeofenceRemovalReason {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Devices_Geolocation_Geofencing\"`*"]
#[repr(transparent)]
pub struct GeofenceState(pub u32);
impl GeofenceState {
    pub const None: Self = Self(0u32);
    pub const Entered: Self = Self(1u32);
    pub const Exited: Self = Self(2u32);
    pub const Removed: Self = Self(4u32);
}
impl ::core::marker::Copy for GeofenceState {}
impl ::core::clone::Clone for GeofenceState {
    fn clone(&self) -> Self {
        *self
    }
}
pub type GeofenceStateChangeReport = *mut ::core::ffi::c_void;
#[repr(C)]
pub struct IGeofence {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub StartTime: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::Foundation::DateTime) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StartTime: usize,
    #[cfg(feature = "Foundation")]
    pub Duration: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Duration: usize,
    #[cfg(feature = "Foundation")]
    pub DwellTime: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DwellTime: usize,
    pub Id: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub MonitoredStates: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut MonitoredGeofenceStates) -> ::windows_sys::core::HRESULT,
    pub Geoshape: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SingleUse: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IGeofenceFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, id: ::windows_sys::core::HSTRING, geoshape: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateWithMonitorStates: unsafe extern "system" fn(this: *mut *mut Self, id: ::windows_sys::core::HSTRING, geoshape: *mut ::core::ffi::c_void, monitoredstates: MonitoredGeofenceStates, singleuse: bool, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub CreateWithMonitorStatesAndDwellTime: unsafe extern "system" fn(this: *mut *mut Self, id: ::windows_sys::core::HSTRING, geoshape: *mut ::core::ffi::c_void, monitoredstates: MonitoredGeofenceStates, singleuse: bool, dwelltime: super::super::super::Foundation::TimeSpan, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateWithMonitorStatesAndDwellTime: usize,
    #[cfg(feature = "Foundation")]
    pub CreateWithMonitorStatesDwellTimeStartTimeAndDuration: unsafe extern "system" fn(this: *mut *mut Self, id: ::windows_sys::core::HSTRING, geoshape: *mut ::core::ffi::c_void, monitoredstates: MonitoredGeofenceStates, singleuse: bool, dwelltime: super::super::super::Foundation::TimeSpan, starttime: super::super::super::Foundation::DateTime, duration: super::super::super::Foundation::TimeSpan, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateWithMonitorStatesDwellTimeStartTimeAndDuration: usize,
}
#[repr(C)]
pub struct IGeofenceMonitor {
    pub base__: ::windows_sys::core::IInspectable,
    pub Status: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut GeofenceMonitorStatus) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Geofences: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Geofences: usize,
    pub LastKnownGeoposition: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GeofenceStateChanged: unsafe extern "system" fn(this: *mut *mut Self, eventhandler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GeofenceStateChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveGeofenceStateChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveGeofenceStateChanged: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub ReadReports: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ReadReports: usize,
    #[cfg(feature = "Foundation")]
    pub StatusChanged: unsafe extern "system" fn(this: *mut *mut Self, eventhandler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StatusChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveStatusChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveStatusChanged: usize,
}
#[repr(C)]
pub struct IGeofenceMonitorStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub Current: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IGeofenceStateChangeReport {
    pub base__: ::windows_sys::core::IInspectable,
    pub NewState: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut GeofenceState) -> ::windows_sys::core::HRESULT,
    pub Geofence: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Geoposition: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub RemovalReason: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut GeofenceRemovalReason) -> ::windows_sys::core::HRESULT,
}
#[doc = "*Required features: `\"Devices_Geolocation_Geofencing\"`*"]
#[repr(transparent)]
pub struct MonitoredGeofenceStates(pub u32);
impl MonitoredGeofenceStates {
    pub const None: Self = Self(0u32);
    pub const Entered: Self = Self(1u32);
    pub const Exited: Self = Self(2u32);
    pub const Removed: Self = Self(4u32);
}
impl ::core::marker::Copy for MonitoredGeofenceStates {}
impl ::core::clone::Clone for MonitoredGeofenceStates {
    fn clone(&self) -> Self {
        *self
    }
}
