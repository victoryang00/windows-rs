#[repr(C)]
pub struct CORRELATION_VECTOR {
    pub Version: ::win32_foundation::CHAR,
    pub Vector: [::win32_foundation::CHAR; 129],
}
impl ::core::marker::Copy for CORRELATION_VECTOR {}
impl ::core::clone::Clone for CORRELATION_VECTOR {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CORRELATION_VECTOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CORRELATION_VECTOR").field("Version", &self.Version).field("Vector", &self.Vector).finish()
    }
}
unsafe impl ::windows_core::Abi for CORRELATION_VECTOR {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for CORRELATION_VECTOR {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows_core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CORRELATION_VECTOR>()) == 0 }
    }
}
impl ::core::cmp::Eq for CORRELATION_VECTOR {}
impl ::core::default::Default for CORRELATION_VECTOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const RTL_CORRELATION_VECTOR_STRING_LENGTH: u32 = 129u32;
pub const RTL_CORRELATION_VECTOR_V1_LENGTH: u32 = 64u32;
pub const RTL_CORRELATION_VECTOR_V1_PREFIX_LENGTH: u32 = 16u32;
pub const RTL_CORRELATION_VECTOR_V2_LENGTH: u32 = 128u32;
pub const RTL_CORRELATION_VECTOR_V2_PREFIX_LENGTH: u32 = 22u32;
#[inline]
pub unsafe fn RtlExtendCorrelationVector(correlationvector: *mut CORRELATION_VECTOR) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RtlExtendCorrelationVector(correlationvector: *mut CORRELATION_VECTOR) -> u32;
        }
        ::core::mem::transmute(RtlExtendCorrelationVector(::core::mem::transmute(correlationvector)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn RtlIncrementCorrelationVector(correlationvector: *mut CORRELATION_VECTOR) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RtlIncrementCorrelationVector(correlationvector: *mut CORRELATION_VECTOR) -> u32;
        }
        ::core::mem::transmute(RtlIncrementCorrelationVector(::core::mem::transmute(correlationvector)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn RtlInitializeCorrelationVector(correlationvector: *mut CORRELATION_VECTOR, version: i32, guid: *const ::windows_core::GUID) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RtlInitializeCorrelationVector(correlationvector: *mut CORRELATION_VECTOR, version: i32, guid: *const ::windows_core::GUID) -> u32;
        }
        ::core::mem::transmute(RtlInitializeCorrelationVector(::core::mem::transmute(correlationvector), ::core::mem::transmute(version), ::core::mem::transmute(guid)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn RtlValidateCorrelationVector(vector: *const CORRELATION_VECTOR) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RtlValidateCorrelationVector(vector: *const CORRELATION_VECTOR) -> u32;
        }
        ::core::mem::transmute(RtlValidateCorrelationVector(::core::mem::transmute(vector)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
