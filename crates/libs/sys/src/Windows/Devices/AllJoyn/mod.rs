pub type AllJoynAboutData = *mut ::core::ffi::c_void;
pub type AllJoynAboutDataView = *mut ::core::ffi::c_void;
pub type AllJoynAcceptSessionJoinerEventArgs = *mut ::core::ffi::c_void;
pub type AllJoynAuthenticationCompleteEventArgs = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Devices_AllJoyn\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct AllJoynAuthenticationMechanism(pub i32);
#[cfg(feature = "deprecated")]
impl AllJoynAuthenticationMechanism {
    pub const None: Self = Self(0i32);
    pub const SrpAnonymous: Self = Self(1i32);
    pub const SrpLogon: Self = Self(2i32);
    pub const EcdheNull: Self = Self(3i32);
    pub const EcdhePsk: Self = Self(4i32);
    pub const EcdheEcdsa: Self = Self(5i32);
    pub const EcdheSpeke: Self = Self(6i32);
}
#[cfg(feature = "deprecated")]
impl ::core::marker::Copy for AllJoynAuthenticationMechanism {}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for AllJoynAuthenticationMechanism {
    fn clone(&self) -> Self {
        *self
    }
}
pub type AllJoynBusAttachment = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Devices_AllJoyn\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct AllJoynBusAttachmentState(pub i32);
#[cfg(feature = "deprecated")]
impl AllJoynBusAttachmentState {
    pub const Disconnected: Self = Self(0i32);
    pub const Connecting: Self = Self(1i32);
    pub const Connected: Self = Self(2i32);
    pub const Disconnecting: Self = Self(3i32);
}
#[cfg(feature = "deprecated")]
impl ::core::marker::Copy for AllJoynBusAttachmentState {}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for AllJoynBusAttachmentState {
    fn clone(&self) -> Self {
        *self
    }
}
pub type AllJoynBusAttachmentStateChangedEventArgs = *mut ::core::ffi::c_void;
pub type AllJoynBusObject = *mut ::core::ffi::c_void;
pub type AllJoynBusObjectStoppedEventArgs = *mut ::core::ffi::c_void;
pub type AllJoynCredentials = *mut ::core::ffi::c_void;
pub type AllJoynCredentialsRequestedEventArgs = *mut ::core::ffi::c_void;
pub type AllJoynCredentialsVerificationRequestedEventArgs = *mut ::core::ffi::c_void;
pub type AllJoynMessageInfo = *mut ::core::ffi::c_void;
pub type AllJoynProducerStoppedEventArgs = *mut ::core::ffi::c_void;
pub type AllJoynServiceInfo = *mut ::core::ffi::c_void;
pub type AllJoynServiceInfoRemovedEventArgs = *mut ::core::ffi::c_void;
pub type AllJoynSession = *mut ::core::ffi::c_void;
pub type AllJoynSessionJoinedEventArgs = *mut ::core::ffi::c_void;
pub type AllJoynSessionLostEventArgs = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Devices_AllJoyn\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct AllJoynSessionLostReason(pub i32);
#[cfg(feature = "deprecated")]
impl AllJoynSessionLostReason {
    pub const None: Self = Self(0i32);
    pub const ProducerLeftSession: Self = Self(1i32);
    pub const ProducerClosedAbruptly: Self = Self(2i32);
    pub const RemovedByProducer: Self = Self(3i32);
    pub const LinkTimeout: Self = Self(4i32);
    pub const Other: Self = Self(5i32);
}
#[cfg(feature = "deprecated")]
impl ::core::marker::Copy for AllJoynSessionLostReason {}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for AllJoynSessionLostReason {
    fn clone(&self) -> Self {
        *self
    }
}
pub type AllJoynSessionMemberAddedEventArgs = *mut ::core::ffi::c_void;
pub type AllJoynSessionMemberRemovedEventArgs = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Devices_AllJoyn\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct AllJoynTrafficType(pub i32);
#[cfg(feature = "deprecated")]
impl AllJoynTrafficType {
    pub const Unknown: Self = Self(0i32);
    pub const Messages: Self = Self(1i32);
    pub const RawUnreliable: Self = Self(2i32);
    pub const RawReliable: Self = Self(4i32);
}
#[cfg(feature = "deprecated")]
impl ::core::marker::Copy for AllJoynTrafficType {}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for AllJoynTrafficType {
    fn clone(&self) -> Self {
        *self
    }
}
pub type AllJoynWatcherStoppedEventArgs = *mut ::core::ffi::c_void;
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct IAllJoynAboutData {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "deprecated")]
    pub IsEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    IsEnabled: usize,
    #[cfg(feature = "deprecated")]
    pub SetIsEnabled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SetIsEnabled: usize,
    #[cfg(feature = "deprecated")]
    pub DefaultAppName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    DefaultAppName: usize,
    #[cfg(feature = "deprecated")]
    pub SetDefaultAppName: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SetDefaultAppName: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub AppNames: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "deprecated")))]
    AppNames: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub DateOfManufacture: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    DateOfManufacture: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub SetDateOfManufacture: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    SetDateOfManufacture: usize,
    #[cfg(feature = "deprecated")]
    pub DefaultDescription: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    DefaultDescription: usize,
    #[cfg(feature = "deprecated")]
    pub SetDefaultDescription: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SetDefaultDescription: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub Descriptions: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "deprecated")))]
    Descriptions: usize,
    #[cfg(feature = "deprecated")]
    pub DefaultManufacturer: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    DefaultManufacturer: usize,
    #[cfg(feature = "deprecated")]
    pub SetDefaultManufacturer: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SetDefaultManufacturer: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub Manufacturers: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "deprecated")))]
    Manufacturers: usize,
    #[cfg(feature = "deprecated")]
    pub ModelNumber: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    ModelNumber: usize,
    #[cfg(feature = "deprecated")]
    pub SetModelNumber: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SetModelNumber: usize,
    #[cfg(feature = "deprecated")]
    pub SoftwareVersion: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SoftwareVersion: usize,
    #[cfg(feature = "deprecated")]
    pub SetSoftwareVersion: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SetSoftwareVersion: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub SupportUrl: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    SupportUrl: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub SetSupportUrl: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    SetSupportUrl: usize,
    #[cfg(feature = "deprecated")]
    pub AppId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    AppId: usize,
    #[cfg(feature = "deprecated")]
    pub SetAppId: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SetAppId: usize,
}
#[cfg(feature = "deprecated")]
impl ::windows_sys::core::Interface for IAllJoynAboutData {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3853106944, data2: 8098, data3: 18489, data4: [147, 239, 249, 223, 64, 72, 144, 247] };
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct IAllJoynAboutDataView {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "deprecated")]
    pub Status: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Status: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub Properties: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "deprecated")))]
    Properties: usize,
    #[cfg(feature = "deprecated")]
    pub AJSoftwareVersion: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    AJSoftwareVersion: usize,
    #[cfg(feature = "deprecated")]
    pub AppId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    AppId: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub DateOfManufacture: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    DateOfManufacture: usize,
    #[cfg(all(feature = "Globalization", feature = "deprecated"))]
    pub DefaultLanguage: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Globalization", feature = "deprecated")))]
    DefaultLanguage: usize,
    #[cfg(feature = "deprecated")]
    pub DeviceId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    DeviceId: usize,
    #[cfg(feature = "deprecated")]
    pub HardwareVersion: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    HardwareVersion: usize,
    #[cfg(feature = "deprecated")]
    pub ModelNumber: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    ModelNumber: usize,
    #[cfg(feature = "deprecated")]
    pub SoftwareVersion: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SoftwareVersion: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Globalization", feature = "deprecated"))]
    pub SupportedLanguages: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Globalization", feature = "deprecated")))]
    SupportedLanguages: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub SupportUrl: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    SupportUrl: usize,
    #[cfg(feature = "deprecated")]
    pub AppName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    AppName: usize,
    #[cfg(feature = "deprecated")]
    pub Description: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Description: usize,
    #[cfg(feature = "deprecated")]
    pub DeviceName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    DeviceName: usize,
    #[cfg(feature = "deprecated")]
    pub Manufacturer: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Manufacturer: usize,
}
#[cfg(feature = "deprecated")]
impl ::windows_sys::core::Interface for IAllJoynAboutDataView {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1747128607, data2: 25106, data3: 18740, data4: [156, 72, 225, 156, 164, 152, 66, 136] };
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct IAllJoynAboutDataViewStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub GetDataBySessionPortAsync: unsafe extern "system" fn(this: *mut *mut Self, uniquename: ::windows_sys::core::HSTRING, busattachment: *mut ::core::ffi::c_void, sessionport: u16, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    GetDataBySessionPortAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Globalization", feature = "deprecated"))]
    pub GetDataBySessionPortWithLanguageAsync: unsafe extern "system" fn(this: *mut *mut Self, uniquename: ::windows_sys::core::HSTRING, busattachment: *mut ::core::ffi::c_void, sessionport: u16, language: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Globalization", feature = "deprecated")))]
    GetDataBySessionPortWithLanguageAsync: usize,
}
#[cfg(feature = "deprecated")]
impl ::windows_sys::core::Interface for IAllJoynAboutDataViewStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1475196552, data2: 3166, data3: 16750, data4: [136, 181, 57, 179, 45, 37, 196, 125] };
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct IAllJoynAcceptSessionJoiner {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "deprecated")]
    pub Accept: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Accept: usize,
}
#[cfg(feature = "deprecated")]
impl ::windows_sys::core::Interface for IAllJoynAcceptSessionJoiner {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1302861778, data2: 52509, data3: 16419, data4: [167, 196, 22, 222, 248, 156, 40, 223] };
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct IAllJoynAcceptSessionJoinerEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "deprecated")]
    pub UniqueName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    UniqueName: usize,
    #[cfg(feature = "deprecated")]
    pub SessionPort: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u16) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SessionPort: usize,
    #[cfg(feature = "deprecated")]
    pub TrafficType: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut AllJoynTrafficType) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    TrafficType: usize,
    #[cfg(feature = "deprecated")]
    pub SamePhysicalNode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SamePhysicalNode: usize,
    #[cfg(feature = "deprecated")]
    pub SameNetwork: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SameNetwork: usize,
    #[cfg(feature = "deprecated")]
    pub Accept: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Accept: usize,
}
#[cfg(feature = "deprecated")]
impl ::windows_sys::core::Interface for IAllJoynAcceptSessionJoinerEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1325093733, data2: 16010, data3: 16983, data4: [143, 16, 83, 156, 224, 213, 108, 15] };
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct IAllJoynAcceptSessionJoinerEventArgsFactory {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "deprecated")]
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, uniquename: ::windows_sys::core::HSTRING, sessionport: u16, traffictype: AllJoynTrafficType, proximity: u8, acceptsessionjoiner: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Create: usize,
}
#[cfg(feature = "deprecated")]
impl ::windows_sys::core::Interface for IAllJoynAcceptSessionJoinerEventArgsFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3024313280, data2: 24901, data3: 17054, data4: [132, 219, 213, 191, 231, 114, 177, 79] };
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct IAllJoynAuthenticationCompleteEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "deprecated")]
    pub AuthenticationMechanism: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut AllJoynAuthenticationMechanism) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    AuthenticationMechanism: usize,
    #[cfg(feature = "deprecated")]
    pub PeerUniqueName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    PeerUniqueName: usize,
    #[cfg(feature = "deprecated")]
    pub Succeeded: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Succeeded: usize,
}
#[cfg(feature = "deprecated")]
impl ::windows_sys::core::Interface for IAllJoynAuthenticationCompleteEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2545184796, data2: 5596, data3: 19283, data4: [182, 164, 125, 19, 67, 0, 215, 191] };
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct IAllJoynBusAttachment {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "deprecated")]
    pub AboutData: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    AboutData: usize,
    #[cfg(feature = "deprecated")]
    pub ConnectionSpecification: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    ConnectionSpecification: usize,
    #[cfg(feature = "deprecated")]
    pub State: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut AllJoynBusAttachmentState) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    State: usize,
    #[cfg(feature = "deprecated")]
    pub UniqueName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    UniqueName: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub PingAsync: unsafe extern "system" fn(this: *mut *mut Self, uniquename: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    PingAsync: usize,
    #[cfg(feature = "deprecated")]
    pub Connect: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Connect: usize,
    #[cfg(feature = "deprecated")]
    pub Disconnect: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Disconnect: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub StateChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    StateChanged: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub RemoveStateChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    RemoveStateChanged: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub AuthenticationMechanisms: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "deprecated")))]
    AuthenticationMechanisms: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub CredentialsRequested: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    CredentialsRequested: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub RemoveCredentialsRequested: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    RemoveCredentialsRequested: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub CredentialsVerificationRequested: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    CredentialsVerificationRequested: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub RemoveCredentialsVerificationRequested: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    RemoveCredentialsVerificationRequested: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub AuthenticationComplete: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    AuthenticationComplete: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub RemoveAuthenticationComplete: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    RemoveAuthenticationComplete: usize,
}
#[cfg(feature = "deprecated")]
impl ::windows_sys::core::Interface for IAllJoynBusAttachment {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4077515091, data2: 7917, data3: 17091, data4: [162, 14, 67, 109, 65, 254, 98, 246] };
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct IAllJoynBusAttachment2 {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub GetAboutDataAsync: unsafe extern "system" fn(this: *mut *mut Self, serviceinfo: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    GetAboutDataAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Globalization", feature = "deprecated"))]
    pub GetAboutDataWithLanguageAsync: unsafe extern "system" fn(this: *mut *mut Self, serviceinfo: *mut ::core::ffi::c_void, language: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Globalization", feature = "deprecated")))]
    GetAboutDataWithLanguageAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub AcceptSessionJoinerRequested: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    AcceptSessionJoinerRequested: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub RemoveAcceptSessionJoinerRequested: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    RemoveAcceptSessionJoinerRequested: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub SessionJoined: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    SessionJoined: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub RemoveSessionJoined: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    RemoveSessionJoined: usize,
}
#[cfg(feature = "deprecated")]
impl ::windows_sys::core::Interface for IAllJoynBusAttachment2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 880069406, data2: 9064, data3: 17330, data4: [180, 62, 106, 58, 193, 39, 141, 152] };
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct IAllJoynBusAttachmentFactory {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "deprecated")]
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, connectionspecification: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Create: usize,
}
#[cfg(feature = "deprecated")]
impl ::windows_sys::core::Interface for IAllJoynBusAttachmentFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1680798116, data2: 44421, data3: 19935, data4: [144, 174, 96, 68, 82, 178, 34, 136] };
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct IAllJoynBusAttachmentStateChangedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "deprecated")]
    pub State: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut AllJoynBusAttachmentState) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    State: usize,
    #[cfg(feature = "deprecated")]
    pub Status: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Status: usize,
}
#[cfg(feature = "deprecated")]
impl ::windows_sys::core::Interface for IAllJoynBusAttachmentStateChangedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3626923508, data2: 49194, data3: 16876, data4: [168, 213, 234, 177, 85, 137, 83, 170] };
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct IAllJoynBusAttachmentStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "deprecated")]
    pub GetDefault: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    GetDefault: usize,
    #[cfg(all(feature = "Devices_Enumeration", feature = "Foundation_Collections", feature = "deprecated"))]
    pub GetWatcher: unsafe extern "system" fn(this: *mut *mut Self, requiredinterfaces: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Devices_Enumeration", feature = "Foundation_Collections", feature = "deprecated")))]
    GetWatcher: usize,
}
#[cfg(feature = "deprecated")]
impl ::windows_sys::core::Interface for IAllJoynBusAttachmentStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2208124221, data2: 4177, data3: 16599, data4: [135, 42, 141, 1, 65, 17, 91, 31] };
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct IAllJoynBusObject {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "deprecated")]
    pub Start: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Start: usize,
    #[cfg(feature = "deprecated")]
    pub Stop: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Stop: usize,
    #[cfg(feature = "deprecated")]
    pub AddProducer: unsafe extern "system" fn(this: *mut *mut Self, producer: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    AddProducer: usize,
    #[cfg(feature = "deprecated")]
    pub BusAttachment: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    BusAttachment: usize,
    #[cfg(feature = "deprecated")]
    pub Session: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Session: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub Stopped: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    Stopped: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub RemoveStopped: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    RemoveStopped: usize,
}
#[cfg(feature = "deprecated")]
impl ::windows_sys::core::Interface for IAllJoynBusObject {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3908928094, data2: 63290, data3: 18700, data4: [136, 4, 4, 224, 38, 100, 48, 71] };
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct IAllJoynBusObjectFactory {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "deprecated")]
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, objectpath: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Create: usize,
    #[cfg(feature = "deprecated")]
    pub CreateWithBusAttachment: unsafe extern "system" fn(this: *mut *mut Self, objectpath: ::windows_sys::core::HSTRING, busattachment: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    CreateWithBusAttachment: usize,
}
#[cfg(feature = "deprecated")]
impl ::windows_sys::core::Interface for IAllJoynBusObjectFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 741318411, data2: 36354, data3: 20380, data4: [172, 39, 234, 109, 173, 93, 59, 80] };
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct IAllJoynBusObjectStoppedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "deprecated")]
    pub Status: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Status: usize,
}
#[cfg(feature = "deprecated")]
impl ::windows_sys::core::Interface for IAllJoynBusObjectStoppedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3725598997, data2: 61326, data3: 19778, data4: [185, 59, 162, 174, 116, 81, 151, 102] };
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct IAllJoynBusObjectStoppedEventArgsFactory {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "deprecated")]
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, status: i32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Create: usize,
}
#[cfg(feature = "deprecated")]
impl ::windows_sys::core::Interface for IAllJoynBusObjectStoppedEventArgsFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1797455176, data2: 53411, data3: 16981, data4: [149, 58, 71, 114, 180, 2, 128, 115] };
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct IAllJoynCredentials {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "deprecated")]
    pub AuthenticationMechanism: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut AllJoynAuthenticationMechanism) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    AuthenticationMechanism: usize,
    #[cfg(all(feature = "Security_Cryptography_Certificates", feature = "deprecated"))]
    pub Certificate: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Security_Cryptography_Certificates", feature = "deprecated")))]
    Certificate: usize,
    #[cfg(all(feature = "Security_Cryptography_Certificates", feature = "deprecated"))]
    pub SetCertificate: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Security_Cryptography_Certificates", feature = "deprecated")))]
    SetCertificate: usize,
    #[cfg(all(feature = "Security_Credentials", feature = "deprecated"))]
    pub PasswordCredential: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Security_Credentials", feature = "deprecated")))]
    PasswordCredential: usize,
    #[cfg(all(feature = "Security_Credentials", feature = "deprecated"))]
    pub SetPasswordCredential: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Security_Credentials", feature = "deprecated")))]
    SetPasswordCredential: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub Timeout: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    Timeout: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub SetTimeout: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    SetTimeout: usize,
}
#[cfg(feature = "deprecated")]
impl ::windows_sys::core::Interface for IAllJoynCredentials {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2185646322, data2: 41360, data3: 16561, data4: [171, 171, 52, 158, 194, 68, 223, 170] };
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct IAllJoynCredentialsRequestedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "deprecated")]
    pub AttemptCount: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u16) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    AttemptCount: usize,
    #[cfg(feature = "deprecated")]
    pub Credentials: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Credentials: usize,
    #[cfg(feature = "deprecated")]
    pub PeerUniqueName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    PeerUniqueName: usize,
    #[cfg(feature = "deprecated")]
    pub RequestedUserName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    RequestedUserName: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub GetDeferral: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    GetDeferral: usize,
}
#[cfg(feature = "deprecated")]
impl ::windows_sys::core::Interface for IAllJoynCredentialsRequestedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1787290446, data2: 45161, data3: 19328, data4: [158, 26, 65, 188, 131, 124, 101, 210] };
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct IAllJoynCredentialsVerificationRequestedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "deprecated")]
    pub AuthenticationMechanism: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut AllJoynAuthenticationMechanism) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    AuthenticationMechanism: usize,
    #[cfg(feature = "deprecated")]
    pub PeerUniqueName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    PeerUniqueName: usize,
    #[cfg(all(feature = "Security_Cryptography_Certificates", feature = "deprecated"))]
    pub PeerCertificate: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Security_Cryptography_Certificates", feature = "deprecated")))]
    PeerCertificate: usize,
    #[cfg(all(feature = "Networking_Sockets", feature = "deprecated"))]
    pub PeerCertificateErrorSeverity: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Networking::Sockets::SocketSslErrorSeverity) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Networking_Sockets", feature = "deprecated")))]
    PeerCertificateErrorSeverity: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Security_Cryptography_Certificates", feature = "deprecated"))]
    pub PeerCertificateErrors: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Security_Cryptography_Certificates", feature = "deprecated")))]
    PeerCertificateErrors: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Security_Cryptography_Certificates", feature = "deprecated"))]
    pub PeerIntermediateCertificates: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Security_Cryptography_Certificates", feature = "deprecated")))]
    PeerIntermediateCertificates: usize,
    #[cfg(feature = "deprecated")]
    pub Accept: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Accept: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub GetDeferral: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    GetDeferral: usize,
}
#[cfg(feature = "deprecated")]
impl ::windows_sys::core::Interface for IAllJoynCredentialsVerificationRequestedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2148169234, data2: 47109, data3: 17583, data4: [162, 225, 121, 42, 182, 85, 162, 208] };
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct IAllJoynMessageInfo {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "deprecated")]
    pub SenderUniqueName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SenderUniqueName: usize,
}
#[cfg(feature = "deprecated")]
impl ::windows_sys::core::Interface for IAllJoynMessageInfo {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4281008423, data2: 11282, data3: 18521, data4: [170, 58, 199, 68, 97, 238, 129, 76] };
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct IAllJoynMessageInfoFactory {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "deprecated")]
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, senderuniquename: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Create: usize,
}
#[cfg(feature = "deprecated")]
impl ::windows_sys::core::Interface for IAllJoynMessageInfoFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 879119402, data2: 33417, data3: 17364, data4: [180, 168, 63, 77, 227, 89, 240, 67] };
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct IAllJoynProducer {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "deprecated")]
    pub SetBusObject: unsafe extern "system" fn(this: *mut *mut Self, busobject: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SetBusObject: usize,
}
#[cfg(feature = "deprecated")]
impl ::windows_sys::core::Interface for IAllJoynProducer {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2634565241, data2: 18075, data3: 18778, data4: [167, 16, 172, 80, 241, 35, 6, 159] };
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct IAllJoynProducerStoppedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "deprecated")]
    pub Status: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Status: usize,
}
#[cfg(feature = "deprecated")]
impl ::windows_sys::core::Interface for IAllJoynProducerStoppedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1362138992, data2: 18743, data3: 18733, data4: [128, 128, 35, 100, 57, 152, 124, 235] };
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct IAllJoynProducerStoppedEventArgsFactory {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "deprecated")]
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, status: i32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Create: usize,
}
#[cfg(feature = "deprecated")]
impl ::windows_sys::core::Interface for IAllJoynProducerStoppedEventArgsFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1448253793, data2: 45593, data3: 19822, data4: [159, 120, 250, 63, 153, 250, 143, 229] };
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct IAllJoynServiceInfo {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "deprecated")]
    pub UniqueName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    UniqueName: usize,
    #[cfg(feature = "deprecated")]
    pub ObjectPath: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    ObjectPath: usize,
    #[cfg(feature = "deprecated")]
    pub SessionPort: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u16) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SessionPort: usize,
}
#[cfg(feature = "deprecated")]
impl ::windows_sys::core::Interface for IAllJoynServiceInfo {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1287553545, data2: 47422, data3: 16770, data4: [153, 155, 221, 208, 0, 249, 197, 117] };
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct IAllJoynServiceInfoFactory {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "deprecated")]
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, uniquename: ::windows_sys::core::HSTRING, objectpath: ::windows_sys::core::HSTRING, sessionport: u16, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Create: usize,
}
#[cfg(feature = "deprecated")]
impl ::windows_sys::core::Interface for IAllJoynServiceInfoFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1971444413, data2: 65027, data3: 20299, data4: [148, 164, 240, 47, 220, 189, 17, 184] };
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct IAllJoynServiceInfoRemovedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "deprecated")]
    pub UniqueName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    UniqueName: usize,
}
#[cfg(feature = "deprecated")]
impl ::windows_sys::core::Interface for IAllJoynServiceInfoRemovedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 811051359, data2: 7487, data3: 16883, data4: [137, 105, 227, 39, 146, 98, 115, 150] };
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct IAllJoynServiceInfoRemovedEventArgsFactory {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "deprecated")]
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, uniquename: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Create: usize,
}
#[cfg(feature = "deprecated")]
impl ::windows_sys::core::Interface for IAllJoynServiceInfoRemovedEventArgsFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 230655527, data2: 39679, data3: 18773, data4: [146, 39, 105, 83, 186, 244, 21, 105] };
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct IAllJoynServiceInfoStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub FromIdAsync: unsafe extern "system" fn(this: *mut *mut Self, deviceid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    FromIdAsync: usize,
}
#[cfg(feature = "deprecated")]
impl ::windows_sys::core::Interface for IAllJoynServiceInfoStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1450727178, data2: 24634, data3: 18940, data4: [183, 80, 14, 241, 54, 9, 33, 60] };
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct IAllJoynSession {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "deprecated")]
    pub Id: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Id: usize,
    #[cfg(feature = "deprecated")]
    pub Status: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Status: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub RemoveMemberAsync: unsafe extern "system" fn(this: *mut *mut Self, uniquename: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    RemoveMemberAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub MemberAdded: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    MemberAdded: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub RemoveMemberAdded: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    RemoveMemberAdded: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub MemberRemoved: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    MemberRemoved: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub RemoveMemberRemoved: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    RemoveMemberRemoved: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub Lost: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    Lost: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub RemoveLost: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    RemoveLost: usize,
}
#[cfg(feature = "deprecated")]
impl ::windows_sys::core::Interface for IAllJoynSession {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3906018060, data2: 49364, data3: 16492, data4: [136, 169, 169, 62, 250, 133, 212, 177] };
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct IAllJoynSessionJoinedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "deprecated")]
    pub Session: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Session: usize,
}
#[cfg(feature = "deprecated")]
impl ::windows_sys::core::Interface for IAllJoynSessionJoinedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2661243856, data2: 46551, data3: 18373, data4: [141, 171, 176, 64, 204, 25, 40, 113] };
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct IAllJoynSessionJoinedEventArgsFactory {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "deprecated")]
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, session: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Create: usize,
}
#[cfg(feature = "deprecated")]
impl ::windows_sys::core::Interface for IAllJoynSessionJoinedEventArgsFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1747244681, data2: 54987, data3: 19870, data4: [160, 158, 53, 128, 104, 112, 177, 127] };
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct IAllJoynSessionLostEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "deprecated")]
    pub Reason: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut AllJoynSessionLostReason) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Reason: usize,
}
#[cfg(feature = "deprecated")]
impl ::windows_sys::core::Interface for IAllJoynSessionLostEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3882263690, data2: 35768, data3: 18772, data4: [174, 103, 210, 250, 67, 209, 249, 107] };
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct IAllJoynSessionLostEventArgsFactory {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "deprecated")]
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, reason: AllJoynSessionLostReason, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Create: usize,
}
#[cfg(feature = "deprecated")]
impl ::windows_sys::core::Interface for IAllJoynSessionLostEventArgsFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 331087154, data2: 54004, data3: 18889, data4: [152, 14, 40, 5, 225, 53, 134, 177] };
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct IAllJoynSessionMemberAddedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "deprecated")]
    pub UniqueName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    UniqueName: usize,
}
#[cfg(feature = "deprecated")]
impl ::windows_sys::core::Interface for IAllJoynSessionMemberAddedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1235384714, data2: 3537, data3: 18113, data4: [156, 214, 39, 25, 14, 80, 58, 94] };
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct IAllJoynSessionMemberAddedEventArgsFactory {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "deprecated")]
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, uniquename: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Create: usize,
}
#[cfg(feature = "deprecated")]
impl ::windows_sys::core::Interface for IAllJoynSessionMemberAddedEventArgsFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 874373970, data2: 7475, data3: 16545, data4: [161, 211, 229, 119, 112, 32, 225, 241] };
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct IAllJoynSessionMemberRemovedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "deprecated")]
    pub UniqueName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    UniqueName: usize,
}
#[cfg(feature = "deprecated")]
impl ::windows_sys::core::Interface for IAllJoynSessionMemberRemovedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1083842975, data2: 43594, data3: 18579, data4: [180, 48, 186, 161, 182, 60, 98, 25] };
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct IAllJoynSessionMemberRemovedEventArgsFactory {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "deprecated")]
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, uniquename: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Create: usize,
}
#[cfg(feature = "deprecated")]
impl ::windows_sys::core::Interface for IAllJoynSessionMemberRemovedEventArgsFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3302184424, data2: 17080, data3: 19303, data4: [183, 87, 208, 207, 202, 213, 146, 128] };
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct IAllJoynSessionStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub GetFromServiceInfoAsync: unsafe extern "system" fn(this: *mut *mut Self, serviceinfo: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    GetFromServiceInfoAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub GetFromServiceInfoAndBusAttachmentAsync: unsafe extern "system" fn(this: *mut *mut Self, serviceinfo: *mut ::core::ffi::c_void, busattachment: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    GetFromServiceInfoAndBusAttachmentAsync: usize,
}
#[cfg(feature = "deprecated")]
impl ::windows_sys::core::Interface for IAllJoynSessionStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2651182596, data2: 41068, data3: 18132, data4: [180, 108, 11, 11, 84, 16, 91, 68] };
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct IAllJoynStatusStatics {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "deprecated")]
    pub Ok: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Ok: usize,
    #[cfg(feature = "deprecated")]
    pub Fail: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Fail: usize,
    #[cfg(feature = "deprecated")]
    pub OperationTimedOut: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    OperationTimedOut: usize,
    #[cfg(feature = "deprecated")]
    pub OtherEndClosed: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    OtherEndClosed: usize,
    #[cfg(feature = "deprecated")]
    pub ConnectionRefused: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    ConnectionRefused: usize,
    #[cfg(feature = "deprecated")]
    pub AuthenticationFailed: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    AuthenticationFailed: usize,
    #[cfg(feature = "deprecated")]
    pub AuthenticationRejectedByUser: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    AuthenticationRejectedByUser: usize,
    #[cfg(feature = "deprecated")]
    pub SslConnectFailed: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SslConnectFailed: usize,
    #[cfg(feature = "deprecated")]
    pub SslIdentityVerificationFailed: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SslIdentityVerificationFailed: usize,
    #[cfg(feature = "deprecated")]
    pub InsufficientSecurity: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    InsufficientSecurity: usize,
    #[cfg(feature = "deprecated")]
    pub InvalidArgument1: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    InvalidArgument1: usize,
    #[cfg(feature = "deprecated")]
    pub InvalidArgument2: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    InvalidArgument2: usize,
    #[cfg(feature = "deprecated")]
    pub InvalidArgument3: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    InvalidArgument3: usize,
    #[cfg(feature = "deprecated")]
    pub InvalidArgument4: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    InvalidArgument4: usize,
    #[cfg(feature = "deprecated")]
    pub InvalidArgument5: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    InvalidArgument5: usize,
    #[cfg(feature = "deprecated")]
    pub InvalidArgument6: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    InvalidArgument6: usize,
    #[cfg(feature = "deprecated")]
    pub InvalidArgument7: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    InvalidArgument7: usize,
    #[cfg(feature = "deprecated")]
    pub InvalidArgument8: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    InvalidArgument8: usize,
}
#[cfg(feature = "deprecated")]
impl ::windows_sys::core::Interface for IAllJoynStatusStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3501695358, data2: 3369, data3: 19881, data4: [138, 198, 84, 197, 84, 190, 219, 197] };
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct IAllJoynWatcherStoppedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "deprecated")]
    pub Status: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Status: usize,
}
#[cfg(feature = "deprecated")]
impl ::windows_sys::core::Interface for IAllJoynWatcherStoppedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3388776507, data2: 28701, data3: 19112, data4: [151, 221, 162, 187, 10, 143, 95, 163] };
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct IAllJoynWatcherStoppedEventArgsFactory {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "deprecated")]
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, status: i32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Create: usize,
}
#[cfg(feature = "deprecated")]
impl ::windows_sys::core::Interface for IAllJoynWatcherStoppedEventArgsFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2274338216, data2: 11600, data3: 18401, data4: [144, 74, 32, 191, 13, 72, 199, 130] };
}
