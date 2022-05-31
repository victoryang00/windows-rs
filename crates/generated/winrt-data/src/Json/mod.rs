#[doc(hidden)]
#[repr(transparent)]
pub struct IJsonArray(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IJsonArray {
    type Vtable = IJsonArray_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x08c1ddb6_0cbd_4a9a_b5d3_2f852dc37e81);
}
#[repr(C)]
#[doc(hidden)]
pub struct IJsonArray_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub GetObjectAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetArrayAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetStringAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub GetNumberAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, result__: *mut f64) -> ::windows_core::HRESULT,
    pub GetBooleanAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IJsonArrayStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IJsonArrayStatics {
    type Vtable = IJsonArrayStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xdb1434a9_e164_499f_93e2_8a8f49bb90ba);
}
#[repr(C)]
#[doc(hidden)]
pub struct IJsonArrayStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Parse: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub TryParse: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result: *mut ::windows_core::RawPtr, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IJsonErrorStatics2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IJsonErrorStatics2 {
    type Vtable = IJsonErrorStatics2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x404030da_87d0_436c_83ab_fc7b12c0cc26);
}
#[repr(C)]
#[doc(hidden)]
pub struct IJsonErrorStatics2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub GetJsonStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hresult: i32, result__: *mut JsonErrorStatus) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IJsonObject(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IJsonObject {
    type Vtable = IJsonObject_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x064e24dd_29c2_4f83_9ac1_9ee11578beb3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IJsonObject_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub GetNamedValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub SetNamedValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, value: ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetNamedObject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetNamedArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetNamedString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub GetNamedNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut f64) -> ::windows_core::HRESULT,
    pub GetNamedBoolean: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IJsonObjectStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IJsonObjectStatics {
    type Vtable = IJsonObjectStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2289f159_54de_45d8_abcc_22603fa066a0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IJsonObjectStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Parse: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub TryParse: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result: *mut ::windows_core::RawPtr, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IJsonObjectWithDefaultValues(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IJsonObjectWithDefaultValues {
    type Vtable = IJsonObjectWithDefaultValues_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd960d2a2_b7f0_4f00_8e44_d82cf415ea13);
}
#[repr(C)]
#[doc(hidden)]
pub struct IJsonObjectWithDefaultValues_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub GetNamedValueOrDefault: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, defaultvalue: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetNamedObjectOrDefault: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, defaultvalue: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetNamedStringOrDefault: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, defaultvalue: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub GetNamedArrayOrDefault: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, defaultvalue: ::windows_core::RawPtr, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetNamedNumberOrDefault: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, defaultvalue: f64, result__: *mut f64) -> ::windows_core::HRESULT,
    pub GetNamedBooleanOrDefault: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, defaultvalue: bool, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct IJsonValue(::windows_core::IUnknown);
impl IJsonValue {
    pub fn ValueType(&self) -> ::windows_core::Result<JsonValueType> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<JsonValueType>::zeroed();
            (::windows_core::Interface::vtable(this).ValueType)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<JsonValueType>(result__)
        }
    }
    pub fn Stringify(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Stringify)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn GetString(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).GetString)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn GetNumber(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).GetNumber)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn GetBoolean(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).GetBoolean)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn GetArray(&self) -> ::windows_core::Result<JsonArray> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetArray)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<JsonArray>(result__)
        }
    }
    pub fn GetObject(&self) -> ::windows_core::Result<JsonObject> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetObject)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<JsonObject>(result__)
        }
    }
}
impl ::core::convert::From<IJsonValue> for ::windows_core::IUnknown {
    fn from(value: IJsonValue) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IJsonValue> for ::windows_core::IUnknown {
    fn from(value: &IJsonValue) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for IJsonValue {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a IJsonValue {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IJsonValue> for ::windows_core::IInspectable {
    fn from(value: IJsonValue) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IJsonValue> for ::windows_core::IInspectable {
    fn from(value: &IJsonValue) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for IJsonValue {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a IJsonValue {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IJsonValue {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IJsonValue {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IJsonValue {}
impl ::core::fmt::Debug for IJsonValue {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IJsonValue").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for IJsonValue {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"{a3219ecb-f0b3-4dcd-beee-19d48cd3ed1e}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for IJsonValue {
    type Vtable = IJsonValue_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa3219ecb_f0b3_4dcd_beee_19d48cd3ed1e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IJsonValue_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub ValueType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut JsonValueType) -> ::windows_core::HRESULT,
    pub Stringify: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub GetString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub GetNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub GetBoolean: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub GetArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub GetObject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IJsonValueStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IJsonValueStatics {
    type Vtable = IJsonValueStatics_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5f6b544a_2f53_48e1_91a3_f78b50a6345c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IJsonValueStatics_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub Parse: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub TryParse: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result: *mut ::windows_core::RawPtr, result__: *mut bool) -> ::windows_core::HRESULT,
    pub CreateBooleanValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: bool, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CreateNumberValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: f64, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
    pub CreateStringValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows_core::HSTRING>, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IJsonValueStatics2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IJsonValueStatics2 {
    type Vtable = IJsonValueStatics2_Vtbl;
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1d9ecbe4_3fe8_4335_8392_93d8e36865f0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IJsonValueStatics2_Vtbl {
    pub base__: ::windows_core::IInspectableVtbl,
    pub CreateNullValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::RawPtr) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
pub struct JsonArray(::windows_core::IUnknown);
impl JsonArray {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<JsonArray, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn First(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IIterator<IJsonValue>> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IIterable<IJsonValue>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).First)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IIterator<IJsonValue>>(result__)
        }
    }
    pub fn GetObjectAt(&self, index: u32) -> ::windows_core::Result<JsonObject> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetObjectAt)(::windows_core::Interface::as_raw(this), index, result__.as_mut_ptr()).from_abi::<JsonObject>(result__)
        }
    }
    pub fn GetArrayAt(&self, index: u32) -> ::windows_core::Result<JsonArray> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetArrayAt)(::windows_core::Interface::as_raw(this), index, result__.as_mut_ptr()).from_abi::<JsonArray>(result__)
        }
    }
    pub fn GetStringAt(&self, index: u32) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).GetStringAt)(::windows_core::Interface::as_raw(this), index, result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn GetNumberAt(&self, index: u32) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).GetNumberAt)(::windows_core::Interface::as_raw(this), index, result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn GetBooleanAt(&self, index: u32) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).GetBooleanAt)(::windows_core::Interface::as_raw(this), index, result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn Parse<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(input: Param0) -> ::windows_core::Result<JsonArray> {
        Self::IJsonArrayStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Parse)(::windows_core::Interface::as_raw(this), input.into_param().abi(), result__.as_mut_ptr()).from_abi::<JsonArray>(result__)
        })
    }
    pub fn TryParse<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(input: Param0, result: &mut ::core::option::Option<JsonArray>) -> ::windows_core::Result<bool> {
        Self::IJsonArrayStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).TryParse)(::windows_core::Interface::as_raw(this), input.into_param().abi(), result as *mut _ as _, result__.as_mut_ptr()).from_abi::<bool>(result__)
        })
    }
    pub fn ValueType(&self) -> ::windows_core::Result<JsonValueType> {
        let this = &::windows_core::Interface::cast::<IJsonValue>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<JsonValueType>::zeroed();
            (::windows_core::Interface::vtable(this).ValueType)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<JsonValueType>(result__)
        }
    }
    pub fn Stringify(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IJsonValue>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Stringify)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn GetString(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IJsonValue>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).GetString)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn GetNumber(&self) -> ::windows_core::Result<f64> {
        let this = &::windows_core::Interface::cast::<IJsonValue>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).GetNumber)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn GetBoolean(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IJsonValue>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).GetBoolean)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn GetArray(&self) -> ::windows_core::Result<JsonArray> {
        let this = &::windows_core::Interface::cast::<IJsonValue>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetArray)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<JsonArray>(result__)
        }
    }
    pub fn GetObject(&self) -> ::windows_core::Result<JsonObject> {
        let this = &::windows_core::Interface::cast::<IJsonValue>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetObject)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<JsonObject>(result__)
        }
    }
    pub fn ToString(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::IStringable>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).ToString)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetAt(&self, index: u32) -> ::windows_core::Result<IJsonValue> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IVector<IJsonValue>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetAt)(::windows_core::Interface::as_raw(this), index, result__.as_mut_ptr()).from_abi::<IJsonValue>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Size(&self) -> ::windows_core::Result<u32> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IVector<IJsonValue>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).Size)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetView(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IVectorView<IJsonValue>> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IVector<IJsonValue>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetView)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IVectorView<IJsonValue>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn IndexOf<'a, Param0: ::windows_core::IntoParam<'a, IJsonValue>>(&self, value: Param0, index: &mut u32) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IVector<IJsonValue>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).IndexOf)(::windows_core::Interface::as_raw(this), value.into_param().abi(), index, result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn SetAt<'a, Param1: ::windows_core::IntoParam<'a, IJsonValue>>(&self, index: u32, value: Param1) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IVector<IJsonValue>>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetAt)(::windows_core::Interface::as_raw(this), index, value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn InsertAt<'a, Param1: ::windows_core::IntoParam<'a, IJsonValue>>(&self, index: u32, value: Param1) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IVector<IJsonValue>>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).InsertAt)(::windows_core::Interface::as_raw(this), index, value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn RemoveAt(&self, index: u32) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IVector<IJsonValue>>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).RemoveAt)(::windows_core::Interface::as_raw(this), index).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Append<'a, Param0: ::windows_core::IntoParam<'a, IJsonValue>>(&self, value: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IVector<IJsonValue>>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Append)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn RemoveAtEnd(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IVector<IJsonValue>>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).RemoveAtEnd)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Clear(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IVector<IJsonValue>>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Clear)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetMany(&self, startindex: u32, items: &mut [::core::option::Option<IJsonValue>]) -> ::windows_core::Result<u32> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IVector<IJsonValue>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).GetMany)(::windows_core::Interface::as_raw(this), startindex, items.len() as u32, ::core::mem::transmute_copy(&items), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn ReplaceAll(&self, items: &[::core::option::Option<IJsonValue>]) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IVector<IJsonValue>>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).ReplaceAll)(::windows_core::Interface::as_raw(this), items.len() as u32, ::core::mem::transmute(items.as_ptr())).ok() }
    }
    pub fn IJsonArrayStatics<R, F: FnOnce(&IJsonArrayStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<JsonArray, IJsonArrayStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for JsonArray {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for JsonArray {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for JsonArray {}
impl ::core::fmt::Debug for JsonArray {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("JsonArray").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for JsonArray {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Data.Json.JsonArray;{08c1ddb6-0cbd-4a9a-b5d3-2f852dc37e81})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for JsonArray {
    type Vtable = IJsonArray_Vtbl;
    const IID: ::windows_core::GUID = <IJsonArray as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for JsonArray {
    const NAME: &'static str = "Windows.Data.Json.JsonArray";
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for JsonArray {
    type Item = IJsonValue;
    type IntoIter = ::winrt_foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for &JsonArray {
    type Item = IJsonValue;
    type IntoIter = ::winrt_foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::winrt_foundation::Collections::VectorIterator::new(::core::convert::TryInto::try_into(self).ok())
    }
}
impl ::core::convert::From<JsonArray> for ::windows_core::IUnknown {
    fn from(value: JsonArray) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&JsonArray> for ::windows_core::IUnknown {
    fn from(value: &JsonArray) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for JsonArray {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a JsonArray {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<JsonArray> for ::windows_core::IInspectable {
    fn from(value: JsonArray) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&JsonArray> for ::windows_core::IInspectable {
    fn from(value: &JsonArray) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for JsonArray {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a JsonArray {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<JsonArray> for ::winrt_foundation::Collections::IIterable<IJsonValue> {
    type Error = ::windows_core::Error;
    fn try_from(value: JsonArray) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&JsonArray> for ::winrt_foundation::Collections::IIterable<IJsonValue> {
    type Error = ::windows_core::Error;
    fn try_from(value: &JsonArray) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<IJsonValue>> for JsonArray {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::Collections::IIterable<IJsonValue>> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<IJsonValue>> for &JsonArray {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::Collections::IIterable<IJsonValue>> {
        ::core::convert::TryInto::<::winrt_foundation::Collections::IIterable<IJsonValue>>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<JsonArray> for IJsonValue {
    type Error = ::windows_core::Error;
    fn try_from(value: JsonArray) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&JsonArray> for IJsonValue {
    type Error = ::windows_core::Error;
    fn try_from(value: &JsonArray) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IJsonValue> for JsonArray {
    fn into_param(self) -> ::windows_core::Param<'a, IJsonValue> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IJsonValue> for &JsonArray {
    fn into_param(self) -> ::windows_core::Param<'a, IJsonValue> {
        ::core::convert::TryInto::<IJsonValue>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<JsonArray> for ::winrt_foundation::IStringable {
    type Error = ::windows_core::Error;
    fn try_from(value: JsonArray) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&JsonArray> for ::winrt_foundation::IStringable {
    type Error = ::windows_core::Error;
    fn try_from(value: &JsonArray) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IStringable> for JsonArray {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IStringable> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IStringable> for &JsonArray {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IStringable> {
        ::core::convert::TryInto::<::winrt_foundation::IStringable>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<JsonArray> for ::winrt_foundation::Collections::IVector<IJsonValue> {
    type Error = ::windows_core::Error;
    fn try_from(value: JsonArray) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&JsonArray> for ::winrt_foundation::Collections::IVector<IJsonValue> {
    type Error = ::windows_core::Error;
    fn try_from(value: &JsonArray) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IVector<IJsonValue>> for JsonArray {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::Collections::IVector<IJsonValue>> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IVector<IJsonValue>> for &JsonArray {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::Collections::IVector<IJsonValue>> {
        ::core::convert::TryInto::<::winrt_foundation::Collections::IVector<IJsonValue>>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for JsonArray {}
unsafe impl ::core::marker::Sync for JsonArray {}
pub struct JsonError;
impl JsonError {
    pub fn GetJsonStatus(hresult: i32) -> ::windows_core::Result<JsonErrorStatus> {
        Self::IJsonErrorStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<JsonErrorStatus>::zeroed();
            (::windows_core::Interface::vtable(this).GetJsonStatus)(::windows_core::Interface::as_raw(this), hresult, result__.as_mut_ptr()).from_abi::<JsonErrorStatus>(result__)
        })
    }
    pub fn IJsonErrorStatics2<R, F: FnOnce(&IJsonErrorStatics2) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<JsonError, IJsonErrorStatics2> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows_core::RuntimeName for JsonError {
    const NAME: &'static str = "Windows.Data.Json.JsonError";
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
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
impl ::core::default::Default for JsonErrorStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for JsonErrorStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for JsonErrorStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("JsonErrorStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for JsonErrorStatus {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Data.Json.JsonErrorStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
#[repr(transparent)]
pub struct JsonObject(::windows_core::IUnknown);
impl JsonObject {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<JsonObject, ::windows_core::IGenericFactory> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn First(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IIterator<::winrt_foundation::Collections::IKeyValuePair<::windows_core::HSTRING, IJsonValue>>> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IIterable<::winrt_foundation::Collections::IKeyValuePair<::windows_core::HSTRING, IJsonValue>>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).First)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IIterator<::winrt_foundation::Collections::IKeyValuePair<::windows_core::HSTRING, IJsonValue>>>(result__)
        }
    }
    pub fn GetNamedValue<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, name: Param0) -> ::windows_core::Result<JsonValue> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetNamedValue)(::windows_core::Interface::as_raw(this), name.into_param().abi(), result__.as_mut_ptr()).from_abi::<JsonValue>(result__)
        }
    }
    pub fn SetNamedValue<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, IJsonValue>>(&self, name: Param0, value: Param1) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetNamedValue)(::windows_core::Interface::as_raw(this), name.into_param().abi(), value.into_param().abi()).ok() }
    }
    pub fn GetNamedObject<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, name: Param0) -> ::windows_core::Result<JsonObject> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetNamedObject)(::windows_core::Interface::as_raw(this), name.into_param().abi(), result__.as_mut_ptr()).from_abi::<JsonObject>(result__)
        }
    }
    pub fn GetNamedArray<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, name: Param0) -> ::windows_core::Result<JsonArray> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetNamedArray)(::windows_core::Interface::as_raw(this), name.into_param().abi(), result__.as_mut_ptr()).from_abi::<JsonArray>(result__)
        }
    }
    pub fn GetNamedString<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, name: Param0) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).GetNamedString)(::windows_core::Interface::as_raw(this), name.into_param().abi(), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn GetNamedNumber<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, name: Param0) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).GetNamedNumber)(::windows_core::Interface::as_raw(this), name.into_param().abi(), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn GetNamedBoolean<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, name: Param0) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).GetNamedBoolean)(::windows_core::Interface::as_raw(this), name.into_param().abi(), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn Parse<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(input: Param0) -> ::windows_core::Result<JsonObject> {
        Self::IJsonObjectStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Parse)(::windows_core::Interface::as_raw(this), input.into_param().abi(), result__.as_mut_ptr()).from_abi::<JsonObject>(result__)
        })
    }
    pub fn TryParse<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(input: Param0, result: &mut ::core::option::Option<JsonObject>) -> ::windows_core::Result<bool> {
        Self::IJsonObjectStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).TryParse)(::windows_core::Interface::as_raw(this), input.into_param().abi(), result as *mut _ as _, result__.as_mut_ptr()).from_abi::<bool>(result__)
        })
    }
    pub fn GetNamedValueOrDefault<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, JsonValue>>(&self, name: Param0, defaultvalue: Param1) -> ::windows_core::Result<JsonValue> {
        let this = &::windows_core::Interface::cast::<IJsonObjectWithDefaultValues>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetNamedValueOrDefault)(::windows_core::Interface::as_raw(this), name.into_param().abi(), defaultvalue.into_param().abi(), result__.as_mut_ptr()).from_abi::<JsonValue>(result__)
        }
    }
    pub fn GetNamedObjectOrDefault<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, JsonObject>>(&self, name: Param0, defaultvalue: Param1) -> ::windows_core::Result<JsonObject> {
        let this = &::windows_core::Interface::cast::<IJsonObjectWithDefaultValues>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetNamedObjectOrDefault)(::windows_core::Interface::as_raw(this), name.into_param().abi(), defaultvalue.into_param().abi(), result__.as_mut_ptr()).from_abi::<JsonObject>(result__)
        }
    }
    pub fn GetNamedStringOrDefault<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, name: Param0, defaultvalue: Param1) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IJsonObjectWithDefaultValues>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).GetNamedStringOrDefault)(::windows_core::Interface::as_raw(this), name.into_param().abi(), defaultvalue.into_param().abi(), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn GetNamedArrayOrDefault<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, JsonArray>>(&self, name: Param0, defaultvalue: Param1) -> ::windows_core::Result<JsonArray> {
        let this = &::windows_core::Interface::cast::<IJsonObjectWithDefaultValues>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetNamedArrayOrDefault)(::windows_core::Interface::as_raw(this), name.into_param().abi(), defaultvalue.into_param().abi(), result__.as_mut_ptr()).from_abi::<JsonArray>(result__)
        }
    }
    pub fn GetNamedNumberOrDefault<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, name: Param0, defaultvalue: f64) -> ::windows_core::Result<f64> {
        let this = &::windows_core::Interface::cast::<IJsonObjectWithDefaultValues>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).GetNamedNumberOrDefault)(::windows_core::Interface::as_raw(this), name.into_param().abi(), defaultvalue, result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn GetNamedBooleanOrDefault<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, name: Param0, defaultvalue: bool) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IJsonObjectWithDefaultValues>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).GetNamedBooleanOrDefault)(::windows_core::Interface::as_raw(this), name.into_param().abi(), defaultvalue, result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn ValueType(&self) -> ::windows_core::Result<JsonValueType> {
        let this = &::windows_core::Interface::cast::<IJsonValue>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<JsonValueType>::zeroed();
            (::windows_core::Interface::vtable(this).ValueType)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<JsonValueType>(result__)
        }
    }
    pub fn Stringify(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IJsonValue>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Stringify)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn GetString(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<IJsonValue>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).GetString)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn GetNumber(&self) -> ::windows_core::Result<f64> {
        let this = &::windows_core::Interface::cast::<IJsonValue>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).GetNumber)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn GetBoolean(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<IJsonValue>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).GetBoolean)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn GetArray(&self) -> ::windows_core::Result<JsonArray> {
        let this = &::windows_core::Interface::cast::<IJsonValue>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetArray)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<JsonArray>(result__)
        }
    }
    pub fn GetObject(&self) -> ::windows_core::Result<JsonObject> {
        let this = &::windows_core::Interface::cast::<IJsonValue>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetObject)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<JsonObject>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Lookup<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, key: Param0) -> ::windows_core::Result<IJsonValue> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IMap<::windows_core::HSTRING, IJsonValue>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Lookup)(::windows_core::Interface::as_raw(this), key.into_param().abi(), result__.as_mut_ptr()).from_abi::<IJsonValue>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Size(&self) -> ::windows_core::Result<u32> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IMap<::windows_core::HSTRING, IJsonValue>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows_core::Interface::vtable(this).Size)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn HasKey<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, key: Param0) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IMap<::windows_core::HSTRING, IJsonValue>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).HasKey)(::windows_core::Interface::as_raw(this), key.into_param().abi(), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetView(&self) -> ::windows_core::Result<::winrt_foundation::Collections::IMapView<::windows_core::HSTRING, IJsonValue>> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IMap<::windows_core::HSTRING, IJsonValue>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetView)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::winrt_foundation::Collections::IMapView<::windows_core::HSTRING, IJsonValue>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Insert<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>, Param1: ::windows_core::IntoParam<'a, IJsonValue>>(&self, key: Param0, value: Param1) -> ::windows_core::Result<bool> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IMap<::windows_core::HSTRING, IJsonValue>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).Insert)(::windows_core::Interface::as_raw(this), key.into_param().abi(), value.into_param().abi(), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Remove<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(&self, key: Param0) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IMap<::windows_core::HSTRING, IJsonValue>>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Remove)(::windows_core::Interface::as_raw(this), key.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Clear(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::Collections::IMap<::windows_core::HSTRING, IJsonValue>>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Clear)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn ToString(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::IStringable>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).ToString)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn IJsonObjectStatics<R, F: FnOnce(&IJsonObjectStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<JsonObject, IJsonObjectStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for JsonObject {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for JsonObject {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for JsonObject {}
impl ::core::fmt::Debug for JsonObject {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("JsonObject").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for JsonObject {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Data.Json.JsonObject;{064e24dd-29c2-4f83-9ac1-9ee11578beb3})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for JsonObject {
    type Vtable = IJsonObject_Vtbl;
    const IID: ::windows_core::GUID = <IJsonObject as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for JsonObject {
    const NAME: &'static str = "Windows.Data.Json.JsonObject";
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for JsonObject {
    type Item = ::winrt_foundation::Collections::IKeyValuePair<::windows_core::HSTRING, IJsonValue>;
    type IntoIter = ::winrt_foundation::Collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for &JsonObject {
    type Item = ::winrt_foundation::Collections::IKeyValuePair<::windows_core::HSTRING, IJsonValue>;
    type IntoIter = ::winrt_foundation::Collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.First().unwrap()
    }
}
impl ::core::convert::From<JsonObject> for ::windows_core::IUnknown {
    fn from(value: JsonObject) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&JsonObject> for ::windows_core::IUnknown {
    fn from(value: &JsonObject) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for JsonObject {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a JsonObject {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<JsonObject> for ::windows_core::IInspectable {
    fn from(value: JsonObject) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&JsonObject> for ::windows_core::IInspectable {
    fn from(value: &JsonObject) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for JsonObject {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a JsonObject {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<JsonObject> for ::winrt_foundation::Collections::IIterable<::winrt_foundation::Collections::IKeyValuePair<::windows_core::HSTRING, IJsonValue>> {
    type Error = ::windows_core::Error;
    fn try_from(value: JsonObject) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&JsonObject> for ::winrt_foundation::Collections::IIterable<::winrt_foundation::Collections::IKeyValuePair<::windows_core::HSTRING, IJsonValue>> {
    type Error = ::windows_core::Error;
    fn try_from(value: &JsonObject) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<::winrt_foundation::Collections::IKeyValuePair<::windows_core::HSTRING, IJsonValue>>> for JsonObject {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::Collections::IIterable<::winrt_foundation::Collections::IKeyValuePair<::windows_core::HSTRING, IJsonValue>>> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IIterable<::winrt_foundation::Collections::IKeyValuePair<::windows_core::HSTRING, IJsonValue>>> for &JsonObject {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::Collections::IIterable<::winrt_foundation::Collections::IKeyValuePair<::windows_core::HSTRING, IJsonValue>>> {
        ::core::convert::TryInto::<::winrt_foundation::Collections::IIterable<::winrt_foundation::Collections::IKeyValuePair<::windows_core::HSTRING, IJsonValue>>>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<JsonObject> for IJsonValue {
    type Error = ::windows_core::Error;
    fn try_from(value: JsonObject) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&JsonObject> for IJsonValue {
    type Error = ::windows_core::Error;
    fn try_from(value: &JsonObject) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IJsonValue> for JsonObject {
    fn into_param(self) -> ::windows_core::Param<'a, IJsonValue> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IJsonValue> for &JsonObject {
    fn into_param(self) -> ::windows_core::Param<'a, IJsonValue> {
        ::core::convert::TryInto::<IJsonValue>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<JsonObject> for ::winrt_foundation::Collections::IMap<::windows_core::HSTRING, IJsonValue> {
    type Error = ::windows_core::Error;
    fn try_from(value: JsonObject) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&JsonObject> for ::winrt_foundation::Collections::IMap<::windows_core::HSTRING, IJsonValue> {
    type Error = ::windows_core::Error;
    fn try_from(value: &JsonObject) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IMap<::windows_core::HSTRING, IJsonValue>> for JsonObject {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::Collections::IMap<::windows_core::HSTRING, IJsonValue>> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::Collections::IMap<::windows_core::HSTRING, IJsonValue>> for &JsonObject {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::Collections::IMap<::windows_core::HSTRING, IJsonValue>> {
        ::core::convert::TryInto::<::winrt_foundation::Collections::IMap<::windows_core::HSTRING, IJsonValue>>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<JsonObject> for ::winrt_foundation::IStringable {
    type Error = ::windows_core::Error;
    fn try_from(value: JsonObject) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&JsonObject> for ::winrt_foundation::IStringable {
    type Error = ::windows_core::Error;
    fn try_from(value: &JsonObject) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IStringable> for JsonObject {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IStringable> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IStringable> for &JsonObject {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IStringable> {
        ::core::convert::TryInto::<::winrt_foundation::IStringable>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for JsonObject {}
unsafe impl ::core::marker::Sync for JsonObject {}
#[repr(transparent)]
pub struct JsonValue(::windows_core::IUnknown);
impl JsonValue {
    pub fn ValueType(&self) -> ::windows_core::Result<JsonValueType> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<JsonValueType>::zeroed();
            (::windows_core::Interface::vtable(this).ValueType)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<JsonValueType>(result__)
        }
    }
    pub fn Stringify(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).Stringify)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn GetString(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).GetString)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn GetNumber(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows_core::Interface::vtable(this).GetNumber)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn GetBoolean(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).GetBoolean)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn GetArray(&self) -> ::windows_core::Result<JsonArray> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetArray)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<JsonArray>(result__)
        }
    }
    pub fn GetObject(&self) -> ::windows_core::Result<JsonObject> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).GetObject)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<JsonObject>(result__)
        }
    }
    pub fn Parse<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(input: Param0) -> ::windows_core::Result<JsonValue> {
        Self::IJsonValueStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).Parse)(::windows_core::Interface::as_raw(this), input.into_param().abi(), result__.as_mut_ptr()).from_abi::<JsonValue>(result__)
        })
    }
    pub fn TryParse<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(input: Param0, result: &mut ::core::option::Option<JsonValue>) -> ::windows_core::Result<bool> {
        Self::IJsonValueStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows_core::Interface::vtable(this).TryParse)(::windows_core::Interface::as_raw(this), input.into_param().abi(), result as *mut _ as _, result__.as_mut_ptr()).from_abi::<bool>(result__)
        })
    }
    pub fn CreateBooleanValue(input: bool) -> ::windows_core::Result<JsonValue> {
        Self::IJsonValueStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateBooleanValue)(::windows_core::Interface::as_raw(this), input, result__.as_mut_ptr()).from_abi::<JsonValue>(result__)
        })
    }
    pub fn CreateNumberValue(input: f64) -> ::windows_core::Result<JsonValue> {
        Self::IJsonValueStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateNumberValue)(::windows_core::Interface::as_raw(this), input, result__.as_mut_ptr()).from_abi::<JsonValue>(result__)
        })
    }
    pub fn CreateStringValue<'a, Param0: ::windows_core::IntoParam<'a, ::windows_core::HSTRING>>(input: Param0) -> ::windows_core::Result<JsonValue> {
        Self::IJsonValueStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateStringValue)(::windows_core::Interface::as_raw(this), input.into_param().abi(), result__.as_mut_ptr()).from_abi::<JsonValue>(result__)
        })
    }
    pub fn CreateNullValue() -> ::windows_core::Result<JsonValue> {
        Self::IJsonValueStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows_core::RawPtr>::zeroed();
            (::windows_core::Interface::vtable(this).CreateNullValue)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<JsonValue>(result__)
        })
    }
    pub fn ToString(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<::winrt_foundation::IStringable>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows_core::HSTRING>>::zeroed();
            (::windows_core::Interface::vtable(this).ToString)(::windows_core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows_core::HSTRING>(result__)
        }
    }
    pub fn IJsonValueStatics<R, F: FnOnce(&IJsonValueStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<JsonValue, IJsonValueStatics> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IJsonValueStatics2<R, F: FnOnce(&IJsonValueStatics2) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static mut SHARED: ::windows_core::FactoryCache<JsonValue, IJsonValueStatics2> = ::windows_core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for JsonValue {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for JsonValue {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for JsonValue {}
impl ::core::fmt::Debug for JsonValue {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("JsonValue").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for JsonValue {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"rc(Windows.Data.Json.JsonValue;{a3219ecb-f0b3-4dcd-beee-19d48cd3ed1e})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows_core::Error::OK)
    }
}
unsafe impl ::windows_core::Interface for JsonValue {
    type Vtable = IJsonValue_Vtbl;
    const IID: ::windows_core::GUID = <IJsonValue as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for JsonValue {
    const NAME: &'static str = "Windows.Data.Json.JsonValue";
}
impl ::core::convert::From<JsonValue> for ::windows_core::IUnknown {
    fn from(value: JsonValue) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&JsonValue> for ::windows_core::IUnknown {
    fn from(value: &JsonValue) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for JsonValue {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IUnknown> for &'a JsonValue {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IUnknown> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<JsonValue> for ::windows_core::IInspectable {
    fn from(value: JsonValue) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&JsonValue> for ::windows_core::IInspectable {
    fn from(value: &JsonValue) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for JsonValue {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::windows_core::IInspectable> for &'a JsonValue {
    fn into_param(self) -> ::windows_core::Param<'a, ::windows_core::IInspectable> {
        ::windows_core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<JsonValue> for IJsonValue {
    type Error = ::windows_core::Error;
    fn try_from(value: JsonValue) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&JsonValue> for IJsonValue {
    type Error = ::windows_core::Error;
    fn try_from(value: &JsonValue) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IJsonValue> for JsonValue {
    fn into_param(self) -> ::windows_core::Param<'a, IJsonValue> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, IJsonValue> for &JsonValue {
    fn into_param(self) -> ::windows_core::Param<'a, IJsonValue> {
        ::core::convert::TryInto::<IJsonValue>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
impl ::core::convert::TryFrom<JsonValue> for ::winrt_foundation::IStringable {
    type Error = ::windows_core::Error;
    fn try_from(value: JsonValue) -> ::windows_core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&JsonValue> for ::winrt_foundation::IStringable {
    type Error = ::windows_core::Error;
    fn try_from(value: &JsonValue) -> ::windows_core::Result<Self> {
        ::windows_core::Interface::cast(value)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IStringable> for JsonValue {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IStringable> {
        ::windows_core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows_core::IntoParam<'a, ::winrt_foundation::IStringable> for &JsonValue {
    fn into_param(self) -> ::windows_core::Param<'a, ::winrt_foundation::IStringable> {
        ::core::convert::TryInto::<::winrt_foundation::IStringable>::try_into(self).map(::windows_core::Param::Owned).unwrap_or(::windows_core::Param::None)
    }
}
unsafe impl ::core::marker::Send for JsonValue {}
unsafe impl ::core::marker::Sync for JsonValue {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
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
impl ::core::default::Default for JsonValueType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows_core::Abi for JsonValueType {
    type Abi = Self;
}
impl ::core::fmt::Debug for JsonValueType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("JsonValueType").field(&self.0).finish()
    }
}
unsafe impl ::windows_core::RuntimeType for JsonValueType {
    const SIGNATURE: ::windows_core::ConstBuffer = ::windows_core::ConstBuffer::from_slice(b"enum(Windows.Data.Json.JsonValueType;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows_core::Result<Self> {
        Ok(*from)
    }
}
