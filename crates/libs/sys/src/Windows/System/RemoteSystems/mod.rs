#[repr(C)]
pub struct IKnownRemoteSystemCapabilitiesStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub AppService: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub LaunchUri: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub RemoteSession: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SpatialEntity: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IRemoteSystem {
    pub base__: ::windows_sys::core::IInspectable,
    pub DisplayName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Id: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Kind: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Status: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut RemoteSystemStatus) -> ::windows_sys::core::HRESULT,
    pub IsAvailableByProximity: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IRemoteSystem2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsAvailableBySpatialProximity: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetCapabilitySupportedAsync: unsafe extern "system" fn(this: *mut *mut Self, capabilityname: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetCapabilitySupportedAsync: usize,
}
#[repr(C)]
pub struct IRemoteSystem3 {
    pub base__: ::windows_sys::core::IInspectable,
    pub ManufacturerDisplayName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub ModelDisplayName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IRemoteSystem4 {
    pub base__: ::windows_sys::core::IInspectable,
    pub Platform: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut RemoteSystemPlatform) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IRemoteSystem5 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub Apps: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Apps: usize,
}
#[repr(C)]
pub struct IRemoteSystem6 {
    pub base__: ::windows_sys::core::IInspectable,
    pub User: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IRemoteSystemAddedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub RemoteSystem: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IRemoteSystemApp {
    pub base__: ::windows_sys::core::IInspectable,
    pub Id: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub DisplayName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub IsAvailableByProximity: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsAvailableBySpatialProximity: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Attributes: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Attributes: usize,
}
#[repr(C)]
pub struct IRemoteSystemApp2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub User: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ConnectionToken: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IRemoteSystemAppRegistration {
    pub base__: ::windows_sys::core::IInspectable,
    pub User: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Attributes: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Attributes: usize,
    #[cfg(feature = "Foundation")]
    pub SaveAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SaveAsync: usize,
}
#[repr(C)]
pub struct IRemoteSystemAppRegistrationStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetDefault: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetForUser: unsafe extern "system" fn(this: *mut *mut Self, user: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IRemoteSystemAuthorizationKindFilter {
    pub base__: ::windows_sys::core::IInspectable,
    pub RemoteSystemAuthorizationKind: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut RemoteSystemAuthorizationKind) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IRemoteSystemAuthorizationKindFilterFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, remotesystemauthorizationkind: RemoteSystemAuthorizationKind, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IRemoteSystemConnectionInfo {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsProximal: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IRemoteSystemConnectionInfoStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "ApplicationModel_AppService")]
    pub TryCreateFromAppServiceConnection: unsafe extern "system" fn(this: *mut *mut Self, connection: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_AppService"))]
    TryCreateFromAppServiceConnection: usize,
}
#[repr(C)]
pub struct IRemoteSystemConnectionRequest {
    pub base__: ::windows_sys::core::IInspectable,
    pub RemoteSystem: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IRemoteSystemConnectionRequest2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub RemoteSystemApp: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IRemoteSystemConnectionRequest3 {
    pub base__: ::windows_sys::core::IInspectable,
    pub ConnectionToken: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IRemoteSystemConnectionRequestFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, remotesystem: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IRemoteSystemConnectionRequestStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateForApp: unsafe extern "system" fn(this: *mut *mut Self, remotesystemapp: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IRemoteSystemConnectionRequestStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateFromConnectionToken: unsafe extern "system" fn(this: *mut *mut Self, connectiontoken: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateFromConnectionTokenForUser: unsafe extern "system" fn(this: *mut *mut Self, user: *mut ::core::ffi::c_void, connectiontoken: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IRemoteSystemDiscoveryTypeFilter {
    pub base__: ::windows_sys::core::IInspectable,
    pub RemoteSystemDiscoveryType: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut RemoteSystemDiscoveryType) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IRemoteSystemDiscoveryTypeFilterFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, discoverytype: RemoteSystemDiscoveryType, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IRemoteSystemEnumerationCompletedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IRemoteSystemFilter {
    pub base__: ::windows_sys::core::IInspectable,
}
#[repr(C)]
pub struct IRemoteSystemKindFilter {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub RemoteSystemKinds: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    RemoteSystemKinds: usize,
}
#[repr(C)]
pub struct IRemoteSystemKindFilterFactory {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, remotesystemkinds: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Create: usize,
}
#[repr(C)]
pub struct IRemoteSystemKindStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub Phone: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Hub: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Holographic: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Desktop: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Xbox: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IRemoteSystemKindStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub Iot: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Tablet: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Laptop: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IRemoteSystemRemovedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub RemoteSystemId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IRemoteSystemSession {
    pub base__: ::windows_sys::core::IInspectable,
    pub Id: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub DisplayName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub ControllerDisplayName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Disconnected: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Disconnected: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveDisconnected: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveDisconnected: usize,
    pub CreateParticipantWatcher: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub SendInvitationAsync: unsafe extern "system" fn(this: *mut *mut Self, invitee: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SendInvitationAsync: usize,
}
#[repr(C)]
pub struct IRemoteSystemSessionAddedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub SessionInfo: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IRemoteSystemSessionController {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub JoinRequested: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    JoinRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveJoinRequested: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveJoinRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveParticipantAsync: unsafe extern "system" fn(this: *mut *mut Self, pparticipant: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveParticipantAsync: usize,
    #[cfg(feature = "Foundation")]
    pub CreateSessionAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateSessionAsync: usize,
}
#[repr(C)]
pub struct IRemoteSystemSessionControllerFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateController: unsafe extern "system" fn(this: *mut *mut Self, displayname: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateControllerWithSessionOptions: unsafe extern "system" fn(this: *mut *mut Self, displayname: ::windows_sys::core::HSTRING, options: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IRemoteSystemSessionCreationResult {
    pub base__: ::windows_sys::core::IInspectable,
    pub Status: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut RemoteSystemSessionCreationStatus) -> ::windows_sys::core::HRESULT,
    pub Session: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IRemoteSystemSessionDisconnectedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Reason: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut RemoteSystemSessionDisconnectedReason) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IRemoteSystemSessionInfo {
    pub base__: ::windows_sys::core::IInspectable,
    pub DisplayName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub ControllerDisplayName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub JoinAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    JoinAsync: usize,
}
#[repr(C)]
pub struct IRemoteSystemSessionInvitation {
    pub base__: ::windows_sys::core::IInspectable,
    pub Sender: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SessionInfo: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IRemoteSystemSessionInvitationListener {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub InvitationReceived: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    InvitationReceived: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveInvitationReceived: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveInvitationReceived: usize,
}
#[repr(C)]
pub struct IRemoteSystemSessionInvitationReceivedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Invitation: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IRemoteSystemSessionJoinRequest {
    pub base__: ::windows_sys::core::IInspectable,
    pub Participant: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Accept: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IRemoteSystemSessionJoinRequestedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub JoinRequest: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetDeferral: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDeferral: usize,
}
#[repr(C)]
pub struct IRemoteSystemSessionJoinResult {
    pub base__: ::windows_sys::core::IInspectable,
    pub Status: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut RemoteSystemSessionJoinStatus) -> ::windows_sys::core::HRESULT,
    pub Session: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IRemoteSystemSessionMessageChannel {
    pub base__: ::windows_sys::core::IInspectable,
    pub Session: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub BroadcastValueSetAsync: unsafe extern "system" fn(this: *mut *mut Self, messagedata: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    BroadcastValueSetAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub SendValueSetAsync: unsafe extern "system" fn(this: *mut *mut Self, messagedata: *mut ::core::ffi::c_void, participant: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SendValueSetAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub SendValueSetToParticipantsAsync: unsafe extern "system" fn(this: *mut *mut Self, messagedata: *mut ::core::ffi::c_void, participants: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SendValueSetToParticipantsAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ValueSetReceived: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ValueSetReceived: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveValueSetReceived: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveValueSetReceived: usize,
}
#[repr(C)]
pub struct IRemoteSystemSessionMessageChannelFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, session: *mut ::core::ffi::c_void, channelname: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateWithReliability: unsafe extern "system" fn(this: *mut *mut Self, session: *mut ::core::ffi::c_void, channelname: ::windows_sys::core::HSTRING, reliability: RemoteSystemSessionMessageChannelReliability, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IRemoteSystemSessionOptions {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsInviteOnly: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsInviteOnly: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IRemoteSystemSessionParticipant {
    pub base__: ::windows_sys::core::IInspectable,
    pub RemoteSystem: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(all(feature = "Foundation_Collections", feature = "Networking"))]
    pub GetHostNames: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Networking")))]
    GetHostNames: usize,
}
#[repr(C)]
pub struct IRemoteSystemSessionParticipantAddedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Participant: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IRemoteSystemSessionParticipantRemovedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Participant: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IRemoteSystemSessionParticipantWatcher {
    pub base__: ::windows_sys::core::IInspectable,
    pub Start: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Stop: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Status: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut RemoteSystemSessionParticipantWatcherStatus) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Added: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Added: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveAdded: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveAdded: usize,
    #[cfg(feature = "Foundation")]
    pub Removed: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Removed: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveRemoved: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveRemoved: usize,
    #[cfg(feature = "Foundation")]
    pub EnumerationCompleted: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    EnumerationCompleted: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveEnumerationCompleted: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveEnumerationCompleted: usize,
}
#[repr(C)]
pub struct IRemoteSystemSessionRemovedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub SessionInfo: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IRemoteSystemSessionStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateWatcher: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IRemoteSystemSessionUpdatedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub SessionInfo: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IRemoteSystemSessionValueSetReceivedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Sender: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Message: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Message: usize,
}
#[repr(C)]
pub struct IRemoteSystemSessionWatcher {
    pub base__: ::windows_sys::core::IInspectable,
    pub Start: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Stop: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Status: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut RemoteSystemSessionWatcherStatus) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Added: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Added: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveAdded: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveAdded: usize,
    #[cfg(feature = "Foundation")]
    pub Updated: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Updated: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveUpdated: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveUpdated: usize,
    #[cfg(feature = "Foundation")]
    pub Removed: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Removed: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveRemoved: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveRemoved: usize,
}
#[repr(C)]
pub struct IRemoteSystemStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Foundation", feature = "Networking"))]
    pub FindByHostNameAsync: unsafe extern "system" fn(this: *mut *mut Self, hostname: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Networking")))]
    FindByHostNameAsync: usize,
    pub CreateWatcher: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateWatcherWithFilters: unsafe extern "system" fn(this: *mut *mut Self, filters: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateWatcherWithFilters: usize,
    #[cfg(feature = "Foundation")]
    pub RequestAccessAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestAccessAsync: usize,
}
#[repr(C)]
pub struct IRemoteSystemStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsAuthorizationKindEnabled: unsafe extern "system" fn(this: *mut *mut Self, kind: RemoteSystemAuthorizationKind, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IRemoteSystemStatics3 {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateWatcherForUser: unsafe extern "system" fn(this: *mut *mut Self, user: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateWatcherWithFiltersForUser: unsafe extern "system" fn(this: *mut *mut Self, user: *mut ::core::ffi::c_void, filters: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateWatcherWithFiltersForUser: usize,
}
#[repr(C)]
pub struct IRemoteSystemStatusTypeFilter {
    pub base__: ::windows_sys::core::IInspectable,
    pub RemoteSystemStatusType: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut RemoteSystemStatusType) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IRemoteSystemStatusTypeFilterFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, remotesystemstatustype: RemoteSystemStatusType, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IRemoteSystemUpdatedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub RemoteSystem: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IRemoteSystemWatcher {
    pub base__: ::windows_sys::core::IInspectable,
    pub Start: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Stop: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub RemoteSystemAdded: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoteSystemAdded: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveRemoteSystemAdded: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveRemoteSystemAdded: usize,
    #[cfg(feature = "Foundation")]
    pub RemoteSystemUpdated: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoteSystemUpdated: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveRemoteSystemUpdated: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveRemoteSystemUpdated: usize,
    #[cfg(feature = "Foundation")]
    pub RemoteSystemRemoved: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoteSystemRemoved: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveRemoteSystemRemoved: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveRemoteSystemRemoved: usize,
}
#[repr(C)]
pub struct IRemoteSystemWatcher2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation")]
    pub EnumerationCompleted: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    EnumerationCompleted: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveEnumerationCompleted: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveEnumerationCompleted: usize,
    #[cfg(feature = "Foundation")]
    pub ErrorOccurred: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ErrorOccurred: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveErrorOccurred: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveErrorOccurred: usize,
}
#[repr(C)]
pub struct IRemoteSystemWatcher3 {
    pub base__: ::windows_sys::core::IInspectable,
    pub User: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IRemoteSystemWatcherErrorOccurredEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Error: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut RemoteSystemWatcherError) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IRemoteSystemWebAccountFilter {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Security_Credentials")]
    pub Account: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))]
    Account: usize,
}
#[repr(C)]
pub struct IRemoteSystemWebAccountFilterFactory {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Security_Credentials")]
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, account: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))]
    Create: usize,
}
pub type RemoteSystem = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"System_RemoteSystems\"`*"]
#[repr(transparent)]
pub struct RemoteSystemAccessStatus(pub i32);
impl RemoteSystemAccessStatus {
    pub const Unspecified: Self = Self(0i32);
    pub const Allowed: Self = Self(1i32);
    pub const DeniedByUser: Self = Self(2i32);
    pub const DeniedBySystem: Self = Self(3i32);
}
impl ::core::marker::Copy for RemoteSystemAccessStatus {}
impl ::core::clone::Clone for RemoteSystemAccessStatus {
    fn clone(&self) -> Self {
        *self
    }
}
pub type RemoteSystemAddedEventArgs = *mut ::core::ffi::c_void;
pub type RemoteSystemApp = *mut ::core::ffi::c_void;
pub type RemoteSystemAppRegistration = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"System_RemoteSystems\"`*"]
#[repr(transparent)]
pub struct RemoteSystemAuthorizationKind(pub i32);
impl RemoteSystemAuthorizationKind {
    pub const SameUser: Self = Self(0i32);
    pub const Anonymous: Self = Self(1i32);
}
impl ::core::marker::Copy for RemoteSystemAuthorizationKind {}
impl ::core::clone::Clone for RemoteSystemAuthorizationKind {
    fn clone(&self) -> Self {
        *self
    }
}
pub type RemoteSystemAuthorizationKindFilter = *mut ::core::ffi::c_void;
pub type RemoteSystemConnectionInfo = *mut ::core::ffi::c_void;
pub type RemoteSystemConnectionRequest = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"System_RemoteSystems\"`*"]
#[repr(transparent)]
pub struct RemoteSystemDiscoveryType(pub i32);
impl RemoteSystemDiscoveryType {
    pub const Any: Self = Self(0i32);
    pub const Proximal: Self = Self(1i32);
    pub const Cloud: Self = Self(2i32);
    pub const SpatiallyProximal: Self = Self(3i32);
}
impl ::core::marker::Copy for RemoteSystemDiscoveryType {}
impl ::core::clone::Clone for RemoteSystemDiscoveryType {
    fn clone(&self) -> Self {
        *self
    }
}
pub type RemoteSystemDiscoveryTypeFilter = *mut ::core::ffi::c_void;
pub type RemoteSystemEnumerationCompletedEventArgs = *mut ::core::ffi::c_void;
pub type RemoteSystemKindFilter = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"System_RemoteSystems\"`*"]
#[repr(transparent)]
pub struct RemoteSystemPlatform(pub i32);
impl RemoteSystemPlatform {
    pub const Unknown: Self = Self(0i32);
    pub const Windows: Self = Self(1i32);
    pub const Android: Self = Self(2i32);
    pub const Ios: Self = Self(3i32);
    pub const Linux: Self = Self(4i32);
}
impl ::core::marker::Copy for RemoteSystemPlatform {}
impl ::core::clone::Clone for RemoteSystemPlatform {
    fn clone(&self) -> Self {
        *self
    }
}
pub type RemoteSystemRemovedEventArgs = *mut ::core::ffi::c_void;
pub type RemoteSystemSession = *mut ::core::ffi::c_void;
pub type RemoteSystemSessionAddedEventArgs = *mut ::core::ffi::c_void;
pub type RemoteSystemSessionController = *mut ::core::ffi::c_void;
pub type RemoteSystemSessionCreationResult = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"System_RemoteSystems\"`*"]
#[repr(transparent)]
pub struct RemoteSystemSessionCreationStatus(pub i32);
impl RemoteSystemSessionCreationStatus {
    pub const Success: Self = Self(0i32);
    pub const SessionLimitsExceeded: Self = Self(1i32);
    pub const OperationAborted: Self = Self(2i32);
}
impl ::core::marker::Copy for RemoteSystemSessionCreationStatus {}
impl ::core::clone::Clone for RemoteSystemSessionCreationStatus {
    fn clone(&self) -> Self {
        *self
    }
}
pub type RemoteSystemSessionDisconnectedEventArgs = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"System_RemoteSystems\"`*"]
#[repr(transparent)]
pub struct RemoteSystemSessionDisconnectedReason(pub i32);
impl RemoteSystemSessionDisconnectedReason {
    pub const SessionUnavailable: Self = Self(0i32);
    pub const RemovedByController: Self = Self(1i32);
    pub const SessionClosed: Self = Self(2i32);
}
impl ::core::marker::Copy for RemoteSystemSessionDisconnectedReason {}
impl ::core::clone::Clone for RemoteSystemSessionDisconnectedReason {
    fn clone(&self) -> Self {
        *self
    }
}
pub type RemoteSystemSessionInfo = *mut ::core::ffi::c_void;
pub type RemoteSystemSessionInvitation = *mut ::core::ffi::c_void;
pub type RemoteSystemSessionInvitationListener = *mut ::core::ffi::c_void;
pub type RemoteSystemSessionInvitationReceivedEventArgs = *mut ::core::ffi::c_void;
pub type RemoteSystemSessionJoinRequest = *mut ::core::ffi::c_void;
pub type RemoteSystemSessionJoinRequestedEventArgs = *mut ::core::ffi::c_void;
pub type RemoteSystemSessionJoinResult = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"System_RemoteSystems\"`*"]
#[repr(transparent)]
pub struct RemoteSystemSessionJoinStatus(pub i32);
impl RemoteSystemSessionJoinStatus {
    pub const Success: Self = Self(0i32);
    pub const SessionLimitsExceeded: Self = Self(1i32);
    pub const OperationAborted: Self = Self(2i32);
    pub const SessionUnavailable: Self = Self(3i32);
    pub const RejectedByController: Self = Self(4i32);
}
impl ::core::marker::Copy for RemoteSystemSessionJoinStatus {}
impl ::core::clone::Clone for RemoteSystemSessionJoinStatus {
    fn clone(&self) -> Self {
        *self
    }
}
pub type RemoteSystemSessionMessageChannel = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"System_RemoteSystems\"`*"]
#[repr(transparent)]
pub struct RemoteSystemSessionMessageChannelReliability(pub i32);
impl RemoteSystemSessionMessageChannelReliability {
    pub const Reliable: Self = Self(0i32);
    pub const Unreliable: Self = Self(1i32);
}
impl ::core::marker::Copy for RemoteSystemSessionMessageChannelReliability {}
impl ::core::clone::Clone for RemoteSystemSessionMessageChannelReliability {
    fn clone(&self) -> Self {
        *self
    }
}
pub type RemoteSystemSessionOptions = *mut ::core::ffi::c_void;
pub type RemoteSystemSessionParticipant = *mut ::core::ffi::c_void;
pub type RemoteSystemSessionParticipantAddedEventArgs = *mut ::core::ffi::c_void;
pub type RemoteSystemSessionParticipantRemovedEventArgs = *mut ::core::ffi::c_void;
pub type RemoteSystemSessionParticipantWatcher = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"System_RemoteSystems\"`*"]
#[repr(transparent)]
pub struct RemoteSystemSessionParticipantWatcherStatus(pub i32);
impl RemoteSystemSessionParticipantWatcherStatus {
    pub const Created: Self = Self(0i32);
    pub const Started: Self = Self(1i32);
    pub const EnumerationCompleted: Self = Self(2i32);
    pub const Stopping: Self = Self(3i32);
    pub const Stopped: Self = Self(4i32);
    pub const Aborted: Self = Self(5i32);
}
impl ::core::marker::Copy for RemoteSystemSessionParticipantWatcherStatus {}
impl ::core::clone::Clone for RemoteSystemSessionParticipantWatcherStatus {
    fn clone(&self) -> Self {
        *self
    }
}
pub type RemoteSystemSessionRemovedEventArgs = *mut ::core::ffi::c_void;
pub type RemoteSystemSessionUpdatedEventArgs = *mut ::core::ffi::c_void;
pub type RemoteSystemSessionValueSetReceivedEventArgs = *mut ::core::ffi::c_void;
pub type RemoteSystemSessionWatcher = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"System_RemoteSystems\"`*"]
#[repr(transparent)]
pub struct RemoteSystemSessionWatcherStatus(pub i32);
impl RemoteSystemSessionWatcherStatus {
    pub const Created: Self = Self(0i32);
    pub const Started: Self = Self(1i32);
    pub const EnumerationCompleted: Self = Self(2i32);
    pub const Stopping: Self = Self(3i32);
    pub const Stopped: Self = Self(4i32);
    pub const Aborted: Self = Self(5i32);
}
impl ::core::marker::Copy for RemoteSystemSessionWatcherStatus {}
impl ::core::clone::Clone for RemoteSystemSessionWatcherStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"System_RemoteSystems\"`*"]
#[repr(transparent)]
pub struct RemoteSystemStatus(pub i32);
impl RemoteSystemStatus {
    pub const Unavailable: Self = Self(0i32);
    pub const DiscoveringAvailability: Self = Self(1i32);
    pub const Available: Self = Self(2i32);
    pub const Unknown: Self = Self(3i32);
}
impl ::core::marker::Copy for RemoteSystemStatus {}
impl ::core::clone::Clone for RemoteSystemStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"System_RemoteSystems\"`*"]
#[repr(transparent)]
pub struct RemoteSystemStatusType(pub i32);
impl RemoteSystemStatusType {
    pub const Any: Self = Self(0i32);
    pub const Available: Self = Self(1i32);
}
impl ::core::marker::Copy for RemoteSystemStatusType {}
impl ::core::clone::Clone for RemoteSystemStatusType {
    fn clone(&self) -> Self {
        *self
    }
}
pub type RemoteSystemStatusTypeFilter = *mut ::core::ffi::c_void;
pub type RemoteSystemUpdatedEventArgs = *mut ::core::ffi::c_void;
pub type RemoteSystemWatcher = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"System_RemoteSystems\"`*"]
#[repr(transparent)]
pub struct RemoteSystemWatcherError(pub i32);
impl RemoteSystemWatcherError {
    pub const Unknown: Self = Self(0i32);
    pub const InternetNotAvailable: Self = Self(1i32);
    pub const AuthenticationError: Self = Self(2i32);
}
impl ::core::marker::Copy for RemoteSystemWatcherError {}
impl ::core::clone::Clone for RemoteSystemWatcherError {
    fn clone(&self) -> Self {
        *self
    }
}
pub type RemoteSystemWatcherErrorOccurredEventArgs = *mut ::core::ffi::c_void;
pub type RemoteSystemWebAccountFilter = *mut ::core::ffi::c_void;
