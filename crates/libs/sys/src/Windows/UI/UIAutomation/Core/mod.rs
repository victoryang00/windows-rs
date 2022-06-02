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
#[repr(C)]
pub struct ICoreAutomationConnectionBoundObjectProvider {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsComThreadingRequired: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ICoreAutomationRegistrarStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub RegisterAnnotationType: unsafe extern "system" fn(this: *mut *mut Self, guid: ::windows_sys::core::GUID, result__: *mut AutomationAnnotationTypeRegistration) -> ::windows_sys::core::HRESULT,
    pub UnregisterAnnotationType: unsafe extern "system" fn(this: *mut *mut Self, registration: AutomationAnnotationTypeRegistration) -> ::windows_sys::core::HRESULT,
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
#[repr(C)]
pub struct ICoreAutomationRemoteOperation2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub ImportConnectionBoundObject: unsafe extern "system" fn(this: *mut *mut Self, operandid: AutomationRemoteOperationOperandId, connectionboundobject: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ICoreAutomationRemoteOperationContext {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetOperand: unsafe extern "system" fn(this: *mut *mut Self, id: AutomationRemoteOperationOperandId, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetOperand: unsafe extern "system" fn(this: *mut *mut Self, id: AutomationRemoteOperationOperandId, operand: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetOperand2: unsafe extern "system" fn(this: *mut *mut Self, id: AutomationRemoteOperationOperandId, operand: *mut ::core::ffi::c_void, operandinterfaceid: ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ICoreAutomationRemoteOperationExtensionProvider {
    pub base__: ::windows_sys::core::IInspectable,
    pub CallExtension: unsafe extern "system" fn(this: *mut *mut Self, extensionid: ::windows_sys::core::GUID, context: *mut ::core::ffi::c_void, operandIds_array_size: u32, operandids: *const AutomationRemoteOperationOperandId) -> ::windows_sys::core::HRESULT,
    pub IsExtensionSupported: unsafe extern "system" fn(this: *mut *mut Self, extensionid: ::windows_sys::core::GUID, result__: *mut bool) -> ::windows_sys::core::HRESULT,
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
#[repr(C)]
pub struct IRemoteAutomationClientSessionFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateInstance: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateInstance2: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::HSTRING, sessionid: ::windows_sys::core::GUID, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IRemoteAutomationConnectionRequestedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub LocalPipeName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub RemoteProcessId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IRemoteAutomationDisconnectedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub LocalPipeName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IRemoteAutomationServerStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub ReportSession: unsafe extern "system" fn(this: *mut *mut Self, sessionid: ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
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
pub type RemoteAutomationClientSession = *mut ::core::ffi::c_void;
pub type RemoteAutomationConnectionRequestedEventArgs = *mut ::core::ffi::c_void;
pub type RemoteAutomationDisconnectedEventArgs = *mut ::core::ffi::c_void;
pub type RemoteAutomationWindow = *mut ::core::ffi::c_void;
