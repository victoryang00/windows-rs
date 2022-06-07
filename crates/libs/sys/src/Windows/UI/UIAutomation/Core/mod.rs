#[repr(C)]
#[doc = "*Required features: `\"UI_UIAutomation_Core\"`*"]
pub struct AutomationAnnotationTypeRegistration {
    pub LocalId: i32,
}
impl ::core::marker::Copy for AutomationAnnotationTypeRegistration {}
impl ::core::clone::Clone for AutomationAnnotationTypeRegistration {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"UI_UIAutomation_Core\"`*"]
pub struct AutomationRemoteOperationOperandId {
    pub Value: i32,
}
impl ::core::marker::Copy for AutomationRemoteOperationOperandId {}
impl ::core::clone::Clone for AutomationRemoteOperationOperandId {
    fn clone(&self) -> Self {
        *self
    }
}
pub type AutomationRemoteOperationResult = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"UI_UIAutomation_Core\"`*"]
#[repr(transparent)]
pub struct AutomationRemoteOperationStatus(pub i32);
impl AutomationRemoteOperationStatus {
    pub const Success: Self = Self(0i32);
    pub const MalformedBytecode: Self = Self(1i32);
    pub const InstructionLimitExceeded: Self = Self(2i32);
    pub const UnhandledException: Self = Self(3i32);
    pub const ExecutionFailure: Self = Self(4i32);
}
impl ::core::marker::Copy for AutomationRemoteOperationStatus {}
impl ::core::clone::Clone for AutomationRemoteOperationStatus {
    fn clone(&self) -> Self {
        *self
    }
}
pub type CoreAutomationRemoteOperation = *mut ::core::ffi::c_void;
pub type CoreAutomationRemoteOperationContext = *mut ::core::ffi::c_void;
#[repr(C)]
pub struct IAutomationRemoteOperationResult {
    pub base__: ::windows_sys::core::IInspectable,
    pub Status: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut AutomationRemoteOperationStatus) -> ::windows_sys::core::HRESULT,
    pub ExtendedError: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HRESULT) -> ::windows_sys::core::HRESULT,
    pub ErrorLocation: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub HasOperand: unsafe extern "system" fn(this: *mut *mut Self, operandid: AutomationRemoteOperationOperandId, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub GetOperand: unsafe extern "system" fn(this: *mut *mut Self, operandid: AutomationRemoteOperationOperandId, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IAutomationRemoteOperationResult {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3774352450, data2: 19047, data3: 21812, data4: [191, 90, 9, 232, 169, 155, 54, 177] };
}
#[repr(C)]
pub struct ICoreAutomationConnectionBoundObjectProvider {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsComThreadingRequired: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ICoreAutomationConnectionBoundObjectProvider {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 102808420, data2: 38422, data3: 21907, data4: [190, 58, 235, 142, 109, 174, 179, 250] };
}
#[repr(C)]
pub struct ICoreAutomationRegistrarStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub RegisterAnnotationType: unsafe extern "system" fn(this: *mut *mut Self, guid: ::windows_sys::core::GUID, result__: *mut AutomationAnnotationTypeRegistration) -> ::windows_sys::core::HRESULT,
    pub UnregisterAnnotationType: unsafe extern "system" fn(this: *mut *mut Self, registration: AutomationAnnotationTypeRegistration) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ICoreAutomationRegistrarStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1045435035, data2: 55004, data3: 22144, data4: [181, 128, 255, 255, 120, 48, 3, 4] };
}
#[repr(C)]
pub struct ICoreAutomationRemoteOperation {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsOpcodeSupported: unsafe extern "system" fn(this: *mut *mut Self, opcode: u32, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub ImportElement: unsafe extern "system" fn(this: *mut *mut Self, operandid: AutomationRemoteOperationOperandId, element: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub ImportTextRange: unsafe extern "system" fn(this: *mut *mut Self, operandid: AutomationRemoteOperationOperandId, textrange: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub AddToResults: unsafe extern "system" fn(this: *mut *mut Self, operandid: AutomationRemoteOperationOperandId) -> ::windows_sys::core::HRESULT,
    pub Execute: unsafe extern "system" fn(this: *mut *mut Self, bytecodeBuffer_array_size: u32, bytecodebuffer: *const u8, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ICoreAutomationRemoteOperation {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 986076916, data2: 58044, data3: 23662, data4: [184, 231, 178, 36, 251, 116, 176, 96] };
}
#[repr(C)]
pub struct ICoreAutomationRemoteOperation2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub ImportConnectionBoundObject: unsafe extern "system" fn(this: *mut *mut Self, operandid: AutomationRemoteOperationOperandId, connectionboundobject: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ICoreAutomationRemoteOperation2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4009425007, data2: 59731, data3: 20633, data4: [140, 233, 220, 168, 19, 72, 43, 160] };
}
#[repr(C)]
pub struct ICoreAutomationRemoteOperationContext {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetOperand: unsafe extern "system" fn(this: *mut *mut Self, id: AutomationRemoteOperationOperandId, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetOperand: unsafe extern "system" fn(this: *mut *mut Self, id: AutomationRemoteOperationOperandId, operand: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetOperand2: unsafe extern "system" fn(this: *mut *mut Self, id: AutomationRemoteOperationOperandId, operand: *mut ::core::ffi::c_void, operandinterfaceid: ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ICoreAutomationRemoteOperationContext {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3115293883, data2: 15678, data3: 22808, data4: [161, 107, 120, 97, 98, 106, 58, 235] };
}
#[repr(C)]
pub struct ICoreAutomationRemoteOperationExtensionProvider {
    pub base__: ::windows_sys::core::IInspectable,
    pub CallExtension: unsafe extern "system" fn(this: *mut *mut Self, extensionid: ::windows_sys::core::GUID, context: *mut ::core::ffi::c_void, operandIds_array_size: u32, operandids: *const AutomationRemoteOperationOperandId) -> ::windows_sys::core::HRESULT,
    pub IsExtensionSupported: unsafe extern "system" fn(this: *mut *mut Self, extensionid: ::windows_sys::core::GUID, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ICoreAutomationRemoteOperationExtensionProvider {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2297773671, data2: 56425, data3: 21819, data4: [160, 170, 112, 71, 126, 114, 77, 168] };
}
#[repr(C)]
pub struct IRemoteAutomationClientSession {
    pub base__: ::windows_sys::core::IInspectable,
    pub Start: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Stop: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub CreateWindowAsync: unsafe extern "system" fn(this: *mut *mut Self, remotewindowid: u64, remoteprocessid: u32, parentautomationelement: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateWindowAsync: usize,
    pub SessionId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ConnectionRequested: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ConnectionRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveConnectionRequested: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveConnectionRequested: usize,
    #[cfg(feature = "Foundation")]
    pub Disconnected: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Disconnected: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveDisconnected: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveDisconnected: usize,
}
impl ::windows_sys::core::Interface for IRemoteAutomationClientSession {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1552550173, data2: 38092, data3: 23347, data4: [175, 219, 103, 140, 222, 210, 189, 84] };
}
#[repr(C)]
pub struct IRemoteAutomationClientSessionFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateInstance2: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::HSTRING, sessionid: ::windows_sys::core::GUID, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IRemoteAutomationClientSessionFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4065338941, data2: 24663, data3: 21363, data4: [165, 165, 237, 114, 101, 254, 3, 118] };
}
#[repr(C)]
pub struct IRemoteAutomationConnectionRequestedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub LocalPipeName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub RemoteProcessId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IRemoteAutomationConnectionRequestedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3929217448, data2: 58280, data3: 24006, data4: [173, 248, 4, 78, 70, 177, 74, 245] };
}
#[repr(C)]
pub struct IRemoteAutomationDisconnectedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub LocalPipeName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IRemoteAutomationDisconnectedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3149085245, data2: 23952, data3: 23608, data4: [158, 178, 221, 157, 204, 27, 46, 63] };
}
#[repr(C)]
pub struct IRemoteAutomationServerStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub ReportSession: unsafe extern "system" fn(this: *mut *mut Self, sessionid: ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IRemoteAutomationServerStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3874002014, data2: 3089, data3: 20520, data4: [154, 227, 194, 119, 18, 136, 182, 183] };
}
#[repr(C)]
pub struct IRemoteAutomationWindow {
    pub base__: ::windows_sys::core::IInspectable,
    pub AutomationProvider: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub UnregisterAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    UnregisterAsync: usize,
}
impl ::windows_sys::core::Interface for IRemoteAutomationWindow {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2086696585, data2: 18797, data3: 20778, data4: [155, 213, 192, 80, 207, 175, 20, 40] };
}
pub type RemoteAutomationClientSession = *mut ::core::ffi::c_void;
pub type RemoteAutomationConnectionRequestedEventArgs = *mut ::core::ffi::c_void;
pub type RemoteAutomationDisconnectedEventArgs = *mut ::core::ffi::c_void;
pub type RemoteAutomationWindow = *mut ::core::ffi::c_void;
