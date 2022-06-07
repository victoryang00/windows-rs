#[repr(C)]
pub struct IKnownRemoteSystemCapabilitiesStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub AppService: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub LaunchUri: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub RemoteSession: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SpatialEntity: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IKnownRemoteSystemCapabilitiesStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2164843392, data2: 32650, data3: 17636, data4: [146, 205, 3, 182, 70, 155, 148, 163] };
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
impl ::windows_sys::core::Interface for IRemoteSystem {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3981981901, data2: 7696, data3: 19084, data4: [180, 166, 78, 95, 214, 249, 119, 33] };
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
impl ::windows_sys::core::Interface for IRemoteSystem2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 165668076, data2: 64395, data3: 18952, data4: [167, 88, 104, 118, 67, 93, 118, 158] };
}
#[repr(C)]
pub struct IRemoteSystem3 {
    pub base__: ::windows_sys::core::IInspectable,
    pub ManufacturerDisplayName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub ModelDisplayName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IRemoteSystem3 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1924445333, data2: 47046, data3: 16574, data4: [131, 27, 115, 86, 47, 18, 255, 168] };
}
#[repr(C)]
pub struct IRemoteSystem4 {
    pub base__: ::windows_sys::core::IInspectable,
    pub Platform: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut RemoteSystemPlatform) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IRemoteSystem4 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4049928165, data2: 47495, data3: 19621, data4: [153, 38, 250, 4, 56, 190, 98, 115] };
}
#[repr(C)]
pub struct IRemoteSystem5 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub Apps: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Apps: usize,
}
impl ::windows_sys::core::Interface for IRemoteSystem5 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3945453347, data2: 58850, data3: 19170, data4: [167, 167, 161, 9, 122, 9, 142, 144] };
}
#[repr(C)]
pub struct IRemoteSystem6 {
    pub base__: ::windows_sys::core::IInspectable,
    pub User: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IRemoteSystem6 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3570248002, data2: 49191, data3: 21310, data4: [147, 132, 58, 25, 180, 247, 238, 243] };
}
#[repr(C)]
pub struct IRemoteSystemAddedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub RemoteSystem: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IRemoteSystemAddedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2402899471, data2: 58676, data3: 18071, data4: [136, 54, 122, 190, 161, 81, 81, 110] };
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
impl ::windows_sys::core::Interface for IRemoteSystemApp {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2162539709, data2: 54605, data3: 16817, data4: [155, 22, 104, 16, 168, 113, 237, 79] };
}
#[repr(C)]
pub struct IRemoteSystemApp2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub User: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ConnectionToken: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IRemoteSystemApp2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1667874581, data2: 2710, data3: 22394, data4: [143, 246, 195, 89, 4, 223, 168, 243] };
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
impl ::windows_sys::core::Interface for IRemoteSystemAppRegistration {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3027847093, data2: 28725, data3: 19034, data4: [184, 223, 150, 45, 143, 132, 49, 244] };
}
#[repr(C)]
pub struct IRemoteSystemAppRegistrationStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetDefault: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetForUser: unsafe extern "system" fn(this: *mut *mut Self, user: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IRemoteSystemAppRegistrationStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 28940352, data2: 53202, data3: 17727, data4: [174, 37, 194, 83, 159, 8, 106, 253] };
}
#[repr(C)]
pub struct IRemoteSystemAuthorizationKindFilter {
    pub base__: ::windows_sys::core::IInspectable,
    pub RemoteSystemAuthorizationKind: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut RemoteSystemAuthorizationKind) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IRemoteSystemAuthorizationKindFilter {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1796071054, data2: 1232, data3: 16628, data4: [162, 127, 194, 172, 187, 214, 183, 52] };
}
#[repr(C)]
pub struct IRemoteSystemAuthorizationKindFilterFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, remotesystemauthorizationkind: RemoteSystemAuthorizationKind, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IRemoteSystemAuthorizationKindFilterFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2909134669, data2: 46698, data3: 17828, data4: [129, 119, 140, 174, 215, 93, 158, 90] };
}
#[repr(C)]
pub struct IRemoteSystemConnectionInfo {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsProximal: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IRemoteSystemConnectionInfo {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 589794243, data2: 3337, data3: 21195, data4: [156, 106, 238, 210, 148, 11, 238, 67] };
}
#[repr(C)]
pub struct IRemoteSystemConnectionInfoStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "ApplicationModel_AppService")]
    pub TryCreateFromAppServiceConnection: unsafe extern "system" fn(this: *mut *mut Self, connection: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_AppService"))]
    TryCreateFromAppServiceConnection: usize,
}
impl ::windows_sys::core::Interface for IRemoteSystemConnectionInfoStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2894274093, data2: 26309, data3: 22231, data4: [164, 206, 112, 93, 148, 146, 90, 214] };
}
#[repr(C)]
pub struct IRemoteSystemConnectionRequest {
    pub base__: ::windows_sys::core::IInspectable,
    pub RemoteSystem: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IRemoteSystemConnectionRequest {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2230141188, data2: 36190, data3: 19826, data4: [130, 56, 118, 33, 87, 108, 122, 103] };
}
#[repr(C)]
pub struct IRemoteSystemConnectionRequest2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub RemoteSystemApp: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IRemoteSystemConnectionRequest2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 316632431, data2: 49148, data3: 18490, data4: [138, 190, 211, 74, 108, 25, 249, 43] };
}
#[repr(C)]
pub struct IRemoteSystemConnectionRequest3 {
    pub base__: ::windows_sys::core::IInspectable,
    pub ConnectionToken: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IRemoteSystemConnectionRequest3 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3733373927, data2: 51660, data3: 23120, data4: [184, 217, 186, 123, 52, 187, 141, 14] };
}
#[repr(C)]
pub struct IRemoteSystemConnectionRequestFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, remotesystem: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IRemoteSystemConnectionRequestFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2852784672, data2: 47851, data3: 17781, data4: [181, 48, 129, 11, 185, 120, 99, 52] };
}
#[repr(C)]
pub struct IRemoteSystemConnectionRequestStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateForApp: unsafe extern "system" fn(this: *mut *mut Self, remotesystemapp: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IRemoteSystemConnectionRequestStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2261390397, data2: 33300, data3: 16988, data4: [137, 50, 219, 73, 3, 45, 19, 6] };
}
#[repr(C)]
pub struct IRemoteSystemConnectionRequestStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateFromConnectionToken: unsafe extern "system" fn(this: *mut *mut Self, connectiontoken: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateFromConnectionTokenForUser: unsafe extern "system" fn(this: *mut *mut Self, user: *mut ::core::ffi::c_void, connectiontoken: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IRemoteSystemConnectionRequestStatics2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1175392295, data2: 25836, data3: 22926, data4: [168, 0, 79, 46, 229, 141, 239, 25] };
}
#[repr(C)]
pub struct IRemoteSystemDiscoveryTypeFilter {
    pub base__: ::windows_sys::core::IInspectable,
    pub RemoteSystemDiscoveryType: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut RemoteSystemDiscoveryType) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IRemoteSystemDiscoveryTypeFilter {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1121518623, data2: 61018, data3: 17370, data4: [172, 106, 111, 238, 37, 70, 7, 65] };
}
#[repr(C)]
pub struct IRemoteSystemDiscoveryTypeFilterFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, discoverytype: RemoteSystemDiscoveryType, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IRemoteSystemDiscoveryTypeFilterFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2677979539, data2: 49760, data3: 16737, data4: [146, 242, 156, 2, 31, 35, 254, 93] };
}
#[repr(C)]
pub struct IRemoteSystemEnumerationCompletedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
}
impl ::windows_sys::core::Interface for IRemoteSystemEnumerationCompletedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3337108831, data2: 16432, data3: 17236, data4: [160, 96, 20, 241, 178, 44, 84, 93] };
}
#[repr(C)]
pub struct IRemoteSystemFilter {
    pub base__: ::windows_sys::core::IInspectable,
}
impl ::windows_sys::core::Interface for IRemoteSystemFilter {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1245424100, data2: 39403, data3: 17899, data4: [186, 22, 3, 103, 114, 143, 243, 116] };
}
#[repr(C)]
pub struct IRemoteSystemKindFilter {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub RemoteSystemKinds: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    RemoteSystemKinds: usize,
}
impl ::windows_sys::core::Interface for IRemoteSystemKindFilter {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 954321388, data2: 8899, data3: 20214, data4: [144, 26, 187, 177, 199, 170, 212, 237] };
}
#[repr(C)]
pub struct IRemoteSystemKindFilterFactory {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Foundation_Collections")]
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, remotesystemkinds: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Create: usize,
}
impl ::windows_sys::core::Interface for IRemoteSystemKindFilterFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2717587694, data2: 39402, data3: 16572, data4: [154, 57, 198, 112, 170, 128, 74, 40] };
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
impl ::windows_sys::core::Interface for IRemoteSystemKindStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4130436659, data2: 43796, data3: 16848, data4: [149, 83, 121, 106, 173, 184, 130, 219] };
}
#[repr(C)]
pub struct IRemoteSystemKindStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub Iot: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Tablet: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Laptop: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IRemoteSystemKindStatics2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3118703568, data2: 1126, data3: 18249, data4: [145, 232, 101, 249, 209, 154, 150, 165] };
}
#[repr(C)]
pub struct IRemoteSystemRemovedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub RemoteSystemId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IRemoteSystemRemovedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2336036539, data2: 29446, data3: 18922, data4: [183, 223, 103, 213, 113, 76, 176, 19] };
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
impl ::windows_sys::core::Interface for IRemoteSystemSession {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1766287873, data2: 39642, data3: 18703, data4: [149, 73, 211, 28, 177, 76, 158, 149] };
}
#[repr(C)]
pub struct IRemoteSystemSessionAddedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub SessionInfo: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IRemoteSystemSessionAddedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3582318420, data2: 48279, data3: 19513, data4: [153, 180, 190, 202, 118, 224, 76, 63] };
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
impl ::windows_sys::core::Interface for IRemoteSystemSessionController {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3834326482, data2: 26656, data3: 18535, data4: [180, 37, 216, 156, 10, 62, 247, 186] };
}
#[repr(C)]
pub struct IRemoteSystemSessionControllerFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateController: unsafe extern "system" fn(this: *mut *mut Self, displayname: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateControllerWithSessionOptions: unsafe extern "system" fn(this: *mut *mut Self, displayname: ::windows_sys::core::HSTRING, options: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IRemoteSystemSessionControllerFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3217829739, data2: 44093, data3: 16793, data4: [130, 205, 102, 112, 167, 115, 239, 46] };
}
#[repr(C)]
pub struct IRemoteSystemSessionCreationResult {
    pub base__: ::windows_sys::core::IInspectable,
    pub Status: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut RemoteSystemSessionCreationStatus) -> ::windows_sys::core::HRESULT,
    pub Session: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IRemoteSystemSessionCreationResult {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2811761346, data2: 14302, data3: 17548, data4: [139, 131, 163, 10, 163, 196, 234, 214] };
}
#[repr(C)]
pub struct IRemoteSystemSessionDisconnectedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Reason: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut RemoteSystemSessionDisconnectedReason) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IRemoteSystemSessionDisconnectedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3725313691, data2: 30661, data3: 17948, data4: [130, 9, 124, 108, 93, 49, 17, 171] };
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
impl ::windows_sys::core::Interface for IRemoteSystemSessionInfo {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4283299400, data2: 35594, data3: 20122, data4: [153, 5, 105, 228, 184, 65, 197, 136] };
}
#[repr(C)]
pub struct IRemoteSystemSessionInvitation {
    pub base__: ::windows_sys::core::IInspectable,
    pub Sender: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SessionInfo: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IRemoteSystemSessionInvitation {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1043516561, data2: 20951, data3: 18278, data4: [161, 33, 37, 81, 108, 59, 130, 148] };
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
impl ::windows_sys::core::Interface for IRemoteSystemSessionInvitationListener {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 150208575, data2: 48241, data3: 18913, data4: [135, 74, 49, 221, 255, 154, 39, 185] };
}
#[repr(C)]
pub struct IRemoteSystemSessionInvitationReceivedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Invitation: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IRemoteSystemSessionInvitationReceivedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1586907693, data2: 41229, data3: 20187, data4: [141, 234, 84, 210, 10, 193, 149, 67] };
}
#[repr(C)]
pub struct IRemoteSystemSessionJoinRequest {
    pub base__: ::windows_sys::core::IInspectable,
    pub Participant: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Accept: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IRemoteSystemSessionJoinRequest {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 543162472, data2: 31124, data3: 17201, data4: [134, 209, 216, 157, 136, 37, 133, 238] };
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
impl ::windows_sys::core::Interface for IRemoteSystemSessionJoinRequestedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3687468995, data2: 33465, data3: 18454, data4: [156, 36, 228, 14, 97, 119, 75, 216] };
}
#[repr(C)]
pub struct IRemoteSystemSessionJoinResult {
    pub base__: ::windows_sys::core::IInspectable,
    pub Status: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut RemoteSystemSessionJoinStatus) -> ::windows_sys::core::HRESULT,
    pub Session: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IRemoteSystemSessionJoinResult {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3464175364, data2: 41022, data3: 16804, data4: [144, 11, 30, 121, 50, 140, 18, 103] };
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
impl ::windows_sys::core::Interface for IRemoteSystemSessionMessageChannel {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2502218026, data2: 29657, data3: 19472, data4: [183, 81, 194, 103, 132, 67, 113, 39] };
}
#[repr(C)]
pub struct IRemoteSystemSessionMessageChannelFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, session: *mut ::core::ffi::c_void, channelname: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateWithReliability: unsafe extern "system" fn(this: *mut *mut Self, session: *mut ::core::ffi::c_void, channelname: ::windows_sys::core::HSTRING, reliability: RemoteSystemSessionMessageChannelReliability, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IRemoteSystemSessionMessageChannelFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 694033482, data2: 48406, data3: 17048, data4: [183, 206, 65, 84, 130, 176, 225, 29] };
}
#[repr(C)]
pub struct IRemoteSystemSessionOptions {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsInviteOnly: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsInviteOnly: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IRemoteSystemSessionOptions {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1947129685, data2: 33816, data3: 20225, data4: [147, 83, 226, 28, 158, 204, 108, 252] };
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
impl ::windows_sys::core::Interface for IRemoteSystemSessionParticipant {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2123367820, data2: 44281, data3: 18217, data4: [138, 23, 68, 231, 186, 237, 93, 204] };
}
#[repr(C)]
pub struct IRemoteSystemSessionParticipantAddedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Participant: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IRemoteSystemSessionParticipantAddedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3545913304, data2: 51617, data3: 19383, data4: [182, 176, 121, 187, 145, 173, 249, 61] };
}
#[repr(C)]
pub struct IRemoteSystemSessionParticipantRemovedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Participant: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IRemoteSystemSessionParticipantRemovedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2255417480, data2: 56936, data3: 19135, data4: [136, 161, 249, 13, 22, 39, 65, 146] };
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
impl ::windows_sys::core::Interface for IRemoteSystemSessionParticipantWatcher {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3705471692, data2: 43655, data3: 19833, data4: [182, 204, 68, 89, 179, 233, 32, 117] };
}
#[repr(C)]
pub struct IRemoteSystemSessionRemovedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub SessionInfo: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IRemoteSystemSessionRemovedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2944569678, data2: 14753, data3: 19946, data4: [157, 99, 67, 121, 141, 91, 187, 208] };
}
#[repr(C)]
pub struct IRemoteSystemSessionStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateWatcher: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IRemoteSystemSessionStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2233764255, data2: 64800, data3: 17635, data4: [149, 101, 231, 90, 59, 20, 198, 110] };
}
#[repr(C)]
pub struct IRemoteSystemSessionUpdatedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub SessionInfo: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IRemoteSystemSessionUpdatedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 377966697, data2: 8990, data3: 19601, data4: [142, 200, 179, 163, 157, 158, 85, 163] };
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
impl ::windows_sys::core::Interface for IRemoteSystemSessionValueSetReceivedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 116594565, data2: 11685, data3: 20056, data4: [167, 143, 158, 141, 7, 132, 238, 37] };
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
impl ::windows_sys::core::Interface for IRemoteSystemSessionWatcher {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2147738432, data2: 3137, data3: 19042, data4: [182, 215, 189, 190, 43, 25, 190, 45] };
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
impl ::windows_sys::core::Interface for IRemoteSystemStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2760225682, data2: 65323, data3: 19271, data4: [190, 98, 116, 63, 47, 20, 15, 48] };
}
#[repr(C)]
pub struct IRemoteSystemStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsAuthorizationKindEnabled: unsafe extern "system" fn(this: *mut *mut Self, kind: RemoteSystemAuthorizationKind, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IRemoteSystemStatics2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 211348938, data2: 28569, data3: 19538, data4: [162, 114, 234, 79, 54, 71, 23, 68] };
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
impl ::windows_sys::core::Interface for IRemoteSystemStatics3 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2576740719, data2: 2876, data3: 23237, data4: [179, 37, 204, 115, 244, 55, 223, 205] };
}
#[repr(C)]
pub struct IRemoteSystemStatusTypeFilter {
    pub base__: ::windows_sys::core::IInspectable,
    pub RemoteSystemStatusType: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut RemoteSystemStatusType) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IRemoteSystemStatusTypeFilter {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 205082958, data2: 52150, data3: 18295, data4: [133, 52, 46, 12, 82, 26, 255, 162] };
}
#[repr(C)]
pub struct IRemoteSystemStatusTypeFilterFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, remotesystemstatustype: RemoteSystemStatusType, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IRemoteSystemStatusTypeFilterFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 869234938, data2: 55076, data3: 16677, data4: [172, 122, 141, 40, 30, 68, 201, 73] };
}
#[repr(C)]
pub struct IRemoteSystemUpdatedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub RemoteSystem: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IRemoteSystemUpdatedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1963130638, data2: 56267, data3: 16725, data4: [180, 202, 179, 10, 4, 242, 118, 39] };
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
impl ::windows_sys::core::Interface for IRemoteSystemWatcher {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1566575742, data2: 11271, data3: 18629, data4: [136, 156, 69, 93, 43, 9, 151, 113] };
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
impl ::windows_sys::core::Interface for IRemoteSystemWatcher2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1933797120, data2: 6602, data3: 18681, data4: [164, 205, 120, 15, 122, 213, 140, 113] };
}
#[repr(C)]
pub struct IRemoteSystemWatcher3 {
    pub base__: ::windows_sys::core::IInspectable,
    pub User: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IRemoteSystemWatcher3 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4154200015, data2: 43283, data3: 21971, data4: [132, 19, 65, 143, 207, 21, 186, 84] };
}
#[repr(C)]
pub struct IRemoteSystemWatcherErrorOccurredEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Error: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut RemoteSystemWatcherError) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IRemoteSystemWatcherErrorOccurredEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1959118511, data2: 20756, data3: 17446, data4: [146, 22, 32, 216, 31, 133, 25, 174] };
}
#[repr(C)]
pub struct IRemoteSystemWebAccountFilter {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Security_Credentials")]
    pub Account: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))]
    Account: usize,
}
impl ::windows_sys::core::Interface for IRemoteSystemWebAccountFilter {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1068980339, data2: 34760, data3: 23951, data4: [151, 126, 246, 159, 150, 214, 114, 56] };
}
#[repr(C)]
pub struct IRemoteSystemWebAccountFilterFactory {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Security_Credentials")]
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, account: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))]
    Create: usize,
}
impl ::windows_sys::core::Interface for IRemoteSystemWebAccountFilterFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 881469193, data2: 24397, data3: 20775, data4: [180, 167, 191, 153, 213, 37, 43, 27] };
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
