#[cfg(feature = "Media_Protection_PlayReady")]
pub mod PlayReady;
pub type ComponentLoadFailedEventArgs = *mut ::core::ffi::c_void;
pub type ComponentLoadFailedEventHandler = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Media_Protection\"`*"]
#[repr(transparent)]
pub struct GraphicsTrustStatus(pub i32);
impl GraphicsTrustStatus {
    pub const TrustNotRequired: Self = Self(0i32);
    pub const TrustEstablished: Self = Self(1i32);
    pub const EnvironmentNotSupported: Self = Self(2i32);
    pub const DriverNotSupported: Self = Self(3i32);
    pub const DriverSigningFailure: Self = Self(4i32);
    pub const UnknownFailure: Self = Self(5i32);
}
impl ::core::marker::Copy for GraphicsTrustStatus {}
impl ::core::clone::Clone for GraphicsTrustStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Media_Protection\"`*"]
#[repr(transparent)]
pub struct HdcpProtection(pub i32);
impl HdcpProtection {
    pub const Off: Self = Self(0i32);
    pub const On: Self = Self(1i32);
    pub const OnWithTypeEnforcement: Self = Self(2i32);
}
impl ::core::marker::Copy for HdcpProtection {}
impl ::core::clone::Clone for HdcpProtection {
    fn clone(&self) -> Self {
        *self
    }
}
pub type HdcpSession = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Media_Protection\"`*"]
#[repr(transparent)]
pub struct HdcpSetProtectionResult(pub i32);
impl HdcpSetProtectionResult {
    pub const Success: Self = Self(0i32);
    pub const TimedOut: Self = Self(1i32);
    pub const NotSupported: Self = Self(2i32);
    pub const UnknownFailure: Self = Self(3i32);
}
impl ::core::marker::Copy for HdcpSetProtectionResult {}
impl ::core::clone::Clone for HdcpSetProtectionResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct IComponentLoadFailedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Information: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Completion: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IComponentRenewalStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub RenewSystemComponentsAsync: unsafe extern "system" fn(this: *mut *mut Self, information: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RenewSystemComponentsAsync: usize,
}
#[repr(C)]
pub struct IHdcpSession {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsEffectiveProtectionAtLeast: unsafe extern "system" fn(this: *mut *mut Self, protection: HdcpProtection, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetEffectiveProtection: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetEffectiveProtection: usize,
    #[cfg(feature = "Foundation")]
    pub SetDesiredMinProtectionAsync: unsafe extern "system" fn(this: *mut *mut Self, protection: HdcpProtection, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetDesiredMinProtectionAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ProtectionChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ProtectionChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveProtectionChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveProtectionChanged: usize,
}
#[repr(C)]
pub struct IMediaProtectionManager {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub ServiceRequested: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ServiceRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveServiceRequested: unsafe extern "system" fn(this: *mut *mut Self, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveServiceRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RebootNeeded: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RebootNeeded: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveRebootNeeded: unsafe extern "system" fn(this: *mut *mut Self, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveRebootNeeded: usize,
    #[cfg(feature = "Foundation")]
    pub ComponentLoadFailed: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ComponentLoadFailed: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveComponentLoadFailed: unsafe extern "system" fn(this: *mut *mut Self, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveComponentLoadFailed: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Properties: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Properties: usize,
}
#[repr(C)]
pub struct IMediaProtectionPMPServer {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub Properties: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Properties: usize,
}
#[repr(C)]
pub struct IMediaProtectionPMPServerFactory {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub CreatePMPServer: unsafe extern "system" fn(this: *mut *mut Self, pproperties: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreatePMPServer: usize,
}
#[repr(C)]
pub struct IMediaProtectionServiceCompletion {
    pub base__: ::windows_sys::core::IInspectable,
    pub Complete: unsafe extern "system" fn(this: *mut *mut Self, success: bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IMediaProtectionServiceRequest {
    pub base__: ::windows_sys::core::IInspectable,
    pub ProtectionSystem: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub Type: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IProtectionCapabilities {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsTypeSupported: unsafe extern "system" fn(this: *mut *mut Self, r#type: ::windows_sys::core::HSTRING, keysystem: ::windows_sys::core::HSTRING, result__: *mut ProtectionCapabilityResult) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IRevocationAndRenewalInformation {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub Items: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Items: usize,
}
#[repr(C)]
pub struct IRevocationAndRenewalItem {
    pub base__: ::windows_sys::core::IInspectable,
    pub Reasons: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut RevocationAndRenewalReasons) -> ::windows_sys::core::HRESULT,
    pub HeaderHash: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub PublicKeyHash: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Name: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub RenewalId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IServiceRequestedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Request: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Completion: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IServiceRequestedEventArgs2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Media_Playback")]
    pub MediaPlaybackItem: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Media_Playback"))]
    MediaPlaybackItem: usize,
}
pub type MediaProtectionManager = *mut ::core::ffi::c_void;
pub type MediaProtectionPMPServer = *mut ::core::ffi::c_void;
pub type MediaProtectionServiceCompletion = *mut ::core::ffi::c_void;
pub type ProtectionCapabilities = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Media_Protection\"`*"]
#[repr(transparent)]
pub struct ProtectionCapabilityResult(pub i32);
impl ProtectionCapabilityResult {
    pub const NotSupported: Self = Self(0i32);
    pub const Maybe: Self = Self(1i32);
    pub const Probably: Self = Self(2i32);
}
impl ::core::marker::Copy for ProtectionCapabilityResult {}
impl ::core::clone::Clone for ProtectionCapabilityResult {
    fn clone(&self) -> Self {
        *self
    }
}
pub type RebootNeededEventHandler = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Media_Protection\"`*"]
#[repr(transparent)]
pub struct RenewalStatus(pub i32);
impl RenewalStatus {
    pub const NotStarted: Self = Self(0i32);
    pub const UpdatesInProgress: Self = Self(1i32);
    pub const UserCancelled: Self = Self(2i32);
    pub const AppComponentsMayNeedUpdating: Self = Self(3i32);
    pub const NoComponentsFound: Self = Self(4i32);
}
impl ::core::marker::Copy for RenewalStatus {}
impl ::core::clone::Clone for RenewalStatus {
    fn clone(&self) -> Self {
        *self
    }
}
pub type RevocationAndRenewalInformation = *mut ::core::ffi::c_void;
pub type RevocationAndRenewalItem = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Media_Protection\"`*"]
#[repr(transparent)]
pub struct RevocationAndRenewalReasons(pub u32);
impl RevocationAndRenewalReasons {
    pub const UserModeComponentLoad: Self = Self(1u32);
    pub const KernelModeComponentLoad: Self = Self(2u32);
    pub const AppComponent: Self = Self(4u32);
    pub const GlobalRevocationListLoadFailed: Self = Self(16u32);
    pub const InvalidGlobalRevocationListSignature: Self = Self(32u32);
    pub const GlobalRevocationListAbsent: Self = Self(4096u32);
    pub const ComponentRevoked: Self = Self(8192u32);
    pub const InvalidComponentCertificateExtendedKeyUse: Self = Self(16384u32);
    pub const ComponentCertificateRevoked: Self = Self(32768u32);
    pub const InvalidComponentCertificateRoot: Self = Self(65536u32);
    pub const ComponentHighSecurityCertificateRevoked: Self = Self(131072u32);
    pub const ComponentLowSecurityCertificateRevoked: Self = Self(262144u32);
    pub const BootDriverVerificationFailed: Self = Self(1048576u32);
    pub const ComponentSignedWithTestCertificate: Self = Self(16777216u32);
    pub const EncryptionFailure: Self = Self(268435456u32);
}
impl ::core::marker::Copy for RevocationAndRenewalReasons {}
impl ::core::clone::Clone for RevocationAndRenewalReasons {
    fn clone(&self) -> Self {
        *self
    }
}
pub type ServiceRequestedEventArgs = *mut ::core::ffi::c_void;
pub type ServiceRequestedEventHandler = *mut ::core::ffi::c_void;
