#[repr(C)]
pub struct IPlatformDiagnosticActionsStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsScenarioEnabled: unsafe extern "system" fn(this: *mut *mut Self, scenarioid: ::windows_sys::core::GUID, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub TryEscalateScenario: unsafe extern "system" fn(this: *mut *mut Self, scenarioid: ::windows_sys::core::GUID, escalationtype: PlatformDiagnosticEscalationType, outputdirectory: ::windows_sys::core::HSTRING, timestampoutputdirectory: bool, forceescalationupload: bool, triggers: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    TryEscalateScenario: usize,
    pub DownloadLatestSettingsForNamespace: unsafe extern "system" fn(this: *mut *mut Self, partner: ::windows_sys::core::HSTRING, feature: ::windows_sys::core::HSTRING, isscenarionamespace: bool, downloadovercostednetwork: bool, downloadoverbattery: bool, result__: *mut PlatformDiagnosticActionState) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub GetActiveScenarioList: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetActiveScenarioList: usize,
    pub ForceUpload: unsafe extern "system" fn(this: *mut *mut Self, latency: PlatformDiagnosticEventBufferLatencies, uploadovercostednetwork: bool, uploadoverbattery: bool, result__: *mut PlatformDiagnosticActionState) -> ::windows_sys::core::HRESULT,
    pub IsTraceRunning: unsafe extern "system" fn(this: *mut *mut Self, slottype: PlatformDiagnosticTraceSlotType, scenarioid: ::windows_sys::core::GUID, traceprofilehash: u64, result__: *mut PlatformDiagnosticTraceSlotState) -> ::windows_sys::core::HRESULT,
    pub GetActiveTraceRuntime: unsafe extern "system" fn(this: *mut *mut Self, slottype: PlatformDiagnosticTraceSlotType, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub GetKnownTraceList: unsafe extern "system" fn(this: *mut *mut Self, slottype: PlatformDiagnosticTraceSlotType, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetKnownTraceList: usize,
}
impl ::windows_sys::core::Interface for IPlatformDiagnosticActionsStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3239337210, data2: 37522, data3: 16999, data4: [137, 10, 158, 163, 237, 7, 35, 18] };
}
#[repr(C)]
pub struct IPlatformDiagnosticTraceInfo {
    pub base__: ::windows_sys::core::IInspectable,
    pub ScenarioId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub ProfileHash: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u64) -> ::windows_sys::core::HRESULT,
    pub IsExclusive: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsAutoLogger: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub MaxTraceDurationFileTime: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i64) -> ::windows_sys::core::HRESULT,
    pub Priority: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut PlatformDiagnosticTracePriority) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IPlatformDiagnosticTraceInfo {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4168150423, data2: 54679, data3: 19447, data4: [136, 220, 207, 92, 125, 194, 161, 210] };
}
#[repr(C)]
pub struct IPlatformDiagnosticTraceRuntimeInfo {
    pub base__: ::windows_sys::core::IInspectable,
    pub RuntimeFileTime: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i64) -> ::windows_sys::core::HRESULT,
    pub EtwRuntimeFileTime: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i64) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IPlatformDiagnosticTraceRuntimeInfo {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1028480557, data2: 472, data3: 18280, data4: [133, 84, 30, 177, 202, 97, 9, 134] };
}
#[doc = "*Required features: `\"System_Diagnostics_TraceReporting\"`*"]
#[repr(transparent)]
pub struct PlatformDiagnosticActionState(pub i32);
impl PlatformDiagnosticActionState {
    pub const Success: Self = Self(0i32);
    pub const FreeNetworkNotAvailable: Self = Self(1i32);
    pub const ACPowerNotAvailable: Self = Self(2i32);
}
impl ::core::marker::Copy for PlatformDiagnosticActionState {}
impl ::core::clone::Clone for PlatformDiagnosticActionState {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"System_Diagnostics_TraceReporting\"`*"]
#[repr(transparent)]
pub struct PlatformDiagnosticEscalationType(pub i32);
impl PlatformDiagnosticEscalationType {
    pub const OnCompletion: Self = Self(0i32);
    pub const OnFailure: Self = Self(1i32);
}
impl ::core::marker::Copy for PlatformDiagnosticEscalationType {}
impl ::core::clone::Clone for PlatformDiagnosticEscalationType {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"System_Diagnostics_TraceReporting\"`*"]
#[repr(transparent)]
pub struct PlatformDiagnosticEventBufferLatencies(pub u32);
impl PlatformDiagnosticEventBufferLatencies {
    pub const Normal: Self = Self(1u32);
    pub const CostDeferred: Self = Self(2u32);
    pub const Realtime: Self = Self(4u32);
}
impl ::core::marker::Copy for PlatformDiagnosticEventBufferLatencies {}
impl ::core::clone::Clone for PlatformDiagnosticEventBufferLatencies {
    fn clone(&self) -> Self {
        *self
    }
}
pub type PlatformDiagnosticTraceInfo = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"System_Diagnostics_TraceReporting\"`*"]
#[repr(transparent)]
pub struct PlatformDiagnosticTracePriority(pub i32);
impl PlatformDiagnosticTracePriority {
    pub const Normal: Self = Self(0i32);
    pub const UserElevated: Self = Self(1i32);
}
impl ::core::marker::Copy for PlatformDiagnosticTracePriority {}
impl ::core::clone::Clone for PlatformDiagnosticTracePriority {
    fn clone(&self) -> Self {
        *self
    }
}
pub type PlatformDiagnosticTraceRuntimeInfo = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"System_Diagnostics_TraceReporting\"`*"]
#[repr(transparent)]
pub struct PlatformDiagnosticTraceSlotState(pub i32);
impl PlatformDiagnosticTraceSlotState {
    pub const NotRunning: Self = Self(0i32);
    pub const Running: Self = Self(1i32);
    pub const Throttled: Self = Self(2i32);
}
impl ::core::marker::Copy for PlatformDiagnosticTraceSlotState {}
impl ::core::clone::Clone for PlatformDiagnosticTraceSlotState {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"System_Diagnostics_TraceReporting\"`*"]
#[repr(transparent)]
pub struct PlatformDiagnosticTraceSlotType(pub i32);
impl PlatformDiagnosticTraceSlotType {
    pub const Alternative: Self = Self(0i32);
    pub const AlwaysOn: Self = Self(1i32);
    pub const Mini: Self = Self(2i32);
}
impl ::core::marker::Copy for PlatformDiagnosticTraceSlotType {}
impl ::core::clone::Clone for PlatformDiagnosticTraceSlotType {
    fn clone(&self) -> Self {
        *self
    }
}
