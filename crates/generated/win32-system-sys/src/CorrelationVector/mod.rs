#[link(name = "windows")]
extern "system" {
    pub fn RtlExtendCorrelationVector(correlationvector: *mut CORRELATION_VECTOR) -> u32;
    pub fn RtlIncrementCorrelationVector(correlationvector: *mut CORRELATION_VECTOR) -> u32;
    pub fn RtlInitializeCorrelationVector(correlationvector: *mut CORRELATION_VECTOR, version: i32, guid: *const ::windows_core_sys::GUID) -> u32;
    pub fn RtlValidateCorrelationVector(vector: *const CORRELATION_VECTOR) -> u32;
}
#[repr(C)]
pub struct CORRELATION_VECTOR {
    pub Version: ::win32_foundation_sys::CHAR,
    pub Vector: [::win32_foundation_sys::CHAR; 129],
}
impl ::core::marker::Copy for CORRELATION_VECTOR {}
impl ::core::clone::Clone for CORRELATION_VECTOR {
    fn clone(&self) -> Self {
        *self
    }
}
pub const RTL_CORRELATION_VECTOR_STRING_LENGTH: u32 = 129u32;
pub const RTL_CORRELATION_VECTOR_V1_LENGTH: u32 = 64u32;
pub const RTL_CORRELATION_VECTOR_V1_PREFIX_LENGTH: u32 = 16u32;
pub const RTL_CORRELATION_VECTOR_V2_LENGTH: u32 = 128u32;
pub const RTL_CORRELATION_VECTOR_V2_PREFIX_LENGTH: u32 = 22u32;
