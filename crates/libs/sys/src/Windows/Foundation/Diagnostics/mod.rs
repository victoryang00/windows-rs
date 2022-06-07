#[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
#[repr(transparent)]
pub struct CausalityRelation(pub i32);
impl CausalityRelation {
    pub const AssignDelegate: Self = Self(0i32);
    pub const Join: Self = Self(1i32);
    pub const Choice: Self = Self(2i32);
    pub const Cancel: Self = Self(3i32);
    pub const Error: Self = Self(4i32);
}
impl ::core::marker::Copy for CausalityRelation {}
impl ::core::clone::Clone for CausalityRelation {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
#[repr(transparent)]
pub struct CausalitySource(pub i32);
impl CausalitySource {
    pub const Application: Self = Self(0i32);
    pub const Library: Self = Self(1i32);
    pub const System: Self = Self(2i32);
}
impl ::core::marker::Copy for CausalitySource {}
impl ::core::clone::Clone for CausalitySource {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
#[repr(transparent)]
pub struct CausalitySynchronousWork(pub i32);
impl CausalitySynchronousWork {
    pub const CompletionNotification: Self = Self(0i32);
    pub const ProgressNotification: Self = Self(1i32);
    pub const Execution: Self = Self(2i32);
}
impl ::core::marker::Copy for CausalitySynchronousWork {}
impl ::core::clone::Clone for CausalitySynchronousWork {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
#[repr(transparent)]
pub struct CausalityTraceLevel(pub i32);
impl CausalityTraceLevel {
    pub const Required: Self = Self(0i32);
    pub const Important: Self = Self(1i32);
    pub const Verbose: Self = Self(2i32);
}
impl ::core::marker::Copy for CausalityTraceLevel {}
impl ::core::clone::Clone for CausalityTraceLevel {
    fn clone(&self) -> Self {
        *self
    }
}
pub type ErrorDetails = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
#[repr(transparent)]
pub struct ErrorOptions(pub u32);
impl ErrorOptions {
    pub const None: Self = Self(0u32);
    pub const SuppressExceptions: Self = Self(1u32);
    pub const ForceExceptions: Self = Self(2u32);
    pub const UseSetErrorInfo: Self = Self(4u32);
    pub const SuppressSetErrorInfo: Self = Self(8u32);
}
impl ::core::marker::Copy for ErrorOptions {}
impl ::core::clone::Clone for ErrorOptions {
    fn clone(&self) -> Self {
        *self
    }
}
pub type FileLoggingSession = *mut ::core::ffi::c_void;
#[repr(C)]
pub struct IAsyncCausalityTracerStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub TraceOperationCreation: unsafe extern "system" fn(this: *mut *mut Self, tracelevel: CausalityTraceLevel, source: CausalitySource, platformid: ::windows_sys::core::GUID, operationid: u64, operationname: ::windows_sys::core::HSTRING, relatedcontext: u64) -> ::windows_sys::core::HRESULT,
    pub TraceOperationCompletion: unsafe extern "system" fn(this: *mut *mut Self, tracelevel: CausalityTraceLevel, source: CausalitySource, platformid: ::windows_sys::core::GUID, operationid: u64, status: super::AsyncStatus) -> ::windows_sys::core::HRESULT,
    pub TraceOperationRelation: unsafe extern "system" fn(this: *mut *mut Self, tracelevel: CausalityTraceLevel, source: CausalitySource, platformid: ::windows_sys::core::GUID, operationid: u64, relation: CausalityRelation) -> ::windows_sys::core::HRESULT,
    pub TraceSynchronousWorkStart: unsafe extern "system" fn(this: *mut *mut Self, tracelevel: CausalityTraceLevel, source: CausalitySource, platformid: ::windows_sys::core::GUID, operationid: u64, work: CausalitySynchronousWork) -> ::windows_sys::core::HRESULT,
    pub TraceSynchronousWorkCompletion: unsafe extern "system" fn(this: *mut *mut Self, tracelevel: CausalityTraceLevel, source: CausalitySource, work: CausalitySynchronousWork) -> ::windows_sys::core::HRESULT,
    pub TracingStatusChanged: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    pub RemoveTracingStatusChanged: unsafe extern "system" fn(this: *mut *mut Self, cookie: super::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IAsyncCausalityTracerStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1350896422, data2: 9854, data3: 17691, data4: [168, 144, 171, 106, 55, 2, 69, 238] };
}
#[repr(C)]
pub struct IErrorDetails {
    pub base__: ::windows_sys::core::IInspectable,
    pub Description: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub LongDescription: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub HelpUri: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IErrorDetails {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 931969793, data2: 11465, data3: 17039, data4: [140, 85, 44, 153, 13, 70, 62, 143] };
}
#[repr(C)]
pub struct IErrorDetailsStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateFromHResultAsync: unsafe extern "system" fn(this: *mut *mut Self, errorcode: i32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IErrorDetailsStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3077584720, data2: 2845, data3: 18120, data4: [170, 14, 75, 129, 120, 228, 252, 233] };
}
#[repr(C)]
pub struct IErrorReportingSettings {
    pub base__: ::windows_sys::core::IInspectable,
    pub SetErrorOptions: unsafe extern "system" fn(this: *mut *mut Self, value: ErrorOptions) -> ::windows_sys::core::HRESULT,
    pub GetErrorOptions: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ErrorOptions) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IErrorReportingSettings {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 372676498, data2: 45118, data3: 19361, data4: [139, 184, 210, 143, 74, 180, 210, 192] };
}
#[repr(C)]
pub struct IFileLoggingSession {
    pub base__: ::windows_sys::core::IInspectable,
    pub Name: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub AddLoggingChannel: unsafe extern "system" fn(this: *mut *mut Self, loggingchannel: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub AddLoggingChannelWithLevel: unsafe extern "system" fn(this: *mut *mut Self, loggingchannel: *mut ::core::ffi::c_void, maxlevel: LoggingLevel) -> ::windows_sys::core::HRESULT,
    pub RemoveLoggingChannel: unsafe extern "system" fn(this: *mut *mut Self, loggingchannel: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Storage")]
    pub CloseAndSaveToFileAsync: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage"))]
    CloseAndSaveToFileAsync: usize,
    pub LogFileGenerated: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    pub RemoveLogFileGenerated: unsafe extern "system" fn(this: *mut *mut Self, token: super::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IFileLoggingSession {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 617038358, data2: 65234, data3: 16460, data4: [137, 95, 31, 150, 153, 203, 2, 247] };
}
#[repr(C)]
pub struct IFileLoggingSessionFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IFileLoggingSessionFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4003499470, data2: 33863, data3: 19882, data4: [145, 51, 18, 235, 70, 246, 151, 212] };
}
#[repr(C)]
pub struct ILogFileGeneratedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Storage")]
    pub File: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage"))]
    File: usize,
}
impl ::windows_sys::core::Interface for ILogFileGeneratedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 647927663, data2: 3384, data3: 19482, data4: [181, 63, 179, 149, 216, 129, 223, 132] };
}
#[repr(C)]
pub struct ILoggingActivity {
    pub base__: ::windows_sys::core::IInspectable,
    pub Name: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Id: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ILoggingActivity {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3154323777, data2: 46950, data3: 19637, data4: [152, 72, 151, 172, 107, 166, 214, 12] };
}
#[repr(C)]
pub struct ILoggingActivity2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub Channel: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub StopActivity: unsafe extern "system" fn(this: *mut *mut Self, stopeventname: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub StopActivityWithFields: unsafe extern "system" fn(this: *mut *mut Self, stopeventname: ::windows_sys::core::HSTRING, fields: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub StopActivityWithFieldsAndOptions: unsafe extern "system" fn(this: *mut *mut Self, stopeventname: ::windows_sys::core::HSTRING, fields: *mut ::core::ffi::c_void, options: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ILoggingActivity2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 650287112, data2: 25378, data3: 17770, data4: [175, 130, 128, 200, 100, 47, 23, 139] };
}
#[repr(C)]
pub struct ILoggingActivityFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateLoggingActivity: unsafe extern "system" fn(this: *mut *mut Self, activityname: ::windows_sys::core::HSTRING, loggingchannel: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateLoggingActivityWithLevel: unsafe extern "system" fn(this: *mut *mut Self, activityname: ::windows_sys::core::HSTRING, loggingchannel: *mut ::core::ffi::c_void, level: LoggingLevel, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ILoggingActivityFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1798550659, data2: 57610, data3: 19544, data4: [151, 213, 16, 251, 69, 16, 116, 251] };
}
#[repr(C)]
pub struct ILoggingChannel {
    pub base__: ::windows_sys::core::IInspectable,
    pub Name: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Enabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub Level: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut LoggingLevel) -> ::windows_sys::core::HRESULT,
    pub LogMessage: unsafe extern "system" fn(this: *mut *mut Self, eventstring: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub LogMessageWithLevel: unsafe extern "system" fn(this: *mut *mut Self, eventstring: ::windows_sys::core::HSTRING, level: LoggingLevel) -> ::windows_sys::core::HRESULT,
    pub LogValuePair: unsafe extern "system" fn(this: *mut *mut Self, value1: ::windows_sys::core::HSTRING, value2: i32) -> ::windows_sys::core::HRESULT,
    pub LogValuePairWithLevel: unsafe extern "system" fn(this: *mut *mut Self, value1: ::windows_sys::core::HSTRING, value2: i32, level: LoggingLevel) -> ::windows_sys::core::HRESULT,
    pub LoggingEnabled: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut super::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    pub RemoveLoggingEnabled: unsafe extern "system" fn(this: *mut *mut Self, token: super::EventRegistrationToken) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ILoggingChannel {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3919905603, data2: 4567, data3: 20225, data4: [181, 202, 207, 73, 82, 120, 192, 168] };
}
#[repr(C)]
pub struct ILoggingChannel2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub Id: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ILoggingChannel2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2672573683, data2: 2988, data3: 17829, data4: [158, 51, 186, 243, 243, 162, 70, 165] };
}
#[repr(C)]
pub struct ILoggingChannelFactory {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "deprecated")]
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Create: usize,
}
impl ::windows_sys::core::Interface for ILoggingChannelFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1323064220, data2: 44928, data3: 19099, data4: [176, 220, 57, 143, 154, 229, 32, 123] };
}
#[repr(C)]
pub struct ILoggingChannelFactory2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateWithOptions: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::HSTRING, options: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateWithOptionsAndId: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::HSTRING, options: *mut ::core::ffi::c_void, id: ::windows_sys::core::GUID, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ILoggingChannelFactory2 {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1282340317, data2: 15143, data3: 19913, data4: [153, 240, 41, 156, 110, 70, 3, 161] };
}
#[repr(C)]
pub struct ILoggingChannelOptions {
    pub base__: ::windows_sys::core::IInspectable,
    pub Group: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub SetGroup: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ILoggingChannelOptions {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3286779903, data2: 3771, data3: 19027, data4: [140, 84, 222, 194, 73, 38, 203, 44] };
}
#[repr(C)]
pub struct ILoggingChannelOptionsFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, group: ::windows_sys::core::GUID, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ILoggingChannelOptionsFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2838581722, data2: 32687, data3: 16785, data4: [135, 85, 94, 134, 220, 101, 216, 150] };
}
#[repr(C)]
pub struct ILoggingFields {
    pub base__: ::windows_sys::core::IInspectable,
    pub Clear: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub BeginStruct: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub BeginStructWithTags: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::HSTRING, tags: i32) -> ::windows_sys::core::HRESULT,
    pub EndStruct: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub AddEmpty: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub AddEmptyWithFormat: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::HSTRING, format: LoggingFieldFormat) -> ::windows_sys::core::HRESULT,
    pub AddEmptyWithFormatAndTags: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::HSTRING, format: LoggingFieldFormat, tags: i32) -> ::windows_sys::core::HRESULT,
    pub AddUInt8: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::HSTRING, value: u8) -> ::windows_sys::core::HRESULT,
    pub AddUInt8WithFormat: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::HSTRING, value: u8, format: LoggingFieldFormat) -> ::windows_sys::core::HRESULT,
    pub AddUInt8WithFormatAndTags: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::HSTRING, value: u8, format: LoggingFieldFormat, tags: i32) -> ::windows_sys::core::HRESULT,
    pub AddUInt8Array: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::HSTRING, value_array_size: u32, value: *const u8) -> ::windows_sys::core::HRESULT,
    pub AddUInt8ArrayWithFormat: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::HSTRING, value_array_size: u32, value: *const u8, format: LoggingFieldFormat) -> ::windows_sys::core::HRESULT,
    pub AddUInt8ArrayWithFormatAndTags: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::HSTRING, value_array_size: u32, value: *const u8, format: LoggingFieldFormat, tags: i32) -> ::windows_sys::core::HRESULT,
    pub AddInt16: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::HSTRING, value: i16) -> ::windows_sys::core::HRESULT,
    pub AddInt16WithFormat: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::HSTRING, value: i16, format: LoggingFieldFormat) -> ::windows_sys::core::HRESULT,
    pub AddInt16WithFormatAndTags: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::HSTRING, value: i16, format: LoggingFieldFormat, tags: i32) -> ::windows_sys::core::HRESULT,
    pub AddInt16Array: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::HSTRING, value_array_size: u32, value: *const i16) -> ::windows_sys::core::HRESULT,
    pub AddInt16ArrayWithFormat: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::HSTRING, value_array_size: u32, value: *const i16, format: LoggingFieldFormat) -> ::windows_sys::core::HRESULT,
    pub AddInt16ArrayWithFormatAndTags: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::HSTRING, value_array_size: u32, value: *const i16, format: LoggingFieldFormat, tags: i32) -> ::windows_sys::core::HRESULT,
    pub AddUInt16: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::HSTRING, value: u16) -> ::windows_sys::core::HRESULT,
    pub AddUInt16WithFormat: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::HSTRING, value: u16, format: LoggingFieldFormat) -> ::windows_sys::core::HRESULT,
    pub AddUInt16WithFormatAndTags: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::HSTRING, value: u16, format: LoggingFieldFormat, tags: i32) -> ::windows_sys::core::HRESULT,
    pub AddUInt16Array: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::HSTRING, value_array_size: u32, value: *const u16) -> ::windows_sys::core::HRESULT,
    pub AddUInt16ArrayWithFormat: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::HSTRING, value_array_size: u32, value: *const u16, format: LoggingFieldFormat) -> ::windows_sys::core::HRESULT,
    pub AddUInt16ArrayWithFormatAndTags: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::HSTRING, value_array_size: u32, value: *const u16, format: LoggingFieldFormat, tags: i32) -> ::windows_sys::core::HRESULT,
    pub AddInt32: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::HSTRING, value: i32) -> ::windows_sys::core::HRESULT,
    pub AddInt32WithFormat: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::HSTRING, value: i32, format: LoggingFieldFormat) -> ::windows_sys::core::HRESULT,
    pub AddInt32WithFormatAndTags: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::HSTRING, value: i32, format: LoggingFieldFormat, tags: i32) -> ::windows_sys::core::HRESULT,
    pub AddInt32Array: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::HSTRING, value_array_size: u32, value: *const i32) -> ::windows_sys::core::HRESULT,
    pub AddInt32ArrayWithFormat: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::HSTRING, value_array_size: u32, value: *const i32, format: LoggingFieldFormat) -> ::windows_sys::core::HRESULT,
    pub AddInt32ArrayWithFormatAndTags: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::HSTRING, value_array_size: u32, value: *const i32, format: LoggingFieldFormat, tags: i32) -> ::windows_sys::core::HRESULT,
    pub AddUInt32: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::HSTRING, value: u32) -> ::windows_sys::core::HRESULT,
    pub AddUInt32WithFormat: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::HSTRING, value: u32, format: LoggingFieldFormat) -> ::windows_sys::core::HRESULT,
    pub AddUInt32WithFormatAndTags: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::HSTRING, value: u32, format: LoggingFieldFormat, tags: i32) -> ::windows_sys::core::HRESULT,
    pub AddUInt32Array: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::HSTRING, value_array_size: u32, value: *const u32) -> ::windows_sys::core::HRESULT,
    pub AddUInt32ArrayWithFormat: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::HSTRING, value_array_size: u32, value: *const u32, format: LoggingFieldFormat) -> ::windows_sys::core::HRESULT,
    pub AddUInt32ArrayWithFormatAndTags: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::HSTRING, value_array_size: u32, value: *const u32, format: LoggingFieldFormat, tags: i32) -> ::windows_sys::core::HRESULT,
    pub AddInt64: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::HSTRING, value: i64) -> ::windows_sys::core::HRESULT,
    pub AddInt64WithFormat: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::HSTRING, value: i64, format: LoggingFieldFormat) -> ::windows_sys::core::HRESULT,
    pub AddInt64WithFormatAndTags: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::HSTRING, value: i64, format: LoggingFieldFormat, tags: i32) -> ::windows_sys::core::HRESULT,
    pub AddInt64Array: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::HSTRING, value_array_size: u32, value: *const i64) -> ::windows_sys::core::HRESULT,
    pub AddInt64ArrayWithFormat: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::HSTRING, value_array_size: u32, value: *const i64, format: LoggingFieldFormat) -> ::windows_sys::core::HRESULT,
    pub AddInt64ArrayWithFormatAndTags: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::HSTRING, value_array_size: u32, value: *const i64, format: LoggingFieldFormat, tags: i32) -> ::windows_sys::core::HRESULT,
    pub AddUInt64: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::HSTRING, value: u64) -> ::windows_sys::core::HRESULT,
    pub AddUInt64WithFormat: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::HSTRING, value: u64, format: LoggingFieldFormat) -> ::windows_sys::core::HRESULT,
    pub AddUInt64WithFormatAndTags: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::HSTRING, value: u64, format: LoggingFieldFormat, tags: i32) -> ::windows_sys::core::HRESULT,
    pub AddUInt64Array: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::HSTRING, value_array_size: u32, value: *const u64) -> ::windows_sys::core::HRESULT,
    pub AddUInt64ArrayWithFormat: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::HSTRING, value_array_size: u32, value: *const u64, format: LoggingFieldFormat) -> ::windows_sys::core::HRESULT,
    pub AddUInt64ArrayWithFormatAndTags: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::HSTRING, value_array_size: u32, value: *const u64, format: LoggingFieldFormat, tags: i32) -> ::windows_sys::core::HRESULT,
    pub AddSingle: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::HSTRING, value: f32) -> ::windows_sys::core::HRESULT,
    pub AddSingleWithFormat: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::HSTRING, value: f32, format: LoggingFieldFormat) -> ::windows_sys::core::HRESULT,
    pub AddSingleWithFormatAndTags: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::HSTRING, value: f32, format: LoggingFieldFormat, tags: i32) -> ::windows_sys::core::HRESULT,
    pub AddSingleArray: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::HSTRING, value_array_size: u32, value: *const f32) -> ::windows_sys::core::HRESULT,
    pub AddSingleArrayWithFormat: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::HSTRING, value_array_size: u32, value: *const f32, format: LoggingFieldFormat) -> ::windows_sys::core::HRESULT,
    pub AddSingleArrayWithFormatAndTags: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::HSTRING, value_array_size: u32, value: *const f32, format: LoggingFieldFormat, tags: i32) -> ::windows_sys::core::HRESULT,
    pub AddDouble: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::HSTRING, value: f64) -> ::windows_sys::core::HRESULT,
    pub AddDoubleWithFormat: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::HSTRING, value: f64, format: LoggingFieldFormat) -> ::windows_sys::core::HRESULT,
    pub AddDoubleWithFormatAndTags: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::HSTRING, value: f64, format: LoggingFieldFormat, tags: i32) -> ::windows_sys::core::HRESULT,
    pub AddDoubleArray: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::HSTRING, value_array_size: u32, value: *const f64) -> ::windows_sys::core::HRESULT,
    pub AddDoubleArrayWithFormat: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::HSTRING, value_array_size: u32, value: *const f64, format: LoggingFieldFormat) -> ::windows_sys::core::HRESULT,
    pub AddDoubleArrayWithFormatAndTags: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::HSTRING, value_array_size: u32, value: *const f64, format: LoggingFieldFormat, tags: i32) -> ::windows_sys::core::HRESULT,
    pub AddChar16: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::HSTRING, value: u16) -> ::windows_sys::core::HRESULT,
    pub AddChar16WithFormat: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::HSTRING, value: u16, format: LoggingFieldFormat) -> ::windows_sys::core::HRESULT,
    pub AddChar16WithFormatAndTags: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::HSTRING, value: u16, format: LoggingFieldFormat, tags: i32) -> ::windows_sys::core::HRESULT,
    pub AddChar16Array: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::HSTRING, value_array_size: u32, value: *const u16) -> ::windows_sys::core::HRESULT,
    pub AddChar16ArrayWithFormat: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::HSTRING, value_array_size: u32, value: *const u16, format: LoggingFieldFormat) -> ::windows_sys::core::HRESULT,
    pub AddChar16ArrayWithFormatAndTags: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::HSTRING, value_array_size: u32, value: *const u16, format: LoggingFieldFormat, tags: i32) -> ::windows_sys::core::HRESULT,
    pub AddBoolean: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::HSTRING, value: bool) -> ::windows_sys::core::HRESULT,
    pub AddBooleanWithFormat: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::HSTRING, value: bool, format: LoggingFieldFormat) -> ::windows_sys::core::HRESULT,
    pub AddBooleanWithFormatAndTags: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::HSTRING, value: bool, format: LoggingFieldFormat, tags: i32) -> ::windows_sys::core::HRESULT,
    pub AddBooleanArray: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::HSTRING, value_array_size: u32, value: *const bool) -> ::windows_sys::core::HRESULT,
    pub AddBooleanArrayWithFormat: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::HSTRING, value_array_size: u32, value: *const bool, format: LoggingFieldFormat) -> ::windows_sys::core::HRESULT,
    pub AddBooleanArrayWithFormatAndTags: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::HSTRING, value_array_size: u32, value: *const bool, format: LoggingFieldFormat, tags: i32) -> ::windows_sys::core::HRESULT,
    pub AddString: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::HSTRING, value: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub AddStringWithFormat: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::HSTRING, value: ::windows_sys::core::HSTRING, format: LoggingFieldFormat) -> ::windows_sys::core::HRESULT,
    pub AddStringWithFormatAndTags: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::HSTRING, value: ::windows_sys::core::HSTRING, format: LoggingFieldFormat, tags: i32) -> ::windows_sys::core::HRESULT,
    pub AddStringArray: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::HSTRING, value_array_size: u32, value: *const ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub AddStringArrayWithFormat: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::HSTRING, value_array_size: u32, value: *const ::windows_sys::core::HSTRING, format: LoggingFieldFormat) -> ::windows_sys::core::HRESULT,
    pub AddStringArrayWithFormatAndTags: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::HSTRING, value_array_size: u32, value: *const ::windows_sys::core::HSTRING, format: LoggingFieldFormat, tags: i32) -> ::windows_sys::core::HRESULT,
    pub AddGuid: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::HSTRING, value: ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub AddGuidWithFormat: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::HSTRING, value: ::windows_sys::core::GUID, format: LoggingFieldFormat) -> ::windows_sys::core::HRESULT,
    pub AddGuidWithFormatAndTags: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::HSTRING, value: ::windows_sys::core::GUID, format: LoggingFieldFormat, tags: i32) -> ::windows_sys::core::HRESULT,
    pub AddGuidArray: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::HSTRING, value_array_size: u32, value: *const ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub AddGuidArrayWithFormat: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::HSTRING, value_array_size: u32, value: *const ::windows_sys::core::GUID, format: LoggingFieldFormat) -> ::windows_sys::core::HRESULT,
    pub AddGuidArrayWithFormatAndTags: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::HSTRING, value_array_size: u32, value: *const ::windows_sys::core::GUID, format: LoggingFieldFormat, tags: i32) -> ::windows_sys::core::HRESULT,
    pub AddDateTime: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::HSTRING, value: super::DateTime) -> ::windows_sys::core::HRESULT,
    pub AddDateTimeWithFormat: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::HSTRING, value: super::DateTime, format: LoggingFieldFormat) -> ::windows_sys::core::HRESULT,
    pub AddDateTimeWithFormatAndTags: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::HSTRING, value: super::DateTime, format: LoggingFieldFormat, tags: i32) -> ::windows_sys::core::HRESULT,
    pub AddDateTimeArray: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::HSTRING, value_array_size: u32, value: *const super::DateTime) -> ::windows_sys::core::HRESULT,
    pub AddDateTimeArrayWithFormat: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::HSTRING, value_array_size: u32, value: *const super::DateTime, format: LoggingFieldFormat) -> ::windows_sys::core::HRESULT,
    pub AddDateTimeArrayWithFormatAndTags: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::HSTRING, value_array_size: u32, value: *const super::DateTime, format: LoggingFieldFormat, tags: i32) -> ::windows_sys::core::HRESULT,
    pub AddTimeSpan: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::HSTRING, value: super::TimeSpan) -> ::windows_sys::core::HRESULT,
    pub AddTimeSpanWithFormat: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::HSTRING, value: super::TimeSpan, format: LoggingFieldFormat) -> ::windows_sys::core::HRESULT,
    pub AddTimeSpanWithFormatAndTags: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::HSTRING, value: super::TimeSpan, format: LoggingFieldFormat, tags: i32) -> ::windows_sys::core::HRESULT,
    pub AddTimeSpanArray: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::HSTRING, value_array_size: u32, value: *const super::TimeSpan) -> ::windows_sys::core::HRESULT,
    pub AddTimeSpanArrayWithFormat: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::HSTRING, value_array_size: u32, value: *const super::TimeSpan, format: LoggingFieldFormat) -> ::windows_sys::core::HRESULT,
    pub AddTimeSpanArrayWithFormatAndTags: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::HSTRING, value_array_size: u32, value: *const super::TimeSpan, format: LoggingFieldFormat, tags: i32) -> ::windows_sys::core::HRESULT,
    pub AddPoint: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::HSTRING, value: super::Point) -> ::windows_sys::core::HRESULT,
    pub AddPointWithFormat: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::HSTRING, value: super::Point, format: LoggingFieldFormat) -> ::windows_sys::core::HRESULT,
    pub AddPointWithFormatAndTags: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::HSTRING, value: super::Point, format: LoggingFieldFormat, tags: i32) -> ::windows_sys::core::HRESULT,
    pub AddPointArray: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::HSTRING, value_array_size: u32, value: *const super::Point) -> ::windows_sys::core::HRESULT,
    pub AddPointArrayWithFormat: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::HSTRING, value_array_size: u32, value: *const super::Point, format: LoggingFieldFormat) -> ::windows_sys::core::HRESULT,
    pub AddPointArrayWithFormatAndTags: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::HSTRING, value_array_size: u32, value: *const super::Point, format: LoggingFieldFormat, tags: i32) -> ::windows_sys::core::HRESULT,
    pub AddSize: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::HSTRING, value: super::Size) -> ::windows_sys::core::HRESULT,
    pub AddSizeWithFormat: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::HSTRING, value: super::Size, format: LoggingFieldFormat) -> ::windows_sys::core::HRESULT,
    pub AddSizeWithFormatAndTags: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::HSTRING, value: super::Size, format: LoggingFieldFormat, tags: i32) -> ::windows_sys::core::HRESULT,
    pub AddSizeArray: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::HSTRING, value_array_size: u32, value: *const super::Size) -> ::windows_sys::core::HRESULT,
    pub AddSizeArrayWithFormat: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::HSTRING, value_array_size: u32, value: *const super::Size, format: LoggingFieldFormat) -> ::windows_sys::core::HRESULT,
    pub AddSizeArrayWithFormatAndTags: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::HSTRING, value_array_size: u32, value: *const super::Size, format: LoggingFieldFormat, tags: i32) -> ::windows_sys::core::HRESULT,
    pub AddRect: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::HSTRING, value: super::Rect) -> ::windows_sys::core::HRESULT,
    pub AddRectWithFormat: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::HSTRING, value: super::Rect, format: LoggingFieldFormat) -> ::windows_sys::core::HRESULT,
    pub AddRectWithFormatAndTags: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::HSTRING, value: super::Rect, format: LoggingFieldFormat, tags: i32) -> ::windows_sys::core::HRESULT,
    pub AddRectArray: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::HSTRING, value_array_size: u32, value: *const super::Rect) -> ::windows_sys::core::HRESULT,
    pub AddRectArrayWithFormat: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::HSTRING, value_array_size: u32, value: *const super::Rect, format: LoggingFieldFormat) -> ::windows_sys::core::HRESULT,
    pub AddRectArrayWithFormatAndTags: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::HSTRING, value_array_size: u32, value: *const super::Rect, format: LoggingFieldFormat, tags: i32) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ILoggingFields {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3623270319, data2: 30253, data3: 17785, data4: [131, 189, 82, 194, 59, 195, 51, 188] };
}
#[repr(C)]
pub struct ILoggingOptions {
    pub base__: ::windows_sys::core::IInspectable,
    pub Keywords: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i64) -> ::windows_sys::core::HRESULT,
    pub SetKeywords: unsafe extern "system" fn(this: *mut *mut Self, value: i64) -> ::windows_sys::core::HRESULT,
    pub Tags: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub SetTags: unsafe extern "system" fn(this: *mut *mut Self, value: i32) -> ::windows_sys::core::HRESULT,
    pub Task: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i16) -> ::windows_sys::core::HRESULT,
    pub SetTask: unsafe extern "system" fn(this: *mut *mut Self, value: i16) -> ::windows_sys::core::HRESULT,
    pub Opcode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut LoggingOpcode) -> ::windows_sys::core::HRESULT,
    pub SetOpcode: unsafe extern "system" fn(this: *mut *mut Self, value: LoggingOpcode) -> ::windows_sys::core::HRESULT,
    pub ActivityId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub SetActivityId: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub RelatedActivityId: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub SetRelatedActivityId: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ILoggingOptions {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2428270672, data2: 402, data3: 20317, data4: [172, 38, 0, 106, 218, 202, 18, 216] };
}
#[repr(C)]
pub struct ILoggingOptionsFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateWithKeywords: unsafe extern "system" fn(this: *mut *mut Self, keywords: i64, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ILoggingOptionsFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3608397515, data2: 39083, data3: 17995, data4: [159, 34, 163, 38, 132, 120, 54, 138] };
}
#[repr(C)]
pub struct ILoggingSession {
    pub base__: ::windows_sys::core::IInspectable,
    pub Name: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    #[cfg(feature = "Storage")]
    pub SaveToFileAsync: unsafe extern "system" fn(this: *mut *mut Self, folder: *mut ::core::ffi::c_void, filename: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage"))]
    SaveToFileAsync: usize,
    pub AddLoggingChannel: unsafe extern "system" fn(this: *mut *mut Self, loggingchannel: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub AddLoggingChannelWithLevel: unsafe extern "system" fn(this: *mut *mut Self, loggingchannel: *mut ::core::ffi::c_void, maxlevel: LoggingLevel) -> ::windows_sys::core::HRESULT,
    pub RemoveLoggingChannel: unsafe extern "system" fn(this: *mut *mut Self, loggingchannel: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ILoggingSession {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1646392070, data2: 37760, data3: 19159, data4: [186, 245, 65, 234, 147, 16, 215, 104] };
}
#[repr(C)]
pub struct ILoggingSessionFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ILoggingSessionFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1318289125, data2: 22781, data3: 17888, data4: [140, 47, 161, 50, 239, 249, 92, 30] };
}
#[repr(C)]
pub struct ILoggingTarget {
    pub base__: ::windows_sys::core::IInspectable,
    pub IsEnabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsEnabledWithLevel: unsafe extern "system" fn(this: *mut *mut Self, level: LoggingLevel, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub IsEnabledWithLevelAndKeywords: unsafe extern "system" fn(this: *mut *mut Self, level: LoggingLevel, keywords: i64, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub LogEvent: unsafe extern "system" fn(this: *mut *mut Self, eventname: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub LogEventWithFields: unsafe extern "system" fn(this: *mut *mut Self, eventname: ::windows_sys::core::HSTRING, fields: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub LogEventWithFieldsAndLevel: unsafe extern "system" fn(this: *mut *mut Self, eventname: ::windows_sys::core::HSTRING, fields: *mut ::core::ffi::c_void, level: LoggingLevel) -> ::windows_sys::core::HRESULT,
    pub LogEventWithFieldsAndOptions: unsafe extern "system" fn(this: *mut *mut Self, eventname: ::windows_sys::core::HSTRING, fields: *mut ::core::ffi::c_void, level: LoggingLevel, options: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub StartActivity: unsafe extern "system" fn(this: *mut *mut Self, starteventname: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub StartActivityWithFields: unsafe extern "system" fn(this: *mut *mut Self, starteventname: ::windows_sys::core::HSTRING, fields: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub StartActivityWithFieldsAndLevel: unsafe extern "system" fn(this: *mut *mut Self, starteventname: ::windows_sys::core::HSTRING, fields: *mut ::core::ffi::c_void, level: LoggingLevel, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub StartActivityWithFieldsAndOptions: unsafe extern "system" fn(this: *mut *mut Self, starteventname: ::windows_sys::core::HSTRING, fields: *mut ::core::ffi::c_void, level: LoggingLevel, options: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ILoggingTarget {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1710320693, data2: 58248, data3: 20006, data4: [177, 122, 245, 28, 211, 168, 57, 22] };
}
#[repr(C)]
pub struct ITracingStatusChangedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Enabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub TraceLevel: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut CausalityTraceLevel) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for ITracingStatusChangedEventArgs {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1091270417, data2: 65339, data3: 18303, data4: [156, 154, 210, 239, 218, 48, 45, 195] };
}
pub type LogFileGeneratedEventArgs = *mut ::core::ffi::c_void;
pub type LoggingActivity = *mut ::core::ffi::c_void;
pub type LoggingChannel = *mut ::core::ffi::c_void;
pub type LoggingChannelOptions = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
#[repr(transparent)]
pub struct LoggingFieldFormat(pub i32);
impl LoggingFieldFormat {
    pub const Default: Self = Self(0i32);
    pub const Hidden: Self = Self(1i32);
    pub const String: Self = Self(2i32);
    pub const Boolean: Self = Self(3i32);
    pub const Hexadecimal: Self = Self(4i32);
    pub const ProcessId: Self = Self(5i32);
    pub const ThreadId: Self = Self(6i32);
    pub const Port: Self = Self(7i32);
    pub const Ipv4Address: Self = Self(8i32);
    pub const Ipv6Address: Self = Self(9i32);
    pub const SocketAddress: Self = Self(10i32);
    pub const Xml: Self = Self(11i32);
    pub const Json: Self = Self(12i32);
    pub const Win32Error: Self = Self(13i32);
    pub const NTStatus: Self = Self(14i32);
    pub const HResult: Self = Self(15i32);
    pub const FileTime: Self = Self(16i32);
    pub const Signed: Self = Self(17i32);
    pub const Unsigned: Self = Self(18i32);
}
impl ::core::marker::Copy for LoggingFieldFormat {}
impl ::core::clone::Clone for LoggingFieldFormat {
    fn clone(&self) -> Self {
        *self
    }
}
pub type LoggingFields = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
#[repr(transparent)]
pub struct LoggingLevel(pub i32);
impl LoggingLevel {
    pub const Verbose: Self = Self(0i32);
    pub const Information: Self = Self(1i32);
    pub const Warning: Self = Self(2i32);
    pub const Error: Self = Self(3i32);
    pub const Critical: Self = Self(4i32);
}
impl ::core::marker::Copy for LoggingLevel {}
impl ::core::clone::Clone for LoggingLevel {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Foundation_Diagnostics\"`*"]
#[repr(transparent)]
pub struct LoggingOpcode(pub i32);
impl LoggingOpcode {
    pub const Info: Self = Self(0i32);
    pub const Start: Self = Self(1i32);
    pub const Stop: Self = Self(2i32);
    pub const Reply: Self = Self(6i32);
    pub const Resume: Self = Self(7i32);
    pub const Suspend: Self = Self(8i32);
    pub const Send: Self = Self(9i32);
}
impl ::core::marker::Copy for LoggingOpcode {}
impl ::core::clone::Clone for LoggingOpcode {
    fn clone(&self) -> Self {
        *self
    }
}
pub type LoggingOptions = *mut ::core::ffi::c_void;
pub type LoggingSession = *mut ::core::ffi::c_void;
pub type RuntimeBrokerErrorSettings = *mut ::core::ffi::c_void;
pub type TracingStatusChangedEventArgs = *mut ::core::ffi::c_void;
