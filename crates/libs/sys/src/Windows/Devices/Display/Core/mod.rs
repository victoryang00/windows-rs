pub type DisplayAdapter = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Devices_Display_Core\"`*"]
#[repr(transparent)]
pub struct DisplayBitsPerChannel(pub u32);
impl DisplayBitsPerChannel {
    pub const None: Self = Self(0u32);
    pub const Bpc6: Self = Self(1u32);
    pub const Bpc8: Self = Self(2u32);
    pub const Bpc10: Self = Self(4u32);
    pub const Bpc12: Self = Self(8u32);
    pub const Bpc14: Self = Self(16u32);
    pub const Bpc16: Self = Self(32u32);
}
impl ::core::marker::Copy for DisplayBitsPerChannel {}
impl ::core::clone::Clone for DisplayBitsPerChannel {
    fn clone(&self) -> Self {
        *self
    }
}
pub type DisplayDevice = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Devices_Display_Core\"`*"]
#[repr(transparent)]
pub struct DisplayDeviceCapability(pub i32);
impl DisplayDeviceCapability {
    pub const FlipOverride: Self = Self(0i32);
}
impl ::core::marker::Copy for DisplayDeviceCapability {}
impl ::core::clone::Clone for DisplayDeviceCapability {
    fn clone(&self) -> Self {
        *self
    }
}
pub type DisplayFence = *mut ::core::ffi::c_void;
pub type DisplayManager = *mut ::core::ffi::c_void;
pub type DisplayManagerChangedEventArgs = *mut ::core::ffi::c_void;
pub type DisplayManagerDisabledEventArgs = *mut ::core::ffi::c_void;
pub type DisplayManagerEnabledEventArgs = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Devices_Display_Core\"`*"]
#[repr(transparent)]
pub struct DisplayManagerOptions(pub u32);
impl DisplayManagerOptions {
    pub const None: Self = Self(0u32);
    pub const EnforceSourceOwnership: Self = Self(1u32);
    pub const VirtualRefreshRateAware: Self = Self(2u32);
}
impl ::core::marker::Copy for DisplayManagerOptions {}
impl ::core::clone::Clone for DisplayManagerOptions {
    fn clone(&self) -> Self {
        *self
    }
}
pub type DisplayManagerPathsFailedOrInvalidatedEventArgs = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Devices_Display_Core\"`*"]
#[repr(transparent)]
pub struct DisplayManagerResult(pub i32);
impl DisplayManagerResult {
    pub const Success: Self = Self(0i32);
    pub const UnknownFailure: Self = Self(1i32);
    pub const TargetAccessDenied: Self = Self(2i32);
    pub const TargetStale: Self = Self(3i32);
    pub const RemoteSessionNotSupported: Self = Self(4i32);
}
impl ::core::marker::Copy for DisplayManagerResult {}
impl ::core::clone::Clone for DisplayManagerResult {
    fn clone(&self) -> Self {
        *self
    }
}
pub type DisplayManagerResultWithState = *mut ::core::ffi::c_void;
pub type DisplayModeInfo = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Devices_Display_Core\"`*"]
#[repr(transparent)]
pub struct DisplayModeQueryOptions(pub u32);
impl DisplayModeQueryOptions {
    pub const None: Self = Self(0u32);
    pub const OnlyPreferredResolution: Self = Self(1u32);
}
impl ::core::marker::Copy for DisplayModeQueryOptions {}
impl ::core::clone::Clone for DisplayModeQueryOptions {
    fn clone(&self) -> Self {
        *self
    }
}
pub type DisplayPath = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Devices_Display_Core\"`*"]
#[repr(transparent)]
pub struct DisplayPathScaling(pub i32);
impl DisplayPathScaling {
    pub const Identity: Self = Self(0i32);
    pub const Centered: Self = Self(1i32);
    pub const Stretched: Self = Self(2i32);
    pub const AspectRatioStretched: Self = Self(3i32);
    pub const Custom: Self = Self(4i32);
    pub const DriverPreferred: Self = Self(5i32);
}
impl ::core::marker::Copy for DisplayPathScaling {}
impl ::core::clone::Clone for DisplayPathScaling {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Devices_Display_Core\"`*"]
#[repr(transparent)]
pub struct DisplayPathStatus(pub i32);
impl DisplayPathStatus {
    pub const Unknown: Self = Self(0i32);
    pub const Succeeded: Self = Self(1i32);
    pub const Pending: Self = Self(2i32);
    pub const Failed: Self = Self(3i32);
    pub const FailedAsync: Self = Self(4i32);
    pub const InvalidatedAsync: Self = Self(5i32);
}
impl ::core::marker::Copy for DisplayPathStatus {}
impl ::core::clone::Clone for DisplayPathStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Devices_Display_Core\"`*"]
#[repr(transparent)]
pub struct DisplayPresentStatus(pub i32);
impl DisplayPresentStatus {
    pub const Success: Self = Self(0i32);
    pub const SourceStatusPreventedPresent: Self = Self(1i32);
    pub const ScanoutInvalid: Self = Self(2i32);
    pub const SourceInvalid: Self = Self(3i32);
    pub const DeviceInvalid: Self = Self(4i32);
    pub const UnknownFailure: Self = Self(5i32);
}
impl ::core::marker::Copy for DisplayPresentStatus {}
impl ::core::clone::Clone for DisplayPresentStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Devices_Display_Core\"`, `\"Foundation_Numerics\"`*"]
#[cfg(feature = "Foundation_Numerics")]
pub struct DisplayPresentationRate {
    pub VerticalSyncRate: super::super::super::Foundation::Numerics::Rational,
    pub VerticalSyncsPerPresentation: i32,
}
#[cfg(feature = "Foundation_Numerics")]
impl ::core::marker::Copy for DisplayPresentationRate {}
#[cfg(feature = "Foundation_Numerics")]
impl ::core::clone::Clone for DisplayPresentationRate {
    fn clone(&self) -> Self {
        *self
    }
}
pub type DisplayPrimaryDescription = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Devices_Display_Core\"`*"]
#[repr(transparent)]
pub struct DisplayRotation(pub i32);
impl DisplayRotation {
    pub const None: Self = Self(0i32);
    pub const Clockwise90Degrees: Self = Self(1i32);
    pub const Clockwise180Degrees: Self = Self(2i32);
    pub const Clockwise270Degrees: Self = Self(3i32);
}
impl ::core::marker::Copy for DisplayRotation {}
impl ::core::clone::Clone for DisplayRotation {
    fn clone(&self) -> Self {
        *self
    }
}
pub type DisplayScanout = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Devices_Display_Core\"`*"]
#[repr(transparent)]
pub struct DisplayScanoutOptions(pub u32);
impl DisplayScanoutOptions {
    pub const None: Self = Self(0u32);
    pub const AllowTearing: Self = Self(2u32);
}
impl ::core::marker::Copy for DisplayScanoutOptions {}
impl ::core::clone::Clone for DisplayScanoutOptions {
    fn clone(&self) -> Self {
        *self
    }
}
pub type DisplaySource = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Devices_Display_Core\"`*"]
#[repr(transparent)]
pub struct DisplaySourceStatus(pub i32);
impl DisplaySourceStatus {
    pub const Active: Self = Self(0i32);
    pub const PoweredOff: Self = Self(1i32);
    pub const Invalid: Self = Self(2i32);
    pub const OwnedByAnotherDevice: Self = Self(3i32);
    pub const Unowned: Self = Self(4i32);
}
impl ::core::marker::Copy for DisplaySourceStatus {}
impl ::core::clone::Clone for DisplaySourceStatus {
    fn clone(&self) -> Self {
        *self
    }
}
pub type DisplayState = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Devices_Display_Core\"`*"]
#[repr(transparent)]
pub struct DisplayStateApplyOptions(pub u32);
impl DisplayStateApplyOptions {
    pub const None: Self = Self(0u32);
    pub const FailIfStateChanged: Self = Self(1u32);
    pub const ForceReapply: Self = Self(2u32);
    pub const ForceModeEnumeration: Self = Self(4u32);
}
impl ::core::marker::Copy for DisplayStateApplyOptions {}
impl ::core::clone::Clone for DisplayStateApplyOptions {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Devices_Display_Core\"`*"]
#[repr(transparent)]
pub struct DisplayStateFunctionalizeOptions(pub u32);
impl DisplayStateFunctionalizeOptions {
    pub const None: Self = Self(0u32);
    pub const FailIfStateChanged: Self = Self(1u32);
    pub const ValidateTopologyOnly: Self = Self(2u32);
}
impl ::core::marker::Copy for DisplayStateFunctionalizeOptions {}
impl ::core::clone::Clone for DisplayStateFunctionalizeOptions {
    fn clone(&self) -> Self {
        *self
    }
}
pub type DisplayStateOperationResult = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Devices_Display_Core\"`*"]
#[repr(transparent)]
pub struct DisplayStateOperationStatus(pub i32);
impl DisplayStateOperationStatus {
    pub const Success: Self = Self(0i32);
    pub const PartialFailure: Self = Self(1i32);
    pub const UnknownFailure: Self = Self(2i32);
    pub const TargetOwnershipLost: Self = Self(3i32);
    pub const SystemStateChanged: Self = Self(4i32);
    pub const TooManyPathsForAdapter: Self = Self(5i32);
    pub const ModesNotSupported: Self = Self(6i32);
    pub const RemoteSessionNotSupported: Self = Self(7i32);
}
impl ::core::marker::Copy for DisplayStateOperationStatus {}
impl ::core::clone::Clone for DisplayStateOperationStatus {
    fn clone(&self) -> Self {
        *self
    }
}
pub type DisplaySurface = *mut ::core::ffi::c_void;
pub type DisplayTarget = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Devices_Display_Core\"`*"]
#[repr(transparent)]
pub struct DisplayTargetPersistence(pub i32);
impl DisplayTargetPersistence {
    pub const None: Self = Self(0i32);
    pub const BootPersisted: Self = Self(1i32);
    pub const TemporaryPersisted: Self = Self(2i32);
    pub const PathPersisted: Self = Self(3i32);
}
impl ::core::marker::Copy for DisplayTargetPersistence {}
impl ::core::clone::Clone for DisplayTargetPersistence {
    fn clone(&self) -> Self {
        *self
    }
}
pub type DisplayTask = *mut ::core::ffi::c_void;
pub type DisplayTaskPool = *mut ::core::ffi::c_void;
pub type DisplayTaskResult = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Devices_Display_Core\"`*"]
#[repr(transparent)]
pub struct DisplayTaskSignalKind(pub i32);
impl DisplayTaskSignalKind {
    pub const OnPresentFlipAway: Self = Self(0i32);
    pub const OnPresentFlipTo: Self = Self(1i32);
}
impl ::core::marker::Copy for DisplayTaskSignalKind {}
impl ::core::clone::Clone for DisplayTaskSignalKind {
    fn clone(&self) -> Self {
        *self
    }
}
pub type DisplayView = *mut ::core::ffi::c_void;
pub type DisplayWireFormat = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Devices_Display_Core\"`*"]
#[repr(transparent)]
pub struct DisplayWireFormatColorSpace(pub i32);
impl DisplayWireFormatColorSpace {
    pub const BT709: Self = Self(0i32);
    pub const BT2020: Self = Self(1i32);
    pub const ProfileDefinedWideColorGamut: Self = Self(2i32);
}
impl ::core::marker::Copy for DisplayWireFormatColorSpace {}
impl ::core::clone::Clone for DisplayWireFormatColorSpace {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Devices_Display_Core\"`*"]
#[repr(transparent)]
pub struct DisplayWireFormatEotf(pub i32);
impl DisplayWireFormatEotf {
    pub const Sdr: Self = Self(0i32);
    pub const HdrSmpte2084: Self = Self(1i32);
}
impl ::core::marker::Copy for DisplayWireFormatEotf {}
impl ::core::clone::Clone for DisplayWireFormatEotf {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Devices_Display_Core\"`*"]
#[repr(transparent)]
pub struct DisplayWireFormatHdrMetadata(pub i32);
impl DisplayWireFormatHdrMetadata {
    pub const None: Self = Self(0i32);
    pub const Hdr10: Self = Self(1i32);
    pub const Hdr10Plus: Self = Self(2i32);
    pub const DolbyVisionLowLatency: Self = Self(3i32);
}
impl ::core::marker::Copy for DisplayWireFormatHdrMetadata {}
impl ::core::clone::Clone for DisplayWireFormatHdrMetadata {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Devices_Display_Core\"`*"]
#[repr(transparent)]
pub struct DisplayWireFormatPixelEncoding(pub i32);
impl DisplayWireFormatPixelEncoding {
    pub const Rgb444: Self = Self(0i32);
    pub const Ycc444: Self = Self(1i32);
    pub const Ycc422: Self = Self(2i32);
    pub const Ycc420: Self = Self(3i32);
    pub const Intensity: Self = Self(4i32);
}
impl ::core::marker::Copy for DisplayWireFormatPixelEncoding {}
impl ::core::clone::Clone for DisplayWireFormatPixelEncoding {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct IDisplayAdapter {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Graphics")]
    pub Id: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::Graphics::DisplayAdapterId) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Graphics"))]
    Id: usize,
    pub DeviceInterfacePath: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SourceCount: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub PciVendorId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub PciDeviceId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub PciSubSystemId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub PciRevision: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Properties: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Properties: usize,
}
#[repr(C)]
pub struct IDisplayAdapterStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Graphics")]
    pub FromId: unsafe extern "system" fn(this: *mut *mut Self, id: super::super::super::Graphics::DisplayAdapterId, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Graphics"))]
    FromId: usize,
}
#[repr(C)]
pub struct IDisplayDevice {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateScanoutSource: unsafe extern "system" fn(this: *mut *mut Self, target: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreatePrimary: unsafe extern "system" fn(this: *mut *mut Self, target: *mut ::core::ffi::c_void, desc: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateTaskPool: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub CreatePeriodicFence: unsafe extern "system" fn(this: *mut *mut Self, target: *mut ::core::ffi::c_void, offsetfromvblank: super::super::super::Foundation::TimeSpan, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreatePeriodicFence: usize,
    pub WaitForVBlank: unsafe extern "system" fn(this: *mut *mut Self, source: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateSimpleScanout: unsafe extern "system" fn(this: *mut *mut Self, psource: *mut ::core::ffi::c_void, psurface: *mut ::core::ffi::c_void, subresourceindex: u32, syncinterval: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub IsCapabilitySupported: unsafe extern "system" fn(this: *mut *mut Self, capability: DisplayDeviceCapability, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IDisplayDevice2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Foundation_Collections", feature = "Graphics"))]
    pub CreateSimpleScanoutWithDirtyRectsAndOptions: unsafe extern "system" fn(this: *mut *mut Self, source: *mut ::core::ffi::c_void, surface: *mut ::core::ffi::c_void, subresourceindex: u32, syncinterval: u32, dirtyrects: *mut ::core::ffi::c_void, options: DisplayScanoutOptions, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Graphics")))]
    CreateSimpleScanoutWithDirtyRectsAndOptions: usize,
}
#[repr(C)]
pub struct IDisplayFence {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IDisplayManager {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub GetCurrentTargets: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetCurrentTargets: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetCurrentAdapters: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetCurrentAdapters: usize,
    pub TryAcquireTarget: unsafe extern "system" fn(this: *mut *mut Self, target: *mut ::core::ffi::c_void, result__: *mut DisplayManagerResult) -> ::windows_sys::core::HRESULT,
    pub ReleaseTarget: unsafe extern "system" fn(this: *mut *mut Self, target: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub TryReadCurrentStateForAllTargets: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub TryAcquireTargetsAndReadCurrentState: unsafe extern "system" fn(this: *mut *mut Self, targets: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    TryAcquireTargetsAndReadCurrentState: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub TryAcquireTargetsAndCreateEmptyState: unsafe extern "system" fn(this: *mut *mut Self, targets: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    TryAcquireTargetsAndCreateEmptyState: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub TryAcquireTargetsAndCreateSubstate: unsafe extern "system" fn(this: *mut *mut Self, existingstate: *mut ::core::ffi::c_void, targets: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    TryAcquireTargetsAndCreateSubstate: usize,
    pub CreateDisplayDevice: unsafe extern "system" fn(this: *mut *mut Self, adapter: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Enabled: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Enabled: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveEnabled: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveEnabled: usize,
    #[cfg(feature = "Foundation")]
    pub Disabled: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Disabled: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveDisabled: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveDisabled: usize,
    #[cfg(feature = "Foundation")]
    pub Changed: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Changed: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveChanged: usize,
    #[cfg(feature = "Foundation")]
    pub PathsFailedOrInvalidated: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PathsFailedOrInvalidated: usize,
    #[cfg(feature = "Foundation")]
    pub RemovePathsFailedOrInvalidated: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemovePathsFailedOrInvalidated: usize,
    pub Start: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Stop: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IDisplayManagerChangedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Handled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetDeferral: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDeferral: usize,
}
#[repr(C)]
pub struct IDisplayManagerDisabledEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Handled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetDeferral: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDeferral: usize,
}
#[repr(C)]
pub struct IDisplayManagerEnabledEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Handled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetDeferral: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDeferral: usize,
}
#[repr(C)]
pub struct IDisplayManagerPathsFailedOrInvalidatedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Handled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetDeferral: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDeferral: usize,
}
#[repr(C)]
pub struct IDisplayManagerResultWithState {
    pub base__: ::windows_sys::core::IInspectable,
    pub ErrorCode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut DisplayManagerResult) -> ::windows_sys::core::HRESULT,
    pub ExtendedErrorCode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HRESULT) -> ::windows_sys::core::HRESULT,
    pub State: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IDisplayManagerStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, options: DisplayManagerOptions, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IDisplayModeInfo {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Graphics")]
    pub SourceResolution: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::Graphics::SizeInt32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Graphics"))]
    SourceResolution: usize,
    pub IsStereo: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Graphics_DirectX")]
    pub SourcePixelFormat: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::Graphics::DirectX::DirectXPixelFormat) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Graphics_DirectX"))]
    SourcePixelFormat: usize,
    #[cfg(feature = "Graphics")]
    pub TargetResolution: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::Graphics::SizeInt32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Graphics"))]
    TargetResolution: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub PresentationRate: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut DisplayPresentationRate) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    PresentationRate: usize,
    pub IsInterlaced: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub GetWireFormatSupportedBitsPerChannel: unsafe extern "system" fn(this: *mut *mut Self, encoding: DisplayWireFormatPixelEncoding, result__: *mut DisplayBitsPerChannel) -> ::windows_sys::core::HRESULT,
    pub IsWireFormatSupported: unsafe extern "system" fn(this: *mut *mut Self, wireformat: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Properties: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Properties: usize,
}
#[repr(C)]
pub struct IDisplayModeInfo2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Numerics")]
    pub PhysicalPresentationRate: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut DisplayPresentationRate) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    PhysicalPresentationRate: usize,
}
#[repr(C)]
pub struct IDisplayPath {
    pub base__: ::windows_sys::core::IInspectable,
    pub View: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Target: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Status: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut DisplayPathStatus) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Graphics"))]
    pub SourceResolution: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Graphics")))]
    SourceResolution: usize,
    #[cfg(all(feature = "Foundation", feature = "Graphics"))]
    pub SetSourceResolution: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Graphics")))]
    SetSourceResolution: usize,
    #[cfg(feature = "Graphics_DirectX")]
    pub SourcePixelFormat: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::Graphics::DirectX::DirectXPixelFormat) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Graphics_DirectX"))]
    SourcePixelFormat: usize,
    #[cfg(feature = "Graphics_DirectX")]
    pub SetSourcePixelFormat: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::super::Graphics::DirectX::DirectXPixelFormat) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Graphics_DirectX"))]
    SetSourcePixelFormat: usize,
    pub IsStereo: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsStereo: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Graphics"))]
    pub TargetResolution: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Graphics")))]
    TargetResolution: usize,
    #[cfg(all(feature = "Foundation", feature = "Graphics"))]
    pub SetTargetResolution: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Graphics")))]
    SetTargetResolution: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub PresentationRate: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    PresentationRate: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub SetPresentationRate: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    SetPresentationRate: usize,
    #[cfg(feature = "Foundation")]
    pub IsInterlaced: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    IsInterlaced: usize,
    #[cfg(feature = "Foundation")]
    pub SetIsInterlaced: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetIsInterlaced: usize,
    pub WireFormat: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetWireFormat: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Rotation: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut DisplayRotation) -> ::windows_sys::core::HRESULT,
    pub SetRotation: unsafe extern "system" fn(this: *mut *mut Self, value: DisplayRotation) -> ::windows_sys::core::HRESULT,
    pub Scaling: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut DisplayPathScaling) -> ::windows_sys::core::HRESULT,
    pub SetScaling: unsafe extern "system" fn(this: *mut *mut Self, value: DisplayPathScaling) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub FindModes: unsafe extern "system" fn(this: *mut *mut Self, flags: DisplayModeQueryOptions, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FindModes: usize,
    pub ApplyPropertiesFromMode: unsafe extern "system" fn(this: *mut *mut Self, moderesult: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Properties: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Properties: usize,
}
#[repr(C)]
pub struct IDisplayPath2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Numerics")]
    pub PhysicalPresentationRate: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    PhysicalPresentationRate: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub SetPhysicalPresentationRate: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    SetPhysicalPresentationRate: usize,
}
#[repr(C)]
pub struct IDisplayPrimaryDescription {
    pub base__: ::windows_sys::core::IInspectable,
    pub Width: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub Height: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Graphics_DirectX")]
    pub Format: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::Graphics::DirectX::DirectXPixelFormat) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Graphics_DirectX"))]
    Format: usize,
    #[cfg(feature = "Graphics_DirectX")]
    pub ColorSpace: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::Graphics::DirectX::DirectXColorSpace) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Graphics_DirectX"))]
    ColorSpace: usize,
    pub IsStereo: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Graphics_DirectX_Direct3D11")]
    pub MultisampleDescription: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::Graphics::DirectX::Direct3D11::Direct3DMultisampleDescription) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Graphics_DirectX_Direct3D11"))]
    MultisampleDescription: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Properties: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Properties: usize,
}
#[repr(C)]
pub struct IDisplayPrimaryDescriptionFactory {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Graphics_DirectX_Direct3D11")]
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, width: u32, height: u32, pixelformat: super::super::super::Graphics::DirectX::DirectXPixelFormat, colorspace: super::super::super::Graphics::DirectX::DirectXColorSpace, isstereo: bool, multisampledescription: super::super::super::Graphics::DirectX::Direct3D11::Direct3DMultisampleDescription, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Graphics_DirectX_Direct3D11"))]
    CreateInstance: usize,
}
#[repr(C)]
pub struct IDisplayPrimaryDescriptionStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Foundation_Collections", feature = "Graphics_DirectX_Direct3D11"))]
    pub CreateWithProperties: unsafe extern "system" fn(this: *mut *mut Self, extraproperties: *mut ::core::ffi::c_void, width: u32, height: u32, pixelformat: super::super::super::Graphics::DirectX::DirectXPixelFormat, colorspace: super::super::super::Graphics::DirectX::DirectXColorSpace, isstereo: bool, multisampledescription: super::super::super::Graphics::DirectX::Direct3D11::Direct3DMultisampleDescription, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Graphics_DirectX_Direct3D11")))]
    CreateWithProperties: usize,
}
#[repr(C)]
pub struct IDisplayScanout {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IDisplaySource {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Graphics")]
    pub AdapterId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::super::Graphics::DisplayAdapterId) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Graphics"))]
    AdapterId: usize,
    pub SourceId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub GetMetadata: unsafe extern "system" fn(this: *mut *mut Self, key: ::windows_sys::core::GUID, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    GetMetadata: usize,
}
#[repr(C)]
pub struct IDisplaySource2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub Status: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut DisplaySourceStatus) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub StatusChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StatusChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveStatusChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveStatusChanged: usize,
}
#[repr(C)]
pub struct IDisplayState {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsReadOnly: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsStale: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Targets: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Targets: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Views: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Views: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Properties: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Properties: usize,
    pub ConnectTarget: unsafe extern "system" fn(this: *mut *mut Self, target: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ConnectTargetToView: unsafe extern "system" fn(this: *mut *mut Self, target: *mut ::core::ffi::c_void, view: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CanConnectTargetToView: unsafe extern "system" fn(this: *mut *mut Self, target: *mut ::core::ffi::c_void, view: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub GetViewForTarget: unsafe extern "system" fn(this: *mut *mut Self, target: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetPathForTarget: unsafe extern "system" fn(this: *mut *mut Self, target: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub DisconnectTarget: unsafe extern "system" fn(this: *mut *mut Self, target: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub TryFunctionalize: unsafe extern "system" fn(this: *mut *mut Self, options: DisplayStateFunctionalizeOptions, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub TryApply: unsafe extern "system" fn(this: *mut *mut Self, options: DisplayStateApplyOptions, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IDisplayStateOperationResult {
    pub base__: ::windows_sys::core::IInspectable,
    pub Status: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut DisplayStateOperationStatus) -> ::windows_sys::core::HRESULT,
    pub ExtendedErrorCode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HRESULT) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IDisplaySurface {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IDisplayTarget {
    pub base__: ::windows_sys::core::IInspectable,
    pub Adapter: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub DeviceInterfacePath: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub AdapterRelativeId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub IsConnected: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsVirtualModeEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsVirtualTopologyEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub UsageKind: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::DisplayMonitorUsageKind) -> ::windows_sys::core::HRESULT,
    pub MonitorPersistence: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut DisplayTargetPersistence) -> ::windows_sys::core::HRESULT,
    pub StableMonitorId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub TryGetMonitor: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Properties: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Properties: usize,
    pub IsStale: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsSame: unsafe extern "system" fn(this: *mut *mut Self, othertarget: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsEqual: unsafe extern "system" fn(this: *mut *mut Self, othertarget: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IDisplayTask {
    pub base__: ::windows_sys::core::IInspectable,
    pub SetScanout: unsafe extern "system" fn(this: *mut *mut Self, scanout: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetWait: unsafe extern "system" fn(this: *mut *mut Self, readyfence: *mut ::core::ffi::c_void, readyfencevalue: u64) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IDisplayTask2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub SetSignal: unsafe extern "system" fn(this: *mut *mut Self, signalkind: DisplayTaskSignalKind, fence: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IDisplayTaskPool {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateTask: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "deprecated")]
    pub ExecuteTask: unsafe extern "system" fn(this: *mut *mut Self, task: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    ExecuteTask: usize,
}
#[repr(C)]
pub struct IDisplayTaskPool2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub TryExecuteTask: unsafe extern "system" fn(this: *mut *mut Self, task: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IDisplayTaskResult {
    pub base__: ::windows_sys::core::IInspectable,
    pub PresentStatus: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut DisplayPresentStatus) -> ::windows_sys::core::HRESULT,
    pub PresentId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u64) -> ::windows_sys::core::HRESULT,
    pub SourceStatus: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut DisplaySourceStatus) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IDisplayView {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub Paths: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Paths: usize,
    #[cfg(all(feature = "Foundation", feature = "Graphics"))]
    pub ContentResolution: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Graphics")))]
    ContentResolution: usize,
    #[cfg(all(feature = "Foundation", feature = "Graphics"))]
    pub SetContentResolution: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Graphics")))]
    SetContentResolution: usize,
    pub SetPrimaryPath: unsafe extern "system" fn(this: *mut *mut Self, path: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Properties: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Properties: usize,
}
#[repr(C)]
pub struct IDisplayWireFormat {
    pub base__: ::windows_sys::core::IInspectable,
    pub PixelEncoding: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut DisplayWireFormatPixelEncoding) -> ::windows_sys::core::HRESULT,
    pub BitsPerChannel: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub ColorSpace: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut DisplayWireFormatColorSpace) -> ::windows_sys::core::HRESULT,
    pub Eotf: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut DisplayWireFormatEotf) -> ::windows_sys::core::HRESULT,
    pub HdrMetadata: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut DisplayWireFormatHdrMetadata) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Properties: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Properties: usize,
}
#[repr(C)]
pub struct IDisplayWireFormatFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, pixelencoding: DisplayWireFormatPixelEncoding, bitsperchannel: i32, colorspace: DisplayWireFormatColorSpace, eotf: DisplayWireFormatEotf, hdrmetadata: DisplayWireFormatHdrMetadata, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IDisplayWireFormatStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateWithProperties: unsafe extern "system" fn(this: *mut *mut Self, extraproperties: *mut ::core::ffi::c_void, pixelencoding: DisplayWireFormatPixelEncoding, bitsperchannel: i32, colorspace: DisplayWireFormatColorSpace, eotf: DisplayWireFormatEotf, hdrmetadata: DisplayWireFormatHdrMetadata, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateWithProperties: usize,
}
