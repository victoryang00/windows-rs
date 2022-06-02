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
#[repr(C)]
pub struct IErrorDetails {
    pub base__: ::windows_sys::core::IInspectable,
    pub Description: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub LongDescription: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub HelpUri: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IErrorDetailsStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateFromHResultAsync: unsafe extern "system" fn(this: *mut *mut Self, errorcode: i32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IErrorReportingSettings {
    pub base__: ::windows_sys::core::IInspectable,
    pub SetErrorOptions: unsafe extern "system" fn(this: *mut *mut Self, value: ErrorOptions) -> ::windows_sys::core::HRESULT,
    pub GetErrorOptions: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ErrorOptions) -> ::windows_sys::core::HRESULT,
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
#[repr(C)]
pub struct IFileLoggingSessionFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ILogFileGeneratedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "Storage")]
    pub File: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "Storage"))]
    File: usize,
}
#[repr(C)]
pub struct ILoggingActivity {
    pub base__: ::windows_sys::core::IInspectable,
    pub Name: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Id: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ILoggingActivity2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub Channel: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub StopActivity: unsafe extern "system" fn(this: *mut *mut Self, stopeventname: ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub StopActivityWithFields: unsafe extern "system" fn(this: *mut *mut Self, stopeventname: ::windows_sys::core::HSTRING, fields: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub StopActivityWithFieldsAndOptions: unsafe extern "system" fn(this: *mut *mut Self, stopeventname: ::windows_sys::core::HSTRING, fields: *mut ::core::ffi::c_void, options: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ILoggingActivityFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateLoggingActivity: unsafe extern "system" fn(this: *mut *mut Self, activityname: ::windows_sys::core::HSTRING, loggingchannel: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateLoggingActivityWithLevel: unsafe extern "system" fn(this: *mut *mut Self, activityname: ::windows_sys::core::HSTRING, loggingchannel: *mut ::core::ffi::c_void, level: LoggingLevel, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
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
#[repr(C)]
pub struct ILoggingChannel2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub Id: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ILoggingChannelFactory {
    pub base__: ::windows_sys::core::IInspectable,
    #[cfg(feature = "deprecated")]
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Create: usize,
}
#[repr(C)]
pub struct ILoggingChannelFactory2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateWithOptions: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::HSTRING, options: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateWithOptionsAndId: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::HSTRING, options: *mut ::core::ffi::c_void, id: ::windows_sys::core::GUID, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ILoggingChannelOptions {
    pub base__: ::windows_sys::core::IInspectable,
    pub Group: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub SetGroup: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct ILoggingChannelOptionsFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, group: ::windows_sys::core::GUID, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
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
#[repr(C)]
pub struct ILoggingOptionsFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateWithKeywords: unsafe extern "system" fn(this: *mut *mut Self, keywords: i64, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
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
#[repr(C)]
pub struct ILoggingSessionFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
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
#[repr(C)]
pub struct ITracingStatusChangedEventArgs {
    pub base__: ::windows_sys::core::IInspectable,
    pub Enabled: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub TraceLevel: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut CausalityTraceLevel) -> ::windows_sys::core::HRESULT,
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
