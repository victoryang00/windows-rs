pub type ErrorReceivedEventArgs = *mut ::core::ffi::c_void;
#[repr(C)]
pub struct IErrorReceivedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Error: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut SerialError) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IPinChangedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub PinChange: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut SerialPinChange) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ISerialDevice {
    pub base__: ::windows_sys::core::IInspectable,
    pub BaudRate: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub SetBaudRate: unsafe extern "system" fn(this: *mut *mut Self, value: u32) -> ::windows_sys::core::HRESULT,
    pub BreakSignalState: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetBreakSignalState: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub BytesReceived: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub CarrierDetectState: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub ClearToSendState: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub DataBits: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u16) -> ::windows_sys::core::HRESULT,
    pub SetDataBits: unsafe extern "system" fn(this: *mut *mut Self, value: u16) -> ::windows_sys::core::HRESULT,
    pub DataSetReadyState: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub Handshake: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut SerialHandshake) -> ::windows_sys::core::HRESULT,
    pub SetHandshake: unsafe extern "system" fn(this: *mut *mut Self, value: SerialHandshake) -> ::windows_sys::core::HRESULT,
    pub IsDataTerminalReadyEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsDataTerminalReadyEnabled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub IsRequestToSendEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub SetIsRequestToSendEnabled: unsafe extern "system" fn(this: *mut *mut Self, value: bool) -> ::windows_sys::core::HRESULT,
    pub Parity: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut SerialParity) -> ::windows_sys::core::HRESULT,
    pub SetParity: unsafe extern "system" fn(this: *mut *mut Self, value: SerialParity) -> ::windows_sys::core::HRESULT,
    pub PortName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ReadTimeout: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReadTimeout: usize,
    #[cfg(feature = "Foundation")]
    pub SetReadTimeout: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetReadTimeout: usize,
    pub StopBits: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut SerialStopBitCount) -> ::windows_sys::core::HRESULT,
    pub SetStopBits: unsafe extern "system" fn(this: *mut *mut Self, value: SerialStopBitCount) -> ::windows_sys::core::HRESULT,
    pub UsbVendorId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u16) -> ::windows_sys::core::HRESULT,
    pub UsbProductId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u16) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub WriteTimeout: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    WriteTimeout: usize,
    #[cfg(feature = "Foundation")]
    pub SetWriteTimeout: unsafe extern "system" fn(this: *mut *mut Self, value: super::super::Foundation::TimeSpan) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetWriteTimeout: usize,
    #[cfg(feature = "Storage_Streams")]
    pub InputStream: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    InputStream: usize,
    #[cfg(feature = "Storage_Streams")]
    pub OutputStream: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    OutputStream: usize,
    #[cfg(feature = "Foundation")]
    pub ErrorReceived: unsafe extern "system" fn(this: *mut *mut Self, reporthandler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ErrorReceived: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveErrorReceived: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveErrorReceived: usize,
    #[cfg(feature = "Foundation")]
    pub PinChanged: unsafe extern "system" fn(this: *mut *mut Self, reporthandler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PinChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemovePinChanged: unsafe extern "system" fn(this: *mut *mut Self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemovePinChanged: usize,
}
#[repr(C)]
pub struct ISerialDeviceStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetDeviceSelector: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub GetDeviceSelectorFromPortName: unsafe extern "system" fn(this: *mut *mut Self, portname: ::windows_sys::core::HSTRING, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub GetDeviceSelectorFromUsbVidPid: unsafe extern "system" fn(this: *mut *mut Self, vendorid: u16, productid: u16, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub FromIdAsync: unsafe extern "system" fn(this: *mut *mut Self, deviceid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FromIdAsync: usize,
}
pub type PinChangedEventArgs = *mut ::core::ffi::c_void;
pub type SerialDevice = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Devices_SerialCommunication\"`*"]
#[repr(transparent)]
pub struct SerialError(pub i32);
impl SerialError {
    pub const Frame: Self = Self(0i32);
    pub const BufferOverrun: Self = Self(1i32);
    pub const ReceiveFull: Self = Self(2i32);
    pub const ReceiveParity: Self = Self(3i32);
    pub const TransmitFull: Self = Self(4i32);
}
impl ::core::marker::Copy for SerialError {}
impl ::core::clone::Clone for SerialError {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Devices_SerialCommunication\"`*"]
#[repr(transparent)]
pub struct SerialHandshake(pub i32);
impl SerialHandshake {
    pub const None: Self = Self(0i32);
    pub const RequestToSend: Self = Self(1i32);
    pub const XOnXOff: Self = Self(2i32);
    pub const RequestToSendXOnXOff: Self = Self(3i32);
}
impl ::core::marker::Copy for SerialHandshake {}
impl ::core::clone::Clone for SerialHandshake {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Devices_SerialCommunication\"`*"]
#[repr(transparent)]
pub struct SerialParity(pub i32);
impl SerialParity {
    pub const None: Self = Self(0i32);
    pub const Odd: Self = Self(1i32);
    pub const Even: Self = Self(2i32);
    pub const Mark: Self = Self(3i32);
    pub const Space: Self = Self(4i32);
}
impl ::core::marker::Copy for SerialParity {}
impl ::core::clone::Clone for SerialParity {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Devices_SerialCommunication\"`*"]
#[repr(transparent)]
pub struct SerialPinChange(pub i32);
impl SerialPinChange {
    pub const BreakSignal: Self = Self(0i32);
    pub const CarrierDetect: Self = Self(1i32);
    pub const ClearToSend: Self = Self(2i32);
    pub const DataSetReady: Self = Self(3i32);
    pub const RingIndicator: Self = Self(4i32);
}
impl ::core::marker::Copy for SerialPinChange {}
impl ::core::clone::Clone for SerialPinChange {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Devices_SerialCommunication\"`*"]
#[repr(transparent)]
pub struct SerialStopBitCount(pub i32);
impl SerialStopBitCount {
    pub const One: Self = Self(0i32);
    pub const OnePointFive: Self = Self(1i32);
    pub const Two: Self = Self(2i32);
}
impl ::core::marker::Copy for SerialStopBitCount {}
impl ::core::clone::Clone for SerialStopBitCount {
    fn clone(&self) -> Self {
        *self
    }
}
