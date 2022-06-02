#[repr(C)]
pub struct IJsonArray {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetObjectAt: unsafe extern "system" fn(this: *mut *mut Self, index: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetArrayAt: unsafe extern "system" fn(this: *mut *mut Self, index: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetStringAt: unsafe extern "system" fn(this: *mut *mut Self, index: u32, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub GetNumberAt: unsafe extern "system" fn(this: *mut *mut Self, index: u32, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub GetBooleanAt: unsafe extern "system" fn(this: *mut *mut Self, index: u32, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IJsonArrayStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub Parse: unsafe extern "system" fn(this: *mut *mut Self, input: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub TryParse: unsafe extern "system" fn(this: *mut *mut Self, input: ::windows_sys::core::HSTRING, result: *mut *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IJsonErrorStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetJsonStatus: unsafe extern "system" fn(this: *mut *mut Self, hresult: i32, result__: *mut JsonErrorStatus) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IJsonObject {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetNamedValue: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub SetNamedValue: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::HSTRING, value: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetNamedObject: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetNamedArray: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetNamedString: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::HSTRING, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub GetNamedNumber: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::HSTRING, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub GetNamedBoolean: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::HSTRING, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IJsonObjectStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub Parse: unsafe extern "system" fn(this: *mut *mut Self, input: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub TryParse: unsafe extern "system" fn(this: *mut *mut Self, input: ::windows_sys::core::HSTRING, result: *mut *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IJsonObjectWithDefaultValues {
    pub base__: ::windows_sys::core::IInspectable,
    pub GetNamedValueOrDefault: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::HSTRING, defaultvalue: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetNamedObjectOrDefault: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::HSTRING, defaultvalue: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetNamedStringOrDefault: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::HSTRING, defaultvalue: ::windows_sys::core::HSTRING, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub GetNamedArrayOrDefault: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::HSTRING, defaultvalue: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetNamedNumberOrDefault: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::HSTRING, defaultvalue: f64, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub GetNamedBooleanOrDefault: unsafe extern "system" fn(this: *mut *mut Self, name: ::windows_sys::core::HSTRING, defaultvalue: bool, result__: *mut bool) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IJsonValue {
    pub base__: ::windows_sys::core::IInspectable,
    pub ValueType: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut JsonValueType) -> ::windows_sys::core::HRESULT,
    pub Stringify: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub GetString: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut ::windows_sys::core::HSTRING) -> ::windows_sys::core::HRESULT,
    pub GetNumber: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut f64) -> ::windows_sys::core::HRESULT,
    pub GetBoolean: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub GetArray: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub GetObject: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IJsonValueStatics {
    pub base__: ::windows_sys::core::IInspectable,
    pub Parse: unsafe extern "system" fn(this: *mut *mut Self, input: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub TryParse: unsafe extern "system" fn(this: *mut *mut Self, input: ::windows_sys::core::HSTRING, result: *mut *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_sys::core::HRESULT,
    pub CreateBooleanValue: unsafe extern "system" fn(this: *mut *mut Self, input: bool, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateNumberValue: unsafe extern "system" fn(this: *mut *mut Self, input: f64, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
    pub CreateStringValue: unsafe extern "system" fn(this: *mut *mut Self, input: ::windows_sys::core::HSTRING, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
#[repr(C)]
pub struct IJsonValueStatics2 {
    pub base__: ::windows_sys::core::IInspectable,
    pub CreateNullValue: unsafe extern "system" fn(this: *mut *mut Self, result__: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT,
}
pub type JsonArray = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Data_Json\"`*"]
#[repr(transparent)]
pub struct JsonErrorStatus(pub i32);
impl JsonErrorStatus {
    pub const Unknown: Self = Self(0i32);
    pub const InvalidJsonString: Self = Self(1i32);
    pub const InvalidJsonNumber: Self = Self(2i32);
    pub const JsonValueNotFound: Self = Self(3i32);
    pub const ImplementationLimit: Self = Self(4i32);
}
impl ::core::marker::Copy for JsonErrorStatus {}
impl ::core::clone::Clone for JsonErrorStatus {
    fn clone(&self) -> Self {
        *self
    }
}
pub type JsonObject = *mut ::core::ffi::c_void;
pub type JsonValue = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Data_Json\"`*"]
#[repr(transparent)]
pub struct JsonValueType(pub i32);
impl JsonValueType {
    pub const Null: Self = Self(0i32);
    pub const Boolean: Self = Self(1i32);
    pub const Number: Self = Self(2i32);
    pub const String: Self = Self(3i32);
    pub const Array: Self = Self(4i32);
    pub const Object: Self = Self(5i32);
}
impl ::core::marker::Copy for JsonValueType {}
impl ::core::clone::Clone for JsonValueType {
    fn clone(&self) -> Self {
        *self
    }
}
