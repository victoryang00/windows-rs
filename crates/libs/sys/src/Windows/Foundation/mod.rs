#[cfg(feature = "Foundation_Collections")]
pub mod Collections;
#[cfg(feature = "Foundation_Diagnostics")]
pub mod Diagnostics;
#[cfg(feature = "Foundation_Metadata")]
pub mod Metadata;
#[cfg(feature = "Foundation_Numerics")]
pub mod Numerics;
pub type AsyncActionCompletedHandler = *mut ::core::ffi::c_void;
pub type AsyncActionProgressHandler = *mut ::core::ffi::c_void;
pub type AsyncActionWithProgressCompletedHandler = *mut ::core::ffi::c_void;
pub type AsyncOperationCompletedHandler = *mut ::core::ffi::c_void;
pub type AsyncOperationProgressHandler = *mut ::core::ffi::c_void;
pub type AsyncOperationWithProgressCompletedHandler = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Foundation\"`*"]
#[repr(transparent)]
pub struct AsyncStatus(pub i32);
impl AsyncStatus {
    pub const Canceled: Self = Self(2i32);
    pub const Completed: Self = Self(1i32);
    pub const Error: Self = Self(3i32);
    pub const Started: Self = Self(0i32);
}
impl ::core::marker::Copy for AsyncStatus {}
impl ::core::clone::Clone for AsyncStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Foundation\"`*"]
pub struct DateTime {
    pub UniversalTime: i64,
}
impl ::core::marker::Copy for DateTime {}
impl ::core::clone::Clone for DateTime {
    fn clone(&self) -> Self {
        *self
    }
}
pub type Deferral = *mut ::core::ffi::c_void;
pub type DeferralCompletedHandler = *mut ::core::ffi::c_void;
pub type EventHandler = *mut ::core::ffi::c_void;
#[repr(C)]
#[doc = "*Required features: `\"Foundation\"`*"]
pub struct EventRegistrationToken {
    pub Value: i64,
}
impl ::core::marker::Copy for EventRegistrationToken {}
impl ::core::clone::Clone for EventRegistrationToken {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct IAsyncAction {
    pub base__: ::windows_sys::core::IInspectable,
    pub SetCompleted: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Completed: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetResults: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IAsyncAction {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1516535814, data2: 33850, data3: 19881, data4: [134, 91, 157, 38, 229, 223, 173, 123] };
}
#[repr(C)]
pub struct IAsyncActionWithProgress<TProgress> {
    pub base__: ::windows_sys::core::IInspectable,
    pub SetProgress: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Progress: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetCompleted: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Completed: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetResults: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub TProgress: ::core::marker::PhantomData<TProgress>,
}
impl ::windows_sys::core::Interface for IAsyncActionWithProgress<TProgress> {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 527282776, data2: 59395, data3: 18593, data4: [149, 70, 235, 115, 83, 57, 136, 132] };
}
#[repr(C)]
pub struct IAsyncInfo {
    pub base__: ::windows_sys::core::IInspectable,
    pub Id: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub Status: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut AsyncStatus) -> ::windows_sys::core::HRESULT,
    pub ErrorCode: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HRESULT) -> ::windows_sys::core::HRESULT,
    pub Cancel: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
    pub Close: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IAsyncInfo {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 54, data2: 0, data3: 0, data4: [192, 0, 0, 0, 0, 0, 0, 70] };
}
#[repr(C)]
pub struct IAsyncOperation<TResult> {
    pub base__: ::windows_sys::core::IInspectable,
    pub SetCompleted: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Completed: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetResults: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut TResult) -> ::windows_sys::core::HRESULT,
    pub TResult: ::core::marker::PhantomData<TResult>,
}
impl ::windows_sys::core::Interface for IAsyncOperation<TResult> {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2680336571, data2: 58438, data3: 17634, data4: [170, 97, 156, 171, 143, 99, 106, 242] };
}
#[repr(C)]
pub struct IAsyncOperationWithProgress<TResult, TProgress> {
    pub base__: ::windows_sys::core::IInspectable,
    pub SetProgress: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Progress: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetCompleted: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub Completed: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetResults: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut TResult) -> ::windows_sys::core::HRESULT,
    pub TResult: ::core::marker::PhantomData<TResult>,
    pub TProgress: ::core::marker::PhantomData<TProgress>,
}
impl ::windows_sys::core::Interface for IAsyncOperationWithProgress<TResult, TProgress> {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3050321623, data2: 58007, data3: 18831, data4: [186, 96, 2, 137, 231, 110, 35, 221] };
}
#[repr(C)]
pub struct IClosable {
    pub base__: ::windows_sys::core::IInspectable,
    pub Close: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IClosable {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 819308585, data2: 32676, data3: 16422, data4: [131, 187, 215, 91, 174, 78, 169, 158] };
}
#[repr(C)]
pub struct IDeferral {
    pub base__: ::windows_sys::core::IInspectable,
    pub Complete: unsafe extern "system" fn(this: *mut *mut Self) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDeferral {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3592853298, data2: 15231, data3: 18087, data4: [180, 11, 79, 220, 162, 162, 198, 147] };
}
#[repr(C)]
pub struct IDeferralFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IDeferralFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1705110725, data2: 16309, data3: 18482, data4: [140, 169, 240, 97, 178, 129, 209, 58] };
}
#[repr(C)]
pub struct IGetActivationFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetActivationFactory: unsafe extern "system" fn(this: *mut *mut Self, activatableclassid: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IGetActivationFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1323011810, data2: 38621, data3: 18855, data4: [148, 247, 70, 7, 221, 171, 142, 60] };
}
#[repr(C)]
pub struct IGuidHelperStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateNewGuid: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub Empty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub Equals: unsafe extern "system" fn(this: *mut *mut Self, target: &::windows_sys::core::GUID, value: &::windows_sys::core::GUID, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IGuidHelperStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1506252395, data2: 44626, data3: 21123, data4: [173, 127, 161, 185, 233, 103, 138, 221] };
}
#[repr(C)]
pub struct IMemoryBuffer {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateReference: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IMemoryBuffer {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4223982890, data2: 9307, data3: 4580, data4: [175, 152, 104, 148, 35, 38, 12, 248] };
}
#[repr(C)]
pub struct IMemoryBufferFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub Create: unsafe extern "system" fn(this: *mut *mut Self, capacity: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IMemoryBufferFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4223982891, data2: 9307, data3: 4580, data4: [175, 152, 104, 148, 35, 38, 12, 248] };
}
#[repr(C)]
pub struct IMemoryBufferReference {
    pub base__: ::windows_sys::core::IInspectable,
    pub Capacity: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub Closed: unsafe extern "system" fn(this: *mut *mut Self, handler: *mut ::core::ffi::c_void, result__: *mut EventRegistrationToken) -> ::windows_sys::core::HRESULT,
    pub RemoveClosed: unsafe extern "system" fn(this: *mut *mut Self, cookie: EventRegistrationToken) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IMemoryBufferReference {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4223982889, data2: 9307, data3: 4580, data4: [175, 152, 104, 148, 35, 38, 12, 248] };
}
#[repr(C)]
pub struct IPropertyValue {
    pub base__: ::windows_sys::core::IInspectable,
    pub Type: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut PropertyType) -> ::windows_sys::core::HRESULT,
    pub IsNumericScalar: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub GetUInt8: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u8) -> ::windows_sys::core::HRESULT,
    pub GetInt16: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i16) -> ::windows_sys::core::HRESULT,
    pub GetUInt16: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u16) -> ::windows_sys::core::HRESULT,
    pub GetInt32: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub GetUInt32: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetInt64: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i64) -> ::windows_sys::core::HRESULT,
    pub GetUInt64: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u64) -> ::windows_sys::core::HRESULT,
    pub GetSingle: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f32) -> ::windows_sys::core::HRESULT,
    pub GetDouble: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub GetChar16: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut u16) -> ::windows_sys::core::HRESULT,
    pub GetBoolean: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub GetString: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub GetGuid: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub GetDateTime: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut DateTime) -> ::windows_sys::core::HRESULT,
    pub GetTimeSpan: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut TimeSpan) -> ::windows_sys::core::HRESULT,
    pub GetPoint: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut Point) -> ::windows_sys::core::HRESULT,
    pub GetSize: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut Size) -> ::windows_sys::core::HRESULT,
    pub GetRect: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut Rect) -> ::windows_sys::core::HRESULT,
    pub GetUInt8Array: unsafe extern "system" fn(this: *mut *mut Self, value_array_size: *mut u32, value: *mut *mut u8) -> ::windows_sys::core::HRESULT,
    pub GetInt16Array: unsafe extern "system" fn(this: *mut *mut Self, value_array_size: *mut u32, value: *mut *mut i16) -> ::windows_sys::core::HRESULT,
    pub GetUInt16Array: unsafe extern "system" fn(this: *mut *mut Self, value_array_size: *mut u32, value: *mut *mut u16) -> ::windows_sys::core::HRESULT,
    pub GetInt32Array: unsafe extern "system" fn(this: *mut *mut Self, value_array_size: *mut u32, value: *mut *mut i32) -> ::windows_sys::core::HRESULT,
    pub GetUInt32Array: unsafe extern "system" fn(this: *mut *mut Self, value_array_size: *mut u32, value: *mut *mut u32) -> ::windows_sys::core::HRESULT,
    pub GetInt64Array: unsafe extern "system" fn(this: *mut *mut Self, value_array_size: *mut u32, value: *mut *mut i64) -> ::windows_sys::core::HRESULT,
    pub GetUInt64Array: unsafe extern "system" fn(this: *mut *mut Self, value_array_size: *mut u32, value: *mut *mut u64) -> ::windows_sys::core::HRESULT,
    pub GetSingleArray: unsafe extern "system" fn(this: *mut *mut Self, value_array_size: *mut u32, value: *mut *mut f32) -> ::windows_sys::core::HRESULT,
    pub GetDoubleArray: unsafe extern "system" fn(this: *mut *mut Self, value_array_size: *mut u32, value: *mut *mut f64) -> ::windows_sys::core::HRESULT,
    pub GetChar16Array: unsafe extern "system" fn(this: *mut *mut Self, value_array_size: *mut u32, value: *mut *mut u16) -> ::windows_sys::core::HRESULT,
    pub GetBooleanArray: unsafe extern "system" fn(this: *mut *mut Self, value_array_size: *mut u32, value: *mut *mut bool) -> ::windows_sys::core::HRESULT,
    pub GetStringArray: unsafe extern "system" fn(this: *mut *mut Self, value_array_size: *mut u32, value: *mut *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub GetInspectableArray: unsafe extern "system" fn(this: *mut *mut Self, value_array_size: *mut u32, value: *mut *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetGuidArray: unsafe extern "system" fn(this: *mut *mut Self, value_array_size: *mut u32, value: *mut *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT,
    pub GetDateTimeArray: unsafe extern "system" fn(this: *mut *mut Self, value_array_size: *mut u32, value: *mut *mut DateTime) -> ::windows_sys::core::HRESULT,
    pub GetTimeSpanArray: unsafe extern "system" fn(this: *mut *mut Self, value_array_size: *mut u32, value: *mut *mut TimeSpan) -> ::windows_sys::core::HRESULT,
    pub GetPointArray: unsafe extern "system" fn(this: *mut *mut Self, value_array_size: *mut u32, value: *mut *mut Point) -> ::windows_sys::core::HRESULT,
    pub GetSizeArray: unsafe extern "system" fn(this: *mut *mut Self, value_array_size: *mut u32, value: *mut *mut Size) -> ::windows_sys::core::HRESULT,
    pub GetRectArray: unsafe extern "system" fn(this: *mut *mut Self, value_array_size: *mut u32, value: *mut *mut Rect) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IPropertyValue {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1272349405, data2: 30036, data3: 16617, data4: [154, 155, 130, 101, 78, 222, 126, 98] };
}
#[repr(C)]
pub struct IPropertyValueStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateEmpty: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateUInt8: unsafe extern "system" fn(this: *mut *mut Self, value: u8, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateInt16: unsafe extern "system" fn(this: *mut *mut Self, value: i16, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateUInt16: unsafe extern "system" fn(this: *mut *mut Self, value: u16, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateInt32: unsafe extern "system" fn(this: *mut *mut Self, value: i32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateUInt32: unsafe extern "system" fn(this: *mut *mut Self, value: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateInt64: unsafe extern "system" fn(this: *mut *mut Self, value: i64, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateUInt64: unsafe extern "system" fn(this: *mut *mut Self, value: u64, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateSingle: unsafe extern "system" fn(this: *mut *mut Self, value: f32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateDouble: unsafe extern "system" fn(this: *mut *mut Self, value: f64, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateChar16: unsafe extern "system" fn(this: *mut *mut Self, value: u16, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateBoolean: unsafe extern "system" fn(this: *mut *mut Self, value: bool, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateString: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateInspectable: unsafe extern "system" fn(this: *mut *mut Self, value: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateGuid: unsafe extern "system" fn(this: *mut *mut Self, value: ::windows_sys::core::GUID, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateDateTime: unsafe extern "system" fn(this: *mut *mut Self, value: DateTime, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateTimeSpan: unsafe extern "system" fn(this: *mut *mut Self, value: TimeSpan, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreatePoint: unsafe extern "system" fn(this: *mut *mut Self, value: Point, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateSize: unsafe extern "system" fn(this: *mut *mut Self, value: Size, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateRect: unsafe extern "system" fn(this: *mut *mut Self, value: Rect, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateUInt8Array: unsafe extern "system" fn(this: *mut *mut Self, value_array_size: u32, value: *const u8, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateInt16Array: unsafe extern "system" fn(this: *mut *mut Self, value_array_size: u32, value: *const i16, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateUInt16Array: unsafe extern "system" fn(this: *mut *mut Self, value_array_size: u32, value: *const u16, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateInt32Array: unsafe extern "system" fn(this: *mut *mut Self, value_array_size: u32, value: *const i32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateUInt32Array: unsafe extern "system" fn(this: *mut *mut Self, value_array_size: u32, value: *const u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateInt64Array: unsafe extern "system" fn(this: *mut *mut Self, value_array_size: u32, value: *const i64, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateUInt64Array: unsafe extern "system" fn(this: *mut *mut Self, value_array_size: u32, value: *const u64, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateSingleArray: unsafe extern "system" fn(this: *mut *mut Self, value_array_size: u32, value: *const f32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateDoubleArray: unsafe extern "system" fn(this: *mut *mut Self, value_array_size: u32, value: *const f64, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateChar16Array: unsafe extern "system" fn(this: *mut *mut Self, value_array_size: u32, value: *const u16, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateBooleanArray: unsafe extern "system" fn(this: *mut *mut Self, value_array_size: u32, value: *const bool, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateStringArray: unsafe extern "system" fn(this: *mut *mut Self, value_array_size: u32, value: *const ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateInspectableArray: unsafe extern "system" fn(this: *mut *mut Self, value_array_size: u32, value: *const *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateGuidArray: unsafe extern "system" fn(this: *mut *mut Self, value_array_size: u32, value: *const ::windows_sys::core::GUID, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateDateTimeArray: unsafe extern "system" fn(this: *mut *mut Self, value_array_size: u32, value: *const DateTime, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateTimeSpanArray: unsafe extern "system" fn(this: *mut *mut Self, value_array_size: u32, value: *const TimeSpan, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreatePointArray: unsafe extern "system" fn(this: *mut *mut Self, value_array_size: u32, value: *const Point, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateSizeArray: unsafe extern "system" fn(this: *mut *mut Self, value_array_size: u32, value: *const Size, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateRectArray: unsafe extern "system" fn(this: *mut *mut Self, value_array_size: u32, value: *const Rect, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IPropertyValueStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1654381512, data2: 55602, data3: 20468, data4: [150, 185, 141, 150, 197, 193, 232, 88] };
}
#[repr(C)]
pub struct IReference<T> {
    pub base__: ::windows_sys::core::IInspectable,
    pub Value: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut T) -> ::windows_sys::core::HRESULT,
    pub T: ::core::marker::PhantomData<T>,
}
impl ::windows_sys::core::Interface for IReference<T> {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1640068870, data2: 11621, data3: 4576, data4: [154, 232, 212, 133, 100, 1, 84, 114] };
}
#[repr(C)]
pub struct IReferenceArray<T> {
    pub base__: ::windows_sys::core::IInspectable,
    pub Value: unsafe extern "system" fn(this: *mut *mut Self, result_size__: *mut u32, result__: *mut *mut T) -> ::windows_sys::core::HRESULT,
    pub T: ::core::marker::PhantomData<T>,
}
impl ::windows_sys::core::Interface for IReferenceArray<T> {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1640068871, data2: 11621, data3: 4576, data4: [154, 232, 212, 133, 100, 1, 84, 114] };
}
#[repr(C)]
pub struct IStringable {
    pub base__: ::windows_sys::core::IInspectable,
    pub ToString: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IStringable {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2520162132, data2: 36534, data3: 18672, data4: [171, 206, 193, 178, 17, 230, 39, 195] };
}
#[repr(C)]
pub struct IUriEscapeStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub UnescapeComponent: unsafe extern "system" fn(this: *mut *mut Self, tounescape: ::windows_sys::core::HSTRING, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub EscapeComponent: unsafe extern "system" fn(this: *mut *mut Self, toescape: ::windows_sys::core::HSTRING, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IUriEscapeStatics {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3251909306, data2: 51236, data3: 17490, data4: [167, 253, 81, 43, 195, 187, 233, 161] };
}
#[repr(C)]
pub struct IUriRuntimeClass {
    pub base__: ::windows_sys::core::IInspectable,
    pub AbsoluteUri: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub DisplayUri: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Domain: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Extension: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Fragment: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Host: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Password: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Path: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Query: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub QueryParsed: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub RawUri: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub SchemeName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub UserName: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Port: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut i32) -> ::windows_sys::core::HRESULT,
    pub Suspicious: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub Equals: unsafe extern "system" fn(this: *mut *mut Self, puri: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub CombineUri: unsafe extern "system" fn(this: *mut *mut Self, relativeuri: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IUriRuntimeClass {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2654363223, data2: 18610, data3: 16736, data4: [149, 111, 199, 56, 81, 32, 187, 252] };
}
#[repr(C)]
pub struct IUriRuntimeClassFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateUri: unsafe extern "system" fn(this: *mut *mut Self, uri: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateWithRelativeUri: unsafe extern "system" fn(this: *mut *mut Self, baseuri: ::windows_sys::core::HSTRING, relativeuri: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IUriRuntimeClassFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1151957359, data2: 29246, data3: 20447, data4: [162, 24, 3, 62, 117, 176, 192, 132] };
}
#[repr(C)]
pub struct IUriRuntimeClassWithAbsoluteCanonicalUri {
    pub base__: ::windows_sys::core::IInspectable,
    pub AbsoluteCanonicalUri: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub DisplayIri: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IUriRuntimeClassWithAbsoluteCanonicalUri {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1972213345, data2: 8732, data3: 18447, data4: [163, 57, 80, 101, 102, 115, 244, 111] };
}
#[repr(C)]
pub struct IWwwFormUrlDecoderEntry {
    pub base__: ::windows_sys::core::IInspectable,
    pub Name: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub Value: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IWwwFormUrlDecoderEntry {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 308180017, data2: 63096, data3: 20110, data4: [182, 112, 32, 169, 176, 108, 81, 45] };
}
#[repr(C)]
pub struct IWwwFormUrlDecoderRuntimeClass {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetFirstValueByName: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::HSTRING, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IWwwFormUrlDecoderRuntimeClass {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3562669137, data2: 61989, data3: 17730, data4: [146, 150, 14, 29, 245, 210, 84, 223] };
}
#[repr(C)]
pub struct IWwwFormUrlDecoderRuntimeClassFactory {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateWwwFormUrlDecoder: unsafe extern "system" fn(this: *mut *mut Self, query: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
impl ::windows_sys::core::Interface for IWwwFormUrlDecoderRuntimeClassFactory {
    const IID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1535929149, data2: 9390, data3: 16821, data4: [161, 191, 240, 195, 213, 68, 132, 91] };
}
pub type MemoryBuffer = *mut ::core::ffi::c_void;
#[repr(C)]
#[doc = "*Required features: `\"Foundation\"`*"]
pub struct Point {
    pub X: f32,
    pub Y: f32,
}
impl ::core::marker::Copy for Point {}
impl ::core::clone::Clone for Point {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Foundation\"`*"]
#[repr(transparent)]
pub struct PropertyType(pub i32);
impl PropertyType {
    pub const Empty: Self = Self(0i32);
    pub const UInt8: Self = Self(1i32);
    pub const Int16: Self = Self(2i32);
    pub const UInt16: Self = Self(3i32);
    pub const Int32: Self = Self(4i32);
    pub const UInt32: Self = Self(5i32);
    pub const Int64: Self = Self(6i32);
    pub const UInt64: Self = Self(7i32);
    pub const Single: Self = Self(8i32);
    pub const Double: Self = Self(9i32);
    pub const Char16: Self = Self(10i32);
    pub const Boolean: Self = Self(11i32);
    pub const String: Self = Self(12i32);
    pub const Inspectable: Self = Self(13i32);
    pub const DateTime: Self = Self(14i32);
    pub const TimeSpan: Self = Self(15i32);
    pub const Guid: Self = Self(16i32);
    pub const Point: Self = Self(17i32);
    pub const Size: Self = Self(18i32);
    pub const Rect: Self = Self(19i32);
    pub const OtherType: Self = Self(20i32);
    pub const UInt8Array: Self = Self(1025i32);
    pub const Int16Array: Self = Self(1026i32);
    pub const UInt16Array: Self = Self(1027i32);
    pub const Int32Array: Self = Self(1028i32);
    pub const UInt32Array: Self = Self(1029i32);
    pub const Int64Array: Self = Self(1030i32);
    pub const UInt64Array: Self = Self(1031i32);
    pub const SingleArray: Self = Self(1032i32);
    pub const DoubleArray: Self = Self(1033i32);
    pub const Char16Array: Self = Self(1034i32);
    pub const BooleanArray: Self = Self(1035i32);
    pub const StringArray: Self = Self(1036i32);
    pub const InspectableArray: Self = Self(1037i32);
    pub const DateTimeArray: Self = Self(1038i32);
    pub const TimeSpanArray: Self = Self(1039i32);
    pub const GuidArray: Self = Self(1040i32);
    pub const PointArray: Self = Self(1041i32);
    pub const SizeArray: Self = Self(1042i32);
    pub const RectArray: Self = Self(1043i32);
    pub const OtherTypeArray: Self = Self(1044i32);
}
impl ::core::marker::Copy for PropertyType {}
impl ::core::clone::Clone for PropertyType {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Foundation\"`*"]
pub struct Rect {
    pub X: f32,
    pub Y: f32,
    pub Width: f32,
    pub Height: f32,
}
impl ::core::marker::Copy for Rect {}
impl ::core::clone::Clone for Rect {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Foundation\"`*"]
pub struct Size {
    pub Width: f32,
    pub Height: f32,
}
impl ::core::marker::Copy for Size {}
impl ::core::clone::Clone for Size {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Foundation\"`*"]
pub struct TimeSpan {
    pub Duration: i64,
}
impl ::core::marker::Copy for TimeSpan {}
impl ::core::clone::Clone for TimeSpan {
    fn clone(&self) -> Self {
        *self
    }
}
pub type TypedEventHandler = *mut ::core::ffi::c_void;
pub type Uri = *mut ::core::ffi::c_void;
pub type WwwFormUrlDecoder = *mut ::core::ffi::c_void;
pub type WwwFormUrlDecoderEntry = *mut ::core::ffi::c_void;
